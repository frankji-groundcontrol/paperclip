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

async fn make_pipeline(app: &Router) -> String {
    let (_, p) = send(
        app,
        "POST",
        "/api/companies/co-1/pipelines",
        Some(json!({ "key": "intake", "name": "Intake" })),
    )
    .await;
    p["id"].as_str().unwrap().to_string()
}

// Ports server/src/routes/pipelines.ts case ingest core (ingest/list/get/patch);
// transitions/documents/leases are deferred.
#[tokio::test]
async fn ingests_case_under_pipeline() {
    let app = app_with(Repositories::default());
    let pid = make_pipeline(&app).await;

    let (status, c) = send(
        &app,
        "POST",
        &format!("/api/companies/co-1/pipelines/{pid}/cases"),
        Some(json!({ "title": "Refund #12" })),
    )
    .await;

    assert_eq!(status, StatusCode::CREATED);
    assert_eq!(c["title"], "Refund #12");
    assert_eq!(c["pipelineId"], pid);
    assert_eq!(c["fields"], json!({}));
    assert_eq!(c["summary"], Value::Null);
    assert_eq!(c["caseKey"], Value::Null);
    assert!(c["id"].as_str().is_some_and(|s| !s.is_empty()));
}

#[tokio::test]
async fn lists_and_gets_a_case() {
    let app = app_with(Repositories::default());
    let pid = make_pipeline(&app).await;
    let (_, created) = send(
        &app,
        "POST",
        &format!("/api/companies/co-1/pipelines/{pid}/cases"),
        Some(json!({ "title": "A", "stageKey": "triage" })),
    )
    .await;
    let cid = created["id"].as_str().unwrap();

    let (status, list) = send(
        &app,
        "GET",
        &format!("/api/companies/co-1/pipelines/{pid}/cases"),
        None,
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(list.as_array().unwrap().len(), 1);

    let (status, got) = send(
        &app,
        "GET",
        &format!("/api/companies/co-1/pipelines/{pid}/cases/{cid}"),
        None,
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(got["id"], cid);
    assert_eq!(got["stageKey"], "triage");
}

#[tokio::test]
async fn ingest_under_unknown_pipeline_is_404() {
    let app = app_with(Repositories::default());
    let (status, body) = send(
        &app,
        "POST",
        "/api/companies/co-1/pipelines/nope/cases",
        Some(json!({ "title": "A" })),
    )
    .await;
    assert_eq!(status, StatusCode::NOT_FOUND);
    assert_eq!(body, json!({ "error": "Pipeline not found" }));
}

#[tokio::test]
async fn patches_case_title_and_fields() {
    let app = app_with(Repositories::default());
    let pid = make_pipeline(&app).await;
    let (_, created) = send(
        &app,
        "POST",
        &format!("/api/companies/co-1/pipelines/{pid}/cases"),
        Some(json!({ "title": "A" })),
    )
    .await;
    let cid = created["id"].as_str().unwrap();

    let (status, updated) = send(
        &app,
        "PATCH",
        &format!("/api/companies/co-1/pipelines/{pid}/cases/{cid}"),
        Some(json!({ "title": "A v2", "fields": { "priority": "high" } })),
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(updated["title"], "A v2");
    assert_eq!(updated["fields"], json!({ "priority": "high" }));
}

#[tokio::test]
async fn get_unknown_case_is_404() {
    let app = app_with(Repositories::default());
    let pid = make_pipeline(&app).await;
    let (status, body) = send(
        &app,
        "GET",
        &format!("/api/companies/co-1/pipelines/{pid}/cases/nope"),
        None,
    )
    .await;
    assert_eq!(status, StatusCode::NOT_FOUND);
    assert_eq!(body, json!({ "error": "Case not found" }));
}
