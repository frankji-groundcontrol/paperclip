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

async fn make_project(app: &Router) -> String {
    let (_, p) = send(
        app,
        "POST",
        "/api/companies/co-1/projects",
        None,
        None,
        Some(json!({ "name": "Website" })),
    )
    .await;
    p["id"].as_str().unwrap().to_string()
}

// Ports server/src/routes/resource-memberships.ts (OSS-default policy):
// board-user-scoped project/agent membership state (joined/left).
#[tokio::test]
async fn lists_empty_memberships_by_default() {
    let app = app_with(Repositories::default());
    let (status, m) = send(
        &app,
        "GET",
        "/api/companies/co-1/resource-memberships/me",
        None,
        Some("u-1"),
        None,
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(
        m,
        json!({ "projectMemberships": {}, "agentMemberships": {}, "updatedAt": null })
    );
}

#[tokio::test]
async fn joins_and_leaves_a_project() {
    let app = app_with(Repositories::default());
    let project_id = make_project(&app).await;

    let (status, result) = send(
        &app,
        "PUT",
        &format!("/api/companies/co-1/resource-memberships/me/projects/{project_id}"),
        None,
        Some("u-1"),
        Some(json!({ "state": "left" })),
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(result["resourceType"], "project");
    assert_eq!(result["resourceId"], project_id);
    assert_eq!(result["state"], "left");

    let (_, m) = send(
        &app,
        "GET",
        "/api/companies/co-1/resource-memberships/me",
        None,
        Some("u-1"),
        None,
    )
    .await;
    assert_eq!(m["projectMemberships"][&project_id], "left");
}

#[tokio::test]
async fn update_membership_for_unknown_project_is_404() {
    let app = app_with(Repositories::default());
    let (status, body) = send(
        &app,
        "PUT",
        "/api/companies/co-1/resource-memberships/me/projects/nope",
        None,
        Some("u-1"),
        Some(json!({ "state": "left" })),
    )
    .await;
    assert_eq!(status, StatusCode::NOT_FOUND);
    assert_eq!(body, json!({ "error": "Project not found" }));
}

#[tokio::test]
async fn invalid_state_is_rejected() {
    let app = app_with(Repositories::default());
    let project_id = make_project(&app).await;
    let (status, body) = send(
        &app,
        "PUT",
        &format!("/api/companies/co-1/resource-memberships/me/projects/{project_id}"),
        None,
        Some("u-1"),
        Some(json!({ "state": "banana" })),
    )
    .await;
    assert_eq!(status, StatusCode::BAD_REQUEST);
    assert_eq!(body, json!({ "error": "Invalid membership state" }));
}

#[tokio::test]
async fn memberships_scoped_per_user() {
    let app = app_with(Repositories::default());
    let project_id = make_project(&app).await;
    send(
        &app,
        "PUT",
        &format!("/api/companies/co-1/resource-memberships/me/projects/{project_id}"),
        None,
        Some("u-1"),
        Some(json!({ "state": "left" })),
    )
    .await;

    let (_, other) = send(
        &app,
        "GET",
        "/api/companies/co-1/resource-memberships/me",
        None,
        Some("u-2"),
        None,
    )
    .await;
    assert_eq!(other["projectMemberships"], json!({}));
}

#[tokio::test]
async fn agent_actor_is_forbidden() {
    let keys = AgentKeyStore::default();
    keys.insert("k", "co-1");
    let app = app_with(Repositories {
        agent_keys: keys,
        ..Default::default()
    });
    let (status, body) = send(
        &app,
        "GET",
        "/api/companies/co-1/resource-memberships/me",
        Some("k"),
        None,
        None,
    )
    .await;
    assert_eq!(status, StatusCode::FORBIDDEN);
    assert_eq!(body, json!({ "error": "Board authentication required" }));
}
