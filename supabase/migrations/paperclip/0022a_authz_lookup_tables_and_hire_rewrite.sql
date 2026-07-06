-- Phase 01 authz wiring: canonical agent_role/adapter_type/icon/pause_reason lookup tables + RLS,
-- agents.role + agents.adapter_type FKs to lookups, agent_config_revisions audit table, rewrite
-- of hire_agent to also accept users holding the agents:create permission grant.
-- Phase 01 + 05: authz wiring + remaining agent schema + adapter/secret/skill writes.
-- Wires fn_has_permission into hire gates; adds canonical agent_role/adapter_type/icon enums
-- (as CHECK constraints + lookup tables for validation); secret + skill + adapter write RPCs.

-- ===== lookup tables for canonical enums (text + validated against these rows) =====
create table if not exists paperclip.agent_role_names (
  role text primary key,
  label text not null,
  created_at timestamptz not null default now()
);
insert into paperclip.agent_role_names(role, label) values
  ('general','General'), ('ceo','CEO'), ('cto','CTO'), ('cmo','CMO'), ('cfo','CFO'),
  ('engineer','Engineer'), ('analyst','Analyst'), ('researcher','Researcher'),
  ('designer','Designer'), ('writer','Writer'), ('sales','Sales'), ('support','Support')
on conflict (role) do update set label=excluded.label;

create table if not exists paperclip.agent_icon_names (
  icon text primary key,
  created_at timestamptz not null default now()
);
insert into paperclip.agent_icon_names(icon) values
  ('bot'),('brain'),('crown'),('settings'),('rocket'),('users'),('mail'),('code'),
  ('chart-bar'),('palette'),('message-circle'),('search'),('book-open'),('wrench')
on conflict (icon) do nothing;

create table if not exists paperclip.agent_adapter_types (
  adapter_type text primary key,
  label text not null,
  created_at timestamptz not null default now()
);
insert into paperclip.agent_adapter_types(adapter_type, label) values
  ('process','Process'),('http','HTTP'),('claude_local','Claude Local'),
  ('codex_local','Codex Local'),('gemini_local','Gemini Local'),
  ('cursor','Cursor'),('hermes_local','Hermes Local'),
  ('hermes_gateway','Hermes Gateway'),('openclaw_gateway','OpenClaw Gateway'),
  ('opencode_local','OpenCode Local'),('pi_local','Pi Local'),
  ('acpx_local','ACPX Local'),('openai','OpenAI (legacy)')
on conflict (adapter_type) do update set label=excluded.label;

create table if not exists paperclip.pause_reasons (
  reason text primary key,
  created_at timestamptz not null default now()
);
insert into paperclip.pause_reasons(reason) values
  ('manual'),('budget_exceeded'),('instance_maintenance'),('operator_request')
on conflict (reason) do nothing;

-- ===== lookup table grants =====
do $$
declare t text;
begin
  foreach t in array array['agent_role_names','agent_icon_names','agent_adapter_types','pause_reasons'] loop
    execute format('alter table paperclip.%I enable row level security', t);
    execute format('create policy %I on paperclip.%I for select to authenticated using (true)', t||'_read', t);
    execute format('grant select on paperclip.%I to authenticated', t);
  end loop;
end $$;

-- ===== agents: add icon, role enum FK, adapter_type FK (text validated against lookup) =====
alter table paperclip.agents add column if not exists icon text;
do $$ begin
  if not exists (select 1 from information_schema.table_constraints
                 where constraint_name='agents_role_fkey' and table_schema='paperclip' and table_name='agents') then
    alter table paperclip.agents add constraint agents_role_fkey foreign key (role) references paperclip.agent_role_names(role);
  end if;
  if not exists (select 1 from information_schema.table_constraints
                 where constraint_name='agents_adapter_type_fkey' and table_schema='paperclip' and table_name='agents') then
    alter table paperclip.agents add constraint agents_adapter_type_fkey foreign key (adapter_type) references paperclip.agent_adapter_types(adapter_type);
  end if;
end $$;

-- ===== agent_config_revisions table (audit trail for agent config changes) =====
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
alter table paperclip.agent_config_revisions enable row level security;
create policy agent_config_revisions_member_read on paperclip.agent_config_revisions for select to authenticated
  using (team_id in (select paperclip_private.fn_user_team_ids(auth.uid())));
grant select on paperclip.agent_config_revisions to authenticated;

-- ===== wire fn_has_permission into hire_agent session path =====
-- The existing hire_agent allows owner/admin/operator. Extend to also allow
-- users who hold the 'agents:create' permission grant for the team.
-- (Keep operator for backward compatibility; original had agents:create.)
create or replace function paperclip.hire_agent(
  p_company_id uuid, p_name text, p_role text default 'general',
  p_adapter_type text default 'openai', p_model text default 'gpt-5.4-mini',
  p_title text default null, p_reports_to uuid default null, p_can_create_agents boolean default false)
returns jsonb language plpgsql security definer set search_path='' as $$
declare v_uid uuid := auth.uid(); v_team uuid; v_require boolean; v_status paperclip.agent_status;
        v_is_board boolean; v_agent uuid; v_approval uuid; v_perms jsonb; v_role_ok boolean;
begin
  if v_uid is null then raise exception 'not authenticated' using errcode='28000'; end if;
  select team_id into v_team from paperclip.companies where id=p_company_id;
  if v_team is null then raise exception 'company not found' using errcode='42501'; end if;
  v_role_ok := paperclip_private.fn_has_team_role(v_team, v_uid, array['owner','admin','operator']::paperclip.team_role[]);
  if not v_role_ok and not paperclip_private.fn_has_permission(v_team, p_company_id, 'user', v_uid, 'agents:create') then
    raise exception 'insufficient role or agents:create grant' using errcode='42501'; end if;
  v_is_board := paperclip_private.fn_has_team_role(v_team, v_uid, array['owner','admin']::paperclip.team_role[]);
  if p_reports_to is not null and not exists(select 1 from paperclip.agents where id=p_reports_to and company_id=p_company_id) then
    raise exception 'reports_to must be an agent in the same company' using errcode='42501'; end if;
  v_perms := jsonb_build_object('can_create_agents', (p_can_create_agents and v_is_board));
  v_status := case when v_require then 'pending_approval' else 'idle' end;
  insert into paperclip.agents(company_id, team_id, name, role, title, reports_to, status, adapter_type, model, permissions, created_by)
    values (p_company_id, v_team, p_name, p_role, p_title, p_reports_to, v_status, p_adapter_type, p_model, v_perms, v_uid)
    returning id into v_agent;
  if v_require then
    insert into paperclip.approvals(company_id, team_id, type, status, subject_agent_id, payload, requested_by)
    values (p_company_id, v_team, 'hire_agent', 'pending', v_agent,
            jsonb_build_object('name',p_name,'role',p_role,'model',p_model), v_uid)
    returning id into v_approval;
  end if;
  perform paperclip_private.log_activity(v_team, p_company_id, 'user', v_uid::text, 'agent.hire_created', 'agent', v_agent::text);
  return jsonb_build_object('agentId', v_agent, 'status', v_status, 'approvalId', v_approval);
end $$;


revoke execute on function
  paperclip.validate_adapter_type(text) from public, anon;
grant execute on function
  paperclip.validate_adapter_type(text) to anon, authenticated;
