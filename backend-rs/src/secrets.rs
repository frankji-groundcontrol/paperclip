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

/// A secret *reference* — the only shape the API ever returns. The plaintext
/// value is never serialized (ports the invariant in
/// server/src/services/secrets.ts that secret values do not leave the server).
#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SecretRef {
    pub id: String,
    pub company_id: String,
    pub name: String,
    pub provider: String,
}

/// Ports the create-secret payload: `name` + `value` required, `provider`
/// defaulting to "local".
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateSecret {
    pub name: String,
    #[serde(default = "default_provider")]
    pub provider: String,
    pub value: String,
}

fn default_provider() -> String {
    "local".to_string()
}

/// Internal record. The `value` is held for the runtime but is never part of any
/// serialized response — hence intentionally not read back out here.
#[derive(Clone)]
struct StoredSecret {
    reference: SecretRef,
    #[allow(dead_code)]
    value: String,
}

pub trait SecretRepository {
    fn list_by_company(&self, company_id: &str) -> Vec<SecretRef>;
    fn create(&self, company_id: &str, input: CreateSecret) -> SecretRef;
    fn delete(&self, company_id: &str, secret_id: &str) -> bool;
}

#[derive(Clone, Default)]
pub struct SecretStore {
    inner: Arc<Mutex<Vec<StoredSecret>>>,
}

impl SecretStore {
    pub fn create(&self, company_id: &str, input: CreateSecret) -> SecretRef {
        let reference = SecretRef {
            id: Uuid::new_v4().to_string(),
            company_id: company_id.to_string(),
            name: input.name,
            provider: input.provider,
        };
        self.inner.lock().unwrap().push(StoredSecret {
            reference: reference.clone(),
            value: input.value,
        });
        reference
    }

    pub fn list_by_company(&self, company_id: &str) -> Vec<SecretRef> {
        self.inner
            .lock()
            .unwrap()
            .iter()
            .filter(|secret| secret.reference.company_id == company_id)
            .map(|secret| secret.reference.clone())
            .collect()
    }

    pub fn delete(&self, company_id: &str, secret_id: &str) -> bool {
        let mut guard = self.inner.lock().unwrap();
        let before = guard.len();
        guard.retain(|secret| {
            !(secret.reference.company_id == company_id && secret.reference.id == secret_id)
        });
        guard.len() != before
    }
}

impl SecretRepository for SecretStore {
    fn list_by_company(&self, company_id: &str) -> Vec<SecretRef> {
        SecretStore::list_by_company(self, company_id)
    }
    fn create(&self, company_id: &str, input: CreateSecret) -> SecretRef {
        SecretStore::create(self, company_id, input)
    }
    fn delete(&self, company_id: &str, secret_id: &str) -> bool {
        SecretStore::delete(self, company_id, secret_id)
    }
}

/// Cloneable handle to whichever [`SecretRepository`] backs the running app.
#[derive(Clone)]
pub struct SecretRepo(pub Arc<dyn SecretRepository + Send + Sync>);

impl Default for SecretRepo {
    fn default() -> Self {
        SecretRepo(Arc::new(SecretStore::default()))
    }
}

pub async fn list_secrets(
    actor: Actor,
    State(repo): State<SecretRepo>,
    Path(company_id): Path<String>,
) -> Result<Json<Vec<SecretRef>>, (StatusCode, Json<Value>)> {
    authorize_company_access(&actor, &company_id)?;
    Ok(Json(repo.0.list_by_company(&company_id)))
}

pub async fn create_secret(
    actor: Actor,
    State(repo): State<SecretRepo>,
    Path(company_id): Path<String>,
    Json(input): Json<CreateSecret>,
) -> Result<(StatusCode, Json<SecretRef>), (StatusCode, Json<Value>)> {
    authorize_company_access(&actor, &company_id)?;
    Ok((StatusCode::CREATED, Json(repo.0.create(&company_id, input))))
}

fn secret_not_found() -> (StatusCode, Json<Value>) {
    (
        StatusCode::NOT_FOUND,
        Json(json!({ "error": "Secret not found" })),
    )
}

pub async fn delete_secret(
    actor: Actor,
    State(repo): State<SecretRepo>,
    Path((company_id, secret_id)): Path<(String, String)>,
) -> Result<StatusCode, (StatusCode, Json<Value>)> {
    authorize_company_access(&actor, &company_id)?;
    if repo.0.delete(&company_id, &secret_id) {
        Ok(StatusCode::NO_CONTENT)
    } else {
        Err(secret_not_found())
    }
}
