use std::sync::{Arc, Mutex};

use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use uuid::Uuid;

use crate::auth::{authorize_company_access, require_own_agent, Actor};
use crate::companies::CompanyRepo;

/// A reported cost event. Ports the core columns of server/src/routes/costs.ts
/// (`cost_events`); fuller fields (project/goal/heartbeat/billingCode) land in a
/// later slice.
#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CostEvent {
    pub id: String,
    pub company_id: String,
    pub agent_id: String,
    pub issue_id: Option<String>,
    pub provider: String,
    pub biller: String,
    pub billing_type: String,
    pub model: String,
    pub input_tokens: i64,
    pub cached_input_tokens: i64,
    pub output_tokens: i64,
    pub cost_cents: i64,
    pub occurred_at: String,
}

/// Ports the create-cost-event payload (packages/shared/src/validators/cost.ts):
/// `biller` defaults to `provider`; `billingType` to "unknown"; token counts to 0.
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateCostEvent {
    pub agent_id: String,
    #[serde(default)]
    pub issue_id: Option<String>,
    pub provider: String,
    #[serde(default)]
    pub biller: Option<String>,
    #[serde(default = "default_billing_type")]
    pub billing_type: String,
    pub model: String,
    #[serde(default)]
    pub input_tokens: i64,
    #[serde(default)]
    pub cached_input_tokens: i64,
    #[serde(default)]
    pub output_tokens: i64,
    pub cost_cents: i64,
    pub occurred_at: String,
}

fn default_billing_type() -> String {
    "unknown".to_string()
}

/// In-memory, company-scoped cost-event store.
#[derive(Clone, Default)]
pub struct CostStore {
    inner: Arc<Mutex<Vec<CostEvent>>>,
}

impl CostStore {
    pub fn create(&self, company_id: &str, input: CreateCostEvent) -> CostEvent {
        let biller = input.biller.unwrap_or_else(|| input.provider.clone());
        let event = CostEvent {
            id: Uuid::new_v4().to_string(),
            company_id: company_id.to_string(),
            agent_id: input.agent_id,
            issue_id: input.issue_id,
            provider: input.provider,
            biller,
            billing_type: input.billing_type,
            model: input.model,
            input_tokens: input.input_tokens,
            cached_input_tokens: input.cached_input_tokens,
            output_tokens: input.output_tokens,
            cost_cents: input.cost_cents,
            occurred_at: input.occurred_at,
        };
        self.inner.lock().unwrap().push(event.clone());
        event
    }

    /// Total reported spend (in cents) for a company.
    pub fn spend_cents(&self, company_id: &str) -> i64 {
        self.inner
            .lock()
            .unwrap()
            .iter()
            .filter(|e| e.company_id == company_id)
            .map(|e| e.cost_cents)
            .sum()
    }
}

pub trait CostRepository {
    fn create(&self, company_id: &str, input: CreateCostEvent) -> CostEvent;
    fn spend_cents(&self, company_id: &str) -> i64;
}

impl CostRepository for CostStore {
    fn create(&self, company_id: &str, input: CreateCostEvent) -> CostEvent {
        CostStore::create(self, company_id, input)
    }
    fn spend_cents(&self, company_id: &str) -> i64 {
        CostStore::spend_cents(self, company_id)
    }
}

/// Cloneable handle to whichever [`CostRepository`] backs the running app.
#[derive(Clone)]
pub struct CostRepo(pub Arc<dyn CostRepository + Send + Sync>);

impl Default for CostRepo {
    fn default() -> Self {
        CostRepo(Arc::new(CostStore::default()))
    }
}

pub async fn create_cost_event(
    actor: Actor,
    State(repo): State<CostRepo>,
    Path(company_id): Path<String>,
    Json(input): Json<CreateCostEvent>,
) -> Result<(StatusCode, Json<CostEvent>), (StatusCode, Json<Value>)> {
    authorize_company_access(&actor, &company_id)?;
    // An identified agent may only report costs attributed to itself.
    require_own_agent(&actor, &input.agent_id)?;
    Ok((StatusCode::CREATED, Json(repo.0.create(&company_id, input))))
}

/// `GET /costs/summary` — total spend vs the company's monthly budget. Ports the
/// summary in server/src/services/costs.ts (cross-domain: cost events + company).
pub async fn cost_summary(
    actor: Actor,
    State(costs): State<CostRepo>,
    State(companies): State<CompanyRepo>,
    Path(company_id): Path<String>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    authorize_company_access(&actor, &company_id)?;

    let Some(company) = companies.0.get(&company_id) else {
        return Err((
            StatusCode::NOT_FOUND,
            Json(json!({ "error": "Company not found" })),
        ));
    };

    let spend_cents = costs.0.spend_cents(&company_id);
    let budget_cents = company.budget_monthly_cents;
    let utilization_percent = if budget_cents > 0 {
        let raw = (spend_cents as f64 / budget_cents as f64) * 100.0;
        (raw * 100.0).round() / 100.0 // two decimals, matching Number.toFixed(2)
    } else {
        0.0
    };

    Ok(Json(json!({
        "companyId": company_id,
        "spendCents": spend_cents,
        "budgetCents": budget_cents,
        "utilizationPercent": utilization_percent,
    })))
}
