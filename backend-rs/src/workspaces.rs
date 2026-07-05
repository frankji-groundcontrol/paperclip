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

/// An execution workspace — where an agent runs an issue. Ports the core
/// execution-workspace shape; lease/runtime fields land later.
#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Workspace {
    pub id: String,
    pub company_id: String,
    pub issue_id: String,
    pub status: String,
}

/// Ports the create-workspace payload: `issueId` required, `status` defaulting to
/// "starting".
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateWorkspace {
    pub issue_id: String,
    #[serde(default = "default_status")]
    pub status: String,
}

fn default_status() -> String {
    "starting".to_string()
}

pub trait WorkspaceRepository {
    fn list_by_company(&self, company_id: &str) -> Vec<Workspace>;
    fn create(&self, company_id: &str, input: CreateWorkspace) -> Workspace;
    fn update(
        &self,
        company_id: &str,
        workspace_id: &str,
        patch: UpdateWorkspace,
    ) -> Option<Workspace>;
    fn delete(&self, company_id: &str, workspace_id: &str) -> bool;
}

/// Ports the partial update-workspace payload — primarily lifecycle status.
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateWorkspace {
    #[serde(default)]
    pub status: Option<String>,
    #[serde(default)]
    pub issue_id: Option<String>,
}

#[derive(Clone, Default)]
pub struct WorkspaceStore {
    inner: Arc<Mutex<Vec<Workspace>>>,
}

impl WorkspaceStore {
    pub fn create(&self, company_id: &str, input: CreateWorkspace) -> Workspace {
        let workspace = Workspace {
            id: Uuid::new_v4().to_string(),
            company_id: company_id.to_string(),
            issue_id: input.issue_id,
            status: input.status,
        };
        self.inner.lock().unwrap().push(workspace.clone());
        workspace
    }

    pub fn list_by_company(&self, company_id: &str) -> Vec<Workspace> {
        self.inner
            .lock()
            .unwrap()
            .iter()
            .filter(|workspace| workspace.company_id == company_id)
            .cloned()
            .collect()
    }

    pub fn update(
        &self,
        company_id: &str,
        workspace_id: &str,
        patch: UpdateWorkspace,
    ) -> Option<Workspace> {
        let mut guard = self.inner.lock().unwrap();
        let workspace = guard
            .iter_mut()
            .find(|workspace| workspace.company_id == company_id && workspace.id == workspace_id)?;
        if let Some(status) = patch.status {
            workspace.status = status;
        }
        if let Some(issue_id) = patch.issue_id {
            workspace.issue_id = issue_id;
        }
        Some(workspace.clone())
    }

    pub fn delete(&self, company_id: &str, workspace_id: &str) -> bool {
        let mut guard = self.inner.lock().unwrap();
        let before = guard.len();
        guard.retain(|workspace| {
            !(workspace.company_id == company_id && workspace.id == workspace_id)
        });
        guard.len() != before
    }
}

impl WorkspaceRepository for WorkspaceStore {
    fn list_by_company(&self, company_id: &str) -> Vec<Workspace> {
        WorkspaceStore::list_by_company(self, company_id)
    }
    fn create(&self, company_id: &str, input: CreateWorkspace) -> Workspace {
        WorkspaceStore::create(self, company_id, input)
    }
    fn update(
        &self,
        company_id: &str,
        workspace_id: &str,
        patch: UpdateWorkspace,
    ) -> Option<Workspace> {
        WorkspaceStore::update(self, company_id, workspace_id, patch)
    }
    fn delete(&self, company_id: &str, workspace_id: &str) -> bool {
        WorkspaceStore::delete(self, company_id, workspace_id)
    }
}

/// Cloneable handle to whichever [`WorkspaceRepository`] backs the running app.
#[derive(Clone)]
pub struct WorkspaceRepo(pub Arc<dyn WorkspaceRepository + Send + Sync>);

impl Default for WorkspaceRepo {
    fn default() -> Self {
        WorkspaceRepo(Arc::new(WorkspaceStore::default()))
    }
}

pub async fn list_workspaces(
    actor: Actor,
    State(repo): State<WorkspaceRepo>,
    Path(company_id): Path<String>,
) -> Result<Json<Vec<Workspace>>, (StatusCode, Json<Value>)> {
    authorize_company_access(&actor, &company_id)?;
    Ok(Json(repo.0.list_by_company(&company_id)))
}

pub async fn create_workspace(
    actor: Actor,
    State(repo): State<WorkspaceRepo>,
    Path(company_id): Path<String>,
    Json(input): Json<CreateWorkspace>,
) -> Result<(StatusCode, Json<Workspace>), (StatusCode, Json<Value>)> {
    authorize_company_access(&actor, &company_id)?;
    Ok((StatusCode::CREATED, Json(repo.0.create(&company_id, input))))
}

fn workspace_not_found() -> (StatusCode, Json<Value>) {
    (
        StatusCode::NOT_FOUND,
        Json(json!({ "error": "Workspace not found" })),
    )
}

pub async fn update_workspace(
    actor: Actor,
    State(repo): State<WorkspaceRepo>,
    Path((company_id, workspace_id)): Path<(String, String)>,
    Json(patch): Json<UpdateWorkspace>,
) -> Result<Json<Workspace>, (StatusCode, Json<Value>)> {
    authorize_company_access(&actor, &company_id)?;
    match repo.0.update(&company_id, &workspace_id, patch) {
        Some(workspace) => Ok(Json(workspace)),
        None => Err(workspace_not_found()),
    }
}

pub async fn delete_workspace(
    actor: Actor,
    State(repo): State<WorkspaceRepo>,
    Path((company_id, workspace_id)): Path<(String, String)>,
) -> Result<StatusCode, (StatusCode, Json<Value>)> {
    authorize_company_access(&actor, &company_id)?;
    if repo.0.delete(&company_id, &workspace_id) {
        Ok(StatusCode::NO_CONTENT)
    } else {
        Err(workspace_not_found())
    }
}
