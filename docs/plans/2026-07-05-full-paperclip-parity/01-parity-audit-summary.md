# Parity Audit Summary

## Corrected audit result

Two layers of evidence feed this plan:

1. The adversarial parity audit + focused retries produced an initial synthesized tally of `3 full / 48 partial / 98 missing / 24 divergent`. When converted into a row-level closure ledger, the focused retry item lists were more precise than their declared counters, yielding `3 full / 50 partial / 100 missing / 22 divergent` across 175 rows for the originally-audited hiring/governance/runtime surface.
2. The plan-design review then added the remaining original-V1 control-plane areas (goals/projects/issues, deployment/auth/onboarding, workspaces/routines/plugins, import/export/catalogs, observability/pipelines, shared contracts/docs, cutover, and the parity oracle itself) as additional owned rows. With those coverage rows the authoritative ledger is `3 full / 50 partial / 145 missing / 22 divergent` across 220 rows.

| Status | Count |
|---|---:|
| Full | 3 |
| Partial | 50 |
| Missing | 145 |
| Divergent | 22 |

**Authoritative source:** `evidence/rewrite-gap-matrix.jsonl`. Workers close rows, not summary buckets.

**Verdict:** current rewrite is **core-slice-only**, not full parity.

## Critical direct divergences

### Company hire-approval default

Original:

- `packages/db/src/migrations/0071_default_hire_approval_off.sql:1`
- `doc/SPEC-implementation.md:139`

```sql
ALTER TABLE "companies" ALTER COLUMN "require_board_approval_for_new_agents" SET DEFAULT false;
```

Rewrite:

- `supabase/migrations/paperclip/0013_agents_hiring.sql:6-7`

```sql
alter table paperclip.companies
  add column if not exists require_board_approval_for_new_agents boolean not null default true;
```

Plan requirement: default must be `false`; direct-create/hire semantics must match original.

### Agent statuses

Original:

- `packages/shared/src/constants.ts:19-27`

```ts
active | paused | idle | running | error | pending_approval | terminated
```

Rewrite:

- `supabase/migrations/paperclip/0013_agents_hiring.sql:9-12`

```sql
pending_approval | active | paused | archived
```

Plan requirement: restore canonical status vocabulary and lifecycle semantics. Approved/runnable agents are `idle`; execution moves `idle -> running -> idle/error`; rejection/termination uses `terminated` and revokes keys.

### Approval types and statuses

Original:

- `packages/shared/src/constants.ts:538-552`

Types: `hire_agent`, `approve_ceo_strategy`, `budget_override_required`, `request_board_approval`.

Statuses: `pending`, `revision_requested`, `approved`, `rejected`, `cancelled`.

Rewrite:

- `supabase/migrations/paperclip/0013_agents_hiring.sql:13-18`

Only `hire_agent`, `approve_ceo_strategy`; no `revision_requested`.

Plan requirement: restore approval enum/status contract and side-effect behavior.

## Highest-priority gaps

1. **Control-plane contract repairs** — schema defaults/enums/transitions/permission keys.
2. **Original authorization model** — company memberships, board actor semantics, permission grants, low-trust policy checks.
3. **Agent runtime/lifecycle** — heartbeat runs, wakeups, cancellation, logs/events, runtime state, task sessions.
4. **Approval governance** — generic approvals, decision notes, revision flow, budget/request-board approval types, activity side effects.
5. **Adapter/config/secrets/skills** — adapter registry, runtime config, instructions bundles, Codex isolation, secret refs/redaction, desired skills.
6. **Tasks/issues/work products** — assignment, checkout, comments, documents, artifacts, recovery/work product flows.
7. **Interfaces** — UI/CLI/MCP/API parity for the above.
8. **Acceptance** — real users + real agents + real OpenAI Responses + parity diff harness.
