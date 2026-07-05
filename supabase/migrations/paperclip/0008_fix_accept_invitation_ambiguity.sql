-- BUGFIX: accept_invitation RETURNS TABLE(team_id,...) makes `team_id` an OUT
-- variable, so ON CONFLICT (team_id, user_id) was ambiguous (42702). Resolve
-- unqualified identifiers to columns. (CREATE OR REPLACE preserves ACL from 0007.)
create or replace function paperclip.accept_invitation(p_code_hash text)
returns table(team_id uuid, role paperclip.team_role)
language plpgsql security definer set search_path = '' as $$
#variable_conflict use_column
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
    if lower(v_email) <> lower(v_inv.invited_email::text) then
      raise exception 'invitation is for a different email' using errcode='42501'; end if;
  end if;
  insert into paperclip.team_members(team_id, user_id, role, added_by)
  values (v_inv.team_id, v_uid, v_inv.role, v_inv.created_by)
  on conflict (team_id, user_id) do update set role = excluded.role;
  update paperclip.invitations set accepted_by = v_uid, accepted_at = now() where id = v_inv.id;
  team_id := v_inv.team_id; role := v_inv.role; return next;
end $$;

revoke execute on function paperclip.accept_invitation(text) from public, anon;
grant  execute on function paperclip.accept_invitation(text) to authenticated;
