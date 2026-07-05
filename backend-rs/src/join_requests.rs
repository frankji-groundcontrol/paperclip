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

/// A request to join a company (from an invite). Ports the OSS-default core of
/// server/src/routes/access.ts join-requests; the membership/agent
/// materialisation on approval is deferred.
#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct JoinRequest {
    pub id: String,
    pub company_id: String,
    pub request_type: String,
    pub requester_name: Option<String>,
    pub status: String,
}

/// Ports the create payload: `requestType` (human/agent) required;
/// `requesterName` optional.
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateJoinRequest {
    pub request_type: String,
    #[serde(default)]
    pub requester_name: Option<String>,
}

/// Optional list filters.
#[derive(Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct JoinRequestFilter {
    #[serde(default)]
    pub status: Option<String>,
    #[serde(default)]
    pub request_type: Option<String>,
}

/// Outcome of an approve/reject transition (pending-only).
pub enum TransitionResult {
    NotFound,
    NotPending,
    Changed(JoinRequest),
}

/// In-memory, company-scoped join-request store.
#[derive(Clone, Default)]
pub struct JoinRequestStore {
    inner: Arc<Mutex<Vec<JoinRequest>>>,
}

impl JoinRequestStore {
    pub fn create(&self, company_id: &str, input: CreateJoinRequest) -> JoinRequest {
        let request = JoinRequest {
            id: Uuid::new_v4().to_string(),
            company_id: company_id.to_string(),
            request_type: input.request_type,
            requester_name: input.requester_name,
            status: "pending_approval".to_string(),
        };
        self.inner.lock().unwrap().push(request.clone());
        request
    }

    pub fn list_by_company(
        &self,
        company_id: &str,
        filter: &JoinRequestFilter,
    ) -> Vec<JoinRequest> {
        self.inner
            .lock()
            .unwrap()
            .iter()
            .filter(|r| r.company_id == company_id)
            .filter(|r| filter.status.as_ref().is_none_or(|s| &r.status == s))
            .filter(|r| {
                filter
                    .request_type
                    .as_ref()
                    .is_none_or(|t| &r.request_type == t)
            })
            .cloned()
            .collect()
    }

    /// Transitions a *pending* request to `to`; anything else is `NotPending`.
    pub fn transition(&self, company_id: &str, request_id: &str, to: &str) -> TransitionResult {
        let mut guard = self.inner.lock().unwrap();
        let Some(request) = guard
            .iter_mut()
            .find(|r| r.company_id == company_id && r.id == request_id)
        else {
            return TransitionResult::NotFound;
        };
        if request.status != "pending_approval" {
            return TransitionResult::NotPending;
        }
        request.status = to.to_string();
        TransitionResult::Changed(request.clone())
    }
}

pub trait JoinRequestRepository {
    fn create(&self, company_id: &str, input: CreateJoinRequest) -> JoinRequest;
    fn list_by_company(&self, company_id: &str, filter: &JoinRequestFilter) -> Vec<JoinRequest>;
    fn transition(&self, company_id: &str, request_id: &str, to: &str) -> TransitionResult;
}

impl JoinRequestRepository for JoinRequestStore {
    fn create(&self, company_id: &str, input: CreateJoinRequest) -> JoinRequest {
        JoinRequestStore::create(self, company_id, input)
    }
    fn list_by_company(&self, company_id: &str, filter: &JoinRequestFilter) -> Vec<JoinRequest> {
        JoinRequestStore::list_by_company(self, company_id, filter)
    }
    fn transition(&self, company_id: &str, request_id: &str, to: &str) -> TransitionResult {
        JoinRequestStore::transition(self, company_id, request_id, to)
    }
}

/// Cloneable handle to whichever [`JoinRequestRepository`] backs the running app.
#[derive(Clone)]
pub struct JoinRequestRepo(pub Arc<dyn JoinRequestRepository + Send + Sync>);

impl Default for JoinRequestRepo {
    fn default() -> Self {
        JoinRequestRepo(Arc::new(JoinRequestStore::default()))
    }
}

pub async fn list_join_requests(
    actor: Actor,
    State(repo): State<JoinRequestRepo>,
    Path(company_id): Path<String>,
    Query(filter): Query<JoinRequestFilter>,
) -> Result<Json<Vec<JoinRequest>>, (StatusCode, Json<Value>)> {
    authorize_company_access(&actor, &company_id)?;
    Ok(Json(repo.0.list_by_company(&company_id, &filter)))
}

pub async fn create_join_request(
    actor: Actor,
    State(repo): State<JoinRequestRepo>,
    Path(company_id): Path<String>,
    Json(input): Json<CreateJoinRequest>,
) -> Result<(StatusCode, Json<JoinRequest>), (StatusCode, Json<Value>)> {
    authorize_company_access(&actor, &company_id)?;
    Ok((StatusCode::CREATED, Json(repo.0.create(&company_id, input))))
}

fn apply_transition(
    repo: &JoinRequestRepo,
    company_id: &str,
    request_id: &str,
    to: &str,
) -> Result<Json<JoinRequest>, (StatusCode, Json<Value>)> {
    match repo.0.transition(company_id, request_id, to) {
        TransitionResult::Changed(request) => Ok(Json(request)),
        TransitionResult::NotFound => Err((
            StatusCode::NOT_FOUND,
            Json(json!({ "error": "Join request not found" })),
        )),
        TransitionResult::NotPending => Err((
            StatusCode::CONFLICT,
            Json(json!({ "error": "Join request is not pending" })),
        )),
    }
}

pub async fn approve_join_request(
    actor: Actor,
    State(repo): State<JoinRequestRepo>,
    Path((company_id, request_id)): Path<(String, String)>,
) -> Result<Json<JoinRequest>, (StatusCode, Json<Value>)> {
    authorize_company_access(&actor, &company_id)?;
    apply_transition(&repo, &company_id, &request_id, "approved")
}

pub async fn reject_join_request(
    actor: Actor,
    State(repo): State<JoinRequestRepo>,
    Path((company_id, request_id)): Path<(String, String)>,
) -> Result<Json<JoinRequest>, (StatusCode, Json<Value>)> {
    authorize_company_access(&actor, &company_id)?;
    apply_transition(&repo, &company_id, &request_id, "rejected")
}
