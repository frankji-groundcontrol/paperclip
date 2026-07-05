# Paperclip Supabase Auth & Permissions — Implementation Plan

> Date: 2026-07-05 · Branch: `rewrite-nuxt-rust` (off `franky`) · Status: **DRAFT (pre eng-review)**
>
> Project: `hgyjvkuloaouxwdgromz` (supabase-**franky**, a dedicated multi-app project).
> Schemas to own: **`paperclip`** (exposed) + **`paperclip_private`** (internal). Both already exist and are **empty**.

## Goal (verbatim intent)

Build a real, tested identity/teams/api-key/permissions foundation for Paperclip on Supabase, at **full capability parity** with the original Paperclip TS auth (board/agent/user actors, api keys, CLI auth, memberships/roles, invites, join-requests, instance admin) **and better**, under a strict security model:

- **Custom schema only** — everything lives in `paperclip` / `paperclip_private`. **Never `public`.**
- **No service-role key.** The server uses only the **publishable/anon** key + the end user's JWT (or an API key resolved to a principal). RLS does the enforcement.
- **RLS + helper RPCs + views + `paperclip_private`** implement the permission system.
- **users / teams / api_keys** tables; **team-based access**; a **default (personal) team** auto-created per user.
- **API-key auth** (for MCP + CLI tools) in addition to email/password.
- **Server-mediated**: clients (browser, CLI, MCP) talk to *our* server; the server brokers Supabase. **No Supabase URL/keys/JWT ever reach the client.**
- Register + invite flows, proven with **real users** (no mocks).

## House pattern (franky reference — adopt the good, fix the weak)

Sibling apps here (`opconfig`, `qwenimage`, `memos`, `gbrain`, `docmind`) already follow a consistent shape. We **match the house style** for consistency, and **improve** two concrete weaknesses:

| Concern | Reference (opconfig/qwenimage/memos) | Our decision |
|---|---|---|
| Schema split | `<app>` exposed + `<app>_private` helpers | ✅ Same: `paperclip` / `paperclip_private` |
| API key storage | `prefix` + `key_hash = sha256(full_key)`; reveal plaintext once | ✅ Same, but **server-side generation** (qwenimage-style): plaintext is minted by the server, only its `sha256` reaches the DB. DB never sees plaintext. |
| API-key resolve | memos/opconfig pass **plaintext** to the DB RPC (`resolve_api_key(p_api_key)`) | ❌ Avoid. Use qwenimage-style **`resolve_api_key(prefix, key_hash)`** — server hashes, DB compares hashes only. |
| SECURITY DEFINER hardening | mixed: some `SET search_path='opconfig,extensions,pg_temp'`, memos uses `SET search_path=''` + fully-qualified | ✅ Adopt **`SET search_path=''` + fully-qualified** everywhere (most injection-resistant). |
| CLI device-login key delivery | opconfig stores minted key **plaintext at rest** in `cli_device_logins.api_key_full` | ❌ Smell. **Fix**: DB never stores the minted plaintext; the **server** delivers the approved key to the polling CLI via a short-TTL server-side cache keyed by `device_code_hash`. DB row tracks state + `api_key_id` (hash) only. (See `05`; hardening options in `09`.) |
| Invitations | qwenimage `invitations` lack a **target team + role** | ❌ Add **`team_id` + `role`** so "invite X to team T as admin" is first-class. |
| Roles | opconfig uses an enum; qwenimage/memos free-text | ✅ Proper enum `paperclip.team_role`. |
| New-user bootstrap | trigger on `auth.users` → create profile + personal team + owner membership | ✅ Same. |

## Security invariants (must always hold — verified by tests)

1. **No `public` schema objects.** All Paperclip objects in `paperclip`/`paperclip_private`.
2. **No service-role key** anywhere in server code, env, or tests. Server auth uses anon/publishable + user JWT + api-key resolution.
3. **RLS enabled** on every `paperclip.*` table; default-deny; a user sees only rows for teams they belong to. `paperclip_private.*` has **no grants** to `anon`/`authenticated` (only `SECURITY DEFINER` functions touch it).
4. **API keys / invite codes / device codes**: only **hashes** stored; plaintext revealed **once** at creation and never retrievable from the DB.
5. **Server-mediated**: the client never receives the Supabase URL, anon key, or a Supabase JWT. The server issues its own opaque session to the browser and holds the Supabase session server-side.
6. **Least privilege**: `authenticated` role can only invoke the whitelisted `paperclip.*` RPCs + read the `my_*` views; direct table writes are blocked/limited by RLS.

## Deliverables & phases

- **Phase A — DB foundation** (`01`–`03`, `06`): migrations for schema + RLS + `paperclip_private` helpers + `paperclip.*` RPCs + `auth.users` bootstrap trigger. Fully testable against real GoTrue users via the MCP `execute_sql` + REST.
- **Phase B — Server broker** (`04`, `05`): the auth broker in `backend-rs` (axum) — register/login/logout/whoami, API-key auth middleware, team/invite/key proxies. Client-facing; hides Supabase.
- **Phase C — Real-user test matrix** (`07`): register testuser3, invite→admin, api-key auth (MCP/CLI), RLS isolation, CLI device-login — with `test_users.json` (gitignored).

## Reference map (files)

- `01-schema.md` — DDL for `paperclip` + `paperclip_private`.
- `02-rls.md` — RLS policies + `my_*` views + grants.
- `03-rpcs.md` — `paperclip_private` helpers + `paperclip.*` RPCs (teams, keys, invites, whoami, bootstrap).
- `04-server-auth-broker.md` — backend-rs endpoints + session model + config (anon key only).
- `05-api-keys-and-cli-mcp.md` — API-key auth path + CLI/MCP device-login (no plaintext at rest).
- `06-invites-and-registration.md` — register + invite/accept state machines.
- `07-testing-and-real-users.md` — TDD strategy + real-user matrix + `test_users.json`.
- `08-migration-runbook.md` — ordered migrations, rollback, advisors/RLS verification.
- `09-open-questions-for-review.md` — decisions to lock with plan-eng-review.
- `10-parity-checklist.md` — original-Paperclip capability parity (filled from research).

## Execution model

Plan → **plan-eng-review** (lock architecture/security/edge-cases) → **codex (gpt-5.5, high reasoning) TDD** implements each unit red→green → I verify diffs/tests → **real-user tests** with the four test accounts. Nothing is "done" without a real request against real users.
