use axum::body::Body;
use axum::http::{header, Request, StatusCode};
use axum::Router;
use paperclip_backend::auth::AgentKeyStore;
use paperclip_backend::{app_with, Repositories};
use tower::ServiceExt; // for `oneshot`

/// Returns (status, content_type, body_text).
async fn get_text(app: &Router, uri: &str, bearer: Option<&str>) -> (StatusCode, String, String) {
    let mut builder = Request::builder().method("GET").uri(uri);
    if let Some(token) = bearer {
        builder = builder.header(header::AUTHORIZATION, format!("Bearer {token}"));
    }
    let response = app
        .clone()
        .oneshot(builder.body(Body::empty()).unwrap())
        .await
        .unwrap();
    let status = response.status();
    let content_type = response
        .headers()
        .get(header::CONTENT_TYPE)
        .and_then(|v| v.to_str().ok())
        .unwrap_or("")
        .to_string();
    let bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let body = String::from_utf8(bytes.to_vec()).unwrap();
    (status, content_type, body)
}

// Ports server/src/routes/llms.ts text/plain config reflection endpoints.
#[tokio::test]
async fn agent_configuration_index_is_text_plain() {
    let app = app_with(Repositories::default());
    let (status, content_type, body) = get_text(&app, "/llms/agent-configuration.txt", None).await;

    assert_eq!(status, StatusCode::OK);
    assert!(content_type.starts_with("text/plain"), "got {content_type}");
    assert!(body.contains("Paperclip Agent Configuration Index"));
    assert!(body.contains("/llms/agent-icons.txt"));
}

#[tokio::test]
async fn agent_icons_lists_icon_names() {
    let app = app_with(Repositories::default());
    let (status, content_type, body) = get_text(&app, "/llms/agent-icons.txt", None).await;

    assert_eq!(status, StatusCode::OK);
    assert!(content_type.starts_with("text/plain"));
    assert!(body.contains("- bot"));
    assert!(body.contains("- search"));
    assert!(body.contains("- fingerprint"));
}

#[tokio::test]
async fn agent_without_permission_is_forbidden() {
    let keys = AgentKeyStore::default();
    keys.insert("k", "co-1");
    let app = app_with(Repositories {
        agent_keys: keys,
        ..Default::default()
    });
    let (status, _ct, body) = get_text(&app, "/llms/agent-icons.txt", Some("k")).await;
    assert_eq!(status, StatusCode::FORBIDDEN);
    assert!(body.contains("Board or permitted agent authentication required"));
}
