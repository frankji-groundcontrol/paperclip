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

// Ports server/src/routes/goals.ts: company-scoped goals with level/status
// defaults (packages/shared/src/validators/goal.ts).
#[tokio::test]
async fn creates_goal_under_company_with_defaults() {
    let app = app_with(Repositories::default());

    let (status, goal) = send(
        &app,
        "POST",
        "/api/companies/co-1/goals",
        None,
        Some(json!({ "title": "Ship v2" })),
    )
    .await;

    assert_eq!(status, StatusCode::CREATED);
    assert_eq!(goal["title"], "Ship v2");
    assert_eq!(goal["companyId"], "co-1");
    assert_eq!(goal["level"], "task"); // schema default
    assert_eq!(goal["status"], "planned"); // schema default
    assert_eq!(goal["description"], Value::Null);
    assert_eq!(goal["parentId"], Value::Null);
    assert!(goal["id"].as_str().is_some_and(|s| !s.is_empty()));
}

#[tokio::test]
async fn honors_explicit_level_status_and_parent() {
    let app = app_with(Repositories::default());

    let (_, goal) = send(
        &app,
        "POST",
        "/api/companies/co-1/goals",
        None,
        Some(json!({
            "title": "Grow revenue",
            "level": "company",
            "status": "active",
            "description": "north star",
            "parentId": "g-parent",
        })),
    )
    .await;

    assert_eq!(goal["level"], "company");
    assert_eq!(goal["status"], "active");
    assert_eq!(goal["description"], "north star");
    assert_eq!(goal["parentId"], "g-parent");
}

#[tokio::test]
async fn goals_are_isolated_by_company() {
    let app = app_with(Repositories::default());
    send(
        &app,
        "POST",
        "/api/companies/co-1/goals",
        None,
        Some(json!({ "title": "A" })),
    )
    .await;

    let (status, list) = send(&app, "GET", "/api/companies/co-1/goals", None, None).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(list.as_array().unwrap().len(), 1);

    let (_, other) = send(&app, "GET", "/api/companies/co-2/goals", None, None).await;
    assert_eq!(other, json!([]));
}

#[tokio::test]
async fn agent_cannot_access_other_company_goals() {
    let keys = AgentKeyStore::default();
    keys.insert("k", "co-1");
    let app = app_with(Repositories {
        agent_keys: keys,
        ..Default::default()
    });

    let (status, body) = send(&app, "GET", "/api/companies/co-2/goals", Some("k"), None).await;
    assert_eq!(status, StatusCode::FORBIDDEN);
    assert_eq!(body, json!({ "error": "forbidden" }));
}
