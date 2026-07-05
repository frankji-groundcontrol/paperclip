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

/// Security invariant: creating a secret returns only a reference — never the
/// value (server/src/services/secrets.ts never returns plaintext).
#[tokio::test]
async fn create_secret_returns_reference_without_value() {
    let app = app_with(Repositories::default());
    let (status, created) = send(
        &app,
        "POST",
        "/api/companies/co-1/secrets",
        None,
        Some(json!({ "name": "API_KEY", "value": "super-secret-value" })),
    )
    .await;

    assert_eq!(status, StatusCode::CREATED);
    assert_eq!(created["name"], "API_KEY");
    assert_eq!(created["companyId"], "co-1");
    assert_eq!(created["provider"], "local");
    assert!(created["id"].as_str().is_some_and(|s| !s.is_empty()));
    assert!(
        created.get("value").is_none(),
        "secret value must never be returned"
    );
    assert!(
        !created.to_string().contains("super-secret-value"),
        "response must not leak the secret value"
    );
}

/// Listing secrets returns references only, never values, and is company-scoped.
#[tokio::test]
async fn listing_secrets_never_includes_values() {
    let app = app_with(Repositories::default());
    send(
        &app,
        "POST",
        "/api/companies/co-1/secrets",
        None,
        Some(json!({ "name": "API_KEY", "value": "super-secret-value" })),
    )
    .await;

    let (status, list) = send(&app, "GET", "/api/companies/co-1/secrets", None, None).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(list.as_array().unwrap().len(), 1);
    assert!(list[0].get("value").is_none());
    assert!(!list.to_string().contains("super-secret-value"));

    let (_, other) = send(&app, "GET", "/api/companies/co-2/secrets", None, None).await;
    assert_eq!(other, json!([]));
}

/// Company-scoped authorization also applies to secrets.
#[tokio::test]
async fn agent_cannot_access_other_company_secrets() {
    let keys = AgentKeyStore::default();
    keys.insert("k", "co-1");
    let app = app_with(Repositories {
        agent_keys: keys,
        ..Default::default()
    });

    let (status, body) = send(&app, "GET", "/api/companies/co-2/secrets", Some("k"), None).await;
    assert_eq!(status, StatusCode::FORBIDDEN);
    assert_eq!(body, json!({ "error": "forbidden" }));
}
