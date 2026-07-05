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

/// A company's budget defaults to no limit and no spend.
#[tokio::test]
async fn budget_defaults_to_zero() {
    let app = app_with(Repositories::default());
    let (status, body) = send(&app, "GET", "/api/companies/co-1/budget", None, None).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(
        body,
        json!({ "companyId": "co-1", "monthlyLimitCents": 0, "spentCents": 0, "exceeded": false })
    );
}

/// The budget hard-stop invariant: once spend reaches the limit, `exceeded` flips.
#[tokio::test]
async fn budget_tracks_spend_and_hard_stops_at_limit() {
    let app = app_with(Repositories::default());
    send(
        &app,
        "POST",
        "/api/companies/co-1/budget/limit",
        None,
        Some(json!({ "monthlyLimitCents": 1000 })),
    )
    .await;

    let (_, after_first) = send(
        &app,
        "POST",
        "/api/companies/co-1/budget/spend",
        None,
        Some(json!({ "amountCents": 400 })),
    )
    .await;
    assert_eq!(after_first["spentCents"], 400);
    assert_eq!(after_first["exceeded"], false);

    let (_, after_second) = send(
        &app,
        "POST",
        "/api/companies/co-1/budget/spend",
        None,
        Some(json!({ "amountCents": 700 })),
    )
    .await;
    assert_eq!(after_second["spentCents"], 1100);
    assert_eq!(after_second["exceeded"], true);
}

/// Budgets are company-scoped: one company's spend does not affect another's.
#[tokio::test]
async fn budgets_are_company_scoped() {
    let app = app_with(Repositories::default());
    send(
        &app,
        "POST",
        "/api/companies/co-1/budget/spend",
        None,
        Some(json!({ "amountCents": 500 })),
    )
    .await;

    let (_, other) = send(&app, "GET", "/api/companies/co-2/budget", None, None).await;
    assert_eq!(other["spentCents"], 0);
}

/// Company-scoped authorization also applies to budgets.
#[tokio::test]
async fn agent_cannot_access_other_company_budget() {
    let keys = AgentKeyStore::default();
    keys.insert("k", "co-1");
    let app = app_with(Repositories {
        agent_keys: keys,
        ..Default::default()
    });

    let (status, body) = send(&app, "GET", "/api/companies/co-2/budget", Some("k"), None).await;
    assert_eq!(status, StatusCode::FORBIDDEN);
    assert_eq!(body, json!({ "error": "forbidden" }));
}
