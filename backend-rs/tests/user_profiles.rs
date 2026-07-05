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

async fn post_activity(app: &Router, actor_id: &str, action: &str) {
    send(
        app,
        "POST",
        "/api/companies/co-1/activity",
        None,
        Some(json!({ "actorId": actor_id, "action": action, "entityType": "issue", "entityId": "i-1" })),
    )
    .await;
}

// Ports server/src/routes/user-profiles.ts (focused): a per-user rollup over the
// activity feed (the full cost/issue/agent aggregation is deferred).
#[tokio::test]
async fn profile_aggregates_user_activity() {
    let app = app_with(Repositories::default());
    post_activity(&app, "u-1", "issue.created").await;
    post_activity(&app, "u-1", "goal.updated").await;
    post_activity(&app, "u-2", "issue.created").await;

    let (status, profile) = send(
        &app,
        "GET",
        "/api/companies/co-1/users/u-1/profile",
        None,
        None,
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(profile["userId"], "u-1");
    assert_eq!(profile["companyId"], "co-1");
    assert_eq!(profile["activityCount"], 2);
    assert_eq!(profile["recentActivity"].as_array().unwrap().len(), 2);
    assert_eq!(profile["actionCounts"]["issue.created"], 1);
    assert_eq!(profile["actionCounts"]["goal.updated"], 1);
}

#[tokio::test]
async fn profile_for_user_with_no_activity_is_empty() {
    let app = app_with(Repositories::default());
    let (status, profile) = send(
        &app,
        "GET",
        "/api/companies/co-1/users/nobody/profile",
        None,
        None,
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(profile["userId"], "nobody");
    assert_eq!(profile["activityCount"], 0);
    assert_eq!(profile["recentActivity"], json!([]));
    assert_eq!(profile["actionCounts"], json!({}));
}

#[tokio::test]
async fn agent_cannot_read_other_company_profiles() {
    let keys = AgentKeyStore::default();
    keys.insert("k", "co-1");
    let app = app_with(Repositories {
        agent_keys: keys,
        ..Default::default()
    });
    let (status, body) = send(
        &app,
        "GET",
        "/api/companies/co-2/users/u-1/profile",
        Some("k"),
        None,
    )
    .await;
    assert_eq!(status, StatusCode::FORBIDDEN);
    assert_eq!(body, json!({ "error": "forbidden" }));
}
