use axum::body::Body;
use axum::http::{Request, StatusCode};
use paperclip_backend::{app_with, Repositories};
use tower::ServiceExt; // for `oneshot`

/// Ports the MCP tool surface (packages/mcp-server): `GET /api/mcp/tools` lists
/// the control-plane tools an agent can call, each with a name and description.
#[tokio::test]
async fn lists_mcp_tools() {
    let app = app_with(Repositories::default());
    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/mcp/tools")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let body: serde_json::Value = serde_json::from_slice(&bytes).unwrap();

    let tools = body.as_array().expect("tools should be an array");
    assert!(!tools.is_empty(), "expected at least one tool");
    assert!(
        tools.iter().any(|t| t["name"] == "paperclipListIssues"),
        "expected the paperclipListIssues tool to be listed"
    );
    assert!(
        tools
            .iter()
            .all(|t| t["name"].is_string() && t["description"].is_string()),
        "every tool must have a name and description"
    );
}
