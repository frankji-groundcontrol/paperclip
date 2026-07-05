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

/// A routine — a recurring scheduled task. Ports the core `routines` columns
/// (packages/db/src/schema/routines.ts); cadence/policy fields land later.
#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Routine {
    pub id: String,
    pub company_id: String,
    pub title: String,
    pub status: String,
}

/// Ports the create-routine payload (packages/shared/src/validators/routine.ts):
/// `title` required, `status` defaulting to "active".
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateRoutine {
    pub title: String,
    #[serde(default = "default_status")]
    pub status: String,
}

fn default_status() -> String {
    "active".to_string()
}

pub trait RoutineRepository {
    fn list_by_company(&self, company_id: &str) -> Vec<Routine>;
    fn create(&self, company_id: &str, input: CreateRoutine) -> Routine;
    fn update(&self, company_id: &str, routine_id: &str, patch: UpdateRoutine) -> Option<Routine>;
    fn delete(&self, company_id: &str, routine_id: &str) -> bool;
}

/// Ports the partial update-routine payload.
#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateRoutine {
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default)]
    pub status: Option<String>,
}

#[derive(Clone, Default)]
pub struct RoutineStore {
    inner: Arc<Mutex<Vec<Routine>>>,
}

impl RoutineStore {
    pub fn create(&self, company_id: &str, input: CreateRoutine) -> Routine {
        let routine = Routine {
            id: Uuid::new_v4().to_string(),
            company_id: company_id.to_string(),
            title: input.title,
            status: input.status,
        };
        self.inner.lock().unwrap().push(routine.clone());
        routine
    }

    pub fn list_by_company(&self, company_id: &str) -> Vec<Routine> {
        self.inner
            .lock()
            .unwrap()
            .iter()
            .filter(|routine| routine.company_id == company_id)
            .cloned()
            .collect()
    }

    pub fn update(
        &self,
        company_id: &str,
        routine_id: &str,
        patch: UpdateRoutine,
    ) -> Option<Routine> {
        let mut guard = self.inner.lock().unwrap();
        let routine = guard
            .iter_mut()
            .find(|routine| routine.company_id == company_id && routine.id == routine_id)?;
        if let Some(title) = patch.title {
            routine.title = title;
        }
        if let Some(status) = patch.status {
            routine.status = status;
        }
        Some(routine.clone())
    }

    pub fn delete(&self, company_id: &str, routine_id: &str) -> bool {
        let mut guard = self.inner.lock().unwrap();
        let before = guard.len();
        guard.retain(|routine| !(routine.company_id == company_id && routine.id == routine_id));
        guard.len() != before
    }
}

impl RoutineRepository for RoutineStore {
    fn list_by_company(&self, company_id: &str) -> Vec<Routine> {
        RoutineStore::list_by_company(self, company_id)
    }
    fn create(&self, company_id: &str, input: CreateRoutine) -> Routine {
        RoutineStore::create(self, company_id, input)
    }
    fn update(&self, company_id: &str, routine_id: &str, patch: UpdateRoutine) -> Option<Routine> {
        RoutineStore::update(self, company_id, routine_id, patch)
    }
    fn delete(&self, company_id: &str, routine_id: &str) -> bool {
        RoutineStore::delete(self, company_id, routine_id)
    }
}

/// Cloneable handle to whichever [`RoutineRepository`] backs the running app.
#[derive(Clone)]
pub struct RoutineRepo(pub Arc<dyn RoutineRepository + Send + Sync>);

impl Default for RoutineRepo {
    fn default() -> Self {
        RoutineRepo(Arc::new(RoutineStore::default()))
    }
}

pub async fn list_routines(
    actor: Actor,
    State(repo): State<RoutineRepo>,
    Path(company_id): Path<String>,
) -> Result<Json<Vec<Routine>>, (StatusCode, Json<Value>)> {
    authorize_company_access(&actor, &company_id)?;
    Ok(Json(repo.0.list_by_company(&company_id)))
}

pub async fn create_routine(
    actor: Actor,
    State(repo): State<RoutineRepo>,
    Path(company_id): Path<String>,
    Json(input): Json<CreateRoutine>,
) -> Result<(StatusCode, Json<Routine>), (StatusCode, Json<Value>)> {
    authorize_company_access(&actor, &company_id)?;
    Ok((StatusCode::CREATED, Json(repo.0.create(&company_id, input))))
}

fn routine_not_found() -> (StatusCode, Json<Value>) {
    (
        StatusCode::NOT_FOUND,
        Json(json!({ "error": "Routine not found" })),
    )
}

pub async fn update_routine(
    actor: Actor,
    State(repo): State<RoutineRepo>,
    Path((company_id, routine_id)): Path<(String, String)>,
    Json(patch): Json<UpdateRoutine>,
) -> Result<Json<Routine>, (StatusCode, Json<Value>)> {
    authorize_company_access(&actor, &company_id)?;
    match repo.0.update(&company_id, &routine_id, patch) {
        Some(routine) => Ok(Json(routine)),
        None => Err(routine_not_found()),
    }
}

pub async fn delete_routine(
    actor: Actor,
    State(repo): State<RoutineRepo>,
    Path((company_id, routine_id)): Path<(String, String)>,
) -> Result<StatusCode, (StatusCode, Json<Value>)> {
    authorize_company_access(&actor, &company_id)?;
    if repo.0.delete(&company_id, &routine_id) {
        Ok(StatusCode::NO_CONTENT)
    } else {
        Err(routine_not_found())
    }
}
