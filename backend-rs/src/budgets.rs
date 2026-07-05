use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde::Deserialize;
use serde_json::{json, Value};

use crate::auth::{authorize_company_access, Actor};

/// A company's budget state. Ports the budget hard-stop invariant
/// (server/src/services/budgets.ts): once `spent_cents` reaches a positive
/// `monthly_limit_cents`, the company is over budget and agents auto-pause.
#[derive(Clone)]
pub struct Budget {
    pub company_id: String,
    pub monthly_limit_cents: i64,
    pub spent_cents: i64,
}

impl Budget {
    fn zero(company_id: &str) -> Self {
        Budget {
            company_id: company_id.to_string(),
            monthly_limit_cents: 0,
            spent_cents: 0,
        }
    }

    /// The hard-stop: over budget when a positive limit has been reached.
    pub fn exceeded(&self) -> bool {
        self.monthly_limit_cents > 0 && self.spent_cents >= self.monthly_limit_cents
    }

    fn to_json(&self) -> Value {
        json!({
            "companyId": self.company_id,
            "monthlyLimitCents": self.monthly_limit_cents,
            "spentCents": self.spent_cents,
            "exceeded": self.exceeded(),
        })
    }
}

pub trait BudgetRepository {
    fn get(&self, company_id: &str) -> Budget;
    fn set_limit(&self, company_id: &str, monthly_limit_cents: i64) -> Budget;
    fn record_spend(&self, company_id: &str, amount_cents: i64) -> Budget;
}

#[derive(Clone, Default)]
pub struct BudgetStore {
    inner: Arc<Mutex<HashMap<String, Budget>>>,
}

impl BudgetStore {
    pub fn get(&self, company_id: &str) -> Budget {
        self.inner
            .lock()
            .unwrap()
            .get(company_id)
            .cloned()
            .unwrap_or_else(|| Budget::zero(company_id))
    }

    pub fn set_limit(&self, company_id: &str, monthly_limit_cents: i64) -> Budget {
        let mut map = self.inner.lock().unwrap();
        let budget = map
            .entry(company_id.to_string())
            .or_insert_with(|| Budget::zero(company_id));
        budget.monthly_limit_cents = monthly_limit_cents;
        budget.clone()
    }

    pub fn record_spend(&self, company_id: &str, amount_cents: i64) -> Budget {
        let mut map = self.inner.lock().unwrap();
        let budget = map
            .entry(company_id.to_string())
            .or_insert_with(|| Budget::zero(company_id));
        budget.spent_cents += amount_cents;
        budget.clone()
    }
}

impl BudgetRepository for BudgetStore {
    fn get(&self, company_id: &str) -> Budget {
        BudgetStore::get(self, company_id)
    }
    fn set_limit(&self, company_id: &str, monthly_limit_cents: i64) -> Budget {
        BudgetStore::set_limit(self, company_id, monthly_limit_cents)
    }
    fn record_spend(&self, company_id: &str, amount_cents: i64) -> Budget {
        BudgetStore::record_spend(self, company_id, amount_cents)
    }
}

/// Cloneable handle to whichever [`BudgetRepository`] backs the running app.
#[derive(Clone)]
pub struct BudgetRepo(pub Arc<dyn BudgetRepository + Send + Sync>);

impl Default for BudgetRepo {
    fn default() -> Self {
        BudgetRepo(Arc::new(BudgetStore::default()))
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetLimit {
    pub monthly_limit_cents: i64,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Spend {
    pub amount_cents: i64,
}

pub async fn get_budget(
    actor: Actor,
    State(repo): State<BudgetRepo>,
    Path(company_id): Path<String>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    authorize_company_access(&actor, &company_id)?;
    Ok(Json(repo.0.get(&company_id).to_json()))
}

pub async fn set_budget_limit(
    actor: Actor,
    State(repo): State<BudgetRepo>,
    Path(company_id): Path<String>,
    Json(input): Json<SetLimit>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    authorize_company_access(&actor, &company_id)?;
    Ok(Json(
        repo.0
            .set_limit(&company_id, input.monthly_limit_cents)
            .to_json(),
    ))
}

pub async fn record_budget_spend(
    actor: Actor,
    State(repo): State<BudgetRepo>,
    Path(company_id): Path<String>,
    Json(input): Json<Spend>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    authorize_company_access(&actor, &company_id)?;
    Ok(Json(
        repo.0
            .record_spend(&company_id, input.amount_cents)
            .to_json(),
    ))
}
