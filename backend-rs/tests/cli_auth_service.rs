use std::{
    sync::{Arc, Mutex},
    time::{Duration, SystemTime},
};

use async_trait::async_trait;
use paperclip_backend::{
    cli_auth::{CliAuthService, StartDeviceLogin},
    supabase::{
        broker::{AuthBroker, InMemorySessionStore, SessionStore, StoredSession},
        data::{Auth, DataGateway},
    },
};
use serde_json::{json, Value};

#[derive(Debug, Clone, PartialEq)]
struct RpcCall {
    name: String,
    body: Value,
    auth: Auth,
}

#[derive(Clone, Default)]
struct FakeData {
    calls: Arc<Mutex<Vec<RpcCall>>>,
}

impl FakeData {
    fn calls(&self) -> Vec<RpcCall> {
        self.calls.lock().unwrap().clone()
    }
}

#[derive(Clone, Default)]
struct TablePollData;

#[async_trait]
impl DataGateway for TablePollData {
    async fn rpc(&self, name: &str, _body: Value, _auth: Auth) -> anyhow::Result<Value> {
        assert_eq!(name, "cli_poll_device_login");
        Ok(json!([{
            "status": "approved",
            "prefix": "pc_abcdef12",
            "team_id": "team-2",
            "user_id": "user-2",
            "secret_hash": "must-not-leak"
        }]))
    }

    async fn get(&self, _path: &str, _auth: Auth) -> anyhow::Result<Value> {
        anyhow::bail!("unexpected get")
    }
}

#[async_trait]
impl DataGateway for FakeData {
    async fn rpc(&self, name: &str, body: Value, auth: Auth) -> anyhow::Result<Value> {
        self.calls.lock().unwrap().push(RpcCall {
            name: name.to_string(),
            body,
            auth,
        });
        Ok(match name {
            "cli_start_device_login" => json!(true),
            "cli_poll_device_login" => json!({
                "status": "approved",
                "prefix": "pc_12345678",
                "team_id": "team-1",
                "user_id": "user-1",
                "secret_hash": "must-not-leak",
                "pending_key_hash": "must-not-leak"
            }),
            "cli_approve_device_login" => json!(true),
            other => anyhow::bail!("unexpected rpc {other}"),
        })
    }

    async fn get(&self, _path: &str, _auth: Auth) -> anyhow::Result<Value> {
        anyhow::bail!("unexpected get")
    }
}

#[tokio::test]
async fn start_calls_device_login_rpc_with_anon_auth_and_exact_params() {
    let data = FakeData::default();
    let service = CliAuthService::new(Arc::new(data.clone()), AuthBroker::default());

    service
        .start(StartDeviceLogin {
            secret_hash: "secret-hash".to_string(),
            user_code_hash: "code-hash".to_string(),
            pending_key_prefix: "pc_12345678".to_string(),
            pending_key_hash: "key-hash".to_string(),
            pending_key_name: "Frank laptop".to_string(),
            device_name: Some("frank-laptop".to_string()),
            requested_access: Some("team".to_string()),
            team_id: Some("team-1".to_string()),
        })
        .await
        .unwrap();

    let calls = data.calls();
    assert_eq!(calls.len(), 1);
    assert_eq!(calls[0].name, "cli_start_device_login");
    assert_eq!(calls[0].auth, Auth::Anon);
    assert_eq!(
        sorted_keys(&calls[0].body),
        vec![
            "p_device_name",
            "p_pending_key_hash",
            "p_pending_key_name",
            "p_pending_key_prefix",
            "p_requested_access",
            "p_secret_hash",
            "p_team_id",
            "p_user_code_hash"
        ]
    );
    assert_eq!(calls[0].body["p_secret_hash"], "secret-hash");
    assert_eq!(calls[0].body["p_user_code_hash"], "code-hash");
    assert_eq!(calls[0].body["p_pending_key_prefix"], "pc_12345678");
    assert_eq!(calls[0].body["p_pending_key_hash"], "key-hash");
    assert_eq!(calls[0].body["p_pending_key_name"], "Frank laptop");
    assert_eq!(calls[0].body["p_device_name"], "frank-laptop");
    assert_eq!(calls[0].body["p_requested_access"], "team");
    assert_eq!(calls[0].body["p_team_id"], "team-1");
}

#[tokio::test]
async fn poll_calls_device_login_rpc_with_anon_auth_and_maps_row_to_camel_case() {
    let data = FakeData::default();
    let service = CliAuthService::new(Arc::new(data.clone()), AuthBroker::default());

    let row = service.poll("secret-hash").await.unwrap();

    assert_eq!(
        row,
        json!({
            "status": "approved",
            "prefix": "pc_12345678",
            "teamId": "team-1",
            "userId": "user-1"
        })
    );
    let calls = data.calls();
    assert_eq!(calls.len(), 1);
    assert_eq!(calls[0].name, "cli_poll_device_login");
    assert_eq!(calls[0].auth, Auth::Anon);
    assert_eq!(sorted_keys(&calls[0].body), vec!["p_secret_hash"]);
    assert_eq!(calls[0].body["p_secret_hash"], "secret-hash");
}

#[tokio::test]
async fn poll_maps_single_row_table_response_to_camel_case() {
    let service = CliAuthService::new(Arc::new(TablePollData), AuthBroker::default());

    let row = service.poll("secret-hash").await.unwrap();

    assert_eq!(
        row,
        json!({
            "status": "approved",
            "prefix": "pc_abcdef12",
            "teamId": "team-2",
            "userId": "user-2"
        })
    );
}

#[tokio::test]
async fn approve_resolves_session_jwt_and_calls_rpc_with_bearer_auth() {
    let data = FakeData::default();
    let service = CliAuthService::new(Arc::new(data.clone()), broker_with_session());

    service.approve("pcs_test", "code-hash").await.unwrap();

    let calls = data.calls();
    assert_eq!(calls.len(), 1);
    assert_eq!(calls[0].name, "cli_approve_device_login");
    assert_eq!(calls[0].auth, Auth::Bearer("jwt-abc".to_string()));
    assert_eq!(sorted_keys(&calls[0].body), vec!["p_user_code_hash"]);
    assert_eq!(calls[0].body["p_user_code_hash"], "code-hash");
}

#[tokio::test]
async fn approve_rejects_non_session_bearer_before_rpc() {
    let data = FakeData::default();
    let service = CliAuthService::new(Arc::new(data.clone()), AuthBroker::default());

    let err = service
        .approve("paperclip_pc_test_secret", "code-hash")
        .await
        .unwrap_err();

    assert!(err.to_string().contains("session required"));
    assert!(data.calls().is_empty());
}

fn sorted_keys(value: &Value) -> Vec<&str> {
    let mut keys = value
        .as_object()
        .expect("object")
        .keys()
        .map(String::as_str)
        .collect::<Vec<_>>();
    keys.sort_unstable();
    keys
}

fn broker_with_session() -> AuthBroker {
    let store = InMemorySessionStore::default();
    store.put(
        "pcs_test".to_string(),
        StoredSession {
            access_token: "jwt-abc".to_string(),
            refresh_token: "refresh-abc".to_string(),
            expires_at: SystemTime::now() + Duration::from_secs(3600),
            user_id: "user-1".to_string(),
        },
    );
    AuthBroker::new(
        Arc::new(paperclip_backend::supabase::gateway::DisabledSupabaseGateway),
        Arc::new(store),
    )
}
