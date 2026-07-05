use axum::body::Body;
use axum::http::{Request, StatusCode};
use axum::Router;
use paperclip_backend::app;
use serde_json::{json, Value};
use tower::ServiceExt; // for `oneshot`

async fn send(app: &Router, method: &str, uri: &str, body: Option<Value>) -> (StatusCode, Value) {
    let builder = Request::builder().method(method).uri(uri);
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

/// Ports `POST /api/companies/:companyId/agents`: 201 with the created agent
/// (createAgentSchema: `name` + `adapterType` required; `role` defaults to
/// "general", `status` to "idle" per packages/db/src/schema/agents.ts).
#[tokio::test]
async fn creates_agent_under_company() {
    let app = app();
    let (status, created) = send(
        &app,
        "POST",
        "/api/companies/co-1/agents",
        Some(json!({ "name": "Ada", "adapterType": "claude_local" })),
    )
    .await;

    assert_eq!(status, StatusCode::CREATED);
    assert_eq!(created["name"], "Ada");
    assert_eq!(created["companyId"], "co-1");
    assert_eq!(created["role"], "general");
    assert_eq!(created["status"], "idle");
    assert_eq!(created["adapterType"], "claude_local");
    assert!(created["id"].as_str().is_some_and(|s| !s.is_empty()));
}

/// Ports `GET /api/companies/:companyId/agents`: lists that company's agents.
#[tokio::test]
async fn lists_agents_for_a_company() {
    let app = app();
    send(
        &app,
        "POST",
        "/api/companies/co-1/agents",
        Some(json!({ "name": "A", "adapterType": "process" })),
    )
    .await;
    send(
        &app,
        "POST",
        "/api/companies/co-1/agents",
        Some(json!({ "name": "B", "adapterType": "process" })),
    )
    .await;

    let (status, list) = send(&app, "GET", "/api/companies/co-1/agents", None).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(list.as_array().unwrap().len(), 2);
}

/// Company-scoping invariant: one company's agents are invisible to another.
#[tokio::test]
async fn agents_are_isolated_by_company() {
    let app = app();
    send(
        &app,
        "POST",
        "/api/companies/co-1/agents",
        Some(json!({ "name": "secret", "adapterType": "process" })),
    )
    .await;

    let (status, other) = send(&app, "GET", "/api/companies/co-2/agents", None).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(other, json!([]));
}
