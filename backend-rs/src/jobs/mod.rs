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
                .get(
                    "my_companies?select=*&order=created_at.desc",
                    Auth::Bearer(jwt),
                )
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

    pub async fn hire_agent(
        &self,
        bearer: &str,
        company_id: &str,
        name: &str,
        role: Option<&str>,
        model: Option<&str>,
        title: Option<&str>,
    ) -> anyhow::Result<Value> {
        if let Some(jwt) = self.session_jwt(bearer).await? {
            return self
                .data
                .rpc(
                    "hire_agent",
                    json!({
                        "p_company_id": company_id,
                        "p_name": name,
                        "p_role": role,
                        "p_model": model,
                        "p_title": title,
                    }),
                    Auth::Bearer(jwt),
                )
                .await;
        }
        let key = api_key_parts(bearer)?;
        self.data
            .rpc(
                "hire_agent_with_key",
                json!({
                    "p_prefix": key.prefix,
                    "p_key_hash": key.key_hash,
                    "p_company_id": company_id,
                    "p_name": name,
                    "p_role": role,
                    "p_model": model,
                    "p_title": title,
                }),
                Auth::Anon,
            )
            .await
    }

    pub async fn list_agents(&self, bearer: &str, company_id: &str) -> anyhow::Result<Value> {
        if let Some(jwt) = self.session_jwt(bearer).await? {
            return self
                .data
                .get(
                    &format!("my_agents?company_id=eq.{company_id}&select=*&order=created_at.desc"),
                    Auth::Bearer(jwt),
                )
                .await;
        }
        let key = api_key_parts(bearer)?;
        self.data
            .rpc(
                "list_agents_with_key",
                json!({ "p_prefix": key.prefix, "p_key_hash": key.key_hash, "p_company_id": company_id }),
                Auth::Anon,
            )
            .await
    }

    pub async fn list_approvals(&self, bearer: &str, company_id: &str) -> anyhow::Result<Value> {
        if let Some(jwt) = self.session_jwt(bearer).await? {
            return self
                .data
                .get(
                    &format!(
                        "my_approvals?company_id=eq.{company_id}&select=*&order=created_at.desc"
                    ),
                    Auth::Bearer(jwt),
                )
                .await;
        }
        let key = api_key_parts(bearer)?;
        self.data
            .rpc(
                "list_approvals_with_key",
                json!({ "p_prefix": key.prefix, "p_key_hash": key.key_hash, "p_company_id": company_id }),
                Auth::Anon,
            )
            .await
    }

    pub async fn decide_approval(
        &self,
        bearer: &str,
        approval_id: &str,
        approve: bool,
    ) -> anyhow::Result<Value> {
        if let Some(jwt) = self.session_jwt(bearer).await? {
            return self
                .data
                .rpc(
                    "decide_approval",
                    json!({ "p_approval_id": approval_id, "p_approve": approve }),
                    Auth::Bearer(jwt),
                )
                .await;
        }
        let key = api_key_parts(bearer)?;
        self.data
            .rpc(
                "decide_approval_with_key",
                json!({
                    "p_prefix": key.prefix,
                    "p_key_hash": key.key_hash,
                    "p_approval_id": approval_id,
                    "p_approve": approve,
                }),
                Auth::Anon,
            )
            .await
    }

    pub async fn create_issue(
        &self,
        bearer: &str,
        company_id: &str,
        title: &str,
        parent_id: Option<&str>,
        project_id: Option<&str>,
        goal_id: Option<&str>,
        assignee_agent_id: Option<&str>,
        priority: &str,
    ) -> anyhow::Result<Value> {
        if let Some(jwt) = self.session_jwt(bearer).await? {
            return self
                .data
                .rpc(
                    "create_issue",
                    json!({
                        "p_company_id": company_id,
                        "p_title": title,
                        "p_parent_id": parent_id,
                        "p_project_id": project_id,
                        "p_goal_id": goal_id,
                        "p_assignee_agent_id": assignee_agent_id,
                        "p_priority": priority,
                    }),
                    Auth::Bearer(jwt),
                )
                .await;
        }
        let key = api_key_parts(bearer)?;
        self.data
            .rpc(
                "create_issue_with_key",
                json!({
                    "p_prefix": key.prefix,
                    "p_key_hash": key.key_hash,
                    "p_company_id": company_id,
                    "p_title": title,
                    "p_parent_id": parent_id,
                    "p_project_id": project_id,
                    "p_goal_id": goal_id,
                    "p_assignee_agent_id": assignee_agent_id,
                    "p_priority": priority,
                }),
                Auth::Anon,
            )
            .await
    }

    pub async fn list_issues(&self, bearer: &str, company_id: &str) -> anyhow::Result<Value> {
        if let Some(jwt) = self.session_jwt(bearer).await? {
            return self
                .data
                .get(
                    &format!(
                        "my_issues?company_id=eq.{company_id}&select=*&order=created_at.desc"
                    ),
                    Auth::Bearer(jwt),
                )
                .await;
        }
        let key = api_key_parts(bearer)?;
        self.data
            .rpc(
                "list_issues_with_key",
                json!({ "p_prefix": key.prefix, "p_key_hash": key.key_hash, "p_company_id": company_id }),
                Auth::Anon,
            )
            .await
    }

    pub async fn add_issue_comment(
        &self,
        bearer: &str,
        issue_id: &str,
        body: &str,
    ) -> anyhow::Result<Value> {
        if let Some(jwt) = self.session_jwt(bearer).await? {
            return self
                .data
                .rpc(
                    "add_issue_comment",
                    json!({ "p_issue_id": issue_id, "p_body": body }),
                    Auth::Bearer(jwt),
                )
                .await;
        }
        let key = api_key_parts(bearer)?;
        self.data
            .rpc(
                "add_issue_comment_with_key",
                json!({
                    "p_prefix": key.prefix,
                    "p_key_hash": key.key_hash,
                    "p_issue_id": issue_id,
                    "p_body": body,
                }),
                Auth::Anon,
            )
            .await
    }

    pub async fn list_issue_comments(
        &self,
        bearer: &str,
        issue_id: &str,
    ) -> anyhow::Result<Value> {
        if let Some(jwt) = self.session_jwt(bearer).await? {
            return self
                .data
                .get(
                    &format!(
                        "my_issue_comments?issue_id=eq.{issue_id}&select=*&order=created_at.asc"
                    ),
                    Auth::Bearer(jwt),
                )
                .await;
        }
        let key = api_key_parts(bearer)?;
        self.data
            .rpc(
                "list_issue_comments_with_key",
                json!({ "p_prefix": key.prefix, "p_key_hash": key.key_hash, "p_issue_id": issue_id }),
                Auth::Anon,
            )
            .await
    }

    pub async fn create_goal(
        &self,
        bearer: &str,
        company_id: &str,
        title: &str,
        level: &str,
    ) -> anyhow::Result<Value> {
        if let Some(jwt) = self.session_jwt(bearer).await? {
            return self
                .data
                .rpc(
                    "create_goal",
                    json!({ "p_company_id": company_id, "p_title": title, "p_level": level }),
                    Auth::Bearer(jwt),
                )
                .await;
        }
        let key = api_key_parts(bearer)?;
        self.data
            .rpc(
                "create_goal_with_key",
                json!({
                    "p_prefix": key.prefix,
                    "p_key_hash": key.key_hash,
                    "p_company_id": company_id,
                    "p_title": title,
                    "p_level": level,
                }),
                Auth::Anon,
            )
            .await
    }

    pub async fn list_goals(&self, bearer: &str, company_id: &str) -> anyhow::Result<Value> {
        if let Some(jwt) = self.session_jwt(bearer).await? {
            return self
                .data
                .get(
                    &format!("my_goals?company_id=eq.{company_id}&select=*&order=created_at.desc"),
                    Auth::Bearer(jwt),
                )
                .await;
        }
        let key = api_key_parts(bearer)?;
        self.data
            .rpc(
                "list_goals_with_key",
                json!({ "p_prefix": key.prefix, "p_key_hash": key.key_hash, "p_company_id": company_id }),
                Auth::Anon,
            )
            .await
    }

    pub async fn create_project(
        &self,
        bearer: &str,
        company_id: &str,
        name: &str,
    ) -> anyhow::Result<Value> {
        if let Some(jwt) = self.session_jwt(bearer).await? {
            return self
                .data
                .rpc(
                    "create_project",
                    json!({ "p_company_id": company_id, "p_name": name }),
                    Auth::Bearer(jwt),
                )
                .await;
        }
        let key = api_key_parts(bearer)?;
        self.data
            .rpc(
                "create_project_with_key",
                json!({
                    "p_prefix": key.prefix,
                    "p_key_hash": key.key_hash,
                    "p_company_id": company_id,
                    "p_name": name,
                }),
                Auth::Anon,
            )
            .await
    }

    pub async fn list_projects(&self, bearer: &str, company_id: &str) -> anyhow::Result<Value> {
        if let Some(jwt) = self.session_jwt(bearer).await? {
            return self
                .data
                .get(
                    &format!(
                        "my_projects?company_id=eq.{company_id}&select=*&order=created_at.desc"
                    ),
                    Auth::Bearer(jwt),
                )
                .await;
        }
        let key = api_key_parts(bearer)?;
        self.data
            .rpc(
                "list_projects_with_key",
                json!({ "p_prefix": key.prefix, "p_key_hash": key.key_hash, "p_company_id": company_id }),
                Auth::Anon,
            )
            .await
    }

    pub async fn pause_agent(&self, bearer: &str, agent_id: &str) -> anyhow::Result<Value> {
        if let Some(jwt) = self.session_jwt(bearer).await? {
            return self
                .data
                .rpc(
                    "pause_agent",
                    json!({ "p_agent_id": agent_id }),
                    Auth::Bearer(jwt),
                )
                .await;
        }
        let key = api_key_parts(bearer)?;
        self.data
            .rpc(
                "pause_agent_with_key",
                json!({ "p_prefix": key.prefix, "p_key_hash": key.key_hash, "p_agent_id": agent_id }),
                Auth::Anon,
            )
            .await
    }

    pub async fn resume_agent(&self, bearer: &str, agent_id: &str) -> anyhow::Result<Value> {
        if let Some(jwt) = self.session_jwt(bearer).await? {
            return self
                .data
                .rpc(
                    "resume_agent",
                    json!({ "p_agent_id": agent_id }),
                    Auth::Bearer(jwt),
                )
                .await;
        }
        let key = api_key_parts(bearer)?;
        self.data
            .rpc(
                "resume_agent_with_key",
                json!({ "p_prefix": key.prefix, "p_key_hash": key.key_hash, "p_agent_id": agent_id }),
                Auth::Anon,
            )
            .await
    }

    pub async fn terminate_agent(&self, bearer: &str, agent_id: &str) -> anyhow::Result<Value> {
        if let Some(jwt) = self.session_jwt(bearer).await? {
            return self
                .data
                .rpc(
                    "terminate_agent",
                    json!({ "p_agent_id": agent_id }),
                    Auth::Bearer(jwt),
                )
                .await;
        }
        let key = api_key_parts(bearer)?;
        self.data
            .rpc(
                "terminate_agent_with_key",
                json!({ "p_prefix": key.prefix, "p_key_hash": key.key_hash, "p_agent_id": agent_id }),
                Auth::Anon,
            )
            .await
    }

    pub async fn create_heartbeat_run(
        &self,
        bearer: &str,
        company_id: &str,
        agent_id: &str,
    ) -> anyhow::Result<Value> {
        if let Some(jwt) = self.session_jwt(bearer).await? {
            return self
                .data
                .rpc(
                    "create_heartbeat_run",
                    json!({ "p_company_id": company_id, "p_agent_id": agent_id }),
                    Auth::Bearer(jwt),
                )
                .await;
        }
        let key = api_key_parts(bearer)?;
        self.data
            .rpc(
                "create_heartbeat_run_with_key",
                json!({
                    "p_prefix": key.prefix,
                    "p_key_hash": key.key_hash,
                    "p_company_id": company_id,
                    "p_agent_id": agent_id,
                }),
                Auth::Anon,
            )
            .await
    }

    pub async fn complete_heartbeat_run(
        &self,
        bearer: &str,
        run_id: &str,
        result_text: &str,
    ) -> anyhow::Result<Value> {
        if let Some(jwt) = self.session_jwt(bearer).await? {
            return self
                .data
                .rpc(
                    "complete_heartbeat_run",
                    json!({ "p_run_id": run_id, "p_result_text": result_text }),
                    Auth::Bearer(jwt),
                )
                .await;
        }
        let key = api_key_parts(bearer)?;
        self.data
            .rpc(
                "complete_heartbeat_run_with_key",
                json!({
                    "p_prefix": key.prefix,
                    "p_key_hash": key.key_hash,
                    "p_run_id": run_id,
                    "p_result_text": result_text,
                }),
                Auth::Anon,
            )
            .await
    }

    pub async fn ingest_cost_event(
        &self,
        bearer: &str,
        company_id: &str,
        agent_id: Option<&str>,
        input_tokens: i64,
        output_tokens: i64,
        cost_cents: i64,
    ) -> anyhow::Result<Value> {
        if let Some(jwt) = self.session_jwt(bearer).await? {
            return self
                .data
                .rpc(
                    "ingest_cost_event",
                    json!({
                        "p_company_id": company_id,
                        "p_agent_id": agent_id,
                        "p_input_tokens": input_tokens,
                        "p_output_tokens": output_tokens,
                        "p_cost_cents": cost_cents,
                    }),
                    Auth::Bearer(jwt),
                )
                .await;
        }
        let key = api_key_parts(bearer)?;
        self.data
            .rpc(
                "ingest_cost_event_with_key",
                json!({
                    "p_prefix": key.prefix,
                    "p_key_hash": key.key_hash,
                    "p_company_id": company_id,
                    "p_agent_id": agent_id,
                    "p_input_tokens": input_tokens,
                    "p_output_tokens": output_tokens,
                    "p_cost_cents": cost_cents,
                }),
                Auth::Anon,
            )
            .await
    }

    pub async fn get_dashboard_summary(
        &self,
        bearer: &str,
        company_id: &str,
    ) -> anyhow::Result<Value> {
        if let Some(jwt) = self.session_jwt(bearer).await? {
            return self
                .data
                .rpc(
                    "get_dashboard_summary",
                    json!({ "p_company_id": company_id }),
                    Auth::Bearer(jwt),
                )
                .await;
        }
        let key = api_key_parts(bearer)?;
        self.data
            .rpc(
                "get_dashboard_summary_with_key",
                json!({ "p_prefix": key.prefix, "p_key_hash": key.key_hash, "p_company_id": company_id }),
                Auth::Anon,
            )
            .await
    }

    pub async fn list_activity(&self, bearer: &str, company_id: &str) -> anyhow::Result<Value> {
        if let Some(jwt) = self.session_jwt(bearer).await? {
            return self
                .data
                .get(
                    &format!(
                        "my_activity?company_id=eq.{company_id}&select=*&order=created_at.desc"
                    ),
                    Auth::Bearer(jwt),
                )
                .await;
        }
        let key = api_key_parts(bearer)?;
        self.data
            .rpc(
                "list_activity_with_key",
                json!({ "p_prefix": key.prefix, "p_key_hash": key.key_hash, "p_company_id": company_id }),
                Auth::Anon,
            )
            .await
    }

    pub async fn list_cost_events(&self, bearer: &str, company_id: &str) -> anyhow::Result<Value> {
        if let Some(jwt) = self.session_jwt(bearer).await? {
            return self
                .data
                .get(
                    &format!(
                        "my_cost_events?company_id=eq.{company_id}&select=*&order=occurred_at.desc"
                    ),
                    Auth::Bearer(jwt),
                )
                .await;
        }
        let key = api_key_parts(bearer)?;
        self.data
            .rpc(
                "list_cost_events_with_key",
                json!({ "p_prefix": key.prefix, "p_key_hash": key.key_hash, "p_company_id": company_id }),
                Auth::Anon,
            )
            .await
    }

    pub async fn list_heartbeat_runs(
        &self,
        bearer: &str,
        company_id: &str,
    ) -> anyhow::Result<Value> {
        if let Some(jwt) = self.session_jwt(bearer).await? {
            return self
                .data
                .get(
                    &format!(
                        "my_heartbeat_runs?company_id=eq.{company_id}&select=*&order=created_at.desc"
                    ),
                    Auth::Bearer(jwt),
                )
                .await;
        }
        let key = api_key_parts(bearer)?;
        self.data
            .rpc(
                "list_heartbeat_runs_with_key",
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
