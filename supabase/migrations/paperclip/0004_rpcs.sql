-- Public RPCs (SECURITY DEFINER, search_path=''). See 03-rpcs.md
-- ===== Identity =====
create or replace function paperclip.bootstrap_current_user()
returns table(user_id uuid, team_id uuid) language plpgsql security definer set search_path = '' as $$
declare v_uid uuid := auth.uid(); v_email text; v_team uuid;
begin
  if v_uid is null then raise exception 'not authenticated' using errcode='28000'; end if;
  select email into v_email from auth.users where id = v_uid;
  v_team := paperclip_private.fn_bootstrap_auth_user(v_uid, v_email);
  user_id := v_uid; team_id := v_team; return next;
end $$;

create or replace function paperclip.whoami()
returns jsonb language sql stable security definer set search_path = '' as $$
  select case when auth.uid() is null then null else jsonb_build_object(
    'user', (select to_jsonb(u) - 'meta' from paperclip.users u where u.id = auth.uid()),
    'teams', coalesce((select jsonb_agg(jsonb_build_object('team_id',tm.team_id,'role',tm.role,'name',t.name,'slug',t.slug))
                        from paperclip.team_members tm join paperclip.teams t on t.id=tm.team_id
                        where tm.user_id = auth.uid()), '[]'::jsonb)
  ) end
$$;

-- ===== Teams =====
create or replace function paperclip.create_team(p_name text)
returns uuid language plpgsql security definer set search_path = '' as $$
declare v_uid uuid := auth.uid(); v_id uuid; v_slug text;
begin
  if v_uid is null then raise exception 'not authenticated' using errcode='28000'; end if;
  v_slug := lower(regexp_replace(coalesce(p_name,'team'),'[^a-zA-Z0-9]+','-','g')) || '-' || lower(encode(extensions.gen_random_bytes(3),'hex'));
  insert into paperclip.teams(name, slug, created_by) values (p_name, v_slug, v_uid) returning id into v_id;
  insert into paperclip.team_members(team_id, user_id, role, added_by) values (v_id, v_uid, 'owner', v_uid);
  return v_id;
end $$;

create or replace function paperclip.rename_team(p_team uuid, p_name text)
returns boolean language plpgsql security definer set search_path = '' as $$
declare v_uid uuid := auth.uid();
begin
  if not paperclip_private.fn_has_team_role(p_team, v_uid, array['owner','admin']::paperclip.team_role[]) then
    raise exception 'insufficient role' using errcode='42501'; end if;
  update paperclip.teams set name = p_name where id = p_team; return found;
end $$;

create or replace function paperclip.delete_team(p_team uuid)
returns boolean language plpgsql security definer set search_path = '' as $$
declare v_uid uuid := auth.uid(); v_personal boolean; v_is_default boolean;
begin
  if not paperclip_private.fn_has_team_role(p_team, v_uid, array['owner']::paperclip.team_role[]) then
    raise exception 'owner required' using errcode='42501'; end if;
  select is_personal into v_personal from paperclip.teams where id = p_team;
  if v_personal then raise exception 'cannot delete a personal team' using errcode='42501'; end if;
  select exists(select 1 from paperclip.users where default_team_id = p_team) into v_is_default;
  if v_is_default then raise exception 'team is a default team for some user; reassign first' using errcode='42501'; end if;
  delete from paperclip.teams where id = p_team; return found;
end $$;

create or replace function paperclip.add_team_member(p_team uuid, p_user uuid, p_role paperclip.team_role)
returns boolean language plpgsql security definer set search_path = '' as $$
declare v_uid uuid := auth.uid();
begin
  if p_role = 'owner' then
    if not paperclip_private.fn_has_team_role(p_team, v_uid, array['owner']::paperclip.team_role[]) then
      raise exception 'only owner can grant owner' using errcode='42501'; end if;
  else
    if not paperclip_private.fn_has_team_role(p_team, v_uid, array['owner','admin']::paperclip.team_role[]) then
      raise exception 'insufficient role' using errcode='42501'; end if;
  end if;
  insert into paperclip.team_members(team_id, user_id, role, added_by)
  values (p_team, p_user, p_role, v_uid)
  on conflict (team_id, user_id) do update set role = excluded.role;
  return true;
end $$;

create or replace function paperclip.update_team_member_role(p_team uuid, p_user uuid, p_role paperclip.team_role)
returns boolean language plpgsql security definer set search_path = '' as $$
declare v_uid uuid := auth.uid(); v_owner_count int;
begin
  if not paperclip_private.fn_has_team_role(p_team, v_uid, array['owner','admin']::paperclip.team_role[]) then
    raise exception 'insufficient role' using errcode='42501'; end if;
  if p_role = 'owner' and not paperclip_private.fn_has_team_role(p_team, v_uid, array['owner']::paperclip.team_role[]) then
    raise exception 'only owner can grant owner' using errcode='42501'; end if;
  -- protect last owner
  if p_role <> 'owner' then
    select count(*) into v_owner_count from paperclip.team_members where team_id=p_team and role='owner';
    if v_owner_count = 1 and exists(select 1 from paperclip.team_members where team_id=p_team and user_id=p_user and role='owner') then
      raise exception 'cannot demote the last owner' using errcode='42501'; end if;
  end if;
  update paperclip.team_members set role = p_role where team_id=p_team and user_id=p_user; return found;
end $$;

create or replace function paperclip.remove_team_member(p_team uuid, p_user uuid)
returns boolean language plpgsql security definer set search_path = '' as $$
declare v_uid uuid := auth.uid(); v_owner_count int;
begin
  if not paperclip_private.fn_has_team_role(p_team, v_uid, array['owner','admin']::paperclip.team_role[]) then
    raise exception 'insufficient role' using errcode='42501'; end if;
  if exists(select 1 from paperclip.team_members where team_id=p_team and user_id=p_user and role='owner') then
    select count(*) into v_owner_count from paperclip.team_members where team_id=p_team and role='owner';
    if v_owner_count = 1 then raise exception 'cannot remove the last owner' using errcode='42501'; end if;
  end if;
  delete from paperclip.team_members where team_id=p_team and user_id=p_user; return found;
end $$;

-- ===== API keys =====
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
  insert into paperclip.api_keys(team_id, created_by, subject_type, agent_id, name, prefix, key_hash, scopes, scope_config, expires_at)
  values (p_team_id, v_uid, p_subject_type, p_agent_id, p_name, p_prefix, p_key_hash, coalesce(p_scopes,'[]'::jsonb), p_scope_config, p_expires_at)
  returning id into v_id;
  return v_id;
end $$;

create or replace function paperclip.revoke_api_key(p_id uuid)
returns boolean language plpgsql security definer set search_path = '' as $$
declare v_uid uuid := auth.uid(); v_team uuid;
begin
  select team_id into v_team from paperclip.api_keys where id = p_id;
  if v_team is null then return false; end if;
  if not paperclip_private.fn_has_team_role(v_team, v_uid, array['owner','admin']::paperclip.team_role[]) then
    raise exception 'insufficient role' using errcode='42501'; end if;
  update paperclip.api_keys set revoked_at = now() where id = p_id and revoked_at is null;
  return found;
end $$;

create or replace function paperclip.resolve_api_key(p_prefix text, p_key_hash text)
returns table(api_key_id uuid, team_id uuid, created_by uuid, subject_type paperclip.api_key_subject, agent_id uuid, scopes jsonb, scope_config jsonb)
language plpgsql security definer set search_path = '' as $$
begin
  return query
  update paperclip.api_keys k set last_used_at = now()
   where k.prefix = p_prefix and k.key_hash = p_key_hash and k.revoked_at is null and (k.expires_at is null or k.expires_at > now())
  returning k.id, k.team_id, k.created_by, k.subject_type, k.agent_id, k.scopes, k.scope_config;
end $$;

-- ===== Invitations =====
create or replace function paperclip.create_invitation(
  p_team_id uuid, p_role paperclip.team_role, p_code_hash text,
  p_invited_email text default null, p_allowed_join_types text default 'both',
  p_defaults_payload jsonb default null, p_ttl interval default interval '7 days')
returns uuid language plpgsql security definer set search_path = '' as $$
declare v_uid uuid := auth.uid(); v_id uuid;
begin
  if not paperclip_private.fn_has_team_role(p_team_id, v_uid, array['owner','admin']::paperclip.team_role[]) then
    raise exception 'insufficient role to invite' using errcode='42501'; end if;
  if p_role = 'owner' and not paperclip_private.fn_has_team_role(p_team_id, v_uid, array['owner']::paperclip.team_role[]) then
    raise exception 'only owner can invite owner' using errcode='42501'; end if;
  insert into paperclip.invitations(team_id, role, code_hash, invited_email, allowed_join_types, defaults_payload, created_by, expires_at)
  values (p_team_id, p_role, p_code_hash, p_invited_email, p_allowed_join_types, p_defaults_payload, v_uid, now()+p_ttl)
  returning id into v_id;
  return v_id;
end $$;

create or replace function paperclip.accept_invitation(p_code_hash text)
returns table(team_id uuid, role paperclip.team_role) language plpgsql security definer set search_path = '' as $$
declare v_uid uuid := auth.uid(); v_inv paperclip.invitations%rowtype; v_email text;
begin
  if v_uid is null then raise exception 'not authenticated' using errcode='28000'; end if;
  select * into v_inv from paperclip.invitations where code_hash = p_code_hash for update;
  if v_inv.id is null then raise exception 'invitation not found' using errcode='P0002'; end if;
  if v_inv.revoked_at is not null then raise exception 'invitation revoked' using errcode='22023'; end if;
  if v_inv.accepted_at is not null then raise exception 'invitation already used' using errcode='22023'; end if;
  if v_inv.expires_at <= now() then raise exception 'invitation expired' using errcode='22023'; end if;
  if v_inv.invited_email is not null then
    select email into v_email from auth.users where id = v_uid;
    if lower(v_email) <> lower(v_inv.invited_email::text) then raise exception 'invitation is for a different email' using errcode='42501'; end if;
  end if;
  insert into paperclip.team_members(team_id, user_id, role, added_by)
  values (v_inv.team_id, v_uid, v_inv.role, v_inv.created_by)
  on conflict (team_id, user_id) do update set role = excluded.role;
  update paperclip.invitations set accepted_by = v_uid, accepted_at = now() where id = v_inv.id;
  team_id := v_inv.team_id; role := v_inv.role; return next;
end $$;

create or replace function paperclip.revoke_invitation(p_id uuid)
returns boolean language plpgsql security definer set search_path = '' as $$
declare v_uid uuid := auth.uid(); v_team uuid;
begin
  select team_id into v_team from paperclip.invitations where id = p_id;
  if v_team is null then return false; end if;
  if not paperclip_private.fn_has_team_role(v_team, v_uid, array['owner','admin']::paperclip.team_role[]) then
    raise exception 'insufficient role' using errcode='42501'; end if;
  update paperclip.invitations set revoked_at = now() where id = p_id and revoked_at is null; return found;
end $$;

-- ===== Join requests =====
create or replace function paperclip.create_join_request(p_team_id uuid, p_request_type text default 'human', p_requester_name text default null)
returns uuid language plpgsql security definer set search_path = '' as $$
declare v_uid uuid := auth.uid(); v_id uuid;
begin
  insert into paperclip.join_requests(team_id, requested_by, request_type, requester_name)
  values (p_team_id, v_uid, p_request_type, p_requester_name) returning id into v_id;
  return v_id;
end $$;

create or replace function paperclip.decide_join_request(p_id uuid, p_approve boolean, p_role paperclip.team_role default 'member')
returns paperclip.join_request_status language plpgsql security definer set search_path = '' as $$
declare v_uid uuid := auth.uid(); v_jr paperclip.join_requests%rowtype;
begin
  select * into v_jr from paperclip.join_requests where id = p_id for update;
  if v_jr.id is null then raise exception 'join request not found' using errcode='P0002'; end if;
  if not paperclip_private.fn_has_team_role(v_jr.team_id, v_uid, array['owner','admin']::paperclip.team_role[]) then
    raise exception 'insufficient role to approve joins' using errcode='42501'; end if;
  if v_jr.status <> 'pending_approval' then raise exception 'join request is not pending' using errcode='22023'; end if;
  if p_approve then
    if v_jr.requested_by is not null then
      insert into paperclip.team_members(team_id, user_id, role, added_by)
      values (v_jr.team_id, v_jr.requested_by, p_role, v_uid) on conflict (team_id, user_id) do nothing;
    end if;
    update paperclip.join_requests set status='approved', decided_by=v_uid, decided_at=now() where id=p_id;
    return 'approved';
  else
    update paperclip.join_requests set status='rejected', decided_by=v_uid, decided_at=now() where id=p_id;
    return 'rejected';
  end if;
end $$;

-- ===== CLI / MCP device login (CLI holds the plaintext; DB stores hashes only) =====
create or replace function paperclip.cli_start_device_login(
  p_secret_hash text, p_user_code_hash text, p_pending_key_prefix text, p_pending_key_hash text,
  p_pending_key_name text, p_device_name text default 'paperclip CLI', p_command text default null,
  p_requested_access text default 'team', p_team_id uuid default null)
returns uuid language plpgsql security definer set search_path = '' as $$
declare v_id uuid;
begin
  delete from paperclip.cli_auth where expires_at < now() - interval '1 day';
  insert into paperclip.cli_auth(secret_hash, user_code_hash, pending_key_prefix, pending_key_hash, pending_key_name, device_name, command, requested_access, team_id)
  values (p_secret_hash, p_user_code_hash, p_pending_key_prefix, p_pending_key_hash, p_pending_key_name, p_device_name, p_command, p_requested_access, p_team_id)
  returning id into v_id;
  return v_id;
end $$;

create or replace function paperclip.cli_approve_device_login(p_user_code_hash text)
returns boolean language plpgsql security definer set search_path = '' as $$
declare v_uid uuid := auth.uid(); v_row paperclip.cli_auth%rowtype; v_team uuid; v_keyid uuid;
begin
  if v_uid is null then raise exception 'not authenticated' using errcode='28000'; end if;
  select * into v_row from paperclip.cli_auth where user_code_hash = p_user_code_hash for update;
  if v_row.id is null then raise exception 'code not found' using errcode='P0002'; end if;
  if v_row.expires_at <= now() then raise exception 'code expired' using errcode='22023'; end if;
  if v_row.approved_at is not null then raise exception 'already approved' using errcode='22023'; end if;
  v_team := coalesce(v_row.team_id, (select default_team_id from paperclip.users where id = v_uid));
  begin
    insert into paperclip.api_keys(team_id, created_by, subject_type, name, prefix, key_hash, meta)
    values (v_team, v_uid, 'user', v_row.pending_key_name, v_row.pending_key_prefix, v_row.pending_key_hash,
            jsonb_build_object('source','cli_device_login','device_name',v_row.device_name))
    returning id into v_keyid;
  exception when unique_violation then
    raise exception 'key prefix collision; retry device login' using errcode='40001';
  end;
  update paperclip.cli_auth set approved_by_user_id = v_uid, api_key_id = v_keyid, team_id = v_team, approved_at = now() where id = v_row.id;
  return true;
end $$;

create or replace function paperclip.cli_poll_device_login(p_secret_hash text)
returns table(status text, prefix text, team_id uuid, user_id uuid) language plpgsql security definer set search_path = '' as $$
declare v_row paperclip.cli_auth%rowtype;
begin
  select * into v_row from paperclip.cli_auth where secret_hash = p_secret_hash;
  if v_row.id is null or v_row.expires_at <= now() then status:='expired'; return next; return; end if;
  if v_row.cancelled_at is not null then status:='cancelled'; return next; return; end if;
  if v_row.approved_at is null then status:='pending'; return next; return; end if;
  status:='approved'; prefix:=v_row.pending_key_prefix; team_id:=v_row.team_id; user_id:=v_row.approved_by_user_id; return next;
end $$;

-- ===== Execute grants (superseded by 0007 least-privilege reset) =====
grant execute on function
  paperclip.bootstrap_current_user(), paperclip.whoami(),
  paperclip.create_team(text), paperclip.rename_team(uuid,text), paperclip.delete_team(uuid),
  paperclip.add_team_member(uuid,uuid,paperclip.team_role), paperclip.remove_team_member(uuid,uuid),
  paperclip.update_team_member_role(uuid,uuid,paperclip.team_role),
  paperclip.create_api_key(uuid,text,text,text,paperclip.api_key_subject,uuid,jsonb,jsonb,timestamptz),
  paperclip.revoke_api_key(uuid),
  paperclip.create_invitation(uuid,paperclip.team_role,text,text,text,jsonb,interval),
  paperclip.accept_invitation(text), paperclip.revoke_invitation(uuid),
  paperclip.create_join_request(uuid,text,text), paperclip.decide_join_request(uuid,boolean,paperclip.team_role),
  paperclip.cli_approve_device_login(text)
  to authenticated;
grant execute on function
  paperclip.resolve_api_key(text,text),
  paperclip.cli_start_device_login(text,text,text,text,text,text,text,text,uuid),
  paperclip.cli_poll_device_login(text)
  to anon, authenticated;
