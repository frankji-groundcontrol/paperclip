use std::sync::{Arc, Mutex};

use axum::{extract::State, http::StatusCode, Json};
use serde_json::{json, Value};

use crate::auth::{require_board, Actor};

/// The default `general` settings block (server/src/services/instance-settings.ts
/// normalizeGeneralSettings).
fn default_general() -> Value {
    json!({
        "censorUsernameInLogs": false,
        "keyboardShortcuts": false,
        "feedbackDataSharingPreference": "prompt",
        "backupRetention": { "dailyDays": 7, "weeklyWeeks": 4, "monthlyMonths": 1 },
    })
}

/// The default `experimental` flag block (normalizeExperimentalSettings).
fn default_experimental() -> Value {
    json!({
        "enableEnvironments": false,
        "enableIsolatedWorkspaces": false,
        "enableStreamlinedLeftNavigation": true,
        "enablePipelines": false,
        "enableConferenceRoomChat": false,
        "enableIssuePlanDecompositions": false,
        "enableExperimentalFileViewer": false,
        "enableTaskWatchdogs": false,
        "enableCloudSync": false,
        "enableExternalObjects": false,
        "enableServerInfoDebugView": false,
        "autoRestartDevServerWhenIdle": false,
        "enableIssueGraphLivenessAutoRecovery": false,
        "issueGraphLivenessAutoRecoveryLookbackHours": 24,
    })
}

struct InstanceState {
    default_environment_id: Option<String>,
    general: Value,
    experimental: Value,
}

impl Default for InstanceState {
    fn default() -> Self {
        Self {
            default_environment_id: None,
            general: default_general(),
            experimental: default_experimental(),
        }
    }
}

/// Merges the top-level keys of `patch` (an object) into `target` (an object).
fn merge_object(target: &mut Value, patch: Value) {
    if let (Some(target_map), Value::Object(patch_map)) = (target.as_object_mut(), patch) {
        for (key, value) in patch_map {
            target_map.insert(key, value);
        }
    }
}

/// The instance settings singleton store.
#[derive(Clone, Default)]
pub struct InstanceSettingsStore {
    inner: Arc<Mutex<InstanceState>>,
}

impl InstanceSettingsStore {
    fn snapshot(state: &InstanceState) -> Value {
        json!({
            "id": "instance",
            "defaultEnvironmentId": state.default_environment_id.clone(),
            "general": state.general.clone(),
            "experimental": state.experimental.clone(),
        })
    }

    pub fn get(&self) -> Value {
        Self::snapshot(&self.inner.lock().unwrap())
    }

    pub fn update(&self, patch: Value) -> Value {
        let mut state = self.inner.lock().unwrap();
        if let Some(map) = patch.as_object() {
            if map.contains_key("defaultEnvironmentId") {
                state.default_environment_id =
                    map["defaultEnvironmentId"].as_str().map(|s| s.to_string());
            }
        }
        Self::snapshot(&state)
    }

    pub fn get_general(&self) -> Value {
        self.inner.lock().unwrap().general.clone()
    }

    pub fn update_general(&self, patch: Value) -> Value {
        let mut state = self.inner.lock().unwrap();
        merge_object(&mut state.general, patch);
        state.general.clone()
    }

    pub fn get_experimental(&self) -> Value {
        self.inner.lock().unwrap().experimental.clone()
    }

    pub fn update_experimental(&self, patch: Value) -> Value {
        let mut state = self.inner.lock().unwrap();
        merge_object(&mut state.experimental, patch);
        state.experimental.clone()
    }
}

pub trait InstanceSettingsRepository {
    fn get(&self) -> Value;
    fn update(&self, patch: Value) -> Value;
    fn get_general(&self) -> Value;
    fn update_general(&self, patch: Value) -> Value;
    fn get_experimental(&self) -> Value;
    fn update_experimental(&self, patch: Value) -> Value;
}

impl InstanceSettingsRepository for InstanceSettingsStore {
    fn get(&self) -> Value {
        InstanceSettingsStore::get(self)
    }
    fn update(&self, patch: Value) -> Value {
        InstanceSettingsStore::update(self, patch)
    }
    fn get_general(&self) -> Value {
        InstanceSettingsStore::get_general(self)
    }
    fn update_general(&self, patch: Value) -> Value {
        InstanceSettingsStore::update_general(self, patch)
    }
    fn get_experimental(&self) -> Value {
        InstanceSettingsStore::get_experimental(self)
    }
    fn update_experimental(&self, patch: Value) -> Value {
        InstanceSettingsStore::update_experimental(self, patch)
    }
}

/// Cloneable handle to whichever [`InstanceSettingsRepository`] backs the app.
#[derive(Clone)]
pub struct InstanceSettingsRepo(pub Arc<dyn InstanceSettingsRepository + Send + Sync>);

impl Default for InstanceSettingsRepo {
    fn default() -> Self {
        InstanceSettingsRepo(Arc::new(InstanceSettingsStore::default()))
    }
}

pub async fn get_instance_settings(
    actor: Actor,
    State(repo): State<InstanceSettingsRepo>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    require_board(&actor)?;
    Ok(Json(repo.0.get()))
}

pub async fn patch_instance_settings(
    actor: Actor,
    State(repo): State<InstanceSettingsRepo>,
    Json(patch): Json<Value>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    require_board(&actor)?;
    Ok(Json(repo.0.update(patch)))
}

pub async fn get_instance_general(
    actor: Actor,
    State(repo): State<InstanceSettingsRepo>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    require_board(&actor)?;
    Ok(Json(repo.0.get_general()))
}

pub async fn patch_instance_general(
    actor: Actor,
    State(repo): State<InstanceSettingsRepo>,
    Json(patch): Json<Value>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    require_board(&actor)?;
    Ok(Json(repo.0.update_general(patch)))
}

pub async fn get_instance_experimental(
    actor: Actor,
    State(repo): State<InstanceSettingsRepo>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    require_board(&actor)?;
    Ok(Json(repo.0.get_experimental()))
}

pub async fn patch_instance_experimental(
    actor: Actor,
    State(repo): State<InstanceSettingsRepo>,
    Json(patch): Json<Value>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    require_board(&actor)?;
    Ok(Json(repo.0.update_experimental(patch)))
}
