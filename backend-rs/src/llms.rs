use axum::{
    http::{header, StatusCode},
    response::{IntoResponse, Response},
    Json,
};
use serde_json::{json, Value};

use crate::auth::Actor;

/// Agent icon names — ports AGENT_ICON_NAMES (packages/shared/src/constants.ts).
const ICON_NAMES: &[&str] = &[
    "bot",
    "cpu",
    "brain",
    "zap",
    "rocket",
    "code",
    "terminal",
    "shield",
    "eye",
    "search",
    "wrench",
    "hammer",
    "lightbulb",
    "sparkles",
    "star",
    "heart",
    "flame",
    "bug",
    "cog",
    "database",
    "globe",
    "lock",
    "mail",
    "message-square",
    "file-code",
    "git-branch",
    "package",
    "puzzle",
    "target",
    "wand",
    "atom",
    "circuit-board",
    "radar",
    "swords",
    "telescope",
    "microscope",
    "crown",
    "gem",
    "hexagon",
    "pentagon",
    "fingerprint",
];

/// Config-reflection read guard. Ports the `assertCanRead` gate in
/// server/src/routes/llms.ts: the board may read; agents need the
/// `canCreateAgents` permission — which the OSS actor model doesn't yet track,
/// so agents are rejected for now (the permission check lands with a richer
/// agent model).
fn assert_can_read(actor: &Actor) -> Result<(), (StatusCode, Json<Value>)> {
    match actor {
        Actor::Board { .. } => Ok(()),
        Actor::Agent { .. } => Err((
            StatusCode::FORBIDDEN,
            Json(json!({ "error": "Board or permitted agent authentication required" })),
        )),
    }
}

/// Wraps a `String` body as a `text/plain` response.
fn text(body: String) -> Response {
    ([(header::CONTENT_TYPE, "text/plain; charset=utf-8")], body).into_response()
}

pub async fn agent_configuration_index(
    actor: Actor,
) -> Result<Response, (StatusCode, Json<Value>)> {
    assert_can_read(&actor)?;
    let lines = [
        "# Paperclip Agent Configuration Index",
        "",
        "Related API endpoints:",
        "- GET /api/companies/:companyId/agent-configurations",
        "- GET /api/agents/:id/configuration",
        "- POST /api/companies/:companyId/agent-hires",
        "",
        "Agent identity references:",
        "- GET /llms/agent-icons.txt",
        "",
        "Notes:",
        "- Sensitive values are redacted in configuration read APIs.",
        "- New hires may be created in pending_approval state depending on company settings.",
        "",
    ];
    Ok(text(lines.join("\n")))
}

pub async fn agent_icons(actor: Actor) -> Result<Response, (StatusCode, Json<Value>)> {
    assert_can_read(&actor)?;
    let mut lines = vec![
        "# Paperclip Agent Icon Names".to_string(),
        String::new(),
        "Set the `icon` field on hire/create payloads to one of:".to_string(),
    ];
    lines.extend(ICON_NAMES.iter().map(|name| format!("- {name}")));
    lines.push(String::new());
    Ok(text(lines.join("\n")))
}
