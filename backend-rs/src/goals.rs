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

/// A goal — the hierarchical north-star objective for a company/team/agent/task.
/// Ports server/src/routes/goals.ts + packages/shared/src/types/goal.ts core
/// columns (timestamps land with SQLite persistence in a later slice).
#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Goal {
    pub id: String,
    pub company_id: String,
    pub title: String,
    pub description: Option<String>,
    pub level: String,
    pub status: String,
    pub parent_id: Option<String>,
    pub owner_agent_id: Option<String>,
}

/// Ports the create-goal payload (packages/shared/src/validators/goal.ts):
/// `title` required; `level` defaults to "task"; `status` to "planned"; the rest
/// are optional/nullable.
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateGoal {
    pub title: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default = "default_level")]
    pub level: String,
    #[serde(default = "default_status")]
    pub status: String,
    #[serde(default)]
    pub parent_id: Option<String>,
    #[serde(default)]
    pub owner_agent_id: Option<String>,
}

fn default_level() -> String {
    "task".to_string()
}

fn default_status() -> String {
    "planned".to_string()
}

/// Ports the partial update-goal payload (`createGoalSchema.partial()`): any
/// provided field is applied; others are left unchanged.
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateGoal {
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub level: Option<String>,
    #[serde(default)]
    pub status: Option<String>,
    #[serde(default)]
    pub parent_id: Option<String>,
    #[serde(default)]
    pub owner_agent_id: Option<String>,
}

/// In-memory, company-scoped goal repository. A SQLite-backed implementation
/// behind the same trait lands in a later slice (as with the other domains).
#[derive(Clone, Default)]
pub struct GoalStore {
    inner: Arc<Mutex<Vec<Goal>>>,
}

impl GoalStore {
    pub fn create(&self, company_id: &str, input: CreateGoal) -> Goal {
        let goal = Goal {
            id: Uuid::new_v4().to_string(),
            company_id: company_id.to_string(),
            title: input.title,
            description: input.description,
            level: input.level,
            status: input.status,
            parent_id: input.parent_id,
            owner_agent_id: input.owner_agent_id,
        };
        self.inner.lock().unwrap().push(goal.clone());
        goal
    }

    /// Company scoping: only goals belonging to `company_id` are returned.
    pub fn list_by_company(&self, company_id: &str) -> Vec<Goal> {
        self.inner
            .lock()
            .unwrap()
            .iter()
            .filter(|goal| goal.company_id == company_id)
            .cloned()
            .collect()
    }

    pub fn update(&self, company_id: &str, goal_id: &str, patch: UpdateGoal) -> Option<Goal> {
        let mut guard = self.inner.lock().unwrap();
        let goal = guard
            .iter_mut()
            .find(|goal| goal.company_id == company_id && goal.id == goal_id)?;
        if let Some(title) = patch.title {
            goal.title = title;
        }
        if let Some(description) = patch.description {
            goal.description = Some(description);
        }
        if let Some(level) = patch.level {
            goal.level = level;
        }
        if let Some(status) = patch.status {
            goal.status = status;
        }
        if let Some(parent_id) = patch.parent_id {
            goal.parent_id = Some(parent_id);
        }
        if let Some(owner_agent_id) = patch.owner_agent_id {
            goal.owner_agent_id = Some(owner_agent_id);
        }
        Some(goal.clone())
    }

    pub fn delete(&self, company_id: &str, goal_id: &str) -> bool {
        let mut guard = self.inner.lock().unwrap();
        let before = guard.len();
        guard.retain(|goal| !(goal.company_id == company_id && goal.id == goal_id));
        guard.len() != before
    }
}

/// The goal persistence interface. Both the in-memory [`GoalStore`] and a future
/// SQLite-backed store implement this.
pub trait GoalRepository {
    fn list_by_company(&self, company_id: &str) -> Vec<Goal>;
    fn create(&self, company_id: &str, input: CreateGoal) -> Goal;
    fn update(&self, company_id: &str, goal_id: &str, patch: UpdateGoal) -> Option<Goal>;
    fn delete(&self, company_id: &str, goal_id: &str) -> bool;
}

impl GoalRepository for GoalStore {
    fn list_by_company(&self, company_id: &str) -> Vec<Goal> {
        GoalStore::list_by_company(self, company_id)
    }
    fn create(&self, company_id: &str, input: CreateGoal) -> Goal {
        GoalStore::create(self, company_id, input)
    }
    fn update(&self, company_id: &str, goal_id: &str, patch: UpdateGoal) -> Option<Goal> {
        GoalStore::update(self, company_id, goal_id, patch)
    }
    fn delete(&self, company_id: &str, goal_id: &str) -> bool {
        GoalStore::delete(self, company_id, goal_id)
    }
}

/// Cloneable handle to whichever [`GoalRepository`] backs the running app.
#[derive(Clone)]
pub struct GoalRepo(pub Arc<dyn GoalRepository + Send + Sync>);

impl Default for GoalRepo {
    fn default() -> Self {
        GoalRepo(Arc::new(GoalStore::default()))
    }
}

pub async fn list_goals(
    actor: Actor,
    State(repo): State<GoalRepo>,
    Path(company_id): Path<String>,
) -> Result<Json<Vec<Goal>>, (StatusCode, Json<Value>)> {
    authorize_company_access(&actor, &company_id)?;
    Ok(Json(repo.0.list_by_company(&company_id)))
}

pub async fn create_goal(
    actor: Actor,
    State(repo): State<GoalRepo>,
    Path(company_id): Path<String>,
    Json(input): Json<CreateGoal>,
) -> Result<(StatusCode, Json<Goal>), (StatusCode, Json<Value>)> {
    authorize_company_access(&actor, &company_id)?;
    Ok((StatusCode::CREATED, Json(repo.0.create(&company_id, input))))
}

fn goal_not_found() -> (StatusCode, Json<Value>) {
    (
        StatusCode::NOT_FOUND,
        Json(json!({ "error": "Goal not found" })),
    )
}

pub async fn update_goal(
    actor: Actor,
    State(repo): State<GoalRepo>,
    Path((company_id, goal_id)): Path<(String, String)>,
    Json(patch): Json<UpdateGoal>,
) -> Result<Json<Goal>, (StatusCode, Json<Value>)> {
    authorize_company_access(&actor, &company_id)?;
    match repo.0.update(&company_id, &goal_id, patch) {
        Some(goal) => Ok(Json(goal)),
        None => Err(goal_not_found()),
    }
}

pub async fn delete_goal(
    actor: Actor,
    State(repo): State<GoalRepo>,
    Path((company_id, goal_id)): Path<(String, String)>,
) -> Result<StatusCode, (StatusCode, Json<Value>)> {
    authorize_company_access(&actor, &company_id)?;
    if repo.0.delete(&company_id, &goal_id) {
        Ok(StatusCode::NO_CONTENT)
    } else {
        Err(goal_not_found())
    }
}
