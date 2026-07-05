# 00 — Overview: real user → real company → real job

**Goal.** Prove the product actually works: a **real user** (esp. an *agent* user via CLI/MCP,
authenticating with an API key) **forms a real company** and **runs a real job** whose work is done
by a **real OpenAI call** (Responses API), with the result persisted and access-controlled. Built on
the existing `paperclip` custom schema (no `public`, no service-role key) and the server-mediated
broker. Primary interface: **CLI + MCP + skills** for agent users (Claude Code / Codex / Hermes);
a **frontend** for non-agent users is a follow-on.

## The end-to-end we must demonstrate (real data, real LLM)
1. Agent user runs `paperclip login` → device-login mints an **API key** (no plaintext at rest; the
   existing `cli_*` flow).
2. `paperclip company create "Acme"` → a real `paperclip.companies` row scoped to the user's team.
3. `paperclip job run --company <id> "Summarize the theory of relativity in one sentence."`
   → backend creates a `paperclip.jobs` row (running), calls **OpenAI `/v1/responses`** server-side,
   persists the real answer + token usage, marks it succeeded, and prints the answer.
4. Isolation: a different user cannot see Acme or its jobs (RLS).
5. Same three operations available as **MCP tools** (`paperclip_create_company`, `paperclip_run_job`,
   …) so Claude Code/Codex/Hermes can do it directly, and documented as a **skill**.

## Hard constraints (carried from the auth system)
- `paperclip` / `paperclip_private` schemas only; **no `public`**, **no service-role key**.
- **No Supabase detail and no OpenAI key** ever reach the client — the server holds both.
- API-key (agent) data path uses **key-credential RPCs**: every mutation RPC on the api-key path takes
  `(p_prefix, p_key_hash, …)` and **re-resolves + re-authorizes internally**, so a caller who lacks a
  valid key can forge nothing (closes the same class as the cli_approve bug in the auth review).
- Session (human/frontend) data path uses the broker's stored **user JWT** → RLS applies as `auth.uid()`.

## OpenAI Responses contract (verified live against `${OPENAI_BASE_URL}`)
- `POST ${OPENAI_BASE_URL}/v1/responses`, header `Authorization: Bearer ${OPENAI_API_KEY}`.
- Body: `{"model": "gpt-5.4-mini", "input": [{"role":"user","content":"…"}]}` — **input must be a list**.
- Response is **SSE** (streaming is forced): answer = concat of every `response.output_text.delta`
  event's `delta`; token usage from the final `response.completed` event's `response.usage`.
- Available models include `gpt-5.4-mini` (default job model), `gpt-5.4`, `gpt-5.5`. `gpt-4o*` is rejected.

## File map
- `01-db-companies-jobs.md` — `companies` + `jobs` schema, RLS, views, session RPCs + api-key-credential RPCs.
- `02-backend-llm-jobs.md` — Rust OpenAI Responses SSE client, job runner, gateway RPC methods, broker endpoints.
- `03-cli-mcp-skills.md` — `paperclip` CLI, MCP server, and the agent skill.
- `04-frontend.md` — Nuxt pages for non-agent users (follow-on, minimal).
- `05-testing-real-user.md` — real-user acceptance matrix (CLI/MCP path + real OpenAI + RLS).
- `06-eng-review.md` — plan-eng-review findings + decisions.

## Phasing
- **P1 (must work):** DB companies+jobs+RPCs → backend Responses client + job-run endpoint (api-key path)
  → real end-to-end proof via HTTP with a real key.
- **P2 (agent UX):** CLI + MCP + skill wrapping P1; prove the flow *through the CLI and MCP*.
- **P3 (humans):** Nuxt frontend (login → company → job) — scaffold + note; not the acceptance gate.
