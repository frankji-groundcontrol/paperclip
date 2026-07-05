use axum::body::Body;
use axum::http::{header, Request, StatusCode};
use axum::Router;
use paperclip_backend::auth::AgentKeyStore;
use paperclip_backend::{app_with, Repositories};
use serde_json::{json, Value};
use tower::ServiceExt; // for `oneshot`

async fn get(app: &Router, uri: &str, bearer: Option<&str>) -> (StatusCode, Value) {
    let mut builder = Request::builder().method("GET").uri(uri);
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

#[tokio::test]
async fn agent_cannot_access_other_company_projects() {
    let app = app_with_agent("k", "co-1");
    let (status, body) = get(&app, "/api/companies/co-2/projects", Some("k")).await;
    assert_eq!(status, StatusCode::FORBIDDEN);
    assert_eq!(body, json!({ "error": "forbidden" }));
}

#[tokio::test]
async fn agent_cannot_access_other_company_agents() {
    let app = app_with_agent("k", "co-1");
    let (status, body) = get(&app, "/api/companies/co-2/agents", Some("k")).await;
    assert_eq!(status, StatusCode::FORBIDDEN);
    assert_eq!(body, json!({ "error": "forbidden" }));
}

#[tokio::test]
async fn agent_can_access_own_company_projects_and_agents() {
    let app = app_with_agent("k", "co-1");
    let (projects, _) = get(&app, "/api/companies/co-1/projects", Some("k")).await;
    let (agents, _) = get(&app, "/api/companies/co-1/agents", Some("k")).await;
    assert_eq!(projects, StatusCode::OK);
    assert_eq!(agents, StatusCode::OK);
}
