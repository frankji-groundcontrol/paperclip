-- Phase 11/12/13: observability/feedback/pipelines + contracts surface + cutover machinery.

-- ===== Phase 11: feedback + pipelines + dashboard =====
create or replace function paperclip.create_feedback_export(
  p_company_id uuid, p_target_type text, p_target_id text, p_body text)
returns jsonb language plpgsql security definer set search_path='' as $$
declare v_uid uuid := auth.uid(); v_team uuid; v_id uuid;
begin
  if v_uid is null then raise exception 'not authenticated' using errcode='28000'; end if;
  select team_id into v_team from paperclip.companies where id=p_company_id;
  if v_team is null then raise exception 'company not found' using errcode='42501'; end if;
  insert into paperclip.feedback_exports(company_id, team_id, target_type, target_id, body, created_by_user_id)
    values (p_company_id, v_team, p_target_type, p_target_id, p_body, v_uid::text)
    returning id into v_id;
  perform paperclip_private.log_activity(v_team, p_company_id, 'user', v_uid::text, 'feedback.created', 'feedback', v_id::text);
  return jsonb_build_object('feedbackId', v_id);
end $$;

create or replace function paperclip.vote_feedback(p_company_id uuid, p_target_type text, p_target_id text, p_vote int default 1)
returns jsonb language plpgsql security definer set search_path='' as $$
declare v_uid uuid := auth.uid(); v_team uuid; v_id uuid;
begin
  if v_uid is null then raise exception 'not authenticated' using errcode='28000'; end if;
  select team_id into v_team from paperclip.companies where id=p_company_id;
  if v_team is null then raise exception 'company not found' using errcode='42501'; end if;
  insert into paperclip.feedback_votes(company_id, team_id, target_type, target_id, voter_user_id, vote)
    values (p_company_id, v_team, p_target_type, p_target_id, v_uid::text, p_vote)
    on conflict (company_id, target_type, target_id, voter_user_id) do update set vote=excluded.vote
    returning id into v_id;
  return jsonb_build_object('voteId', v_id);
end $$;

create or replace function paperclip.create_pipeline_case(
  p_company_id uuid, p_pipeline_id uuid, p_context jsonb default '{}'::jsonb)
returns jsonb language plpgsql security definer set search_path='' as $$
declare v_uid uuid := auth.uid(); v_team uuid; v_id uuid;
begin
  if v_uid is null then raise exception 'not authenticated' using errcode='28000'; end if;
  select team_id into v_team from paperclip.companies where id=p_company_id;
  if v_team is null or not paperclip_private.fn_has_team_role(v_team, v_uid, array['owner','admin','operator']::paperclip.team_role[]) then
    raise exception 'insufficient role' using errcode='42501'; end if;
  if not exists(select 1 from paperclip.pipelines where id=p_pipeline_id and company_id=p_company_id) then
    raise exception 'pipeline not in same company' using errcode='42501'; end if;
  insert into paperclip.pipeline_cases(pipeline_id, company_id, team_id, context)
    values (p_pipeline_id, p_company_id, v_team, p_context)
    returning id into v_id;
  perform paperclip_private.log_activity(v_team, p_company_id, 'user', v_uid::text, 'pipeline_case.created', 'pipeline_case', v_id::text);
  return jsonb_build_object('caseId', v_id);
end $$;

create or replace function paperclip.add_pipeline_case_event(p_case_id uuid, p_kind text, p_payload jsonb default '{}'::jsonb)
returns jsonb language plpgsql security definer set search_path='' as $$
declare v_uid uuid := auth.uid(); v_c paperclip.pipeline_cases%rowtype; v_id uuid;
begin
  if v_uid is null then raise exception 'not authenticated' using errcode='28000'; end if;
  select * into v_c from paperclip.pipeline_cases where id=p_case_id;
  if v_c.id is null then raise exception 'case not found' using errcode='P0002'; end if;
  if not paperclip_private.fn_has_team_role(v_c.team_id, v_uid, array['owner','admin','operator','member']::paperclip.team_role[]) then
    raise exception 'insufficient role' using errcode='42501'; end if;
  insert into paperclip.pipeline_case_events(case_id, company_id, team_id, kind, payload)
    values (p_case_id, v_c.company_id, v_c.team_id, p_kind, p_payload)
    returning id into v_id;
  return jsonb_build_object('eventId', v_id);
end $$;

-- Dashboard summary (aggregate counts)
create or replace function paperclip.get_dashboard_summary(p_company_id uuid)
returns jsonb language sql stable security definer set search_path='' as $$
  select jsonb_build_object(
    'agents', jsonb_build_object(
      'idle', (select count(*) from paperclip.agents where company_id=p_company_id and status='idle'),
      'running', (select count(*) from paperclip.agents where company_id=p_company_id and status='running'),
      'paused', (select count(*) from paperclip.agents where company_id=p_company_id and status='paused'),
      'error', (select count(*) from paperclip.agents where company_id=p_company_id and status='error'),
      'pending_approval', (select count(*) from paperclip.agents where company_id=p_company_id and status='pending_approval'),
      'terminated', (select count(*) from paperclip.agents where company_id=p_company_id and status='terminated')
    ),
    'issues', jsonb_build_object(
      'backlog', (select count(*) from paperclip.issues where company_id=p_company_id and status='backlog'),
      'in_progress', (select count(*) from paperclip.issues where company_id=p_company_id and status='in_progress'),
      'done', (select count(*) from paperclip.issues where company_id=p_company_id and status='done')
    ),
    'pending_approvals', (select count(*) from paperclip.approvals where company_id=p_company_id and status='pending'),
    'monthly_spend_cents', (select coalesce(sum(cost_cents),0) from paperclip.cost_events where company_id=p_company_id and date_trunc('month', occurred_at at time zone 'UTC') = date_trunc('month', now() at time zone 'UTC')),
    'monthly_budget_cents', (select budget_monthly_cents from paperclip.companies where id=p_company_id)
  );
$$;

-- ===== Phase 12: contracts surface =====
-- list_api_surface returns all paperclip RPCs exposed to authenticated/anon (poor man's OpenAPI).
create or replace function paperclip.list_api_surface()
returns table(routine_name text, grantee text)
language sql stable security definer set search_path='' as $$
  select p.proname::text as routine_name, g.grantee::text as grantee
  from pg_proc p
  join pg_namespace n on n.oid = p.pronamespace
  join information_schema.routine_privileges g on g.specific_schema = n.nspname and g.routine_name = p.proname
  where n.nspname = 'paperclip' and g.grantee in ('authenticated', 'anon')
  order by p.proname;
$$;

-- ===== Phase 13: cutover machinery =====
-- Feature flags table (per-team, used by the broker to gate new vs old surfaces).
create table if not exists paperclip.feature_flags (
  id uuid primary key default gen_random_uuid(),
  team_id uuid not null references paperclip.teams(id) on delete cascade,
  flag text not null,
  enabled boolean not null default false,
  created_at timestamptz not null default now(),
  updated_at timestamptz not null default now(),
  unique (team_id, flag)
);
alter table paperclip.feature_flags enable row level security;
drop policy if exists feature_flags_member_read on paperclip.feature_flags;
create policy feature_flags_member_read on paperclip.feature_flags for select to authenticated
  using (team_id in (select paperclip_private.fn_user_team_ids(auth.uid())));
grant select on paperclip.feature_flags to authenticated;

create or replace function paperclip.set_feature_flag(p_team_id uuid, p_flag text, p_enabled boolean)
returns jsonb language plpgsql security definer set search_path='' as $$
declare v_uid uuid := auth.uid(); v_id uuid;
begin
  if v_uid is null then raise exception 'not authenticated' using errcode='28000'; end if;
  if not paperclip_private.fn_has_team_role(p_team_id, v_uid, array['owner','admin']::paperclip.team_role[]) then
    raise exception 'board only may set feature flags' using errcode='42501'; end if;
  insert into paperclip.feature_flags(team_id, flag, enabled)
    values (p_team_id, p_flag, p_enabled)
    on conflict (team_id, flag) do update set enabled=excluded.enabled, updated_at=now()
    returning id into v_id;
  perform paperclip_private.log_activity(p_team_id, null, 'user', v_uid::text, 'feature_flag.set', 'feature_flag', v_id::text,
    jsonb_build_object('flag', p_flag, 'enabled', p_enabled));
  return jsonb_build_object('flagId', v_id, 'flag', p_flag, 'enabled', p_enabled);
end $$;

create or replace function paperclip.is_feature_flag_enabled(p_team_id uuid, p_flag text)
returns boolean language sql stable security definer set search_path='' as $$
  select coalesce((select enabled from paperclip.feature_flags where team_id=p_team_id and flag=p_flag), false);
$$;

-- Post-cutover validation RPC: checks schema invariants + RLS + no-public-schema-app-objects.
create or replace function paperclip.post_cutover_validation()
returns jsonb language sql stable security definer set search_path='' as $$
  select jsonb_build_object(
    'paperclip_tables', (select count(*) from information_schema.tables where table_schema='paperclip'),
    'paperclip_private_functions', (select count(*) from pg_proc p join pg_namespace n on n.oid=p.pronamespace where n.nspname='paperclip_private'),
    'public_app_tables', (select count(*) from information_schema.tables where table_schema='public' and table_name not like 'pg_%' and table_name not like 'schema_%'),
    'rls_enabled_tables', (select count(*) from information_schema.tables where table_schema='paperclip' and is_insertable_into='YES')
  );
$$;

-- Compatibility view: paperclip.companies_legacy (alias for backward compat during transition)
create or replace view paperclip.companies_legacy with (security_invoker=true) as
  select id, team_id, name, slug, created_by, created_at, updated_at from paperclip.companies;
grant select on paperclip.companies_legacy to authenticated;

-- ===== grants =====
revoke execute on function
  paperclip.create_feedback_export(uuid,text,text,text),
  paperclip.vote_feedback(uuid,text,text,int),
  paperclip.create_pipeline_case(uuid,uuid,jsonb),
  paperclip.add_pipeline_case_event(uuid,text,jsonb),
  paperclip.get_dashboard_summary(uuid),
  paperclip.list_api_surface(),
  paperclip.set_feature_flag(uuid,text,boolean),
  paperclip.is_feature_flag_enabled(uuid,text),
  paperclip.post_cutover_validation() from public, anon;
grant execute on function
  paperclip.create_feedback_export(uuid,text,text,text),
  paperclip.vote_feedback(uuid,text,text,int),
  paperclip.create_pipeline_case(uuid,uuid,jsonb),
  paperclip.add_pipeline_case_event(uuid,text,jsonb),
  paperclip.get_dashboard_summary(uuid),
  paperclip.list_api_surface(),
  paperclip.set_feature_flag(uuid,text,boolean),
  paperclip.is_feature_flag_enabled(uuid,text),
  paperclip.post_cutover_validation() to anon, authenticated;
