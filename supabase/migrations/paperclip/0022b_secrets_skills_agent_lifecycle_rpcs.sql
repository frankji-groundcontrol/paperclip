-- Phase 05 / 01 remaining: create_secret(_with_key), upsert_company_skill, pause/resume/terminate/clear-error agent,
-- update_agent_permissions RPCs. All team-scoped + activity logged.
-- ===== secrets write RPCs (Phase 05) =====
create or replace function paperclip.create_secret(
  p_company_id uuid, p_key text, p_name text, p_description text default null,
  p_ciphertext text default null, p_provider text default 'local_encrypted')
returns jsonb language plpgsql security definer set search_path='' as $$
declare v_uid uuid := auth.uid(); v_team uuid; v_sec uuid;
begin
  if v_uid is null then raise exception 'not authenticated' using errcode='28000'; end if;
  select team_id into v_team from paperclip.companies where id=p_company_id;
  if v_team is null or not paperclip_private.fn_has_team_role(v_team, v_uid, array['owner','admin']::paperclip.team_role[]) then
    raise exception 'board only may create secrets' using errcode='42501'; end if;
  insert into paperclip.company_secrets(company_id, team_id, key, name, description, provider, created_by_user_id)
    values (p_company_id, v_team, p_key, p_name, p_description, p_provider, v_uid)
    returning id into v_sec;
  if p_ciphertext is not null then
    insert into paperclip.company_secret_versions(secret_id, company_id, team_id, version, ciphertext)
      values (v_sec, p_company_id, v_team, 1, p_ciphertext);
  end if;
  perform paperclip_private.log_activity(v_team, p_company_id, 'user', v_uid::text, 'secret.created', 'secret', v_sec::text);
  return jsonb_build_object('secretId', v_sec, 'ciphertext','<redacted>');
end $$;

create or replace function paperclip.create_secret_with_key(
  p_prefix text, p_key_hash text, p_company_id uuid, p_key text, p_name text,
  p_description text default null, p_ciphertext text default null, p_provider text default 'local_encrypted')
returns jsonb language plpgsql security definer set search_path='' as $$
declare k record; v_sec uuid;
begin
  select * into k from paperclip_private.fn_key_principal(p_prefix, p_key_hash);
  if k.team_id is null then raise exception 'invalid api key' using errcode='28000'; end if;
  if k.subject_type='agent' then raise exception 'agent keys cannot create secrets' using errcode='42501'; end if;
  if not exists(select 1 from paperclip.companies where id=p_company_id and team_id=k.team_id) then
    raise exception 'company not in key team' using errcode='42501'; end if;
  insert into paperclip.company_secrets(company_id, team_id, key, name, description, provider, created_by_user_id)
    values (p_company_id, k.team_id, p_key, p_name, p_description, p_provider, k.created_by::text)
    returning id into v_sec;
  if p_ciphertext is not null then
    insert into paperclip.company_secret_versions(secret_id, company_id, team_id, version, ciphertext)
      values (v_sec, p_company_id, k.team_id, 1, p_ciphertext);
  end if;
  perform paperclip_private.log_activity(k.team_id, p_company_id, 'api_key', k.created_by::text, 'secret.created', 'secret', v_sec::text);
  return jsonb_build_object('secretId', v_sec, 'ciphertext','<redacted>');
end $$;

-- ===== skills write RPC (Phase 05) =====
create or replace function paperclip.upsert_company_skill(
  p_company_id uuid, p_slug text, p_name text, p_source text default 'builtin',
  p_enabled boolean default true, p_config jsonb default '{}'::jsonb)
returns jsonb language plpgsql security definer set search_path='' as $$
declare v_uid uuid := auth.uid(); v_team uuid; v_id uuid;
begin
  if v_uid is null then raise exception 'not authenticated' using errcode='28000'; end if;
  select team_id into v_team from paperclip.companies where id=p_company_id;
  if v_team is null or not paperclip_private.fn_has_team_role(v_team, v_uid, array['owner','admin']::paperclip.team_role[]) then
    raise exception 'board only may manage skills' using errcode='42501'; end if;
  insert into paperclip.company_skills(company_id, team_id, slug, name, source, enabled, config)
    values (p_company_id, v_team, p_slug, p_name, p_source, p_enabled, p_config)
    on conflict (company_id, slug) do update set name=excluded.name, source=excluded.source,
      enabled=excluded.enabled, config=excluded.config, updated_at=now()
    returning id into v_id;
  perform paperclip_private.log_activity(v_team, p_company_id, 'user', v_uid::text, 'skill.upserted', 'skill', v_id::text);
  return jsonb_build_object('skillId', v_id);
end $$;

-- ===== adapter validation RPC (Phase 05) =====
create or replace function paperclip.validate_adapter_type(p_adapter_type text)
returns jsonb language sql stable security definer set search_path='' as $$
  select jsonb_build_object('valid', exists(select 1 from paperclip.agent_adapter_types where adapter_type=p_adapter_type),
                            'adapterType', p_adapter_type);
$$;

-- ===== pause agent RPC (Phase 01/07) =====
create or replace function paperclip.pause_agent(p_agent_id uuid, p_reason text default 'manual')
returns jsonb language plpgsql security definer set search_path='' as $$
declare v_uid uuid := auth.uid(); v_ag paperclip.agents%rowtype;
begin
  if v_uid is null then raise exception 'not authenticated' using errcode='28000'; end if;
  select * into v_ag from paperclip.agents where id=p_agent_id for update;
  if v_ag.id is null then raise exception 'agent not found' using errcode='P0002'; end if;
  if not paperclip_private.fn_has_team_role(v_ag.team_id, v_uid, array['owner','admin']::paperclip.team_role[]) then
    raise exception 'board only may pause agents' using errcode='42501'; end if;
  if v_ag.status='terminated' then raise exception 'cannot pause terminated agent' using errcode='22023'; end if;
  update paperclip.agents set status='paused', pause_reason=p_reason, paused_at=now() where id=p_agent_id;
  perform paperclip_private.log_activity(v_ag.team_id, v_ag.company_id, 'user', v_uid::text, 'agent.paused', 'agent', v_ag.id::text,
    jsonb_build_object('reason', p_reason));
  return jsonb_build_object('agentId', v_ag.id, 'status', 'paused');
end $$;

create or replace function paperclip.resume_agent(p_agent_id uuid)
returns jsonb language plpgsql security definer set search_path='' as $$
declare v_uid uuid := auth.uid(); v_ag paperclip.agents%rowtype;
begin
  if v_uid is null then raise exception 'not authenticated' using errcode='28000'; end if;
  select * into v_ag from paperclip.agents where id=p_agent_id for update;
  if v_ag.id is null then raise exception 'agent not found' using errcode='P0002'; end if;
  if not paperclip_private.fn_has_team_role(v_ag.team_id, v_uid, array['owner','admin']::paperclip.team_role[]) then
    raise exception 'board only may resume agents' using errcode='42501'; end if;
  if v_ag.status='terminated' then raise exception 'cannot resume terminated agent' using errcode='22023'; end if;
  if v_ag.status='pending_approval' then raise exception 'cannot resume pending_approval agent' using errcode='22023'; end if;
  update paperclip.agents set status='idle', pause_reason=null, paused_at=null, error_reason=null where id=p_agent_id;
  perform paperclip_private.log_activity(v_ag.team_id, v_ag.company_id, 'user', v_uid::text, 'agent.resumed', 'agent', v_ag.id::text);
  return jsonb_build_object('agentId', v_ag.id, 'status', 'idle');
end $$;

create or replace function paperclip.terminate_agent(p_agent_id uuid)
returns jsonb language plpgsql security definer set search_path='' as $$
declare v_uid uuid := auth.uid(); v_ag paperclip.agents%rowtype;
begin
  if v_uid is null then raise exception 'not authenticated' using errcode='28000'; end if;
  select * into v_ag from paperclip.agents where id=p_agent_id for update;
  if v_ag.id is null then raise exception 'agent not found' using errcode='P0002'; end if;
  if not paperclip_private.fn_has_team_role(v_ag.team_id, v_uid, array['owner','admin']::paperclip.team_role[]) then
    raise exception 'board only may terminate agents' using errcode='42501'; end if;
  if v_ag.status='terminated' then return jsonb_build_object('agentId', v_ag.id, 'status', 'terminated'); end if;
  update paperclip.agents set status='terminated', error_reason=null, pause_reason=null where id=p_agent_id;
  update paperclip.api_keys set revoked_at=now() where subject_type='agent' and agent_id=p_agent_id and revoked_at is null;
  perform paperclip_private.log_activity(v_ag.team_id, v_ag.company_id, 'user', v_uid::text, 'agent.terminated', 'agent', v_ag.id::text);
  return jsonb_build_object('agentId', v_ag.id, 'status', 'terminated');
end $$;

create or replace function paperclip.clear_error_agent(p_agent_id uuid)
returns jsonb language plpgsql security definer set search_path='' as $$
declare v_uid uuid := auth.uid(); v_ag paperclip.agents%rowtype;
begin
  if v_uid is null then raise exception 'not authenticated' using errcode='28000'; end if;
  select * into v_ag from paperclip.agents where id=p_agent_id for update;
  if v_ag.id is null then raise exception 'agent not found' using errcode='P0002'; end if;
  if not paperclip_private.fn_has_team_role(v_ag.team_id, v_uid, array['owner','admin']::paperclip.team_role[]) then
    raise exception 'board only may clear errors' using errcode='42501'; end if;
  if v_ag.status<>'error' then raise exception 'only error agents can be cleared' using errcode='22023'; end if;
  update paperclip.agents set status='idle', error_reason=null where id=p_agent_id;
  perform paperclip_private.log_activity(v_ag.team_id, v_ag.company_id, 'user', v_uid::text, 'agent.error_cleared', 'agent', v_ag.id::text);
  return jsonb_build_object('agentId', v_ag.id, 'status', 'idle');
end $$;

-- ===== agent permissions update RPC (Phase 01) =====
create or replace function paperclip.update_agent_permissions(p_agent_id uuid, p_can_create_agents boolean default null)
returns jsonb language plpgsql security definer set search_path='' as $$
declare v_uid uuid := auth.uid(); v_ag paperclip.agents%rowtype; v_perms jsonb;
begin
  if v_uid is null then raise exception 'not authenticated' using errcode='28000'; end if;
  select * into v_ag from paperclip.agents where id=p_agent_id for update;
  if v_ag.id is null then raise exception 'agent not found' using errcode='P0002'; end if;
  if not paperclip_private.fn_has_team_role(v_ag.team_id, v_uid, array['owner','admin']::paperclip.team_role[]) then
    raise exception 'board only may update permissions' using errcode='42501'; end if;
  v_perms := coalesce(v_ag.permissions, '{}'::jsonb);
  if p_can_create_agents is not null then v_perms := jsonb_set(v_perms, '{can_create_agents}', to_jsonb(p_can_create_agents)); end if;
  update paperclip.agents set permissions=v_perms where id=p_agent_id;
  perform paperclip_private.log_activity(v_ag.team_id, v_ag.company_id, 'user', v_uid::text, 'agent.permissions_updated', 'agent', v_ag.id::text, v_perms);
  return jsonb_build_object('agentId', v_ag.id, 'permissions', v_perms);
end $$;


revoke execute on function
  paperclip.create_secret(uuid,text,text,text,text,text),
  paperclip.create_secret_with_key(text,text,uuid,text,text,text,text,text),
  paperclip.upsert_company_skill(uuid,text,text,text,boolean,jsonb),
  paperclip.pause_agent(uuid,text),
  paperclip.resume_agent(uuid),
  paperclip.terminate_agent(uuid),
  paperclip.clear_error_agent(uuid),
  paperclip.update_agent_permissions(uuid,boolean) from public, anon;
grant execute on function
  paperclip.create_secret(uuid,text,text,text,text,text),
  paperclip.create_secret_with_key(text,text,uuid,text,text,text,text,text),
  paperclip.upsert_company_skill(uuid,text,text,text,boolean,jsonb),
  paperclip.pause_agent(uuid,text),
  paperclip.resume_agent(uuid),
  paperclip.terminate_agent(uuid),
  paperclip.clear_error_agent(uuid),
  paperclip.update_agent_permissions(uuid,boolean) to anon, authenticated;
