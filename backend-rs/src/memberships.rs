use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};

use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde::Deserialize;
use serde_json::{json, Map, Value};

use crate::agents::AgentRepo;
use crate::auth::{authorize_company_access, require_board_user, Actor};
use crate::projects::ProjectRepo;

/// A board user's explicit membership in a resource. Ports the OSS-default
/// behaviour of server/src/services/resource-memberships.ts (the enterprise
/// policy hook resolves to `oss_default`, which is stripped from responses).
#[derive(Clone)]
pub struct ResourceMembership {
    pub company_id: String,
    pub user_id: String,
    pub resource_type: String,
    pub resource_id: String,
    pub state: String,
}

#[derive(Deserialize)]
pub struct MembershipPayload {
    pub state: String,
}

fn now_millis() -> String {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis().to_string())
        .unwrap_or_default()
}

/// In-memory, (company, user)-scoped membership store.
#[derive(Clone, Default)]
pub struct MembershipStore {
    inner: Arc<Mutex<Vec<ResourceMembership>>>,
}

impl MembershipStore {
    pub fn upsert(
        &self,
        company_id: &str,
        user_id: &str,
        resource_type: &str,
        resource_id: &str,
        state: &str,
    ) {
        let mut guard = self.inner.lock().unwrap();
        if let Some(existing) = guard.iter_mut().find(|m| {
            m.company_id == company_id
                && m.user_id == user_id
                && m.resource_type == resource_type
                && m.resource_id == resource_id
        }) {
            existing.state = state.to_string();
        } else {
            guard.push(ResourceMembership {
                company_id: company_id.to_string(),
                user_id: user_id.to_string(),
                resource_type: resource_type.to_string(),
                resource_id: resource_id.to_string(),
                state: state.to_string(),
            });
        }
    }

    pub fn list_for_user(&self, company_id: &str, user_id: &str) -> Vec<ResourceMembership> {
        self.inner
            .lock()
            .unwrap()
            .iter()
            .filter(|m| m.company_id == company_id && m.user_id == user_id)
            .cloned()
            .collect()
    }
}

pub trait MembershipRepository {
    fn upsert(
        &self,
        company_id: &str,
        user_id: &str,
        resource_type: &str,
        resource_id: &str,
        state: &str,
    );
    fn list_for_user(&self, company_id: &str, user_id: &str) -> Vec<ResourceMembership>;
}

impl MembershipRepository for MembershipStore {
    fn upsert(
        &self,
        company_id: &str,
        user_id: &str,
        resource_type: &str,
        resource_id: &str,
        state: &str,
    ) {
        MembershipStore::upsert(self, company_id, user_id, resource_type, resource_id, state)
    }
    fn list_for_user(&self, company_id: &str, user_id: &str) -> Vec<ResourceMembership> {
        MembershipStore::list_for_user(self, company_id, user_id)
    }
}

/// Cloneable handle to whichever [`MembershipRepository`] backs the running app.
#[derive(Clone)]
pub struct MembershipRepo(pub Arc<dyn MembershipRepository + Send + Sync>);

impl Default for MembershipRepo {
    fn default() -> Self {
        MembershipRepo(Arc::new(MembershipStore::default()))
    }
}

pub async fn get_memberships(
    actor: Actor,
    State(repo): State<MembershipRepo>,
    Path(company_id): Path<String>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    authorize_company_access(&actor, &company_id)?;
    let user_id = require_board_user(&actor)?;

    let mut projects = Map::new();
    let mut agents = Map::new();
    for m in repo.0.list_for_user(&company_id, &user_id) {
        match m.resource_type.as_str() {
            "project" => {
                projects.insert(m.resource_id, Value::String(m.state));
            }
            "agent" => {
                agents.insert(m.resource_id, Value::String(m.state));
            }
            _ => {}
        }
    }
    Ok(Json(json!({
        "projectMemberships": projects,
        "agentMemberships": agents,
        "updatedAt": Value::Null,
    })))
}

/// Validates the membership state (`joined`/`left`).
fn validate_state(state: &str) -> Result<(), (StatusCode, Json<Value>)> {
    if state == "joined" || state == "left" {
        Ok(())
    } else {
        Err((
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": "Invalid membership state" })),
        ))
    }
}

fn resource_response(resource_type: &str, resource_id: &str, state: &str) -> Value {
    json!({
        "resourceType": resource_type,
        "resourceId": resource_id,
        "state": state,
        "updatedAt": now_millis(),
    })
}

pub async fn put_project_membership(
    actor: Actor,
    State(repo): State<MembershipRepo>,
    State(projects): State<ProjectRepo>,
    Path((company_id, project_id)): Path<(String, String)>,
    Json(payload): Json<MembershipPayload>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    authorize_company_access(&actor, &company_id)?;
    let user_id = require_board_user(&actor)?;
    validate_state(&payload.state)?;

    let exists = projects
        .0
        .list_by_company(&company_id)
        .iter()
        .any(|p| p.id == project_id);
    if !exists {
        return Err((
            StatusCode::NOT_FOUND,
            Json(json!({ "error": "Project not found" })),
        ));
    }

    repo.0.upsert(
        &company_id,
        &user_id,
        "project",
        &project_id,
        &payload.state,
    );
    Ok(Json(resource_response(
        "project",
        &project_id,
        &payload.state,
    )))
}

pub async fn put_agent_membership(
    actor: Actor,
    State(repo): State<MembershipRepo>,
    State(agents): State<AgentRepo>,
    Path((company_id, agent_id)): Path<(String, String)>,
    Json(payload): Json<MembershipPayload>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    authorize_company_access(&actor, &company_id)?;
    let user_id = require_board_user(&actor)?;
    validate_state(&payload.state)?;

    let exists = agents
        .0
        .list_by_company(&company_id)
        .iter()
        .any(|a| a.id == agent_id);
    if !exists {
        return Err((
            StatusCode::NOT_FOUND,
            Json(json!({ "error": "Agent not found" })),
        ));
    }

    repo.0
        .upsert(&company_id, &user_id, "agent", &agent_id, &payload.state);
    Ok(Json(resource_response("agent", &agent_id, &payload.state)))
}
