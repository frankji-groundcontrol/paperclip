use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;

use axum::body::Body;
use axum::http::{Request, StatusCode};
use axum::Router;
use paperclip_backend::db::SqlitePluginStore;
use paperclip_backend::plugins::PluginRepo;
use paperclip_backend::{app_with, Repositories};
use serde_json::{json, Value};
use tower::ServiceExt; // for `oneshot`

static COUNTER: AtomicU32 = AtomicU32::new(0);

fn temp_db_path() -> std::path::PathBuf {
    let n = COUNTER.fetch_add(1, Ordering::SeqCst);
    std::env::temp_dir().join(format!(
        "paperclip-plugin-http-{}-{}.sqlite",
        std::process::id(),
        n
    ))
}

async fn send(app: &Router, method: &str, uri: &str, body: Option<Value>) -> (StatusCode, Value) {
    let builder = Request::builder().method(method).uri(uri);
    let request = match body {
        Some(b) => builder
            .header("content-type", "application/json")
            .body(Body::from(b.to_string()))
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

fn app_with_sqlite_plugins(path: &str) -> Router {
    app_with(Repositories {
        plugins: PluginRepo(Arc::new(SqlitePluginStore::open(path))),
        ..Default::default()
    })
}

/// End-to-end: a plugin installed over HTTP against a SQLite-backed app is still
/// there when a fresh app is built on the same database file, company-scoped.
#[tokio::test]
async fn plugins_installed_over_http_persist_to_disk() {
    let path = temp_db_path();
    let path_str = path.to_str().unwrap().to_string();

    {
        let app = app_with_sqlite_plugins(&path_str);
        let (status, _) = send(
            &app,
            "POST",
            "/api/companies/co-1/plugins",
            Some(json!({ "pluginId": "llm-wiki" })),
        )
        .await;
        assert_eq!(status, StatusCode::CREATED);
    }

    let app = app_with_sqlite_plugins(&path_str);
    let (status, list) = send(&app, "GET", "/api/companies/co-1/plugins", None).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(list.as_array().unwrap().len(), 1);
    assert_eq!(list[0]["pluginId"], "llm-wiki");
    assert_eq!(list[0]["enabled"], true);

    let (_, other) = send(&app, "GET", "/api/companies/co-2/plugins", None).await;
    assert_eq!(other, json!([]));

    std::fs::remove_file(&path).ok();
}
