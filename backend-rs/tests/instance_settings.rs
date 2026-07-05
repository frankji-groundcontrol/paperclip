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

// Ports server/src/routes/instance-settings.ts: an instance-wide settings
// singleton (general + experimental) with PATCH-merge semantics, board-only.
#[tokio::test]
async fn get_returns_defaults() {
    let app = app_with(Repositories::default());
    let (status, settings) = send(&app, "GET", "/api/instance/settings", None, None).await;

    assert_eq!(status, StatusCode::OK);
    assert_eq!(settings["defaultEnvironmentId"], Value::Null);
    assert_eq!(settings["general"]["keyboardShortcuts"], false);
    assert_eq!(
        settings["general"]["feedbackDataSharingPreference"],
        "prompt"
    );
    assert_eq!(settings["experimental"]["enableEnvironments"], false);
    // The one flag that defaults to true:
    assert_eq!(
        settings["experimental"]["enableStreamlinedLeftNavigation"],
        true
    );
}

#[tokio::test]
async fn patch_general_merges_only_provided_keys() {
    let app = app_with(Repositories::default());

    let (status, general) = send(
        &app,
        "PATCH",
        "/api/instance/settings/general",
        None,
        Some(json!({ "keyboardShortcuts": true })),
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(general["keyboardShortcuts"], true);
    assert_eq!(general["censorUsernameInLogs"], false); // untouched default

    // GET reflects the merge.
    let (_, again) = send(&app, "GET", "/api/instance/settings/general", None, None).await;
    assert_eq!(again["keyboardShortcuts"], true);
}

#[tokio::test]
async fn patch_experimental_merges_only_provided_keys() {
    let app = app_with(Repositories::default());
    let (status, experimental) = send(
        &app,
        "PATCH",
        "/api/instance/settings/experimental",
        None,
        Some(json!({ "enableEnvironments": true })),
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(experimental["enableEnvironments"], true);
    assert_eq!(experimental["enableStreamlinedLeftNavigation"], true); // untouched default
}

#[tokio::test]
async fn patch_settings_sets_default_environment() {
    let app = app_with(Repositories::default());
    send(
        &app,
        "PATCH",
        "/api/instance/settings",
        None,
        Some(json!({ "defaultEnvironmentId": "env-1" })),
    )
    .await;

    let (_, settings) = send(&app, "GET", "/api/instance/settings", None, None).await;
    assert_eq!(settings["defaultEnvironmentId"], "env-1");
}

#[tokio::test]
async fn agent_cannot_read_instance_settings() {
    let keys = AgentKeyStore::default();
    keys.insert("k", "co-1");
    let app = app_with(Repositories {
        agent_keys: keys,
        ..Default::default()
    });
    let (status, body) = send(&app, "GET", "/api/instance/settings", Some("k"), None).await;
    assert_eq!(status, StatusCode::FORBIDDEN);
    assert_eq!(body, json!({ "error": "forbidden" }));
}
