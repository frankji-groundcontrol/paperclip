use axum::body::Body;
use axum::http::{Request, StatusCode};
use axum::Router;
use paperclip_backend::{app_with, Repositories};
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

async fn create_issue(app: &Router) -> String {
    let (_, created) = send(
        app,
        "POST",
        "/api/companies/co-1/issues",
        Some(json!({ "title": "Fix bug" })),
    )
    .await;
    created["id"].as_str().unwrap().to_string()
}

/// Ports `PATCH /api/companies/:companyId/issues/:issueId`: applies a partial
/// update, leaving unspecified fields unchanged.
#[tokio::test]
async fn updates_issue_status() {
    let app = app_with(Repositories::default());
    let id = create_issue(&app).await;

    let (status, updated) = send(
        &app,
        "PATCH",
        &format!("/api/companies/co-1/issues/{id}"),
        Some(json!({ "status": "in_progress" })),
    )
    .await;

    assert_eq!(status, StatusCode::OK);
    assert_eq!(updated["status"], "in_progress");
    assert_eq!(updated["title"], "Fix bug");
}

/// Updating an unknown issue is a 404.
#[tokio::test]
async fn update_unknown_issue_is_404() {
    let app = app_with(Repositories::default());
    let (status, body) = send(
        &app,
        "PATCH",
        "/api/companies/co-1/issues/does-not-exist",
        Some(json!({ "status": "in_progress" })),
    )
    .await;

    assert_eq!(status, StatusCode::NOT_FOUND);
    assert_eq!(body, json!({ "error": "Issue not found" }));
}

/// Ports `DELETE /api/companies/:companyId/issues/:issueId`: 204, and the issue
/// no longer appears in the list.
#[tokio::test]
async fn deletes_issue() {
    let app = app_with(Repositories::default());
    let id = create_issue(&app).await;

    let (status, _) = send(
        &app,
        "DELETE",
        &format!("/api/companies/co-1/issues/{id}"),
        None,
    )
    .await;
    assert_eq!(status, StatusCode::NO_CONTENT);

    let (_, list) = send(&app, "GET", "/api/companies/co-1/issues", None).await;
    assert_eq!(list, json!([]));
}

/// Deleting an unknown issue is a 404.
#[tokio::test]
async fn delete_unknown_issue_is_404() {
    let app = app_with(Repositories::default());
    let (status, body) = send(
        &app,
        "DELETE",
        "/api/companies/co-1/issues/does-not-exist",
        None,
    )
    .await;

    assert_eq!(status, StatusCode::NOT_FOUND);
    assert_eq!(body, json!({ "error": "Issue not found" }));
}
