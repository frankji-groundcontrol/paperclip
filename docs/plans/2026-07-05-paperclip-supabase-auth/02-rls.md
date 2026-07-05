# 02 — RLS Policies, Views, Grants

Principle: **default-deny**. `authenticated` reads its own rows via policies and the `my_*` views; **all writes go through `SECURITY DEFINER` RPCs** (`03`) — so table-level write policies are intentionally absent (deny-by-default) except where a direct write is genuinely safe. `anon` gets no table access; it can only call the whitelisted pre-auth RPCs. `paperclip_private` is never granted.

Migration `0002_paperclip_rls`.

```sql
-- Enable RLS everywhere ------------------------------------------------------
-- ENABLE (not FORCE). Tables are owned by the migration/admin role (postgres,
-- has bypassrls); the API roles anon/authenticated are non-owners, so ENABLE
-- alone fully constrains them. SECURITY DEFINER helpers run as the owner and
-- thus BYPASS RLS on the base tables — which is REQUIRED to avoid infinite
-- recursion (42P17): the team_members SELECT policy calls fn_user_team_ids(),
-- which itself SELECTs team_members. If we FORCED RLS, that helper would re-enter
-- the policy → recursion. So: do NOT FORCE. (See 11-eng-review F1.)
alter table paperclip.users          enable row level security;
alter table paperclip.teams          enable row level security;
alter table paperclip.team_members   enable row level security;
alter table paperclip.api_keys       enable row level security;
alter table paperclip.invitations    enable row level security;
alter table paperclip.join_requests  enable row level security;
alter table paperclip.cli_auth       enable row level security;
-- Do NOT `alter table ... force row level security` on any table whose RLS
-- policy transitively reads that same table via a SECURITY DEFINER helper.

-- READ policies (SELECT) — a user sees itself + teams it belongs to ---------
create policy users_self_read on paperclip.users
  for select to authenticated
  using ( id = auth.uid()
          or id in (select tm.user_id from paperclip.team_members tm
                    where tm.team_id in (select paperclip_private.fn_user_team_ids(auth.uid()))) );

create policy teams_member_read on paperclip.teams
  for select to authenticated
  using ( id in (select paperclip_private.fn_user_team_ids(auth.uid())) );

create policy team_members_member_read on paperclip.team_members
  for select to authenticated
  using ( team_id in (select paperclip_private.fn_user_team_ids(auth.uid())) );

create policy api_keys_member_read on paperclip.api_keys
  for select to authenticated
  using ( team_id in (select paperclip_private.fn_user_team_ids(auth.uid())) );

create policy invitations_admin_read on paperclip.invitations
  for select to authenticated
  using ( paperclip_private.fn_has_team_role(team_id, auth.uid(), array['owner','admin']::paperclip.team_role[]) );

create policy join_requests_admin_read on paperclip.join_requests
  for select to authenticated
  using ( paperclip_private.fn_has_team_role(team_id, auth.uid(), array['owner','admin']::paperclip.team_role[]) );

-- cli_auth: no direct SELECT for anyone; only RPCs touch it (deny-all).

-- NO write policies -> INSERT/UPDATE/DELETE denied for anon/authenticated on
-- every table. Mutations happen exclusively through SECURITY DEFINER RPCs (03),
-- which enforce role checks. (This is stricter and simpler than per-op policies.)
```

## Views — `my_*` (safe projections; never expose secrets)

```sql
create view paperclip.my_teams
  with (security_invoker = true) as
  select t.*, tm.role as my_role
  from paperclip.teams t
  join paperclip.team_members tm
    on tm.team_id = t.id and tm.user_id = auth.uid();

create view paperclip.my_team_members
  with (security_invoker = true) as
  select tm.team_id, tm.user_id, tm.role, tm.created_at, u.email, u.display_name
  from paperclip.team_members tm
  join paperclip.users u on u.id = tm.user_id
  where tm.team_id in (select paperclip_private.fn_user_team_ids(auth.uid()));

create view paperclip.my_api_keys
  with (security_invoker = true) as
  select id, team_id, created_by, subject_type, agent_id, name, prefix,
         scopes, scope_config, expires_at, last_used_at, revoked_at, created_at, meta
  from paperclip.api_keys                    -- NOTE: key_hash intentionally excluded
  where team_id in (select paperclip_private.fn_user_team_ids(auth.uid()));

create view paperclip.my_invitations
  with (security_invoker = true) as
  select id, team_id, role, invited_email, allowed_join_types, expires_at,
         accepted_by, accepted_at, revoked_at, created_at, created_by
  from paperclip.invitations                 -- NOTE: code_hash excluded
  where paperclip_private.fn_has_team_role(team_id, auth.uid(), array['owner','admin']::paperclip.team_role[]);
```

## Grants

```sql
-- Views run security_invoker => underlying RLS still applies. Grant select.
grant select on paperclip.my_teams, paperclip.my_team_members,
                 paperclip.my_api_keys, paperclip.my_invitations to authenticated;

-- Tables: SELECT only (writes via RPC). No INSERT/UPDATE/DELETE grants.
grant select on paperclip.users, paperclip.teams, paperclip.team_members,
                 paperclip.api_keys, paperclip.invitations, paperclip.join_requests
  to authenticated;
-- api_keys select excludes key_hash at the app layer; also consider column-level:
revoke select (key_hash) on paperclip.api_keys from authenticated;   -- defense in depth
revoke select (code_hash) on paperclip.invitations from authenticated;
revoke all on paperclip.cli_auth from authenticated, anon;

-- Execute grants for RPCs are in 03 (per-function).
```

## Security-test assertions (must pass)
- As `authenticated` user A: cannot `select` a team A is not a member of (0 rows). ✅
- As A: cannot `select paperclip.api_keys.key_hash` (column privilege denied). ✅
- As `anon`: cannot select any `paperclip.*` table; can only `execute` the whitelisted pre-auth RPCs. ✅
- `paperclip_private.*`: `has_schema_privilege('authenticated','paperclip_private','USAGE') = false`. ✅
- Every `paperclip.*` table has `relrowsecurity = true` (RLS **enabled**; intentionally **not forced** — see F1). ✅
- **Positive isolation test** (proves ENABLE is enough): as a *simulated authenticated user* (`set local role authenticated` + `request.jwt.claims.sub`), a `select` on a team the user isn't in returns **0 rows** and querying `my_teams` does **not** raise `42P17`. ✅ (This replaces relying on FORCE.)
