# RESULTS — Paperclip custom-schema auth (applied + proven)

Status as of 2026-07-05. DB layer applied to the real **supabase-franky** project
(`hgyjvkuloaouxwdgromz`), schemas `paperclip` / `paperclip_private`. **No `public`
schema, no service-role key.** Backend broker (`backend-rs`) implemented separately per
[`12-backend-broker-spec.md`](12-backend-broker-spec.md).

## What shipped (DB)
Migrations `0001`–`0010` in [`supabase/migrations/paperclip/`](../../../supabase/migrations/paperclip/):
7 tables (`users, teams, team_members, api_keys, invitations, join_requests, cli_auth`),
3 enums, private `SECURITY DEFINER` helpers, RLS + `my_*` views, 19 public RPCs, and the
`auth.users` bootstrap trigger.

## Findings caught by verification (each fixed, re-verified)
1. **Column secrecy (0006, SECURITY).** The plan's `revoke select (key_hash)` after a
   table-wide `grant select` was a **no-op** — Postgres `has_column_privilege` returns true
   if the relation-level grant covers the column. Live check showed `key_hash` still
   selectable. Fixed by dropping the blanket grant and granting only non-secret columns.
   Re-verified: `key_hash`/`code_hash` **not** selectable; `prefix`/`name`/`role` are.
2. **Least-privilege execute (0007, SECURITY).** `CREATE FUNCTION` grants `EXECUTE` to
   `PUBLIC` by default, so `anon` could call every RPC (all self-guard on `auth.uid()`, but
   least-privilege demanded a fix). Reset + re-grant: `anon` executes **only**
   `resolve_api_key`, `cli_start_device_login`, `cli_poll_device_login`; the other 16 are
   `authenticated`-only. Verified via `has_function_privilege`.
3. **`accept_invitation` ambiguity (0008, BUG).** `RETURNS TABLE(team_id …)` made
   `ON CONFLICT (team_id, …)` ambiguous (`42702`), so accepts failed with HTTP 400. Fixed
   with `#variable_conflict use_column`. (Found by the acceptance run, not by reading.)

## Pre-commit adversarial review (8 confirmed defects, all fixed — 0009/0010)
A multi-agent security/correctness review of the broker + migrations before committing found
defects the happy-path acceptance never exercised. All fixed and re-proven:
- **CRITICAL** `cli_approve_device_login` minted an API key for a **caller-supplied team with no
  membership check** → cross-team credential theft. Fixed: approver must be a member of a
  requested team (ATK1).
- **HIGH** `add_team_member` upsert could demote/remove an owner; `decide_join_request` let an
  admin grant `owner`; `update_team_member_role`/`remove_team_member` let an admin act on owners.
  Fixed: only an owner may modify/remove an owner or grant owner (ATK2–ATK5).
- **HIGH/LOW (broker)** logout not fail-safe on GoTrue error; failed-refresh left a dead session;
  `SystemTime` overflow panic; bearer tokens in `Debug`. Fixed with new unit tests
  (`logout_is_failsafe_when_signout_errors`, `failed_refresh_evicts_dead_session`) + token redaction.
- **0010** corrects a regression 0009 introduced (over-broad revoke of the RLS/view helper EXECUTE).

## Acceptance matrix — 25/25 PASS (real PostgREST + genuine GoTrue JWTs; incl. attack matrix)
Driver: [`acceptance/run_acceptance.py`](acceptance/run_acceptance.py). Accounts:
`testuser1/2`, `testadmin1`, `testuser3` (registered during the test). Password in the
gitignored `test_users.json`.

| # | Scenario | Result |
|---|----------|--------|
| A0 | RLS recursion smoke (`my_teams`, `team_members`) | PASS — no `42P17` |
| A1 | bootstrap idempotency (2× → one personal team) | PASS |
| A2 | register testuser3 via GoTrue → auto profile+team | PASS |
| A3 | testadmin1 creates "Paperclip Test Team" | PASS |
| A4 | invite testuser3 as **admin** (code hashed) | PASS |
| A5 | testuser3 accepts → admin member | PASS |
| A6 | RLS isolation: non-members see 0 rows | PASS (a/b/c) |
| A7 | mint user API key; `key_hash` never exposed | PASS |
| A8 | `resolve_api_key` correct; wrong hash → empty | PASS |
| A9 | agent key (`agent_id` + `scope_config`) resolves | PASS |
| A10 | CLI device-login start→approve→poll→resolve | PASS |
| A11 | `key_hash` column blocked; anon cannot `create_team` | PASS |
| A12 | revoke → resolve fails | PASS |
| A13 | role gates: admin invites member not owner; viewer cannot mint | PASS |
| ATK1 | non-member cannot mint a cross-team key via device-login | BLOCKED |
| ATK2 | admin cannot demote the owner via `add_team_member` | BLOCKED |
| ATK3 | admin cannot change the owner's role | BLOCKED |
| ATK4 | admin cannot remove the owner | BLOCKED |
| ATK5 | admin cannot grant `owner` by approving a join request | BLOCKED |

Durable proof left in the project: team `Paperclip Test Team` with **owner** testadmin1,
**admin** testuser3 (via accepted invite), **viewer** testuser2.

## Security invariants (verified on the live project)
- `paperclip` + `paperclip_private` only; **0** `public` paperclip objects.
- `paperclip_private` USAGE = false for `authenticated`.
- RLS **ENABLED, not FORCED** on all 7 tables (avoids the `SECURITY DEFINER` recursion trap).
- Secrets (`key_hash`, `code_hash`) column-unreadable; `cli_auth` fully denied to anon/authenticated.
- No service-role key anywhere; server path uses anon key + user JWT only.
- Supabase `get_advisors(security)`: remaining paperclip warnings are the expected
  `authenticated_security_definer_function_executable` set (our RPC write-model, by design)
  plus one `rls_enabled_no_policy` INFO on `cli_auth` (deny-all by design).
