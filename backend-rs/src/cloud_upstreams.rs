use std::sync::{Arc, Mutex};

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use uuid::Uuid;

use crate::auth::{require_board, Actor};
use crate::instance_settings::InstanceSettingsRepo;

/// A cloud upstream connection. Ports the core of server/src/routes/
/// cloud-upstreams.ts; the OAuth connect handshake and push-runs are deferred.
#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CloudUpstream {
    pub id: String,
    pub company_id: String,
    pub remote_url: String,
    pub status: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RegisterUpstream {
    pub company_id: String,
    pub remote_url: String,
}

/// Optional `?companyId=` list filter.
#[derive(Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct UpstreamFilter {
    #[serde(default)]
    pub company_id: Option<String>,
}

/// In-memory upstream connection store.
#[derive(Clone, Default)]
pub struct CloudUpstreamStore {
    inner: Arc<Mutex<Vec<CloudUpstream>>>,
}

impl CloudUpstreamStore {
    pub fn register(&self, input: RegisterUpstream) -> CloudUpstream {
        let upstream = CloudUpstream {
            id: Uuid::new_v4().to_string(),
            company_id: input.company_id,
            remote_url: input.remote_url,
            status: "connected".to_string(),
        };
        self.inner.lock().unwrap().push(upstream.clone());
        upstream
    }

    pub fn list(&self, company_id: Option<&str>) -> Vec<CloudUpstream> {
        self.inner
            .lock()
            .unwrap()
            .iter()
            .filter(|u| company_id.is_none_or(|c| u.company_id == c))
            .cloned()
            .collect()
    }

    pub fn delete(&self, connection_id: &str) -> bool {
        let mut guard = self.inner.lock().unwrap();
        let before = guard.len();
        guard.retain(|u| u.id != connection_id);
        guard.len() != before
    }
}

pub trait CloudUpstreamRepository {
    fn register(&self, input: RegisterUpstream) -> CloudUpstream;
    fn list(&self, company_id: Option<&str>) -> Vec<CloudUpstream>;
    fn delete(&self, connection_id: &str) -> bool;
}

impl CloudUpstreamRepository for CloudUpstreamStore {
    fn register(&self, input: RegisterUpstream) -> CloudUpstream {
        CloudUpstreamStore::register(self, input)
    }
    fn list(&self, company_id: Option<&str>) -> Vec<CloudUpstream> {
        CloudUpstreamStore::list(self, company_id)
    }
    fn delete(&self, connection_id: &str) -> bool {
        CloudUpstreamStore::delete(self, connection_id)
    }
}

/// Cloneable handle to whichever [`CloudUpstreamRepository`] backs the app.
#[derive(Clone)]
pub struct CloudUpstreamRepo(pub Arc<dyn CloudUpstreamRepository + Send + Sync>);

impl Default for CloudUpstreamRepo {
    fn default() -> Self {
        CloudUpstreamRepo(Arc::new(CloudUpstreamStore::default()))
    }
}

/// Gate: cloud sync must be enabled in instance settings (experimental flag),
/// else 404 — ports `assertEnabled` (cross-domain read of instance-settings).
fn assert_cloud_sync_enabled(
    settings: &InstanceSettingsRepo,
) -> Result<(), (StatusCode, Json<Value>)> {
    let experimental = settings.0.get_experimental();
    let enabled = experimental
        .get("enableCloudSync")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    if enabled {
        Ok(())
    } else {
        Err((
            StatusCode::NOT_FOUND,
            Json(json!({ "error": "Cloud sync is not enabled" })),
        ))
    }
}

pub async fn list_cloud_upstreams(
    actor: Actor,
    State(repo): State<CloudUpstreamRepo>,
    State(settings): State<InstanceSettingsRepo>,
    Query(filter): Query<UpstreamFilter>,
) -> Result<Json<Vec<CloudUpstream>>, (StatusCode, Json<Value>)> {
    require_board(&actor)?;
    assert_cloud_sync_enabled(&settings)?;
    Ok(Json(repo.0.list(filter.company_id.as_deref())))
}

pub async fn register_cloud_upstream(
    actor: Actor,
    State(repo): State<CloudUpstreamRepo>,
    State(settings): State<InstanceSettingsRepo>,
    Json(input): Json<RegisterUpstream>,
) -> Result<(StatusCode, Json<CloudUpstream>), (StatusCode, Json<Value>)> {
    require_board(&actor)?;
    assert_cloud_sync_enabled(&settings)?;
    Ok((StatusCode::CREATED, Json(repo.0.register(input))))
}

pub async fn delete_cloud_upstream(
    actor: Actor,
    State(repo): State<CloudUpstreamRepo>,
    State(settings): State<InstanceSettingsRepo>,
    Path(connection_id): Path<String>,
) -> Result<StatusCode, (StatusCode, Json<Value>)> {
    require_board(&actor)?;
    assert_cloud_sync_enabled(&settings)?;
    if repo.0.delete(&connection_id) {
        Ok(StatusCode::NO_CONTENT)
    } else {
        Err((
            StatusCode::NOT_FOUND,
            Json(json!({ "error": "Cloud upstream not found" })),
        ))
    }
}
