-- Phase 02/05/06/07: port V1 control-plane tables to the paperclip schema (unify on Supabase).
-- Sources: packages/db/src/schema/*.ts (original Drizzle). Conventions: team_id tenant boundary +
-- company_id business boundary; RLS via paperclip_private.fn_user_team_ids; grant select to authenticated.
-- Status enums are text (matching original Drizzle). Forward-only, idempotent.

-- ===== goals / projects / issues work backbone (Phase 02) =====
create table if not exists paperclip.goals (
  id uuid primary key default gen_random_uuid(),
  company_id uuid not null references paperclip.companies(id) on delete cascade,
  team_id uuid not null references paperclip.teams(id) on delete cascade,
  title text not null,
  description text,
  level text not null default 'task',
  status text not null default 'planned',
  parent_id uuid,
  owner_agent_id uuid references paperclip.agents(id) on delete set null,
  created_at timestamptz not null default now(),
  updated_at timestamptz not null default now()
);
create index if not exists goals_company_idx on paperclip.goals(company_id);

create table if not exists paperclip.projects (
  id uuid primary key default gen_random_uuid(),
  company_id uuid not null references paperclip.companies(id) on delete cascade,
  team_id uuid not null references paperclip.teams(id) on delete cascade,
  goal_id uuid,
  name text not null,
  description text,
  status text not null default 'backlog',
  lead_agent_id uuid references paperclip.agents(id) on delete set null,
  target_date date,
  color text,
  icon text,
  env jsonb not null default '{}'::jsonb,
  pause_reason text,
  paused_at timestamptz,
  archived_at timestamptz,
  created_at timestamptz not null default now(),
  updated_at timestamptz not null default now()
);
create index if not exists projects_company_idx on paperclip.projects(company_id);

create table if not exists paperclip.issues (
  id uuid primary key default gen_random_uuid(),
  company_id uuid not null references paperclip.companies(id) on delete cascade,
  team_id uuid not null references paperclip.teams(id) on delete cascade,
  project_id uuid,
  goal_id uuid,
  parent_id uuid,
  title text not null,
  description text,
  status text not null default 'backlog',
  work_mode text not null default 'standard',
  priority text not null default 'medium',
  assignee_agent_id uuid references paperclip.agents(id) on delete set null,
  assignee_user_id text,
  execution_locked_at timestamptz,
  issue_number integer,
  identifier text,
  origin_kind text not null default 'manual',
  billing_code text,
  execution_policy jsonb,
  execution_state jsonb,
  created_by_agent_id uuid references paperclip.agents(id) on delete set null,
  created_by_user_id text,
  created_at timestamptz not null default now(),
  updated_at timestamptz not null default now()
);
create index if not exists issues_company_status_idx on paperclip.issues(company_id, status);
create index if not exists issues_assignee_idx on paperclip.issues(assignee_agent_id);
create index if not exists issues_parent_idx on paperclip.issues(parent_id);

create table if not exists paperclip.issue_comments (
  id uuid primary key default gen_random_uuid(),
  issue_id uuid not null references paperclip.issues(id) on delete cascade,
  company_id uuid not null references paperclip.companies(id) on delete cascade,
  team_id uuid not null references paperclip.teams(id) on delete cascade,
  author_agent_id uuid references paperclip.agents(id) on delete set null,
  author_user_id text,
  body text not null,
  created_at timestamptz not null default now(),
  updated_at timestamptz not null default now()
);
create index if not exists issue_comments_issue_idx on paperclip.issue_comments(issue_id);

create table if not exists paperclip.issue_documents (
  id uuid primary key default gen_random_uuid(),
  issue_id uuid not null references paperclip.issues(id) on delete cascade,
  company_id uuid not null references paperclip.companies(id) on delete cascade,
  team_id uuid not null references paperclip.teams(id) on delete cascade,
  title text not null,
  body text,
  format text not null default 'markdown',
  author_agent_id uuid references paperclip.agents(id) on delete set null,
  author_user_id text,
  created_at timestamptz not null default now(),
  updated_at timestamptz not null default now()
);
create index if not exists issue_documents_issue_idx on paperclip.issue_documents(issue_id);

create table if not exists paperclip.issue_work_products (
  id uuid primary key default gen_random_uuid(),
  issue_id uuid not null references paperclip.issues(id) on delete cascade,
  company_id uuid not null references paperclip.companies(id) on delete cascade,
  team_id uuid not null references paperclip.teams(id) on delete cascade,
  name text not null,
  resource_ref jsonb not null,
  metadata jsonb not null default '{}'::jsonb,
  created_by_agent_id uuid references paperclip.agents(id) on delete set null,
  created_by_user_id text,
  created_at timestamptz not null default now()
);
create index if not exists issue_work_products_issue_idx on paperclip.issue_work_products(issue_id);

create table if not exists paperclip.issue_attachments (
  id uuid primary key default gen_random_uuid(),
  issue_id uuid not null references paperclip.issues(id) on delete cascade,
  company_id uuid not null references paperclip.companies(id) on delete cascade,
  team_id uuid not null references paperclip.teams(id) on delete cascade,
  filename text not null,
  storage_url text not null,
  content_type text,
  size_bytes integer,
  uploaded_by_agent_id uuid references paperclip.agents(id) on delete set null,
  uploaded_by_user_id text,
  created_at timestamptz not null default now()
);
create index if not exists issue_attachments_issue_idx on paperclip.issue_attachments(issue_id);

create table if not exists paperclip.issue_relations (
  id uuid primary key default gen_random_uuid(),
  company_id uuid not null references paperclip.companies(id) on delete cascade,
  team_id uuid not null references paperclip.teams(id) on delete cascade,
  source_issue_id uuid not null references paperclip.issues(id) on delete cascade,
  target_issue_id uuid not null references paperclip.issues(id) on delete cascade,
  relation text not null,
  created_at timestamptz not null default now()
);

-- ===== cost / budget / activity (Phase 06) =====
create table if not exists paperclip.cost_events (
  id uuid primary key default gen_random_uuid(),
  company_id uuid not null references paperclip.companies(id) on delete cascade,
  team_id uuid not null references paperclip.teams(id) on delete cascade,
  agent_id uuid references paperclip.agents(id) on delete set null,
  issue_id uuid,
  project_id uuid,
  run_id uuid,
  billing_code text,
  model text,
  input_tokens integer not null default 0,
  output_tokens integer not null default 0,
  total_tokens integer not null default 0,
  cost_cents integer not null default 0,
  occurred_at timestamptz not null default now(),
  created_at timestamptz not null default now()
);
create index if not exists cost_events_company_idx on paperclip.cost_events(company_id, occurred_at);

create table if not exists paperclip.budget_policies (
  id uuid primary key default gen_random_uuid(),
  company_id uuid not null references paperclip.companies(id) on delete cascade,
  team_id uuid not null references paperclip.teams(id) on delete cascade,
  scope_type text not null,
  scope_id uuid,
  monthly_limit_cents integer not null default 0,
  hard_stop boolean not null default true,
  created_at timestamptz not null default now(),
  updated_at timestamptz not null default now(),
  unique (company_id, scope_type, scope_id)
);

create table if not exists paperclip.budget_incidents (
  id uuid primary key default gen_random_uuid(),
  company_id uuid not null references paperclip.companies(id) on delete cascade,
  team_id uuid not null references paperclip.teams(id) on delete cascade,
  policy_id uuid references paperclip.budget_policies(id) on delete cascade,
  kind text not null,
  status text not null default 'open',
  details jsonb not null default '{}'::jsonb,
  created_at timestamptz not null default now(),
  resolved_at timestamptz
);

create table if not exists paperclip.activity_log (
  id uuid primary key default gen_random_uuid(),
  company_id uuid not null references paperclip.companies(id) on delete cascade,
  team_id uuid not null references paperclip.teams(id) on delete cascade,
  actor_type text not null,
  actor_id text,
  action text not null,
  target_type text,
  target_id text,
  details jsonb not null default '{}'::jsonb,
  created_at timestamptz not null default now()
);
create index if not exists activity_log_company_idx on paperclip.activity_log(company_id, created_at desc);

-- ===== agent runtime / heartbeat (Phase 07) =====
create table if not exists paperclip.agent_wakeup_requests (
  id uuid primary key default gen_random_uuid(),
  company_id uuid not null references paperclip.companies(id) on delete cascade,
  team_id uuid not null references paperclip.teams(id) on delete cascade,
  agent_id uuid not null references paperclip.agents(id) on delete cascade,
  status text not null default 'queued',
  trigger_detail text,
  requested_by_agent_id uuid references paperclip.agents(id) on delete set null,
  requested_by_user_id text,
  created_at timestamptz not null default now(),
  fulfilled_at timestamptz
);
create index if not exists wakeup_requests_agent_idx on paperclip.agent_wakeup_requests(agent_id, status);

create table if not exists paperclip.heartbeat_runs (
  id uuid primary key default gen_random_uuid(),
  company_id uuid not null references paperclip.companies(id) on delete cascade,
  team_id uuid not null references paperclip.teams(id) on delete cascade,
  agent_id uuid not null references paperclip.agents(id) on delete cascade,
  invocation_source text not null default 'on_demand',
  trigger_detail text,
  status text not null default 'queued',
  wakeup_request_id uuid references paperclip.agent_wakeup_requests(id) on delete set null,
  started_at timestamptz,
  finished_at timestamptz,
  error text,
  exit_code integer,
  signal text,
  usage_json jsonb,
  result_text text,
  created_at timestamptz not null default now()
);
create index if not exists heartbeat_runs_agent_status_idx on paperclip.heartbeat_runs(agent_id, status);
create index if not exists heartbeat_runs_company_status_idx on paperclip.heartbeat_runs(company_id, status);

create table if not exists paperclip.heartbeat_run_events (
  id uuid primary key default gen_random_uuid(),
  run_id uuid not null references paperclip.heartbeat_runs(id) on delete cascade,
  company_id uuid not null references paperclip.companies(id) on delete cascade,
  team_id uuid not null references paperclip.teams(id) on delete cascade,
  kind text not null,
  payload jsonb not null default '{}'::jsonb,
  created_at timestamptz not null default now()
);
create index if not exists heartbeat_run_events_run_idx on paperclip.heartbeat_run_events(run_id);

create table if not exists paperclip.agent_runtime_state (
  agent_id uuid primary key references paperclip.agents(id) on delete cascade,
  company_id uuid not null references paperclip.companies(id) on delete cascade,
  team_id uuid not null references paperclip.teams(id) on delete cascade,
  context_snapshot jsonb not null default '{}'::jsonb,
  updated_at timestamptz not null default now()
);

create table if not exists paperclip.agent_task_sessions (
  id uuid primary key default gen_random_uuid(),
  agent_id uuid not null references paperclip.agents(id) on delete cascade,
  company_id uuid not null references paperclip.companies(id) on delete cascade,
  team_id uuid not null references paperclip.teams(id) on delete cascade,
  task_key text not null,
  state jsonb not null default '{}'::jsonb,
  updated_at timestamptz not null default now(),
  unique (agent_id, task_key)
);

create table if not exists paperclip.agent_config_revisions (
  id uuid primary key default gen_random_uuid(),
  agent_id uuid not null references paperclip.agents(id) on delete cascade,
  company_id uuid not null references paperclip.companies(id) on delete cascade,
  team_id uuid not null references paperclip.teams(id) on delete cascade,
  before jsonb,
  after jsonb,
  changed_keys text[],
  source text,
  revised_by_user_id text,
  created_at timestamptz not null default now()
);
create index if not exists agent_config_revisions_agent_idx on paperclip.agent_config_revisions(agent_id, created_at desc);

-- ===== environments / routines / workspaces (Phase 03) =====
create table if not exists paperclip.environments (
  id uuid primary key default gen_random_uuid(),
  company_id uuid not null references paperclip.companies(id) on delete cascade,
  team_id uuid not null references paperclip.teams(id) on delete cascade,
  name text not null,
  driver text not null,
  status text not null default 'active',
  config jsonb not null default '{}'::jsonb,
  env_vars jsonb not null default '{}'::jsonb,
  metadata jsonb,
  created_at timestamptz not null default now(),
  updated_at timestamptz not null default now()
);
create index if not exists environments_company_idx on paperclip.environments(company_id);

-- link agents.default_environment_id now that environments exists (idempotent).
do $$ begin
  if not exists (select 1 from information_schema.table_constraints
                 where constraint_name='agents_default_environment_id_fkey'
                   and table_schema='paperclip' and table_name='agents') then
    alter table paperclip.agents add constraint agents_default_environment_id_fkey
      foreign key (default_environment_id) references paperclip.environments(id) on delete set null;
  end if;
end $$;

create table if not exists paperclip.routines (
  id uuid primary key default gen_random_uuid(),
  company_id uuid not null references paperclip.companies(id) on delete cascade,
  team_id uuid not null references paperclip.teams(id) on delete cascade,
  title text not null,
  status text not null default 'active',
  trigger jsonb not null default '{}'::jsonb,
  env jsonb not null default '{}'::jsonb,
  agent_id uuid references paperclip.agents(id) on delete set null,
  created_at timestamptz not null default now(),
  updated_at timestamptz not null default now()
);
create index if not exists routines_company_idx on paperclip.routines(company_id);

create table if not exists paperclip.execution_workspaces (
  id uuid primary key default gen_random_uuid(),
  company_id uuid not null references paperclip.companies(id) on delete cascade,
  team_id uuid not null references paperclip.teams(id) on delete cascade,
  project_id uuid,
  source_issue_id uuid,
  mode text not null,
  strategy_type text not null,
  name text not null,
  status text not null default 'active',
  cwd text,
  repo_url text,
  base_ref text,
  branch_name text,
  provider_type text not null default 'local_fs',
  provider_ref text,
  opened_at timestamptz not null default now(),
  closed_at timestamptz,
  metadata jsonb,
  created_at timestamptz not null default now(),
  updated_at timestamptz not null default now()
);
create index if not exists execution_workspaces_company_status_idx on paperclip.execution_workspaces(company_id, status);

create table if not exists paperclip.workspace_runtime_services (
  id uuid primary key default gen_random_uuid(),
  company_id uuid not null references paperclip.companies(id) on delete cascade,
  team_id uuid not null references paperclip.teams(id) on delete cascade,
  workspace_id uuid references paperclip.execution_workspaces(id) on delete cascade,
  kind text not null,
  status text not null default 'active',
  config jsonb not null default '{}'::jsonb,
  created_at timestamptz not null default now(),
  updated_at timestamptz not null default now()
);

-- ===== secrets (Phase 05) =====
create table if not exists paperclip.company_secret_provider_configs (
  id uuid primary key default gen_random_uuid(),
  company_id uuid not null references paperclip.companies(id) on delete cascade,
  team_id uuid not null references paperclip.teams(id) on delete cascade,
  provider text not null,
  config jsonb not null default '{}'::jsonb,
  status text not null default 'active',
  created_at timestamptz not null default now(),
  updated_at timestamptz not null default now()
);

create table if not exists paperclip.company_secrets (
  id uuid primary key default gen_random_uuid(),
  company_id uuid not null references paperclip.companies(id) on delete cascade,
  team_id uuid not null references paperclip.teams(id) on delete cascade,
  key text not null,
  name text not null,
  provider text not null default 'local_encrypted',
  status text not null default 'active',
  managed_mode text not null default 'paperclip_managed',
  external_ref text,
  provider_config_id uuid references paperclip.company_secret_provider_configs(id) on delete set null,
  provider_metadata jsonb,
  latest_version integer not null default 1,
  description text,
  last_rotated_at timestamptz,
  deleted_at timestamptz,
  created_by_agent_id uuid references paperclip.agents(id) on delete set null,
  created_by_user_id text,
  created_at timestamptz not null default now(),
  updated_at timestamptz not null default now(),
  unique (company_id, name)
);
create index if not exists company_secrets_company_idx on paperclip.company_secrets(company_id);

create table if not exists paperclip.company_secret_versions (
  id uuid primary key default gen_random_uuid(),
  secret_id uuid not null references paperclip.company_secrets(id) on delete cascade,
  company_id uuid not null references paperclip.companies(id) on delete cascade,
  team_id uuid not null references paperclip.teams(id) on delete cascade,
  version integer not null,
  ciphertext text,
  created_at timestamptz not null default now(),
  unique (secret_id, version)
);

create table if not exists paperclip.company_secret_bindings (
  id uuid primary key default gen_random_uuid(),
  secret_id uuid not null references paperclip.company_secrets(id) on delete cascade,
  company_id uuid not null references paperclip.companies(id) on delete cascade,
  team_id uuid not null references paperclip.teams(id) on delete cascade,
  target_type text not null,
  target_id uuid,
  config_path text not null,
  created_at timestamptz not null default now()
);

create table if not exists paperclip.secret_access_events (
  id uuid primary key default gen_random_uuid(),
  secret_id uuid not null references paperclip.company_secrets(id) on delete cascade,
  company_id uuid not null references paperclip.companies(id) on delete cascade,
  team_id uuid not null references paperclip.teams(id) on delete cascade,
  accessor_type text,
  accessor_id text,
  action text not null,
  created_at timestamptz not null default now()
);

-- ===== skills / pipelines / feedback (Phase 10/11) =====
create table if not exists paperclip.company_skills (
  id uuid primary key default gen_random_uuid(),
  company_id uuid not null references paperclip.companies(id) on delete cascade,
  team_id uuid not null references paperclip.teams(id) on delete cascade,
  slug text not null,
  name text not null,
  source text not null default 'builtin',
  enabled boolean not null default true,
  config jsonb not null default '{}'::jsonb,
  created_at timestamptz not null default now(),
  updated_at timestamptz not null default now(),
  unique (company_id, slug)
);

create table if not exists paperclip.pipelines (
  id uuid primary key default gen_random_uuid(),
  company_id uuid not null references paperclip.companies(id) on delete cascade,
  team_id uuid not null references paperclip.teams(id) on delete cascade,
  name text not null,
  status text not null default 'active',
  config jsonb not null default '{}'::jsonb,
  created_at timestamptz not null default now(),
  updated_at timestamptz not null default now()
);

create table if not exists paperclip.pipeline_cases (
  id uuid primary key default gen_random_uuid(),
  pipeline_id uuid not null references paperclip.pipelines(id) on delete cascade,
  company_id uuid not null references paperclip.companies(id) on delete cascade,
  team_id uuid not null references paperclip.teams(id) on delete cascade,
  status text not null default 'open',
  context jsonb not null default '{}'::jsonb,
  created_at timestamptz not null default now(),
  updated_at timestamptz not null default now()
);

create table if not exists paperclip.pipeline_case_events (
  id uuid primary key default gen_random_uuid(),
  case_id uuid not null references paperclip.pipeline_cases(id) on delete cascade,
  company_id uuid not null references paperclip.companies(id) on delete cascade,
  team_id uuid not null references paperclip.teams(id) on delete cascade,
  kind text not null,
  payload jsonb not null default '{}'::jsonb,
  created_at timestamptz not null default now()
);

create table if not exists paperclip.feedback_exports (
  id uuid primary key default gen_random_uuid(),
  company_id uuid not null references paperclip.companies(id) on delete cascade,
  team_id uuid not null references paperclip.teams(id) on delete cascade,
  target_type text not null,
  target_id text,
  body text not null,
  created_by_user_id text,
  created_at timestamptz not null default now()
);

create table if not exists paperclip.feedback_votes (
  id uuid primary key default gen_random_uuid(),
  company_id uuid not null references paperclip.companies(id) on delete cascade,
  team_id uuid not null references paperclip.teams(id) on delete cascade,
  target_type text not null,
  target_id text,
  voter_user_id text,
  vote integer not null default 1,
  created_at timestamptz not null default now(),
  unique (company_id, target_type, target_id, voter_user_id)
);

-- ===== RLS + grants for all new tables =====
do $$
declare t text;
begin
  foreach t in array array[
    'goals','projects','issues','issue_comments','issue_documents','issue_work_products',
    'issue_attachments','issue_relations','cost_events','budget_policies','budget_incidents',
    'activity_log','agent_wakeup_requests','heartbeat_runs','heartbeat_run_events',
    'agent_runtime_state','agent_task_sessions','agent_config_revisions','environments',
    'routines','execution_workspaces','workspace_runtime_services','company_secret_provider_configs',
    'company_secrets','company_secret_versions','company_secret_bindings','secret_access_events',
    'company_skills','pipelines','pipeline_cases','pipeline_case_events','feedback_exports',
    'feedback_votes'
  ] loop
    execute format('alter table paperclip.%I enable row level security', t);
    execute format('drop policy if exists %I on paperclip.%I', t||'_member_read', t);
    execute format('create policy %I on paperclip.%I for select to authenticated using (team_id in (select paperclip_private.fn_user_team_ids(auth.uid())))', t||'_member_read', t);
    execute format('grant select on paperclip.%I to authenticated', t);
  end loop;
end $$;

-- activity_log and secret tables: deny direct member writes (append-only / service-mediated).
-- Read access already granted above for transparency; writes go through SECURITY DEFINER RPCs.
