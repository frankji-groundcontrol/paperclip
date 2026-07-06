use async_trait::async_trait;
use axum::{
    extract::{FromRef, FromRequestParts, State},
    http::{header::AUTHORIZATION, request::Parts, HeaderMap, StatusCode},
    routing::{get, post},
    Json, Router,
};
use serde::Deserialize;
use serde_json::{json, Map, Value};

use crate::{
    cli_auth::{CliAuthService, StartDeviceLogin},
    jobs::JobService,
};

use super::broker::{AuthBroker, Principal};

#[derive(Deserialize)]
struct Credentials {
    email: String,
    password: String,
}

pub fn auth_routes<S>() -> Router<S>
where
    AuthBroker: FromRef<S>,
    S: Clone + Send + Sync + 'static,
{
    Router::new()
        .route("/api/auth/register", post(register::<S>))
        .route("/api/auth/login", post(login::<S>))
        .route("/api/auth/logout", post(logout::<S>))
        .route("/api/auth/session", get(session))
}

pub fn paperclip_routes<S>() -> Router<S>
where
    JobService: FromRef<S>,
    S: Clone + Send + Sync + 'static,
{
    Router::new()
        .route(
            "/api/paperclip/companies",
            get(list_paperclip_companies::<S>).post(create_paperclip_company::<S>),
        )
        .route(
            "/api/paperclip/companies/:companyId/jobs",
            get(list_paperclip_jobs::<S>).post(run_paperclip_job::<S>),
        )
        .route(
            "/api/paperclip/companies/:companyId/agents",
            get(list_paperclip_agents::<S>).post(hire_paperclip_agent::<S>),
        )
        .route(
            "/api/paperclip/companies/:companyId/approvals",
            get(list_paperclip_approvals::<S>),
        )
        .route(
            "/api/paperclip/approvals/:approvalId/decide",
            post(decide_paperclip_approval::<S>),
        )
        // Phase 02: issues / goals / projects
        .route(
            "/api/paperclip/companies/:companyId/issues",
            get(list_paperclip_issues::<S>).post(create_paperclip_issue::<S>),
        )
        .route(
            "/api/paperclip/issues/:issueId/comments",
            get(list_paperclip_issue_comments::<S>).post(add_paperclip_issue_comment::<S>),
        )
        .route(
            "/api/paperclip/companies/:companyId/goals",
            get(list_paperclip_goals::<S>).post(create_paperclip_goal::<S>),
        )
        .route(
            "/api/paperclip/companies/:companyId/projects",
            get(list_paperclip_projects::<S>).post(create_paperclip_project::<S>),
        )
        // Phase 07: agent lifecycle + runs
        .route("/api/paperclip/agents/:agentId/pause", post(pause_paperclip_agent::<S>))
        .route("/api/paperclip/agents/:agentId/resume", post(resume_paperclip_agent::<S>))
        .route("/api/paperclip/agents/:agentId/terminate", post(terminate_paperclip_agent::<S>))
        .route(
            "/api/paperclip/companies/:companyId/runs",
            get(list_paperclip_runs::<S>).post(create_paperclip_run::<S>),
        )
        .route("/api/paperclip/runs/:runId/complete", post(complete_paperclip_run::<S>))
        // Phase 06: costs + dashboard + activity
        .route("/api/paperclip/companies/:companyId/costs", post(ingest_paperclip_cost::<S>))
        .route("/api/paperclip/companies/:companyId/cost-events", get(list_paperclip_cost_events::<S>))
        .route("/api/paperclip/companies/:companyId/dashboard", get(get_paperclip_dashboard::<S>))
        .route("/api/paperclip/companies/:companyId/activity", get(list_paperclip_activity::<S>))
}

pub fn cli_auth_routes<S>() -> Router<S>
where
    AuthBroker: FromRef<S>,
    CliAuthService: FromRef<S>,
    S: Clone + Send + Sync + 'static,
{
    Router::new()
        .route("/api/cli/start", post(start_cli_device_login::<S>))
        .route("/api/cli/poll", post(poll_cli_device_login::<S>))
        .route("/api/cli/approve", post(approve_cli_device_login::<S>))
}

async fn register<S>(
    State(broker): State<AuthBroker>,
    Json(payload): Json<Credentials>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)>
where
    AuthBroker: FromRef<S>,
    S: Send + Sync,
{
    let outcome = broker
        .register(&payload.email, &payload.password)
        .await
        .map_err(|_| {
            (
                StatusCode::UNAUTHORIZED,
                Json(json!({ "error": "registration_failed" })),
            )
        })?;

    let mut body = Map::new();
    body.insert(
        "needs_confirmation".to_string(),
        Value::Bool(outcome.needs_confirmation),
    );
    if let Some(user_id) = outcome.user_id {
        body.insert("user_id".to_string(), Value::String(user_id));
    }
    Ok(Json(Value::Object(body)))
}

async fn login<S>(
    State(broker): State<AuthBroker>,
    Json(payload): Json<Credentials>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)>
where
    AuthBroker: FromRef<S>,
    S: Send + Sync,
{
    let (session, whoami) = broker
        .login(&payload.email, &payload.password)
        .await
        .map_err(|_| {
            (
                StatusCode::UNAUTHORIZED,
                Json(json!({ "error": "invalid_credentials" })),
            )
        })?;

    Ok(Json(json!({
        "session": session,
        "whoami": whoami,
    })))
}

async fn logout<S>(
    State(broker): State<AuthBroker>,
    headers: HeaderMap,
) -> Result<Json<Value>, (StatusCode, Json<Value>)>
where
    AuthBroker: FromRef<S>,
    S: Send + Sync,
{
    let token = bearer_token_from_headers(&headers)?;
    broker.logout(token).await.map_err(|_| unauthorized())?;
    Ok(Json(json!({ "ok": true })))
}

#[async_trait]
impl<S> FromRequestParts<S> for Principal
where
    AuthBroker: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = (StatusCode, Json<Value>);

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let token = bearer_token(parts)?;
        AuthBroker::from_ref(state)
            .authenticate(token)
            .await
            .map_err(|_| unauthorized())
    }
}

async fn session(principal: Principal) -> Json<Value> {
    Json(principal_json(principal))
}

fn bearer_token(parts: &Parts) -> Result<&str, (StatusCode, Json<Value>)> {
    bearer_token_from_headers(&parts.headers)
}

fn bearer_token_from_headers(headers: &HeaderMap) -> Result<&str, (StatusCode, Json<Value>)> {
    let raw = headers
        .get(AUTHORIZATION)
        .and_then(|value| value.to_str().ok())
        .ok_or_else(unauthorized)?;
    raw.strip_prefix("Bearer ")
        .map(str::trim)
        .filter(|token| !token.is_empty())
        .ok_or_else(unauthorized)
}

#[derive(Deserialize)]
struct CreatePaperclipCompany {
    name: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct RunPaperclipJob {
    prompt: String,
    model: Option<String>,
    client_token: Option<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct HirePaperclipAgent {
    name: String,
    role: Option<String>,
    model: Option<String>,
    title: Option<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct DecidePaperclipApproval {
    approve: bool,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct StartCliDeviceLogin {
    secret_hash: String,
    user_code_hash: String,
    pending_key_prefix: String,
    pending_key_hash: String,
    pending_key_name: String,
    device_name: Option<String>,
    requested_access: Option<String>,
    team_id: Option<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct PollCliDeviceLogin {
    secret_hash: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ApproveCliDeviceLogin {
    user_code_hash: String,
}

async fn create_paperclip_company<S>(
    State(jobs): State<JobService>,
    headers: HeaderMap,
    Json(payload): Json<CreatePaperclipCompany>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)>
where
    JobService: FromRef<S>,
    S: Send + Sync,
{
    let bearer = bearer_token_from_headers(&headers)?;
    let company_id = jobs
        .create_company(bearer, &payload.name)
        .await
        .map_err(map_job_error)?;
    Ok(Json(json!({ "companyId": company_id })))
}

async fn start_cli_device_login<S>(
    State(cli_auth): State<CliAuthService>,
    Json(payload): Json<StartCliDeviceLogin>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)>
where
    CliAuthService: FromRef<S>,
    S: Send + Sync,
{
    cli_auth
        .start(StartDeviceLogin {
            secret_hash: payload.secret_hash,
            user_code_hash: payload.user_code_hash,
            pending_key_prefix: payload.pending_key_prefix,
            pending_key_hash: payload.pending_key_hash,
            pending_key_name: payload.pending_key_name,
            device_name: payload.device_name,
            requested_access: payload.requested_access,
            team_id: payload.team_id,
        })
        .await
        .map_err(map_job_error)?;
    Ok(Json(json!({ "ok": true })))
}

async fn poll_cli_device_login<S>(
    State(cli_auth): State<CliAuthService>,
    Json(payload): Json<PollCliDeviceLogin>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)>
where
    CliAuthService: FromRef<S>,
    S: Send + Sync,
{
    let row = cli_auth
        .poll(&payload.secret_hash)
        .await
        .map_err(map_job_error)?;
    Ok(Json(row))
}

async fn approve_cli_device_login<S>(
    State(cli_auth): State<CliAuthService>,
    headers: HeaderMap,
    principal: Principal,
    Json(payload): Json<ApproveCliDeviceLogin>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)>
where
    AuthBroker: FromRef<S>,
    CliAuthService: FromRef<S>,
    S: Send + Sync,
{
    if !matches!(principal, Principal::User { .. }) {
        return Err(unauthorized());
    }
    let bearer = bearer_token_from_headers(&headers)?;
    cli_auth
        .approve(bearer, &payload.user_code_hash)
        .await
        .map_err(map_job_error)?;
    Ok(Json(json!({ "ok": true })))
}

async fn list_paperclip_companies<S>(
    State(jobs): State<JobService>,
    headers: HeaderMap,
) -> Result<Json<Value>, (StatusCode, Json<Value>)>
where
    JobService: FromRef<S>,
    S: Send + Sync,
{
    let bearer = bearer_token_from_headers(&headers)?;
    let companies = jobs.list_companies(bearer).await.map_err(map_job_error)?;
    Ok(Json(json!({ "companies": companies })))
}

async fn run_paperclip_job<S>(
    State(jobs): State<JobService>,
    headers: HeaderMap,
    axum::extract::Path(company_id): axum::extract::Path<String>,
    Json(payload): Json<RunPaperclipJob>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)>
where
    JobService: FromRef<S>,
    S: Send + Sync,
{
    let bearer = bearer_token_from_headers(&headers)?;
    let job = jobs
        .run_job(
            bearer,
            &company_id,
            &payload.prompt,
            payload.model.as_deref(),
            payload.client_token.as_deref(),
        )
        .await
        .map_err(map_job_error)?;
    Ok(Json(job))
}

async fn list_paperclip_jobs<S>(
    State(jobs): State<JobService>,
    headers: HeaderMap,
    axum::extract::Path(company_id): axum::extract::Path<String>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)>
where
    JobService: FromRef<S>,
    S: Send + Sync,
{
    let bearer = bearer_token_from_headers(&headers)?;
    let jobs = jobs
        .list_jobs(bearer, &company_id)
        .await
        .map_err(map_job_error)?;
    Ok(Json(json!({ "jobs": jobs })))
}

async fn hire_paperclip_agent<S>(
    State(jobs): State<JobService>,
    headers: HeaderMap,
    axum::extract::Path(company_id): axum::extract::Path<String>,
    Json(payload): Json<HirePaperclipAgent>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)>
where
    JobService: FromRef<S>,
    S: Send + Sync,
{
    let bearer = bearer_token_from_headers(&headers)?;
    let agent = jobs
        .hire_agent(
            bearer,
            &company_id,
            &payload.name,
            payload.role.as_deref(),
            payload.model.as_deref(),
            payload.title.as_deref(),
        )
        .await
        .map_err(map_job_error)?;
    Ok(Json(agent))
}

async fn list_paperclip_agents<S>(
    State(jobs): State<JobService>,
    headers: HeaderMap,
    axum::extract::Path(company_id): axum::extract::Path<String>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)>
where
    JobService: FromRef<S>,
    S: Send + Sync,
{
    let bearer = bearer_token_from_headers(&headers)?;
    let agents = jobs
        .list_agents(bearer, &company_id)
        .await
        .map_err(map_job_error)?;
    Ok(Json(json!({ "agents": agents })))
}

async fn list_paperclip_approvals<S>(
    State(jobs): State<JobService>,
    headers: HeaderMap,
    axum::extract::Path(company_id): axum::extract::Path<String>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)>
where
    JobService: FromRef<S>,
    S: Send + Sync,
{
    let bearer = bearer_token_from_headers(&headers)?;
    let approvals = jobs
        .list_approvals(bearer, &company_id)
        .await
        .map_err(map_job_error)?;
    Ok(Json(json!({ "approvals": approvals })))
}

async fn decide_paperclip_approval<S>(
    State(jobs): State<JobService>,
    headers: HeaderMap,
    axum::extract::Path(approval_id): axum::extract::Path<String>,
    Json(payload): Json<DecidePaperclipApproval>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)>
where
    JobService: FromRef<S>,
    S: Send + Sync,
{
    let bearer = bearer_token_from_headers(&headers)?;
    let decision = jobs
        .decide_approval(bearer, &approval_id, payload.approve)
        .await
        .map_err(map_job_error)?;
    Ok(Json(decision))
}

// ===== Phase 02/06/07 handler functions =====

#[derive(serde::Deserialize)]
struct CreateIssueBody {
    title: String,
    #[serde(default)]
    parent_id: Option<String>,
    #[serde(default)]
    project_id: Option<String>,
    #[serde(default)]
    goal_id: Option<String>,
    #[serde(default)]
    assignee_agent_id: Option<String>,
    #[serde(default)]
    priority: Option<String>,
}

async fn create_paperclip_issue<S>(
    State(jobs): State<JobService>,
    headers: HeaderMap,
    axum::extract::Path(company_id): axum::extract::Path<String>,
    Json(payload): Json<CreateIssueBody>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)>
where JobService: FromRef<S>, S: Send + Sync,
{
    let bearer = bearer_token_from_headers(&headers)?;
    let result = jobs
        .create_issue(
            bearer, &company_id, &payload.title,
            payload.parent_id.as_deref(), payload.project_id.as_deref(),
            payload.goal_id.as_deref(), payload.assignee_agent_id.as_deref(),
            payload.priority.as_deref().unwrap_or("medium"),
        )
        .await.map_err(map_job_error)?;
    Ok(Json(result))
}

async fn list_paperclip_issues<S>(
    State(jobs): State<JobService>,
    headers: HeaderMap,
    axum::extract::Path(company_id): axum::extract::Path<String>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)>
where JobService: FromRef<S>, S: Send + Sync,
{
    let bearer = bearer_token_from_headers(&headers)?;
    let issues = jobs.list_issues(bearer, &company_id).await.map_err(map_job_error)?;
    Ok(Json(json!({ "issues": issues })))
}

#[derive(serde::Deserialize)]
struct AddCommentBody { body: String }

async fn add_paperclip_issue_comment<S>(
    State(jobs): State<JobService>,
    headers: HeaderMap,
    axum::extract::Path(issue_id): axum::extract::Path<String>,
    Json(payload): Json<AddCommentBody>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)>
where JobService: FromRef<S>, S: Send + Sync,
{
    let bearer = bearer_token_from_headers(&headers)?;
    let result = jobs.add_issue_comment(bearer, &issue_id, &payload.body).await.map_err(map_job_error)?;
    Ok(Json(result))
}

async fn list_paperclip_issue_comments<S>(
    State(jobs): State<JobService>,
    headers: HeaderMap,
    axum::extract::Path(issue_id): axum::extract::Path<String>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)>
where JobService: FromRef<S>, S: Send + Sync,
{
    let bearer = bearer_token_from_headers(&headers)?;
    let comments = jobs.list_issue_comments(bearer, &issue_id).await.map_err(map_job_error)?;
    Ok(Json(json!({ "comments": comments })))
}

#[derive(serde::Deserialize)]
struct CreateGoalBody { title: String, #[serde(default)] level: Option<String> }

async fn create_paperclip_goal<S>(
    State(jobs): State<JobService>,
    headers: HeaderMap,
    axum::extract::Path(company_id): axum::extract::Path<String>,
    Json(payload): Json<CreateGoalBody>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)>
where JobService: FromRef<S>, S: Send + Sync,
{
    let bearer = bearer_token_from_headers(&headers)?;
    let result = jobs.create_goal(bearer, &company_id, &payload.title, payload.level.as_deref().unwrap_or("task")).await.map_err(map_job_error)?;
    Ok(Json(result))
}

async fn list_paperclip_goals<S>(
    State(jobs): State<JobService>,
    headers: HeaderMap,
    axum::extract::Path(company_id): axum::extract::Path<String>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)>
where JobService: FromRef<S>, S: Send + Sync,
{
    let bearer = bearer_token_from_headers(&headers)?;
    let goals = jobs.list_goals(bearer, &company_id).await.map_err(map_job_error)?;
    Ok(Json(json!({ "goals": goals })))
}

#[derive(serde::Deserialize)]
struct CreateProjectBody { name: String }

async fn create_paperclip_project<S>(
    State(jobs): State<JobService>,
    headers: HeaderMap,
    axum::extract::Path(company_id): axum::extract::Path<String>,
    Json(payload): Json<CreateProjectBody>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)>
where JobService: FromRef<S>, S: Send + Sync,
{
    let bearer = bearer_token_from_headers(&headers)?;
    let result = jobs.create_project(bearer, &company_id, &payload.name).await.map_err(map_job_error)?;
    Ok(Json(result))
}

async fn list_paperclip_projects<S>(
    State(jobs): State<JobService>,
    headers: HeaderMap,
    axum::extract::Path(company_id): axum::extract::Path<String>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)>
where JobService: FromRef<S>, S: Send + Sync,
{
    let bearer = bearer_token_from_headers(&headers)?;
    let projects = jobs.list_projects(bearer, &company_id).await.map_err(map_job_error)?;
    Ok(Json(json!({ "projects": projects })))
}

async fn pause_paperclip_agent<S>(
    State(jobs): State<JobService>,
    headers: HeaderMap,
    axum::extract::Path(agent_id): axum::extract::Path<String>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)>
where JobService: FromRef<S>, S: Send + Sync,
{
    let bearer = bearer_token_from_headers(&headers)?;
    let result = jobs.pause_agent(bearer, &agent_id).await.map_err(map_job_error)?;
    Ok(Json(result))
}

async fn resume_paperclip_agent<S>(
    State(jobs): State<JobService>,
    headers: HeaderMap,
    axum::extract::Path(agent_id): axum::extract::Path<String>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)>
where JobService: FromRef<S>, S: Send + Sync,
{
    let bearer = bearer_token_from_headers(&headers)?;
    let result = jobs.resume_agent(bearer, &agent_id).await.map_err(map_job_error)?;
    Ok(Json(result))
}

async fn terminate_paperclip_agent<S>(
    State(jobs): State<JobService>,
    headers: HeaderMap,
    axum::extract::Path(agent_id): axum::extract::Path<String>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)>
where JobService: FromRef<S>, S: Send + Sync,
{
    let bearer = bearer_token_from_headers(&headers)?;
    let result = jobs.terminate_agent(bearer, &agent_id).await.map_err(map_job_error)?;
    Ok(Json(result))
}

#[derive(serde::Deserialize)]
struct CreateRunBody { agent_id: String }

async fn create_paperclip_run<S>(
    State(jobs): State<JobService>,
    headers: HeaderMap,
    axum::extract::Path(company_id): axum::extract::Path<String>,
    Json(payload): Json<CreateRunBody>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)>
where JobService: FromRef<S>, S: Send + Sync,
{
    let bearer = bearer_token_from_headers(&headers)?;
    let result = jobs.create_heartbeat_run(bearer, &company_id, &payload.agent_id).await.map_err(map_job_error)?;
    Ok(Json(result))
}

#[derive(serde::Deserialize)]
struct CompleteRunBody { result_text: String }

async fn complete_paperclip_run<S>(
    State(jobs): State<JobService>,
    headers: HeaderMap,
    axum::extract::Path(run_id): axum::extract::Path<String>,
    Json(payload): Json<CompleteRunBody>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)>
where JobService: FromRef<S>, S: Send + Sync,
{
    let bearer = bearer_token_from_headers(&headers)?;
    let result = jobs.complete_heartbeat_run(bearer, &run_id, &payload.result_text).await.map_err(map_job_error)?;
    Ok(Json(result))
}

async fn list_paperclip_runs<S>(
    State(jobs): State<JobService>,
    headers: HeaderMap,
    axum::extract::Path(company_id): axum::extract::Path<String>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)>
where JobService: FromRef<S>, S: Send + Sync,
{
    let bearer = bearer_token_from_headers(&headers)?;
    let runs = jobs.list_heartbeat_runs(bearer, &company_id).await.map_err(map_job_error)?;
    Ok(Json(json!({ "runs": runs })))
}

#[derive(serde::Deserialize)]
struct IngestCostBody {
    #[serde(default)]
    agent_id: Option<String>,
    #[serde(default)]
    input_tokens: i64,
    #[serde(default)]
    output_tokens: i64,
    #[serde(default)]
    cost_cents: i64,
}

async fn ingest_paperclip_cost<S>(
    State(jobs): State<JobService>,
    headers: HeaderMap,
    axum::extract::Path(company_id): axum::extract::Path<String>,
    Json(payload): Json<IngestCostBody>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)>
where JobService: FromRef<S>, S: Send + Sync,
{
    let bearer = bearer_token_from_headers(&headers)?;
    let result = jobs
        .ingest_cost_event(bearer, &company_id, payload.agent_id.as_deref(),
            payload.input_tokens, payload.output_tokens, payload.cost_cents)
        .await.map_err(map_job_error)?;
    Ok(Json(result))
}

async fn list_paperclip_cost_events<S>(
    State(jobs): State<JobService>,
    headers: HeaderMap,
    axum::extract::Path(company_id): axum::extract::Path<String>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)>
where JobService: FromRef<S>, S: Send + Sync,
{
    let bearer = bearer_token_from_headers(&headers)?;
    let costs = jobs.list_cost_events(bearer, &company_id).await.map_err(map_job_error)?;
    Ok(Json(json!({ "costEvents": costs })))
}

async fn get_paperclip_dashboard<S>(
    State(jobs): State<JobService>,
    headers: HeaderMap,
    axum::extract::Path(company_id): axum::extract::Path<String>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)>
where JobService: FromRef<S>, S: Send + Sync,
{
    let bearer = bearer_token_from_headers(&headers)?;
    let summary = jobs.get_dashboard_summary(bearer, &company_id).await.map_err(map_job_error)?;
    Ok(Json(summary))
}

async fn list_paperclip_activity<S>(
    State(jobs): State<JobService>,
    headers: HeaderMap,
    axum::extract::Path(company_id): axum::extract::Path<String>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)>
where JobService: FromRef<S>, S: Send + Sync,
{
    let bearer = bearer_token_from_headers(&headers)?;
    let activity = jobs.list_activity(bearer, &company_id).await.map_err(map_job_error)?;
    Ok(Json(json!({ "activity": activity })))
}

fn map_job_error(err: anyhow::Error) -> (StatusCode, Json<Value>) {
    let message = err.to_string();
    if message.contains("api key required") {
        return (
            StatusCode::UNAUTHORIZED,
            Json(json!({ "error": "api_key_required" })),
        );
    }
    if message.contains("session required") {
        return (
            StatusCode::UNAUTHORIZED,
            Json(json!({ "error": "unauthorized" })),
        );
    }
    if message.contains("28000") || message.contains("unknown session") {
        return (
            StatusCode::UNAUTHORIZED,
            Json(json!({ "error": "unauthorized" })),
        );
    }
    if message.contains("42501") {
        return (StatusCode::FORBIDDEN, Json(json!({ "error": "forbidden" })));
    }
    (
        StatusCode::BAD_GATEWAY,
        Json(json!({ "error": "paperclip_backend_unavailable" })),
    )
}

fn principal_json(principal: Principal) -> Value {
    match principal {
        Principal::User { whoami, .. } => {
            let mut object = match whoami {
                Value::Object(object) => object,
                other => {
                    let mut object = Map::new();
                    object.insert("whoami".to_string(), other);
                    object
                }
            };
            object.insert("kind".to_string(), Value::String("user".to_string()));
            Value::Object(object)
        }
        Principal::ApiKey {
            team_id,
            subject_type,
            agent_id,
            scopes,
            scope_config,
        } => {
            let mut object = Map::new();
            object.insert("kind".to_string(), Value::String("apiKey".to_string()));
            object.insert("teamId".to_string(), Value::String(team_id));
            object.insert("subjectType".to_string(), Value::String(subject_type));
            if let Some(agent_id) = agent_id {
                object.insert("agentId".to_string(), Value::String(agent_id));
            }
            object.insert("scopes".to_string(), scopes);
            object.insert("scopeConfig".to_string(), scope_config);
            Value::Object(object)
        }
    }
}

fn unauthorized() -> (StatusCode, Json<Value>) {
    (
        StatusCode::UNAUTHORIZED,
        Json(json!({ "error": "unauthorized" })),
    )
}
