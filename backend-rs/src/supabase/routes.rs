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
