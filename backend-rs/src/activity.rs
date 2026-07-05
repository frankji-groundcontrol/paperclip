use std::sync::{Arc, Mutex};

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

use crate::auth::{authorize_company_access, require_board, Actor};

/// Default and max activity page sizes — ports DEFAULT_ACTIVITY_LIMIT /
/// MAX_ACTIVITY_LIMIT in server/src/services/activity.ts.
const DEFAULT_ACTIVITY_LIMIT: usize = 100;
const MAX_ACTIVITY_LIMIT: usize = 500;

/// An audit-feed event. Ports packages/shared ActivityEvent core columns
/// (timestamps land with SQLite persistence in a later slice).
#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivityEvent {
    pub id: String,
    pub company_id: String,
    pub actor_type: String,
    pub actor_id: String,
    pub action: String,
    pub entity_type: String,
    pub entity_id: String,
    pub agent_id: Option<String>,
    pub run_id: Option<String>,
    pub details: Option<Value>,
}

/// Ports the create-activity payload (server/src/routes/activity.ts):
/// `actorType` defaults to "system"; actorId/action/entityType/entityId
/// required; agentId/details optional & nullable.
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateActivity {
    #[serde(default = "default_actor_type")]
    pub actor_type: String,
    pub actor_id: String,
    pub action: String,
    pub entity_type: String,
    pub entity_id: String,
    #[serde(default)]
    pub agent_id: Option<String>,
    #[serde(default)]
    pub details: Option<Value>,
}

fn default_actor_type() -> String {
    "system".to_string()
}

/// Query filters for the activity list (agent/entity filters + clamped limit).
#[derive(Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ActivityFilters {
    #[serde(default)]
    pub agent_id: Option<String>,
    #[serde(default)]
    pub entity_type: Option<String>,
    #[serde(default)]
    pub entity_id: Option<String>,
    #[serde(default)]
    pub limit: Option<usize>,
}

fn normalize_limit(limit: Option<usize>) -> usize {
    match limit {
        None => DEFAULT_ACTIVITY_LIMIT,
        Some(n) => n.clamp(1, MAX_ACTIVITY_LIMIT),
    }
}

/// In-memory, company-scoped activity store. A SQLite-backed implementation
/// behind the same trait lands in a later slice.
#[derive(Clone, Default)]
pub struct ActivityStore {
    inner: Arc<Mutex<Vec<ActivityEvent>>>,
}

impl ActivityStore {
    pub fn create(&self, company_id: &str, input: CreateActivity) -> ActivityEvent {
        let event = ActivityEvent {
            id: Uuid::new_v4().to_string(),
            company_id: company_id.to_string(),
            actor_type: input.actor_type,
            actor_id: input.actor_id,
            action: input.action,
            entity_type: input.entity_type,
            entity_id: input.entity_id,
            agent_id: input.agent_id,
            run_id: None,
            details: input.details,
        };
        self.inner.lock().unwrap().push(event.clone());
        event
    }

    /// Company-scoped, filtered, newest-first, limit-capped.
    pub fn list(&self, company_id: &str, filters: &ActivityFilters) -> Vec<ActivityEvent> {
        let limit = normalize_limit(filters.limit);
        self.inner
            .lock()
            .unwrap()
            .iter()
            .rev() // newest first (insertion order == chronological)
            .filter(|e| e.company_id == company_id)
            .filter(|e| {
                filters
                    .agent_id
                    .as_ref()
                    .is_none_or(|a| e.agent_id.as_deref() == Some(a.as_str()))
            })
            .filter(|e| {
                filters
                    .entity_type
                    .as_ref()
                    .is_none_or(|t| &e.entity_type == t)
            })
            .filter(|e| filters.entity_id.as_ref().is_none_or(|i| &e.entity_id == i))
            .take(limit)
            .cloned()
            .collect()
    }
}

/// The activity persistence interface.
pub trait ActivityRepository {
    fn list(&self, company_id: &str, filters: &ActivityFilters) -> Vec<ActivityEvent>;
    fn create(&self, company_id: &str, input: CreateActivity) -> ActivityEvent;
}

impl ActivityRepository for ActivityStore {
    fn list(&self, company_id: &str, filters: &ActivityFilters) -> Vec<ActivityEvent> {
        ActivityStore::list(self, company_id, filters)
    }
    fn create(&self, company_id: &str, input: CreateActivity) -> ActivityEvent {
        ActivityStore::create(self, company_id, input)
    }
}

/// Cloneable handle to whichever [`ActivityRepository`] backs the running app.
#[derive(Clone)]
pub struct ActivityRepo(pub Arc<dyn ActivityRepository + Send + Sync>);

impl Default for ActivityRepo {
    fn default() -> Self {
        ActivityRepo(Arc::new(ActivityStore::default()))
    }
}

pub async fn list_activity(
    actor: Actor,
    State(repo): State<ActivityRepo>,
    Path(company_id): Path<String>,
    Query(filters): Query<ActivityFilters>,
) -> Result<Json<Vec<ActivityEvent>>, (StatusCode, Json<Value>)> {
    authorize_company_access(&actor, &company_id)?;
    Ok(Json(repo.0.list(&company_id, &filters)))
}

pub async fn create_activity(
    actor: Actor,
    State(repo): State<ActivityRepo>,
    Path(company_id): Path<String>,
    Json(input): Json<CreateActivity>,
) -> Result<(StatusCode, Json<ActivityEvent>), (StatusCode, Json<Value>)> {
    // Activity events are operator-authored: board only (agents are rejected
    // even for their own company).
    require_board(&actor)?;
    Ok((StatusCode::CREATED, Json(repo.0.create(&company_id, input))))
}
