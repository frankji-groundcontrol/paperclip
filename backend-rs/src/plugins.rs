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

/// A company-installed plugin. Ports the core `plugin_company_settings` shape.
#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CompanyPlugin {
    pub id: String,
    pub company_id: String,
    pub plugin_id: String,
    pub enabled: bool,
}

/// Ports the install-plugin payload: `pluginId` required, `enabled` defaulting to
/// true.
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InstallPlugin {
    pub plugin_id: String,
    #[serde(default = "default_enabled")]
    pub enabled: bool,
}

fn default_enabled() -> bool {
    true
}

pub trait PluginRepository {
    fn list_by_company(&self, company_id: &str) -> Vec<CompanyPlugin>;
    fn create(&self, company_id: &str, input: InstallPlugin) -> CompanyPlugin;
    fn update(
        &self,
        company_id: &str,
        plugin_id: &str,
        patch: UpdatePlugin,
    ) -> Option<CompanyPlugin>;
    fn delete(&self, company_id: &str, plugin_id: &str) -> bool;
}

/// Ports the partial update-plugin payload — toggling `enabled`.
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdatePlugin {
    #[serde(default)]
    pub enabled: Option<bool>,
}

#[derive(Clone, Default)]
pub struct PluginStore {
    inner: Arc<Mutex<Vec<CompanyPlugin>>>,
}

impl PluginStore {
    pub fn create(&self, company_id: &str, input: InstallPlugin) -> CompanyPlugin {
        let plugin = CompanyPlugin {
            id: Uuid::new_v4().to_string(),
            company_id: company_id.to_string(),
            plugin_id: input.plugin_id,
            enabled: input.enabled,
        };
        self.inner.lock().unwrap().push(plugin.clone());
        plugin
    }

    pub fn list_by_company(&self, company_id: &str) -> Vec<CompanyPlugin> {
        self.inner
            .lock()
            .unwrap()
            .iter()
            .filter(|plugin| plugin.company_id == company_id)
            .cloned()
            .collect()
    }

    pub fn update(
        &self,
        company_id: &str,
        plugin_id: &str,
        patch: UpdatePlugin,
    ) -> Option<CompanyPlugin> {
        let mut guard = self.inner.lock().unwrap();
        let plugin = guard
            .iter_mut()
            .find(|plugin| plugin.company_id == company_id && plugin.id == plugin_id)?;
        if let Some(enabled) = patch.enabled {
            plugin.enabled = enabled;
        }
        Some(plugin.clone())
    }

    pub fn delete(&self, company_id: &str, plugin_id: &str) -> bool {
        let mut guard = self.inner.lock().unwrap();
        let before = guard.len();
        guard.retain(|plugin| !(plugin.company_id == company_id && plugin.id == plugin_id));
        guard.len() != before
    }
}

impl PluginRepository for PluginStore {
    fn list_by_company(&self, company_id: &str) -> Vec<CompanyPlugin> {
        PluginStore::list_by_company(self, company_id)
    }
    fn create(&self, company_id: &str, input: InstallPlugin) -> CompanyPlugin {
        PluginStore::create(self, company_id, input)
    }
    fn update(
        &self,
        company_id: &str,
        plugin_id: &str,
        patch: UpdatePlugin,
    ) -> Option<CompanyPlugin> {
        PluginStore::update(self, company_id, plugin_id, patch)
    }
    fn delete(&self, company_id: &str, plugin_id: &str) -> bool {
        PluginStore::delete(self, company_id, plugin_id)
    }
}

/// Cloneable handle to whichever [`PluginRepository`] backs the running app.
#[derive(Clone)]
pub struct PluginRepo(pub Arc<dyn PluginRepository + Send + Sync>);

impl Default for PluginRepo {
    fn default() -> Self {
        PluginRepo(Arc::new(PluginStore::default()))
    }
}

pub async fn list_plugins(
    actor: Actor,
    State(repo): State<PluginRepo>,
    Path(company_id): Path<String>,
) -> Result<Json<Vec<CompanyPlugin>>, (StatusCode, Json<Value>)> {
    authorize_company_access(&actor, &company_id)?;
    Ok(Json(repo.0.list_by_company(&company_id)))
}

pub async fn install_plugin(
    actor: Actor,
    State(repo): State<PluginRepo>,
    Path(company_id): Path<String>,
    Json(input): Json<InstallPlugin>,
) -> Result<(StatusCode, Json<CompanyPlugin>), (StatusCode, Json<Value>)> {
    authorize_company_access(&actor, &company_id)?;
    Ok((StatusCode::CREATED, Json(repo.0.create(&company_id, input))))
}

fn plugin_not_found() -> (StatusCode, Json<Value>) {
    (
        StatusCode::NOT_FOUND,
        Json(json!({ "error": "Plugin not found" })),
    )
}

pub async fn update_plugin(
    actor: Actor,
    State(repo): State<PluginRepo>,
    Path((company_id, plugin_id)): Path<(String, String)>,
    Json(patch): Json<UpdatePlugin>,
) -> Result<Json<CompanyPlugin>, (StatusCode, Json<Value>)> {
    authorize_company_access(&actor, &company_id)?;
    match repo.0.update(&company_id, &plugin_id, patch) {
        Some(plugin) => Ok(Json(plugin)),
        None => Err(plugin_not_found()),
    }
}

pub async fn delete_plugin(
    actor: Actor,
    State(repo): State<PluginRepo>,
    Path((company_id, plugin_id)): Path<(String, String)>,
) -> Result<StatusCode, (StatusCode, Json<Value>)> {
    authorize_company_access(&actor, &company_id)?;
    if repo.0.delete(&company_id, &plugin_id) {
        Ok(StatusCode::NO_CONTENT)
    } else {
        Err(plugin_not_found())
    }
}
