use async_trait::async_trait;
use reqwest::header;
use serde_json::Value;

use super::gateway::HttpSupabaseGateway;

const PAPERCLIP_SCHEMA: &str = "paperclip";

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Auth {
    Anon,
    Bearer(String),
}

#[async_trait]
pub trait DataGateway: Send + Sync {
    async fn rpc(&self, name: &str, body: Value, auth: Auth) -> anyhow::Result<Value>;
}

#[async_trait]
impl DataGateway for HttpSupabaseGateway {
    async fn rpc(&self, name: &str, body: Value, auth: Auth) -> anyhow::Result<Value> {
        let bearer = match auth {
            Auth::Anon => self.anon_key().to_string(),
            Auth::Bearer(token) => token,
        };
        let response = self
            .client()
            .post(self.rpc_url(name))
            .header("apikey", self.anon_key())
            .header(header::AUTHORIZATION, format!("Bearer {bearer}"))
            .header("Content-Profile", PAPERCLIP_SCHEMA)
            .header("Accept-Profile", PAPERCLIP_SCHEMA)
            .json(&body)
            .send()
            .await?;
        let status = response.status();
        if !status.is_success() {
            anyhow::bail!("Supabase RPC failed with {status}");
        }
        Ok(response.json().await?)
    }
}

#[derive(Debug, Clone, Default)]
pub struct DisabledDataGateway;

#[async_trait]
impl DataGateway for DisabledDataGateway {
    async fn rpc(&self, _name: &str, _body: Value, _auth: Auth) -> anyhow::Result<Value> {
        anyhow::bail!("Supabase data is not configured")
    }
}

