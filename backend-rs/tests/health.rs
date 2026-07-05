use axum::body::Body;
use axum::http::{Request, StatusCode};
use paperclip_backend::app;
use serde_json::Value;
use tower::ServiceExt; // for `oneshot`

async fn get_health() -> Value {
    let response = app()
        .oneshot(
            Request::builder()
                .uri("/api/health")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    serde_json::from_slice(&bytes).unwrap()
}

/// Ports the core invariant of the Express `GET /api/health` route
/// (server/src/routes/health.ts): every success response carries `status: "ok"`.
#[tokio::test]
async fn get_health_returns_ok_status() {
    let json = get_health().await;
    assert_eq!(json["status"], "ok");
}

/// Slice 2: the default (non-authenticated) response also carries a `version`
/// string, mirroring `{ status: "ok", version, serverInfo }` in health.ts.
#[tokio::test]
async fn get_health_includes_version() {
    let json = get_health().await;
    let version = json["version"].as_str();
    assert!(
        version.is_some_and(|v| !v.is_empty()),
        "expected a non-empty version string, got {:?}",
        json["version"]
    );
}
