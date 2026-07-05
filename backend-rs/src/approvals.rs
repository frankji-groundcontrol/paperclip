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

/// An approval — a governed-action gate on an issue. Ports the core
/// `issue_approvals` columns (packages/db/src/schema/issue_approvals.ts).
#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Approval {
    pub id: String,
    pub company_id: String,
    pub issue_id: String,
    pub status: String,
}

/// Ports the create-approval payload: `issueId` required, `status` defaulting to
/// "pending" (see APPROVAL_STATUSES).
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateApproval {
    pub issue_id: String,
    #[serde(default = "default_status")]
    pub status: String,
}

fn default_status() -> String {
    "pending".to_string()
}

/// Ports the partial update-approval payload — primarily the decision status.
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateApproval {
    #[serde(default)]
    pub status: Option<String>,
    #[serde(default)]
    pub issue_id: Option<String>,
}

/// The approval persistence interface. In-memory for now; a SQLite-backed store
/// implements this in a later slice (same pattern as the other domains).
pub trait ApprovalRepository {
    fn list_by_company(&self, company_id: &str) -> Vec<Approval>;
    fn create(&self, company_id: &str, input: CreateApproval) -> Approval;
    fn update(
        &self,
        company_id: &str,
        approval_id: &str,
        patch: UpdateApproval,
    ) -> Option<Approval>;
    fn delete(&self, company_id: &str, approval_id: &str) -> bool;
}

#[derive(Clone, Default)]
pub struct ApprovalStore {
    inner: Arc<Mutex<Vec<Approval>>>,
}

impl ApprovalStore {
    pub fn create(&self, company_id: &str, input: CreateApproval) -> Approval {
        let approval = Approval {
            id: Uuid::new_v4().to_string(),
            company_id: company_id.to_string(),
            issue_id: input.issue_id,
            status: input.status,
        };
        self.inner.lock().unwrap().push(approval.clone());
        approval
    }

    pub fn list_by_company(&self, company_id: &str) -> Vec<Approval> {
        self.inner
            .lock()
            .unwrap()
            .iter()
            .filter(|approval| approval.company_id == company_id)
            .cloned()
            .collect()
    }

    pub fn update(
        &self,
        company_id: &str,
        approval_id: &str,
        patch: UpdateApproval,
    ) -> Option<Approval> {
        let mut guard = self.inner.lock().unwrap();
        let approval = guard
            .iter_mut()
            .find(|approval| approval.company_id == company_id && approval.id == approval_id)?;
        if let Some(status) = patch.status {
            approval.status = status;
        }
        if let Some(issue_id) = patch.issue_id {
            approval.issue_id = issue_id;
        }
        Some(approval.clone())
    }

    pub fn delete(&self, company_id: &str, approval_id: &str) -> bool {
        let mut guard = self.inner.lock().unwrap();
        let before = guard.len();
        guard.retain(|approval| !(approval.company_id == company_id && approval.id == approval_id));
        guard.len() != before
    }
}

impl ApprovalRepository for ApprovalStore {
    fn list_by_company(&self, company_id: &str) -> Vec<Approval> {
        ApprovalStore::list_by_company(self, company_id)
    }
    fn create(&self, company_id: &str, input: CreateApproval) -> Approval {
        ApprovalStore::create(self, company_id, input)
    }
    fn update(
        &self,
        company_id: &str,
        approval_id: &str,
        patch: UpdateApproval,
    ) -> Option<Approval> {
        ApprovalStore::update(self, company_id, approval_id, patch)
    }
    fn delete(&self, company_id: &str, approval_id: &str) -> bool {
        ApprovalStore::delete(self, company_id, approval_id)
    }
}

/// Cloneable handle to whichever [`ApprovalRepository`] backs the running app.
#[derive(Clone)]
pub struct ApprovalRepo(pub Arc<dyn ApprovalRepository + Send + Sync>);

impl Default for ApprovalRepo {
    fn default() -> Self {
        ApprovalRepo(Arc::new(ApprovalStore::default()))
    }
}

pub async fn list_approvals(
    actor: Actor,
    State(repo): State<ApprovalRepo>,
    Path(company_id): Path<String>,
) -> Result<Json<Vec<Approval>>, (StatusCode, Json<Value>)> {
    authorize_company_access(&actor, &company_id)?;
    Ok(Json(repo.0.list_by_company(&company_id)))
}

pub async fn create_approval(
    actor: Actor,
    State(repo): State<ApprovalRepo>,
    Path(company_id): Path<String>,
    Json(input): Json<CreateApproval>,
) -> Result<(StatusCode, Json<Approval>), (StatusCode, Json<Value>)> {
    authorize_company_access(&actor, &company_id)?;
    Ok((StatusCode::CREATED, Json(repo.0.create(&company_id, input))))
}

fn approval_not_found() -> (StatusCode, Json<Value>) {
    (
        StatusCode::NOT_FOUND,
        Json(json!({ "error": "Approval not found" })),
    )
}

pub async fn update_approval(
    actor: Actor,
    State(repo): State<ApprovalRepo>,
    Path((company_id, approval_id)): Path<(String, String)>,
    Json(patch): Json<UpdateApproval>,
) -> Result<Json<Approval>, (StatusCode, Json<Value>)> {
    authorize_company_access(&actor, &company_id)?;
    match repo.0.update(&company_id, &approval_id, patch) {
        Some(approval) => Ok(Json(approval)),
        None => Err(approval_not_found()),
    }
}

pub async fn delete_approval(
    actor: Actor,
    State(repo): State<ApprovalRepo>,
    Path((company_id, approval_id)): Path<(String, String)>,
) -> Result<StatusCode, (StatusCode, Json<Value>)> {
    authorize_company_access(&actor, &company_id)?;
    if repo.0.delete(&company_id, &approval_id) {
        Ok(StatusCode::NO_CONTENT)
    } else {
        Err(approval_not_found())
    }
}
