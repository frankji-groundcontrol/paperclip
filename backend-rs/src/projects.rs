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

/// A project — a company-scoped grouping of work. Ports the core `projects`
/// columns (packages/db/src/schema/projects.ts).
#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Project {
    pub id: String,
    pub company_id: String,
    pub name: String,
    pub status: String,
}

/// Ports the create-project payload (packages/shared/src/validators/project.ts):
/// `name` required, `status` defaulting to "backlog".
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateProject {
    pub name: String,
    #[serde(default = "default_status")]
    pub status: String,
}

fn default_status() -> String {
    "backlog".to_string()
}

/// Ports the partial update-project payload: provided fields are applied.
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateProject {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub status: Option<String>,
}

/// In-memory, company-scoped project repository.
#[derive(Clone, Default)]
pub struct ProjectStore {
    inner: Arc<Mutex<Vec<Project>>>,
}

impl ProjectStore {
    pub fn create(&self, company_id: &str, input: CreateProject) -> Project {
        let project = Project {
            id: Uuid::new_v4().to_string(),
            company_id: company_id.to_string(),
            name: input.name,
            status: input.status,
        };
        self.inner.lock().unwrap().push(project.clone());
        project
    }

    /// Company scoping: only projects belonging to `company_id` are returned.
    pub fn list_by_company(&self, company_id: &str) -> Vec<Project> {
        self.inner
            .lock()
            .unwrap()
            .iter()
            .filter(|project| project.company_id == company_id)
            .cloned()
            .collect()
    }

    pub fn update(
        &self,
        company_id: &str,
        project_id: &str,
        patch: UpdateProject,
    ) -> Option<Project> {
        let mut guard = self.inner.lock().unwrap();
        let project = guard
            .iter_mut()
            .find(|project| project.company_id == company_id && project.id == project_id)?;
        if let Some(name) = patch.name {
            project.name = name;
        }
        if let Some(status) = patch.status {
            project.status = status;
        }
        Some(project.clone())
    }

    pub fn delete(&self, company_id: &str, project_id: &str) -> bool {
        let mut guard = self.inner.lock().unwrap();
        let before = guard.len();
        guard.retain(|project| !(project.company_id == company_id && project.id == project_id));
        guard.len() != before
    }
}

/// The project persistence interface. Both the in-memory [`ProjectStore`] and the
/// SQLite-backed store (see `crate::db`) implement this.
pub trait ProjectRepository {
    fn list_by_company(&self, company_id: &str) -> Vec<Project>;
    fn create(&self, company_id: &str, input: CreateProject) -> Project;
    fn update(&self, company_id: &str, project_id: &str, patch: UpdateProject) -> Option<Project>;
    fn delete(&self, company_id: &str, project_id: &str) -> bool;
}

impl ProjectRepository for ProjectStore {
    fn list_by_company(&self, company_id: &str) -> Vec<Project> {
        ProjectStore::list_by_company(self, company_id)
    }
    fn create(&self, company_id: &str, input: CreateProject) -> Project {
        ProjectStore::create(self, company_id, input)
    }
    fn update(&self, company_id: &str, project_id: &str, patch: UpdateProject) -> Option<Project> {
        ProjectStore::update(self, company_id, project_id, patch)
    }
    fn delete(&self, company_id: &str, project_id: &str) -> bool {
        ProjectStore::delete(self, company_id, project_id)
    }
}

/// Cloneable handle to whichever [`ProjectRepository`] backs the running app.
#[derive(Clone)]
pub struct ProjectRepo(pub Arc<dyn ProjectRepository + Send + Sync>);

impl Default for ProjectRepo {
    fn default() -> Self {
        ProjectRepo(Arc::new(ProjectStore::default()))
    }
}

pub async fn list_projects(
    actor: Actor,
    State(repo): State<ProjectRepo>,
    Path(company_id): Path<String>,
) -> Result<Json<Vec<Project>>, (StatusCode, Json<Value>)> {
    authorize_company_access(&actor, &company_id)?;
    Ok(Json(repo.0.list_by_company(&company_id)))
}

pub async fn create_project(
    actor: Actor,
    State(repo): State<ProjectRepo>,
    Path(company_id): Path<String>,
    Json(input): Json<CreateProject>,
) -> Result<(StatusCode, Json<Project>), (StatusCode, Json<Value>)> {
    authorize_company_access(&actor, &company_id)?;
    Ok((StatusCode::CREATED, Json(repo.0.create(&company_id, input))))
}

fn project_not_found() -> (StatusCode, Json<Value>) {
    (
        StatusCode::NOT_FOUND,
        Json(json!({ "error": "Project not found" })),
    )
}

pub async fn update_project(
    actor: Actor,
    State(repo): State<ProjectRepo>,
    Path((company_id, project_id)): Path<(String, String)>,
    Json(patch): Json<UpdateProject>,
) -> Result<Json<Project>, (StatusCode, Json<Value>)> {
    authorize_company_access(&actor, &company_id)?;
    match repo.0.update(&company_id, &project_id, patch) {
        Some(project) => Ok(Json(project)),
        None => Err(project_not_found()),
    }
}

pub async fn delete_project(
    actor: Actor,
    State(repo): State<ProjectRepo>,
    Path((company_id, project_id)): Path<(String, String)>,
) -> Result<StatusCode, (StatusCode, Json<Value>)> {
    authorize_company_access(&actor, &company_id)?;
    if repo.0.delete(&company_id, &project_id) {
        Ok(StatusCode::NO_CONTENT)
    } else {
        Err(project_not_found())
    }
}
