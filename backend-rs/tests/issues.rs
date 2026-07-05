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

/// Ports `POST /api/companies/:companyId/issues`: 201 with the created issue,
/// carrying `companyId`, the given `title`, and the default `status` ("backlog"
/// per packages/db/src/schema/issues.ts).
#[tokio::test]
async fn creates_issue_under_company() {
    let app = app();
    let (status, created) = send(
        &app,
        "POST",
        "/api/companies/co-1/issues",
        Some(json!({ "title": "Fix bug" })),
    )
    .await;

    assert_eq!(status, StatusCode::CREATED);
    assert_eq!(created["title"], "Fix bug");
    assert_eq!(created["companyId"], "co-1");
    assert_eq!(created["status"], "backlog");
    assert!(created["id"].as_str().is_some_and(|s| !s.is_empty()));
}

/// Ports `GET /api/companies/:companyId/issues`: lists that company's issues.
#[tokio::test]
async fn lists_issues_for_a_company() {
    let app = app();
    send(
        &app,
        "POST",
        "/api/companies/co-1/issues",
        Some(json!({ "title": "A" })),
    )
    .await;
    send(
        &app,
        "POST",
        "/api/companies/co-1/issues",
        Some(json!({ "title": "B" })),
    )
    .await;

    let (status, list) = send(&app, "GET", "/api/companies/co-1/issues", None).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(list.as_array().unwrap().len(), 2);
}

/// Enforces the company-scoping invariant: issues created under one company are
/// never visible under another.
#[tokio::test]
async fn issues_are_isolated_by_company() {
    let app = app();
    send(
        &app,
        "POST",
        "/api/companies/co-1/issues",
        Some(json!({ "title": "secret" })),
    )
    .await;

    let (status, other) = send(&app, "GET", "/api/companies/co-2/issues", None).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(other, json!([]));
}
