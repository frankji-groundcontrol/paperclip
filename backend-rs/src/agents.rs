use std::sync::{Arc, Mutex};

use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use uuid::Uuid;

use crate::auth::{authorize_company_access, Actor};

/// An agent — an AI worker in a company. Ports the core `agents` columns
/// (packages/db/src/schema/agents.ts).
#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Agent {
    pub id: String,
    pub company_id: String,
    pub name: String,
    pub role: String,
    pub status: String,
    pub adapter_type: String,
}

/// Ports the create-agent payload (packages/shared/src/validators/agent.ts):
/// `name` and `adapterType` required, `role` defaulting to "general". Newly
/// created agents start `idle` (the schema column default).
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateAgent {
    pub name: String,
    #[serde(default = "default_role")]
    pub role: String,
    pub adapter_type: String,
}

fn default_role() -> String {
    "general".to_string()
}

/// Ports the partial update-agent payload: provided fields are applied.
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateAgent {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub role: Option<String>,
    #[serde(default)]
    pub status: Option<String>,
    #[serde(default)]
    pub adapter_type: Option<String>,
}

/// In-memory, company-scoped agent repository.
#[derive(Clone, Default)]
pub struct AgentStore {
    inner: Arc<Mutex<Vec<Agent>>>,
}

impl AgentStore {
    pub fn create(&self, company_id: &str, input: CreateAgent) -> Agent {
        let agent = Agent {
            id: Uuid::new_v4().to_string(),
            company_id: company_id.to_string(),
            name: input.name,
            role: input.role,
            status: "idle".to_string(),
            adapter_type: input.adapter_type,
        };
        self.inner.lock().unwrap().push(agent.clone());
        agent
    }

    /// Company scoping: only agents belonging to `company_id` are returned.
    pub fn list_by_company(&self, company_id: &str) -> Vec<Agent> {
        self.inner
            .lock()
            .unwrap()
            .iter()
            .filter(|agent| agent.company_id == company_id)
            .cloned()
            .collect()
    }

    pub fn update(&self, company_id: &str, agent_id: &str, patch: UpdateAgent) -> Option<Agent> {
        let mut guard = self.inner.lock().unwrap();
        let agent = guard
            .iter_mut()
            .find(|agent| agent.company_id == company_id && agent.id == agent_id)?;
        if let Some(name) = patch.name {
            agent.name = name;
        }
        if let Some(role) = patch.role {
            agent.role = role;
        }
        if let Some(status) = patch.status {
            agent.status = status;
        }
        if let Some(adapter_type) = patch.adapter_type {
            agent.adapter_type = adapter_type;
        }
        Some(agent.clone())
    }

    pub fn delete(&self, company_id: &str, agent_id: &str) -> bool {
        let mut guard = self.inner.lock().unwrap();
        let before = guard.len();
        guard.retain(|agent| !(agent.company_id == company_id && agent.id == agent_id));
        guard.len() != before
    }
}

/// The agent persistence interface. Both the in-memory [`AgentStore`] and the
/// SQLite-backed store (see `crate::db`) implement this.
pub trait AgentRepository {
    fn list_by_company(&self, company_id: &str) -> Vec<Agent>;
    fn create(&self, company_id: &str, input: CreateAgent) -> Agent;
    fn update(&self, company_id: &str, agent_id: &str, patch: UpdateAgent) -> Option<Agent>;
    fn delete(&self, company_id: &str, agent_id: &str) -> bool;
}

impl AgentRepository for AgentStore {
    fn list_by_company(&self, company_id: &str) -> Vec<Agent> {
        AgentStore::list_by_company(self, company_id)
    }
    fn create(&self, company_id: &str, input: CreateAgent) -> Agent {
        AgentStore::create(self, company_id, input)
    }
    fn update(&self, company_id: &str, agent_id: &str, patch: UpdateAgent) -> Option<Agent> {
        AgentStore::update(self, company_id, agent_id, patch)
    }
    fn delete(&self, company_id: &str, agent_id: &str) -> bool {
        AgentStore::delete(self, company_id, agent_id)
    }
}

/// Cloneable handle to whichever [`AgentRepository`] backs the running app.
#[derive(Clone)]
pub struct AgentRepo(pub Arc<dyn AgentRepository + Send + Sync>);

impl Default for AgentRepo {
    fn default() -> Self {
        AgentRepo(Arc::new(AgentStore::default()))
    }
}

pub async fn list_agents(
    actor: Actor,
    State(repo): State<AgentRepo>,
    Path(company_id): Path<String>,
) -> Result<Json<Vec<Agent>>, (StatusCode, Json<Value>)> {
    authorize_company_access(&actor, &company_id)?;
    Ok(Json(repo.0.list_by_company(&company_id)))
}

pub async fn create_agent(
    actor: Actor,
    State(repo): State<AgentRepo>,
    Path(company_id): Path<String>,
    Json(input): Json<CreateAgent>,
) -> Result<(StatusCode, Json<Agent>), (StatusCode, Json<Value>)> {
    authorize_company_access(&actor, &company_id)?;
    Ok((StatusCode::CREATED, Json(repo.0.create(&company_id, input))))
}

fn agent_not_found() -> (StatusCode, Json<Value>) {
    (
        StatusCode::NOT_FOUND,
        Json(json!({ "error": "Agent not found" })),
    )
}

pub async fn update_agent(
    actor: Actor,
    State(repo): State<AgentRepo>,
    Path((company_id, agent_id)): Path<(String, String)>,
    Json(patch): Json<UpdateAgent>,
) -> Result<Json<Agent>, (StatusCode, Json<Value>)> {
    authorize_company_access(&actor, &company_id)?;
    match repo.0.update(&company_id, &agent_id, patch) {
        Some(agent) => Ok(Json(agent)),
        None => Err(agent_not_found()),
    }
}

pub async fn delete_agent(
    actor: Actor,
    State(repo): State<AgentRepo>,
    Path((company_id, agent_id)): Path<(String, String)>,
) -> Result<StatusCode, (StatusCode, Json<Value>)> {
    authorize_company_access(&actor, &company_id)?;
    if repo.0.delete(&company_id, &agent_id) {
        Ok(StatusCode::NO_CONTENT)
    } else {
        Err(agent_not_found())
    }
}
