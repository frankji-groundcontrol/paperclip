use axum::body::Body;
use axum::http::{header, Request, StatusCode};
use axum::Router;
use paperclip_backend::auth::AgentKeyStore;
use paperclip_backend::{app_with, Repositories};
use serde_json::{json, Value};
use tower::ServiceExt; // for `oneshot`

/// Sends a request with optional bearer token and optional board user header.
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

// Ports server/src/routes/inbox-dismissals.ts: board-user-scoped inbox
// dismissals with itemKey validation.
#[tokio::test]
async fn board_user_dismisses_and_lists_inbox_item() {
    let app = app_with(Repositories::default());

    let (status, dismissal) = send(
        &app,
        "POST",
        "/api/companies/co-1/inbox-dismissals",
        None,
        Some("u-1"),
        Some(json!({ "itemKey": "approval:apr-1" })),
    )
    .await;
    assert_eq!(status, StatusCode::CREATED);
    assert_eq!(dismissal["itemKey"], "approval:apr-1");
    assert!(dismissal["dismissedAt"]
        .as_str()
        .is_some_and(|s| !s.is_empty()));

    let (status, list) = send(
        &app,
        "GET",
        "/api/companies/co-1/inbox-dismissals",
        None,
        Some("u-1"),
        None,
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(list.as_array().unwrap().len(), 1);
    assert_eq!(list[0]["itemKey"], "approval:apr-1");
}

#[tokio::test]
async fn dismissals_are_scoped_per_user() {
    let app = app_with(Repositories::default());
    send(
        &app,
        "POST",
        "/api/companies/co-1/inbox-dismissals",
        None,
        Some("u-1"),
        Some(json!({ "itemKey": "run:r-1" })),
    )
    .await;

    // A different board user sees none of u-1's dismissals.
    let (_, other) = send(
        &app,
        "GET",
        "/api/companies/co-1/inbox-dismissals",
        None,
        Some("u-2"),
        None,
    )
    .await;
    assert_eq!(other, json!([]));
}

#[tokio::test]
async fn agent_gets_board_authentication_required() {
    let keys = AgentKeyStore::default();
    keys.insert("k", "co-1");
    let app = app_with(Repositories {
        agent_keys: keys,
        ..Default::default()
    });

    let (status, body) = send(
        &app,
        "POST",
        "/api/companies/co-1/inbox-dismissals",
        Some("k"),
        None,
        Some(json!({ "itemKey": "approval:apr-1" })),
    )
    .await;
    assert_eq!(status, StatusCode::FORBIDDEN);
    assert_eq!(body, json!({ "error": "Board authentication required" }));
}

#[tokio::test]
async fn board_without_user_context_is_forbidden() {
    let app = app_with(Repositories::default());
    let (status, body) = send(
        &app,
        "POST",
        "/api/companies/co-1/inbox-dismissals",
        None,
        None, // board, but no user header
        Some(json!({ "itemKey": "approval:apr-1" })),
    )
    .await;
    assert_eq!(status, StatusCode::FORBIDDEN);
    assert_eq!(body, json!({ "error": "Board user context required" }));
}

#[tokio::test]
async fn invalid_item_key_is_rejected() {
    let app = app_with(Repositories::default());
    let (status, body) = send(
        &app,
        "POST",
        "/api/companies/co-1/inbox-dismissals",
        None,
        Some("u-1"),
        Some(json!({ "itemKey": "task:t-1" })), // unsupported prefix
    )
    .await;
    assert_eq!(status, StatusCode::BAD_REQUEST);
    assert_eq!(body, json!({ "error": "Unsupported inbox item key" }));
}

/// whoami surfaces the board user id when the header is present.
#[tokio::test]
async fn whoami_includes_board_user_id() {
    let app = app_with(Repositories::default());
    let (status, body) = send(&app, "GET", "/api/whoami", None, Some("u-9"), None).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body, json!({ "actor": "board", "userId": "u-9" }));
}
