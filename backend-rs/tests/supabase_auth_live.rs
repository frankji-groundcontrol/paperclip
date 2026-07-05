use std::sync::Arc;

use axum::body::Body;
use axum::http::{header, Request, StatusCode};
use paperclip_backend::supabase::broker::{AuthBroker, InMemorySessionStore};
use paperclip_backend::supabase::gateway::{HttpSupabaseGateway, SupabaseConfig};
use paperclip_backend::{app_with, Repositories};
use serde_json::{json, Value};
use tower::ServiceExt;

fn live_enabled() -> bool {
    std::env::var("PAPERCLIP_LIVE_SUPABASE").ok().as_deref() == Some("1")
}

fn env(name: &str) -> String {
    std::env::var(name).unwrap_or_else(|_| panic!("{name} is required for live Supabase tests"))
}

fn app() -> axum::Router {
    let config = SupabaseConfig::from_env().expect("SUPABASE_URL and SUPABASE_ANON_KEY");
    app_with(Repositories {
        supabase_auth: AuthBroker::new(
            Arc::new(HttpSupabaseGateway::new(config.url, config.anon_key)),
            Arc::new(InMemorySessionStore::default()),
        ),
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
#[ignore]
async fn live_login_and_whoami() {
    if !live_enabled() {
        return;
    }

    let app = app();
    let email = env("PC_TEST_EMAIL");
    let password = env("PC_TEST_PASSWORD");
    let (login_status, login) = send(
        &app,
        "POST",
        "/api/auth/login",
        None,
        Some(json!({ "email": email, "password": password })),
    )
    .await;
    assert_eq!(login_status, StatusCode::OK);
    let token = login["session"].as_str().expect("session token");

    let (session_status, session) = send(&app, "GET", "/api/auth/session", Some(token), None).await;
    assert_eq!(session_status, StatusCode::OK);
    assert_eq!(session["kind"], "user");
    assert_eq!(session["user"]["email"], env("PC_TEST_EMAIL"));
    assert!(session["teams"]
        .as_array()
        .is_some_and(|teams| !teams.is_empty()));
}

#[tokio::test]
#[ignore]
async fn live_bad_password_rejected() {
    if !live_enabled() {
        return;
    }

    let app = app();
    let (status, _) = send(
        &app,
        "POST",
        "/api/auth/login",
        None,
        Some(json!({
            "email": env("PC_TEST_EMAIL"),
            "password": "definitely-wrong-password",
        })),
    )
    .await;

    assert_eq!(status, StatusCode::UNAUTHORIZED);
}
