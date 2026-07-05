use axum::body::Body;
use axum::http::{header, Request, StatusCode};
use axum::Router;
use paperclip_backend::auth::AgentKeyStore;
use paperclip_backend::{app_with, Repositories};
use serde_json::{json, Value};
use tower::ServiceExt; // for `oneshot`

async fn get_issues(app: &Router, company: &str, bearer: Option<&str>) -> (StatusCode, Value) {
    let mut builder = Request::builder()
        .method("GET")
        .uri(format!("/api/companies/{company}/issues"));
    if let Some(token) = bearer {
        builder = builder.header(header::AUTHORIZATION, format!("Bearer {token}"));
    }
    let response = app
        .clone()
        .oneshot(builder.body(Body::empty()).unwrap())
        .await
        .unwrap();
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

fn app_with_agent(key: &str, company_id: &str) -> Router {
    let keys = AgentKeyStore::default();
    keys.insert(key, company_id);
    app_with(Repositories {
        agent_keys: keys,
        ..Default::default()
    })
}

/// An agent may read its own company's issues.
#[tokio::test]
async fn agent_can_access_own_company_issues() {
    let app = app_with_agent("k", "co-1");
    let (status, _) = get_issues(&app, "co-1", Some("k")).await;
    assert_eq!(status, StatusCode::OK);
}

/// Company-scoping invariant: an agent is forbidden from another company's issues.
#[tokio::test]
async fn agent_cannot_access_other_company_issues() {
    let app = app_with_agent("k", "co-1");
    let (status, body) = get_issues(&app, "co-2", Some("k")).await;
    assert_eq!(status, StatusCode::FORBIDDEN);
    assert_eq!(body, json!({ "error": "forbidden" }));
}

/// The board operator may access any company.
#[tokio::test]
async fn board_can_access_any_company_issues() {
    let app = app_with(Repositories::default());
    let (status, _) = get_issues(&app, "co-9", None).await;
    assert_eq!(status, StatusCode::OK);
}
