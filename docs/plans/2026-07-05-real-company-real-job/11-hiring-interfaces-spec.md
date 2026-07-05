# 11 — Hiring interfaces (codex TDD spec)

The DB hiring layer (migration 0013) is applied + proven. Add the **interfaces** so humans and agents can
hire/approve via the broker, CLI, and MCP — extending the existing bearer-dispatch pattern in
`JobService` (session `pcs_` → session RPCs; api-key `paperclip_` → `*_with_key`). Strict TDD; keep the
full `cargo test` green; no Supabase/OpenAI secret in any client response.

## JobService methods (add; dispatch on bearer exactly like create_company/run_job)
- `hire_agent(bearer, company_id, name, role: Option, model: Option, title: Option) -> Value`
  → session: `rpc("hire_agent", {p_company_id,p_name,p_role,p_model,p_title}, Bearer(jwt))`;
    api-key: `rpc("hire_agent_with_key", {p_prefix,p_key_hash,p_company_id,p_name,p_role,p_model,p_title}, Anon)`.
  Returns the JSON `{agentId,status,approvalId?}` from the RPC unchanged.
- `list_agents(bearer, company_id) -> Value` → session: `get("my_agents?company_id=eq.{c}&select=*&order=created_at.desc", Bearer(jwt))`;
    api-key: `rpc("list_agents_with_key", {p_prefix,p_key_hash,p_company_id}, Anon)`.
- `list_approvals(bearer, company_id) -> Value` → session `my_approvals`; api-key `list_approvals_with_key`.
- `decide_approval(bearer, approval_id, approve: bool) -> Value` → session:
    `rpc("decide_approval", {p_approval_id,p_approve}, Bearer(jwt))`; api-key:
    `rpc("decide_approval_with_key", {p_prefix,p_key_hash,p_approval_id,p_approve}, Anon)`. Return `{status}`.
  (Board-only + agents-cannot-decide are enforced in the DB; the endpoint just maps errors.)

## Routes (broker router, under `/api/paperclip`)
| Method | Path | Body | Returns |
|---|---|---|---|
| POST | `/api/paperclip/companies/:companyId/agents` | `{name, role?, model?, title?}` | `{agentId,status,approvalId?}` |
| GET  | `/api/paperclip/companies/:companyId/agents` | — | `{agents:[…]}` |
| GET  | `/api/paperclip/companies/:companyId/approvals` | — | `{approvals:[…]}` |
| POST | `/api/paperclip/approvals/:approvalId/decide` | `{approve: bool}` | `{status}` |
Reuse `map_job_error` (42501→403, 28000/unknown-session→401, else 502).

## CLI (`paperclip`)
```
paperclip agent hire "<name>" --company <ID> [--role <r>] [--model <m>]   # prints agentId + status (+ approvalId)
paperclip agent list --company <ID>
paperclip approval list --company <ID>
paperclip approval approve <APPROVAL_ID>
paperclip approval reject  <APPROVAL_ID>
```
Same HTTP-client + arg-parsing conventions as the existing commands.

## MCP (`paperclip-mcp`) tools
- `paperclip_hire_agent({ companyId, name, role?, model? }) -> { agentId, status, approvalId? }`
- `paperclip_list_agents({ companyId }) -> { agents }`
- `paperclip_list_approvals({ companyId }) -> { approvals }`
- `paperclip_decide_approval({ approvalId, approve }) -> { status }`

## Tests (TDD)
- **Unit** (spy `FakeData`): `hire_agent` (api-key) calls exactly `hire_agent_with_key` with the right
  `p_*` keys + `Auth::Anon`; session path calls `hire_agent` with `Auth::Bearer(jwt)`; `decide_approval`
  api-key → `decide_approval_with_key`, session → `decide_approval`; list methods hit the right RPC/view.
- **Route** tests over the axum router (like `paperclip_routes.rs`): the 4 endpoints delegate + don't leak.
- Extend `FakeData` in `jobs_service.rs`/`paperclip_routes.rs` to answer the new RPC names
  (`hire_agent`, `hire_agent_with_key`, `decide_approval`, `decide_approval_with_key`, `list_agents_with_key`,
  `list_approvals_with_key`) and the `my_agents`/`my_approvals` GETs.

## Definition of done
- New methods + routes + CLI + MCP tools, TDD; full `cargo test` green; `cargo build --bins` clean.
- Live check (guiding agent): hire an agent via CLI/MCP → board approves → the agent runs a real job.
