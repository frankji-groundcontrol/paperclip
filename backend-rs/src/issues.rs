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

/// An issue — the unit of work in the control plane. Ports the core `issues`
/// columns (packages/db/src/schema/issues.ts); fuller fields (assignee, project,
/// parent, origin, timestamps) land in later slices.
#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Issue {
    pub id: String,
    pub company_id: String,
    pub title: String,
    pub status: String,
}

/// Ports the create-issue payload (packages/shared/src/validators/issue.ts):
/// `title` required, `status` defaulting to "backlog" (the schema column default).
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateIssue {
    pub title: String,
    #[serde(default = "default_status")]
    pub status: String,
}

fn default_status() -> String {
    "backlog".to_string()
}

/// Ports the partial update-issue payload: any provided field is applied; others
/// are left unchanged.
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateIssue {
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default)]
    pub status: Option<String>,
}

/// In-memory, company-scoped issue repository. Replaced by a Postgres-backed
/// implementation behind the same interface in a later slice.
#[derive(Clone, Default)]
pub struct IssueStore {
    inner: Arc<Mutex<Vec<Issue>>>,
}

impl IssueStore {
    pub fn create(&self, company_id: &str, input: CreateIssue) -> Issue {
        let issue = Issue {
            id: Uuid::new_v4().to_string(),
            company_id: company_id.to_string(),
            title: input.title,
            status: input.status,
        };
        self.inner.lock().unwrap().push(issue.clone());
        issue
    }

    /// Company scoping: only issues belonging to `company_id` are returned.
    pub fn list_by_company(&self, company_id: &str) -> Vec<Issue> {
        self.inner
            .lock()
            .unwrap()
            .iter()
            .filter(|issue| issue.company_id == company_id)
            .cloned()
            .collect()
    }

    pub fn update(&self, company_id: &str, issue_id: &str, patch: UpdateIssue) -> Option<Issue> {
        let mut guard = self.inner.lock().unwrap();
        let issue = guard
            .iter_mut()
            .find(|issue| issue.company_id == company_id && issue.id == issue_id)?;
        if let Some(title) = patch.title {
            issue.title = title;
        }
        if let Some(status) = patch.status {
            issue.status = status;
        }
        Some(issue.clone())
    }

    pub fn delete(&self, company_id: &str, issue_id: &str) -> bool {
        let mut guard = self.inner.lock().unwrap();
        let before = guard.len();
        guard.retain(|issue| !(issue.company_id == company_id && issue.id == issue_id));
        guard.len() != before
    }
}

/// The issue persistence interface. Both the in-memory [`IssueStore`] and the
/// SQLite-backed store (see `crate::db`) implement this.
pub trait IssueRepository {
    fn list_by_company(&self, company_id: &str) -> Vec<Issue>;
    fn create(&self, company_id: &str, input: CreateIssue) -> Issue;
    fn update(&self, company_id: &str, issue_id: &str, patch: UpdateIssue) -> Option<Issue>;
    fn delete(&self, company_id: &str, issue_id: &str) -> bool;
}

impl IssueRepository for IssueStore {
    fn list_by_company(&self, company_id: &str) -> Vec<Issue> {
        IssueStore::list_by_company(self, company_id)
    }
    fn create(&self, company_id: &str, input: CreateIssue) -> Issue {
        IssueStore::create(self, company_id, input)
    }
    fn update(&self, company_id: &str, issue_id: &str, patch: UpdateIssue) -> Option<Issue> {
        IssueStore::update(self, company_id, issue_id, patch)
    }
    fn delete(&self, company_id: &str, issue_id: &str) -> bool {
        IssueStore::delete(self, company_id, issue_id)
    }
}

/// Cloneable handle to whichever [`IssueRepository`] backs the running app.
#[derive(Clone)]
pub struct IssueRepo(pub Arc<dyn IssueRepository + Send + Sync>);

impl Default for IssueRepo {
    fn default() -> Self {
        IssueRepo(Arc::new(IssueStore::default()))
    }
}

pub async fn list_issues(
    actor: Actor,
    State(repo): State<IssueRepo>,
    Path(company_id): Path<String>,
) -> Result<Json<Vec<Issue>>, (StatusCode, Json<Value>)> {
    authorize_company_access(&actor, &company_id)?;
    Ok(Json(repo.0.list_by_company(&company_id)))
}

pub async fn create_issue(
    actor: Actor,
    State(repo): State<IssueRepo>,
    Path(company_id): Path<String>,
    Json(input): Json<CreateIssue>,
) -> Result<(StatusCode, Json<Issue>), (StatusCode, Json<Value>)> {
    authorize_company_access(&actor, &company_id)?;
    Ok((StatusCode::CREATED, Json(repo.0.create(&company_id, input))))
}

fn issue_not_found() -> (StatusCode, Json<Value>) {
    (
        StatusCode::NOT_FOUND,
        Json(json!({ "error": "Issue not found" })),
    )
}

pub async fn update_issue(
    actor: Actor,
    State(repo): State<IssueRepo>,
    Path((company_id, issue_id)): Path<(String, String)>,
    Json(patch): Json<UpdateIssue>,
) -> Result<Json<Issue>, (StatusCode, Json<Value>)> {
    authorize_company_access(&actor, &company_id)?;
    match repo.0.update(&company_id, &issue_id, patch) {
        Some(issue) => Ok(Json(issue)),
        None => Err(issue_not_found()),
    }
}

pub async fn delete_issue(
    actor: Actor,
    State(repo): State<IssueRepo>,
    Path((company_id, issue_id)): Path<(String, String)>,
) -> Result<StatusCode, (StatusCode, Json<Value>)> {
    authorize_company_access(&actor, &company_id)?;
    if repo.0.delete(&company_id, &issue_id) {
        Ok(StatusCode::NO_CONTENT)
    } else {
        Err(issue_not_found())
    }
}
