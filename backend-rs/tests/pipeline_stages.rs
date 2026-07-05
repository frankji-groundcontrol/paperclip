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

// Ports server/src/routes/pipelines.ts stage sub-resource (create/list/patch/
// delete), scoped under a pipeline.
#[tokio::test]
async fn creates_stage_under_pipeline() {
    let app = app_with(Repositories::default());
    let pid = make_pipeline(&app).await;

    let (status, stage) = send(
        &app,
        "POST",
        &format!("/api/companies/co-1/pipelines/{pid}/stages"),
        Some(json!({ "key": "triage", "name": "Triage", "kind": "working" })),
    )
    .await;

    assert_eq!(status, StatusCode::CREATED);
    assert_eq!(stage["key"], "triage");
    assert_eq!(stage["kind"], "working");
    assert_eq!(stage["pipelineId"], pid);
    assert_eq!(stage["position"], 0); // default
    assert_eq!(stage["config"], json!({}));
    assert!(stage["id"].as_str().is_some_and(|s| !s.is_empty()));
}

#[tokio::test]
async fn lists_stages_of_a_pipeline() {
    let app = app_with(Repositories::default());
    let pid = make_pipeline(&app).await;
    send(
        &app,
        "POST",
        &format!("/api/companies/co-1/pipelines/{pid}/stages"),
        Some(json!({ "key": "a", "name": "A", "kind": "working" })),
    )
    .await;
    send(
        &app,
        "POST",
        &format!("/api/companies/co-1/pipelines/{pid}/stages"),
        Some(json!({ "key": "b", "name": "B", "kind": "review" })),
    )
    .await;

    let (status, list) = send(
        &app,
        "GET",
        &format!("/api/companies/co-1/pipelines/{pid}/stages"),
        None,
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(list.as_array().unwrap().len(), 2);
}

#[tokio::test]
async fn create_stage_under_unknown_pipeline_is_404() {
    let app = app_with(Repositories::default());
    let (status, body) = send(
        &app,
        "POST",
        "/api/companies/co-1/pipelines/nope/stages",
        Some(json!({ "key": "a", "name": "A", "kind": "working" })),
    )
    .await;
    assert_eq!(status, StatusCode::NOT_FOUND);
    assert_eq!(body, json!({ "error": "Pipeline not found" }));
}

#[tokio::test]
async fn updates_stage_position_and_kind() {
    let app = app_with(Repositories::default());
    let pid = make_pipeline(&app).await;
    let (_, stage) = send(
        &app,
        "POST",
        &format!("/api/companies/co-1/pipelines/{pid}/stages"),
        Some(json!({ "key": "a", "name": "A", "kind": "working" })),
    )
    .await;
    let sid = stage["id"].as_str().unwrap();

    let (status, updated) = send(
        &app,
        "PATCH",
        &format!("/api/companies/co-1/pipelines/{pid}/stages/{sid}"),
        Some(json!({ "kind": "review", "position": 3 })),
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(updated["kind"], "review");
    assert_eq!(updated["position"], 3);
    assert_eq!(updated["key"], "a"); // unchanged
}

#[tokio::test]
async fn deletes_stage() {
    let app = app_with(Repositories::default());
    let pid = make_pipeline(&app).await;
    let (_, stage) = send(
        &app,
        "POST",
        &format!("/api/companies/co-1/pipelines/{pid}/stages"),
        Some(json!({ "key": "a", "name": "A", "kind": "working" })),
    )
    .await;
    let sid = stage["id"].as_str().unwrap();

    let (status, _) = send(
        &app,
        "DELETE",
        &format!("/api/companies/co-1/pipelines/{pid}/stages/{sid}"),
        None,
    )
    .await;
    assert_eq!(status, StatusCode::NO_CONTENT);

    let (_, list) = send(
        &app,
        "GET",
        &format!("/api/companies/co-1/pipelines/{pid}/stages"),
        None,
    )
    .await;
    assert_eq!(list, json!([]));
}

#[tokio::test]
async fn delete_unknown_stage_is_404() {
    let app = app_with(Repositories::default());
    let pid = make_pipeline(&app).await;
    let (status, body) = send(
        &app,
        "DELETE",
        &format!("/api/companies/co-1/pipelines/{pid}/stages/nope"),
        None,
    )
    .await;
    assert_eq!(status, StatusCode::NOT_FOUND);
    assert_eq!(body, json!({ "error": "Stage not found" }));
}
