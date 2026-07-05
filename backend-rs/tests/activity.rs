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

fn event(action: &str, entity_type: &str, entity_id: &str) -> Value {
    json!({
        "actorId": "u-1",
        "action": action,
        "entityType": entity_type,
        "entityId": entity_id,
    })
}

// Ports server/src/routes/activity.ts: company-scoped audit feed. POST is
// board-only; GET is company-scoped with entity/agent filters and a clamped limit.
#[tokio::test]
async fn board_creates_activity_event_with_defaults() {
    let app = app_with(Repositories::default());

    let (status, ev) = send(
        &app,
        "POST",
        "/api/companies/co-1/activity",
        None, // no bearer → board
        Some(event("issue.created", "issue", "i-1")),
    )
    .await;

    assert_eq!(status, StatusCode::CREATED);
    assert_eq!(ev["companyId"], "co-1");
    assert_eq!(ev["actorId"], "u-1");
    assert_eq!(ev["action"], "issue.created");
    assert_eq!(ev["actorType"], "system"); // schema default
    assert_eq!(ev["agentId"], Value::Null);
    assert_eq!(ev["runId"], Value::Null);
    assert_eq!(ev["details"], Value::Null);
    assert!(ev["id"].as_str().is_some_and(|s| !s.is_empty()));
}

#[tokio::test]
async fn agent_cannot_create_activity_even_for_own_company() {
    let keys = AgentKeyStore::default();
    keys.insert("k", "co-1");
    let app = app_with(Repositories {
        agent_keys: keys,
        ..Default::default()
    });

    let (status, body) = send(
        &app,
        "POST",
        "/api/companies/co-1/activity",
        Some("k"),
        Some(event("issue.created", "issue", "i-1")),
    )
    .await;

    assert_eq!(status, StatusCode::FORBIDDEN);
    assert_eq!(body, json!({ "error": "forbidden" }));
}

#[tokio::test]
async fn lists_company_activity_scoped() {
    let app = app_with(Repositories::default());
    send(
        &app,
        "POST",
        "/api/companies/co-1/activity",
        None,
        Some(event("a", "issue", "i-1")),
    )
    .await;
    send(
        &app,
        "POST",
        "/api/companies/co-1/activity",
        None,
        Some(event("b", "issue", "i-2")),
    )
    .await;

    let (status, list) = send(&app, "GET", "/api/companies/co-1/activity", None, None).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(list.as_array().unwrap().len(), 2);

    let (_, other) = send(&app, "GET", "/api/companies/co-2/activity", None, None).await;
    assert_eq!(other, json!([]));
}

#[tokio::test]
async fn filters_activity_by_entity_type() {
    let app = app_with(Repositories::default());
    send(
        &app,
        "POST",
        "/api/companies/co-1/activity",
        None,
        Some(event("a", "issue", "i-1")),
    )
    .await;
    send(
        &app,
        "POST",
        "/api/companies/co-1/activity",
        None,
        Some(event("b", "goal", "g-1")),
    )
    .await;

    let (status, list) = send(
        &app,
        "GET",
        "/api/companies/co-1/activity?entityType=goal",
        None,
        None,
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(list.as_array().unwrap().len(), 1);
    assert_eq!(list[0]["entityType"], "goal");
}

#[tokio::test]
async fn limit_query_caps_the_number_of_events() {
    let app = app_with(Repositories::default());
    for i in 0..3 {
        send(
            &app,
            "POST",
            "/api/companies/co-1/activity",
            None,
            Some(event("a", "issue", &format!("i-{i}"))),
        )
        .await;
    }

    let (status, list) = send(
        &app,
        "GET",
        "/api/companies/co-1/activity?limit=2",
        None,
        None,
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(list.as_array().unwrap().len(), 2);
}
