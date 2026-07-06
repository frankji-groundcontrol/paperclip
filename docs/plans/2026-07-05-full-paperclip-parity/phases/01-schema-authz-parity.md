# Phase 01 — Schema and Authorization Parity

## Goal

Make the Supabase data model and authz primitives parity-compatible before adding more behavior. This phase owns 55 matrix rows (1 full, 20 partial, 24 missing, 10 divergent).

## Source evidence (read these before coding)

- `doc/SPEC-implementation.md:126-184` — company/agent/api-key model.
- `packages/shared/src/constants.ts:19-27` — canonical `AGENT_STATUSES`.
- `packages/shared/src/constants.ts:538-552` — approval types/statuses.
- `packages/shared/src/constants.ts:786-798` — permission keys.
- `packages/db/src/schema/agents.ts:23` — original `agents.status` default `'idle'`.
- `server/src/services/agents.ts:552-684` — pause/resume/clear-error/terminate/approve transitions.
- `server/src/routes/agents.ts:1853-1855, 2257, 2444` — invokable set, hire/default status.
- `server/src/routes/authz.ts` — board/company guards.
- `server/src/services/company-member-roles.ts` — original role grants.

## Canonical semantics (authoritative for this phase)

Original agent status model:

- Default status on create: `idle` (`packages/db/src/schema/agents.ts:23`).
- Direct create (no approval required): `idle` (`server/src/routes/agents.ts:2444`).
- Hire requiring approval: `pending_approval`; on board approve → `idle` (`server/src/services/agents.ts:683`); on reject → `terminated` via terminate (`server/src/services/agents.ts:629`).
- Runtime: `idle` → `running` → `idle` | `error`.
- `terminated` is irreversible; `paused` via pause; resume/clear-error → `idle`.
- Invokable (eligible to act / be scheduled): status NOT IN (`paused`, `terminated`, `pending_approval`) (`server/src/routes/agents.ts:1853-1855`).
- `active` exists in the enum but original agent flows do not set it as a runtime state.

Rewrite divergences to fix:

- `require_board_approval_for_new_agents` default `true` → `false`.
- `paperclip.agent_status` enum `{pending_approval, active, paused, archived}` → canonical `{active, paused, idle, running, error, pending_approval, terminated}`.
- `paperclip.approval_type` enum missing `budget_override_required`, `request_board_approval`.
- `paperclip.approval_status` enum missing `revision_requested`.
- `hire_agent` / `hire_agent_with_key` set approved/non-required status to `active` → must be `idle`.
- `decide_approval` / `decide_approval_with_key` approve → `active` (must be `idle`); reject → `archived` (must be `terminated`).
- `paperclip_private.fn_agent_active` checks `status='active'` → must check invokable set `status IN ('idle','running')`.

## Target files

- `supabase/migrations/paperclip/0014_parity_enums_defaults.sql` — enum/default/transition corrections + data migration.
- `supabase/migrations/paperclip/0015_permission_grants_authz.sql` — permission grant tables + helpers (Phase 01b).
- `docs/plans/2026-07-05-full-paperclip-parity/acceptance/run_schema_parity.py` — schema-parity assertions (RED first).
- `backend-rs/src/authz/*` — Rust principal/permission mapping (Phase 01b).
- `backend-rs/tests/authz_parity.rs`, `backend-rs/tests/supabase_auth_live.rs`.

## Phase 01a — enum/default/transition corrections (this codex TDD cycle)

### Migration approach (forward-only, safe on live project)

`0014_parity_enums_defaults.sql`:

1. `alter table paperclip.companies alter column require_board_approval_for_new_agents set default false`.
2. Agent status enum:
   - `alter type paperclip.agent_status rename value 'archived' to 'terminated'` (rewrite `archived` ≡ original `terminated`).
   - `alter type paperclip.agent_status add value if not exists 'idle'`.
   - `alter type paperclip.agent_status add value if not exists 'running'`.
   - `alter type paperclip.agent_status add value if not exists 'error'`.
   - `alter table paperclip.agents alter column status set default 'idle'`.
   - Data: `update paperclip.agents set status='idle' where status='active'` (approved agents → canonical runnable state).
3. Approval enums:
   - `alter type paperclip.approval_type add value if not exists 'budget_override_required'`.
   - `alter type paperclip.approval_type add value if not exists 'request_board_approval'`.
   - `alter type paperclip.approval_status add value if not exists 'revision_requested'`.
4. RPC transition fixes (recreate functions via `create or replace`):
   - `hire_agent` / `hire_agent_with_key`: non-required and board-approved status → `'idle'::paperclip.agent_status` (not `'active'`).
   - `decide_approval` / `decide_approval_with_key`: approve → `status='idle'`; reject → `status='terminated'`.
   - `paperclip_private.fn_agent_active`: body → `select exists (select 1 from paperclip.agents where id=p_agent and team_id=p_team and status in ('idle','running'))`.
5. Keep `active` in the enum (original has it) but no flow sets it as a runtime state.

### Postgres enum gotcha

`ALTER TYPE ... ADD VALUE` inside a transaction block is allowed in PG12+ only when the new value is not used later in the same transaction. Supabase MCP `apply_migration` wraps in a transaction. Therefore split if needed: if a single migration both adds and uses a new value, split into `0014a` (add values) and `0014b` (data + RPCs using them). Prefer one file if the new values are only used in `create or replace function` bodies (those are not "used" at DDL time).

### Acceptance assertions (`run_schema_parity.py`, RED first)

Asserts against supabase-franky via anon-key RPC + `information_schema`/`pg_type` introspection through a `paperclip_private` helper or direct catalog reads available to `authenticated`:

- `paperclip.companies.require_board_approval_for_new_agents` column default = `false`.
- `paperclip.agent_status` enum values = `{active, paused, idle, running, error, pending_approval, terminated}` (no `archived`).
- `paperclip.approval_type` includes `budget_override_required`, `request_board_approval`.
- `paperclip.approval_status` includes `revision_requested`.
- `paperclip.agents.status` column default = `idle`.
- After a no-approval-required hire (via `hire_agent` with a board JWT), the agent row has `status='idle'`.
- After `decide_approval(approve)`, subject agent `status='idle'`; after `decide_approval(reject)`, `status='terminated'`.
- No `paperclip*` application objects in schema `public`.

## Phase 01b — permission grants + authz (follow-up cycle, not this turn)

- Add `paperclip.company_memberships`, `paperclip.principal_permission_grants` and helpers.
- Map rewrite team roles to original owner/admin/operator/viewer grant defaults.
- Rust `authz` module for principal → permission decisions.

## Acceptance for this cycle

- `run_schema_parity.py` RED before migration, GREEN after.
- Existing `backend-rs` `cargo test` stays green (or tests asserting the old `active`/`archived` behavior are updated to canonical `idle`/`terminated`).
- No `public`-schema app objects; no service-role key used.
