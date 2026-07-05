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

/// Sets up a pipeline with a "review" stage and one ingested case; returns
/// (pipelineId, caseId).
async fn setup(app: &Router) -> (String, String) {
    let (_, p) = send(
        app,
        "POST",
        "/api/companies/co-1/pipelines",
        Some(json!({ "key": "intake", "name": "Intake" })),
    )
    .await;
    let pid = p["id"].as_str().unwrap().to_string();
    send(
        app,
        "POST",
        &format!("/api/companies/co-1/pipelines/{pid}/stages"),
        Some(json!({ "key": "review", "name": "Review", "kind": "review" })),
    )
    .await;
    let (_, c) = send(
        app,
        "POST",
        &format!("/api/companies/co-1/pipelines/{pid}/cases"),
        Some(json!({ "title": "Refund #12" })),
    )
    .await;
    let cid = c["id"].as_str().unwrap().to_string();
    (pid, cid)
}

// Ports server/src/routes/pipelines.ts case transition: move a case to a stage
// with optimistic concurrency (expectedVersion).
#[tokio::test]
async fn transitions_case_to_stage_and_bumps_version() {
    let app = app_with(Repositories::default());
    let (pid, cid) = setup(&app).await;

    let (status, moved) = send(
        &app,
        "POST",
        &format!("/api/companies/co-1/pipelines/{pid}/cases/{cid}/transition"),
        Some(json!({ "toStageKey": "review", "expectedVersion": 1 })),
    )
    .await;

    assert_eq!(status, StatusCode::OK);
    assert_eq!(moved["stageKey"], "review");
    assert_eq!(moved["version"], 2); // bumped from 1
}

#[tokio::test]
async fn transition_to_unknown_stage_is_404() {
    let app = app_with(Repositories::default());
    let (pid, cid) = setup(&app).await;

    let (status, body) = send(
        &app,
        "POST",
        &format!("/api/companies/co-1/pipelines/{pid}/cases/{cid}/transition"),
        Some(json!({ "toStageKey": "nonexistent", "expectedVersion": 1 })),
    )
    .await;
    assert_eq!(status, StatusCode::NOT_FOUND);
    assert_eq!(body, json!({ "error": "Stage not found" }));
}

#[tokio::test]
async fn transition_with_wrong_version_is_conflict() {
    let app = app_with(Repositories::default());
    let (pid, cid) = setup(&app).await;

    let (status, body) = send(
        &app,
        "POST",
        &format!("/api/companies/co-1/pipelines/{pid}/cases/{cid}/transition"),
        Some(json!({ "toStageKey": "review", "expectedVersion": 99 })),
    )
    .await;
    assert_eq!(status, StatusCode::CONFLICT);
    assert_eq!(body, json!({ "error": "Version conflict" }));
}

#[tokio::test]
async fn transition_unknown_case_is_404() {
    let app = app_with(Repositories::default());
    let (pid, _cid) = setup(&app).await;

    let (status, body) = send(
        &app,
        "POST",
        &format!("/api/companies/co-1/pipelines/{pid}/cases/nope/transition"),
        Some(json!({ "toStageKey": "review", "expectedVersion": 1 })),
    )
    .await;
    assert_eq!(status, StatusCode::NOT_FOUND);
    assert_eq!(body, json!({ "error": "Case not found" }));
}
