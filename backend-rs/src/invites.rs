use std::sync::{Arc, Mutex};

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use uuid::Uuid;

use crate::auth::{authorize_company_access, Actor};

/// A company invite — a shareable-token join link. Ports the OSS-default core of
/// server/src/routes/access.ts company invites (enterprise `users:invite`
/// permission resolution is deferred).
#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Invite {
    pub id: String,
    pub company_id: String,
    pub token: String,
    pub allowed_join_types: String,
    pub human_role: Option<String>,
    pub agent_message: Option<String>,
    pub state: String,
}

/// Ports the create-invite payload: `allowedJoinTypes` defaults to "both";
/// humanRole/agentMessage nullable/optional.
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateInvite {
    #[serde(default = "default_join_types")]
    pub allowed_join_types: String,
    #[serde(default)]
    pub human_role: Option<String>,
    #[serde(default)]
    pub agent_message: Option<String>,
}

fn default_join_types() -> String {
    "both".to_string()
}

/// Optional `?state=` list filter.
#[derive(Deserialize, Default)]
pub struct InviteFilter {
    #[serde(default)]
    pub state: Option<String>,
}

/// In-memory, company-scoped invite store.
#[derive(Clone, Default)]
pub struct InviteStore {
    inner: Arc<Mutex<Vec<Invite>>>,
}

impl InviteStore {
    pub fn create(&self, company_id: &str, input: CreateInvite) -> Invite {
        let invite = Invite {
            id: Uuid::new_v4().to_string(),
            company_id: company_id.to_string(),
            token: Uuid::new_v4().to_string(),
            allowed_join_types: input.allowed_join_types,
            human_role: input.human_role,
            agent_message: input.agent_message,
            state: "active".to_string(),
        };
        self.inner.lock().unwrap().push(invite.clone());
        invite
    }

    pub fn list_by_company(&self, company_id: &str, state: Option<&str>) -> Vec<Invite> {
        self.inner
            .lock()
            .unwrap()
            .iter()
            .filter(|i| i.company_id == company_id)
            .filter(|i| state.is_none_or(|s| i.state == s))
            .cloned()
            .collect()
    }

    pub fn revoke(&self, company_id: &str, invite_id: &str) -> Option<Invite> {
        let mut guard = self.inner.lock().unwrap();
        let invite = guard
            .iter_mut()
            .find(|i| i.company_id == company_id && i.id == invite_id)?;
        invite.state = "revoked".to_string();
        Some(invite.clone())
    }
}

pub trait InviteRepository {
    fn create(&self, company_id: &str, input: CreateInvite) -> Invite;
    fn list_by_company(&self, company_id: &str, state: Option<&str>) -> Vec<Invite>;
    fn revoke(&self, company_id: &str, invite_id: &str) -> Option<Invite>;
}

impl InviteRepository for InviteStore {
    fn create(&self, company_id: &str, input: CreateInvite) -> Invite {
        InviteStore::create(self, company_id, input)
    }
    fn list_by_company(&self, company_id: &str, state: Option<&str>) -> Vec<Invite> {
        InviteStore::list_by_company(self, company_id, state)
    }
    fn revoke(&self, company_id: &str, invite_id: &str) -> Option<Invite> {
        InviteStore::revoke(self, company_id, invite_id)
    }
}

/// Cloneable handle to whichever [`InviteRepository`] backs the running app.
#[derive(Clone)]
pub struct InviteRepo(pub Arc<dyn InviteRepository + Send + Sync>);

impl Default for InviteRepo {
    fn default() -> Self {
        InviteRepo(Arc::new(InviteStore::default()))
    }
}

pub async fn list_invites(
    actor: Actor,
    State(repo): State<InviteRepo>,
    Path(company_id): Path<String>,
    Query(filter): Query<InviteFilter>,
) -> Result<Json<Vec<Invite>>, (StatusCode, Json<Value>)> {
    authorize_company_access(&actor, &company_id)?;
    Ok(Json(
        repo.0.list_by_company(&company_id, filter.state.as_deref()),
    ))
}

pub async fn create_invite(
    actor: Actor,
    State(repo): State<InviteRepo>,
    Path(company_id): Path<String>,
    Json(input): Json<CreateInvite>,
) -> Result<(StatusCode, Json<Invite>), (StatusCode, Json<Value>)> {
    authorize_company_access(&actor, &company_id)?;
    Ok((StatusCode::CREATED, Json(repo.0.create(&company_id, input))))
}

pub async fn revoke_invite(
    actor: Actor,
    State(repo): State<InviteRepo>,
    Path((company_id, invite_id)): Path<(String, String)>,
) -> Result<Json<Invite>, (StatusCode, Json<Value>)> {
    authorize_company_access(&actor, &company_id)?;
    match repo.0.revoke(&company_id, &invite_id) {
        Some(invite) => Ok(Json(invite)),
        None => Err((
            StatusCode::NOT_FOUND,
            Json(json!({ "error": "Invite not found" })),
        )),
    }
}
