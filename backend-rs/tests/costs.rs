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

fn cost_event(cost_cents: i64) -> Value {
    json!({
        "agentId": "a-1",
        "provider": "anthropic",
        "model": "claude",
        "costCents": cost_cents,
        "occurredAt": "2026-07-01T00:00:00Z",
    })
}

async fn make_company(app: &Router, budget: i64) -> String {
    let (_, co) = send(
        app,
        "POST",
        "/api/companies",
        None,
        Some(json!({ "name": "Acme", "budgetMonthlyCents": budget })),
    )
    .await;
    co["id"].as_str().unwrap().to_string()
}

// Ports server/src/routes/costs.ts cost-events + summary core.
#[tokio::test]
async fn reports_cost_event_with_defaults() {
    let app = app_with(Repositories::default());
    let company = make_company(&app, 0).await;

    let (status, ev) = send(
        &app,
        "POST",
        &format!("/api/companies/{company}/cost-events"),
        None,
        Some(cost_event(150)),
    )
    .await;

    assert_eq!(status, StatusCode::CREATED);
    assert_eq!(ev["companyId"], company);
    assert_eq!(ev["agentId"], "a-1");
    assert_eq!(ev["costCents"], 150);
    assert_eq!(ev["biller"], "anthropic"); // defaults to provider
    assert_eq!(ev["billingType"], "unknown"); // schema default
    assert_eq!(ev["inputTokens"], 0);
    assert!(ev["id"].as_str().is_some_and(|s| !s.is_empty()));
}

#[tokio::test]
async fn summary_sums_events_against_company_budget() {
    let app = app_with(Repositories::default());
    let company = make_company(&app, 1000).await;

    send(
        &app,
        "POST",
        &format!("/api/companies/{company}/cost-events"),
        None,
        Some(cost_event(150)),
    )
    .await;
    send(
        &app,
        "POST",
        &format!("/api/companies/{company}/cost-events"),
        None,
        Some(cost_event(350)),
    )
    .await;

    let (status, summary) = send(
        &app,
        "GET",
        &format!("/api/companies/{company}/costs/summary"),
        None,
        None,
    )
    .await;

    assert_eq!(status, StatusCode::OK);
    assert_eq!(summary["companyId"], company);
    assert_eq!(summary["spendCents"], 500);
    assert_eq!(summary["budgetCents"], 1000);
    assert_eq!(summary["utilizationPercent"], 50.0);
}

#[tokio::test]
async fn summary_for_unknown_company_is_404() {
    let app = app_with(Repositories::default());
    let (status, body) = send(&app, "GET", "/api/companies/nope/costs/summary", None, None).await;

    assert_eq!(status, StatusCode::NOT_FOUND);
    assert_eq!(body, json!({ "error": "Company not found" }));
}

#[tokio::test]
async fn agent_cannot_read_other_company_costs() {
    let keys = AgentKeyStore::default();
    keys.insert("k", "co-1");
    let app = app_with(Repositories {
        agent_keys: keys,
        ..Default::default()
    });

    let (status, body) = send(
        &app,
        "GET",
        "/api/companies/co-2/costs/summary",
        Some("k"),
        None,
    )
    .await;
    assert_eq!(status, StatusCode::FORBIDDEN);
    assert_eq!(body, json!({ "error": "forbidden" }));
}
