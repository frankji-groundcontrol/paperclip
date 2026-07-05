-- RLS (ENABLE, not FORCE — F1) + my_* views + grants. See 02-rls.md
-- NOTE: the `revoke select (key_hash|code_hash)` below is INEFFECTIVE against a
-- table-wide GRANT SELECT (a column revoke cannot subtract from it). Corrected in
-- migration 0006, which drops the blanket grant and re-grants non-secret columns.
alter table paperclip.users          enable row level security;
alter table paperclip.teams          enable row level security;
alter table paperclip.team_members   enable row level security;
alter table paperclip.api_keys       enable row level security;
alter table paperclip.invitations    enable row level security;
alter table paperclip.join_requests  enable row level security;
alter table paperclip.cli_auth       enable row level security;

-- READ policies
drop policy if exists users_self_read on paperclip.users;
create policy users_self_read on paperclip.users for select to authenticated
  using ( id = auth.uid()
          or id in (select tm.user_id from paperclip.team_members tm
                    where tm.team_id in (select paperclip_private.fn_user_team_ids(auth.uid()))) );

drop policy if exists teams_member_read on paperclip.teams;
create policy teams_member_read on paperclip.teams for select to authenticated
  using ( id in (select paperclip_private.fn_user_team_ids(auth.uid())) );

drop policy if exists team_members_member_read on paperclip.team_members;
create policy team_members_member_read on paperclip.team_members for select to authenticated
  using ( team_id in (select paperclip_private.fn_user_team_ids(auth.uid())) );

drop policy if exists api_keys_member_read on paperclip.api_keys;
create policy api_keys_member_read on paperclip.api_keys for select to authenticated
  using ( team_id in (select paperclip_private.fn_user_team_ids(auth.uid())) );

drop policy if exists invitations_admin_read on paperclip.invitations;
create policy invitations_admin_read on paperclip.invitations for select to authenticated
  using ( paperclip_private.fn_has_team_role(team_id, auth.uid(), array['owner','admin']::paperclip.team_role[]) );

drop policy if exists join_requests_admin_read on paperclip.join_requests;
create policy join_requests_admin_read on paperclip.join_requests for select to authenticated
  using ( paperclip_private.fn_has_team_role(team_id, auth.uid(), array['owner','admin']::paperclip.team_role[]) );
-- cli_auth: no select policy => deny-all direct reads.

-- Views (security_invoker => underlying RLS applies)
create or replace view paperclip.my_teams with (security_invoker = true) as
  select t.*, tm.role as my_role
  from paperclip.teams t
  join paperclip.team_members tm on tm.team_id = t.id and tm.user_id = auth.uid();

create or replace view paperclip.my_team_members with (security_invoker = true) as
  select tm.team_id, tm.user_id, tm.role, tm.created_at, u.email, u.display_name
  from paperclip.team_members tm
  join paperclip.users u on u.id = tm.user_id
  where tm.team_id in (select paperclip_private.fn_user_team_ids(auth.uid()));

create or replace view paperclip.my_api_keys with (security_invoker = true) as
  select id, team_id, created_by, subject_type, agent_id, name, prefix,
         scopes, scope_config, expires_at, last_used_at, revoked_at, created_at, meta
  from paperclip.api_keys
  where team_id in (select paperclip_private.fn_user_team_ids(auth.uid()));

create or replace view paperclip.my_invitations with (security_invoker = true) as
  select id, team_id, role, invited_email, allowed_join_types, expires_at,
         accepted_by, accepted_at, revoked_at, created_at, created_by
  from paperclip.invitations
  where paperclip_private.fn_has_team_role(team_id, auth.uid(), array['owner','admin']::paperclip.team_role[]);

-- Grants: SELECT on tables (writes via RPC only); column-level denies for secrets
grant select on paperclip.users, paperclip.teams, paperclip.team_members,
                 paperclip.api_keys, paperclip.invitations, paperclip.join_requests to authenticated;
grant select on paperclip.my_teams, paperclip.my_team_members,
                 paperclip.my_api_keys, paperclip.my_invitations to authenticated;
revoke select (key_hash) on paperclip.api_keys from authenticated;
revoke select (code_hash) on paperclip.invitations from authenticated;
revoke all on paperclip.cli_auth from authenticated, anon;
