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

const PATH: &str = "/api/companies/co-1/sidebar-preferences/me";

// Ports server/src/routes/sidebar-preferences.ts (company project order): board
// user-scoped ordered ids with dedupe.
#[tokio::test]
async fn board_user_upserts_and_reads_project_order() {
    let app = app_with(Repositories::default());

    let (status, saved) = send(
        &app,
        "PUT",
        PATH,
        None,
        Some("u-1"),
        Some(json!({ "orderedIds": ["p-1", "p-2"] })),
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(saved["orderedIds"], json!(["p-1", "p-2"]));

    let (status, read) = send(&app, "GET", PATH, None, Some("u-1"), None).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(read["orderedIds"], json!(["p-1", "p-2"]));
}

#[tokio::test]
async fn unset_project_order_is_empty() {
    let app = app_with(Repositories::default());
    let (status, read) = send(&app, "GET", PATH, None, Some("u-1"), None).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(read, json!({ "orderedIds": [], "updatedAt": null }));
}

#[tokio::test]
async fn ordered_ids_are_deduped_preserving_first_occurrence() {
    let app = app_with(Repositories::default());
    let (_, saved) = send(
        &app,
        "PUT",
        PATH,
        None,
        Some("u-1"),
        Some(json!({ "orderedIds": ["p-1", "p-1", "p-2", "p-1"] })),
    )
    .await;
    assert_eq!(saved["orderedIds"], json!(["p-1", "p-2"]));
}

#[tokio::test]
async fn project_order_is_scoped_per_user() {
    let app = app_with(Repositories::default());
    send(
        &app,
        "PUT",
        PATH,
        None,
        Some("u-1"),
        Some(json!({ "orderedIds": ["p-1"] })),
    )
    .await;

    let (_, other) = send(&app, "GET", PATH, None, Some("u-2"), None).await;
    assert_eq!(other["orderedIds"], json!([]));
}

#[tokio::test]
async fn agent_is_forbidden() {
    let keys = AgentKeyStore::default();
    keys.insert("k", "co-1");
    let app = app_with(Repositories {
        agent_keys: keys,
        ..Default::default()
    });

    let (status, body) = send(&app, "GET", PATH, Some("k"), None, None).await;
    assert_eq!(status, StatusCode::FORBIDDEN);
    assert_eq!(body, json!({ "error": "Board authentication required" }));
}

#[tokio::test]
async fn board_without_user_is_forbidden() {
    let app = app_with(Repositories::default());
    let (status, body) = send(&app, "GET", PATH, None, None, None).await;
    assert_eq!(status, StatusCode::FORBIDDEN);
    assert_eq!(body, json!({ "error": "Board user context required" }));
}
