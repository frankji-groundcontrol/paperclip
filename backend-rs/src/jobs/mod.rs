use std::sync::Arc;

use serde_json::{json, Value};

use crate::{
    llm::{DisabledLlmClient, LlmAnswer, LlmClient, LlmUsage},
    supabase::{
        data::{Auth, DataGateway, DisabledDataGateway},
        keys::parse_api_key,
    },
};

#[derive(Clone)]
pub struct JobService {
    data: Arc<dyn DataGateway>,
    llm: Arc<dyn LlmClient>,
}

impl Default for JobService {
    fn default() -> Self {
        Self::new(Arc::new(DisabledDataGateway), Arc::new(DisabledLlmClient))
    }
}

impl JobService {
    pub fn new(data: Arc<dyn DataGateway>, llm: Arc<dyn LlmClient>) -> Self {
        Self { data, llm }
    }

    pub async fn create_company(&self, bearer: &str, name: &str) -> anyhow::Result<String> {
        let key = api_key_parts(bearer)?;
        let value = self
            .data
            .rpc(
                "create_company_with_key",
                json!({
                    "p_prefix": key.prefix,
                    "p_key_hash": key.key_hash,
                    "p_name": name,
                }),
                Auth::Anon,
            )
            .await?;
        value
            .as_str()
            .map(ToString::to_string)
            .ok_or_else(|| anyhow::anyhow!("invalid create_company response"))
    }

    pub async fn list_companies(&self, bearer: &str) -> anyhow::Result<Value> {
        let key = api_key_parts(bearer)?;
        self.data
            .rpc(
                "list_companies_with_key",
                json!({
                    "p_prefix": key.prefix,
                    "p_key_hash": key.key_hash,
                }),
                Auth::Anon,
            )
            .await
    }

    pub async fn run_job(
        &self,
        bearer: &str,
        company_id: &str,
        prompt: &str,
        model: Option<&str>,
        client_token: Option<&str>,
    ) -> anyhow::Result<Value> {
        let key = api_key_parts(bearer)?;
        // Resolve the model once (honoring OPENAI_MODEL via the client's default) so the
        // persisted jobs.model always matches the model actually invoked.
        let model = model
            .map(str::trim)
            .filter(|m| !m.is_empty())
            .unwrap_or_else(|| self.llm.default_model());
        let job_id = self.create_job(&key, company_id, prompt, model, client_token).await?;

        match self.llm.respond(model, prompt).await {
            Ok(answer) => self.complete_job(&key, &job_id, answer).await,
            Err(err) => {
                eprintln!("llm request failed: {err}");
                let sanitized = "llm_request_failed";
                self.data
                    .rpc(
                        "fail_job_with_key",
                        json!({
                            "p_prefix": key.prefix,
                            "p_key_hash": key.key_hash,
                            "p_job_id": job_id,
                            "p_error": sanitized,
                        }),
                        Auth::Anon,
                    )
                    .await?;
                Ok(json!({
                    "jobId": job_id,
                    "status": "failed",
                    "error": sanitized,
                }))
            }
        }
    }

    pub async fn list_jobs(&self, bearer: &str, company_id: &str) -> anyhow::Result<Value> {
        let key = api_key_parts(bearer)?;
        self.data
            .rpc(
                "list_jobs_with_key",
                json!({
                    "p_prefix": key.prefix,
                    "p_key_hash": key.key_hash,
                    "p_company_id": company_id,
                }),
                Auth::Anon,
            )
            .await
    }

    async fn create_job(
        &self,
        key: &KeyParts,
        company_id: &str,
        prompt: &str,
        model: &str,
        client_token: Option<&str>,
    ) -> anyhow::Result<String> {
        let value = self
            .data
            .rpc(
                "create_job_with_key",
                json!({
                    "p_prefix": key.prefix,
                    "p_key_hash": key.key_hash,
                    "p_company_id": company_id,
                    "p_prompt": prompt,
                    "p_model": model,
                    "p_client_token": client_token,
                }),
                Auth::Anon,
            )
            .await?;
        value
            .as_str()
            .map(ToString::to_string)
            .ok_or_else(|| anyhow::anyhow!("invalid create_job response"))
    }

    async fn complete_job(
        &self,
        key: &KeyParts,
        job_id: &str,
        answer: LlmAnswer,
    ) -> anyhow::Result<Value> {
        let ok = self
            .data
            .rpc(
                "complete_job_with_key",
                json!({
                    "p_prefix": key.prefix,
                    "p_key_hash": key.key_hash,
                    "p_job_id": job_id,
                    "p_result": answer.text,
                    "p_usage": usage_json(&answer.usage),
                }),
                Auth::Anon,
            )
            .await?;

        if ok.as_bool() == Some(false) {
            return Ok(json!({
                "jobId": job_id,
                "status": "failed",
            }));
        }

        Ok(json!({
            "jobId": job_id,
            "status": "succeeded",
            "result": answer.text,
            "usage": usage_json(&answer.usage),
        }))
    }
}

#[derive(Debug, Clone)]
struct KeyParts {
    prefix: String,
    key_hash: String,
}

fn api_key_parts(bearer: &str) -> anyhow::Result<KeyParts> {
    let parts = parse_api_key(bearer).ok_or_else(|| anyhow::anyhow!("api key required"))?;
    Ok(KeyParts {
        prefix: parts.prefix,
        key_hash: parts.key_hash,
    })
}

fn usage_json(usage: &LlmUsage) -> Value {
    json!({
        "input_tokens": usage.input_tokens,
        "output_tokens": usage.output_tokens,
        "total_tokens": usage.total_tokens,
    })
}

