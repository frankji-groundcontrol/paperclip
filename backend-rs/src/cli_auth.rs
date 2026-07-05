use std::sync::Arc;

use serde_json::{json, Map, Value};

use crate::supabase::{
    broker::AuthBroker,
    data::{Auth, DataGateway, DisabledDataGateway},
};

#[derive(Clone)]
pub struct CliAuthService {
    data: Arc<dyn DataGateway>,
    auth: AuthBroker,
}

impl Default for CliAuthService {
    fn default() -> Self {
        Self::new(Arc::new(DisabledDataGateway), AuthBroker::default())
    }
}

impl CliAuthService {
    pub fn new(data: Arc<dyn DataGateway>, auth: AuthBroker) -> Self {
        Self { data, auth }
    }

    pub async fn start(&self, login: StartDeviceLogin) -> anyhow::Result<()> {
        self.data
            .rpc(
                "cli_start_device_login",
                json!({
                    "p_secret_hash": login.secret_hash,
                    "p_user_code_hash": login.user_code_hash,
                    "p_pending_key_prefix": login.pending_key_prefix,
                    "p_pending_key_hash": login.pending_key_hash,
                    "p_pending_key_name": login.pending_key_name,
                    "p_device_name": login.device_name,
                    "p_requested_access": login.requested_access,
                    "p_team_id": login.team_id,
                }),
                Auth::Anon,
            )
            .await?;
        Ok(())
    }

    pub async fn poll(&self, secret_hash: &str) -> anyhow::Result<Value> {
        let row = self
            .data
            .rpc(
                "cli_poll_device_login",
                json!({ "p_secret_hash": secret_hash }),
                Auth::Anon,
            )
            .await?;
        Ok(poll_row_json(row))
    }

    pub async fn approve(&self, bearer: &str, user_code_hash: &str) -> anyhow::Result<()> {
        let jwt = self
            .auth
            .session_access_token(bearer)
            .await?
            .ok_or_else(|| anyhow::anyhow!("session required"))?;
        self.data
            .rpc(
                "cli_approve_device_login",
                json!({ "p_user_code_hash": user_code_hash }),
                Auth::Bearer(jwt),
            )
            .await?;
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct StartDeviceLogin {
    pub secret_hash: String,
    pub user_code_hash: String,
    pub pending_key_prefix: String,
    pub pending_key_hash: String,
    pub pending_key_name: String,
    pub device_name: Option<String>,
    pub requested_access: Option<String>,
    pub team_id: Option<String>,
}

fn poll_row_json(row: Value) -> Value {
    let row = row
        .as_array()
        .and_then(|rows| rows.first())
        .cloned()
        .unwrap_or(row);
    let mut out = Map::new();
    if let Some(status) = row.get("status").cloned() {
        out.insert("status".to_string(), status);
    }
    if let Some(prefix) = row.get("prefix").cloned() {
        out.insert("prefix".to_string(), prefix);
    }
    if let Some(team_id) = row.get("team_id").cloned() {
        out.insert("teamId".to_string(), team_id);
    }
    if let Some(user_id) = row.get("user_id").cloned() {
        out.insert("userId".to_string(), user_id);
    }
    Value::Object(out)
}
