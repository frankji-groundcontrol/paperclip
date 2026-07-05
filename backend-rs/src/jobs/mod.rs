use std::sync::Arc;

use serde_json::{json, Value};

use crate::{
    llm::{DisabledLlmClient, LlmAnswer, LlmClient, LlmUsage},
    supabase::{
        broker::AuthBroker,
        data::{Auth, DataGateway, DisabledDataGateway},
        keys::parse_api_key,
    },
};

/// Company/job orchestration. Dispatches on the bearer:
/// - `pcs_…` (browser session): use the session's GoTrue JWT (held server-side by the
///   `AuthBroker`) to call the session RPCs / `my_*` views — RLS applies as `auth.uid()`.
/// - `paperclip_…` (agent api key): the forge-proof key-credential RPCs (`*_with_key`).
#[derive(Clone)]
pub struct JobService {
    data: Arc<dyn DataGateway>,
    llm: Arc<dyn LlmClient>,
    auth: AuthBroker,
}

impl Default for JobService {
    fn default() -> Self {
        Self::new(
            Arc::new(DisabledDataGateway),
            Arc::new(DisabledLlmClient),
            AuthBroker::default(),
        )
    }
}

impl JobService {
    pub fn new(data: Arc<dyn DataGateway>, llm: Arc<dyn LlmClient>, auth: AuthBroker) -> Self {
        Self { data, llm, auth }
    }

    /// `Some(jwt)` for a valid `pcs_` session bearer; `None` for an api-key bearer.
    async fn session_jwt(&self, bearer: &str) -> anyhow::Result<Option<String>> {
        self.auth.session_access_token(bearer).await
    }

    async fn session_default_team(&self, jwt: &str) -> anyhow::Result<String> {
        let whoami = self
            .data
            .rpc("whoami", json!({}), Auth::Bearer(jwt.to_string()))
            .await?;
        whoami
            .get("user")
            .and_then(|u| u.get("default_team_id"))
            .and_then(Value::as_str)
            .map(ToString::to_string)
            .ok_or_else(|| anyhow::anyhow!("session user has no default team"))
    }

    pub async fn create_company(&self, bearer: &str, name: &str) -> anyhow::Result<String> {
        if let Some(jwt) = self.session_jwt(bearer).await? {
            let team = self.session_default_team(&jwt).await?;
            let value = self
                .data
                .rpc(
                    "create_company",
                    json!({ "p_team_id": team, "p_name": name }),
                    Auth::Bearer(jwt),
                )
                .await?;
            return as_id(value, "create_company");
        }
        let key = api_key_parts(bearer)?;
        let value = self
            .data
            .rpc(
                "create_company_with_key",
                json!({ "p_prefix": key.prefix, "p_key_hash": key.key_hash, "p_name": name }),
                Auth::Anon,
            )
            .await?;
        as_id(value, "create_company_with_key")
    }

    pub async fn list_companies(&self, bearer: &str) -> anyhow::Result<Value> {
        if let Some(jwt) = self.session_jwt(bearer).await? {
            return self
                .data
                .get("my_companies?select=*&order=created_at.desc", Auth::Bearer(jwt))
                .await;
        }
        let key = api_key_parts(bearer)?;
        self.data
            .rpc(
                "list_companies_with_key",
                json!({ "p_prefix": key.prefix, "p_key_hash": key.key_hash }),
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
        // Resolve the model once (honoring OPENAI_MODEL) so the persisted model matches the invoked one.
        let model = model
            .map(str::trim)
            .filter(|m| !m.is_empty())
            .unwrap_or_else(|| self.llm.default_model())
            .to_string();

        // Session path (browser user): create/complete via session RPCs with the user JWT.
        if let Some(jwt) = self.session_jwt(bearer).await? {
            let job_id = as_id(
                self.data
                    .rpc(
                        "create_job",
                        json!({
                            "p_company_id": company_id, "p_prompt": prompt,
                            "p_model": model, "p_client_token": client_token,
                        }),
                        Auth::Bearer(jwt.clone()),
                    )
                    .await?,
                "create_job",
            )?;
            return match self.llm.respond(&model, prompt).await {
                Ok(answer) => {
                    let ok = self
                        .data
                        .rpc(
                            "complete_job",
                            json!({
                                "p_job_id": job_id, "p_result": answer.text,
                                "p_usage": usage_json(&answer.usage),
                            }),
                            Auth::Bearer(jwt),
                        )
                        .await?;
                    Ok(job_result(&job_id, ok, answer))
                }
                Err(err) => {
                    eprintln!("llm request failed: {err}");
                    self.data
                        .rpc(
                            "fail_job",
                            json!({ "p_job_id": job_id, "p_error": "llm_request_failed" }),
                            Auth::Bearer(jwt),
                        )
                        .await?;
                    Ok(failed_json(&job_id))
                }
            };
        }

        // Api-key path (agent): forge-proof key-credential RPCs.
        let key = api_key_parts(bearer)?;
        let job_id = as_id(
            self.data
                .rpc(
                    "create_job_with_key",
                    json!({
                        "p_prefix": key.prefix, "p_key_hash": key.key_hash,
                        "p_company_id": company_id, "p_prompt": prompt,
                        "p_model": model, "p_client_token": client_token,
                    }),
                    Auth::Anon,
                )
                .await?,
            "create_job_with_key",
        )?;
        match self.llm.respond(&model, prompt).await {
            Ok(answer) => {
                let ok = self
                    .data
                    .rpc(
                        "complete_job_with_key",
                        json!({
                            "p_prefix": key.prefix, "p_key_hash": key.key_hash,
                            "p_job_id": job_id, "p_result": answer.text,
                            "p_usage": usage_json(&answer.usage),
                        }),
                        Auth::Anon,
                    )
                    .await?;
                Ok(job_result(&job_id, ok, answer))
            }
            Err(err) => {
                eprintln!("llm request failed: {err}");
                self.data
                    .rpc(
                        "fail_job_with_key",
                        json!({
                            "p_prefix": key.prefix, "p_key_hash": key.key_hash,
                            "p_job_id": job_id, "p_error": "llm_request_failed",
                        }),
                        Auth::Anon,
                    )
                    .await?;
                Ok(failed_json(&job_id))
            }
        }
    }

    pub async fn list_jobs(&self, bearer: &str, company_id: &str) -> anyhow::Result<Value> {
        if let Some(jwt) = self.session_jwt(bearer).await? {
            return self
                .data
                .get(
                    &format!("my_jobs?company_id=eq.{company_id}&select=*&order=created_at.desc"),
                    Auth::Bearer(jwt),
                )
                .await;
        }
        let key = api_key_parts(bearer)?;
        self.data
            .rpc(
                "list_jobs_with_key",
                json!({ "p_prefix": key.prefix, "p_key_hash": key.key_hash, "p_company_id": company_id }),
                Auth::Anon,
            )
            .await
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

fn as_id(value: Value, what: &str) -> anyhow::Result<String> {
    value
        .as_str()
        .map(ToString::to_string)
        .ok_or_else(|| anyhow::anyhow!("invalid {what} response"))
}

fn job_result(job_id: &str, complete_ok: Value, answer: LlmAnswer) -> Value {
    if complete_ok.as_bool() == Some(false) {
        return json!({ "jobId": job_id, "status": "failed" });
    }
    json!({
        "jobId": job_id,
        "status": "succeeded",
        "result": answer.text,
        "usage": usage_json(&answer.usage),
    })
}

fn failed_json(job_id: &str) -> Value {
    json!({ "jobId": job_id, "status": "failed", "error": "llm_request_failed" })
}

fn usage_json(usage: &LlmUsage) -> Value {
    json!({
        "input_tokens": usage.input_tokens,
        "output_tokens": usage.output_tokens,
        "total_tokens": usage.total_tokens,
    })
}
