use async_trait::async_trait;
use reqwest::{header, Client};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

const PAPERCLIP_SCHEMA: &str = "paperclip";

#[derive(Clone, PartialEq, Eq)]
pub struct GoTrueSession {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_in: i64,
    pub user_id: String,
}

// Redact the bearer tokens from Debug so a stray log line can never leak them.
impl std::fmt::Debug for GoTrueSession {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GoTrueSession")
            .field("access_token", &"<redacted>")
            .field("refresh_token", &"<redacted>")
            .field("expires_in", &self.expires_in)
            .field("user_id", &self.user_id)
            .finish()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SignUpOutcome {
    pub user_id: Option<String>,
    pub needs_confirmation: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResolvedKey {
    pub api_key_id: String,
    pub team_id: String,
    pub created_by: Option<String>,
    pub subject_type: String,
    pub agent_id: Option<String>,
    pub scopes: Value,
    pub scope_config: Value,
}

#[async_trait]
pub trait SupabaseGateway: Send + Sync {
    async fn sign_up(&self, email: &str, password: &str) -> anyhow::Result<SignUpOutcome>;
    async fn sign_in_password(&self, email: &str, password: &str) -> anyhow::Result<GoTrueSession>;
    async fn refresh(&self, refresh_token: &str) -> anyhow::Result<GoTrueSession>;
    async fn sign_out(&self, access_token: &str) -> anyhow::Result<()>;
    async fn whoami(&self, access_token: &str) -> anyhow::Result<Value>;
    async fn resolve_api_key(
        &self,
        prefix: &str,
        key_hash: &str,
    ) -> anyhow::Result<Option<ResolvedKey>>;
}

#[derive(Debug, Clone)]
pub struct SupabaseConfig {
    pub url: String,
    pub anon_key: String,
}

impl SupabaseConfig {
    pub fn from_env() -> anyhow::Result<Self> {
        let url = std::env::var("SUPABASE_URL")?;
        let anon_key = std::env::var("SUPABASE_ANON_KEY")?;
        Ok(Self { url, anon_key })
    }
}

#[derive(Clone)]
pub struct HttpSupabaseGateway {
    url: String,
    anon_key: String,
    client: Client,
}

impl HttpSupabaseGateway {
    pub fn new(url: String, anon_key: String) -> Self {
        Self {
            url: url.trim_end_matches('/').to_string(),
            anon_key,
            client: Client::new(),
        }
    }

    fn auth_url(&self, path: &str) -> String {
        format!("{}/auth/v1{}", self.url, path)
    }

    fn rpc_url(&self, name: &str) -> String {
        format!("{}/rest/v1/rpc/{}", self.url, name)
    }

    async fn post_json(&self, url: String, body: Value) -> anyhow::Result<Value> {
        let response = self
            .client
            .post(url)
            .header("apikey", &self.anon_key)
            .json(&body)
            .send()
            .await?;
        json_response(response).await
    }
}

#[async_trait]
impl SupabaseGateway for HttpSupabaseGateway {
    async fn sign_up(&self, email: &str, password: &str) -> anyhow::Result<SignUpOutcome> {
        let value = self
            .post_json(
                self.auth_url("/signup"),
                json!({ "email": email, "password": password }),
            )
            .await?;
        let user_id = value
            .pointer("/user/id")
            .or_else(|| value.get("id"))
            .and_then(Value::as_str)
            .map(ToString::to_string);
        let needs_confirmation = value.get("session").is_none_or(|session| session.is_null())
            || value.get("access_token").is_none();
        Ok(SignUpOutcome {
            user_id,
            needs_confirmation,
        })
    }

    async fn sign_in_password(&self, email: &str, password: &str) -> anyhow::Result<GoTrueSession> {
        let value = self
            .post_json(
                self.auth_url("/token?grant_type=password"),
                json!({ "email": email, "password": password }),
            )
            .await?;
        parse_gotrue_session(value)
    }

    async fn refresh(&self, refresh_token: &str) -> anyhow::Result<GoTrueSession> {
        let value = self
            .post_json(
                self.auth_url("/token?grant_type=refresh_token"),
                json!({ "refresh_token": refresh_token }),
            )
            .await?;
        parse_gotrue_session(value)
    }

    async fn sign_out(&self, access_token: &str) -> anyhow::Result<()> {
        let response = self
            .client
            .post(self.auth_url("/logout"))
            .header("apikey", &self.anon_key)
            .header(header::AUTHORIZATION, format!("Bearer {access_token}"))
            .json(&json!({}))
            .send()
            .await?;
        if !response.status().is_success() {
            anyhow::bail!("Supabase sign_out failed with {}", response.status());
        }
        Ok(())
    }

    async fn whoami(&self, access_token: &str) -> anyhow::Result<Value> {
        let response = self
            .client
            .post(self.rpc_url("whoami"))
            .header("apikey", &self.anon_key)
            .header(header::AUTHORIZATION, format!("Bearer {access_token}"))
            .header("Content-Profile", PAPERCLIP_SCHEMA)
            .header("Accept-Profile", PAPERCLIP_SCHEMA)
            .json(&json!({}))
            .send()
            .await?;
        json_response(response).await
    }

    async fn resolve_api_key(
        &self,
        prefix: &str,
        key_hash: &str,
    ) -> anyhow::Result<Option<ResolvedKey>> {
        let response = self
            .client
            .post(self.rpc_url("resolve_api_key"))
            .header("apikey", &self.anon_key)
            .header(header::AUTHORIZATION, format!("Bearer {}", self.anon_key))
            .header("Content-Profile", PAPERCLIP_SCHEMA)
            .header("Accept-Profile", PAPERCLIP_SCHEMA)
            .json(&json!({ "p_prefix": prefix, "p_key_hash": key_hash }))
            .send()
            .await?;
        let value = json_response(response).await?;
        if let Some(rows) = value.as_array() {
            return rows
                .first()
                .cloned()
                .map(serde_json::from_value)
                .transpose()
                .map_err(Into::into);
        }
        if value.is_null() {
            return Ok(None);
        }
        serde_json::from_value(value).map(Some).map_err(Into::into)
    }
}

async fn json_response(response: reqwest::Response) -> anyhow::Result<Value> {
    let status = response.status();
    if !status.is_success() {
        anyhow::bail!("Supabase request failed with {status}");
    }
    Ok(response.json().await?)
}

fn parse_gotrue_session(value: Value) -> anyhow::Result<GoTrueSession> {
    let access_token = required_string(&value, "access_token")?;
    let refresh_token = required_string(&value, "refresh_token")?;
    let expires_in = value
        .get("expires_in")
        .and_then(Value::as_i64)
        .ok_or_else(|| anyhow::anyhow!("missing expires_in"))?;
    let user_id = value
        .pointer("/user/id")
        .or_else(|| value.get("user_id"))
        .and_then(Value::as_str)
        .ok_or_else(|| anyhow::anyhow!("missing user id"))?
        .to_string();

    Ok(GoTrueSession {
        access_token,
        refresh_token,
        expires_in,
        user_id,
    })
}

fn required_string(value: &Value, field: &str) -> anyhow::Result<String> {
    value
        .get(field)
        .and_then(Value::as_str)
        .map(ToString::to_string)
        .ok_or_else(|| anyhow::anyhow!("missing {field}"))
}

#[derive(Debug, Clone, Default)]
pub struct DisabledSupabaseGateway;

#[async_trait]
impl SupabaseGateway for DisabledSupabaseGateway {
    async fn sign_up(&self, _email: &str, _password: &str) -> anyhow::Result<SignUpOutcome> {
        anyhow::bail!("Supabase auth is not configured")
    }

    async fn sign_in_password(
        &self,
        _email: &str,
        _password: &str,
    ) -> anyhow::Result<GoTrueSession> {
        anyhow::bail!("Supabase auth is not configured")
    }

    async fn refresh(&self, _refresh_token: &str) -> anyhow::Result<GoTrueSession> {
        anyhow::bail!("Supabase auth is not configured")
    }

    async fn sign_out(&self, _access_token: &str) -> anyhow::Result<()> {
        anyhow::bail!("Supabase auth is not configured")
    }

    async fn whoami(&self, _access_token: &str) -> anyhow::Result<Value> {
        anyhow::bail!("Supabase auth is not configured")
    }

    async fn resolve_api_key(
        &self,
        _prefix: &str,
        _key_hash: &str,
    ) -> anyhow::Result<Option<ResolvedKey>> {
        anyhow::bail!("Supabase auth is not configured")
    }
}
