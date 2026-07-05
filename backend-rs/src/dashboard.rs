use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde_json::{json, Value};

use crate::agents::AgentRepo;
use crate::approvals::ApprovalRepo;
use crate::auth::{authorize_company_access, Actor};
use crate::issues::IssueRepo;

/// `GET /api/companies/:companyId/dashboard` — cross-domain status rollups.
/// Ports the agent/task/approval buckets of server/src/services/dashboard.ts
/// (cost/run-activity/budget sections land in later slices). Aggregates over the
/// already-ported agent, issue, and approval repositories.
pub async fn get_dashboard(
    actor: Actor,
    State(agents): State<AgentRepo>,
    State(issues): State<IssueRepo>,
    State(approvals): State<ApprovalRepo>,
    Path(company_id): Path<String>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    authorize_company_access(&actor, &company_id)?;

    // Agents: "idle" is operational, counted as active (dashboard.ts line 64).
    let (mut a_active, mut a_running, mut a_paused, mut a_error) = (0, 0, 0, 0);
    for agent in agents.0.list_by_company(&company_id) {
        match agent.status.as_str() {
            "idle" | "active" => a_active += 1,
            "running" => a_running += 1,
            "paused" => a_paused += 1,
            "error" => a_error += 1,
            _ => {}
        }
    }

    // Tasks: `open` is anything not done/cancelled (so in_progress/blocked also
    // count toward open, matching dashboard.ts lines 76-79).
    let (mut t_open, mut t_in_progress, mut t_blocked, mut t_done) = (0, 0, 0, 0);
    for issue in issues.0.list_by_company(&company_id) {
        match issue.status.as_str() {
            "in_progress" => t_in_progress += 1,
            "blocked" => t_blocked += 1,
            "done" => t_done += 1,
            _ => {}
        }
        if issue.status != "done" && issue.status != "cancelled" {
            t_open += 1;
        }
    }

    let pending_approvals = approvals
        .0
        .list_by_company(&company_id)
        .into_iter()
        .filter(|approval| approval.status == "pending")
        .count();

    Ok(Json(json!({
        "agents": {
            "active": a_active,
            "running": a_running,
            "paused": a_paused,
            "error": a_error,
        },
        "tasks": {
            "open": t_open,
            "inProgress": t_in_progress,
            "blocked": t_blocked,
            "done": t_done,
        },
        "approvals": {
            "pending": pending_approvals,
        },
    })))
}
