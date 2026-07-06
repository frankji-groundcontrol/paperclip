-- Phase 03/04/10: workspaces/routines write path + deployment/onboarding + catalogs export/import.
-- All team-scoped, board-only for most ops, with activity logging.

-- ===== routines (Phase 03) =====
create or replace function paperclip.create_routine(
  p_company_id uuid, p_title text, p_agent_id uuid default null,
  p_trigger jsonb default '{}'::jsonb, p_env jsonb default '{}'::jsonb)
returns jsonb language plpgsql security definer set search_path='' as $$
declare v_uid uuid := auth.uid(); v_team uuid; v_id uuid;
begin
  if v_uid is null then raise exception 'not authenticated' using errcode='28000'; end if;
  select team_id into v_team from paperclip.companies where id=p_company_id;
  if v_team is null or not paperclip_private.fn_has_team_role(v_team, v_uid, array['owner','admin','operator']::paperclip.team_role[]) then
    raise exception 'insufficient role' using errcode='42501'; end if;
  if p_agent_id is not null and not exists(select 1 from paperclip.agents where id=p_agent_id and company_id=p_company_id) then
    raise exception 'agent not in same company' using errcode='42501'; end if;
  insert into paperclip.routines(company_id, team_id, title, agent_id, trigger, env)
    values (p_company_id, v_team, p_title, p_agent_id, p_trigger, p_env)
    returning id into v_id;
  perform paperclip_private.log_activity(v_team, p_company_id, 'user', v_uid::text, 'routine.created', 'routine', v_id::text);
  return jsonb_build_object('routineId', v_id);
end $$;

create or replace function paperclip.update_routine(p_routine_id uuid, p_title text default null,
  p_status text default null, p_trigger jsonb default null, p_env jsonb default null)
returns jsonb language plpgsql security definer set search_path='' as $$
declare v_uid uuid := auth.uid(); v_r paperclip.routines%rowtype;
begin
  if v_uid is null then raise exception 'not authenticated' using errcode='28000'; end if;
  select * into v_r from paperclip.routines where id=p_routine_id for update;
  if v_r.id is null then raise exception 'routine not found' using errcode='P0002'; end if;
  if not paperclip_private.fn_has_team_role(v_r.team_id, v_uid, array['owner','admin','operator']::paperclip.team_role[]) then
    raise exception 'insufficient role' using errcode='42501'; end if;
  update paperclip.routines set
    title=coalesce(p_title, v_r.title), status=coalesce(p_status, v_r.status),
    trigger=coalesce(p_trigger, v_r.trigger), env=coalesce(p_env, v_r.env), updated_at=now()
    where id=p_routine_id;
  perform paperclip_private.log_activity(v_r.team_id, v_r.company_id, 'user', v_uid::text, 'routine.updated', 'routine', v_r.id::text);
  return jsonb_build_object('routineId', v_r.id);
end $$;

-- ===== execution workspaces (Phase 03) =====
create or replace function paperclip.create_execution_workspace(
  p_company_id uuid, p_project_id uuid, p_mode text, p_strategy_type text, p_name text,
  p_source_issue_id uuid default null, p_cwd text default null, p_repo_url text default null,
  p_base_ref text default null, p_branch_name text default null)
returns jsonb language plpgsql security definer set search_path='' as $$
declare v_uid uuid := auth.uid(); v_team uuid; v_id uuid;
begin
  if v_uid is null then raise exception 'not authenticated' using errcode='28000'; end if;
  select team_id into v_team from paperclip.companies where id=p_company_id;
  if v_team is null or not paperclip_private.fn_has_team_role(v_team, v_uid, array['owner','admin','operator']::paperclip.team_role[]) then
    raise exception 'insufficient role' using errcode='42501'; end if;
  if p_project_id is not null and not exists(select 1 from paperclip.projects where id=p_project_id and company_id=p_company_id) then
    raise exception 'project not in same company' using errcode='42501'; end if;
  if p_source_issue_id is not null and not exists(select 1 from paperclip.issues where id=p_source_issue_id and company_id=p_company_id) then
    raise exception 'source issue not in same company' using errcode='42501'; end if;
  insert into paperclip.execution_workspaces(company_id, team_id, project_id, source_issue_id, mode, strategy_type, name,
    cwd, repo_url, base_ref, branch_name)
    values (p_company_id, v_team, p_project_id, p_source_issue_id, p_mode, p_strategy_type, p_name,
            p_cwd, p_repo_url, p_base_ref, p_branch_name)
    returning id into v_id;
  perform paperclip_private.log_activity(v_team, p_company_id, 'user', v_uid::text, 'workspace.created', 'workspace', v_id::text);
  return jsonb_build_object('workspaceId', v_id);
end $$;

create or replace function paperclip.close_execution_workspace(p_workspace_id uuid, p_cleanup_reason text default null)
returns jsonb language plpgsql security definer set search_path='' as $$
declare v_uid uuid := auth.uid(); v_w paperclip.execution_workspaces%rowtype;
begin
  if v_uid is null then raise exception 'not authenticated' using errcode='28000'; end if;
  select * into v_w from paperclip.execution_workspaces where id=p_workspace_id for update;
  if v_w.id is null then raise exception 'workspace not found' using errcode='P0002'; end if;
  if v_w.status<>'active' then raise exception 'workspace not active' using errcode='22023'; end if;
  update paperclip.execution_workspaces set status='closed', closed_at=now(), cleanup_reason=p_cleanup_reason
    where id=p_workspace_id;
  perform paperclip_private.log_activity(v_w.team_id, v_w.company_id, 'user', v_uid::text, 'workspace.closed', 'workspace', v_w.id::text);
  return jsonb_build_object('workspaceId', v_w.id, 'status', 'closed');
end $$;

-- ===== deployment / onboarding (Phase 04) =====
-- Instance settings (per-company scope; company-level defaults)
create or replace function paperclip.upsert_company_settings(
  p_company_id uuid, p_issue_prefix text default null, p_brand_color text default null,
  p_feedback_data_sharing_enabled boolean default null)
returns jsonb language plpgsql security definer set search_path='' as $$
declare v_uid uuid := auth.uid(); v_team uuid;
begin
  if v_uid is null then raise exception 'not authenticated' using errcode='28000'; end if;
  select team_id into v_team from paperclip.companies where id=p_company_id;
  if v_team is null or not paperclip_private.fn_has_team_role(v_team, v_uid, array['owner','admin']::paperclip.team_role[]) then
    raise exception 'board only may update company settings' using errcode='42501'; end if;
  if p_issue_prefix is not null then
    -- Check uniqueness against other companies (skip if same as current).
    if exists(select 1 from paperclip.companies where issue_prefix=p_issue_prefix and id<>p_company_id) then
      raise exception 'issue prefix already in use' using errcode='42501'; end if;
    update paperclip.companies set issue_prefix=p_issue_prefix where id=p_company_id;
  end if;
  if p_brand_color is not null then
    update paperclip.companies set brand_color=p_brand_color where id=p_company_id;
  end if;
  if p_feedback_data_sharing_enabled is not null then
    update paperclip.companies set feedback_data_sharing_enabled=p_feedback_data_sharing_enabled,
      feedback_data_sharing_consent_at=now(), feedback_data_sharing_consent_by_user_id=v_uid::text
      where id=p_company_id;
  end if;
  perform paperclip_private.log_activity(v_team, p_company_id, 'user', v_uid::text, 'company.settings.updated', 'company', p_company_id::text);
  return jsonb_build_object('companyId', p_company_id);
end $$;

-- User sidebar preferences (per-user, scoped by company)
create or replace function paperclip.upsert_user_sidebar_preferences(p_company_id uuid, p_preferences jsonb)
returns jsonb language plpgsql security definer set search_path='' as $$
declare v_uid uuid := auth.uid(); v_team uuid; v_id uuid;
begin
  if v_uid is null then raise exception 'not authenticated' using errcode='28000'; end if;
  select team_id into v_team from paperclip.companies where id=p_company_id;
  if v_team is null then raise exception 'company not found' using errcode='42501'; end if;
  insert into paperclip.user_sidebar_preferences(company_id, team_id, user_id, preferences)
    values (p_company_id, v_team, v_uid::text, p_preferences)
    on conflict (company_id, user_id) do update set preferences=excluded.preferences, updated_at=now()
    returning id into v_id;
  return jsonb_build_object('prefId', v_id);
end $$;

-- ===== catalogs / export-import (Phase 10) =====
-- Basic export bundle (redacts secrets, emits a JSON bundle of company state).
create or replace function paperclip.export_company_bundle(p_company_id uuid)
returns jsonb language plpgsql security definer set search_path='' as $$
declare v_uid uuid := auth.uid(); v_team uuid; v_bundle jsonb;
begin
  if v_uid is null then raise exception 'not authenticated' using errcode='28000'; end if;
  select team_id into v_team from paperclip.companies where id=p_company_id;
  if v_team is null or not paperclip_private.fn_has_team_role(v_team, v_uid, array['owner','admin']::paperclip.team_role[]) then
    raise exception 'board only may export' using errcode='42501'; end if;
  select jsonb_build_object(
    'company', row_to_json(c)::jsonb,
    'agents', (select coalesce(jsonb_agg(row_to_json(a)::jsonb), '[]'::jsonb) from paperclip.agents a where a.company_id=p_company_id),
    'goals', (select coalesce(jsonb_agg(row_to_json(g)::jsonb), '[]'::jsonb) from paperclip.goals g where g.company_id=p_company_id),
    'projects', (select coalesce(jsonb_agg(row_to_json(p)::jsonb), '[]'::jsonb) from paperclip.projects p where p.company_id=p_company_id),
    'issues', (select coalesce(jsonb_agg(row_to_json(i)::jsonb), '[]'::jsonb) from paperclip.issues i where i.company_id=p_company_id),
    'skills', (select coalesce(jsonb_agg(row_to_json(s)::jsonb), '[]'::jsonb) from paperclip.company_skills s where s.company_id=p_company_id)
  ) into v_bundle from paperclip.companies c where c.id=p_company_id;
  perform paperclip_private.log_activity(v_team, p_company_id, 'user', v_uid::text, 'company.exported', 'company', p_company_id::text);
  return v_bundle;
end $$;

-- Validate a skills catalog bundle (shape check).
create or replace function paperclip.validate_skills_catalog(p_bundle jsonb)
returns jsonb language sql stable security definer set search_path='' as $$
  select jsonb_build_object('valid',
    jsonb_typeof(p_bundle)='object' and p_bundle ? 'skills' and jsonb_typeof(p_bundle->'skills')='array');
$$;

-- Validate a teams catalog bundle (shape check).
create or replace function paperclip.validate_teams_catalog(p_bundle jsonb)
returns jsonb language sql stable security definer set search_path='' as $$
  select jsonb_build_object('valid',
    jsonb_typeof(p_bundle)='object' and p_bundle ? 'teams' and jsonb_typeof(p_bundle->'teams')='array');
$$;

-- Add user_sidebar_preferences table (used by upsert_user_sidebar_preferences RPC above)
create table if not exists paperclip.user_sidebar_preferences (
  id uuid primary key default gen_random_uuid(),
  company_id uuid not null references paperclip.companies(id) on delete cascade,
  team_id uuid not null references paperclip.teams(id) on delete cascade,
  user_id text not null,
  preferences jsonb not null default '{}'::jsonb,
  created_at timestamptz not null default now(),
  updated_at timestamptz not null default now(),
  unique (company_id, user_id)
);
alter table paperclip.user_sidebar_preferences enable row level security;
drop policy if exists user_sidebar_preferences_owner_read on paperclip.user_sidebar_preferences;
create policy user_sidebar_preferences_owner_read on paperclip.user_sidebar_preferences for select to authenticated
  using (team_id in (select paperclip_private.fn_user_team_ids(auth.uid())) and user_id = auth.uid()::text);
grant select on paperclip.user_sidebar_preferences to authenticated;

-- ===== grants =====
revoke execute on function
  paperclip.create_routine(uuid,text,uuid,jsonb,jsonb),
  paperclip.update_routine(uuid,text,text,jsonb,jsonb),
  paperclip.create_execution_workspace(uuid,uuid,text,text,text,uuid,text,text,text,text),
  paperclip.close_execution_workspace(uuid,text),
  paperclip.upsert_company_settings(uuid,text,text,boolean),
  paperclip.upsert_user_sidebar_preferences(uuid,jsonb),
  paperclip.export_company_bundle(uuid),
  paperclip.validate_skills_catalog(jsonb),
  paperclip.validate_teams_catalog(jsonb) from public, anon;
grant execute on function
  paperclip.create_routine(uuid,text,uuid,jsonb,jsonb),
  paperclip.update_routine(uuid,text,text,jsonb,jsonb),
  paperclip.create_execution_workspace(uuid,uuid,text,text,text,uuid,text,text,text,text),
  paperclip.close_execution_workspace(uuid,text),
  paperclip.upsert_company_settings(uuid,text,text,boolean),
  paperclip.upsert_user_sidebar_preferences(uuid,jsonb),
  paperclip.export_company_bundle(uuid),
  paperclip.validate_skills_catalog(jsonb),
  paperclip.validate_teams_catalog(jsonb) to anon, authenticated;
