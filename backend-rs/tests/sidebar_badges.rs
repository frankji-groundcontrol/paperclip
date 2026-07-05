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

async fn id_of(app: &Router, uri: &str, body: Value) -> String {
    let (_, v) = send(app, "POST", uri, None, Some(body)).await;
    v["id"].as_str().unwrap().to_string()
}

// Ports server/src/routes/sidebar-badges.ts core: cross-domain pending counts
// (failed runs + pending approvals + pending join-requests). Alerts/dismissals
// deferred.
#[tokio::test]
async fn badges_aggregate_pending_counts() {
    let app = app_with(Repositories::default());

    // Two pending approvals.
    id_of(
        &app,
        "/api/companies/co-1/approvals",
        json!({ "issueId": "i-1" }),
    )
    .await;
    id_of(
        &app,
        "/api/companies/co-1/approvals",
        json!({ "issueId": "i-2" }),
    )
    .await;

    // One failed run.
    let run = id_of(
        &app,
        "/api/companies/co-1/runs",
        json!({ "agentId": "a-1" }),
    )
    .await;
    send(
        &app,
        "PATCH",
        &format!("/api/companies/co-1/runs/{run}"),
        None,
        Some(json!({ "status": "failed" })),
    )
    .await;

    // One pending join-request.
    id_of(
        &app,
        "/api/companies/co-1/join-requests",
        json!({ "requestType": "human" }),
    )
    .await;

    let (status, badges) = send(
        &app,
        "GET",
        "/api/companies/co-1/sidebar-badges",
        None,
        None,
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(
        badges,
        json!({ "failedRuns": 1, "approvals": 2, "joinRequests": 1, "inbox": 4 })
    );
}

#[tokio::test]
async fn empty_badges_are_all_zero() {
    let app = app_with(Repositories::default());
    let (status, badges) = send(
        &app,
        "GET",
        "/api/companies/co-1/sidebar-badges",
        None,
        None,
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(
        badges,
        json!({ "failedRuns": 0, "approvals": 0, "joinRequests": 0, "inbox": 0 })
    );
}

#[tokio::test]
async fn agent_cannot_access_other_company_badges() {
    let keys = AgentKeyStore::default();
    keys.insert("k", "co-1");
    let app = app_with(Repositories {
        agent_keys: keys,
        ..Default::default()
    });
    let (status, body) = send(
        &app,
        "GET",
        "/api/companies/co-2/sidebar-badges",
        Some("k"),
        None,
    )
    .await;
    assert_eq!(status, StatusCode::FORBIDDEN);
    assert_eq!(body, json!({ "error": "forbidden" }));
}
