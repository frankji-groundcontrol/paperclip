use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    time::SystemTime,
};

use async_trait::async_trait;
use axum::body::Body;
use axum::http::{header, Request, StatusCode};
use paperclip_backend::supabase::broker::{
    AuthBroker, InMemorySessionStore, SessionStore, StoredSession,
};
use paperclip_backend::supabase::gateway::{
    GoTrueSession, ResolvedKey, SignUpOutcome, SupabaseGateway,
};
use paperclip_backend::supabase::keys::{hash_full_key, parse_api_key};
use paperclip_backend::{app_with, Repositories};
use serde_json::{json, Value};
use tower::ServiceExt;

#[derive(Clone, Default)]
struct FakeGateway {
    api_keys: Arc<Mutex<HashMap<String, ResolvedKey>>>,
    calls: Arc<Mutex<Vec<String>>>,
    reject_password: bool,
    reject_signout: bool,
    reject_refresh: bool,
}

impl FakeGateway {
    fn with_api_key(prefix: &str, key: ResolvedKey) -> Self {
        let gateway = Self::default();
        gateway
            .api_keys
            .lock()
            .unwrap()
            .insert(prefix.to_string(), key);
        gateway
    }

    fn rejecting_password() -> Self {
        Self {
            reject_password: true,
            ..Default::default()
        }
    }

    fn rejecting_signout() -> Self {
        Self {
            reject_signout: true,
            ..Default::default()
        }
    }

    fn rejecting_refresh() -> Self {
        Self {
            reject_refresh: true,
            ..Default::default()
        }
    }
}

#[async_trait]
impl SupabaseGateway for FakeGateway {
    async fn sign_up(&self, _email: &str, _password: &str) -> anyhow::Result<SignUpOutcome> {
        Ok(SignUpOutcome {
            user_id: Some("user-1".to_string()),
            needs_confirmation: true,
        })
    }

    async fn sign_in_password(
        &self,
        _email: &str,
        _password: &str,
    ) -> anyhow::Result<GoTrueSession> {
        if self.reject_password {
            anyhow::bail!("bad password");
        }
        Ok(GoTrueSession {
            access_token: "fake.jwt.tok".to_string(),
            refresh_token: "fake-refresh-token".to_string(),
            expires_in: 3600,
            user_id: "user-1".to_string(),
        })
    }

    async fn refresh(&self, _refresh_token: &str) -> anyhow::Result<GoTrueSession> {
        self.calls
            .lock()
            .unwrap()
            .push(format!("refresh:{_refresh_token}"));
        if self.reject_refresh {
            anyhow::bail!("refresh failed");
        }
        Ok(GoTrueSession {
            access_token: "fake.refreshed.jwt".to_string(),
            refresh_token: "fake-refreshed-token".to_string(),
            expires_in: 3600,
            user_id: "user-1".to_string(),
        })
    }

    async fn sign_out(&self, _access_token: &str) -> anyhow::Result<()> {
        self.calls
            .lock()
            .unwrap()
            .push(format!("sign_out:{_access_token}"));
        if self.reject_signout {
            anyhow::bail!("sign_out failed");
        }
        Ok(())
    }

    async fn whoami(&self, _access_token: &str) -> anyhow::Result<Value> {
        Ok(json!({
            "user": { "id": "user-1", "email": "x@y.z" },
            "teams": []
        }))
    }

    async fn resolve_api_key(
        &self,
        prefix: &str,
        _key_hash: &str,
    ) -> anyhow::Result<Option<ResolvedKey>> {
        Ok(self.api_keys.lock().unwrap().get(prefix).cloned())
    }
}

fn app() -> axum::Router {
    app_with_gateway(FakeGateway::default())
}

fn app_with_gateway(gateway: FakeGateway) -> axum::Router {
    app_with_gateway_and_store(gateway, InMemorySessionStore::default())
}

fn app_with_gateway_and_store(gateway: FakeGateway, store: InMemorySessionStore) -> axum::Router {
    app_with(Repositories {
        supabase_auth: AuthBroker::new(Arc::new(gateway), Arc::new(store)),
        ..Default::default()
    })
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
            .header("content-type", "application/json")
            .body(Body::from(body.to_string()))
            .unwrap(),
        None => builder.body(Body::empty()).unwrap(),
    };
    let response = app.clone().oneshot(request).await.unwrap();
    let status = response.status();
    let bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let json = if bytes.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&bytes).unwrap()
    };
    (status, json)
}

#[tokio::test]
async fn login_issues_opaque_session_token() {
    let app = app();
    let (status, body) = send(
        &app,
        "POST",
        "/api/auth/login",
        None,
        Some(json!({ "email": "x@y.z", "password": "correct-password" })),
    )
    .await;

    assert_eq!(status, StatusCode::OK);
    let token = body["session"].as_str().expect("session token");
    assert!(token.starts_with("pcs_"));
    assert_ne!(token, "fake.jwt.tok");
    assert_ne!(
        token.split('.').count(),
        3,
        "session token must not be a JWT"
    );
}

#[tokio::test]
async fn session_bearer_authenticates_as_user() {
    let app = app();
    let (_, login) = send(
        &app,
        "POST",
        "/api/auth/login",
        None,
        Some(json!({ "email": "x@y.z", "password": "correct-password" })),
    )
    .await;
    let token = login["session"].as_str().unwrap();

    let (status, body) = send(&app, "GET", "/api/auth/session", Some(token), None).await;

    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["kind"], "user");
    assert_eq!(body["user"]["email"], "x@y.z");
}

#[tokio::test]
async fn api_key_bearer_authenticates_as_team() {
    let app = app_with_gateway(FakeGateway::with_api_key(
        "pc_x",
        ResolvedKey {
            api_key_id: "key-1".to_string(),
            team_id: "team-1".to_string(),
            created_by: Some("user-1".to_string()),
            subject_type: "agent".to_string(),
            agent_id: Some("agent-1".to_string()),
            scopes: json!(["issues:read"]),
            scope_config: json!({ "mode": "test" }),
        },
    ));

    let (status, body) = send(
        &app,
        "GET",
        "/api/auth/session",
        Some("paperclip_pc_x_y"),
        None,
    )
    .await;

    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["kind"], "apiKey");
    assert_eq!(body["teamId"], "team-1");
    assert_eq!(body["subjectType"], "agent");
}

#[tokio::test]
async fn unknown_api_key_is_unauthorized() {
    let app = app();

    let (status, body) = send(
        &app,
        "GET",
        "/api/auth/session",
        Some("paperclip_pc_x_y"),
        None,
    )
    .await;

    assert_eq!(status, StatusCode::UNAUTHORIZED);
    assert_eq!(body, json!({ "error": "unauthorized" }));
}

#[tokio::test]
async fn logout_invalidates_session() {
    let gateway = FakeGateway::default();
    let app = app_with_gateway(gateway.clone());
    let (_, login) = send(
        &app,
        "POST",
        "/api/auth/login",
        None,
        Some(json!({ "email": "x@y.z", "password": "correct-password" })),
    )
    .await;
    let token = login["session"].as_str().unwrap();

    let (logout_status, logout_body) =
        send(&app, "POST", "/api/auth/logout", Some(token), None).await;
    let (session_status, _) = send(&app, "GET", "/api/auth/session", Some(token), None).await;

    assert_eq!(logout_status, StatusCode::OK);
    assert_eq!(logout_body, json!({ "ok": true }));
    assert_eq!(session_status, StatusCode::UNAUTHORIZED);
    assert_eq!(
        gateway.calls.lock().unwrap().as_slice(),
        ["sign_out:fake.jwt.tok"]
    );
}

#[tokio::test]
async fn expired_session_is_refreshed() {
    let gateway = FakeGateway::default();
    let store = InMemorySessionStore::default();
    store.put(
        "pcs_seed".to_string(),
        StoredSession {
            access_token: "fake.jwt.tok".to_string(),
            refresh_token: "fake-refresh-token".to_string(),
            expires_at: SystemTime::now(),
            user_id: "user-1".to_string(),
        },
    );
    let app = app_with_gateway_and_store(gateway.clone(), store.clone());

    let (status, body) = send(&app, "GET", "/api/auth/session", Some("pcs_seed"), None).await;

    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["kind"], "user");
    assert_eq!(
        gateway.calls.lock().unwrap().as_slice(),
        ["refresh:fake-refresh-token"]
    );
    let refreshed = store.get("pcs_seed").expect("refreshed session stored");
    assert_eq!(refreshed.access_token, "fake.refreshed.jwt");
    assert_eq!(refreshed.refresh_token, "fake-refreshed-token");
}

#[tokio::test]
async fn register_reports_confirmation() {
    let app = app();

    let (status, body) = send(
        &app,
        "POST",
        "/api/auth/register",
        None,
        Some(json!({ "email": "x@y.z", "password": "correct-password" })),
    )
    .await;

    assert_eq!(status, StatusCode::OK);
    assert_eq!(
        body,
        json!({ "needs_confirmation": true, "user_id": "user-1" })
    );
}

#[tokio::test]
async fn bad_password_is_unauthorized() {
    let app = app_with_gateway(FakeGateway::rejecting_password());

    let (status, body) = send(
        &app,
        "POST",
        "/api/auth/login",
        None,
        Some(json!({ "email": "x@y.z", "password": "wrong-password" })),
    )
    .await;

    assert_eq!(status, StatusCode::UNAUTHORIZED);
    assert_eq!(body, json!({ "error": "invalid_credentials" }));
}

#[tokio::test]
async fn no_supabase_leak() {
    let app = app();
    let (_, login) = send(
        &app,
        "POST",
        "/api/auth/login",
        None,
        Some(json!({ "email": "x@y.z", "password": "correct-password" })),
    )
    .await;
    let token = login["session"].as_str().unwrap();
    let (_, session) = send(&app, "GET", "/api/auth/session", Some(token), None).await;

    for response in [login, session] {
        let encoded = response.to_string();
        assert!(!encoded.contains("fake.jwt.tok"));
        assert!(!encoded.contains("fake-refresh-token"));
        assert!(!encoded.contains("SUPABASE"));
        assert!(!encoded.contains("https://supabase.example"));
    }
}

// Adversarial-review fix: logout must revoke the local session even when the
// upstream GoTrue sign_out fails (the pcs_ token is the real security boundary).
#[tokio::test]
async fn logout_is_failsafe_when_signout_errors() {
    let gateway = FakeGateway::rejecting_signout();
    let app = app_with_gateway(gateway.clone());
    let (_, login) = send(
        &app,
        "POST",
        "/api/auth/login",
        None,
        Some(json!({ "email": "x@y.z", "password": "correct-password" })),
    )
    .await;
    let token = login["session"].as_str().unwrap();

    let (logout_status, logout_body) =
        send(&app, "POST", "/api/auth/logout", Some(token), None).await;
    let (session_status, _) = send(&app, "GET", "/api/auth/session", Some(token), None).await;

    assert_eq!(logout_status, StatusCode::OK);
    assert_eq!(logout_body, json!({ "ok": true }));
    assert_eq!(session_status, StatusCode::UNAUTHORIZED);
    // sign_out was still attempted (best-effort), then ignored.
    assert_eq!(
        gateway.calls.lock().unwrap().as_slice(),
        ["sign_out:fake.jwt.tok"]
    );
}

// Adversarial-review fix: a failed refresh must evict the dead session from the
// store rather than leaving an unusable entry behind.
#[tokio::test]
async fn failed_refresh_evicts_dead_session() {
    let gateway = FakeGateway::rejecting_refresh();
    let store = InMemorySessionStore::default();
    store.put(
        "pcs_seed".to_string(),
        StoredSession {
            access_token: "fake.jwt.tok".to_string(),
            refresh_token: "fake-refresh-token".to_string(),
            expires_at: SystemTime::now(),
            user_id: "user-1".to_string(),
        },
    );
    let app = app_with_gateway_and_store(gateway.clone(), store.clone());

    let (status, body) = send(&app, "GET", "/api/auth/session", Some("pcs_seed"), None).await;

    assert_eq!(status, StatusCode::UNAUTHORIZED);
    assert_eq!(body, json!({ "error": "unauthorized" }));
    assert!(
        store.get("pcs_seed").is_none(),
        "dead session must be evicted after a failed refresh"
    );
}

#[test]
fn parse_api_key_happy_rejects_non_paperclip_and_hashes() {
    let parts = parse_api_key("paperclip_pc_ab12cd34_secret").expect("valid api key");
    assert_eq!(parts.prefix, "pc_ab12cd34");
    assert_eq!(
        parts.key_hash,
        hash_full_key("paperclip_pc_ab12cd34_secret")
    );

    assert!(parse_api_key("notpaperclip_pc_ab12cd34_secret").is_none());
    assert!(parse_api_key("paperclip_notpc_secret").is_none());
    assert_eq!(
        hash_full_key("abc"),
        "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
    );
}
