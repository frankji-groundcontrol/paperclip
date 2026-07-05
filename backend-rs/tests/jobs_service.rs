use std::{
    sync::{Arc, Mutex},
};

use async_trait::async_trait;
use paperclip_backend::{
    jobs::JobService,
    llm::{LlmAnswer, LlmClient, LlmUsage},
    supabase::data::{Auth, DataGateway},
};
use serde_json::{json, Value};

const API_KEY: &str = "paperclip_pc_jobs_testsecret";

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
                let err = std::mem::replace(&mut *result, Ok(LlmAnswer {
                    text: String::new(),
                    model: String::new(),
                    usage: LlmUsage {
                        input_tokens: 0,
                        output_tokens: 0,
                        total_tokens: 0,
                    },
                }))
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
            other => anyhow::bail!("unexpected rpc {other}"),
        })
    }
}

#[tokio::test]
async fn run_job_api_key_happy_path_records_exact_rpcs() {
    let data = FakeData::default();
    let service = JobService::new(Arc::new(data.clone()), Arc::new(FakeLlm::succeeds()));

    let result = service
        .run_job(API_KEY, "company-1", "Reply with only the number 7", None, None)
        .await
        .unwrap();

    assert_eq!(result["jobId"], "job-1");
    assert_eq!(result["status"], "succeeded");
    assert_eq!(result["result"], "7");
    assert_eq!(result["usage"]["total_tokens"], 18);

    let calls = data.calls();
    assert_eq!(
        calls.iter().map(|call| call.name.as_str()).collect::<Vec<_>>(),
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
    );

    let result = service
        .run_job(API_KEY, "company-1", "Reply with only the number 7", None, None)
        .await
        .unwrap();

    assert_eq!(result["jobId"], "job-1");
    assert_eq!(result["status"], "failed");
    assert_eq!(result["error"], "llm_request_failed");

    let calls = data.calls();
    assert_eq!(
        calls.iter().map(|call| call.name.as_str()).collect::<Vec<_>>(),
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
    let service = JobService::new(Arc::new(data.clone()), Arc::new(FakeLlm::succeeds()));

    let company_id = service.create_company(API_KEY, "Acme").await.unwrap();

    assert_eq!(company_id, "company-1");
    let calls = data.calls();
    assert_eq!(calls.len(), 1);
    assert_eq!(calls[0].name, "create_company_with_key");
    assert_eq!(sorted_keys(&calls[0].body), vec!["p_key_hash", "p_name", "p_prefix"]);
    assert_eq!(calls[0].body["p_name"], "Acme");
    assert_eq!(calls[0].auth, Auth::Anon);
}

#[tokio::test]
async fn non_api_key_bearer_is_rejected_before_any_rpc() {
    let data = FakeData::default();
    let service = JobService::new(Arc::new(data.clone()), Arc::new(FakeLlm::succeeds()));

    let err = service
        .run_job("pcs_session_token", "company-1", "prompt", None, None)
        .await
        .unwrap_err();

    assert!(err.to_string().contains("api key required"));
    assert!(data.calls().is_empty());
}

#[tokio::test]
async fn list_methods_use_api_key_rpcs() {
    let data = FakeData::default();
    let service = JobService::new(Arc::new(data.clone()), Arc::new(FakeLlm::succeeds()));

    let companies = service.list_companies(API_KEY).await.unwrap();
    let jobs = service.list_jobs(API_KEY, "company-1").await.unwrap();

    assert_eq!(companies[0]["id"], "company-1");
    assert_eq!(jobs[0]["id"], "job-1");
    let calls = data.calls();
    assert_eq!(
        calls.iter().map(|call| call.name.as_str()).collect::<Vec<_>>(),
        vec!["list_companies_with_key", "list_jobs_with_key"]
    );
    assert_eq!(sorted_keys(&calls[0].body), vec!["p_key_hash", "p_prefix"]);
    assert_eq!(
        sorted_keys(&calls[1].body),
        vec!["p_company_id", "p_key_hash", "p_prefix"]
    );
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

