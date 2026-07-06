-- Phase 01 transitions parity.
alter table paperclip.agents alter column status set default 'idle';
update paperclip.agents set status = 'idle' where status = 'active';

-- private helper: is an agent active in a team?
create or replace function paperclip_private.fn_agent_active(p_agent uuid, p_team uuid)
returns boolean language sql stable security definer set search_path='' as $$
  select exists (select 1 from paperclip.agents where id = p_agent and team_id = p_team and status in ('idle','running'))
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
  v_status := case when v_require then 'pending_approval' else 'idle' end;
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
  v_status := case when v_require then 'pending_approval' else 'idle' end;
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
      update paperclip.agents set status='idle' where id=v_ap.subject_agent_id and status='pending_approval';
    end if;
    update paperclip.approvals set status='approved', decided_by=v_uid, decided_at=now() where id=p_approval_id;
    return 'approved';
  else
    if v_ap.type='hire_agent' and v_ap.subject_agent_id is not null then
      update paperclip.agents set status='terminated' where id=v_ap.subject_agent_id and status='pending_approval';
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
      update paperclip.agents set status='idle' where id=v_ap.subject_agent_id and status='pending_approval';
    end if;
    update paperclip.approvals set status='approved', decided_by=k.created_by, decided_at=now() where id=p_approval_id;
    return 'approved';
  else
    if v_ap.type='hire_agent' and v_ap.subject_agent_id is not null then
      update paperclip.agents set status='terminated' where id=v_ap.subject_agent_id and status='pending_approval';
    end if;
    update paperclip.approvals set status='rejected', decided_by=k.created_by, decided_at=now() where id=p_approval_id;
    return 'rejected';
  end if;
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
