use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde_json::{json, Map, Value};

use crate::activity::{ActivityFilters, ActivityRepo};
use crate::auth::{authorize_company_access, Actor};

/// The most recent activity events surfaced on a profile.
const RECENT_LIMIT: usize = 10;

/// `GET /api/companies/:companyId/users/:userId/profile` — a focused port of
/// server/src/routes/user-profiles.ts: a per-user rollup over the activity feed
/// (events authored by `userId`). The fuller cost/issue/agent windows are
/// deferred until those domains carry per-user keys.
pub async fn get_user_profile(
    actor: Actor,
    State(activity): State<ActivityRepo>,
    Path((company_id, user_id)): Path<(String, String)>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    authorize_company_access(&actor, &company_id)?;

    // Newest-first, this company's events authored by the user.
    let events: Vec<_> = activity
        .0
        .list(&company_id, &ActivityFilters::default())
        .into_iter()
        .filter(|e| e.actor_id == user_id)
        .collect();

    let mut action_counts = Map::new();
    for event in &events {
        let entry = action_counts
            .entry(event.action.clone())
            .or_insert(Value::from(0));
        if let Some(n) = entry.as_i64() {
            *entry = Value::from(n + 1);
        }
    }

    let recent: Vec<Value> = events
        .iter()
        .take(RECENT_LIMIT)
        .map(|e| serde_json::to_value(e).expect("serialize activity"))
        .collect();

    Ok(Json(json!({
        "userId": user_id,
        "companyId": company_id,
        "activityCount": events.len(),
        "recentActivity": recent,
        "actionCounts": Value::Object(action_counts),
    })))
}
