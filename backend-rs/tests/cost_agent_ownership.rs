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

fn cost_event(agent_id: &str) -> Value {
    json!({
        "agentId": agent_id,
        "provider": "anthropic",
        "model": "claude",
        "costCents": 100,
        "occurredAt": "2026-07-01T00:00:00Z",
    })
}

fn app_with_agent_key() -> Router {
    let keys = AgentKeyStore::default();
    // Key "k" identifies agent "a-1" in company "co-1".
    keys.insert_agent("k", "co-1", "a-1");
    app_with(Repositories {
        agent_keys: keys,
        ..Default::default()
    })
}

/// An agent may report costs for itself.
#[tokio::test]
async fn agent_reports_its_own_cost() {
    let app = app_with_agent_key();
    let (status, _) = send(
        &app,
        "POST",
        "/api/companies/co-1/cost-events",
        Some("k"),
        Some(cost_event("a-1")),
    )
    .await;
    assert_eq!(status, StatusCode::CREATED);
}

/// An agent may NOT report costs attributed to a different agent.
#[tokio::test]
async fn agent_cannot_report_another_agents_cost() {
    let app = app_with_agent_key();
    let (status, body) = send(
        &app,
        "POST",
        "/api/companies/co-1/cost-events",
        Some("k"),
        Some(cost_event("a-2")),
    )
    .await;
    assert_eq!(status, StatusCode::FORBIDDEN);
    assert_eq!(
        body,
        json!({ "error": "Agent can only report its own costs" })
    );
}

/// whoami surfaces the agent identity when the key carries one.
#[tokio::test]
async fn whoami_includes_agent_id() {
    let app = app_with_agent_key();
    let (status, body) = send(&app, "GET", "/api/whoami", Some("k"), None).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(
        body,
        json!({ "actor": "agent", "companyId": "co-1", "agentId": "a-1" })
    );
}
