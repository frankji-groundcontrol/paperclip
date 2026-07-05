use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;

use axum::body::Body;
use axum::http::{Request, StatusCode};
use axum::Router;
use paperclip_backend::app_with_company_repo;
use paperclip_backend::companies::CompanyRepo;
use paperclip_backend::db::SqliteCompanyStore;
use serde_json::{json, Value};
use tower::ServiceExt; // for `oneshot`

static COUNTER: AtomicU32 = AtomicU32::new(0);

fn temp_db_path() -> std::path::PathBuf {
    let n = COUNTER.fetch_add(1, Ordering::SeqCst);
    std::env::temp_dir().join(format!(
        "paperclip-http-{}-{}.sqlite",
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

/// End-to-end persistence: a company POSTed over HTTP against a SQLite-backed app
/// is still there when a brand-new app is built on the same database file.
#[tokio::test]
async fn companies_created_over_http_persist_to_disk() {
    let path = temp_db_path();
    let path_str = path.to_str().unwrap().to_string();

    {
        let app = app_with_company_repo(CompanyRepo(Arc::new(SqliteCompanyStore::open(&path_str))));
        let (status, _) = send(
            &app,
            "POST",
            "/api/companies",
            Some(json!({ "name": "Acme" })),
        )
        .await;
        assert_eq!(status, StatusCode::CREATED);
    }

    let app = app_with_company_repo(CompanyRepo(Arc::new(SqliteCompanyStore::open(&path_str))));
    let (status, list) = send(&app, "GET", "/api/companies", None).await;
    assert_eq!(status, StatusCode::OK);
    let arr = list.as_array().unwrap();
    assert_eq!(arr.len(), 1);
    assert_eq!(arr[0]["name"], "Acme");

    std::fs::remove_file(&path).ok();
}
