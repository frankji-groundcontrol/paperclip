# 08 — Migration Runbook

Migrations authored under `backend-rs/` or `supabase/migrations/` and applied to `hgyjvkuloaouxwdgromz` via the **supabase-franky** MCP `apply_migration` (idempotent, transactional per file).

## Order
1. `0001_paperclip_schema` — schemas, enums, tables, indexes (`01`). Includes `pending_key_prefix` on `cli_auth`.
2. `0002_paperclip_rls` — enable+force RLS, read policies, views, grants (`02`).
3. `0003_paperclip_private_helpers` — private helper fns (`03`).
4. `0004_paperclip_rpcs` — public RPCs + execute grants (`03`).
5. `0005_bootstrap_trigger` — `auth.users` trigger (`03`).

## Pre-flight
- Confirm `paperclip`/`paperclip_private` empty (already verified).
- Confirm `extensions.digest`, `extensions.gen_random_bytes`, `extensions.citext` resolve.

## Post-migration verification (run as SQL; must pass before app work)
```sql
-- every table RLS ENABLED (relrowsecurity=true); NOT forced (relforcerowsecurity
-- must be false so SECURITY DEFINER helpers bypass and RLS doesn't recurse — F1)
select relname, relrowsecurity, relforcerowsecurity
from pg_class where relnamespace = 'paperclip'::regnamespace and relkind='r';
-- expect: relrowsecurity=true, relforcerowsecurity=false for every table.
-- private schema not usable by authenticated
select has_schema_privilege('authenticated','paperclip_private','USAGE');  -- expect false
-- no paperclip objects in public
select count(*) from information_schema.tables where table_schema='public' and table_name like '%paperclip%'; -- 0
-- key_hash not selectable by authenticated
select has_column_privilege('authenticated','paperclip.api_keys','key_hash','SELECT'); -- expect false
```
- Run `get_advisors(security)` and `get_advisors(performance)`; resolve every **security** finding (missing RLS, SECURITY DEFINER without search_path, etc.) before proceeding.

## Rollback
- Each migration paired with a `down` (drop policies/functions/tables in reverse). Because `paperclip*` schemas are app-owned and were empty, a full teardown = `drop schema paperclip cascade; drop schema paperclip_private cascade;` (dev only).

## Idempotency
- All `create ... if not exists` / `create or replace`. Re-applying a migration is a no-op. `apply_migration` names must be unique + snake_case.
