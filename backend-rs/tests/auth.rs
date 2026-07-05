use axum::body::Body;
use axum::http::{header, Request, StatusCode};
use axum::Router;
use paperclip_backend::auth::AgentKeyStore;
use paperclip_backend::{app_with, Repositories};
use serde_json::{json, Value};
use tower::ServiceExt; // for `oneshot`

async fn whoami(app: &Router, bearer: Option<&str>) -> (StatusCode, Value) {
    let mut builder = Request::builder().method("GET").uri("/api/whoami");
    if let Some(token) = bearer {
        builder = builder.header(header::AUTHORIZATION, format!("Bearer {token}"));
    }
    let response = app
        .clone()
        .oneshot(builder.body(Body::empty()).unwrap())
        .await
        .unwrap();
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

fn app_with_key(key: &str, company_id: &str) -> Router {
    let keys = AgentKeyStore::default();
    keys.insert(key, company_id);
    app_with(Repositories {
        agent_keys: keys,
        ..Default::default()
    })
}

/// Ports actor normalization (server/src/middleware/auth.ts): with no
/// Authorization header the caller is the full-control board operator.
#[tokio::test]
async fn whoami_without_auth_is_board() {
    let app = app_with(Repositories::default());
    let (status, body) = whoami(&app, None).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body, json!({ "actor": "board" }));
}

/// A valid bearer agent key resolves to that agent, scoped to its company.
#[tokio::test]
async fn whoami_with_valid_agent_key_is_agent() {
    let app = app_with_key("secret-key", "co-1");
    let (status, body) = whoami(&app, Some("secret-key")).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body, json!({ "actor": "agent", "companyId": "co-1" }));
}

/// An unknown bearer key is rejected (agent keys are validated, not ignored).
#[tokio::test]
async fn whoami_with_invalid_agent_key_is_unauthorized() {
    let app = app_with_key("secret-key", "co-1");
    let (status, body) = whoami(&app, Some("wrong-key")).await;
    assert_eq!(status, StatusCode::UNAUTHORIZED);
    assert_eq!(body, json!({ "error": "invalid_agent_key" }));
}
