use std::sync::{Arc, Mutex};

use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use uuid::Uuid;

/// A company — the root scope of the control plane. Ports the `companies` row
/// (packages/db/src/schema/companies.ts); fuller fields (createdAt, branding,
/// attachment limits) are added in later slices.
#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Company {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub budget_monthly_cents: i64,
}

/// Ports `createCompanySchema` (packages/shared/src/validators/company.ts).
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateCompany {
    pub name: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub budget_monthly_cents: i64,
}

/// Partial update payload for a company (`updateCompanySchema`).
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateCompany {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub budget_monthly_cents: Option<i64>,
}

/// In-memory company repository. Cloneable and Arc-backed so router clones share
/// the same state; a Postgres-backed implementation replaces this in a later slice.
#[derive(Clone, Default)]
pub struct CompanyStore {
    inner: Arc<Mutex<Vec<Company>>>,
}

impl CompanyStore {
    pub fn list(&self) -> Vec<Company> {
        self.inner.lock().unwrap().clone()
    }

    pub fn create(&self, input: CreateCompany) -> Company {
        let company = Company {
            id: Uuid::new_v4().to_string(),
            name: input.name,
            description: input.description,
            budget_monthly_cents: input.budget_monthly_cents,
        };
        self.inner.lock().unwrap().push(company.clone());
        company
    }

    pub fn get(&self, id: &str) -> Option<Company> {
        self.inner
            .lock()
            .unwrap()
            .iter()
            .find(|c| c.id == id)
            .cloned()
    }

    pub fn update(&self, id: &str, patch: UpdateCompany) -> Option<Company> {
        let mut guard = self.inner.lock().unwrap();
        let company = guard.iter_mut().find(|c| c.id == id)?;
        if let Some(name) = patch.name {
            company.name = name;
        }
        if let Some(description) = patch.description {
            company.description = Some(description);
        }
        if let Some(budget) = patch.budget_monthly_cents {
            company.budget_monthly_cents = budget;
        }
        Some(company.clone())
    }

    pub fn delete(&self, id: &str) -> bool {
        let mut guard = self.inner.lock().unwrap();
        let before = guard.len();
        guard.retain(|c| c.id != id);
        guard.len() != before
    }
}

/// The company persistence interface. Both the in-memory [`CompanyStore`] and the
/// SQLite-backed store (see `crate::db`) implement this, so handlers can be moved
/// onto durable storage without changing their logic.
pub trait CompanyRepository {
    fn list(&self) -> Vec<Company>;
    fn create(&self, input: CreateCompany) -> Company;
    fn get(&self, id: &str) -> Option<Company>;
    fn update(&self, id: &str, patch: UpdateCompany) -> Option<Company>;
    fn delete(&self, id: &str) -> bool;
}

impl CompanyRepository for CompanyStore {
    fn list(&self) -> Vec<Company> {
        CompanyStore::list(self)
    }
    fn create(&self, input: CreateCompany) -> Company {
        CompanyStore::create(self, input)
    }
    fn get(&self, id: &str) -> Option<Company> {
        CompanyStore::get(self, id)
    }
    fn update(&self, id: &str, patch: UpdateCompany) -> Option<Company> {
        CompanyStore::update(self, id, patch)
    }
    fn delete(&self, id: &str) -> bool {
        CompanyStore::delete(self, id)
    }
}

/// Cloneable handle to whichever [`CompanyRepository`] backs the running app
/// (in-memory or SQLite). Used as axum state so handlers are storage-agnostic.
#[derive(Clone)]
pub struct CompanyRepo(pub Arc<dyn CompanyRepository + Send + Sync>);

impl Default for CompanyRepo {
    fn default() -> Self {
        CompanyRepo(Arc::new(CompanyStore::default()))
    }
}

pub async fn list_companies(State(repo): State<CompanyRepo>) -> Json<Vec<Company>> {
    Json(repo.0.list())
}

pub async fn create_company(
    State(repo): State<CompanyRepo>,
    Json(input): Json<CreateCompany>,
) -> (StatusCode, Json<Company>) {
    (StatusCode::CREATED, Json(repo.0.create(input)))
}

pub async fn get_company(
    State(repo): State<CompanyRepo>,
    Path(id): Path<String>,
) -> Result<Json<Company>, (StatusCode, Json<Value>)> {
    match repo.0.get(&id) {
        Some(company) => Ok(Json(company)),
        None => Err(company_not_found()),
    }
}

fn company_not_found() -> (StatusCode, Json<Value>) {
    (
        StatusCode::NOT_FOUND,
        Json(json!({ "error": "Company not found" })),
    )
}

pub async fn update_company(
    State(repo): State<CompanyRepo>,
    Path(id): Path<String>,
    Json(patch): Json<UpdateCompany>,
) -> Result<Json<Company>, (StatusCode, Json<Value>)> {
    match repo.0.update(&id, patch) {
        Some(company) => Ok(Json(company)),
        None => Err(company_not_found()),
    }
}

pub async fn delete_company(
    State(repo): State<CompanyRepo>,
    Path(id): Path<String>,
) -> Result<StatusCode, (StatusCode, Json<Value>)> {
    if repo.0.delete(&id) {
        Ok(StatusCode::NO_CONTENT)
    } else {
        Err(company_not_found())
    }
}
