use std::sync::{Arc, Mutex};

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::auth::{authorize_company_access, Actor};

/// A catalog team template. Ports the core fields of the `CatalogTeam` shape
/// (packages/shared/src/types/teams-catalog.ts); the on-disk bundled loader,
/// counts, skill requirements and files are deferred.
#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CatalogTeam {
    pub id: String,
    pub key: String,
    pub name: String,
    pub description: String,
    pub category: String,
    pub kind: String,
    pub tags: Vec<String>,
}

/// The bundled catalog. In the TS server this is loaded from team-definition
/// files; here it is a small representative seed exercising the same API.
fn catalog() -> Vec<CatalogTeam> {
    vec![
        CatalogTeam {
            id: "cat-engineering".to_string(),
            key: "engineering".to_string(),
            name: "Engineering Team".to_string(),
            description: "Software engineering agents for building and shipping features."
                .to_string(),
            category: "engineering".to_string(),
            kind: "bundled".to_string(),
            tags: vec!["dev".to_string(), "build".to_string()],
        },
        CatalogTeam {
            id: "cat-research".to_string(),
            key: "research".to_string(),
            name: "Research Team".to_string(),
            description: "Research and analysis agents for deep investigation.".to_string(),
            category: "research".to_string(),
            kind: "optional".to_string(),
            tags: vec!["analysis".to_string()],
        },
    ]
}

/// Query filters for the catalog list.
#[derive(Deserialize, Default)]
pub struct CatalogFilter {
    #[serde(default)]
    pub kind: Option<String>,
    #[serde(default)]
    pub category: Option<String>,
    #[serde(default)]
    pub q: Option<String>,
}

/// A company's record of an installed catalog team.
#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InstalledTeam {
    pub company_id: String,
    pub catalog_id: String,
    pub name: String,
}

/// In-memory store of installed catalog teams per company.
#[derive(Clone, Default)]
pub struct InstalledTeamStore {
    inner: Arc<Mutex<Vec<InstalledTeam>>>,
}

impl InstalledTeamStore {
    pub fn install(&self, company_id: &str, team: &CatalogTeam) -> InstalledTeam {
        let installed = InstalledTeam {
            company_id: company_id.to_string(),
            catalog_id: team.id.clone(),
            name: team.name.clone(),
        };
        self.inner.lock().unwrap().push(installed.clone());
        installed
    }

    pub fn list(&self, company_id: &str) -> Vec<InstalledTeam> {
        self.inner
            .lock()
            .unwrap()
            .iter()
            .filter(|t| t.company_id == company_id)
            .cloned()
            .collect()
    }
}

pub trait InstalledTeamRepository {
    fn install(&self, company_id: &str, team: &CatalogTeam) -> InstalledTeam;
    fn list(&self, company_id: &str) -> Vec<InstalledTeam>;
}

impl InstalledTeamRepository for InstalledTeamStore {
    fn install(&self, company_id: &str, team: &CatalogTeam) -> InstalledTeam {
        InstalledTeamStore::install(self, company_id, team)
    }
    fn list(&self, company_id: &str) -> Vec<InstalledTeam> {
        InstalledTeamStore::list(self, company_id)
    }
}

/// Cloneable handle to whichever [`InstalledTeamRepository`] backs the app.
#[derive(Clone)]
pub struct TeamsCatalogRepo(pub Arc<dyn InstalledTeamRepository + Send + Sync>);

impl Default for TeamsCatalogRepo {
    fn default() -> Self {
        TeamsCatalogRepo(Arc::new(InstalledTeamStore::default()))
    }
}

/// `GET /api/teams/catalog` — the (filtered) bundled catalog. Requires an
/// authenticated actor (board or valid agent key).
pub async fn list_catalog(
    _actor: Actor,
    Query(filter): Query<CatalogFilter>,
) -> Json<Vec<CatalogTeam>> {
    let q = filter.q.map(|s| s.to_lowercase());
    let teams = catalog()
        .into_iter()
        .filter(|t| filter.kind.as_ref().is_none_or(|k| &t.kind == k))
        .filter(|t| filter.category.as_ref().is_none_or(|c| &t.category == c))
        .filter(|t| {
            q.as_ref().is_none_or(|needle| {
                format!("{} {}", t.name, t.description)
                    .to_lowercase()
                    .contains(needle)
            })
        })
        .collect();
    Json(teams)
}

pub async fn list_installed_teams(
    actor: Actor,
    State(repo): State<TeamsCatalogRepo>,
    Path(company_id): Path<String>,
) -> Result<Json<Vec<InstalledTeam>>, (StatusCode, Json<Value>)> {
    authorize_company_access(&actor, &company_id)?;
    Ok(Json(repo.0.list(&company_id)))
}

pub async fn install_catalog_team(
    actor: Actor,
    State(repo): State<TeamsCatalogRepo>,
    Path((company_id, catalog_id)): Path<(String, String)>,
) -> Result<(StatusCode, Json<InstalledTeam>), (StatusCode, Json<Value>)> {
    authorize_company_access(&actor, &company_id)?;
    let Some(team) = catalog().into_iter().find(|t| t.id == catalog_id) else {
        return Err((
            StatusCode::NOT_FOUND,
            Json(json!({ "error": "Catalog team not found" })),
        ));
    };
    Ok((
        StatusCode::CREATED,
        Json(repo.0.install(&company_id, &team)),
    ))
}
