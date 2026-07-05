use std::{
    sync::{Arc, Mutex},
    time::{Duration, SystemTime},
};

use async_trait::async_trait;
use paperclip_backend::{
    jobs::JobService,
    llm::{LlmAnswer, LlmClient, LlmUsage},
    supabase::{
        broker::{AuthBroker, InMemorySessionStore, SessionStore, StoredSession},
        data::{Auth, DataGateway},
        gateway::DisabledSupabaseGateway,
        keys::hash_full_key,
    },
};
use serde_json::{json, Value};

const API_KEY: &str = "paperclip_pc_jobs_testsecret";

/// An AuthBroker with a live `pcs_test` session (far-future expiry so no refresh
/// call is needed) — lets the session data path be unit-tested offline.
fn broker_with_session() -> AuthBroker {
    let store = InMemorySessionStore::default();
    store.put(
        "pcs_test".to_string(),
        StoredSession {
            access_token: "jwt-abc".to_string(),
            refresh_token: "refresh-abc".to_string(),
            expires_at: SystemTime::now() + Duration::from_secs(3600),
            user_id: "user-1".to_string(),
        },
    );
    AuthBroker::new(Arc::new(DisabledSupabaseGateway), Arc::new(store))
}

#[derive(Clone)]
struct FakeLlm {
    result: Arc<Mutex<anyhow::Result<LlmAnswer>>>,
}

impl FakeLlm {
    fn succeeds() -> Self {
        Self {
            result: Arc::new(Mutex::new(Ok(LlmAnswer {
                text: "7".to_string(),
                model: "gpt-5.4-mini-test".to_string(),
                usage: LlmUsage {
                    input_tokens: 13,
                    output_tokens: 5,
                    total_tokens: 18,
                },
            }))),
        }
    }

    fn fails_with_secretish_error() -> Self {
        Self {
            result: Arc::new(Mutex::new(Err(anyhow::anyhow!(
                "POST https://api.openai.example/v1/responses failed: Bearer sk-secret"
            )))),
        }
    }
}

#[async_trait]
impl LlmClient for FakeLlm {
    async fn respond(&self, _model: &str, _prompt: &str) -> anyhow::Result<LlmAnswer> {
        let mut result = self.result.lock().unwrap();
        match &*result {
            Ok(answer) => Ok(answer.clone()),
            Err(_) => {
                let err = std::mem::replace(
                    &mut *result,
                    Ok(LlmAnswer {
                        text: String::new(),
                        model: String::new(),
                        usage: LlmUsage {
                            input_tokens: 0,
                            output_tokens: 0,
                            total_tokens: 0,
                        },
                    }),
                )
                .unwrap_err();
                Err(err)
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
struct RpcCall {
    name: String,
    body: Value,
    auth: Auth,
}

#[derive(Clone, Default)]
struct FakeData {
    calls: Arc<Mutex<Vec<RpcCall>>>,
}

impl FakeData {
    fn calls(&self) -> Vec<RpcCall> {
        self.calls.lock().unwrap().clone()
    }
}

#[async_trait]
impl DataGateway for FakeData {
    async fn rpc(&self, name: &str, body: Value, auth: Auth) -> anyhow::Result<Value> {
        self.calls.lock().unwrap().push(RpcCall {
            name: name.to_string(),
            body,
            auth,
        });
        Ok(match name {
            "create_company_with_key" => json!("company-1"),
            "list_companies_with_key" => json!([{ "id": "company-1", "name": "Acme" }]),
            "create_job_with_key" => json!("job-1"),
            "complete_job_with_key" => json!(true),
            "fail_job_with_key" => json!(true),
            "list_jobs_with_key" => json!([{ "id": "job-1", "status": "succeeded" }]),
            "hire_agent_with_key" => json!({
                "agentId": "agent-1",
                "status": "pending_approval",
                "approvalId": "approval-1"
            }),
            "list_agents_with_key" => json!([{ "id": "agent-1", "name": "Ada" }]),
            "list_approvals_with_key" => json!([{ "id": "approval-1", "status": "pending" }]),
            "decide_approval_with_key" => json!({ "status": "approved" }),
            // session-path RPCs
            "whoami" => json!({ "user": { "default_team_id": "team-1" } }),
            "create_company" => json!("company-1"),
            "create_job" => json!("job-1"),
            "complete_job" => json!(true),
            "fail_job" => json!(true),
            "hire_agent" => json!({ "agentId": "agent-1", "status": "active" }),
            "decide_approval" => json!({ "status": "rejected" }),
            other => anyhow::bail!("unexpected rpc {other}"),
        })
    }

    async fn get(&self, path: &str, auth: Auth) -> anyhow::Result<Value> {
        self.calls.lock().unwrap().push(RpcCall {
            name: format!("GET {path}"),
            body: Value::Null,
            auth,
        });
        Ok(if path.starts_with("my_companies") {
            json!([{ "id": "company-1" }])
        } else if path.starts_with("my_agents") {
            json!([{ "id": "agent-1", "name": "Ada" }])
        } else if path.starts_with("my_approvals") {
            json!([{ "id": "approval-1", "status": "pending" }])
        } else {
            json!([{ "id": "job-1" }])
        })
    }
}

#[tokio::test]
async fn run_job_api_key_happy_path_records_exact_rpcs() {
    let data = FakeData::default();
    let service = JobService::new(
        Arc::new(data.clone()),
        Arc::new(FakeLlm::succeeds()),
        AuthBroker::default(),
    );

    let result = service
        .run_job(
            API_KEY,
            "company-1",
            "Reply with only the number 7",
            None,
            None,
        )
        .await
        .unwrap();

    assert_eq!(result["jobId"], "job-1");
    assert_eq!(result["status"], "succeeded");
    assert_eq!(result["result"], "7");
    assert_eq!(result["usage"]["total_tokens"], 18);

    let calls = data.calls();
    assert_eq!(
        calls
            .iter()
            .map(|call| call.name.as_str())
            .collect::<Vec<_>>(),
        vec!["create_job_with_key", "complete_job_with_key"]
    );
    assert_eq!(calls[0].auth, Auth::Anon);
    assert_eq!(calls[1].auth, Auth::Anon);
    assert_eq!(
        sorted_keys(&calls[0].body),
        vec![
            "p_client_token",
            "p_company_id",
            "p_key_hash",
            "p_model",
            "p_prefix",
            "p_prompt"
        ]
    );
    assert_eq!(
        sorted_keys(&calls[1].body),
        vec!["p_job_id", "p_key_hash", "p_prefix", "p_result", "p_usage"]
    );
}

#[tokio::test]
async fn run_job_api_key_llm_error_persists_only_sanitized_error() {
    let data = FakeData::default();
    let service = JobService::new(
        Arc::new(data.clone()),
        Arc::new(FakeLlm::fails_with_secretish_error()),
        AuthBroker::default(),
    );

    let result = service
        .run_job(
            API_KEY,
            "company-1",
            "Reply with only the number 7",
            None,
            None,
        )
        .await
        .unwrap();

    assert_eq!(result["jobId"], "job-1");
    assert_eq!(result["status"], "failed");
    assert_eq!(result["error"], "llm_request_failed");

    let calls = data.calls();
    assert_eq!(
        calls
            .iter()
            .map(|call| call.name.as_str())
            .collect::<Vec<_>>(),
        vec!["create_job_with_key", "fail_job_with_key"]
    );
    assert_eq!(calls[1].body["p_error"], "llm_request_failed");
    let persisted_error = calls[1].body["p_error"].as_str().unwrap();
    assert!(!persisted_error.contains("://"));
    assert!(!persisted_error.contains("Bearer"));
    assert!(!persisted_error.contains("sk-"));
}

#[tokio::test]
async fn create_company_uses_api_key_rpc_with_name_param() {
    let data = FakeData::default();
    let service = JobService::new(
        Arc::new(data.clone()),
        Arc::new(FakeLlm::succeeds()),
        AuthBroker::default(),
    );

    let company_id = service.create_company(API_KEY, "Acme").await.unwrap();

    assert_eq!(company_id, "company-1");
    let calls = data.calls();
    assert_eq!(calls.len(), 1);
    assert_eq!(calls[0].name, "create_company_with_key");
    assert_eq!(
        sorted_keys(&calls[0].body),
        vec!["p_key_hash", "p_name", "p_prefix"]
    );
    assert_eq!(calls[0].body["p_name"], "Acme");
    assert_eq!(calls[0].auth, Auth::Anon);
}

#[tokio::test]
async fn non_api_key_bearer_is_rejected_before_any_rpc() {
    let data = FakeData::default();
    let service = JobService::new(
        Arc::new(data.clone()),
        Arc::new(FakeLlm::succeeds()),
        AuthBroker::default(),
    );

    let err = service
        .run_job("garbage-bearer-token", "company-1", "prompt", None, None)
        .await
        .unwrap_err();

    assert!(err.to_string().contains("api key required"));
    assert!(data.calls().is_empty());
}

#[tokio::test]
async fn run_job_session_path_uses_session_rpcs_with_user_jwt() {
    let data = FakeData::default();
    let service = JobService::new(
        Arc::new(data.clone()),
        Arc::new(FakeLlm::succeeds()),
        broker_with_session(),
    );

    let result = service
        .run_job(
            "pcs_test",
            "company-1",
            "Reply with only the number 7",
            None,
            None,
        )
        .await
        .unwrap();

    assert_eq!(result["status"], "succeeded");
    assert_eq!(result["result"], "7");

    let calls = data.calls();
    // session path: create_job then complete_job (NOT the *_with_key variants),
    // each authorized with the user's JWT (never Anon).
    assert_eq!(
        calls.iter().map(|c| c.name.as_str()).collect::<Vec<_>>(),
        vec!["create_job", "complete_job"]
    );
    assert_eq!(calls[0].auth, Auth::Bearer("jwt-abc".to_string()));
    assert_eq!(calls[1].auth, Auth::Bearer("jwt-abc".to_string()));
}

#[tokio::test]
async fn create_company_session_path_resolves_default_team_then_creates() {
    let data = FakeData::default();
    let service = JobService::new(
        Arc::new(data.clone()),
        Arc::new(FakeLlm::succeeds()),
        broker_with_session(),
    );

    let id = service.create_company("pcs_test", "Acme").await.unwrap();
    assert_eq!(id, "company-1");

    let calls = data.calls();
    assert_eq!(
        calls.iter().map(|c| c.name.as_str()).collect::<Vec<_>>(),
        vec!["whoami", "create_company"]
    );
    assert_eq!(calls[1].body["p_team_id"], "team-1");
    assert_eq!(calls[1].body["p_name"], "Acme");
    assert_eq!(calls[1].auth, Auth::Bearer("jwt-abc".to_string()));
}

#[tokio::test]
async fn list_methods_use_api_key_rpcs() {
    let data = FakeData::default();
    let service = JobService::new(
        Arc::new(data.clone()),
        Arc::new(FakeLlm::succeeds()),
        AuthBroker::default(),
    );

    let companies = service.list_companies(API_KEY).await.unwrap();
    let jobs = service.list_jobs(API_KEY, "company-1").await.unwrap();

    assert_eq!(companies[0]["id"], "company-1");
    assert_eq!(jobs[0]["id"], "job-1");
    let calls = data.calls();
    assert_eq!(
        calls
            .iter()
            .map(|call| call.name.as_str())
            .collect::<Vec<_>>(),
        vec!["list_companies_with_key", "list_jobs_with_key"]
    );
    assert_eq!(sorted_keys(&calls[0].body), vec!["p_key_hash", "p_prefix"]);
    assert_eq!(
        sorted_keys(&calls[1].body),
        vec!["p_company_id", "p_key_hash", "p_prefix"]
    );
}

#[tokio::test]
async fn hire_agent_api_key_uses_with_key_rpc_and_exact_params() {
    let data = FakeData::default();
    let service = JobService::new(
        Arc::new(data.clone()),
        Arc::new(FakeLlm::succeeds()),
        AuthBroker::default(),
    );

    let result = service
        .hire_agent(
            API_KEY,
            "company-1",
            "Ada Lovelace",
            Some("engineer"),
            Some("gpt-5.4-mini"),
            Some("CTO"),
        )
        .await
        .unwrap();

    assert_eq!(result["agentId"], "agent-1");
    assert_eq!(result["status"], "pending_approval");
    assert_eq!(result["approvalId"], "approval-1");

    let calls = data.calls();
    assert_eq!(calls.len(), 1);
    assert_eq!(calls[0].name, "hire_agent_with_key");
    assert_eq!(calls[0].auth, Auth::Anon);
    assert_eq!(
        sorted_keys(&calls[0].body),
        vec![
            "p_company_id",
            "p_key_hash",
            "p_model",
            "p_name",
            "p_prefix",
            "p_role",
            "p_title"
        ]
    );
    assert_eq!(calls[0].body["p_company_id"], "company-1");
    assert_api_key_params(&calls[0].body);
    assert_eq!(calls[0].body["p_name"], "Ada Lovelace");
    assert_eq!(calls[0].body["p_role"], "engineer");
    assert_eq!(calls[0].body["p_model"], "gpt-5.4-mini");
    assert_eq!(calls[0].body["p_title"], "CTO");
}

#[tokio::test]
async fn hire_agent_session_path_uses_session_rpc_with_user_jwt() {
    let data = FakeData::default();
    let service = JobService::new(
        Arc::new(data.clone()),
        Arc::new(FakeLlm::succeeds()),
        broker_with_session(),
    );

    let result = service
        .hire_agent("pcs_test", "company-1", "Ada Lovelace", None, None, None)
        .await
        .unwrap();

    assert_eq!(result["agentId"], "agent-1");
    assert_eq!(result["status"], "active");

    let calls = data.calls();
    assert_eq!(calls.len(), 1);
    assert_eq!(calls[0].name, "hire_agent");
    assert_eq!(calls[0].auth, Auth::Bearer("jwt-abc".to_string()));
    assert_eq!(
        sorted_keys(&calls[0].body),
        vec!["p_company_id", "p_model", "p_name", "p_role", "p_title"]
    );
    assert_eq!(calls[0].body["p_company_id"], "company-1");
    assert_eq!(calls[0].body["p_name"], "Ada Lovelace");
    assert_eq!(calls[0].body["p_role"], Value::Null);
    assert_eq!(calls[0].body["p_model"], Value::Null);
    assert_eq!(calls[0].body["p_title"], Value::Null);
}

#[tokio::test]
async fn list_agent_and_approval_methods_use_api_key_rpcs() {
    let data = FakeData::default();
    let service = JobService::new(
        Arc::new(data.clone()),
        Arc::new(FakeLlm::succeeds()),
        AuthBroker::default(),
    );

    let agents = service.list_agents(API_KEY, "company-1").await.unwrap();
    let approvals = service.list_approvals(API_KEY, "company-1").await.unwrap();

    assert_eq!(agents[0]["id"], "agent-1");
    assert_eq!(approvals[0]["id"], "approval-1");
    let calls = data.calls();
    assert_eq!(
        calls
            .iter()
            .map(|call| call.name.as_str())
            .collect::<Vec<_>>(),
        vec!["list_agents_with_key", "list_approvals_with_key"]
    );
    assert_eq!(
        sorted_keys(&calls[0].body),
        vec!["p_company_id", "p_key_hash", "p_prefix"]
    );
    assert_eq!(calls[0].body["p_company_id"], "company-1");
    assert_api_key_params(&calls[0].body);
    assert_eq!(calls[0].auth, Auth::Anon);
    assert_eq!(
        sorted_keys(&calls[1].body),
        vec!["p_company_id", "p_key_hash", "p_prefix"]
    );
    assert_eq!(calls[1].body["p_company_id"], "company-1");
    assert_api_key_params(&calls[1].body);
    assert_eq!(calls[1].auth, Auth::Anon);
}

#[tokio::test]
async fn list_agent_and_approval_methods_use_session_views_with_user_jwt() {
    let data = FakeData::default();
    let service = JobService::new(
        Arc::new(data.clone()),
        Arc::new(FakeLlm::succeeds()),
        broker_with_session(),
    );

    let agents = service.list_agents("pcs_test", "company-1").await.unwrap();
    let approvals = service
        .list_approvals("pcs_test", "company-1")
        .await
        .unwrap();

    assert_eq!(agents[0]["id"], "agent-1");
    assert_eq!(approvals[0]["id"], "approval-1");
    let calls = data.calls();
    assert_eq!(
        calls
            .iter()
            .map(|call| call.name.as_str())
            .collect::<Vec<_>>(),
        vec![
            "GET my_agents?company_id=eq.company-1&select=*&order=created_at.desc",
            "GET my_approvals?company_id=eq.company-1&select=*&order=created_at.desc"
        ]
    );
    assert_eq!(calls[0].auth, Auth::Bearer("jwt-abc".to_string()));
    assert_eq!(calls[1].auth, Auth::Bearer("jwt-abc".to_string()));
}

#[tokio::test]
async fn decide_approval_api_key_uses_with_key_rpc_and_exact_params() {
    let data = FakeData::default();
    let service = JobService::new(
        Arc::new(data.clone()),
        Arc::new(FakeLlm::succeeds()),
        AuthBroker::default(),
    );

    let result = service
        .decide_approval(API_KEY, "approval-1", true)
        .await
        .unwrap();

    assert_eq!(result["status"], "approved");
    let calls = data.calls();
    assert_eq!(calls.len(), 1);
    assert_eq!(calls[0].name, "decide_approval_with_key");
    assert_eq!(
        sorted_keys(&calls[0].body),
        vec!["p_approval_id", "p_approve", "p_key_hash", "p_prefix"]
    );
    assert_eq!(calls[0].body["p_approval_id"], "approval-1");
    assert_eq!(calls[0].body["p_approve"], true);
    assert_api_key_params(&calls[0].body);
    assert_eq!(calls[0].auth, Auth::Anon);
}

#[tokio::test]
async fn decide_approval_session_path_uses_session_rpc_with_user_jwt() {
    let data = FakeData::default();
    let service = JobService::new(
        Arc::new(data.clone()),
        Arc::new(FakeLlm::succeeds()),
        broker_with_session(),
    );

    let result = service
        .decide_approval("pcs_test", "approval-1", false)
        .await
        .unwrap();

    assert_eq!(result["status"], "rejected");
    let calls = data.calls();
    assert_eq!(calls.len(), 1);
    assert_eq!(calls[0].name, "decide_approval");
    assert_eq!(
        sorted_keys(&calls[0].body),
        vec!["p_approval_id", "p_approve"]
    );
    assert_eq!(calls[0].body["p_approval_id"], "approval-1");
    assert_eq!(calls[0].body["p_approve"], false);
    assert_eq!(calls[0].auth, Auth::Bearer("jwt-abc".to_string()));
}

fn sorted_keys(value: &Value) -> Vec<&str> {
    let mut keys = value
        .as_object()
        .expect("object")
        .keys()
        .map(String::as_str)
        .collect::<Vec<_>>();
    keys.sort_unstable();
    keys
}

fn assert_api_key_params(body: &Value) {
    assert_eq!(body["p_prefix"], "pc_jobs");
    assert_eq!(body["p_key_hash"], hash_full_key(API_KEY));
}
