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

async fn create_secret(app: &Router) -> String {
    let (_, created) = send(
        app,
        "POST",
        "/api/companies/co-1/secrets",
        Some(json!({ "name": "API_KEY", "value": "super-secret-value" })),
    )
    .await;
    created["id"].as_str().unwrap().to_string()
}

#[tokio::test]
async fn deletes_secret() {
    let app = app_with(Repositories::default());
    let id = create_secret(&app).await;

    let (status, body) = send(
        &app,
        "DELETE",
        &format!("/api/companies/co-1/secrets/{id}"),
        None,
    )
    .await;
    assert_eq!(status, StatusCode::NO_CONTENT);
    // Even the delete response must never leak the plaintext value.
    assert!(!body.to_string().contains("super-secret-value"));

    let (_, list) = send(&app, "GET", "/api/companies/co-1/secrets", None).await;
    assert_eq!(list, json!([]));
}

#[tokio::test]
async fn delete_unknown_secret_is_404() {
    let app = app_with(Repositories::default());
    let (status, body) = send(
        &app,
        "DELETE",
        "/api/companies/co-1/secrets/does-not-exist",
        None,
    )
    .await;

    assert_eq!(status, StatusCode::NOT_FOUND);
    assert_eq!(body, json!({ "error": "Secret not found" }));
}
