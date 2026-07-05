use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde_json::{json, Value};

use crate::approvals::ApprovalRepo;
use crate::auth::{authorize_company_access, Actor};
use crate::join_requests::{JoinRequestFilter, JoinRequestRepo};
use crate::runs::RunRepo;

/// `GET /api/companies/:companyId/sidebar-badges` — cross-domain pending counts.
/// Ports the core of server/src/routes/sidebar-badges.ts: failed runs + pending
/// approvals + pending join-requests, with `inbox` as their sum. The alert
/// heuristics (cost-budget/error thresholds) and per-user dismissals are deferred.
pub async fn get_sidebar_badges(
    actor: Actor,
    State(runs): State<RunRepo>,
    State(approvals): State<ApprovalRepo>,
    State(join_requests): State<JoinRequestRepo>,
    Path(company_id): Path<String>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    authorize_company_access(&actor, &company_id)?;

    let failed_runs = runs
        .0
        .list_by_company(&company_id)
        .iter()
        .filter(|r| r.status == "failed")
        .count();

    let approvals_pending = approvals
        .0
        .list_by_company(&company_id)
        .iter()
        .filter(|a| a.status == "pending")
        .count();

    let join_pending = join_requests
        .0
        .list_by_company(
            &company_id,
            &JoinRequestFilter {
                status: Some("pending_approval".to_string()),
                request_type: None,
            },
        )
        .len();

    let inbox = failed_runs + approvals_pending + join_pending;

    Ok(Json(json!({
        "failedRuns": failed_runs,
        "approvals": approvals_pending,
        "joinRequests": join_pending,
        "inbox": inbox,
    })))
}
