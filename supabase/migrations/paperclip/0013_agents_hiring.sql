-- Agent hiring parity: agents (employees) hired via approval-gated governance; a hired agent does jobs
-- attributed to it. Governance fixes folded from the hiring plan-eng-review (board-only approve on the
-- human minter's role, agents may never decide, can_create_agents enforced, terminal/idempotent decide,
-- payload cannot inject permissions, limbo enforced before idempotency).

alter table paperclip.companies
  add column if not exists require_board_approval_for_new_agents boolean not null default true;

do $$ begin
  if not exists (select 1 from pg_type t join pg_namespace n on n.oid=t.typnamespace where n.nspname='paperclip' and t.typname='agent_status') then
    create type paperclip.agent_status as enum ('pending_approval','active','paused','archived');
  end if;
  if not exists (select 1 from pg_type t join pg_namespace n on n.oid=t.typnamespace where n.nspname='paperclip' and t.typname='approval_type') then
    create type paperclip.approval_type as enum ('hire_agent','approve_ceo_strategy');
  end if;
  if not exists (select 1 from pg_type t join pg_namespace n on n.oid=t.typnamespace where n.nspname='paperclip' and t.typname='approval_status') then
    create type paperclip.approval_status as enum ('pending','approved','rejected','cancelled');
  end if;
end $$;

create table if not exists paperclip.agents (
  id           uuid primary key default gen_random_uuid(),
  company_id   uuid not null references paperclip.companies(id) on delete cascade,
  team_id      uuid not null references paperclip.teams(id) on delete cascade,
  name         text not null,
  role         text not null default 'general',
  title        text,
  reports_to   uuid references paperclip.agents(id) on delete set null,
  status       paperclip.agent_status not null default 'pending_approval',
  adapter_type text not null default 'openai',
  model        text not null default 'gpt-5.4-mini',
  permissions  jsonb not null default '{"can_create_agents": false}'::jsonb,
  capabilities jsonb not null default '[]'::jsonb,
  created_by   uuid references paperclip.users(id) on delete set null,
  created_at   timestamptz not null default now(),
  updated_at   timestamptz not null default now(),
  meta         jsonb not null default '{}'::jsonb
);
create index if not exists agents_company_idx on paperclip.agents(company_id);
create index if not exists agents_team_idx on paperclip.agents(team_id);
drop trigger if exists trg_agents_touch on paperclip.agents;
create trigger trg_agents_touch before update on paperclip.agents for each row execute function paperclip_private.fn_touch_updated_at();

create table if not exists paperclip.approvals (
  id               uuid primary key default gen_random_uuid(),
  company_id       uuid not null references paperclip.companies(id) on delete cascade,
  team_id          uuid not null references paperclip.teams(id) on delete cascade,
  type             paperclip.approval_type not null,
  status           paperclip.approval_status not null default 'pending',
  subject_agent_id uuid references paperclip.agents(id) on delete cascade,
  payload          jsonb not null default '{}'::jsonb,
  requested_by     uuid references paperclip.users(id) on delete set null,
  decided_by       uuid references paperclip.users(id) on delete set null,
  decided_at       timestamptz,
  created_at       timestamptz not null default now(),
  meta             jsonb not null default '{}'::jsonb
);
create index if not exists approvals_team_idx on paperclip.approvals(team_id);
create index if not exists approvals_company_idx on paperclip.approvals(company_id);

alter table paperclip.agents    enable row level security;
alter table paperclip.approvals enable row level security;
drop policy if exists agents_member_read on paperclip.agents;
create policy agents_member_read on paperclip.agents for select to authenticated
  using ( team_id in (select paperclip_private.fn_user_team_ids(auth.uid())) );
drop policy if exists approvals_member_read on paperclip.approvals;
create policy approvals_member_read on paperclip.approvals for select to authenticated
  using ( team_id in (select paperclip_private.fn_user_team_ids(auth.uid())) );

create or replace view paperclip.my_agents with (security_invoker=true) as
  select a.* from paperclip.agents a where a.team_id in (select paperclip_private.fn_user_team_ids(auth.uid()));
create or replace view paperclip.my_approvals with (security_invoker=true) as
  select p.* from paperclip.approvals p where p.team_id in (select paperclip_private.fn_user_team_ids(auth.uid()));
grant select on paperclip.agents, paperclip.approvals to authenticated;
grant select on paperclip.my_agents, paperclip.my_approvals to authenticated;

-- private helper: is an agent active in a team?
create or replace function paperclip_private.fn_agent_active(p_agent uuid, p_team uuid)
returns boolean language sql stable security definer set search_path='' as $$
  select exists (select 1 from paperclip.agents where id = p_agent and team_id = p_team and status = 'active')
$$;
revoke execute on function paperclip_private.fn_agent_active(uuid,uuid) from public, anon, authenticated;

-- ===== hire_agent (session) =====
create or replace function paperclip.hire_agent(
  p_company_id uuid, p_name text, p_role text default 'general',
  p_adapter_type text default 'openai', p_model text default 'gpt-5.4-mini',
  p_title text default null, p_reports_to uuid default null, p_can_create_agents boolean default false)
returns jsonb language plpgsql security definer set search_path='' as $$
declare v_uid uuid := auth.uid(); v_team uuid; v_require boolean; v_status paperclip.agent_status;
        v_is_board boolean; v_agent uuid; v_approval uuid; v_perms jsonb;
begin
  if v_uid is null then raise exception 'not authenticated' using errcode='28000'; end if;
  select team_id, require_board_approval_for_new_agents into v_team, v_require from paperclip.companies where id=p_company_id;
  if v_team is null then raise exception 'company not found' using errcode='42501'; end if;
  if not paperclip_private.fn_has_team_role(v_team, v_uid, array['owner','admin','operator']::paperclip.team_role[]) then
    raise exception 'insufficient role to hire' using errcode='42501'; end if;
  v_is_board := paperclip_private.fn_has_team_role(v_team, v_uid, array['owner','admin']::paperclip.team_role[]);
  if p_reports_to is not null and not exists(select 1 from paperclip.agents where id=p_reports_to and company_id=p_company_id) then
    raise exception 'reports_to must be an agent in the same company' using errcode='42501'; end if;
  -- only the board may grant can_create_agents; payload cannot self-propagate permission
  v_perms := jsonb_build_object('can_create_agents', (p_can_create_agents and v_is_board));
  v_status := case when v_require then 'pending_approval' else 'active' end;
  insert into paperclip.agents(company_id, team_id, name, role, title, reports_to, status, adapter_type, model, permissions, created_by)
  values (p_company_id, v_team, p_name, coalesce(p_role,'general'), p_title, p_reports_to, v_status, coalesce(p_adapter_type,'openai'), coalesce(p_model,'gpt-5.4-mini'), v_perms, v_uid)
  returning id into v_agent;
  if v_require then
    insert into paperclip.approvals(company_id, team_id, type, status, subject_agent_id, payload, requested_by)
    values (p_company_id, v_team, 'hire_agent', 'pending', v_agent, jsonb_build_object('name',p_name,'role',p_role,'model',p_model), v_uid)
    returning id into v_approval;
  end if;
  return jsonb_build_object('agentId', v_agent, 'status', v_status, 'approvalId', v_approval);
end $$;

-- ===== hire_agent_with_key (api-key) =====
create or replace function paperclip.hire_agent_with_key(
  p_prefix text, p_key_hash text, p_company_id uuid, p_name text, p_role text default 'general',
  p_adapter_type text default 'openai', p_model text default 'gpt-5.4-mini', p_title text default null, p_reports_to uuid default null)
returns jsonb language plpgsql security definer set search_path='' as $$
declare k record; v_team uuid; v_require boolean; v_status paperclip.agent_status; v_agent uuid; v_approval uuid;
begin
  select * into k from paperclip_private.fn_key_principal(p_prefix, p_key_hash);
  if k.team_id is null then raise exception 'invalid api key' using errcode='28000'; end if;
  if not paperclip_private.fn_key_allows(k.scopes, 'agents:write') then
    raise exception 'api key lacks scope agents:write' using errcode='42501'; end if;
  -- Authorization: agent keys need an active acting agent WITH can_create_agents; user keys need the
  -- minter to hold an org role (board/operator). New hires never get can_create_agents via a key.
  if k.subject_type = 'agent' then
    if not paperclip_private.fn_agent_active(k.agent_id, k.team_id) then
      raise exception 'acting agent is not active' using errcode='42501'; end if;
    if not exists(select 1 from paperclip.agents where id=k.agent_id and (permissions->>'can_create_agents')::boolean is true) then
      raise exception 'acting agent lacks can_create_agents' using errcode='42501'; end if;
  else
    if not paperclip_private.fn_has_team_role(k.team_id, k.created_by, array['owner','admin','operator']::paperclip.team_role[]) then
      raise exception 'key minter lacks role to hire' using errcode='42501'; end if;
  end if;
  select team_id, require_board_approval_for_new_agents into v_team, v_require from paperclip.companies where id=p_company_id;
  if v_team is null or v_team <> k.team_id then raise exception 'company not in key team' using errcode='42501'; end if;
  if p_reports_to is not null and not exists(select 1 from paperclip.agents where id=p_reports_to and company_id=p_company_id) then
    raise exception 'reports_to must be an agent in the same company' using errcode='42501'; end if;
  v_status := case when v_require then 'pending_approval' else 'active' end;
  insert into paperclip.agents(company_id, team_id, name, role, title, reports_to, status, adapter_type, model, permissions, created_by)
  values (p_company_id, k.team_id, p_name, coalesce(p_role,'general'), p_title, p_reports_to, v_status, coalesce(p_adapter_type,'openai'), coalesce(p_model,'gpt-5.4-mini'), '{"can_create_agents": false}'::jsonb, k.created_by)
  returning id into v_agent;
  if v_require then
    insert into paperclip.approvals(company_id, team_id, type, status, subject_agent_id, payload, requested_by)
    values (p_company_id, k.team_id, 'hire_agent', 'pending', v_agent, jsonb_build_object('name',p_name,'role',p_role,'model',p_model), k.created_by)
    returning id into v_approval;
  end if;
  return jsonb_build_object('agentId', v_agent, 'status', v_status, 'approvalId', v_approval);
end $$;

-- ===== decide_approval (session; board only) =====
create or replace function paperclip.decide_approval(p_approval_id uuid, p_approve boolean)
returns paperclip.approval_status language plpgsql security definer set search_path='' as $$
declare v_uid uuid := auth.uid(); v_ap paperclip.approvals%rowtype;
begin
  if v_uid is null then raise exception 'not authenticated' using errcode='28000'; end if;
  select * into v_ap from paperclip.approvals where id=p_approval_id for update;
  if v_ap.id is null then raise exception 'approval not found' using errcode='P0002'; end if;
  if not paperclip_private.fn_has_team_role(v_ap.team_id, v_uid, array['owner','admin']::paperclip.team_role[]) then
    raise exception 'board approval required' using errcode='42501'; end if;
  if v_ap.status <> 'pending' then raise exception 'approval is not pending' using errcode='22023'; end if;
  if p_approve then
    if v_ap.type='hire_agent' and v_ap.subject_agent_id is not null then
      update paperclip.agents set status='active' where id=v_ap.subject_agent_id and status='pending_approval';
    end if;
    update paperclip.approvals set status='approved', decided_by=v_uid, decided_at=now() where id=p_approval_id;
    return 'approved';
  else
    if v_ap.type='hire_agent' and v_ap.subject_agent_id is not null then
      update paperclip.agents set status='archived' where id=v_ap.subject_agent_id and status='pending_approval';
    end if;
    update paperclip.approvals set status='rejected', decided_by=v_uid, decided_at=now() where id=p_approval_id;
    return 'rejected';
  end if;
end $$;

-- ===== decide_approval_with_key (api-key; board human minter only; agents may NEVER decide) =====
create or replace function paperclip.decide_approval_with_key(p_prefix text, p_key_hash text, p_approval_id uuid, p_approve boolean)
returns paperclip.approval_status language plpgsql security definer set search_path='' as $$
declare k record; v_ap paperclip.approvals%rowtype;
begin
  select * into k from paperclip_private.fn_key_principal(p_prefix, p_key_hash);
  if k.team_id is null then raise exception 'invalid api key' using errcode='28000'; end if;
  if k.subject_type = 'agent' then raise exception 'agents may not decide approvals' using errcode='42501'; end if;
  select * into v_ap from paperclip.approvals where id=p_approval_id for update;
  if v_ap.id is null then raise exception 'approval not found' using errcode='P0002'; end if;
  if v_ap.team_id <> k.team_id then raise exception 'approval not in key team' using errcode='42501'; end if;
  if not paperclip_private.fn_has_team_role(v_ap.team_id, k.created_by, array['owner','admin']::paperclip.team_role[]) then
    raise exception 'board approval required' using errcode='42501'; end if;
  if v_ap.status <> 'pending' then raise exception 'approval is not pending' using errcode='22023'; end if;
  if p_approve then
    if v_ap.type='hire_agent' and v_ap.subject_agent_id is not null then
      update paperclip.agents set status='active' where id=v_ap.subject_agent_id and status='pending_approval';
    end if;
    update paperclip.approvals set status='approved', decided_by=k.created_by, decided_at=now() where id=p_approval_id;
    return 'approved';
  else
    if v_ap.type='hire_agent' and v_ap.subject_agent_id is not null then
      update paperclip.agents set status='archived' where id=v_ap.subject_agent_id and status='pending_approval';
    end if;
    update paperclip.approvals set status='rejected', decided_by=k.created_by, decided_at=now() where id=p_approval_id;
    return 'rejected';
  end if;
end $$;

-- ===== list (api-key path) =====
create or replace function paperclip.list_agents_with_key(p_prefix text, p_key_hash text, p_company_id uuid default null)
returns setof paperclip.agents language plpgsql security definer set search_path='' as $$
declare k record; begin
  select * into k from paperclip_private.fn_key_principal(p_prefix, p_key_hash);
  if k.team_id is null then raise exception 'invalid api key' using errcode='28000'; end if;
  if not paperclip_private.fn_key_allows(k.scopes, 'agents:read') then raise exception 'api key lacks scope agents:read' using errcode='42501'; end if;
  return query select * from paperclip.agents where team_id=k.team_id and (p_company_id is null or company_id=p_company_id) order by created_at desc;
end $$;

create or replace function paperclip.list_approvals_with_key(p_prefix text, p_key_hash text, p_company_id uuid default null)
returns setof paperclip.approvals language plpgsql security definer set search_path='' as $$
declare k record; begin
  select * into k from paperclip_private.fn_key_principal(p_prefix, p_key_hash);
  if k.team_id is null then raise exception 'invalid api key' using errcode='28000'; end if;
  if not paperclip_private.fn_key_allows(k.scopes, 'agents:read') then raise exception 'api key lacks scope agents:read' using errcode='42501'; end if;
  return query select * from paperclip.approvals where team_id=k.team_id and (p_company_id is null or company_id=p_company_id) order by created_at desc;
end $$;

-- ===== GUARDS on existing key RPCs (supersede 0004/0012) =====
-- create_api_key: an agent-subject key must reference an ACTIVE agent in the same team.
create or replace function paperclip.create_api_key(
  p_team_id uuid, p_name text, p_prefix text, p_key_hash text,
  p_subject_type paperclip.api_key_subject default 'user', p_agent_id uuid default null,
  p_scopes jsonb default '[]'::jsonb, p_scope_config jsonb default null, p_expires_at timestamptz default null)
returns uuid language plpgsql security definer set search_path = '' as $$
declare v_uid uuid := auth.uid(); v_id uuid;
begin
  if v_uid is null then raise exception 'not authenticated' using errcode='28000'; end if;
  if not paperclip_private.fn_has_team_role(p_team_id, v_uid, array['owner','admin','operator']::paperclip.team_role[]) then
    raise exception 'insufficient role to create api key' using errcode='42501'; end if;
  if p_subject_type = 'agent' and not paperclip_private.fn_agent_active(p_agent_id, p_team_id) then
    raise exception 'agent key requires an active agent in this team' using errcode='42501'; end if;
  insert into paperclip.api_keys(team_id, created_by, subject_type, agent_id, name, prefix, key_hash, scopes, scope_config, expires_at)
  values (p_team_id, v_uid, p_subject_type, p_agent_id, p_name, p_prefix, p_key_hash, coalesce(p_scopes,'[]'::jsonb), p_scope_config, p_expires_at)
  returning id into v_id;
  return v_id;
end $$;

-- create_job_with_key: agent-subject keys can only run if the agent is active (limbo, checked BEFORE idempotency).
create or replace function paperclip.create_job_with_key(p_prefix text, p_key_hash text, p_company_id uuid, p_prompt text, p_model text default 'gpt-5.4-mini', p_client_token text default null)
returns uuid language plpgsql security definer set search_path='' as $$
declare k record; v_team uuid; v_id uuid;
begin
  select * into k from paperclip_private.fn_key_principal(p_prefix, p_key_hash);
  if k.team_id is null then raise exception 'invalid api key' using errcode='28000'; end if;
  if not paperclip_private.fn_key_allows(k.scopes, 'jobs:write') then
    raise exception 'api key lacks scope jobs:write' using errcode='42501'; end if;
  if k.subject_type = 'agent' and not paperclip_private.fn_agent_active(k.agent_id, k.team_id) then
    raise exception 'agent is not active' using errcode='42501'; end if;
  select team_id into v_team from paperclip.companies where id=p_company_id;
  if v_team is null or v_team <> k.team_id then raise exception 'company not in key team' using errcode='42501'; end if;
  if p_client_token is not null then
    select id into v_id from paperclip.jobs where team_id=k.team_id and company_id=p_company_id and client_token=p_client_token;
    if v_id is not null then return v_id; end if;
  end if;
  begin
    insert into paperclip.jobs(company_id, team_id, created_by, subject_type, agent_id, prompt, model, status, client_token)
    values (p_company_id, k.team_id, k.created_by, k.subject_type, k.agent_id, p_prompt, p_model, 'running', p_client_token) returning id into v_id;
  exception when unique_violation then
    select id into v_id from paperclip.jobs where team_id=k.team_id and company_id=p_company_id and client_token=p_client_token;
  end;
  return v_id;
end $$;

-- ===== Grants (least privilege) =====
revoke execute on function paperclip.hire_agent(uuid,text,text,text,text,text,uuid,boolean), paperclip.decide_approval(uuid,boolean) from public, anon;
grant  execute on function paperclip.hire_agent(uuid,text,text,text,text,text,uuid,boolean), paperclip.decide_approval(uuid,boolean) to authenticated;
revoke execute on function
  paperclip.hire_agent_with_key(text,text,uuid,text,text,text,text,text,uuid),
  paperclip.decide_approval_with_key(text,text,uuid,boolean),
  paperclip.list_agents_with_key(text,text,uuid),
  paperclip.list_approvals_with_key(text,text,uuid) from public;
grant  execute on function
  paperclip.hire_agent_with_key(text,text,uuid,text,text,text,text,text,uuid),
  paperclip.decide_approval_with_key(text,text,uuid,boolean),
  paperclip.list_agents_with_key(text,text,uuid),
  paperclip.list_approvals_with_key(text,text,uuid) to anon, authenticated;
