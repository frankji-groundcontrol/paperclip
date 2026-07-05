# 03 — Agent interfaces: CLI + MCP + skill (primary)

For agent users (Claude Code / Codex / Hermes). All three wrap the **same broker HTTP endpoints**
(`02`) and authenticate with a **Paperclip API key** (the agent path). The server hides Supabase and
OpenAI entirely — the agent only ever sees companies, jobs, and results.

## Config / auth
- `PAPERCLIP_SERVER` (default `http://127.0.0.1:8787`) and `PAPERCLIP_API_KEY` (`paperclip_…`).
- The key is obtained by device-login (`paperclip login`, wraps the existing `cli_*` RPCs) **or** set
  directly (`paperclip config set-key …` / `PAPERCLIP_API_KEY`). Device-login onboarding endpoints
  (`POST /api/cli/start`, `/api/cli/poll`, and an authenticated `POST /api/cli/approve`) are a thin
  proxy over the proven `cli_start/approve/poll_device_login` RPCs — included but not the acceptance gate.

## CLI (`paperclip`, Rust bin in the workspace; clap)
```
paperclip config set-key <API_KEY>          # or PAPERCLIP_API_KEY env
paperclip company create "<name>"           # -> POST /api/companies
paperclip company list                      # -> GET  /api/companies
paperclip job run --company <ID> "<prompt>" [--model gpt-5.4-mini]   # -> POST …/jobs ; prints result
paperclip job list --company <ID>           # -> GET  …/jobs
```
Thin HTTP client (reqwest): send `Authorization: Bearer <key>`, print JSON or a friendly line.
Exit non-zero on HTTP error. Unit-test the arg parsing + request building against a mock server.

## MCP server (`paperclip-mcp`, stdio JSON-RPC)
Exposes tools for agents (auth via `PAPERCLIP_API_KEY` + `PAPERCLIP_SERVER` env):
- `paperclip_create_company({ name }) -> { companyId }`
- `paperclip_list_companies() -> { companies }`
- `paperclip_run_job({ companyId, prompt, model? }) -> { jobId, status, result, usage }`
- `paperclip_list_jobs({ companyId }) -> { jobs }`

Implements the MCP handshake (`initialize`, `tools/list`, `tools/call`) over stdio; each `tools/call`
maps to the corresponding broker HTTP endpoint. Testable by feeding JSON-RPC frames on stdin and
asserting the framed responses (dispatch logic unit-tested with a mock HTTP server). Registerable in
Claude Code / Codex as a stdio MCP server:
```json
{ "mcpServers": { "paperclip": { "command": "paperclip-mcp", "env": { "PAPERCLIP_API_KEY": "…", "PAPERCLIP_SERVER": "…" } } } }
```

## Skill (`skills/paperclip/SKILL.md`)
Teaches an agent the flow: ensure `PAPERCLIP_API_KEY` is set → `paperclip_create_company` → capture
`companyId` → `paperclip_run_job` with the task prompt → read `result`. Includes the CLI equivalents
and a worked example ("form a company, run a summarization job, read the answer"). Kept short and
outcome-framed per house skill style.

## Language/impl note
CLI + MCP in **Rust** for consistency with `backend-rs` (shared reqwest client, one toolchain). codex
implements the request/response + dispatch logic TDD; the stdio/loop shells are thin.
