use axum::Json;
use serde_json::{json, Value};

/// `GET /api/mcp/tools` — lists the control-plane tools exposed to agents,
/// mirroring the tool surface of `packages/mcp-server`. Each entry is a
/// `{ name, description }` descriptor. More tools are added as their routes land.
pub async fn list_mcp_tools() -> Json<Value> {
    Json(json!([
        { "name": "paperclipListCompanies", "description": "List companies on the instance." },
        { "name": "paperclipGetCompany", "description": "Get a company by id." },
        { "name": "paperclipListIssues", "description": "List a company's issues." },
        { "name": "paperclipGetIssue", "description": "Get an issue by id." },
        { "name": "paperclipCreateIssue", "description": "Create an issue in a company." },
        { "name": "paperclipListProjects", "description": "List a company's projects." },
        { "name": "paperclipListAgents", "description": "List a company's agents." },
        { "name": "paperclipListRuns", "description": "List a company's runs." },
        { "name": "paperclipListApprovals", "description": "List a company's approvals." },
        { "name": "paperclipGetBudget", "description": "Get a company's budget and hard-stop state." },
        { "name": "paperclipWhoami", "description": "Report the authenticated actor." }
    ]))
}
