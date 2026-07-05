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
    let (status, p) = send(
        app,
        "POST",
        "/api/companies/co-1/pipelines",
        None,
        Some(body),
    )
    .await;
    assert_eq!(status, StatusCode::CREATED);
    p
}

// Ports server/src/routes/pipelines.ts core pipeline entity (list/create/get/
// patch); the case/stage machinery is deferred.
#[tokio::test]
async fn creates_pipeline_with_defaults() {
    let app = app_with(Repositories::default());
    let p = create(&app, json!({ "key": "intake", "name": "Intake" })).await;

    assert_eq!(p["key"], "intake");
    assert_eq!(p["name"], "Intake");
    assert_eq!(p["companyId"], "co-1");
    assert_eq!(p["enforceTransitions"], false); // default
    assert_eq!(p["archived"], false); // default
    assert_eq!(p["description"], Value::Null);
    assert_eq!(p["projectId"], Value::Null);
    assert!(p["id"].as_str().is_some_and(|s| !s.is_empty()));
}

#[tokio::test]
async fn lists_and_gets_a_pipeline() {
    let app = app_with(Repositories::default());
    let created = create(&app, json!({ "key": "intake", "name": "Intake" })).await;
    let id = created["id"].as_str().unwrap();

    let (status, list) = send(&app, "GET", "/api/companies/co-1/pipelines", None, None).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(list.as_array().unwrap().len(), 1);

    let (status, got) = send(
        &app,
        "GET",
        &format!("/api/companies/co-1/pipelines/{id}"),
        None,
        None,
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(got["id"], id);
    assert_eq!(got["key"], "intake");
}

#[tokio::test]
async fn get_unknown_pipeline_is_404() {
    let app = app_with(Repositories::default());
    let (status, body) = send(
        &app,
        "GET",
        "/api/companies/co-1/pipelines/nope",
        None,
        None,
    )
    .await;
    assert_eq!(status, StatusCode::NOT_FOUND);
    assert_eq!(body, json!({ "error": "Pipeline not found" }));
}

#[tokio::test]
async fn patches_pipeline_archived_flag() {
    let app = app_with(Repositories::default());
    let created = create(&app, json!({ "key": "intake", "name": "Intake" })).await;
    let id = created["id"].as_str().unwrap();

    let (status, updated) = send(
        &app,
        "PATCH",
        &format!("/api/companies/co-1/pipelines/{id}"),
        None,
        Some(json!({ "archived": true, "name": "Intake v2" })),
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(updated["archived"], true);
    assert_eq!(updated["name"], "Intake v2");
    assert_eq!(updated["key"], "intake"); // unchanged
}

#[tokio::test]
async fn patch_unknown_pipeline_is_404() {
    let app = app_with(Repositories::default());
    let (status, body) = send(
        &app,
        "PATCH",
        "/api/companies/co-1/pipelines/nope",
        None,
        Some(json!({ "archived": true })),
    )
    .await;
    assert_eq!(status, StatusCode::NOT_FOUND);
    assert_eq!(body, json!({ "error": "Pipeline not found" }));
}

#[tokio::test]
async fn pipelines_isolated_by_company_and_authz() {
    let keys = AgentKeyStore::default();
    keys.insert("k", "co-1");
    let app = app_with(Repositories {
        agent_keys: keys,
        ..Default::default()
    });

    let (status, body) = send(
        &app,
        "GET",
        "/api/companies/co-2/pipelines",
        Some("k"),
        None,
    )
    .await;
    assert_eq!(status, StatusCode::FORBIDDEN);
    assert_eq!(body, json!({ "error": "forbidden" }));
}
