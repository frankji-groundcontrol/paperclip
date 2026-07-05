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

/// A company execution environment. Ports the core columns of
/// server/src/routes/environments.ts (packages/shared createEnvironmentSchema).
#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Environment {
    pub id: String,
    pub company_id: String,
    pub name: String,
    pub description: Option<String>,
    pub driver: String,
    pub status: String,
    pub config: Value,
    pub env_vars: Value,
    pub metadata: Option<Value>,
}

fn empty_object() -> Value {
    json!({})
}

fn default_status() -> String {
    "active".to_string()
}

/// Ports the create-environment payload: `name` + `driver` required; `status`
/// defaults to "active"; `config`/`envVars` default to `{}`; description/metadata
/// nullable.
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateEnvironment {
    pub name: String,
    #[serde(default)]
    pub description: Option<String>,
    pub driver: String,
    #[serde(default = "default_status")]
    pub status: String,
    #[serde(default = "empty_object")]
    pub config: Value,
    #[serde(default = "empty_object")]
    pub env_vars: Value,
    #[serde(default)]
    pub metadata: Option<Value>,
}

/// Ports the partial update-environment payload.
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateEnvironment {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub driver: Option<String>,
    #[serde(default)]
    pub status: Option<String>,
    #[serde(default)]
    pub config: Option<Value>,
    #[serde(default)]
    pub env_vars: Option<Value>,
    #[serde(default)]
    pub metadata: Option<Value>,
}

/// In-memory, company-scoped environment repository.
#[derive(Clone, Default)]
pub struct EnvironmentStore {
    inner: Arc<Mutex<Vec<Environment>>>,
}

impl EnvironmentStore {
    pub fn create(&self, company_id: &str, input: CreateEnvironment) -> Environment {
        let environment = Environment {
            id: Uuid::new_v4().to_string(),
            company_id: company_id.to_string(),
            name: input.name,
            description: input.description,
            driver: input.driver,
            status: input.status,
            config: input.config,
            env_vars: input.env_vars,
            metadata: input.metadata,
        };
        self.inner.lock().unwrap().push(environment.clone());
        environment
    }

    pub fn list_by_company(&self, company_id: &str) -> Vec<Environment> {
        self.inner
            .lock()
            .unwrap()
            .iter()
            .filter(|env| env.company_id == company_id)
            .cloned()
            .collect()
    }

    pub fn update(
        &self,
        company_id: &str,
        environment_id: &str,
        patch: UpdateEnvironment,
    ) -> Option<Environment> {
        let mut guard = self.inner.lock().unwrap();
        let env = guard
            .iter_mut()
            .find(|env| env.company_id == company_id && env.id == environment_id)?;
        if let Some(name) = patch.name {
            env.name = name;
        }
        if let Some(description) = patch.description {
            env.description = Some(description);
        }
        if let Some(driver) = patch.driver {
            env.driver = driver;
        }
        if let Some(status) = patch.status {
            env.status = status;
        }
        if let Some(config) = patch.config {
            env.config = config;
        }
        if let Some(env_vars) = patch.env_vars {
            env.env_vars = env_vars;
        }
        if let Some(metadata) = patch.metadata {
            env.metadata = Some(metadata);
        }
        Some(env.clone())
    }

    pub fn delete(&self, company_id: &str, environment_id: &str) -> bool {
        let mut guard = self.inner.lock().unwrap();
        let before = guard.len();
        guard.retain(|env| !(env.company_id == company_id && env.id == environment_id));
        guard.len() != before
    }
}

pub trait EnvironmentRepository {
    fn list_by_company(&self, company_id: &str) -> Vec<Environment>;
    fn create(&self, company_id: &str, input: CreateEnvironment) -> Environment;
    fn update(
        &self,
        company_id: &str,
        environment_id: &str,
        patch: UpdateEnvironment,
    ) -> Option<Environment>;
    fn delete(&self, company_id: &str, environment_id: &str) -> bool;
}

impl EnvironmentRepository for EnvironmentStore {
    fn list_by_company(&self, company_id: &str) -> Vec<Environment> {
        EnvironmentStore::list_by_company(self, company_id)
    }
    fn create(&self, company_id: &str, input: CreateEnvironment) -> Environment {
        EnvironmentStore::create(self, company_id, input)
    }
    fn update(
        &self,
        company_id: &str,
        environment_id: &str,
        patch: UpdateEnvironment,
    ) -> Option<Environment> {
        EnvironmentStore::update(self, company_id, environment_id, patch)
    }
    fn delete(&self, company_id: &str, environment_id: &str) -> bool {
        EnvironmentStore::delete(self, company_id, environment_id)
    }
}

/// Cloneable handle to whichever [`EnvironmentRepository`] backs the running app.
#[derive(Clone)]
pub struct EnvironmentRepo(pub Arc<dyn EnvironmentRepository + Send + Sync>);

impl Default for EnvironmentRepo {
    fn default() -> Self {
        EnvironmentRepo(Arc::new(EnvironmentStore::default()))
    }
}

pub async fn list_environments(
    actor: Actor,
    State(repo): State<EnvironmentRepo>,
    Path(company_id): Path<String>,
) -> Result<Json<Vec<Environment>>, (StatusCode, Json<Value>)> {
    authorize_company_access(&actor, &company_id)?;
    Ok(Json(repo.0.list_by_company(&company_id)))
}

pub async fn create_environment(
    actor: Actor,
    State(repo): State<EnvironmentRepo>,
    Path(company_id): Path<String>,
    Json(input): Json<CreateEnvironment>,
) -> Result<(StatusCode, Json<Environment>), (StatusCode, Json<Value>)> {
    authorize_company_access(&actor, &company_id)?;
    Ok((StatusCode::CREATED, Json(repo.0.create(&company_id, input))))
}

fn environment_not_found() -> (StatusCode, Json<Value>) {
    (
        StatusCode::NOT_FOUND,
        Json(json!({ "error": "Environment not found" })),
    )
}

pub async fn update_environment(
    actor: Actor,
    State(repo): State<EnvironmentRepo>,
    Path((company_id, environment_id)): Path<(String, String)>,
    Json(patch): Json<UpdateEnvironment>,
) -> Result<Json<Environment>, (StatusCode, Json<Value>)> {
    authorize_company_access(&actor, &company_id)?;
    match repo.0.update(&company_id, &environment_id, patch) {
        Some(env) => Ok(Json(env)),
        None => Err(environment_not_found()),
    }
}

pub async fn delete_environment(
    actor: Actor,
    State(repo): State<EnvironmentRepo>,
    Path((company_id, environment_id)): Path<(String, String)>,
) -> Result<StatusCode, (StatusCode, Json<Value>)> {
    authorize_company_access(&actor, &company_id)?;
    if repo.0.delete(&company_id, &environment_id) {
        Ok(StatusCode::NO_CONTENT)
    } else {
        Err(environment_not_found())
    }
}
