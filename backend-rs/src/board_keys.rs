use std::sync::{Arc, Mutex};

use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use uuid::Uuid;

use crate::auth::{require_board_user_authenticated, Actor};

/// Internal board-api-key record. The `token` is the plaintext secret; it is
/// revealed once on create and never listed (ports the no-leak behaviour of
/// server/src/routes/access.ts board-api-keys).
#[derive(Clone)]
struct BoardApiKeyRecord {
    id: String,
    user_id: String,
    name: String,
    token: String,
    expires_at: Option<String>,
}

/// The create response — includes the one-time plaintext `token`.
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreatedBoardApiKey {
    pub id: String,
    pub name: String,
    pub token: String,
    pub expires_at: Option<String>,
}

/// A listed key — reference only, never the token.
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BoardApiKeyRef {
    pub id: String,
    pub name: String,
    pub expires_at: Option<String>,
}

/// Ports the create-board-api-key payload: `name` defaults to "paperclipai cli";
/// `expiresAt` nullable.
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateBoardApiKey {
    #[serde(default = "default_name")]
    pub name: String,
    #[serde(default)]
    pub expires_at: Option<String>,
}

fn default_name() -> String {
    "paperclipai cli".to_string()
}

/// In-memory, per-user board-api-key store.
#[derive(Clone, Default)]
pub struct BoardKeyStore {
    inner: Arc<Mutex<Vec<BoardApiKeyRecord>>>,
}

impl BoardKeyStore {
    pub fn create(&self, user_id: &str, input: CreateBoardApiKey) -> CreatedBoardApiKey {
        let record = BoardApiKeyRecord {
            id: Uuid::new_v4().to_string(),
            user_id: user_id.to_string(),
            name: input.name,
            token: Uuid::new_v4().to_string(),
            expires_at: input.expires_at,
        };
        self.inner.lock().unwrap().push(record.clone());
        CreatedBoardApiKey {
            id: record.id,
            name: record.name,
            token: record.token,
            expires_at: record.expires_at,
        }
    }

    pub fn list(&self, user_id: &str) -> Vec<BoardApiKeyRef> {
        self.inner
            .lock()
            .unwrap()
            .iter()
            .filter(|k| k.user_id == user_id)
            .map(|k| BoardApiKeyRef {
                id: k.id.clone(),
                name: k.name.clone(),
                expires_at: k.expires_at.clone(),
            })
            .collect()
    }

    pub fn delete(&self, user_id: &str, key_id: &str) -> bool {
        let mut guard = self.inner.lock().unwrap();
        let before = guard.len();
        guard.retain(|k| !(k.user_id == user_id && k.id == key_id));
        guard.len() != before
    }
}

pub trait BoardKeyRepository {
    fn create(&self, user_id: &str, input: CreateBoardApiKey) -> CreatedBoardApiKey;
    fn list(&self, user_id: &str) -> Vec<BoardApiKeyRef>;
    fn delete(&self, user_id: &str, key_id: &str) -> bool;
}

impl BoardKeyRepository for BoardKeyStore {
    fn create(&self, user_id: &str, input: CreateBoardApiKey) -> CreatedBoardApiKey {
        BoardKeyStore::create(self, user_id, input)
    }
    fn list(&self, user_id: &str) -> Vec<BoardApiKeyRef> {
        BoardKeyStore::list(self, user_id)
    }
    fn delete(&self, user_id: &str, key_id: &str) -> bool {
        BoardKeyStore::delete(self, user_id, key_id)
    }
}

/// Cloneable handle to whichever [`BoardKeyRepository`] backs the running app.
#[derive(Clone)]
pub struct BoardKeyRepo(pub Arc<dyn BoardKeyRepository + Send + Sync>);

impl Default for BoardKeyRepo {
    fn default() -> Self {
        BoardKeyRepo(Arc::new(BoardKeyStore::default()))
    }
}

pub async fn list_board_api_keys(
    actor: Actor,
    State(repo): State<BoardKeyRepo>,
) -> Result<Json<Vec<BoardApiKeyRef>>, (StatusCode, Json<Value>)> {
    let user_id = require_board_user_authenticated(&actor)?;
    Ok(Json(repo.0.list(&user_id)))
}

pub async fn create_board_api_key(
    actor: Actor,
    State(repo): State<BoardKeyRepo>,
    Json(input): Json<CreateBoardApiKey>,
) -> Result<(StatusCode, Json<CreatedBoardApiKey>), (StatusCode, Json<Value>)> {
    let user_id = require_board_user_authenticated(&actor)?;
    Ok((StatusCode::CREATED, Json(repo.0.create(&user_id, input))))
}

pub async fn delete_board_api_key(
    actor: Actor,
    State(repo): State<BoardKeyRepo>,
    Path(key_id): Path<String>,
) -> Result<StatusCode, (StatusCode, Json<Value>)> {
    let user_id = require_board_user_authenticated(&actor)?;
    if repo.0.delete(&user_id, &key_id) {
        Ok(StatusCode::NO_CONTENT)
    } else {
        Err((
            StatusCode::NOT_FOUND,
            Json(json!({ "error": "Board API key not found" })),
        ))
    }
}
