use axum::body::Body;
use axum::http::{Request, StatusCode};
use axum::Router;
use paperclip_backend::app;
use serde_json::{json, Value};
use tower::ServiceExt; // for `oneshot`

/// Sends a request against a cloned router so in-memory state (Arc-backed) is
/// shared across calls to the same `app` instance.
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

/// Ports `GET /api/companies` (server/src/routes/companies.ts): returns a JSON
/// array, empty when no companies exist.
#[tokio::test]
async fn lists_companies_empty_initially() {
    let app = app();
    let (status, body) = send(&app, "GET", "/api/companies", None).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body, json!([]));
}

/// Ports `POST /api/companies`: 201 with the created company (createCompanySchema
/// requires `name`), which then appears in the list.
#[tokio::test]
async fn creates_company_and_lists_it() {
    let app = app();
    let (status, created) = send(
        &app,
        "POST",
        "/api/companies",
        Some(json!({ "name": "Acme" })),
    )
    .await;
    assert_eq!(status, StatusCode::CREATED);
    assert_eq!(created["name"], "Acme");
    assert!(
        created["id"].as_str().is_some_and(|s| !s.is_empty()),
        "expected a non-empty id, got {:?}",
        created["id"]
    );

    let (status, list) = send(&app, "GET", "/api/companies", None).await;
    assert_eq!(status, StatusCode::OK);
    let arr = list.as_array().expect("list should be an array");
    assert_eq!(arr.len(), 1);
    assert_eq!(arr[0]["name"], "Acme");
}

/// Ports `GET /api/companies/:id`: the company, or 404
/// `{ error: "Company not found" }` for an unknown id.
#[tokio::test]
async fn gets_company_by_id_or_404() {
    let app = app();
    let (_, created) = send(
        &app,
        "POST",
        "/api/companies",
        Some(json!({ "name": "Beta" })),
    )
    .await;
    let id = created["id"].as_str().unwrap().to_string();

    let (status, fetched) = send(&app, "GET", &format!("/api/companies/{id}"), None).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(fetched["name"], "Beta");

    let (status, err) = send(&app, "GET", "/api/companies/does-not-exist", None).await;
    assert_eq!(status, StatusCode::NOT_FOUND);
    assert_eq!(err, json!({ "error": "Company not found" }));
}
