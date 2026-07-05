# 11 — Engineering Review (plan-eng-review lens, applied autonomously)

Reviewer stance: eng-manager. Dimensions: architecture, security, correctness, tests, edge cases. Decisions made autonomously per user directive.

## Findings (severity · decision)

### F1 · SECURITY-HIGH · RLS recursion via `FORCE` + helper reading the same table  ← the real landmine
`team_members_member_read` calls `paperclip_private.fn_user_team_ids(auth.uid())`, which `SELECT`s `paperclip.team_members`. If RLS is **forced**, the helper (even as SECURITY DEFINER) is itself subject to the `team_members` SELECT policy → **infinite recursion** (`42P17`). Same trap Supabase users hit constantly.
- **Decision:** `ENABLE` RLS on all `paperclip.*` tables but **do NOT `FORCE`**. Tables are owned by the migration/admin role (`postgres`, `bypassrls`); the API roles `anon`/`authenticated` are non-owners so `ENABLE` alone constrains them. SECURITY DEFINER helpers run as the owner and thus **bypass** RLS on the base tables, breaking the recursion. `02` and `08` updated; `07` adds a recursion regression test (A0).
- Guardrail: helpers that RLS policies depend on (`fn_user_team_ids`, `fn_has_team_role`) MUST be owned by a `bypassrls` role and never re-enter a forced policy.

### F2 · SECURITY-HIGH · API-key execution path without a service role (09 Q1) — locked
API-key (MCP/CLI) requests cannot present a Supabase JWT (we can't mint one — no JWT secret, no service role). **Decision:** api-key sessions perform **all** data access through `SECURITY DEFINER` RPCs that take the resolved principal `(user_id/agent_id, team_id, scopes)` explicitly and **re-verify** membership/scope inside the function. No raw RLS reads on the api-key path. For this milestone the only api-key operations are identity/team/key/whoami — already RPCs — so the surface is bounded. Domain-data RPCs added later follow the same contract. Server rate-limits `resolve_api_key` by prefix+IP.

### F3 · SECURITY-MED · `resolve_api_key`/`cli_start_device_login` granted to `anon`
Unauthenticated callers can probe. Safe because they require a full 256-bit key hash (infeasible to brute), but they perform writes (`last_used_at`, row insert). **Decision:** keep the grants (server is the only caller, network-gated), add server-side rate limiting, keep the `cli_auth` GC (`delete where expires_at < now()-1d`), 10-min TTL. Note the risk; don't over-engineer.

### F4 · CORRECTNESS-MED · CLI-generated `prefix` collision on approve
CLI generates `pending_key` + prefix locally; `api_keys.prefix` is UNIQUE. A collision makes `cli_approve_device_login` INSERT fail. Prob ~birthday over 2^32. **Decision:** `cli_approve_device_login` catches `unique_violation` → returns a retryable error; CLI regenerates. Documented in `03`/`05`.

### F5 · CORRECTNESS-MED · `delete_team` must not orphan a user's default team
FK `users.default_team_id → teams on delete set null` would leave a user with no default. **Decision:** `delete_team` blocks deleting a `is_personal` team and any team that is someone's `default_team_id` unless reassigned. Documented in `03`.

### F6 · TESTS-MED · RLS testing needs a real `auth.uid()`
`execute_sql` runs as a privileged role, so RLS isn't exercised. **Decision:** two paths (added to `07`):
- **DB unit:** `set local role authenticated; select set_config('request.jwt.claims', json_build_object('sub','<uid>','role','authenticated')::text, true);` then run — exercises `auth.uid()`.
- **Real-user acceptance:** GoTrue password-login each account → real JWT → call PostgREST `/rest/v1/rpc/*` + views with that JWT. This is the true isolation proof.

### F7 · TESTS-MED · Missing assertions
Add: A0 recursion smoke (querying `my_teams`/`team_members` as an authenticated user does NOT raise `42P17`); bootstrap idempotency (second `bootstrap_current_user` ⇒ no second team); `resolve_api_key` wrong/revoked ⇒ empty/err. Folded into `07`.

### F8 · SECURITY-MED · `key_hash`/`code_hash` column exposure — confirmed handled
`grant select` then `revoke select (key_hash)`/`(code_hash)`; `my_*` views omit them. `08` verifies via `has_column_privilege`. Keep.

### F9 · CORRECTNESS-LOW · privilege escalation check — clean
No RPC sets `users.system_role`; a user can only own teams they create. Instance-admin is out-of-band (seeded). No escalation path. Keep `system_role` seeded manually (09 Q2).

## Scope verdict
Boring-by-default (Supabase Auth + RLS + pgcrypto + SECURITY DEFINER — all Layer 1). No innovation token spent unwisely. Strangler-fig: this is the identity foundation; companies/agents/domain data layer on top of `team_id` later. `join_requests` kept for parity but minimal. Scope is the auth foundation — not creep.

## VERDICT
Plan is sound after the F1 recursion fix (the one true blocker) and the F4/F5 edge-case handlers. Security invariants hold: no `public`, no service-role, RLS constrains API roles, secrets hash-only, private schema ungranted. **Approved to implement** with the `02`/`03`/`07`/`08` amendments below applied.
