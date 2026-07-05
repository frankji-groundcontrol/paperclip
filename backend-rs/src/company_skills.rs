use std::sync::{Arc, Mutex};

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use uuid::Uuid;

use crate::auth::{authorize_company_access, Actor};

/// A company skill (library entry). Ports the core columns of the `CompanySkill`
/// shape (packages/shared/src/types/company-skill.ts); versions, comments,
/// sharing, file inventory and the source-import machinery are deferred.
#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CompanySkill {
    pub id: String,
    pub company_id: String,
    pub key: String,
    pub name: String,
    pub description: Option<String>,
    pub categories: Vec<String>,
    pub star_count: i64,
}

/// Ports the create-skill payload core: `key`+`name` required; description
/// nullable; `categories` default `[]`.
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateSkill {
    pub key: String,
    pub name: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub categories: Vec<String>,
}

/// Query filters for the skill list.
#[derive(Deserialize, Default)]
pub struct SkillFilter {
    #[serde(default)]
    pub q: Option<String>,
    #[serde(default)]
    pub category: Option<String>,
}

/// In-memory, company-scoped skill store.
#[derive(Clone, Default)]
pub struct CompanySkillStore {
    inner: Arc<Mutex<Vec<CompanySkill>>>,
}

impl CompanySkillStore {
    pub fn create(&self, company_id: &str, input: CreateSkill) -> CompanySkill {
        let skill = CompanySkill {
            id: Uuid::new_v4().to_string(),
            company_id: company_id.to_string(),
            key: input.key,
            name: input.name,
            description: input.description,
            categories: input.categories,
            star_count: 0,
        };
        self.inner.lock().unwrap().push(skill.clone());
        skill
    }

    pub fn list_by_company(&self, company_id: &str, filter: &SkillFilter) -> Vec<CompanySkill> {
        let q = filter.q.as_ref().map(|s| s.to_lowercase());
        self.inner
            .lock()
            .unwrap()
            .iter()
            .filter(|s| s.company_id == company_id)
            .filter(|s| {
                q.as_ref().is_none_or(|needle| {
                    format!("{} {}", s.key, s.name)
                        .to_lowercase()
                        .contains(needle)
                        || s.description
                            .as_ref()
                            .is_some_and(|d| d.to_lowercase().contains(needle))
                })
            })
            .filter(|s| {
                filter
                    .category
                    .as_ref()
                    .is_none_or(|c| s.categories.iter().any(|cat| cat == c))
            })
            .cloned()
            .collect()
    }

    pub fn get(&self, company_id: &str, skill_id: &str) -> Option<CompanySkill> {
        self.inner
            .lock()
            .unwrap()
            .iter()
            .find(|s| s.company_id == company_id && s.id == skill_id)
            .cloned()
    }

    /// Adjusts the star count by `delta` (clamped at 0), returning the skill.
    pub fn adjust_star(
        &self,
        company_id: &str,
        skill_id: &str,
        delta: i64,
    ) -> Option<CompanySkill> {
        let mut guard = self.inner.lock().unwrap();
        let skill = guard
            .iter_mut()
            .find(|s| s.company_id == company_id && s.id == skill_id)?;
        skill.star_count = (skill.star_count + delta).max(0);
        Some(skill.clone())
    }
}

pub trait CompanySkillRepository {
    fn create(&self, company_id: &str, input: CreateSkill) -> CompanySkill;
    fn list_by_company(&self, company_id: &str, filter: &SkillFilter) -> Vec<CompanySkill>;
    fn get(&self, company_id: &str, skill_id: &str) -> Option<CompanySkill>;
    fn adjust_star(&self, company_id: &str, skill_id: &str, delta: i64) -> Option<CompanySkill>;
}

impl CompanySkillRepository for CompanySkillStore {
    fn create(&self, company_id: &str, input: CreateSkill) -> CompanySkill {
        CompanySkillStore::create(self, company_id, input)
    }
    fn list_by_company(&self, company_id: &str, filter: &SkillFilter) -> Vec<CompanySkill> {
        CompanySkillStore::list_by_company(self, company_id, filter)
    }
    fn get(&self, company_id: &str, skill_id: &str) -> Option<CompanySkill> {
        CompanySkillStore::get(self, company_id, skill_id)
    }
    fn adjust_star(&self, company_id: &str, skill_id: &str, delta: i64) -> Option<CompanySkill> {
        CompanySkillStore::adjust_star(self, company_id, skill_id, delta)
    }
}

/// Cloneable handle to whichever [`CompanySkillRepository`] backs the app.
#[derive(Clone)]
pub struct CompanySkillRepo(pub Arc<dyn CompanySkillRepository + Send + Sync>);

impl Default for CompanySkillRepo {
    fn default() -> Self {
        CompanySkillRepo(Arc::new(CompanySkillStore::default()))
    }
}

fn skill_not_found() -> (StatusCode, Json<Value>) {
    (
        StatusCode::NOT_FOUND,
        Json(json!({ "error": "Skill not found" })),
    )
}

pub async fn list_skills(
    actor: Actor,
    State(repo): State<CompanySkillRepo>,
    Path(company_id): Path<String>,
    Query(filter): Query<SkillFilter>,
) -> Result<Json<Vec<CompanySkill>>, (StatusCode, Json<Value>)> {
    authorize_company_access(&actor, &company_id)?;
    Ok(Json(repo.0.list_by_company(&company_id, &filter)))
}

pub async fn create_skill(
    actor: Actor,
    State(repo): State<CompanySkillRepo>,
    Path(company_id): Path<String>,
    Json(input): Json<CreateSkill>,
) -> Result<(StatusCode, Json<CompanySkill>), (StatusCode, Json<Value>)> {
    authorize_company_access(&actor, &company_id)?;
    Ok((StatusCode::CREATED, Json(repo.0.create(&company_id, input))))
}

pub async fn get_skill(
    actor: Actor,
    State(repo): State<CompanySkillRepo>,
    Path((company_id, skill_id)): Path<(String, String)>,
) -> Result<Json<CompanySkill>, (StatusCode, Json<Value>)> {
    authorize_company_access(&actor, &company_id)?;
    match repo.0.get(&company_id, &skill_id) {
        Some(skill) => Ok(Json(skill)),
        None => Err(skill_not_found()),
    }
}

pub async fn star_skill(
    actor: Actor,
    State(repo): State<CompanySkillRepo>,
    Path((company_id, skill_id)): Path<(String, String)>,
) -> Result<Json<CompanySkill>, (StatusCode, Json<Value>)> {
    authorize_company_access(&actor, &company_id)?;
    match repo.0.adjust_star(&company_id, &skill_id, 1) {
        Some(skill) => Ok(Json(skill)),
        None => Err(skill_not_found()),
    }
}

pub async fn unstar_skill(
    actor: Actor,
    State(repo): State<CompanySkillRepo>,
    Path((company_id, skill_id)): Path<(String, String)>,
) -> Result<Json<CompanySkill>, (StatusCode, Json<Value>)> {
    authorize_company_access(&actor, &company_id)?;
    match repo.0.adjust_star(&company_id, &skill_id, -1) {
        Some(skill) => Ok(Json(skill)),
        None => Err(skill_not_found()),
    }
}
