use axum::body::Body;
use axum::http::{header, Request, StatusCode};
use axum::Router;
use paperclip_backend::auth::AgentKeyStore;
use paperclip_backend::{app_with, Repositories};
use serde_json::{json, Value};
use tower::ServiceExt; // for `oneshot`

async fn send(
    app: &Router,
    method: &str,
    uri: &str,
    bearer: Option<&str>,
    body: Option<Value>,
) -> (StatusCode, Value) {
    let mut builder = Request::builder().method(method).uri(uri);
    if let Some(token) = bearer {
        builder = builder.header(header::AUTHORIZATION, format!("Bearer {token}"));
    }
    let request = match body {
        Some(b) => builder
            .header("content-type", "application/json")
            .body(Body::from(b.to_string()))
            .unwrap(),
        None => builder.body(Body::empty()).unwrap(),
    };
    let response = app.clone().oneshot(request).await.unwrap();
    let status = response.status();
    let bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let json = if bytes.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&bytes).unwrap()
    };
    (status, json)
}

async fn id_of(app: &Router, method: &str, uri: &str, body: Value) -> String {
    let (_, v) = send(app, method, uri, None, Some(body)).await;
    v["id"].as_str().unwrap().to_string()
}

// Ports server/src/services/dashboard.ts summary: cross-domain status rollups.
#[tokio::test]
async fn dashboard_aggregates_agent_task_and_approval_counts() {
    let app = app_with(Repositories::default());

    // Agents: one stays idle (→ active bucket), one moves to running.
    id_of(
        &app,
        "POST",
        "/api/companies/co-1/agents",
        json!({ "name": "Ada", "adapterType": "process" }),
    )
    .await;
    let running_agent = id_of(
        &app,
        "POST",
        "/api/companies/co-1/agents",
        json!({ "name": "Bee", "adapterType": "process" }),
    )
    .await;
    send(
        &app,
        "PATCH",
        &format!("/api/companies/co-1/agents/{running_agent}"),
        None,
        Some(json!({ "status": "running" })),
    )
    .await;

    // Issues: backlog (open), in_progress (open + inProgress), done (not open).
    id_of(
        &app,
        "POST",
        "/api/companies/co-1/issues",
        json!({ "title": "X" }),
    )
    .await;
    let ip = id_of(
        &app,
        "POST",
        "/api/companies/co-1/issues",
        json!({ "title": "Y" }),
    )
    .await;
    send(
        &app,
        "PATCH",
        &format!("/api/companies/co-1/issues/{ip}"),
        None,
        Some(json!({ "status": "in_progress" })),
    )
    .await;
    let done = id_of(
        &app,
        "POST",
        "/api/companies/co-1/issues",
        json!({ "title": "Z" }),
    )
    .await;
    send(
        &app,
        "PATCH",
        &format!("/api/companies/co-1/issues/{done}"),
        None,
        Some(json!({ "status": "done" })),
    )
    .await;

    // One pending approval.
    id_of(
        &app,
        "POST",
        "/api/companies/co-1/approvals",
        json!({ "issueId": "i-1" }),
    )
    .await;

    let (status, dash) = send(&app, "GET", "/api/companies/co-1/dashboard", None, None).await;
    assert_eq!(status, StatusCode::OK);

    assert_eq!(
        dash["agents"],
        json!({ "active": 1, "running": 1, "paused": 0, "error": 0 })
    );
    assert_eq!(
        dash["tasks"],
        json!({ "open": 2, "inProgress": 1, "blocked": 0, "done": 1 })
    );
    assert_eq!(dash["approvals"], json!({ "pending": 1 }));
}

#[tokio::test]
async fn empty_company_dashboard_is_all_zeroes() {
    let app = app_with(Repositories::default());
    let (status, dash) = send(&app, "GET", "/api/companies/co-1/dashboard", None, None).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(
        dash["agents"],
        json!({ "active": 0, "running": 0, "paused": 0, "error": 0 })
    );
    assert_eq!(
        dash["tasks"],
        json!({ "open": 0, "inProgress": 0, "blocked": 0, "done": 0 })
    );
    assert_eq!(dash["approvals"], json!({ "pending": 0 }));
}

#[tokio::test]
async fn agent_cannot_access_other_company_dashboard() {
    let keys = AgentKeyStore::default();
    keys.insert("k", "co-1");
    let app = app_with(Repositories {
        agent_keys: keys,
        ..Default::default()
    });

    let (status, body) = send(
        &app,
        "GET",
        "/api/companies/co-2/dashboard",
        Some("k"),
        None,
    )
    .await;
    assert_eq!(status, StatusCode::FORBIDDEN);
    assert_eq!(body, json!({ "error": "forbidden" }));
}
