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

async fn create_company(app: &Router) -> String {
    let (_, created) = send(
        app,
        "POST",
        "/api/companies",
        Some(json!({ "name": "Acme" })),
    )
    .await;
    created["id"].as_str().unwrap().to_string()
}

/// Ports `PATCH /api/companies/:companyId`: partial update.
#[tokio::test]
async fn updates_company_name() {
    let app = app_with(Repositories::default());
    let id = create_company(&app).await;

    let (status, updated) = send(
        &app,
        "PATCH",
        &format!("/api/companies/{id}"),
        Some(json!({ "name": "Acme Inc" })),
    )
    .await;

    assert_eq!(status, StatusCode::OK);
    assert_eq!(updated["name"], "Acme Inc");
}

#[tokio::test]
async fn update_unknown_company_is_404() {
    let app = app_with(Repositories::default());
    let (status, body) = send(
        &app,
        "PATCH",
        "/api/companies/does-not-exist",
        Some(json!({ "name": "Nope" })),
    )
    .await;

    assert_eq!(status, StatusCode::NOT_FOUND);
    assert_eq!(body, json!({ "error": "Company not found" }));
}

/// Ports `DELETE /api/companies/:companyId`: 204, then the company is gone (its
/// detail endpoint 404s).
#[tokio::test]
async fn deletes_company() {
    let app = app_with(Repositories::default());
    let id = create_company(&app).await;

    let (status, _) = send(&app, "DELETE", &format!("/api/companies/{id}"), None).await;
    assert_eq!(status, StatusCode::NO_CONTENT);

    let (status, _) = send(&app, "GET", &format!("/api/companies/{id}"), None).await;
    assert_eq!(status, StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn delete_unknown_company_is_404() {
    let app = app_with(Repositories::default());
    let (status, body) = send(&app, "DELETE", "/api/companies/does-not-exist", None).await;

    assert_eq!(status, StatusCode::NOT_FOUND);
    assert_eq!(body, json!({ "error": "Company not found" }));
}
