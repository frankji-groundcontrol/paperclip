# 10 — Agent hiring parity (fill the gap)

**Gap:** jobs were run *as the user* (`subject_type='user'`). The original Paperclip is a company that
**hires agents (employees)** who do the work. This adds that: a company **hires a real agent** via the
original **approval-gated governance**, and the hired agent **does a real OpenAI job attributed to it**.

## Parity model (from `doc/plans/2026-02-19-ceo-agent-creation-and-hiring.md`, `agents.ts`, `approvals`)
- Agents are employees: `name, role, title, reports_to (hierarchy), status, adapter/model, permissions`.
- **Company toggle** `require_board_approval_for_new_agents` (default **true**).
- **Limbo**: a hire starts in `pending_approval` — the agent record exists but **cannot run / receive
  work / mint keys** until approved.
- **`hire_agent` approval**: board approves (or rejects) the hire; on approve the agent becomes `active`.
- **Permission** `can_create_agents` (CEO true, others false) — who may propose hires.
- Hired agent then does jobs → the job carries `subject_type='agent'` + `agent_id`.

## DB (migration `paperclip_0013_agents_hiring`)
- `alter table paperclip.companies add require_board_approval_for_new_agents boolean not null default true`.
- enums `agent_status (pending_approval|active|paused|archived)`, `approval_type (hire_agent|approve_ceo_strategy)`,
  `approval_status (pending|approved|rejected|cancelled)`.
- `paperclip.agents(id, company_id→companies, team_id→teams [RLS], name, role default 'general', title,
  reports_to→agents, status default 'pending_approval', adapter_type default 'openai',
  model default 'gpt-5.4-mini', permissions jsonb default '{"can_create_agents":false}',
  capabilities jsonb, created_by, created_at, updated_at, meta)`.
- `paperclip.approvals(id, company_id, team_id [RLS], type, status default 'pending', subject_agent_id→agents,
  payload jsonb, requested_by, decided_by, decided_at, created_at, meta)`.
- RLS (enable, not force) + `my_agents` / `my_approvals` views (team-scoped).

### RPCs (session + `*_with_key`; least-privilege grants like 0007/0009)
- **hire_agent** `(p_company_id, p_name, p_role, p_adapter_type, p_model, p_title, p_reports_to, p_permissions)`:
  requires `owner/admin/operator` (session) or a key with `agents:write` scope (`_with_key`). Inserts the
  agent; if the company requires approval → `status='pending_approval'` **and** an
  `approval(type=hire_agent, status=pending, subject_agent_id=agent, payload=…)`; else `status='active'`.
  Returns `{ agentId, status, approvalId? }`.
- **decide_approval** `(p_approval_id, p_approve)`: **board only** (`owner/admin`). On approve of a
  `hire_agent` → agent `active`, approval `approved`; on reject → agent `archived`, approval `rejected`.
- **list_agents / list_approvals** (session + `_with_key`, scoped to team; reads via `my_*` views on the
  session path).
- **Guards added to existing key RPCs:**
  - `create_api_key`: if `p_subject_type='agent'`, `p_agent_id` must reference an **active** agent in the
    same team (can't mint a key for a limbo/archived agent).
  - `create_job_with_key`: if the key's `subject_type='agent'`, the agent (`agent_id`) must exist in the
    key's team and be `status='active'` — **enforces limbo** (a pending agent cannot run).

## The hire → work loop (what "a company hires a real agent that does a real job" means)
1. `hire_agent(company, "Marketing Analyst", …)` → agent `pending_approval` + `hire_agent` approval `pending`.
2. Board `decide_approval(approval, approve=true)` → agent `active`.
3. Board `create_api_key(team, subject_type='agent', agent_id=<agent>)` → the agent's key (checks active).
4. The agent runs `create_job_with_key(<agent key>, company, prompt)` → **real OpenAI job** →
   `jobs.subject_type='agent'`, `agent_id=<agent>` — the work is done **by the hired agent**.

## Interfaces
- **CLI:** `paperclip agent hire "<name>" --company <id> [--role]`, `paperclip agent list --company <id>`,
  `paperclip approval list --company <id>`, `paperclip approval approve <id>` / `reject <id>`.
- **MCP:** `paperclip_hire_agent`, `paperclip_list_agents`, `paperclip_list_approvals`, `paperclip_decide_approval`.
- **Frontend:** a "Hire agent" action + an **Approvals** panel (approve/reject) in the console; agent list
  per company; run a job *as* an agent.

## Acceptance (real, `run_hiring_acceptance.py`)
| # | Scenario | Assert |
|---|----------|--------|
| H1 | hire an agent into a real company (approval required) | agent `pending_approval` + a `hire_agent` approval `pending` |
| H2 | the **limbo** agent cannot run / cannot get a key | minting an agent key or running a job → `42501` |
| H3 | board (owner) approves the hire | agent `active`, approval `approved` |
| H4 | mint the agent's key; the agent runs a **real OpenAI job** | 200; result present; `jobs.subject_type='agent'`, `agent_id=<agent>` |
| H5 | non-board member cannot approve | `42501` |
| H6 | reject path archives the agent | approval `rejected`, agent `archived` |
| H7 | company toggle off → direct hire | agent `active` immediately, no approval |
| H8 | RLS: another team can't see the agent/approval | 0 rows |
| CLI/MCP | hire + approve + agent-run via the binaries | real result attributed to the agent |
