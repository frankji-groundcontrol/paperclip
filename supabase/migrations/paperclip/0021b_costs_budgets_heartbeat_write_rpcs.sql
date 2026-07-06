-- Phase 06/07 write-path RPCs: cost ingestion + budget policy + heartbeat run lifecycle + wakeup.
-- ===== cost ingestion (Phase 06) =====
create or replace function paperclip.ingest_cost_event(
  p_company_id uuid, p_agent_id uuid default null, p_issue_id uuid default null, p_project_id uuid default null,
  p_run_id uuid default null, p_model text default null, p_billing_code text default null,
  p_input_tokens int default 0, p_output_tokens int default 0, p_cost_cents int default 0)
returns jsonb language plpgsql security definer set search_path='' as $$
declare v_uid uuid := auth.uid(); v_team uuid; v_id uuid; v_monthly int; v_budget int; v_inc uuid;
begin
  if v_uid is null then raise exception 'not authenticated' using errcode='28000'; end if;
  select team_id into v_team from paperclip.companies where id=p_company_id;
  if v_team is null then raise exception 'company not found' using errcode='42501'; end if;
  insert into paperclip.cost_events(company_id, team_id, agent_id, issue_id, project_id, run_id, model, billing_code,
    input_tokens, output_tokens, total_tokens, cost_cents)
    values (p_company_id, v_team, p_agent_id, p_issue_id, p_project_id, p_run_id, p_model, p_billing_code,
            p_input_tokens, p_output_tokens, p_input_tokens+p_output_tokens, p_cost_cents)
    returning id into v_id;
  -- hard-stop auto-pause: if company budget exceeded, create an incident + pause the agent.
  select budget_monthly_cents into v_monthly from paperclip.companies where id=p_company_id;
  select coalesce(sum(cost_cents),0) into v_budget from paperclip.cost_events
    where company_id=p_company_id and date_trunc('month', occurred_at at time zone 'UTC') = date_trunc('month', now() at time zone 'UTC');
  if v_monthly > 0 and v_budget >= v_monthly then
    insert into paperclip.budget_incidents(company_id, team_id, kind, details)
      values (p_company_id, v_team, 'company_over_budget',
              jsonb_build_object('monthly', v_monthly, 'spent', v_budget))
      returning id into v_inc;
    if p_agent_id is not null then
      update paperclip.agents set status='paused', pause_reason='company_over_budget', paused_at=now()
        where id=p_agent_id and status in ('idle','running');
    end if;
  end if;
  perform paperclip_private.log_activity(v_team, p_company_id, 'system', null, 'cost.ingested', 'cost_event', v_id::text);
  return jsonb_build_object('costEventId', v_id, 'incidentId', v_inc, 'monthlySpent', v_budget, 'monthlyLimit', v_monthly);
end $$;

-- ===== budget policy upsert (Phase 06) =====
create or replace function paperclip.upsert_budget_policy(
  p_company_id uuid, p_scope_type text, p_scope_id uuid default null, p_monthly_limit_cents int, p_hard_stop boolean default true)
returns jsonb language plpgsql security definer set search_path='' as $$
declare v_uid uuid := auth.uid(); v_team uuid; v_id uuid;
begin
  if v_uid is null then raise exception 'not authenticated' using errcode='28000'; end if;
  select team_id into v_team from paperclip.companies where id=p_company_id;
  if v_team is null or not paperclip_private.fn_has_team_role(v_team, v_uid, array['owner','admin']::paperclip.team_role[]) then
    raise exception 'board only may manage budgets' using errcode='42501'; end if;
  insert into paperclip.budget_policies(company_id, team_id, scope_type, scope_id, monthly_limit_cents, hard_stop)
    values (p_company_id, v_team, p_scope_type, p_scope_id, p_monthly_limit_cents, p_hard_stop)
    on conflict (company_id, scope_type, scope_id) do update
      set monthly_limit_cents=excluded.monthly_limit_cents, hard_stop=excluded.hard_stop, updated_at=now()
    returning id into v_id;
  perform paperclip_private.log_activity(v_team, p_company_id, 'user', v_uid::text, 'budget_policy.upserted', 'budget_policy', v_id::text);
  return jsonb_build_object('policyId', v_id);
end $$;

-- ===== heartbeat run lifecycle (Phase 07) =====
create or replace function paperclip.create_heartbeat_run(
  p_company_id uuid, p_agent_id uuid, p_invocation_source text default 'on_demand',
  p_trigger_detail text default null, p_wakeup_request_id uuid default null)
returns jsonb language plpgsql security definer set search_path='' as $$
declare v_uid uuid := auth.uid(); v_team uuid; v_id uuid;
begin
  if v_uid is null then raise exception 'not authenticated' using errcode='28000'; end if;
  select team_id into v_team from paperclip.companies where id=p_company_id;
  if v_team is null then raise exception 'company not found' using errcode='42501'; end if;
  if not paperclip_private.fn_agent_active(p_agent_id, v_team) then
    raise exception 'agent not active/invokable' using errcode='42501'; end if;
  -- transition: idle -> running.
  update paperclip.agents set status='running', last_heartbeat_at=now() where id=p_agent_id and status='idle';
  if not found then raise exception 'agent not in idle state' using errcode='22023'; end if;
  insert into paperclip.heartbeat_runs(company_id, team_id, agent_id, invocation_source, trigger_detail, wakeup_request_id, status, started_at)
    values (p_company_id, v_team, p_agent_id, p_invocation_source, p_trigger_detail, p_wakeup_request_id, 'running', now())
    returning id into v_id;
  perform paperclip_private.log_activity(v_team, p_company_id, 'user', v_uid::text, 'heartbeat_run.started', 'heartbeat_run', v_id::text);
  return jsonb_build_object('runId', v_id);
end $$;

create or replace function paperclip.complete_heartbeat_run(p_run_id uuid, p_result_text text default null, p_usage_json jsonb default null)
returns jsonb language plpgsql security definer set search_path='' as $$
declare v_uid uuid := auth.uid(); v_run paperclip.heartbeat_runs%rowtype;
begin
  if v_uid is null then raise exception 'not authenticated' using errcode='28000'; end if;
  select * into v_run from paperclip.heartbeat_runs where id=p_run_id for update;
  if v_run.id is null then raise exception 'run not found' using errcode='P0002'; end if;
  if v_run.status <> 'running' then raise exception 'run is not running' using errcode='22023'; end if;
  update paperclip.heartbeat_runs set status='succeeded', finished_at=now(), result_text=p_result_text, usage_json=p_usage_json where id=p_run_id;
  insert into paperclip.heartbeat_run_events(run_id, company_id, team_id, kind, payload)
    values (p_run_id, v_run.company_id, v_run.team_id, 'run_completed', jsonb_build_object('usage', coalesce(p_usage_json,'{}'::jsonb)));
  -- transition: running -> idle (unless another run is active).
  if not exists(select 1 from paperclip.heartbeat_runs where agent_id=v_run.agent_id and status='running' and id<>p_run_id) then
    update paperclip.agents set status='idle', last_heartbeat_at=now() where id=v_run.agent_id;
  end if;
  perform paperclip_private.log_activity(v_run.team_id, v_run.company_id, 'system', null, 'heartbeat_run.completed', 'heartbeat_run', v_run.id::text);
  return jsonb_build_object('runId', v_run.id, 'status', 'succeeded');
end $$;

create or replace function paperclip.fail_heartbeat_run(p_run_id uuid, p_error text)
returns jsonb language plpgsql security definer set search_path='' as $$
declare v_uid uuid := auth.uid(); v_run paperclip.heartbeat_runs%rowtype;
begin
  if v_uid is null then raise exception 'not authenticated' using errcode='28000'; end if;
  select * into v_run from paperclip.heartbeat_runs where id=p_run_id for update;
  if v_run.id is null then raise exception 'run not found' using errcode='P0002'; end if;
  if v_run.status <> 'running' then raise exception 'run is not running' using errcode='22023'; end if;
  update paperclip.heartbeat_runs set status='failed', finished_at=now(), error=p_error where id=p_run_id;
  insert into paperclip.heartbeat_run_events(run_id, company_id, team_id, kind, payload)
    values (p_run_id, v_run.company_id, v_run.team_id, 'run_failed', jsonb_build_object('error', p_error));
  if not exists(select 1 from paperclip.heartbeat_runs where agent_id=v_run.agent_id and status='running' and id<>p_run_id) then
    update paperclip.agents set status='error', error_reason=coalesce(p_error,'unknown'), last_heartbeat_at=now() where id=v_run.agent_id;
  end if;
  perform paperclip_private.log_activity(v_run.team_id, v_run.company_id, 'system', null, 'heartbeat_run.failed', 'heartbeat_run', v_run.id::text);
  return jsonb_build_object('runId', v_run.id, 'status', 'failed');
end $$;

create or replace function paperclip.create_wakeup_request(p_company_id uuid, p_agent_id uuid, p_trigger_detail text default null,
  p_requested_by_agent_id uuid default null)
returns jsonb language plpgsql security definer set search_path='' as $$
declare v_uid uuid := auth.uid(); v_team uuid; v_id uuid;
begin
  if v_uid is null then raise exception 'not authenticated' using errcode='28000'; end if;
  select team_id into v_team from paperclip.companies where id=p_company_id;
  if v_team is null then raise exception 'company not found' using errcode='42501'; end if;
  if not exists(select 1 from paperclip.agents where id=p_agent_id and company_id=p_company_id) then
    raise exception 'agent not in same company' using errcode='42501'; end if;
  insert into paperclip.agent_wakeup_requests(company_id, team_id, agent_id, trigger_detail, requested_by_agent_id, requested_by_user_id)
    values (p_company_id, v_team, p_agent_id, p_trigger_detail, p_requested_by_agent_id, v_uid)
    returning id into v_id;
  perform paperclip_private.log_activity(v_team, p_company_id, 'user', v_uid::text, 'wakeup_request.created', 'wakeup_request', v_id::text);
  return jsonb_build_object('requestId', v_id);
end $$;

create or replace function paperclip.fulfill_wakeup_request(p_request_id uuid)
returns jsonb language plpgsql security definer set search_path='' as $$
declare v_uid uuid := auth.uid(); v_req paperclip.agent_wakeup_requests%rowtype;
begin
  if v_uid is null then raise exception 'not authenticated' using errcode='28000'; end if;
  select * into v_req from paperclip.agent_wakeup_requests where id=p_request_id for update;
  if v_req.id is null then raise exception 'request not found' using errcode='P0002'; end if;
  if v_req.status <> 'queued' then raise exception 'request not queued' using errcode='22023'; end if;
  update paperclip.agent_wakeup_requests set status='fulfilled', fulfilled_at=now() where id=p_request_id;
  perform paperclip_private.log_activity(v_req.team_id, v_req.company_id, 'system', null, 'wakeup_request.fulfilled', 'wakeup_request', v_req.id::text);
  return jsonb_build_object('requestId', v_req.id, 'status', 'fulfilled');
end $$;


revoke execute on function
  paperclip.ingest_cost_event(uuid,uuid,uuid,uuid,uuid,text,text,int,int,int),
  paperclip.upsert_budget_policy(uuid,text,uuid,int,boolean),
  paperclip.create_heartbeat_run(uuid,uuid,text,text,uuid),
  paperclip.complete_heartbeat_run(uuid,text,jsonb),
  paperclip.fail_heartbeat_run(uuid,text),
  paperclip.create_wakeup_request(uuid,uuid,text,uuid),
  paperclip.fulfill_wakeup_request(uuid) from public, anon;
grant execute on function
  paperclip.ingest_cost_event(uuid,uuid,uuid,uuid,uuid,text,text,int,int,int),
  paperclip.upsert_budget_policy(uuid,text,uuid,int,boolean),
  paperclip.create_heartbeat_run(uuid,uuid,text,text,uuid),
  paperclip.complete_heartbeat_run(uuid,text,jsonb),
  paperclip.fail_heartbeat_run(uuid,text),
  paperclip.create_wakeup_request(uuid,uuid,text,uuid),
  paperclip.fulfill_wakeup_request(uuid) to anon, authenticated;
