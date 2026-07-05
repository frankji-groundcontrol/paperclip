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

/// A company-configured agent adapter (Claude, Codex, Cursor, Hermes, …).
#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CompanyAdapter {
    pub id: String,
    pub company_id: String,
    pub adapter_type: String,
    pub enabled: bool,
}

/// Ports the configure-adapter payload: `adapterType` required, `enabled`
/// defaulting to true.
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConfigureAdapter {
    pub adapter_type: String,
    #[serde(default = "default_enabled")]
    pub enabled: bool,
}

fn default_enabled() -> bool {
    true
}

pub trait AdapterRepository {
    fn list_by_company(&self, company_id: &str) -> Vec<CompanyAdapter>;
    fn create(&self, company_id: &str, input: ConfigureAdapter) -> CompanyAdapter;
    fn update(
        &self,
        company_id: &str,
        adapter_id: &str,
        patch: UpdateAdapter,
    ) -> Option<CompanyAdapter>;
    fn delete(&self, company_id: &str, adapter_id: &str) -> bool;
}

/// Ports the partial update-adapter payload — toggling `enabled`.
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateAdapter {
    #[serde(default)]
    pub enabled: Option<bool>,
}

#[derive(Clone, Default)]
pub struct AdapterStore {
    inner: Arc<Mutex<Vec<CompanyAdapter>>>,
}

impl AdapterStore {
    pub fn create(&self, company_id: &str, input: ConfigureAdapter) -> CompanyAdapter {
        let adapter = CompanyAdapter {
            id: Uuid::new_v4().to_string(),
            company_id: company_id.to_string(),
            adapter_type: input.adapter_type,
            enabled: input.enabled,
        };
        self.inner.lock().unwrap().push(adapter.clone());
        adapter
    }

    pub fn list_by_company(&self, company_id: &str) -> Vec<CompanyAdapter> {
        self.inner
            .lock()
            .unwrap()
            .iter()
            .filter(|adapter| adapter.company_id == company_id)
            .cloned()
            .collect()
    }

    pub fn update(
        &self,
        company_id: &str,
        adapter_id: &str,
        patch: UpdateAdapter,
    ) -> Option<CompanyAdapter> {
        let mut guard = self.inner.lock().unwrap();
        let adapter = guard
            .iter_mut()
            .find(|adapter| adapter.company_id == company_id && adapter.id == adapter_id)?;
        if let Some(enabled) = patch.enabled {
            adapter.enabled = enabled;
        }
        Some(adapter.clone())
    }

    pub fn delete(&self, company_id: &str, adapter_id: &str) -> bool {
        let mut guard = self.inner.lock().unwrap();
        let before = guard.len();
        guard.retain(|adapter| !(adapter.company_id == company_id && adapter.id == adapter_id));
        guard.len() != before
    }
}

impl AdapterRepository for AdapterStore {
    fn list_by_company(&self, company_id: &str) -> Vec<CompanyAdapter> {
        AdapterStore::list_by_company(self, company_id)
    }
    fn create(&self, company_id: &str, input: ConfigureAdapter) -> CompanyAdapter {
        AdapterStore::create(self, company_id, input)
    }
    fn update(
        &self,
        company_id: &str,
        adapter_id: &str,
        patch: UpdateAdapter,
    ) -> Option<CompanyAdapter> {
        AdapterStore::update(self, company_id, adapter_id, patch)
    }
    fn delete(&self, company_id: &str, adapter_id: &str) -> bool {
        AdapterStore::delete(self, company_id, adapter_id)
    }
}

/// Cloneable handle to whichever [`AdapterRepository`] backs the running app.
#[derive(Clone)]
pub struct AdapterRepo(pub Arc<dyn AdapterRepository + Send + Sync>);

impl Default for AdapterRepo {
    fn default() -> Self {
        AdapterRepo(Arc::new(AdapterStore::default()))
    }
}

pub async fn list_adapters(
    actor: Actor,
    State(repo): State<AdapterRepo>,
    Path(company_id): Path<String>,
) -> Result<Json<Vec<CompanyAdapter>>, (StatusCode, Json<Value>)> {
    authorize_company_access(&actor, &company_id)?;
    Ok(Json(repo.0.list_by_company(&company_id)))
}

pub async fn configure_adapter(
    actor: Actor,
    State(repo): State<AdapterRepo>,
    Path(company_id): Path<String>,
    Json(input): Json<ConfigureAdapter>,
) -> Result<(StatusCode, Json<CompanyAdapter>), (StatusCode, Json<Value>)> {
    authorize_company_access(&actor, &company_id)?;
    Ok((StatusCode::CREATED, Json(repo.0.create(&company_id, input))))
}

fn adapter_not_found() -> (StatusCode, Json<Value>) {
    (
        StatusCode::NOT_FOUND,
        Json(json!({ "error": "Adapter not found" })),
    )
}

pub async fn update_adapter(
    actor: Actor,
    State(repo): State<AdapterRepo>,
    Path((company_id, adapter_id)): Path<(String, String)>,
    Json(patch): Json<UpdateAdapter>,
) -> Result<Json<CompanyAdapter>, (StatusCode, Json<Value>)> {
    authorize_company_access(&actor, &company_id)?;
    match repo.0.update(&company_id, &adapter_id, patch) {
        Some(adapter) => Ok(Json(adapter)),
        None => Err(adapter_not_found()),
    }
}

pub async fn delete_adapter(
    actor: Actor,
    State(repo): State<AdapterRepo>,
    Path((company_id, adapter_id)): Path<(String, String)>,
) -> Result<StatusCode, (StatusCode, Json<Value>)> {
    authorize_company_access(&actor, &company_id)?;
    if repo.0.delete(&company_id, &adapter_id) {
        Ok(StatusCode::NO_CONTENT)
    } else {
        Err(adapter_not_found())
    }
}
