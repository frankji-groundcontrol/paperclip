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

async fn enable_cloud_sync(app: &Router) {
    send(
        app,
        "PATCH",
        "/api/instance/settings/experimental",
        None,
        Some(json!({ "enableCloudSync": true })),
    )
    .await;
}

// Ports server/src/routes/cloud-upstreams.ts core: gated by the instance-settings
// `enableCloudSync` flag; list/register/delete connections (OAuth + push-runs deferred).
#[tokio::test]
async fn list_is_404_when_cloud_sync_disabled() {
    let app = app_with(Repositories::default());
    let (status, body) = send(&app, "GET", "/api/cloud-upstreams", None, None).await;
    assert_eq!(status, StatusCode::NOT_FOUND);
    assert_eq!(body, json!({ "error": "Cloud sync is not enabled" }));
}

#[tokio::test]
async fn lists_and_registers_when_enabled() {
    let app = app_with(Repositories::default());
    enable_cloud_sync(&app).await;

    let (status, empty) = send(&app, "GET", "/api/cloud-upstreams", None, None).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(empty, json!([]));

    let (status, conn) = send(
        &app,
        "POST",
        "/api/cloud-upstreams/register",
        None,
        Some(json!({ "companyId": "co-1", "remoteUrl": "https://remote.example" })),
    )
    .await;
    assert_eq!(status, StatusCode::CREATED);
    assert_eq!(conn["companyId"], "co-1");
    assert_eq!(conn["remoteUrl"], "https://remote.example");
    assert_eq!(conn["status"], "connected");

    let (_, list) = send(&app, "GET", "/api/cloud-upstreams", None, None).await;
    assert_eq!(list.as_array().unwrap().len(), 1);
}

#[tokio::test]
async fn register_is_404_when_disabled() {
    let app = app_with(Repositories::default());
    let (status, body) = send(
        &app,
        "POST",
        "/api/cloud-upstreams/register",
        None,
        Some(json!({ "companyId": "co-1", "remoteUrl": "https://x" })),
    )
    .await;
    assert_eq!(status, StatusCode::NOT_FOUND);
    assert_eq!(body, json!({ "error": "Cloud sync is not enabled" }));
}

#[tokio::test]
async fn deletes_connection_and_unknown_is_404() {
    let app = app_with(Repositories::default());
    enable_cloud_sync(&app).await;
    let (_, conn) = send(
        &app,
        "POST",
        "/api/cloud-upstreams/register",
        None,
        Some(json!({ "companyId": "co-1", "remoteUrl": "https://x" })),
    )
    .await;
    let id = conn["id"].as_str().unwrap();

    let (status, _) = send(
        &app,
        "DELETE",
        &format!("/api/cloud-upstreams/{id}"),
        None,
        None,
    )
    .await;
    assert_eq!(status, StatusCode::NO_CONTENT);

    let (status, body) = send(&app, "DELETE", "/api/cloud-upstreams/nope", None, None).await;
    assert_eq!(status, StatusCode::NOT_FOUND);
    assert_eq!(body, json!({ "error": "Cloud upstream not found" }));
}

#[tokio::test]
async fn agent_is_forbidden() {
    let keys = AgentKeyStore::default();
    keys.insert("k", "co-1");
    let app = app_with(Repositories {
        agent_keys: keys,
        ..Default::default()
    });
    // Board-only: agents are rejected before the enablement check.
    let (status, body) = send(&app, "GET", "/api/cloud-upstreams", Some("k"), None).await;
    assert_eq!(status, StatusCode::FORBIDDEN);
    assert_eq!(body, json!({ "error": "forbidden" }));
}
