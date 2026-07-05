# RESULTS — real user → real company → real job (proven)

Status 2026-07-05. A **real user forms a real company and runs a real OpenAI job**, end to end, on the
`paperclip` custom schema, through the **agent (API-key) path** — via HTTP, the `paperclip` CLI, and the
`paperclip-mcp` MCP server. No mocks in acceptance. OpenAI uses the **Responses API** (`/v1/responses`).

## What shipped
- **DB** (migration `0011_companies_jobs`): `paperclip.companies` + `paperclip.jobs` (status
  `running|succeeded|failed`), RLS (enable-not-force), `my_companies`/`my_jobs` views, session RPCs
  (`auth.uid()`), and **key-credential RPCs** (`*_with_key`) that re-resolve the key internally
  (`fn_key_principal`, explicit-revoked). Idempotency via `client_token`; `fail_stale_jobs` reaper.
- **Backend** (`backend-rs`, codex TDD + hardening): `src/llm` OpenAI **Responses SSE** client — pure
  `parse_responses_sse` (accumulate `output_text.delta`, usage from `response.completed`, terminal-event
  required) + `OpenAiResponsesClient` with a **rate-limit retry/backoff**; `DataGateway` (paperclip
  PostgREST); `JobService` (api-key path: create job → OpenAI → complete/fail, sanitized errors);
  routes under **`/api/paperclip/`** (no collision with the legacy in-memory `/api/companies`);
  `app_from_env()` wiring (`main.rs` serves it).
- **Agent interfaces**: `paperclip` CLI (`company create/list`, `job run/list`, `config set-key`);
  `paperclip-mcp` stdio MCP server (`initialize`/`tools/list`/`tools/call`, 4 tools); skill
  `skills/paperclip-agent/SKILL.md`.

## Acceptance — HTTP agent path: 13/13
`acceptance/run_jobs_acceptance.py` (real key, real OpenAI, DB-verified):
E1 form company (+DB row on user's team) · E2/E3 **run job → real result `4`, persisted succeeded**
(subject=user, model=gpt-5.4-mini) · E2b/E2c rejected-model → **failed + sanitized error** · E5
substantive job → real 212-char sentence · E4 list · E8 RLS isolation · E9 cross-team blocked (42501)
· E10a malformed→401 / E10b bogus key→28000 · E12 **no Supabase/OpenAI secret leak**.

## Acceptance — CLI + MCP: 8/8
`acceptance/run_cli_mcp_acceptance.py` (drives the built binaries):
E6 CLI `company create` + `job run` → prints real `7`, DB `succeeded` · E7 MCP `initialize`
(protocol 2024-11-05) + `tools/list` (4 tools) + `tools/call` create_company & run_job → real `7`,
DB persisted.

## DB unit acceptance (api-key data path): 9/9
`/tmp` PostgREST run D1–D9: create/complete/list company+job with key; result persisted; idempotency;
cross-team blocked; bogus key 28000; RLS; revoked-key blocked.

## Tests
`backend-rs`: **280 pass, 0 fail, 4 ignored** (`cargo test`), `cargo build --bins` clean. New offline
unit tests: `parse_responses_sse` over a committed real SSE fixture (`tests/fixtures/openai_responses.sse`
→ `7`, 18 tokens) + rate-limit detection; `JobService` happy/llm-error with a spy `FakeData` asserting
exact RPC names + sanitized error; route tests; SSE parser.

## Invariants
Custom schema only (no `public`); **no service-role key**; **OpenAI key + Supabase creds stay
server-side** (leak check E12); api-key data path forge-proof (key-credential RPCs); RLS team-scoped.

## Session path + frontend for non-agent users (now done — `08-session-path-and-frontend.md`)
- **Backend session path:** `AuthBroker::session_access_token` exposes the session's GoTrue JWT
  server-side only; `JobService` now dispatches on the bearer — `pcs_…` (browser session) → session
  RPCs + `my_*` views with the user JWT; `paperclip_…` (agent) → the key-credential path. New unit
  tests: `run_job_session_path_uses_session_rpcs_with_user_jwt`,
  `create_company_session_path_resolves_default_team_then_creates`.
- **E11 (session acceptance, real): 9/9** (`run_session_acceptance.py`): a real user logs in via the
  broker (email+password → opaque `pcs_`), forms a real company, and runs a **real OpenAI job → `4`**,
  persisted (subject=user, created_by=the user); lists work; a second user cannot see it (RLS); the
  login payload leaks no JWT/anon key.
- **Frontend (Nuxt):** `pages/paperclip.vue` + `PaperclipConsole.vue` + `usePaperclipSession.ts`
  (login → companies → job console). Frontend tests **162/162** (incl. 5 composable + 2 component);
  `nuxt build` compiles the page. The real browser→proxy→broker→Supabase/OpenAI wire was driven
  end-to-end (login → company → real OpenAI job `4`).
- Backend tests **282/0/4**.
