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

/// A run — an agent's execution record (heartbeat_runs). Ports the core columns
/// (packages/db/src/schema/heartbeat_runs.ts); process/timing fields land later.
#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Run {
    pub id: String,
    pub company_id: String,
    pub agent_id: String,
    pub status: String,
}

/// Ports the create-run payload: `agentId` required, `status` defaulting to
/// "queued" (the schema column default; see HEARTBEAT_RUN_STATUSES).
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateRun {
    pub agent_id: String,
    #[serde(default = "default_status")]
    pub status: String,
}

fn default_status() -> String {
    "queued".to_string()
}

/// Ports the partial update-run payload — primarily status transitions.
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateRun {
    #[serde(default)]
    pub status: Option<String>,
    #[serde(default)]
    pub agent_id: Option<String>,
}

/// The run persistence interface. In-memory for now; a SQLite-backed store
/// implements this in a later slice (same pattern as the other domains).
pub trait RunRepository {
    fn list_by_company(&self, company_id: &str) -> Vec<Run>;
    fn create(&self, company_id: &str, input: CreateRun) -> Run;
    fn update(&self, company_id: &str, run_id: &str, patch: UpdateRun) -> Option<Run>;
    fn delete(&self, company_id: &str, run_id: &str) -> bool;
}

#[derive(Clone, Default)]
pub struct RunStore {
    inner: Arc<Mutex<Vec<Run>>>,
}

impl RunStore {
    pub fn create(&self, company_id: &str, input: CreateRun) -> Run {
        let run = Run {
            id: Uuid::new_v4().to_string(),
            company_id: company_id.to_string(),
            agent_id: input.agent_id,
            status: input.status,
        };
        self.inner.lock().unwrap().push(run.clone());
        run
    }

    pub fn list_by_company(&self, company_id: &str) -> Vec<Run> {
        self.inner
            .lock()
            .unwrap()
            .iter()
            .filter(|run| run.company_id == company_id)
            .cloned()
            .collect()
    }

    pub fn update(&self, company_id: &str, run_id: &str, patch: UpdateRun) -> Option<Run> {
        let mut guard = self.inner.lock().unwrap();
        let run = guard
            .iter_mut()
            .find(|run| run.company_id == company_id && run.id == run_id)?;
        if let Some(status) = patch.status {
            run.status = status;
        }
        if let Some(agent_id) = patch.agent_id {
            run.agent_id = agent_id;
        }
        Some(run.clone())
    }

    pub fn delete(&self, company_id: &str, run_id: &str) -> bool {
        let mut guard = self.inner.lock().unwrap();
        let before = guard.len();
        guard.retain(|run| !(run.company_id == company_id && run.id == run_id));
        guard.len() != before
    }
}

impl RunRepository for RunStore {
    fn list_by_company(&self, company_id: &str) -> Vec<Run> {
        RunStore::list_by_company(self, company_id)
    }
    fn create(&self, company_id: &str, input: CreateRun) -> Run {
        RunStore::create(self, company_id, input)
    }
    fn update(&self, company_id: &str, run_id: &str, patch: UpdateRun) -> Option<Run> {
        RunStore::update(self, company_id, run_id, patch)
    }
    fn delete(&self, company_id: &str, run_id: &str) -> bool {
        RunStore::delete(self, company_id, run_id)
    }
}

/// Cloneable handle to whichever [`RunRepository`] backs the running app.
#[derive(Clone)]
pub struct RunRepo(pub Arc<dyn RunRepository + Send + Sync>);

impl Default for RunRepo {
    fn default() -> Self {
        RunRepo(Arc::new(RunStore::default()))
    }
}

pub async fn list_runs(
    actor: Actor,
    State(repo): State<RunRepo>,
    Path(company_id): Path<String>,
) -> Result<Json<Vec<Run>>, (StatusCode, Json<Value>)> {
    authorize_company_access(&actor, &company_id)?;
    Ok(Json(repo.0.list_by_company(&company_id)))
}

pub async fn create_run(
    actor: Actor,
    State(repo): State<RunRepo>,
    Path(company_id): Path<String>,
    Json(input): Json<CreateRun>,
) -> Result<(StatusCode, Json<Run>), (StatusCode, Json<Value>)> {
    authorize_company_access(&actor, &company_id)?;
    Ok((StatusCode::CREATED, Json(repo.0.create(&company_id, input))))
}

fn run_not_found() -> (StatusCode, Json<Value>) {
    (
        StatusCode::NOT_FOUND,
        Json(json!({ "error": "Run not found" })),
    )
}

pub async fn update_run(
    actor: Actor,
    State(repo): State<RunRepo>,
    Path((company_id, run_id)): Path<(String, String)>,
    Json(patch): Json<UpdateRun>,
) -> Result<Json<Run>, (StatusCode, Json<Value>)> {
    authorize_company_access(&actor, &company_id)?;
    match repo.0.update(&company_id, &run_id, patch) {
        Some(run) => Ok(Json(run)),
        None => Err(run_not_found()),
    }
}

pub async fn delete_run(
    actor: Actor,
    State(repo): State<RunRepo>,
    Path((company_id, run_id)): Path<(String, String)>,
) -> Result<StatusCode, (StatusCode, Json<Value>)> {
    authorize_company_access(&actor, &company_id)?;
    if repo.0.delete(&company_id, &run_id) {
        Ok(StatusCode::NO_CONTENT)
    } else {
        Err(run_not_found())
    }
}
