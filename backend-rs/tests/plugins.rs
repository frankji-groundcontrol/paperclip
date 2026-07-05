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

/// Ports `POST /api/companies/:companyId/plugins`: 201 with the installed plugin
/// (`pluginId` required, `enabled` defaults to true per plugin_company_settings).
#[tokio::test]
async fn installs_plugin_under_company() {
    let app = app_with(Repositories::default());
    let (status, created) = send(
        &app,
        "POST",
        "/api/companies/co-1/plugins",
        None,
        Some(json!({ "pluginId": "llm-wiki" })),
    )
    .await;

    assert_eq!(status, StatusCode::CREATED);
    assert_eq!(created["companyId"], "co-1");
    assert_eq!(created["pluginId"], "llm-wiki");
    assert_eq!(created["enabled"], true);
    assert!(created["id"].as_str().is_some_and(|s| !s.is_empty()));
}

/// Company scoping: one company's installed plugins are invisible to another.
#[tokio::test]
async fn plugins_are_isolated_by_company() {
    let app = app_with(Repositories::default());
    send(
        &app,
        "POST",
        "/api/companies/co-1/plugins",
        None,
        Some(json!({ "pluginId": "llm-wiki" })),
    )
    .await;

    let (status, list) = send(&app, "GET", "/api/companies/co-1/plugins", None, None).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(list.as_array().unwrap().len(), 1);

    let (_, other) = send(&app, "GET", "/api/companies/co-2/plugins", None, None).await;
    assert_eq!(other, json!([]));
}

/// Company-scoped authorization also applies to plugins.
#[tokio::test]
async fn agent_cannot_access_other_company_plugins() {
    let keys = AgentKeyStore::default();
    keys.insert("k", "co-1");
    let app = app_with(Repositories {
        agent_keys: keys,
        ..Default::default()
    });

    let (status, body) = send(&app, "GET", "/api/companies/co-2/plugins", Some("k"), None).await;
    assert_eq!(status, StatusCode::FORBIDDEN);
    assert_eq!(body, json!({ "error": "forbidden" }));
}
