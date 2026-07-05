# Paperclip auth migrations (custom-schema)

Versioned SQL for the Paperclip authentication + permissions system, applied to the
dedicated **supabase-franky** project (`hgyjvkuloaouxwdgromz`) in the `paperclip` /
`paperclip_private` schemas. **No `public` schema. No service-role key.** Design and
rationale live in [`docs/plans/2026-07-05-paperclip-supabase-auth/`](../../../docs/plans/2026-07-05-paperclip-supabase-auth/).

## Apply order
| # | File | Purpose |
|---|------|---------|
| 0001 | `0001_schema.sql` | schemas, enums, 7 tables, indexes; `paperclip_private` ungranted; `paperclip` usage to anon/authenticated |
| 0002 | `0002_private_helpers.sql` | `SECURITY DEFINER` helpers (`fn_user_team_ids`, `fn_has_team_role`, `fn_bootstrap_auth_user`, trigger fn) + `updated_at` touch triggers |
| 0003 | `0003_rls_views_grants.sql` | RLS **ENABLE (not FORCE — F1)**, read policies, `my_*` security_invoker views, table/view grants |
| 0004 | `0004_rpcs.sql` | public RPCs (identity, teams, members, api-keys, invitations, join-requests, CLI device-login) + initial execute grants |
| 0005 | `0005_bootstrap_trigger.sql` | `after insert on auth.users` → profile + personal team + owner membership |
| 0006 | `0006_fix_column_secrecy.sql` | **security fix**: a table-wide `GRANT SELECT` overrides a per-column `REVOKE`, so `key_hash`/`code_hash` stayed readable — re-grant non-secret columns only |
| 0007 | `0007_execute_grants_least_privilege.sql` | **security fix**: `CREATE FUNCTION` grants EXECUTE to `PUBLIC` by default — reset all, re-grant `authenticated` broadly and `anon` to only the 3 pre-auth RPCs |
| 0008 | `0008_fix_accept_invitation_ambiguity.sql` | **bug fix**: `accept_invitation` `RETURNS TABLE(team_id …)` made `ON CONFLICT (team_id, …)` ambiguous (42702) → `#variable_conflict use_column` |
| 0009 | `0009_authz_hardening.sql` | **authz fixes** from pre-commit adversarial review: cli_approve cross-team key mint (CRITICAL), owner-demotion via add_team_member, admin-grants-owner via decide_join_request, admin acting on owners, join-request spam. Proven blocked by ATK1–ATK5. |
| 0010 | `0010_fix_rls_helper_grants.sql` | **fix for 0009**: restore EXECUTE on the two private helpers (`fn_user_team_ids`, `fn_has_team_role`) that RLS policies / `my_*` views invoke as the caller — 0009's blanket revoke had broken all view/RLS reads. |
| 0011 | `0011_companies_jobs.sql` | **companies + jobs**: team-scoped `companies` and LLM-backed `jobs` (`running/succeeded/failed`), RLS, `my_companies`/`my_jobs`, session RPCs + **key-credential** `*_with_key` RPCs (api-key data path, `fn_key_principal` explicit-revoked), idempotency `client_token`, `fail_stale_jobs` reaper. See [`docs/plans/2026-07-05-real-company-real-job/`](../../../docs/plans/2026-07-05-real-company-real-job/). |
| 0012 | `0012_api_key_scopes.sql` | **agent-key scopes**: `fn_key_allows` gates the `*_with_key` RPCs on `companies:write/read`, `jobs:write/read` (`*` = all). Backward compatible — an unscoped key (`[]`) keeps full access; a scoped key is restricted. |

Apply with the Supabase MCP `apply_migration` (or `supabase db push`) **in order**. Each is
idempotent (`create … if not exists` / `create or replace`).

## Post-apply verification (must pass)
- Every `paperclip.*` table: `relrowsecurity = true`, `relforcerowsecurity = false`.
- `has_schema_privilege('authenticated','paperclip_private','USAGE') = false`.
- `has_column_privilege('authenticated','paperclip.api_keys','key_hash','SELECT') = false`.
- `has_column_privilege('authenticated','paperclip.invitations','code_hash','SELECT') = false`.
- `has_function_privilege('anon', fn, 'EXECUTE')` true **only** for `resolve_api_key`,
  `cli_start_device_login`, `cli_poll_device_login`.
- 0 objects in `public` matching `paperclip`.

## Acceptance
Real-user, real-JWT matrix (20/20) documented in
[`RESULTS.md`](../../../docs/plans/2026-07-05-paperclip-supabase-auth/RESULTS.md); reproduce with
[`acceptance/run_acceptance.py`](../../../docs/plans/2026-07-05-paperclip-supabase-auth/acceptance/run_acceptance.py).
