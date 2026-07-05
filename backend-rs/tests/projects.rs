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

/// Ports `POST /api/companies/:companyId/projects`: 201 with the created project
/// (createProjectSchema requires `name`; status defaults to "backlog").
#[tokio::test]
async fn creates_project_under_company() {
    let app = app();
    let (status, created) = send(
        &app,
        "POST",
        "/api/companies/co-1/projects",
        Some(json!({ "name": "Website" })),
    )
    .await;

    assert_eq!(status, StatusCode::CREATED);
    assert_eq!(created["name"], "Website");
    assert_eq!(created["companyId"], "co-1");
    assert_eq!(created["status"], "backlog");
    assert!(created["id"].as_str().is_some_and(|s| !s.is_empty()));
}

/// Ports `GET /api/companies/:companyId/projects`: lists that company's projects.
#[tokio::test]
async fn lists_projects_for_a_company() {
    let app = app();
    send(
        &app,
        "POST",
        "/api/companies/co-1/projects",
        Some(json!({ "name": "A" })),
    )
    .await;
    send(
        &app,
        "POST",
        "/api/companies/co-1/projects",
        Some(json!({ "name": "B" })),
    )
    .await;

    let (status, list) = send(&app, "GET", "/api/companies/co-1/projects", None).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(list.as_array().unwrap().len(), 2);
}

/// Company-scoping invariant: one company's projects are invisible to another.
#[tokio::test]
async fn projects_are_isolated_by_company() {
    let app = app();
    send(
        &app,
        "POST",
        "/api/companies/co-1/projects",
        Some(json!({ "name": "secret" })),
    )
    .await;

    let (status, other) = send(&app, "GET", "/api/companies/co-2/projects", None).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(other, json!([]));
}
