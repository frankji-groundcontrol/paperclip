---
name: paperclip-agent
description: Use when an agent (Claude Code / Codex / Hermes) needs to form a Paperclip company and run a job on it — an LLM-backed unit of work — via the Paperclip CLI or MCP tools. Covers auth (API key), creating a company, running a job, and reading results. For the broader control-plane API, use the `paperclip` skill instead.
---

# Paperclip (agent quickstart): form a company, run a job

Form a **company** (a team-scoped workspace) and run **jobs** on it — each job is a real LLM-backed
task the server executes and persists. Authenticate with a **Paperclip API key**; the server hides all
Supabase and model-provider details (you never see them).

## Setup (once)
```
export PAPERCLIP_API_KEY=paperclip_pc_xxxxxxxx_...
export PAPERCLIP_SERVER=http://127.0.0.1:8787      # your Paperclip broker
```
Get a key with `paperclip login` (device flow) or from a teammate/admin.

## The flow
1. **Form a company**
   - CLI: `paperclip company create "Acme"` → prints a `companyId`.
   - MCP: `paperclip_create_company({ "name": "Acme" })` → `{ "companyId": "…" }`.
2. **Run a job** (the work — an LLM task on that company)
   - CLI: `paperclip job run --company <companyId> "Summarize our Q3 plan in one sentence."`
   - MCP: `paperclip_run_job({ "companyId": "…", "prompt": "…", "model": "gpt-5.4-mini" })`
     → `{ "jobId", "status": "succeeded", "result": "…", "usage": { "total_tokens": … } }`
   - `model` is optional (defaults to `gpt-5.4-mini`). Read `result` for the answer.
3. **Review**
   - `paperclip job list --company <companyId>` / `paperclip_list_jobs({ companyId })`.
   - `paperclip company list` / `paperclip_list_companies()`.

## Worked example
```
paperclip company create "Tagline Co"          # -> companyId=abc123
paperclip job run --company abc123 "Write a 5-word tagline for an AI ops platform."
# -> result: "Ship faster. Sleep better. Scale."
```
Via MCP the same is two `tools/call`s: `paperclip_create_company` then `paperclip_run_job`.

## Notes
- A job is **synchronous**: `run_job` returns once the LLM has answered and the result is persisted.
- Failures come back as `status:"failed"` with a generic `error` (details stay server-side).
- Companies and jobs are scoped to your key's **team** — you only ever see your team's data.
- Register the MCP server in your client:
  `{ "mcpServers": { "paperclip": { "command": "paperclip-mcp",
    "env": { "PAPERCLIP_API_KEY": "…", "PAPERCLIP_SERVER": "…" } } } }`
