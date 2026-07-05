use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    time::{Duration, SystemTime},
};

use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};
use rand::{rngs::OsRng, RngCore};
use serde_json::Value;

use super::gateway::{DisabledSupabaseGateway, GoTrueSession, SignUpOutcome, SupabaseGateway};
use super::keys::parse_api_key;

#[derive(Debug, Clone, PartialEq)]
pub enum Principal {
    User {
        user_id: String,
        whoami: Value,
    },
    ApiKey {
        team_id: String,
        subject_type: String,
        agent_id: Option<String>,
        scopes: Value,
        scope_config: Value,
    },
}

#[derive(Clone)]
pub struct StoredSession {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_at: SystemTime,
    pub user_id: String,
}

// Redact the bearer tokens from Debug so a stray log line can never leak them.
impl std::fmt::Debug for StoredSession {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("StoredSession")
            .field("access_token", &"<redacted>")
            .field("refresh_token", &"<redacted>")
            .field("expires_at", &self.expires_at)
            .field("user_id", &self.user_id)
            .finish()
    }
}

pub trait SessionStore: Send + Sync {
    fn put(&self, token: String, session: StoredSession);
    fn get(&self, token: &str) -> Option<StoredSession>;
    fn delete(&self, token: &str);
}

#[derive(Clone, Default)]
pub struct InMemorySessionStore {
    inner: Arc<Mutex<HashMap<String, StoredSession>>>,
}

impl SessionStore for InMemorySessionStore {
    fn put(&self, token: String, session: StoredSession) {
        self.inner.lock().unwrap().insert(token, session);
    }

    fn get(&self, token: &str) -> Option<StoredSession> {
        self.inner.lock().unwrap().get(token).cloned()
    }

    fn delete(&self, token: &str) {
        self.inner.lock().unwrap().remove(token);
    }
}

#[derive(Clone)]
pub struct AuthBroker {
    gateway: Arc<dyn SupabaseGateway>,
    sessions: Arc<dyn SessionStore>,
}

impl Default for AuthBroker {
    fn default() -> Self {
        Self::new(
            Arc::new(DisabledSupabaseGateway),
            Arc::new(InMemorySessionStore::default()),
        )
    }
}

impl AuthBroker {
    pub fn new(gateway: Arc<dyn SupabaseGateway>, sessions: Arc<dyn SessionStore>) -> Self {
        Self { gateway, sessions }
    }

    pub async fn register(&self, email: &str, password: &str) -> anyhow::Result<SignUpOutcome> {
        self.gateway.sign_up(email, password).await
    }

    pub async fn login(&self, email: &str, password: &str) -> anyhow::Result<(String, Value)> {
        let session = self.gateway.sign_in_password(email, password).await?;
        let token = mint_session_token();
        self.sessions.put(token.clone(), stored_session(&session));
        let whoami = self.gateway.whoami(&session.access_token).await?;
        Ok((token, whoami))
    }

    pub async fn logout(&self, session_token: &str) -> anyhow::Result<()> {
        if !session_token.starts_with("pcs_") {
            anyhow::bail!("unsupported session token");
        }
        let Some(session) = self.sessions.get(session_token) else {
            anyhow::bail!("unknown session");
        };
        // The local pcs_ session is the authoritative security boundary: revoke
        // it FIRST, then treat the upstream GoTrue sign_out as best-effort so a
        // failed sign_out cannot leave the caller logged in locally.
        self.sessions.delete(session_token);
        let _ = self.gateway.sign_out(&session.access_token).await;
        Ok(())
    }

    pub async fn authenticate(&self, bearer: &str) -> anyhow::Result<Principal> {
        if bearer.starts_with("pcs_") {
            let Some(mut session) = self.sessions.get(bearer) else {
                anyhow::bail!("unknown session");
            };
            if session.expires_at <= SystemTime::now() + Duration::from_secs(60) {
                match self.gateway.refresh(&session.refresh_token).await {
                    Ok(refreshed) => {
                        session = stored_session(&refreshed);
                        self.sessions.put(bearer.to_string(), session.clone());
                    }
                    Err(err) => {
                        // Evict the dead session so it cannot linger unusable.
                        self.sessions.delete(bearer);
                        return Err(err);
                    }
                }
            }
            let whoami = self.gateway.whoami(&session.access_token).await?;
            return Ok(Principal::User {
                user_id: session.user_id,
                whoami,
            });
        }

        if let Some(parts) = parse_api_key(bearer) {
            let Some(resolved) = self
                .gateway
                .resolve_api_key(&parts.prefix, &parts.key_hash)
                .await?
            else {
                anyhow::bail!("unknown api key");
            };
            return Ok(Principal::ApiKey {
                team_id: resolved.team_id,
                subject_type: resolved.subject_type,
                agent_id: resolved.agent_id,
                scopes: resolved.scopes,
                scope_config: resolved.scope_config,
            });
        }

        anyhow::bail!("unsupported bearer token")
    }

    /// For a `pcs_` session bearer, return a currently-valid GoTrue access token
    /// (refreshing if near expiry), or `Ok(None)` for a non-session bearer. Used
    /// server-side only for the session data path; the JWT never reaches a client.
    pub async fn session_access_token(&self, bearer: &str) -> anyhow::Result<Option<String>> {
        if !bearer.starts_with("pcs_") {
            return Ok(None);
        }
        let Some(mut session) = self.sessions.get(bearer) else {
            anyhow::bail!("unknown session");
        };
        if session.expires_at <= SystemTime::now() + Duration::from_secs(60) {
            match self.gateway.refresh(&session.refresh_token).await {
                Ok(refreshed) => {
                    session = stored_session(&refreshed);
                    self.sessions.put(bearer.to_string(), session.clone());
                }
                Err(err) => {
                    self.sessions.delete(bearer);
                    return Err(err);
                }
            }
        }
        Ok(Some(session.access_token))
    }
}

fn stored_session(session: &GoTrueSession) -> StoredSession {
    let expires_in = u64::try_from(session.expires_in.max(0)).unwrap_or(0);
    // Guard against an absurd upstream expires_in overflowing SystemTime (panic).
    let expires_at = SystemTime::now()
        .checked_add(Duration::from_secs(expires_in))
        .unwrap_or_else(|| SystemTime::now() + Duration::from_secs(3600));
    StoredSession {
        access_token: session.access_token.clone(),
        refresh_token: session.refresh_token.clone(),
        expires_at,
        user_id: session.user_id.clone(),
    }
}

pub fn mint_session_token() -> String {
    let mut bytes = [0_u8; 32];
    OsRng.fill_bytes(&mut bytes);
    format!("pcs_{}", URL_SAFE_NO_PAD.encode(bytes))
}
