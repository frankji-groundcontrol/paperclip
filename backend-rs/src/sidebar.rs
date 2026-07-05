use std::sync::{Arc, Mutex};

use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::auth::{authorize_company_access, require_board_user, Actor};

/// A board user's sidebar ordering preference for a company. Ports the
/// `SidebarOrderPreference` shape (server/src/services/sidebar-preferences.ts).
#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SidebarOrderPreference {
    pub ordered_ids: Vec<String>,
    pub updated_at: Option<String>,
}

/// Ports the upsert payload: `{ orderedIds: string[] }`.
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpsertOrder {
    #[serde(default)]
    pub ordered_ids: Vec<String>,
}

/// Dedupes ordered ids, preserving first occurrence (ports normalizeOrderedIds).
fn normalize(ordered_ids: Vec<String>) -> Vec<String> {
    let mut seen = std::collections::HashSet::new();
    ordered_ids
        .into_iter()
        .filter(|id| seen.insert(id.clone()))
        .collect()
}

/// In-memory, (company, user)-scoped project-order store.
#[derive(Clone, Default)]
pub struct SidebarStore {
    inner: Arc<Mutex<Vec<(String, String, Vec<String>)>>>, // (company, user, orderedIds)
}

impl SidebarStore {
    pub fn get(&self, company_id: &str, user_id: &str) -> SidebarOrderPreference {
        let guard = self.inner.lock().unwrap();
        let ordered_ids = guard
            .iter()
            .find(|(c, u, _)| c == company_id && u == user_id)
            .map(|(_, _, ids)| ids.clone())
            .unwrap_or_default();
        SidebarOrderPreference {
            ordered_ids,
            updated_at: None,
        }
    }

    pub fn upsert(
        &self,
        company_id: &str,
        user_id: &str,
        ordered_ids: Vec<String>,
    ) -> SidebarOrderPreference {
        let normalized = normalize(ordered_ids);
        let mut guard = self.inner.lock().unwrap();
        if let Some(entry) = guard
            .iter_mut()
            .find(|(c, u, _)| c == company_id && u == user_id)
        {
            entry.2 = normalized.clone();
        } else {
            guard.push((
                company_id.to_string(),
                user_id.to_string(),
                normalized.clone(),
            ));
        }
        SidebarOrderPreference {
            ordered_ids: normalized,
            updated_at: None,
        }
    }
}

pub trait SidebarRepository {
    fn get(&self, company_id: &str, user_id: &str) -> SidebarOrderPreference;
    fn upsert(
        &self,
        company_id: &str,
        user_id: &str,
        ordered_ids: Vec<String>,
    ) -> SidebarOrderPreference;
}

impl SidebarRepository for SidebarStore {
    fn get(&self, company_id: &str, user_id: &str) -> SidebarOrderPreference {
        SidebarStore::get(self, company_id, user_id)
    }
    fn upsert(
        &self,
        company_id: &str,
        user_id: &str,
        ordered_ids: Vec<String>,
    ) -> SidebarOrderPreference {
        SidebarStore::upsert(self, company_id, user_id, ordered_ids)
    }
}

/// Cloneable handle to whichever [`SidebarRepository`] backs the running app.
#[derive(Clone)]
pub struct SidebarRepo(pub Arc<dyn SidebarRepository + Send + Sync>);

impl Default for SidebarRepo {
    fn default() -> Self {
        SidebarRepo(Arc::new(SidebarStore::default()))
    }
}

pub async fn get_project_order(
    actor: Actor,
    State(repo): State<SidebarRepo>,
    Path(company_id): Path<String>,
) -> Result<Json<SidebarOrderPreference>, (StatusCode, Json<Value>)> {
    authorize_company_access(&actor, &company_id)?;
    let user_id = require_board_user(&actor)?;
    Ok(Json(repo.0.get(&company_id, &user_id)))
}

pub async fn put_project_order(
    actor: Actor,
    State(repo): State<SidebarRepo>,
    Path(company_id): Path<String>,
    Json(input): Json<UpsertOrder>,
) -> Result<Json<SidebarOrderPreference>, (StatusCode, Json<Value>)> {
    authorize_company_access(&actor, &company_id)?;
    let user_id = require_board_user(&actor)?;
    Ok(Json(repo.0.upsert(
        &company_id,
        &user_id,
        input.ordered_ids,
    )))
}
