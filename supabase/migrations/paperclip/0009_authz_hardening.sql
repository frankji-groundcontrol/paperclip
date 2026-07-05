-- SECURITY HARDENING (pre-commit adversarial review). Fixes:
--  * CRITICAL: cli_approve_device_login minted keys for a caller-supplied team with no membership check
--  * HIGH: add_team_member upsert could demote/remove an owner (no guard)
--  * HIGH: decide_join_request let an admin grant 'owner'
--  * MED:  update_team_member_role / remove_team_member let an admin act on owners
--  * LOW:  create_join_request had no auth check and no dedupe (spam)
--  * defense-in-depth: revoke PUBLIC execute on paperclip_private helpers
-- Proven blocked by acceptance/attack scenarios ATK1-ATK5 (+ legit L1-L3 still pass).

-- CRITICAL: bind device-login key only to a team the APPROVER belongs to
create or replace function paperclip.cli_approve_device_login(p_user_code_hash text)
returns boolean language plpgsql security definer set search_path = '' as $$
declare v_uid uuid := auth.uid(); v_row paperclip.cli_auth%rowtype; v_team uuid; v_keyid uuid;
begin
  if v_uid is null then raise exception 'not authenticated' using errcode='28000'; end if;
  select * into v_row from paperclip.cli_auth where user_code_hash = p_user_code_hash for update;
  if v_row.id is null then raise exception 'code not found' using errcode='P0002'; end if;
  if v_row.expires_at <= now() then raise exception 'code expired' using errcode='22023'; end if;
  if v_row.approved_at is not null then raise exception 'already approved' using errcode='22023'; end if;
  -- If the CLI requested a specific team, the approver MUST be a member of it.
  if v_row.team_id is not null and not paperclip_private.fn_has_team_role(
        v_row.team_id, v_uid, array['owner','admin','operator']::paperclip.team_role[]) then
    raise exception 'insufficient role to bind api key to requested team' using errcode='42501';
  end if;
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

-- HIGH: add_team_member must not let a non-owner modify an owner, nor drop the last owner
create or replace function paperclip.add_team_member(p_team uuid, p_user uuid, p_role paperclip.team_role)
returns boolean language plpgsql security definer set search_path = '' as $$
declare v_uid uuid := auth.uid(); v_existing paperclip.team_role; v_owner_count int;
begin
  if p_role = 'owner' then
    if not paperclip_private.fn_has_team_role(p_team, v_uid, array['owner']::paperclip.team_role[]) then
      raise exception 'only owner can grant owner' using errcode='42501'; end if;
  else
    if not paperclip_private.fn_has_team_role(p_team, v_uid, array['owner','admin']::paperclip.team_role[]) then
      raise exception 'insufficient role' using errcode='42501'; end if;
  end if;
  select role into v_existing from paperclip.team_members where team_id=p_team and user_id=p_user;
  if v_existing = 'owner' then
    if not paperclip_private.fn_has_team_role(p_team, v_uid, array['owner']::paperclip.team_role[]) then
      raise exception 'only an owner may modify an owner' using errcode='42501'; end if;
    if p_role <> 'owner' then
      select count(*) into v_owner_count from paperclip.team_members where team_id=p_team and role='owner';
      if v_owner_count = 1 then raise exception 'cannot demote the last owner' using errcode='42501'; end if;
    end if;
  end if;
  insert into paperclip.team_members(team_id, user_id, role, added_by)
  values (p_team, p_user, p_role, v_uid)
  on conflict (team_id, user_id) do update set role = excluded.role;
  return true;
end $$;

-- MED: update_team_member_role — only an owner may modify an owner
create or replace function paperclip.update_team_member_role(p_team uuid, p_user uuid, p_role paperclip.team_role)
returns boolean language plpgsql security definer set search_path = '' as $$
declare v_uid uuid := auth.uid(); v_owner_count int; v_target paperclip.team_role;
begin
  if not paperclip_private.fn_has_team_role(p_team, v_uid, array['owner','admin']::paperclip.team_role[]) then
    raise exception 'insufficient role' using errcode='42501'; end if;
  select role into v_target from paperclip.team_members where team_id=p_team and user_id=p_user;
  if v_target = 'owner' and not paperclip_private.fn_has_team_role(p_team, v_uid, array['owner']::paperclip.team_role[]) then
    raise exception 'only an owner may modify an owner' using errcode='42501'; end if;
  if p_role = 'owner' and not paperclip_private.fn_has_team_role(p_team, v_uid, array['owner']::paperclip.team_role[]) then
    raise exception 'only owner can grant owner' using errcode='42501'; end if;
  if p_role <> 'owner' then
    select count(*) into v_owner_count from paperclip.team_members where team_id=p_team and role='owner';
    if v_owner_count = 1 and exists(select 1 from paperclip.team_members where team_id=p_team and user_id=p_user and role='owner') then
      raise exception 'cannot demote the last owner' using errcode='42501'; end if;
  end if;
  update paperclip.team_members set role = p_role where team_id=p_team and user_id=p_user; return found;
end $$;

-- MED: remove_team_member — only an owner may remove an owner
create or replace function paperclip.remove_team_member(p_team uuid, p_user uuid)
returns boolean language plpgsql security definer set search_path = '' as $$
declare v_uid uuid := auth.uid(); v_owner_count int;
begin
  if not paperclip_private.fn_has_team_role(p_team, v_uid, array['owner','admin']::paperclip.team_role[]) then
    raise exception 'insufficient role' using errcode='42501'; end if;
  if exists(select 1 from paperclip.team_members where team_id=p_team and user_id=p_user and role='owner') then
    if not paperclip_private.fn_has_team_role(p_team, v_uid, array['owner']::paperclip.team_role[]) then
      raise exception 'only an owner may remove an owner' using errcode='42501'; end if;
    select count(*) into v_owner_count from paperclip.team_members where team_id=p_team and role='owner';
    if v_owner_count = 1 then raise exception 'cannot remove the last owner' using errcode='42501'; end if;
  end if;
  delete from paperclip.team_members where team_id=p_team and user_id=p_user; return found;
end $$;

-- HIGH: decide_join_request must not let an admin grant 'owner'
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
    if p_role = 'owner' and not paperclip_private.fn_has_team_role(v_jr.team_id, v_uid, array['owner']::paperclip.team_role[]) then
      raise exception 'only owner can grant owner' using errcode='42501'; end if;
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

-- LOW: create_join_request — require auth + dedupe pending requests
create unique index if not exists join_requests_pending_uniq
  on paperclip.join_requests(team_id, requested_by) where status = 'pending_approval';

create or replace function paperclip.create_join_request(p_team_id uuid, p_request_type text default 'human', p_requester_name text default null)
returns uuid language plpgsql security definer set search_path = '' as $$
declare v_uid uuid := auth.uid(); v_id uuid;
begin
  if v_uid is null then raise exception 'not authenticated' using errcode='28000'; end if;
  select id into v_id from paperclip.join_requests
    where team_id=p_team_id and requested_by=v_uid and status='pending_approval' limit 1;
  if v_id is not null then return v_id; end if;
  insert into paperclip.join_requests(team_id, requested_by, request_type, requester_name)
  values (p_team_id, v_uid, p_request_type, p_requester_name) returning id into v_id;
  return v_id;
end $$;

-- defense-in-depth: private helpers must not carry default PUBLIC execute
do $$
declare r record;
begin
  for r in select p.oid::regprocedure::text as sig from pg_proc p
           join pg_namespace n on n.oid=p.pronamespace where n.nspname='paperclip_private'
  loop
    execute format('revoke execute on function %s from public, anon, authenticated', r.sig);
  end loop;
end $$;

-- re-assert least-privilege execute for the redefined public RPCs (all authenticated-only)
revoke execute on function paperclip.cli_approve_device_login(text) from public, anon;
grant  execute on function paperclip.cli_approve_device_login(text) to authenticated;
revoke execute on function paperclip.add_team_member(uuid,uuid,paperclip.team_role) from public, anon;
grant  execute on function paperclip.add_team_member(uuid,uuid,paperclip.team_role) to authenticated;
revoke execute on function paperclip.update_team_member_role(uuid,uuid,paperclip.team_role) from public, anon;
grant  execute on function paperclip.update_team_member_role(uuid,uuid,paperclip.team_role) to authenticated;
revoke execute on function paperclip.remove_team_member(uuid,uuid) from public, anon;
grant  execute on function paperclip.remove_team_member(uuid,uuid) to authenticated;
revoke execute on function paperclip.decide_join_request(uuid,boolean,paperclip.team_role) from public, anon;
grant  execute on function paperclip.decide_join_request(uuid,boolean,paperclip.team_role) to authenticated;
revoke execute on function paperclip.create_join_request(uuid,text,text) from public, anon;
grant  execute on function paperclip.create_join_request(uuid,text,text) to authenticated;
