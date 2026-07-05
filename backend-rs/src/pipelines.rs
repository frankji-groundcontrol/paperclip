use std::sync::{Arc, Mutex};

use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use uuid::Uuid;

use crate::auth::{authorize_company_access, Actor};

/// A pipeline — the definition of a staged review/automation flow. Ports the
/// core pipeline entity of server/src/routes/pipelines.ts; the stage/case
/// machinery (and the list's aggregate counts) is deferred to later slices.
#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Pipeline {
    pub id: String,
    pub company_id: String,
    pub key: String,
    pub name: String,
    pub description: Option<String>,
    pub project_id: Option<String>,
    pub enforce_transitions: bool,
    pub archived: bool,
}

/// Ports the create-pipeline payload: `key` + `name` required; `enforceTransitions`
/// defaults to false; description/projectId nullable. (A `stages` array is
/// accepted but not yet materialised.)
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreatePipeline {
    pub key: String,
    pub name: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub project_id: Option<String>,
    #[serde(default)]
    pub enforce_transitions: bool,
}

/// Ports the partial update-pipeline payload.
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdatePipeline {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub enforce_transitions: Option<bool>,
    #[serde(default)]
    pub archived: Option<bool>,
}

/// In-memory, company-scoped pipeline repository.
#[derive(Clone, Default)]
pub struct PipelineStore {
    inner: Arc<Mutex<Vec<Pipeline>>>,
}

impl PipelineStore {
    pub fn create(&self, company_id: &str, input: CreatePipeline) -> Pipeline {
        let pipeline = Pipeline {
            id: Uuid::new_v4().to_string(),
            company_id: company_id.to_string(),
            key: input.key,
            name: input.name,
            description: input.description,
            project_id: input.project_id,
            enforce_transitions: input.enforce_transitions,
            archived: false,
        };
        self.inner.lock().unwrap().push(pipeline.clone());
        pipeline
    }

    pub fn list_by_company(&self, company_id: &str) -> Vec<Pipeline> {
        self.inner
            .lock()
            .unwrap()
            .iter()
            .filter(|p| p.company_id == company_id)
            .cloned()
            .collect()
    }

    pub fn get(&self, company_id: &str, pipeline_id: &str) -> Option<Pipeline> {
        self.inner
            .lock()
            .unwrap()
            .iter()
            .find(|p| p.company_id == company_id && p.id == pipeline_id)
            .cloned()
    }

    pub fn update(
        &self,
        company_id: &str,
        pipeline_id: &str,
        patch: UpdatePipeline,
    ) -> Option<Pipeline> {
        let mut guard = self.inner.lock().unwrap();
        let pipeline = guard
            .iter_mut()
            .find(|p| p.company_id == company_id && p.id == pipeline_id)?;
        if let Some(name) = patch.name {
            pipeline.name = name;
        }
        if let Some(description) = patch.description {
            pipeline.description = Some(description);
        }
        if let Some(enforce_transitions) = patch.enforce_transitions {
            pipeline.enforce_transitions = enforce_transitions;
        }
        if let Some(archived) = patch.archived {
            pipeline.archived = archived;
        }
        Some(pipeline.clone())
    }
}

pub trait PipelineRepository {
    fn list_by_company(&self, company_id: &str) -> Vec<Pipeline>;
    fn create(&self, company_id: &str, input: CreatePipeline) -> Pipeline;
    fn get(&self, company_id: &str, pipeline_id: &str) -> Option<Pipeline>;
    fn update(
        &self,
        company_id: &str,
        pipeline_id: &str,
        patch: UpdatePipeline,
    ) -> Option<Pipeline>;
}

impl PipelineRepository for PipelineStore {
    fn list_by_company(&self, company_id: &str) -> Vec<Pipeline> {
        PipelineStore::list_by_company(self, company_id)
    }
    fn create(&self, company_id: &str, input: CreatePipeline) -> Pipeline {
        PipelineStore::create(self, company_id, input)
    }
    fn get(&self, company_id: &str, pipeline_id: &str) -> Option<Pipeline> {
        PipelineStore::get(self, company_id, pipeline_id)
    }
    fn update(
        &self,
        company_id: &str,
        pipeline_id: &str,
        patch: UpdatePipeline,
    ) -> Option<Pipeline> {
        PipelineStore::update(self, company_id, pipeline_id, patch)
    }
}

/// Cloneable handle to whichever [`PipelineRepository`] backs the running app.
#[derive(Clone)]
pub struct PipelineRepo(pub Arc<dyn PipelineRepository + Send + Sync>);

impl Default for PipelineRepo {
    fn default() -> Self {
        PipelineRepo(Arc::new(PipelineStore::default()))
    }
}

fn pipeline_not_found() -> (StatusCode, Json<Value>) {
    (
        StatusCode::NOT_FOUND,
        Json(json!({ "error": "Pipeline not found" })),
    )
}

pub async fn list_pipelines(
    actor: Actor,
    State(repo): State<PipelineRepo>,
    Path(company_id): Path<String>,
) -> Result<Json<Vec<Pipeline>>, (StatusCode, Json<Value>)> {
    authorize_company_access(&actor, &company_id)?;
    Ok(Json(repo.0.list_by_company(&company_id)))
}

pub async fn create_pipeline(
    actor: Actor,
    State(repo): State<PipelineRepo>,
    Path(company_id): Path<String>,
    Json(input): Json<CreatePipeline>,
) -> Result<(StatusCode, Json<Pipeline>), (StatusCode, Json<Value>)> {
    authorize_company_access(&actor, &company_id)?;
    Ok((StatusCode::CREATED, Json(repo.0.create(&company_id, input))))
}

pub async fn get_pipeline(
    actor: Actor,
    State(repo): State<PipelineRepo>,
    Path((company_id, pipeline_id)): Path<(String, String)>,
) -> Result<Json<Pipeline>, (StatusCode, Json<Value>)> {
    authorize_company_access(&actor, &company_id)?;
    match repo.0.get(&company_id, &pipeline_id) {
        Some(pipeline) => Ok(Json(pipeline)),
        None => Err(pipeline_not_found()),
    }
}

pub async fn update_pipeline(
    actor: Actor,
    State(repo): State<PipelineRepo>,
    Path((company_id, pipeline_id)): Path<(String, String)>,
    Json(patch): Json<UpdatePipeline>,
) -> Result<Json<Pipeline>, (StatusCode, Json<Value>)> {
    authorize_company_access(&actor, &company_id)?;
    match repo.0.update(&company_id, &pipeline_id, patch) {
        Some(pipeline) => Ok(Json(pipeline)),
        None => Err(pipeline_not_found()),
    }
}

// --- Stages: the ordered steps within a pipeline ---

/// A pipeline stage. Ports the core stage columns; `config` is an opaque JSON
/// blob (server/src/routes/pipelines.ts stageConfigSchema).
#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Stage {
    pub id: String,
    pub pipeline_id: String,
    pub key: String,
    pub name: String,
    pub kind: String,
    pub position: i64,
    pub config: Value,
}

fn empty_object() -> Value {
    json!({})
}

/// Ports the create-stage payload: `key`+`name`+`kind` required; `position`
/// defaults to 0; `config` defaults to `{}`.
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateStage {
    pub key: String,
    pub name: String,
    pub kind: String,
    #[serde(default)]
    pub position: i64,
    #[serde(default = "empty_object")]
    pub config: Value,
}

/// Ports the partial update-stage payload.
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateStage {
    #[serde(default)]
    pub key: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub kind: Option<String>,
    #[serde(default)]
    pub position: Option<i64>,
    #[serde(default)]
    pub config: Option<Value>,
}

/// In-memory, pipeline-scoped stage repository.
#[derive(Clone, Default)]
pub struct StageStore {
    inner: Arc<Mutex<Vec<Stage>>>,
}

impl StageStore {
    pub fn create(&self, pipeline_id: &str, input: CreateStage) -> Stage {
        let stage = Stage {
            id: Uuid::new_v4().to_string(),
            pipeline_id: pipeline_id.to_string(),
            key: input.key,
            name: input.name,
            kind: input.kind,
            position: input.position,
            config: input.config,
        };
        self.inner.lock().unwrap().push(stage.clone());
        stage
    }

    pub fn list_by_pipeline(&self, pipeline_id: &str) -> Vec<Stage> {
        let mut stages: Vec<Stage> = self
            .inner
            .lock()
            .unwrap()
            .iter()
            .filter(|s| s.pipeline_id == pipeline_id)
            .cloned()
            .collect();
        stages.sort_by_key(|s| s.position);
        stages
    }

    pub fn update(&self, pipeline_id: &str, stage_id: &str, patch: UpdateStage) -> Option<Stage> {
        let mut guard = self.inner.lock().unwrap();
        let stage = guard
            .iter_mut()
            .find(|s| s.pipeline_id == pipeline_id && s.id == stage_id)?;
        if let Some(key) = patch.key {
            stage.key = key;
        }
        if let Some(name) = patch.name {
            stage.name = name;
        }
        if let Some(kind) = patch.kind {
            stage.kind = kind;
        }
        if let Some(position) = patch.position {
            stage.position = position;
        }
        if let Some(config) = patch.config {
            stage.config = config;
        }
        Some(stage.clone())
    }

    pub fn delete(&self, pipeline_id: &str, stage_id: &str) -> bool {
        let mut guard = self.inner.lock().unwrap();
        let before = guard.len();
        guard.retain(|s| !(s.pipeline_id == pipeline_id && s.id == stage_id));
        guard.len() != before
    }
}

pub trait StageRepository {
    fn list_by_pipeline(&self, pipeline_id: &str) -> Vec<Stage>;
    fn create(&self, pipeline_id: &str, input: CreateStage) -> Stage;
    fn update(&self, pipeline_id: &str, stage_id: &str, patch: UpdateStage) -> Option<Stage>;
    fn delete(&self, pipeline_id: &str, stage_id: &str) -> bool;
}

impl StageRepository for StageStore {
    fn list_by_pipeline(&self, pipeline_id: &str) -> Vec<Stage> {
        StageStore::list_by_pipeline(self, pipeline_id)
    }
    fn create(&self, pipeline_id: &str, input: CreateStage) -> Stage {
        StageStore::create(self, pipeline_id, input)
    }
    fn update(&self, pipeline_id: &str, stage_id: &str, patch: UpdateStage) -> Option<Stage> {
        StageStore::update(self, pipeline_id, stage_id, patch)
    }
    fn delete(&self, pipeline_id: &str, stage_id: &str) -> bool {
        StageStore::delete(self, pipeline_id, stage_id)
    }
}

/// Cloneable handle to whichever [`StageRepository`] backs the running app.
#[derive(Clone)]
pub struct StageRepo(pub Arc<dyn StageRepository + Send + Sync>);

impl Default for StageRepo {
    fn default() -> Self {
        StageRepo(Arc::new(StageStore::default()))
    }
}

fn stage_not_found() -> (StatusCode, Json<Value>) {
    (
        StatusCode::NOT_FOUND,
        Json(json!({ "error": "Stage not found" })),
    )
}

/// Ensures the pipeline exists within the company (else 404), guarding all stage
/// operations so a company can't reach another company's pipeline.
fn ensure_pipeline(
    pipelines: &PipelineRepo,
    company_id: &str,
    pipeline_id: &str,
) -> Result<(), (StatusCode, Json<Value>)> {
    if pipelines.0.get(company_id, pipeline_id).is_some() {
        Ok(())
    } else {
        Err(pipeline_not_found())
    }
}

pub async fn list_stages(
    actor: Actor,
    State(stages): State<StageRepo>,
    State(pipelines): State<PipelineRepo>,
    Path((company_id, pipeline_id)): Path<(String, String)>,
) -> Result<Json<Vec<Stage>>, (StatusCode, Json<Value>)> {
    authorize_company_access(&actor, &company_id)?;
    ensure_pipeline(&pipelines, &company_id, &pipeline_id)?;
    Ok(Json(stages.0.list_by_pipeline(&pipeline_id)))
}

pub async fn create_stage(
    actor: Actor,
    State(stages): State<StageRepo>,
    State(pipelines): State<PipelineRepo>,
    Path((company_id, pipeline_id)): Path<(String, String)>,
    Json(input): Json<CreateStage>,
) -> Result<(StatusCode, Json<Stage>), (StatusCode, Json<Value>)> {
    authorize_company_access(&actor, &company_id)?;
    ensure_pipeline(&pipelines, &company_id, &pipeline_id)?;
    Ok((
        StatusCode::CREATED,
        Json(stages.0.create(&pipeline_id, input)),
    ))
}

pub async fn update_stage(
    actor: Actor,
    State(stages): State<StageRepo>,
    State(pipelines): State<PipelineRepo>,
    Path((company_id, pipeline_id, stage_id)): Path<(String, String, String)>,
    Json(patch): Json<UpdateStage>,
) -> Result<Json<Stage>, (StatusCode, Json<Value>)> {
    authorize_company_access(&actor, &company_id)?;
    ensure_pipeline(&pipelines, &company_id, &pipeline_id)?;
    match stages.0.update(&pipeline_id, &stage_id, patch) {
        Some(stage) => Ok(Json(stage)),
        None => Err(stage_not_found()),
    }
}

pub async fn delete_stage(
    actor: Actor,
    State(stages): State<StageRepo>,
    State(pipelines): State<PipelineRepo>,
    Path((company_id, pipeline_id, stage_id)): Path<(String, String, String)>,
) -> Result<StatusCode, (StatusCode, Json<Value>)> {
    authorize_company_access(&actor, &company_id)?;
    ensure_pipeline(&pipelines, &company_id, &pipeline_id)?;
    if stages.0.delete(&pipeline_id, &stage_id) {
        Ok(StatusCode::NO_CONTENT)
    } else {
        Err(stage_not_found())
    }
}

// --- Cases: the runtime items flowing through a pipeline's stages ---

/// A pipeline case. Ports the core ingest columns; lifecycle machinery
/// (transitions, documents/revisions, leases, blockers) is deferred.
#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Case {
    pub id: String,
    pub pipeline_id: String,
    pub case_key: Option<String>,
    pub title: String,
    pub summary: Option<String>,
    pub fields: Value,
    pub stage_key: Option<String>,
    pub parent_case_id: Option<String>,
    /// Optimistic-concurrency version, bumped on each transition.
    pub version: i64,
}

/// Ports the transition payload core: move to `toStageKey`, guarded by
/// `expectedVersion` (optimistic concurrency).
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransitionCase {
    pub to_stage_key: String,
    pub expected_version: i64,
}

/// Outcome of a case transition attempt.
pub enum TransitionOutcome {
    NotFound,
    VersionConflict,
    Moved(Case),
}

/// Ports the ingest-case payload core: `title` required; `fields` defaults to
/// `{}`; the rest are nullable/optional. (requestKey/workspaceRef/blockedBy are
/// accepted but not yet materialised.)
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateCase {
    #[serde(default)]
    pub case_key: Option<String>,
    pub title: String,
    #[serde(default)]
    pub summary: Option<String>,
    #[serde(default = "empty_object")]
    pub fields: Value,
    #[serde(default)]
    pub stage_key: Option<String>,
    #[serde(default)]
    pub parent_case_id: Option<String>,
}

/// Ports the partial case-patch payload core.
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateCase {
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default)]
    pub summary: Option<String>,
    #[serde(default)]
    pub fields: Option<Value>,
    #[serde(default)]
    pub parent_case_id: Option<String>,
}

/// In-memory, pipeline-scoped case repository.
#[derive(Clone, Default)]
pub struct CaseStore {
    inner: Arc<Mutex<Vec<Case>>>,
}

impl CaseStore {
    pub fn create(&self, pipeline_id: &str, input: CreateCase) -> Case {
        let case = Case {
            id: Uuid::new_v4().to_string(),
            pipeline_id: pipeline_id.to_string(),
            case_key: input.case_key,
            title: input.title,
            summary: input.summary,
            fields: input.fields,
            stage_key: input.stage_key,
            parent_case_id: input.parent_case_id,
            version: 1,
        };
        self.inner.lock().unwrap().push(case.clone());
        case
    }

    /// Moves a case to `to_stage_key` iff `expected_version` matches, bumping the
    /// version. The caller is responsible for validating that the stage exists.
    pub fn transition(
        &self,
        pipeline_id: &str,
        case_id: &str,
        to_stage_key: &str,
        expected_version: i64,
    ) -> TransitionOutcome {
        let mut guard = self.inner.lock().unwrap();
        let Some(case) = guard
            .iter_mut()
            .find(|c| c.pipeline_id == pipeline_id && c.id == case_id)
        else {
            return TransitionOutcome::NotFound;
        };
        if case.version != expected_version {
            return TransitionOutcome::VersionConflict;
        }
        case.stage_key = Some(to_stage_key.to_string());
        case.version += 1;
        TransitionOutcome::Moved(case.clone())
    }

    pub fn list_by_pipeline(&self, pipeline_id: &str) -> Vec<Case> {
        self.inner
            .lock()
            .unwrap()
            .iter()
            .filter(|c| c.pipeline_id == pipeline_id)
            .cloned()
            .collect()
    }

    pub fn get(&self, pipeline_id: &str, case_id: &str) -> Option<Case> {
        self.inner
            .lock()
            .unwrap()
            .iter()
            .find(|c| c.pipeline_id == pipeline_id && c.id == case_id)
            .cloned()
    }

    pub fn update(&self, pipeline_id: &str, case_id: &str, patch: UpdateCase) -> Option<Case> {
        let mut guard = self.inner.lock().unwrap();
        let case = guard
            .iter_mut()
            .find(|c| c.pipeline_id == pipeline_id && c.id == case_id)?;
        if let Some(title) = patch.title {
            case.title = title;
        }
        if let Some(summary) = patch.summary {
            case.summary = Some(summary);
        }
        if let Some(fields) = patch.fields {
            case.fields = fields;
        }
        if let Some(parent_case_id) = patch.parent_case_id {
            case.parent_case_id = Some(parent_case_id);
        }
        Some(case.clone())
    }
}

pub trait CaseRepository {
    fn list_by_pipeline(&self, pipeline_id: &str) -> Vec<Case>;
    fn create(&self, pipeline_id: &str, input: CreateCase) -> Case;
    fn get(&self, pipeline_id: &str, case_id: &str) -> Option<Case>;
    fn update(&self, pipeline_id: &str, case_id: &str, patch: UpdateCase) -> Option<Case>;
    fn transition(
        &self,
        pipeline_id: &str,
        case_id: &str,
        to_stage_key: &str,
        expected_version: i64,
    ) -> TransitionOutcome;
}

impl CaseRepository for CaseStore {
    fn list_by_pipeline(&self, pipeline_id: &str) -> Vec<Case> {
        CaseStore::list_by_pipeline(self, pipeline_id)
    }
    fn create(&self, pipeline_id: &str, input: CreateCase) -> Case {
        CaseStore::create(self, pipeline_id, input)
    }
    fn get(&self, pipeline_id: &str, case_id: &str) -> Option<Case> {
        CaseStore::get(self, pipeline_id, case_id)
    }
    fn update(&self, pipeline_id: &str, case_id: &str, patch: UpdateCase) -> Option<Case> {
        CaseStore::update(self, pipeline_id, case_id, patch)
    }
    fn transition(
        &self,
        pipeline_id: &str,
        case_id: &str,
        to_stage_key: &str,
        expected_version: i64,
    ) -> TransitionOutcome {
        CaseStore::transition(self, pipeline_id, case_id, to_stage_key, expected_version)
    }
}

/// Cloneable handle to whichever [`CaseRepository`] backs the running app.
#[derive(Clone)]
pub struct CaseRepo(pub Arc<dyn CaseRepository + Send + Sync>);

impl Default for CaseRepo {
    fn default() -> Self {
        CaseRepo(Arc::new(CaseStore::default()))
    }
}

fn case_not_found() -> (StatusCode, Json<Value>) {
    (
        StatusCode::NOT_FOUND,
        Json(json!({ "error": "Case not found" })),
    )
}

pub async fn list_cases(
    actor: Actor,
    State(cases): State<CaseRepo>,
    State(pipelines): State<PipelineRepo>,
    Path((company_id, pipeline_id)): Path<(String, String)>,
) -> Result<Json<Vec<Case>>, (StatusCode, Json<Value>)> {
    authorize_company_access(&actor, &company_id)?;
    ensure_pipeline(&pipelines, &company_id, &pipeline_id)?;
    Ok(Json(cases.0.list_by_pipeline(&pipeline_id)))
}

pub async fn ingest_case(
    actor: Actor,
    State(cases): State<CaseRepo>,
    State(pipelines): State<PipelineRepo>,
    Path((company_id, pipeline_id)): Path<(String, String)>,
    Json(input): Json<CreateCase>,
) -> Result<(StatusCode, Json<Case>), (StatusCode, Json<Value>)> {
    authorize_company_access(&actor, &company_id)?;
    ensure_pipeline(&pipelines, &company_id, &pipeline_id)?;
    Ok((
        StatusCode::CREATED,
        Json(cases.0.create(&pipeline_id, input)),
    ))
}

pub async fn get_case(
    actor: Actor,
    State(cases): State<CaseRepo>,
    State(pipelines): State<PipelineRepo>,
    Path((company_id, pipeline_id, case_id)): Path<(String, String, String)>,
) -> Result<Json<Case>, (StatusCode, Json<Value>)> {
    authorize_company_access(&actor, &company_id)?;
    ensure_pipeline(&pipelines, &company_id, &pipeline_id)?;
    match cases.0.get(&pipeline_id, &case_id) {
        Some(case) => Ok(Json(case)),
        None => Err(case_not_found()),
    }
}

pub async fn update_case(
    actor: Actor,
    State(cases): State<CaseRepo>,
    State(pipelines): State<PipelineRepo>,
    Path((company_id, pipeline_id, case_id)): Path<(String, String, String)>,
    Json(patch): Json<UpdateCase>,
) -> Result<Json<Case>, (StatusCode, Json<Value>)> {
    authorize_company_access(&actor, &company_id)?;
    ensure_pipeline(&pipelines, &company_id, &pipeline_id)?;
    match cases.0.update(&pipeline_id, &case_id, patch) {
        Some(case) => Ok(Json(case)),
        None => Err(case_not_found()),
    }
}

pub async fn transition_case(
    actor: Actor,
    State(cases): State<CaseRepo>,
    State(stages): State<StageRepo>,
    State(pipelines): State<PipelineRepo>,
    Path((company_id, pipeline_id, case_id)): Path<(String, String, String)>,
    Json(input): Json<TransitionCase>,
) -> Result<Json<Case>, (StatusCode, Json<Value>)> {
    authorize_company_access(&actor, &company_id)?;
    ensure_pipeline(&pipelines, &company_id, &pipeline_id)?;

    // The target stage must exist in this pipeline.
    let stage_exists = stages
        .0
        .list_by_pipeline(&pipeline_id)
        .iter()
        .any(|s| s.key == input.to_stage_key);
    if !stage_exists {
        return Err(stage_not_found());
    }

    match cases.0.transition(
        &pipeline_id,
        &case_id,
        &input.to_stage_key,
        input.expected_version,
    ) {
        TransitionOutcome::Moved(case) => Ok(Json(case)),
        TransitionOutcome::NotFound => Err(case_not_found()),
        TransitionOutcome::VersionConflict => Err((
            StatusCode::CONFLICT,
            Json(json!({ "error": "Version conflict" })),
        )),
    }
}
