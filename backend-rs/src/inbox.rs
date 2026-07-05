use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};

use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::auth::{authorize_company_access, require_board_user, Actor};

/// A per-user dismissal of an inbox item. Ports the shape returned by
/// server/src/routes/inbox-dismissals.ts.
#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InboxDismissal {
    pub company_id: String,
    pub user_id: String,
    pub item_key: String,
    pub dismissed_at: String,
}

/// Ports the dismiss payload: `itemKey` must be `approval:`, `join:`, or `run:`
/// prefixed with a non-empty suffix.
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateDismissal {
    pub item_key: String,
}

fn is_valid_item_key(key: &str) -> bool {
    match key.split_once(':') {
        Some((prefix, rest)) => matches!(prefix, "approval" | "join" | "run") && !rest.is_empty(),
        None => false,
    }
}

fn now_millis() -> String {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis().to_string())
        .unwrap_or_default()
}

/// In-memory, (company, user)-scoped dismissal store.
#[derive(Clone, Default)]
pub struct InboxStore {
    inner: Arc<Mutex<Vec<InboxDismissal>>>,
}

impl InboxStore {
    /// Upserts a dismissal: dismissing the same item again just refreshes its
    /// timestamp (idempotent per company+user+itemKey).
    pub fn dismiss(
        &self,
        company_id: &str,
        user_id: &str,
        item_key: &str,
        dismissed_at: String,
    ) -> InboxDismissal {
        let mut guard = self.inner.lock().unwrap();
        if let Some(existing) = guard
            .iter_mut()
            .find(|d| d.company_id == company_id && d.user_id == user_id && d.item_key == item_key)
        {
            existing.dismissed_at = dismissed_at;
            return existing.clone();
        }
        let dismissal = InboxDismissal {
            company_id: company_id.to_string(),
            user_id: user_id.to_string(),
            item_key: item_key.to_string(),
            dismissed_at,
        };
        guard.push(dismissal.clone());
        dismissal
    }

    pub fn list(&self, company_id: &str, user_id: &str) -> Vec<InboxDismissal> {
        self.inner
            .lock()
            .unwrap()
            .iter()
            .filter(|d| d.company_id == company_id && d.user_id == user_id)
            .cloned()
            .collect()
    }
}

pub trait InboxRepository {
    fn dismiss(
        &self,
        company_id: &str,
        user_id: &str,
        item_key: &str,
        dismissed_at: String,
    ) -> InboxDismissal;
    fn list(&self, company_id: &str, user_id: &str) -> Vec<InboxDismissal>;
}

impl InboxRepository for InboxStore {
    fn dismiss(
        &self,
        company_id: &str,
        user_id: &str,
        item_key: &str,
        dismissed_at: String,
    ) -> InboxDismissal {
        InboxStore::dismiss(self, company_id, user_id, item_key, dismissed_at)
    }
    fn list(&self, company_id: &str, user_id: &str) -> Vec<InboxDismissal> {
        InboxStore::list(self, company_id, user_id)
    }
}

/// Cloneable handle to whichever [`InboxRepository`] backs the running app.
#[derive(Clone)]
pub struct InboxRepo(pub Arc<dyn InboxRepository + Send + Sync>);

impl Default for InboxRepo {
    fn default() -> Self {
        InboxRepo(Arc::new(InboxStore::default()))
    }
}

pub async fn list_inbox_dismissals(
    actor: Actor,
    State(repo): State<InboxRepo>,
    Path(company_id): Path<String>,
) -> Result<Json<Vec<InboxDismissal>>, (StatusCode, Json<Value>)> {
    authorize_company_access(&actor, &company_id)?;
    let user_id = require_board_user(&actor)?;
    Ok(Json(repo.0.list(&company_id, &user_id)))
}

pub async fn create_inbox_dismissal(
    actor: Actor,
    State(repo): State<InboxRepo>,
    Path(company_id): Path<String>,
    Json(input): Json<CreateDismissal>,
) -> Result<(StatusCode, Json<InboxDismissal>), (StatusCode, Json<Value>)> {
    authorize_company_access(&actor, &company_id)?;
    let user_id = require_board_user(&actor)?;
    if !is_valid_item_key(&input.item_key) {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": "Unsupported inbox item key" })),
        ));
    }
    let dismissal = repo
        .0
        .dismiss(&company_id, &user_id, &input.item_key, now_millis());
    Ok((StatusCode::CREATED, Json(dismissal)))
}
