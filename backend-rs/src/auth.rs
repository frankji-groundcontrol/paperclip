use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use axum::{
    async_trait,
    extract::{FromRef, FromRequestParts},
    http::{header::AUTHORIZATION, request::Parts, StatusCode},
    Json,
};
use serde_json::{json, Value};

/// The authenticated caller. Ports the actor model of
/// server/src/middleware/auth.ts: a full-control board operator, or an agent
/// scoped to its company (and optionally identified by `agent_id`). (The `none`
/// case is modeled as a rejection here.)
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Actor {
    Board {
        user_id: Option<String>,
    },
    Agent {
        company_id: String,
        agent_id: Option<String>,
    },
}

/// What a bearer agent key resolves to: the company it is scoped to, and
/// optionally the specific agent it identifies.
#[derive(Clone)]
struct AgentIdentity {
    company_id: String,
    agent_id: Option<String>,
}

/// Maps bearer agent keys to their company (and optional agent identity).
/// In-memory for now; a hashed, Postgres-backed lookup (`agent_api_keys`)
/// replaces it in a later slice.
#[derive(Clone, Default)]
pub struct AgentKeyStore {
    inner: Arc<Mutex<HashMap<String, AgentIdentity>>>,
}

impl AgentKeyStore {
    /// Registers a company-scoped key with no specific agent identity.
    pub fn insert(&self, key: &str, company_id: &str) {
        self.inner.lock().unwrap().insert(
            key.to_string(),
            AgentIdentity {
                company_id: company_id.to_string(),
                agent_id: None,
            },
        );
    }

    /// Registers a key that identifies a specific agent within a company.
    pub fn insert_agent(&self, key: &str, company_id: &str, agent_id: &str) {
        self.inner.lock().unwrap().insert(
            key.to_string(),
            AgentIdentity {
                company_id: company_id.to_string(),
                agent_id: Some(agent_id.to_string()),
            },
        );
    }

    pub fn company_for(&self, key: &str) -> Option<String> {
        self.inner
            .lock()
            .unwrap()
            .get(key)
            .map(|identity| identity.company_id.clone())
    }

    /// Resolves a key to its (company_id, agent_id) identity.
    fn identity_for(&self, key: &str) -> Option<(String, Option<String>)> {
        self.inner
            .lock()
            .unwrap()
            .get(key)
            .map(|identity| (identity.company_id.clone(), identity.agent_id.clone()))
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for Actor
where
    AgentKeyStore: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = (StatusCode, Json<Value>);

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let Some(header_value) = parts.headers.get(AUTHORIZATION) else {
            // No bearer credentials: full-control board/operator context. An
            // optional `X-Actor-User` header carries the board user identity
            // (ports the `req.actor.userId` board-user context).
            let user_id = parts
                .headers
                .get("x-actor-user")
                .and_then(|v| v.to_str().ok())
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty());
            return Ok(Actor::Board { user_id });
        };

        let raw = header_value.to_str().unwrap_or("");
        let Some(token) = raw.strip_prefix("Bearer ") else {
            return Err((
                StatusCode::UNAUTHORIZED,
                Json(json!({ "error": "invalid_authorization" })),
            ));
        };

        let store = AgentKeyStore::from_ref(state);
        match store.identity_for(token.trim()) {
            Some((company_id, agent_id)) => Ok(Actor::Agent {
                company_id,
                agent_id,
            }),
            None => Err((
                StatusCode::UNAUTHORIZED,
                Json(json!({ "error": "invalid_agent_key" })),
            )),
        }
    }
}

/// `GET /api/whoami` — reports the resolved actor. The `agentId` is included only
/// when the key carries a specific agent identity.
pub async fn whoami(actor: Actor) -> Json<Value> {
    match actor {
        Actor::Board {
            user_id: Some(user_id),
        } => Json(json!({ "actor": "board", "userId": user_id })),
        Actor::Board { user_id: None } => Json(json!({ "actor": "board" })),
        Actor::Agent {
            company_id,
            agent_id: Some(agent_id),
        } => Json(json!({ "actor": "agent", "companyId": company_id, "agentId": agent_id })),
        Actor::Agent {
            company_id,
            agent_id: None,
        } => Json(json!({ "actor": "agent", "companyId": company_id })),
    }
}

/// Company-scoping authorization: the board operator may access any company; an
/// agent may only access its own. Ports the company-boundary check in
/// server/src/services/authorization.ts.
pub fn authorize_company_access(
    actor: &Actor,
    company_id: &str,
) -> Result<(), (StatusCode, Json<Value>)> {
    match actor {
        Actor::Board { .. } => Ok(()),
        Actor::Agent {
            company_id: agent_company,
            ..
        } => {
            if agent_company == company_id {
                Ok(())
            } else {
                Err((StatusCode::FORBIDDEN, Json(json!({ "error": "forbidden" }))))
            }
        }
    }
}

/// Board-only authorization: rejects agents outright (even for their own
/// company). Ports `assertBoard` in server/src/routes/authz.ts — used by
/// operator-only writes such as the activity feed.
pub fn require_board(actor: &Actor) -> Result<(), (StatusCode, Json<Value>)> {
    match actor {
        Actor::Board { .. } => Ok(()),
        Actor::Agent { .. } => Err((StatusCode::FORBIDDEN, Json(json!({ "error": "forbidden" })))),
    }
}

/// Like [`require_board_user`] but returns a single **401** "Board authentication
/// required" for any non-board-user (agent OR board without a user). Ports the
/// `unauthorized("Board authentication required")` guard on the board-api-keys
/// routes (server/src/routes/access.ts).
pub fn require_board_user_authenticated(
    actor: &Actor,
) -> Result<String, (StatusCode, Json<Value>)> {
    match actor {
        Actor::Board {
            user_id: Some(user_id),
        } => Ok(user_id.clone()),
        _ => Err((
            StatusCode::UNAUTHORIZED,
            Json(json!({ "error": "Board authentication required" })),
        )),
    }
}

/// Requires a board operator *with a user context*, returning the user id.
/// Agents get "Board authentication required"; a board with no user identity
/// gets "Board user context required". Ports `requireBoardUserId` in
/// server/src/routes/inbox-dismissals.ts / sidebar-preferences.ts.
pub fn require_board_user(actor: &Actor) -> Result<String, (StatusCode, Json<Value>)> {
    match actor {
        Actor::Agent { .. } => Err((
            StatusCode::FORBIDDEN,
            Json(json!({ "error": "Board authentication required" })),
        )),
        Actor::Board { user_id: None } => Err((
            StatusCode::FORBIDDEN,
            Json(json!({ "error": "Board user context required" })),
        )),
        Actor::Board {
            user_id: Some(user_id),
        } => Ok(user_id.clone()),
    }
}

/// Enforces that an agent only acts on its own behalf: an [`Actor::Agent`] that
/// carries an identity may only reference its own `agent_id`. Board actors and
/// identity-less keys are unrestricted. Ports the per-agent ownership check in
/// server/src/routes/costs.ts.
pub fn require_own_agent(actor: &Actor, agent_id: &str) -> Result<(), (StatusCode, Json<Value>)> {
    match actor {
        Actor::Agent {
            agent_id: Some(own),
            ..
        } if own != agent_id => Err((
            StatusCode::FORBIDDEN,
            Json(json!({ "error": "Agent can only report its own costs" })),
        )),
        _ => Ok(()),
    }
}
