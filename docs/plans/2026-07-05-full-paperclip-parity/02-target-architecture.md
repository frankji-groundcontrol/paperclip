# Target Architecture

## Principle

The rewrite may use Rust, Supabase, and Nuxt, but it must preserve original Paperclip V1 contracts. Implementation language is not allowed to shrink the control plane.

## Layers

### Supabase/Postgres

- Exposed schema: `paperclip`.
- Private security-definer helpers: `paperclip_private`.
- No application objects in `public`.
- RLS on all exposed tables.
- `SECURITY DEFINER` helper/RPC functions use `set search_path=''`.
- Session path uses broker-held Supabase JWT only server-side.
- API-key path uses forge-proof `(prefix, hash)` re-resolution in every RPC.

### Rust backend (`backend-rs`)

Break the current broad job service into control-plane modules:

- `authz/` — principal resolution, team/company membership, board predicates, permission grant checks, low-trust policy decisions.
- `companies/` — company lifecycle/settings/budgets.
- `agents/` — schema DTOs, lifecycle actions, org tree validation, agent keys, config revisions.
- `approvals/` — generic approval creation/decision/revision/cancel side effects.
- `runtime/` — heartbeat runs, wakeups, cancellation, logs/events, runtime state.
- `adapters/` — OpenAI Responses adapter now, local CLI/session adapter contract later in phase order.
- `tasks/` — issue/task assignment and checkout control plane.
- `activity/` — append-only audit log API.
- `secrets/` — secret refs/redaction boundaries.

### Nuxt frontend (`frontend-nuxt`)

Keep the current console as a smoke UI, but build parity pages/components:

- Company settings.
- Agents list/detail/new-agent wizard/org chart.
- Approvals list/detail/revision/comments.
- Runtime/live runs/logs.
- Tasks/issues/work products.
- Budgets/costs.
- Access/members/invites/join requests.

### CLI + MCP

CLI and MCP must expose operator/agent capabilities, not just job/hire shortcuts. Every new governed backend capability gets:

- HTTP route test.
- CLI command or explicit documented non-CLI reason.
- MCP tool or explicit documented non-MCP reason.
- Auth boundary tests for session, board key, agent key, low-trust agent key.

## Compatibility model

- Existing rewrite tests remain green, but tests should be updated when they assert non-parity behavior (for example `active`/`archived` hire transitions).
- Migrations must include data fixes for current Supabase rows created by the slice (`active` approved agents -> `idle`; `archived` rejected pending agents -> `terminated` when semantically terminal).
