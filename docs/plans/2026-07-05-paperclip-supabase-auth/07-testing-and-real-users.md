# 07 — TDD Strategy + Real-User Test Matrix

> "Full implementation and full test — do not fool me." Every capability is proven against the **real** Supabase project and **real** GoTrue users. No mocks in acceptance.

## Test layers
1. **DB unit (pgTAP / SQL assertions)** — codex writes, run via `execute_sql`. Red→green per function: helpers, RLS deny/allow, RPC role gates, bootstrap idempotency, api-key hash-only, invite state machine, cli-auth flow.
2. **Rust unit (`cargo test`)** — key mint/hash/format, session store, principal extractor (mock transport).
3. **Real-user integration** — driven by me via the MCP + GoTrue REST, using the four accounts. This is the acceptance gate.

## How to exercise RLS as a specific user (F6)
`execute_sql` runs privileged, so it must **impersonate** a user to test RLS. Wrap each RLS assertion in one transaction:
```sql
begin;
set local role authenticated;
select set_config('request.jwt.claims',
       json_build_object('sub', '<auth_uid>', 'role', 'authenticated')::text, true);
-- ... queries here see auth.uid() = <auth_uid>, RLS applies ...
rollback;   -- (or commit for writes)
```
- **Real-user acceptance** (stronger): GoTrue password-login each account → real `access_token` (JWT) → call PostgREST `POST {url}/rest/v1/rpc/<fn>` and `GET {url}/rest/v1/my_*` with headers `apikey: <anon>`, `Authorization: Bearer <jwt>`. This proves end-to-end isolation with a genuine session (no impersonation shortcut).

## Real accounts
- `testuser1_paperclip@gmail.com`, `testuser2_paperclip@gmail.com`, `testadmin1_paperclip@gmail.com` — **exist, confirmed**. Password `<stored only in gitignored test_users.json>`.
- `testuser3_paperclip@gmail.com` — **register during test** (register-flow proof).

## Acceptance matrix (each must PASS with real data)
| # | Scenario | Assertion |
|---|---|---|
| A0 | **RLS recursion smoke** (F1): as a simulated authenticated user, `select * from paperclip.my_teams` and `select * from paperclip.team_members` | returns rows, **never raises `42P17` infinite recursion** |
| A1 | Bootstrap: each existing user has a personal team + owner membership (via `bootstrap_current_user` if trigger predates them); calling it **twice** creates no 2nd team (idempotent) | `my_teams` shows exactly one personal team, role `owner` |
| A2 | Register testuser3 via GoTrue signup (anon key) | `auth.users` row created; `paperclip.users` + personal team auto-bootstrapped |
| A3 | testadmin1 creates a team "Paperclip Test Team" | team + owner membership created |
| A4 | testadmin1 invites testuser3 as **admin** to that team | invitation row (code hashed); code returned once |
| A5 | testuser3 accepts the invite | `team_members(team, testuser3, admin)`; visible in `my_team_members` |
| A6 | RLS isolation: testuser1 (not a member) cannot see the team or its members | 0 rows |
| A7 | testadmin1 mints an API key on the team | `api_keys` row; `full_key` returned once; **`key_hash` never selectable** |
| A8 | API-key auth: present the key → `resolve_api_key` returns the correct team/subject; `last_used_at` bumped | resolves; wrong/revoked key → error |
| A9 | Agent key: mint `subject_type=agent` key with `agent_id`+`scope_config`; resolve returns them | parity with agent_api_keys |
| A10 | CLI device-login: start → approve (as testuser1) → poll approved; the pending key resolves | end-to-end; DB holds only hashes (assert no plaintext column) |
| A11 | Security: `paperclip_private` has no `USAGE` for authenticated; no `public` Paperclip objects; every table RLS forced; no service-role key in env | all true |
| A12 | Revoke: revoke the API key → subsequent `resolve_api_key` fails | revoked path |
| A13 | Role gate: testuser3 (admin) can invite a `member` but cannot invite `owner`; a `viewer` cannot mint keys | errcode 42501 |

## `test_users.json` (gitignored)
```json
{
  "project_url": "https://hgyjvkuloaouxwdgromz.supabase.co",
  "password": "<stored only in gitignored test_users.json>",
  "users": [
    {"email":"testuser1_paperclip@gmail.com","auth_uid":"…","default_team_id":"…"},
    {"email":"testuser2_paperclip@gmail.com","auth_uid":"…","default_team_id":"…"},
    {"email":"testadmin1_paperclip@gmail.com","auth_uid":"…","default_team_id":"…","is_team_admin":true},
    {"email":"testuser3_paperclip@gmail.com","auth_uid":"…","registered_in_test":true,"invited_role":"admin"}
  ],
  "team": {"id":"…","name":"Paperclip Test Team"},
  "api_key_prefix_example":"paperclip_xxxxxxxx"
}
```
Add `test_users.json` and `**/test_users.json` to `.gitignore` **before** writing it. Never commit plaintext keys/passwords beyond this ignored file.
