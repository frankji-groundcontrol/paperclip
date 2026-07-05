use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use async_trait::async_trait;
use axum::{
    body::{to_bytes, Body},
    http::{header, Request, StatusCode},
};
use paperclip_backend::{
    jobs::JobService,
    llm::{LlmAnswer, LlmClient, LlmUsage},
    supabase::{
        broker::AuthBroker,
        data::{Auth, DataGateway},
        gateway::{GoTrueSession, ResolvedKey, SignUpOutcome, SupabaseGateway},
    },
    app_from_env, app_with, Repositories,
};
use serde_json::{json, Value};
use tower::ServiceExt;

const API_KEY: &str = "paperclip_pc_routes_testsecret";

#[derive(Clone)]
struct FakeLlm;

#[async_trait]
impl LlmClient for FakeLlm {
    async fn respond(&self, _model: &str, _prompt: &str) -> anyhow::Result<LlmAnswer> {
        Ok(LlmAnswer {
            text: "7".to_string(),
            model: "gpt-5.4-mini-test".to_string(),
            usage: LlmUsage {
                input_tokens: 13,
                output_tokens: 5,
                total_tokens: 18,
            },
        })
    }
}

#[derive(Clone, Default)]
struct FakeData {
    calls: Arc<Mutex<Vec<String>>>,
}

#[async_trait]
impl DataGateway for FakeData {
    async fn rpc(&self, name: &str, _body: Value, _auth: Auth) -> anyhow::Result<Value> {
        self.calls.lock().unwrap().push(name.to_string());
        Ok(match name {
            "create_company_with_key" => json!("company-1"),
            "list_companies_with_key" => json!([{ "id": "company-1", "name": "Acme" }]),
            "create_job_with_key" => json!("job-1"),
            "complete_job_with_key" => json!(true),
            "list_jobs_with_key" => json!([{ "id": "job-1", "status": "succeeded" }]),
            other => anyhow::bail!("unexpected rpc {other}"),
        })
    }
}

#[derive(Clone, Default)]
struct FakeGateway {
    keys: Arc<Mutex<HashMap<String, ResolvedKey>>>,
}

impl FakeGateway {
    fn with_key(prefix: &str) -> Self {
        let gateway = Self::default();
        gateway.keys.lock().unwrap().insert(
            prefix.to_string(),
            ResolvedKey {
                api_key_id: "key-1".to_string(),
                team_id: "team-1".to_string(),
                created_by: Some("user-1".to_string()),
                subject_type: "agent".to_string(),
                agent_id: Some("agent-1".to_string()),
                scopes: json!([]),
                scope_config: json!({}),
            },
        );
        gateway
    }
}

#[async_trait]
impl SupabaseGateway for FakeGateway {
    async fn sign_up(&self, _email: &str, _password: &str) -> anyhow::Result<SignUpOutcome> {
        anyhow::bail!("unused")
    }

    async fn sign_in_password(
        &self,
        _email: &str,
        _password: &str,
    ) -> anyhow::Result<GoTrueSession> {
        anyhow::bail!("unused")
    }

    async fn refresh(&self, _refresh_token: &str) -> anyhow::Result<GoTrueSession> {
        anyhow::bail!("unused")
    }

    async fn sign_out(&self, _access_token: &str) -> anyhow::Result<()> {
        anyhow::bail!("unused")
    }

    async fn whoami(&self, _access_token: &str) -> anyhow::Result<Value> {
        anyhow::bail!("unused")
    }

    async fn resolve_api_key(
        &self,
        prefix: &str,
        _key_hash: &str,
    ) -> anyhow::Result<Option<ResolvedKey>> {
        Ok(self.keys.lock().unwrap().get(prefix).cloned())
    }
}

#[tokio::test]
async fn paperclip_routes_are_mounted_without_colliding_with_in_memory_companies() {
    let app = route_app();

    let (paperclip_status, paperclip_body) = send(
        &app,
        "POST",
        "/api/paperclip/companies",
        Some(API_KEY),
        Some(json!({ "name": "Acme" })),
    )
    .await;
    let (legacy_status, legacy_body) =
        send(&app, "GET", "/api/companies", None, None).await;

    assert_eq!(paperclip_status, StatusCode::OK);
    assert_eq!(paperclip_body, json!({ "companyId": "company-1" }));
    assert_eq!(legacy_status, StatusCode::OK);
    assert!(legacy_body.as_array().is_some(), "legacy route returns array");
}

#[tokio::test]
async fn paperclip_job_routes_delegate_to_job_service_and_do_not_leak_internals() {
    let app = route_app();

    let (run_status, run_body) = send(
        &app,
        "POST",
        "/api/paperclip/companies/company-1/jobs",
        Some(API_KEY),
        Some(json!({
            "prompt": "Reply with only the number 7",
            "clientToken": "nonce-1"
        })),
    )
    .await;
    let (list_status, list_body) = send(
        &app,
        "GET",
        "/api/paperclip/companies/company-1/jobs",
        Some(API_KEY),
        None,
    )
    .await;

    assert_eq!(run_status, StatusCode::OK);
    assert_eq!(run_body["jobId"], "job-1");
    assert_eq!(run_body["status"], "succeeded");
    assert_eq!(run_body["result"], "7");
    assert_eq!(run_body["usage"]["total_tokens"], 18);
    assert_eq!(list_status, StatusCode::OK);
    assert_eq!(list_body["jobs"][0]["id"], "job-1");

    for body in [run_body, list_body] {
        let encoded = body.to_string();
        assert!(!encoded.contains("supabase"));
        assert!(!encoded.contains("openai"));
        assert!(!encoded.contains("://"));
        assert!(!encoded.contains("Bearer"));
        assert!(!encoded.contains("sk-"));
        assert!(!encoded.contains("key_hash"));
    }
}

#[tokio::test]
async fn paperclip_routes_reject_non_api_key_bearer_with_generic_401() {
    let app = route_app();

    let (status, body) = send(
        &app,
        "POST",
        "/api/paperclip/companies/company-1/jobs",
        Some("pcs_session_token"),
        Some(json!({ "prompt": "hello" })),
    )
    .await;

    assert_eq!(status, StatusCode::UNAUTHORIZED);
    assert_eq!(body, json!({ "error": "api_key_required" }));
}

#[tokio::test]
async fn app_from_env_builds_router_without_contacting_network() {
    let app = app_from_env();
    let (status, body) = send(&app, "GET", "/api/health", None, None).await;

    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["status"], "ok");
}

fn route_app() -> axum::Router {
    app_with(Repositories {
        jobs: JobService::new(Arc::new(FakeData::default()), Arc::new(FakeLlm)),
        supabase_auth: AuthBroker::new(
            Arc::new(FakeGateway::with_key("pc_routes")),
            Arc::new(paperclip_backend::supabase::broker::InMemorySessionStore::default()),
        ),
        ..Default::default()
    })
}

async fn send(
    app: &axum::Router,
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
        Some(body) => builder
            .header(header::CONTENT_TYPE, "application/json")
            .body(Body::from(body.to_string()))
            .unwrap(),
        None => builder.body(Body::empty()).unwrap(),
    };
    let response = app.clone().oneshot(request).await.unwrap();
    let status = response.status();
    let bytes = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let body = if bytes.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&bytes).unwrap()
    };
    (status, body)
}

