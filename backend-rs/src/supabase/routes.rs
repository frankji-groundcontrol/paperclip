use async_trait::async_trait;
use axum::{
    extract::{FromRef, FromRequestParts, State},
    http::{header::AUTHORIZATION, request::Parts, HeaderMap, StatusCode},
    routing::{get, post},
    Json, Router,
};
use serde::Deserialize;
use serde_json::{json, Map, Value};

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
