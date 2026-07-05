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

// Ports server/src/routes/access.ts company invites (OSS default): create/list/
// filter-by-state/revoke of a shareable-token invite.
#[tokio::test]
async fn creates_invite_with_defaults() {
    let app = app_with(Repositories::default());
    let (status, invite) = send(
        &app,
        "POST",
        "/api/companies/co-1/invites",
        None,
        Some(json!({})),
    )
    .await;

    assert_eq!(status, StatusCode::CREATED);
    assert_eq!(invite["companyId"], "co-1");
    assert_eq!(invite["allowedJoinTypes"], "both"); // schema default
    assert_eq!(invite["state"], "active");
    assert!(invite["token"].as_str().is_some_and(|s| !s.is_empty()));
    assert!(invite["id"].as_str().is_some_and(|s| !s.is_empty()));
}

#[tokio::test]
async fn honors_explicit_join_types_and_human_role() {
    let app = app_with(Repositories::default());
    let (_, invite) = send(
        &app,
        "POST",
        "/api/companies/co-1/invites",
        None,
        Some(json!({ "allowedJoinTypes": "human", "humanRole": "member", "agentMessage": "welcome" })),
    )
    .await;
    assert_eq!(invite["allowedJoinTypes"], "human");
    assert_eq!(invite["humanRole"], "member");
    assert_eq!(invite["agentMessage"], "welcome");
}

#[tokio::test]
async fn lists_and_filters_invites_by_state() {
    let app = app_with(Repositories::default());
    let (_, invite) = send(
        &app,
        "POST",
        "/api/companies/co-1/invites",
        None,
        Some(json!({})),
    )
    .await;
    let id = invite["id"].as_str().unwrap();

    let (status, list) = send(&app, "GET", "/api/companies/co-1/invites", None, None).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(list.as_array().unwrap().len(), 1);

    // Revoke, then the state filter separates them.
    send(
        &app,
        "POST",
        &format!("/api/companies/co-1/invites/{id}/revoke"),
        None,
        None,
    )
    .await;

    let (_, active) = send(
        &app,
        "GET",
        "/api/companies/co-1/invites?state=active",
        None,
        None,
    )
    .await;
    assert_eq!(active, json!([]));
    let (_, revoked) = send(
        &app,
        "GET",
        "/api/companies/co-1/invites?state=revoked",
        None,
        None,
    )
    .await;
    assert_eq!(revoked.as_array().unwrap().len(), 1);
}

#[tokio::test]
async fn revoke_sets_state_and_unknown_is_404() {
    let app = app_with(Repositories::default());
    let (_, invite) = send(
        &app,
        "POST",
        "/api/companies/co-1/invites",
        None,
        Some(json!({})),
    )
    .await;
    let id = invite["id"].as_str().unwrap();

    let (status, revoked) = send(
        &app,
        "POST",
        &format!("/api/companies/co-1/invites/{id}/revoke"),
        None,
        None,
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(revoked["state"], "revoked");

    let (status, body) = send(
        &app,
        "POST",
        "/api/companies/co-1/invites/nope/revoke",
        None,
        None,
    )
    .await;
    assert_eq!(status, StatusCode::NOT_FOUND);
    assert_eq!(body, json!({ "error": "Invite not found" }));
}

#[tokio::test]
async fn agent_cannot_access_other_company_invites() {
    let keys = AgentKeyStore::default();
    keys.insert("k", "co-1");
    let app = app_with(Repositories {
        agent_keys: keys,
        ..Default::default()
    });
    let (status, body) = send(&app, "GET", "/api/companies/co-2/invites", Some("k"), None).await;
    assert_eq!(status, StatusCode::FORBIDDEN);
    assert_eq!(body, json!({ "error": "forbidden" }));
}
