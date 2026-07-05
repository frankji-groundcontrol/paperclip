use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    time::{Duration, SystemTime},
};

use async_trait::async_trait;
use axum::{
    body::{to_bytes, Body},
    http::{header, Request, StatusCode},
};
use paperclip_backend::{
    app_with,
    cli_auth::CliAuthService,
    supabase::{
        broker::{AuthBroker, InMemorySessionStore, SessionStore, StoredSession},
        data::{Auth, DataGateway},
        gateway::{GoTrueSession, ResolvedKey, SignUpOutcome, SupabaseGateway},
    },
    Repositories,
};
use serde_json::{json, Value};
use tower::ServiceExt;

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

#[derive(Clone, Default)]
struct FakeGateway {
    keys: Arc<Mutex<HashMap<String, ResolvedKey>>>,
}

impl FakeGateway {
    fn with_api_key(prefix: &str) -> Self {
        let gateway = Self::default();
        gateway.keys.lock().unwrap().insert(
            prefix.to_string(),
            ResolvedKey {
                api_key_id: "key-1".to_string(),
                team_id: "team-1".to_string(),
                created_by: Some("user-1".to_string()),
                subject_type: "agent".to_string(),
                agent_id: Some("agent-1".to_string()),
                scopes: json!([]),
                scope_config: json!({}),
            },
        );
        gateway
    }
}

#[async_trait]
impl SupabaseGateway for FakeGateway {
    async fn sign_up(&self, _email: &str, _password: &str) -> anyhow::Result<SignUpOutcome> {
        anyhow::bail!("unused")
    }

    async fn sign_in_password(
        &self,
        _email: &str,
        _password: &str,
    ) -> anyhow::Result<GoTrueSession> {
        anyhow::bail!("unused")
    }

    async fn refresh(&self, _refresh_token: &str) -> anyhow::Result<GoTrueSession> {
        anyhow::bail!("unused")
    }

    async fn sign_out(&self, _access_token: &str) -> anyhow::Result<()> {
        anyhow::bail!("unused")
    }

    async fn whoami(&self, _access_token: &str) -> anyhow::Result<Value> {
        Ok(json!({ "user": { "id": "user-1" }, "teams": [] }))
    }

    async fn resolve_api_key(
        &self,
        prefix: &str,
        _key_hash: &str,
    ) -> anyhow::Result<Option<ResolvedKey>> {
        Ok(self.keys.lock().unwrap().get(prefix).cloned())
    }
}

#[tokio::test]
async fn cli_start_and_poll_routes_are_public_and_sanitize_poll_row() {
    let (app, data) = route_app();

    let (start_status, start_body) = send(
        &app,
        "POST",
        "/api/cli/start",
        None,
        Some(json!({
            "secretHash": "secret-hash",
            "userCodeHash": "code-hash",
            "pendingKeyPrefix": "pc_12345678",
            "pendingKeyHash": "key-hash",
            "pendingKeyName": "Frank laptop",
            "deviceName": "frank-laptop",
            "requestedAccess": "team",
            "teamId": "team-1"
        })),
    )
    .await;
    let (poll_status, poll_body) = send(
        &app,
        "POST",
        "/api/cli/poll",
        None,
        Some(json!({ "secretHash": "secret-hash" })),
    )
    .await;

    assert_eq!(start_status, StatusCode::OK);
    assert_eq!(start_body, json!({ "ok": true }));
    assert_eq!(poll_status, StatusCode::OK);
    assert_eq!(
        poll_body,
        json!({
            "status": "approved",
            "prefix": "pc_12345678",
            "teamId": "team-1",
            "userId": "user-1"
        })
    );
    assert!(!poll_body.to_string().contains("key_hash"));
    assert_eq!(
        data.calls()
            .iter()
            .map(|call| call.name.as_str())
            .collect::<Vec<_>>(),
        vec!["cli_start_device_login", "cli_poll_device_login"]
    );
}

#[tokio::test]
async fn cli_approve_route_requires_session_bearer() {
    let (app, data) = route_app();

    let (api_key_status, api_key_body) = send(
        &app,
        "POST",
        "/api/cli/approve",
        Some("paperclip_pc_routes_testsecret"),
        Some(json!({ "userCodeHash": "code-hash" })),
    )
    .await;
    let (session_status, session_body) = send(
        &app,
        "POST",
        "/api/cli/approve",
        Some("pcs_test"),
        Some(json!({ "userCodeHash": "code-hash" })),
    )
    .await;

    assert_eq!(api_key_status, StatusCode::UNAUTHORIZED);
    assert_eq!(api_key_body, json!({ "error": "unauthorized" }));
    assert_eq!(session_status, StatusCode::OK);
    assert_eq!(session_body, json!({ "ok": true }));

    let calls = data.calls();
    assert_eq!(calls.len(), 1);
    assert_eq!(calls[0].name, "cli_approve_device_login");
    assert_eq!(calls[0].auth, Auth::Bearer("jwt-abc".to_string()));
}

fn route_app() -> (axum::Router, FakeData) {
    let data = FakeData::default();
    let auth = broker_with_session_and_key();
    let app = app_with(Repositories {
        cli_auth: CliAuthService::new(Arc::new(data.clone()), auth.clone()),
        supabase_auth: auth,
        ..Default::default()
    });
    (app, data)
}

fn broker_with_session_and_key() -> AuthBroker {
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
        Arc::new(FakeGateway::with_api_key("pc_routes")),
        Arc::new(store),
    )
}

async fn send(
    app: &axum::Router,
    method: &str,
    uri: &str,
    bearer: Option<&str>,
    body: Option<Value>,
) -> (StatusCode, Value) {
    let mut builder = Request::builder().method(method).uri(uri);
    if let Some(token) = bearer {
        builder = builder.header(header::AUTHORIZATION, format!("Bearer {token}"));
    }
    let request = match body {
        Some(body) => builder
            .header(header::CONTENT_TYPE, "application/json")
            .body(Body::from(body.to_string()))
            .unwrap(),
        None => builder.body(Body::empty()).unwrap(),
    };
    let response = app.clone().oneshot(request).await.unwrap();
    let status = response.status();
    let bytes = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let body = if bytes.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&bytes).unwrap()
    };
    (status, body)
}
