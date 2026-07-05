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

async fn create(app: &Router, body: Value) -> Value {
    let (status, env) = send(
        app,
        "POST",
        "/api/companies/co-1/environments",
        None,
        Some(body),
    )
    .await;
    assert_eq!(status, StatusCode::CREATED);
    env
}

// Ports server/src/routes/environments.ts core CRUD (packages/shared
// createEnvironmentSchema): company-scoped execution environments.
#[tokio::test]
async fn creates_environment_with_defaults() {
    let app = app_with(Repositories::default());
    let env = create(&app, json!({ "name": "prod", "driver": "local" })).await;

    assert_eq!(env["name"], "prod");
    assert_eq!(env["companyId"], "co-1");
    assert_eq!(env["driver"], "local");
    assert_eq!(env["status"], "active"); // schema default
    assert_eq!(env["config"], json!({}));
    assert_eq!(env["envVars"], json!({}));
    assert_eq!(env["description"], Value::Null);
    assert_eq!(env["metadata"], Value::Null);
    assert!(env["id"].as_str().is_some_and(|s| !s.is_empty()));
}

#[tokio::test]
async fn honors_explicit_status_and_config() {
    let app = app_with(Repositories::default());
    let env = create(
        &app,
        json!({
            "name": "sandbox-1",
            "driver": "sandbox",
            "status": "archived",
            "config": { "image": "ubuntu" },
            "description": "throwaway",
        }),
    )
    .await;

    assert_eq!(env["status"], "archived");
    assert_eq!(env["config"], json!({ "image": "ubuntu" }));
    assert_eq!(env["description"], "throwaway");
}

#[tokio::test]
async fn environments_isolated_by_company() {
    let app = app_with(Repositories::default());
    create(&app, json!({ "name": "prod", "driver": "local" })).await;

    let (status, list) = send(&app, "GET", "/api/companies/co-1/environments", None, None).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(list.as_array().unwrap().len(), 1);

    let (_, other) = send(&app, "GET", "/api/companies/co-2/environments", None, None).await;
    assert_eq!(other, json!([]));
}

#[tokio::test]
async fn agent_cannot_access_other_company_environments() {
    let keys = AgentKeyStore::default();
    keys.insert("k", "co-1");
    let app = app_with(Repositories {
        agent_keys: keys,
        ..Default::default()
    });

    let (status, body) = send(
        &app,
        "GET",
        "/api/companies/co-2/environments",
        Some("k"),
        None,
    )
    .await;
    assert_eq!(status, StatusCode::FORBIDDEN);
    assert_eq!(body, json!({ "error": "forbidden" }));
}

#[tokio::test]
async fn updates_environment_status() {
    let app = app_with(Repositories::default());
    let env = create(&app, json!({ "name": "prod", "driver": "local" })).await;
    let id = env["id"].as_str().unwrap();

    let (status, updated) = send(
        &app,
        "PATCH",
        &format!("/api/companies/co-1/environments/{id}"),
        None,
        Some(json!({ "status": "archived" })),
    )
    .await;

    assert_eq!(status, StatusCode::OK);
    assert_eq!(updated["status"], "archived");
    assert_eq!(updated["name"], "prod"); // unchanged
}

#[tokio::test]
async fn update_unknown_environment_is_404() {
    let app = app_with(Repositories::default());
    let (status, body) = send(
        &app,
        "PATCH",
        "/api/companies/co-1/environments/nope",
        None,
        Some(json!({ "status": "archived" })),
    )
    .await;

    assert_eq!(status, StatusCode::NOT_FOUND);
    assert_eq!(body, json!({ "error": "Environment not found" }));
}

#[tokio::test]
async fn deletes_environment() {
    let app = app_with(Repositories::default());
    let env = create(&app, json!({ "name": "prod", "driver": "local" })).await;
    let id = env["id"].as_str().unwrap();

    let (status, _) = send(
        &app,
        "DELETE",
        &format!("/api/companies/co-1/environments/{id}"),
        None,
        None,
    )
    .await;
    assert_eq!(status, StatusCode::NO_CONTENT);

    let (_, list) = send(&app, "GET", "/api/companies/co-1/environments", None, None).await;
    assert_eq!(list, json!([]));
}
