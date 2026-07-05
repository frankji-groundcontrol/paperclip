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
    user: Option<&str>,
    body: Option<Value>,
) -> (StatusCode, Value) {
    let mut builder = Request::builder().method(method).uri(uri);
    if let Some(token) = bearer {
        builder = builder.header(header::AUTHORIZATION, format!("Bearer {token}"));
    }
    if let Some(u) = user {
        builder = builder.header("x-actor-user", u);
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

// Ports server/src/routes/access.ts board-api-keys: per-user token CRUD. The
// plaintext token is revealed once on create and NEVER listed (no-leak).
#[tokio::test]
async fn creates_key_revealing_token_once() {
    let app = app_with(Repositories::default());
    let (status, key) = send(
        &app,
        "POST",
        "/api/board-api-keys",
        None,
        Some("u-1"),
        Some(json!({ "name": "cli" })),
    )
    .await;

    assert_eq!(status, StatusCode::CREATED);
    assert_eq!(key["name"], "cli");
    assert!(key["token"].as_str().is_some_and(|s| !s.is_empty()));
    assert!(key["id"].as_str().is_some_and(|s| !s.is_empty()));
}

#[tokio::test]
async fn list_never_includes_the_token() {
    let app = app_with(Repositories::default());
    let (_, created) = send(
        &app,
        "POST",
        "/api/board-api-keys",
        None,
        Some("u-1"),
        Some(json!({ "name": "cli" })),
    )
    .await;
    let token = created["token"].as_str().unwrap().to_string();

    let (status, list) = send(&app, "GET", "/api/board-api-keys", None, Some("u-1"), None).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(list.as_array().unwrap().len(), 1);
    assert!(
        list[0].get("token").is_none(),
        "listed keys must not carry the token"
    );
    assert!(
        !list.to_string().contains(&token),
        "response must not leak the token"
    );
}

#[tokio::test]
async fn deletes_key() {
    let app = app_with(Repositories::default());
    let (_, created) = send(
        &app,
        "POST",
        "/api/board-api-keys",
        None,
        Some("u-1"),
        Some(json!({ "name": "cli" })),
    )
    .await;
    let id = created["id"].as_str().unwrap();

    let (status, _) = send(
        &app,
        "DELETE",
        &format!("/api/board-api-keys/{id}"),
        None,
        Some("u-1"),
        None,
    )
    .await;
    assert_eq!(status, StatusCode::NO_CONTENT);

    let (_, list) = send(&app, "GET", "/api/board-api-keys", None, Some("u-1"), None).await;
    assert_eq!(list, json!([]));
}

#[tokio::test]
async fn delete_unknown_key_is_404() {
    let app = app_with(Repositories::default());
    let (status, body) = send(
        &app,
        "DELETE",
        "/api/board-api-keys/nope",
        None,
        Some("u-1"),
        None,
    )
    .await;
    assert_eq!(status, StatusCode::NOT_FOUND);
    assert_eq!(body, json!({ "error": "Board API key not found" }));
}

#[tokio::test]
async fn agent_gets_401() {
    let keys = AgentKeyStore::default();
    keys.insert("k", "co-1");
    let app = app_with(Repositories {
        agent_keys: keys,
        ..Default::default()
    });
    let (status, body) = send(&app, "GET", "/api/board-api-keys", Some("k"), None, None).await;
    assert_eq!(status, StatusCode::UNAUTHORIZED);
    assert_eq!(body, json!({ "error": "Board authentication required" }));
}

#[tokio::test]
async fn keys_scoped_per_user() {
    let app = app_with(Repositories::default());
    send(
        &app,
        "POST",
        "/api/board-api-keys",
        None,
        Some("u-1"),
        Some(json!({ "name": "cli" })),
    )
    .await;

    let (_, other) = send(&app, "GET", "/api/board-api-keys", None, Some("u-2"), None).await;
    assert_eq!(other, json!([]));
}
