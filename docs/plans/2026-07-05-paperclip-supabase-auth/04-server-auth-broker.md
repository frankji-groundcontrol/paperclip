# 04 — Server Auth Broker (`backend-rs`, axum)

The **only** thing that talks to Supabase. Clients (browser, CLI, MCP) talk to *this*. No Supabase URL/key/JWT ever crosses to the client.

## Config (server-only env — never shipped to client)
```
PAPERCLIP_SUPABASE_URL      = https://hgyjvkuloaouxwdgromz.supabase.co
PAPERCLIP_SUPABASE_ANON_KEY = sb_publishable_<paperclip>   # publishable/anon ONLY
# NO service-role key. If one is ever present, boot-time assertion fails.
PAPERCLIP_SESSION_SECRET    = <hs256 secret for our own opaque session cookies>
```
Boot assertion: refuse to start if any env var named like `*SERVICE_ROLE*` / `*SECRET_KEY*` (Supabase secret key) is set → enforces "no service-role".

## Two auth inputs → one `Principal`
```
Principal = User { user_id, email, teams:[{team_id, role}] , system_role }
          | Agent { api_key_id, team_id, agent_id, scopes, scope_config }
```
1. **Browser/session** → user JWT (held server-side). Server calls Supabase REST/RPC with `Authorization: Bearer <user_jwt>` + `apikey: <anon>`; RLS applies.
2. **API key** (MCP/CLI) → header `Authorization: Bearer paperclip_<...>`. Server computes `prefix,sha256`, calls `paperclip.resolve_api_key(prefix, hash)` (anon key) → principal context. Then acts on the DB with the anon key + a **service JWT minted from the resolved user**? No — RLS needs a JWT with `sub=user_id`. Options in `09`; default: after resolve, the server uses a **short-lived signed JWT it mints for that user** using the project's JWT secret **is a service secret we don't have**. → So instead, API-key requests execute via **`SECURITY DEFINER` RPCs only** (which take the resolved context explicitly and self-authorize), never via raw RLS reads. This keeps us off the service key. (Locked in `09`.)

## Endpoints (client-facing; all return app-shaped JSON, no Supabase leakage)
| Method | Path | Auth | Supabase call |
|---|---|---|---|
| POST | `/api/auth/register` | none | GoTrue `POST /auth/v1/signup` (anon) → trigger bootstraps team; return app session |
| POST | `/api/auth/login` | none | GoTrue `POST /auth/v1/token?grant_type=password` (anon) → store supabase session server-side, set opaque `pc_session` cookie |
| POST | `/api/auth/logout` | session | drop server session; GoTrue `logout` |
| GET  | `/api/auth/whoami` | session or api-key | `paperclip.whoami()` (JWT) / from resolved principal |
| GET/POST | `/api/teams` | session | `my_teams` view / `create_team` |
| POST | `/api/teams/:id/members` | session | `add_team_member` |
| POST | `/api/teams/:id/invitations` | session | server mints code → `create_invitation(code_hash)` → returns code once |
| POST | `/api/invitations/accept` | session | server hashes code → `accept_invitation(code_hash)` |
| GET/POST/DELETE | `/api/api-keys` | session | server mints key → `create_api_key` / `my_api_keys` / `revoke_api_key` |
| POST | `/api/cli-auth/start` | none | server hashes CLI-provided hashes → `cli_start_device_login` |
| POST | `/api/cli-auth/approve` | session | `cli_approve_device_login(user_code_hash)` |
| GET  | `/api/cli-auth/poll` | none | `cli_poll_device_login(secret_hash)` |

## Session model
- On login, the server holds the Supabase `access_token`+`refresh_token` in a server-side session store keyed by an opaque cookie/bearer `pc_session`. The client only ever sees `pc_session`. Refresh handled server-side.
- Middleware resolves `pc_session` → user JWT (for REST/RPC), or `paperclip_*` api key → `resolve_api_key` → agent principal.

## Modules (Rust)
`backend-rs/src/auth/` : `supabase_client.rs` (thin REST/RPC over reqwest, anon key), `session.rs` (server session store), `keys.rs` (mint/format/hash api keys), `principal.rs` (extractor), `routes.rs` (endpoints). Unit-tested with a mock Supabase transport; integration-tested against the real project.
