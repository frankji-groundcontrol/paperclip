use axum::{extract::FromRef, routing::get, Json, Router};
use serde_json::{json, Value};

pub mod activity;
pub mod adapters;
pub mod agents;
pub mod approvals;
pub mod auth;
pub mod board_keys;
pub mod budgets;
pub mod cloud_upstreams;
pub mod companies;
pub mod company_skills;
pub mod costs;
pub mod dashboard;
pub mod db;
pub mod environments;
pub mod goals;
pub mod inbox;
pub mod instance_settings;
pub mod invites;
pub mod issues;
pub mod join_requests;
pub mod llms;
pub mod mcp;
pub mod memberships;
pub mod pipelines;
pub mod plugins;
pub mod projects;
pub mod routines;
pub mod runs;
pub mod secrets;
pub mod sidebar;
pub mod sidebar_badges;
pub mod supabase;
pub mod teams_catalog;
pub mod user_profiles;
pub mod workspaces;

use activity::{create_activity, list_activity, ActivityRepo};
use adapters::{configure_adapter, delete_adapter, list_adapters, update_adapter, AdapterRepo};
use agents::{create_agent, delete_agent, list_agents, update_agent, AgentRepo};
use approvals::{create_approval, delete_approval, list_approvals, update_approval, ApprovalRepo};
use auth::{whoami, AgentKeyStore};
use board_keys::{create_board_api_key, delete_board_api_key, list_board_api_keys, BoardKeyRepo};
use budgets::{get_budget, record_budget_spend, set_budget_limit, BudgetRepo};
use cloud_upstreams::{
    delete_cloud_upstream, list_cloud_upstreams, register_cloud_upstream, CloudUpstreamRepo,
};
use companies::{
    create_company, delete_company, get_company, list_companies, update_company, CompanyRepo,
};
use company_skills::{
    create_skill, get_skill, list_skills, star_skill, unstar_skill, CompanySkillRepo,
};
use costs::{cost_summary, create_cost_event, CostRepo};
use dashboard::get_dashboard;
use environments::{
    create_environment, delete_environment, list_environments, update_environment, EnvironmentRepo,
};
use goals::{create_goal, delete_goal, list_goals, update_goal, GoalRepo};
use inbox::{create_inbox_dismissal, list_inbox_dismissals, InboxRepo};
use instance_settings::{
    get_instance_experimental, get_instance_general, get_instance_settings,
    patch_instance_experimental, patch_instance_general, patch_instance_settings,
    InstanceSettingsRepo,
};
use invites::{create_invite, list_invites, revoke_invite, InviteRepo};
use issues::{create_issue, delete_issue, list_issues, update_issue, IssueRepo};
use join_requests::{
    approve_join_request, create_join_request, list_join_requests, reject_join_request,
    JoinRequestRepo,
};
use llms::{agent_configuration_index, agent_icons};
use mcp::list_mcp_tools;
use memberships::{get_memberships, put_agent_membership, put_project_membership, MembershipRepo};
use pipelines::{
    create_pipeline, create_stage, delete_stage, get_case, get_pipeline, ingest_case, list_cases,
    list_pipelines, list_stages, transition_case, update_case, update_pipeline, update_stage,
    CaseRepo, PipelineRepo, StageRepo,
};
use plugins::{delete_plugin, install_plugin, list_plugins, update_plugin, PluginRepo};
use projects::{create_project, delete_project, list_projects, update_project, ProjectRepo};
use routines::{create_routine, delete_routine, list_routines, update_routine, RoutineRepo};
use runs::{create_run, delete_run, list_runs, update_run, RunRepo};
use secrets::{create_secret, delete_secret, list_secrets, SecretRepo};
use sidebar::{get_project_order, put_project_order, SidebarRepo};
use sidebar_badges::get_sidebar_badges;
use supabase::{broker::AuthBroker, routes::auth_routes};
use teams_catalog::{install_catalog_team, list_catalog, list_installed_teams, TeamsCatalogRepo};
use user_profiles::get_user_profile;
use workspaces::{
    create_workspace, delete_workspace, list_workspaces, update_workspace, WorkspaceRepo,
};

/// The set of domain repositories backing the app. Each is a cloneable handle to
/// an in-memory or SQLite-backed store; `FromRef` lets each handler extract just
/// the one it needs. Defaults are all in-memory.
#[derive(Clone, Default)]
pub struct Repositories {
    pub companies: CompanyRepo,
    pub issues: IssueRepo,
    pub projects: ProjectRepo,
    pub agents: AgentRepo,
    pub runs: RunRepo,
    pub approvals: ApprovalRepo,
    pub budgets: BudgetRepo,
    pub routines: RoutineRepo,
    pub secrets: SecretRepo,
    pub workspaces: WorkspaceRepo,
    pub plugins: PluginRepo,
    pub adapters: AdapterRepo,
    pub goals: GoalRepo,
    pub activity: ActivityRepo,
    pub costs: CostRepo,
    pub environments: EnvironmentRepo,
    pub inbox: InboxRepo,
    pub sidebar: SidebarRepo,
    pub memberships: MembershipRepo,
    pub instance_settings: InstanceSettingsRepo,
    pub pipelines: PipelineRepo,
    pub stages: StageRepo,
    pub cases: CaseRepo,
    pub board_keys: BoardKeyRepo,
    pub invites: InviteRepo,
    pub join_requests: JoinRequestRepo,
    pub teams_catalog: TeamsCatalogRepo,
    pub company_skills: CompanySkillRepo,
    pub cloud_upstreams: CloudUpstreamRepo,
    pub agent_keys: AgentKeyStore,
    pub supabase_auth: AuthBroker,
}

impl FromRef<Repositories> for CompanyRepo {
    fn from_ref(repos: &Repositories) -> Self {
        repos.companies.clone()
    }
}

impl FromRef<Repositories> for IssueRepo {
    fn from_ref(repos: &Repositories) -> Self {
        repos.issues.clone()
    }
}

impl FromRef<Repositories> for ProjectRepo {
    fn from_ref(repos: &Repositories) -> Self {
        repos.projects.clone()
    }
}

impl FromRef<Repositories> for AgentRepo {
    fn from_ref(repos: &Repositories) -> Self {
        repos.agents.clone()
    }
}

impl FromRef<Repositories> for RunRepo {
    fn from_ref(repos: &Repositories) -> Self {
        repos.runs.clone()
    }
}

impl FromRef<Repositories> for ApprovalRepo {
    fn from_ref(repos: &Repositories) -> Self {
        repos.approvals.clone()
    }
}

impl FromRef<Repositories> for BudgetRepo {
    fn from_ref(repos: &Repositories) -> Self {
        repos.budgets.clone()
    }
}

impl FromRef<Repositories> for RoutineRepo {
    fn from_ref(repos: &Repositories) -> Self {
        repos.routines.clone()
    }
}

impl FromRef<Repositories> for SecretRepo {
    fn from_ref(repos: &Repositories) -> Self {
        repos.secrets.clone()
    }
}

impl FromRef<Repositories> for WorkspaceRepo {
    fn from_ref(repos: &Repositories) -> Self {
        repos.workspaces.clone()
    }
}

impl FromRef<Repositories> for PluginRepo {
    fn from_ref(repos: &Repositories) -> Self {
        repos.plugins.clone()
    }
}

impl FromRef<Repositories> for AdapterRepo {
    fn from_ref(repos: &Repositories) -> Self {
        repos.adapters.clone()
    }
}

impl FromRef<Repositories> for GoalRepo {
    fn from_ref(repos: &Repositories) -> Self {
        repos.goals.clone()
    }
}

impl FromRef<Repositories> for ActivityRepo {
    fn from_ref(repos: &Repositories) -> Self {
        repos.activity.clone()
    }
}

impl FromRef<Repositories> for CostRepo {
    fn from_ref(repos: &Repositories) -> Self {
        repos.costs.clone()
    }
}

impl FromRef<Repositories> for EnvironmentRepo {
    fn from_ref(repos: &Repositories) -> Self {
        repos.environments.clone()
    }
}

impl FromRef<Repositories> for InboxRepo {
    fn from_ref(repos: &Repositories) -> Self {
        repos.inbox.clone()
    }
}

impl FromRef<Repositories> for SidebarRepo {
    fn from_ref(repos: &Repositories) -> Self {
        repos.sidebar.clone()
    }
}

impl FromRef<Repositories> for MembershipRepo {
    fn from_ref(repos: &Repositories) -> Self {
        repos.memberships.clone()
    }
}

impl FromRef<Repositories> for InstanceSettingsRepo {
    fn from_ref(repos: &Repositories) -> Self {
        repos.instance_settings.clone()
    }
}

impl FromRef<Repositories> for PipelineRepo {
    fn from_ref(repos: &Repositories) -> Self {
        repos.pipelines.clone()
    }
}

impl FromRef<Repositories> for StageRepo {
    fn from_ref(repos: &Repositories) -> Self {
        repos.stages.clone()
    }
}

impl FromRef<Repositories> for CaseRepo {
    fn from_ref(repos: &Repositories) -> Self {
        repos.cases.clone()
    }
}

impl FromRef<Repositories> for BoardKeyRepo {
    fn from_ref(repos: &Repositories) -> Self {
        repos.board_keys.clone()
    }
}

impl FromRef<Repositories> for InviteRepo {
    fn from_ref(repos: &Repositories) -> Self {
        repos.invites.clone()
    }
}

impl FromRef<Repositories> for JoinRequestRepo {
    fn from_ref(repos: &Repositories) -> Self {
        repos.join_requests.clone()
    }
}

impl FromRef<Repositories> for TeamsCatalogRepo {
    fn from_ref(repos: &Repositories) -> Self {
        repos.teams_catalog.clone()
    }
}

impl FromRef<Repositories> for CompanySkillRepo {
    fn from_ref(repos: &Repositories) -> Self {
        repos.company_skills.clone()
    }
}

impl FromRef<Repositories> for CloudUpstreamRepo {
    fn from_ref(repos: &Repositories) -> Self {
        repos.cloud_upstreams.clone()
    }
}

impl FromRef<Repositories> for AgentKeyStore {
    fn from_ref(repos: &Repositories) -> Self {
        repos.agent_keys.clone()
    }
}

impl FromRef<Repositories> for AuthBroker {
    fn from_ref(repos: &Repositories) -> Self {
        repos.supabase_auth.clone()
    }
}

/// Builds the router over a caller-supplied set of repositories.
pub fn app_with(repos: Repositories) -> Router {
    Router::new()
        .merge(auth_routes())
        .route("/api/health", get(health))
        .route("/api/whoami", get(whoami))
        .route("/api/mcp/tools", get(list_mcp_tools))
        .route(
            "/llms/agent-configuration.txt",
            get(agent_configuration_index),
        )
        .route("/llms/agent-icons.txt", get(agent_icons))
        .route("/api/cloud-upstreams", get(list_cloud_upstreams))
        .route(
            "/api/cloud-upstreams/register",
            axum::routing::post(register_cloud_upstream),
        )
        .route(
            "/api/cloud-upstreams/:connectionId",
            axum::routing::delete(delete_cloud_upstream),
        )
        .route("/api/teams/catalog", get(list_catalog))
        .route(
            "/api/companies/:companyId/teams/catalog/installed",
            get(list_installed_teams),
        )
        .route(
            "/api/companies/:companyId/teams/catalog/:catalogId/install",
            axum::routing::post(install_catalog_team),
        )
        .route(
            "/api/board-api-keys",
            get(list_board_api_keys).post(create_board_api_key),
        )
        .route(
            "/api/board-api-keys/:keyId",
            axum::routing::delete(delete_board_api_key),
        )
        .route(
            "/api/instance/settings",
            get(get_instance_settings).patch(patch_instance_settings),
        )
        .route(
            "/api/instance/settings/general",
            get(get_instance_general).patch(patch_instance_general),
        )
        .route(
            "/api/instance/settings/experimental",
            get(get_instance_experimental).patch(patch_instance_experimental),
        )
        .route("/api/companies", get(list_companies).post(create_company))
        .route(
            "/api/companies/:companyId",
            get(get_company)
                .patch(update_company)
                .delete(delete_company),
        )
        .route(
            "/api/companies/:companyId/issues",
            get(list_issues).post(create_issue),
        )
        .route(
            "/api/companies/:companyId/issues/:issueId",
            axum::routing::patch(update_issue).delete(delete_issue),
        )
        .route(
            "/api/companies/:companyId/projects",
            get(list_projects).post(create_project),
        )
        .route(
            "/api/companies/:companyId/projects/:projectId",
            axum::routing::patch(update_project).delete(delete_project),
        )
        .route(
            "/api/companies/:companyId/agents",
            get(list_agents).post(create_agent),
        )
        .route(
            "/api/companies/:companyId/agents/:agentId",
            axum::routing::patch(update_agent).delete(delete_agent),
        )
        .route(
            "/api/companies/:companyId/runs",
            get(list_runs).post(create_run),
        )
        .route(
            "/api/companies/:companyId/runs/:runId",
            axum::routing::patch(update_run).delete(delete_run),
        )
        .route(
            "/api/companies/:companyId/approvals",
            get(list_approvals).post(create_approval),
        )
        .route(
            "/api/companies/:companyId/approvals/:approvalId",
            axum::routing::patch(update_approval).delete(delete_approval),
        )
        .route(
            "/api/companies/:companyId/routines",
            get(list_routines).post(create_routine),
        )
        .route(
            "/api/companies/:companyId/routines/:routineId",
            axum::routing::patch(update_routine).delete(delete_routine),
        )
        .route(
            "/api/companies/:companyId/secrets",
            get(list_secrets).post(create_secret),
        )
        .route(
            "/api/companies/:companyId/secrets/:secretId",
            axum::routing::delete(delete_secret),
        )
        .route(
            "/api/companies/:companyId/workspaces",
            get(list_workspaces).post(create_workspace),
        )
        .route(
            "/api/companies/:companyId/workspaces/:workspaceId",
            axum::routing::patch(update_workspace).delete(delete_workspace),
        )
        .route(
            "/api/companies/:companyId/plugins",
            get(list_plugins).post(install_plugin),
        )
        .route(
            "/api/companies/:companyId/plugins/:pluginId",
            axum::routing::patch(update_plugin).delete(delete_plugin),
        )
        .route(
            "/api/companies/:companyId/adapters",
            get(list_adapters).post(configure_adapter),
        )
        .route(
            "/api/companies/:companyId/adapters/:adapterId",
            axum::routing::patch(update_adapter).delete(delete_adapter),
        )
        .route(
            "/api/companies/:companyId/goals",
            get(list_goals).post(create_goal),
        )
        .route(
            "/api/companies/:companyId/goals/:goalId",
            axum::routing::patch(update_goal).delete(delete_goal),
        )
        .route(
            "/api/companies/:companyId/activity",
            get(list_activity).post(create_activity),
        )
        .route("/api/companies/:companyId/dashboard", get(get_dashboard))
        .route(
            "/api/companies/:companyId/users/:userId/profile",
            get(get_user_profile),
        )
        .route(
            "/api/companies/:companyId/sidebar-badges",
            get(get_sidebar_badges),
        )
        .route(
            "/api/companies/:companyId/skills",
            get(list_skills).post(create_skill),
        )
        .route("/api/companies/:companyId/skills/:skillId", get(get_skill))
        .route(
            "/api/companies/:companyId/skills/:skillId/star",
            axum::routing::post(star_skill).delete(unstar_skill),
        )
        .route(
            "/api/companies/:companyId/invites",
            get(list_invites).post(create_invite),
        )
        .route(
            "/api/companies/:companyId/invites/:inviteId/revoke",
            axum::routing::post(revoke_invite),
        )
        .route(
            "/api/companies/:companyId/join-requests",
            get(list_join_requests).post(create_join_request),
        )
        .route(
            "/api/companies/:companyId/join-requests/:requestId/approve",
            axum::routing::post(approve_join_request),
        )
        .route(
            "/api/companies/:companyId/join-requests/:requestId/reject",
            axum::routing::post(reject_join_request),
        )
        .route(
            "/api/companies/:companyId/pipelines",
            get(list_pipelines).post(create_pipeline),
        )
        .route(
            "/api/companies/:companyId/pipelines/:pipelineId",
            get(get_pipeline).patch(update_pipeline),
        )
        .route(
            "/api/companies/:companyId/pipelines/:pipelineId/stages",
            get(list_stages).post(create_stage),
        )
        .route(
            "/api/companies/:companyId/pipelines/:pipelineId/stages/:stageId",
            axum::routing::patch(update_stage).delete(delete_stage),
        )
        .route(
            "/api/companies/:companyId/pipelines/:pipelineId/cases",
            get(list_cases).post(ingest_case),
        )
        .route(
            "/api/companies/:companyId/pipelines/:pipelineId/cases/:caseId",
            get(get_case).patch(update_case),
        )
        .route(
            "/api/companies/:companyId/pipelines/:pipelineId/cases/:caseId/transition",
            axum::routing::post(transition_case),
        )
        .route(
            "/api/companies/:companyId/cost-events",
            axum::routing::post(create_cost_event),
        )
        .route("/api/companies/:companyId/costs/summary", get(cost_summary))
        .route(
            "/api/companies/:companyId/environments",
            get(list_environments).post(create_environment),
        )
        .route(
            "/api/companies/:companyId/environments/:environmentId",
            axum::routing::patch(update_environment).delete(delete_environment),
        )
        .route(
            "/api/companies/:companyId/inbox-dismissals",
            get(list_inbox_dismissals).post(create_inbox_dismissal),
        )
        .route(
            "/api/companies/:companyId/sidebar-preferences/me",
            get(get_project_order).put(put_project_order),
        )
        .route(
            "/api/companies/:companyId/resource-memberships/me",
            get(get_memberships),
        )
        .route(
            "/api/companies/:companyId/resource-memberships/me/projects/:projectId",
            axum::routing::put(put_project_membership),
        )
        .route(
            "/api/companies/:companyId/resource-memberships/me/agents/:agentId",
            axum::routing::put(put_agent_membership),
        )
        .route("/api/companies/:companyId/budget", get(get_budget))
        .route(
            "/api/companies/:companyId/budget/limit",
            axum::routing::post(set_budget_limit),
        )
        .route(
            "/api/companies/:companyId/budget/spend",
            axum::routing::post(record_budget_spend),
        )
        .with_state(repos)
}

/// Builds the router with a caller-supplied company repository; other domains
/// stay in-memory.
pub fn app_with_company_repo(companies: CompanyRepo) -> Router {
    app_with(Repositories {
        companies,
        ..Default::default()
    })
}

/// Builds the router with caller-supplied company and issue repositories.
pub fn app_with_repos(companies: CompanyRepo, issues: IssueRepo) -> Router {
    app_with(Repositories {
        companies,
        issues,
        ..Default::default()
    })
}

/// Builds the application router with default in-memory stores.
///
/// Routes are added slice-by-slice as the Express `server/` surface is ported.
pub fn app() -> Router {
    app_with(Repositories::default())
}

/// Server version, mirroring `server/src/version.ts` (`serverVersion`).
/// Overridable via `PAPERCLIP_VERSION`; falls back to the crate version.
fn server_version() -> String {
    std::env::var("PAPERCLIP_VERSION")
        .ok()
        .filter(|v| !v.trim().is_empty())
        .unwrap_or_else(|| env!("CARGO_PKG_VERSION").to_string())
}

/// `GET /api/health` — ports the Express health route (server/src/routes/health.ts).
/// Default (non-authenticated) response carries `status: "ok"` and `version`.
/// Further detail (deploymentMode, serverInfo, bootstrapStatus) lands in later slices.
async fn health() -> Json<Value> {
    Json(json!({
        "status": "ok",
        "version": server_version(),
    }))
}
