use axum::body::Body;
use axum::http::{Request, StatusCode};
use axum::Router;
use paperclip_backend::{app_with, Repositories};
use serde_json::{json, Value};
use tower::ServiceExt; // for `oneshot`

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

// Ports server/src/routes/teams-catalog.ts core: catalog list/filter + company
// install tracking (over a seeded catalog; the on-disk bundled loader is deferred).
#[tokio::test]
async fn lists_catalog_teams() {
    let app = app_with(Repositories::default());
    let (status, list) = send(&app, "GET", "/api/teams/catalog", None).await;

    assert_eq!(status, StatusCode::OK);
    let teams = list.as_array().unwrap();
    assert!(teams.len() >= 2);
    assert!(teams
        .iter()
        .all(|t| t["id"].is_string() && t["key"].is_string() && t["name"].is_string()));
}

#[tokio::test]
async fn filters_catalog_by_category() {
    let app = app_with(Repositories::default());
    let (status, list) = send(&app, "GET", "/api/teams/catalog?category=engineering", None).await;
    assert_eq!(status, StatusCode::OK);
    let teams = list.as_array().unwrap();
    assert!(!teams.is_empty());
    assert!(teams.iter().all(|t| t["category"] == "engineering"));
}

#[tokio::test]
async fn filters_catalog_by_search_query() {
    let app = app_with(Repositories::default());
    let (_, list) = send(&app, "GET", "/api/teams/catalog?q=research", None).await;
    let teams = list.as_array().unwrap();
    assert!(!teams.is_empty());
    assert!(teams.iter().all(|t| {
        let hay = format!("{} {}", t["name"], t["description"]).to_lowercase();
        hay.contains("research")
    }));
}

#[tokio::test]
async fn installs_and_lists_installed() {
    let app = app_with(Repositories::default());

    // Pick the first catalog team's id.
    let (_, catalog) = send(&app, "GET", "/api/teams/catalog", None).await;
    let catalog_id = catalog[0]["id"].as_str().unwrap().to_string();

    let (status, installed) = send(
        &app,
        "POST",
        &format!("/api/companies/co-1/teams/catalog/{catalog_id}/install"),
        Some(json!({})),
    )
    .await;
    assert_eq!(status, StatusCode::CREATED);
    assert_eq!(installed["catalogId"], catalog_id);

    let (status, list) = send(
        &app,
        "GET",
        "/api/companies/co-1/teams/catalog/installed",
        None,
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(list.as_array().unwrap().len(), 1);
    assert_eq!(list[0]["catalogId"], catalog_id);
}

#[tokio::test]
async fn install_unknown_catalog_is_404() {
    let app = app_with(Repositories::default());
    let (status, body) = send(
        &app,
        "POST",
        "/api/companies/co-1/teams/catalog/nope/install",
        Some(json!({})),
    )
    .await;
    assert_eq!(status, StatusCode::NOT_FOUND);
    assert_eq!(body, json!({ "error": "Catalog team not found" }));
}
