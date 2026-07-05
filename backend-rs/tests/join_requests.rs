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

async fn create(app: &Router, body: Value) -> Value {
    let (status, jr) = send(
        app,
        "POST",
        "/api/companies/co-1/join-requests",
        None,
        Some(body),
    )
    .await;
    assert_eq!(status, StatusCode::CREATED);
    jr
}

// Ports server/src/routes/access.ts join-requests core: create (pending)/list/
// filter/approve/reject with the pending-only transition guard.
#[tokio::test]
async fn creates_pending_join_request() {
    let app = app_with(Repositories::default());
    let jr = create(
        &app,
        json!({ "requestType": "human", "requesterName": "Alice" }),
    )
    .await;

    assert_eq!(jr["companyId"], "co-1");
    assert_eq!(jr["requestType"], "human");
    assert_eq!(jr["requesterName"], "Alice");
    assert_eq!(jr["status"], "pending_approval");
    assert!(jr["id"].as_str().is_some_and(|s| !s.is_empty()));
}

#[tokio::test]
async fn lists_and_filters_join_requests() {
    let app = app_with(Repositories::default());
    create(&app, json!({ "requestType": "human" })).await;
    create(&app, json!({ "requestType": "agent" })).await;

    let (status, all) = send(&app, "GET", "/api/companies/co-1/join-requests", None, None).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(all.as_array().unwrap().len(), 2);

    let (_, agents) = send(
        &app,
        "GET",
        "/api/companies/co-1/join-requests?requestType=agent",
        None,
        None,
    )
    .await;
    assert_eq!(agents.as_array().unwrap().len(), 1);
    assert_eq!(agents[0]["requestType"], "agent");
}

#[tokio::test]
async fn approves_a_pending_request() {
    let app = app_with(Repositories::default());
    let jr = create(&app, json!({ "requestType": "human" })).await;
    let id = jr["id"].as_str().unwrap();

    let (status, approved) = send(
        &app,
        "POST",
        &format!("/api/companies/co-1/join-requests/{id}/approve"),
        None,
        None,
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(approved["status"], "approved");
}

#[tokio::test]
async fn approving_non_pending_is_conflict() {
    let app = app_with(Repositories::default());
    let jr = create(&app, json!({ "requestType": "human" })).await;
    let id = jr["id"].as_str().unwrap();
    send(
        &app,
        "POST",
        &format!("/api/companies/co-1/join-requests/{id}/approve"),
        None,
        None,
    )
    .await;

    let (status, body) = send(
        &app,
        "POST",
        &format!("/api/companies/co-1/join-requests/{id}/approve"),
        None,
        None,
    )
    .await;
    assert_eq!(status, StatusCode::CONFLICT);
    assert_eq!(body, json!({ "error": "Join request is not pending" }));
}

#[tokio::test]
async fn approve_unknown_is_404() {
    let app = app_with(Repositories::default());
    let (status, body) = send(
        &app,
        "POST",
        "/api/companies/co-1/join-requests/nope/approve",
        None,
        None,
    )
    .await;
    assert_eq!(status, StatusCode::NOT_FOUND);
    assert_eq!(body, json!({ "error": "Join request not found" }));
}

#[tokio::test]
async fn rejects_a_pending_request() {
    let app = app_with(Repositories::default());
    let jr = create(&app, json!({ "requestType": "agent" })).await;
    let id = jr["id"].as_str().unwrap();

    let (status, rejected) = send(
        &app,
        "POST",
        &format!("/api/companies/co-1/join-requests/{id}/reject"),
        None,
        None,
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(rejected["status"], "rejected");
}

#[tokio::test]
async fn agent_cannot_access_other_company_join_requests() {
    let keys = AgentKeyStore::default();
    keys.insert("k", "co-1");
    let app = app_with(Repositories {
        agent_keys: keys,
        ..Default::default()
    });
    let (status, body) = send(
        &app,
        "GET",
        "/api/companies/co-2/join-requests",
        Some("k"),
        None,
    )
    .await;
    assert_eq!(status, StatusCode::FORBIDDEN);
    assert_eq!(body, json!({ "error": "forbidden" }));
}
