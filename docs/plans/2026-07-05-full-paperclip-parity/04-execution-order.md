# Execution Order

The first draft had seven phases. Plan-design review found that was still too narrow for full parity. The locked plan uses fourteen phases so every original Paperclip control-plane area has an owner.

## Phase order

0. [`00-parity-oracle-and-matrix.md`](phases/00-parity-oracle-and-matrix.md) — build the failing parity oracle and row-level matrix.
1. [`01-schema-authz-parity.md`](phases/01-schema-authz-parity.md) — schema/default/enums/authorization/permission grants.
2. [`02-goals-projects-issues-work-products.md`](phases/02-goals-projects-issues-work-products.md) — company mission, goals, projects, issues, comments, documents, artifacts.
3. [`04-deployment-auth-onboarding-ops.md`](phases/04-deployment-auth-onboarding-ops.md) — local/authenticated modes, onboarding, bootstrap, ops safety.
4. [`05-adapters-secrets-skills-config.md`](phases/05-adapters-secrets-skills-config.md) — adapter config, secrets, skills, config revisions.
5. [`06-approvals-budgets-costs-activity.md`](phases/06-approvals-budgets-costs-activity.md) — approval lifecycle, budgets, cost rollups, activity logs.
6. [`07-agent-runtime-lifecycle.md`](phases/07-agent-runtime-lifecycle.md) — canonical statuses, heartbeat runtime, wakeups, cancellation, logs/events.
7. [`03-workspaces-routines-plugins-sandbox.md`](phases/03-workspaces-routines-plugins-sandbox.md) — runtime workspaces, routines, plugin sandbox/current-addendum surfaces.
8. [`08-interfaces-rest-cli-mcp.md`](phases/08-interfaces-rest-cli-mcp.md) — REST, CLI, MCP parity after domain APIs exist.
9. [`09-nuxt-board-ui-parity.md`](phases/09-nuxt-board-ui-parity.md) — human board UI parity.
10. [`10-import-export-catalogs-portability.md`](phases/10-import-export-catalogs-portability.md) — catalogs, templates, export/import.
11. [`11-observability-feedback-pipelines.md`](phases/11-observability-feedback-pipelines.md) — observability/productivity/pipeline current-addendum classification and implementation.
12. [`12-shared-contract-docs-parity.md`](phases/12-shared-contract-docs-parity.md) — shared constants/validators/docs/OpenAPI contract sync.
13. [`13-migration-compatibility-cutover.md`](phases/13-migration-compatibility-cutover.md) — compatibility, cutover, rollback.

## Dependency rules

- Phase 00 blocks all implementation. No worker can close a gap that is not represented in the matrix.
- Phase 01 blocks all domain phases. The wrong status/default/authz contracts make later work invalid.
- Domain backend phases must land before their UI/CLI/MCP parity rows.
- Live Supabase/OpenAI acceptance is final proof, not a substitute for deterministic local red/green tests.
- A waiver for any V1-core row requires a product-spec update and reviewer signoff.

## Stop condition

The plan is complete only when `acceptance/run_original_gap_diff.py` reports:

```text
gaps_missing=0 gaps_partial=0 gaps_divergent=0
```

or every non-full row is explicitly waived by an updated product contract.


## Row-level closure rule

Each phase owns explicit rows in `evidence/rewrite-gap-matrix.md`. A phase cannot close from a category-level pass. Every row must end with:

```text
phase=<owner> acceptance_id=<id> failing_test=<path::name> passing_test=<command> evidence_ref=<source+rewrite> final_status=<full|waived>
```

The final cutover phase must fail if any blocking row remains `partial`, `missing`, `divergent`, or lacks a reviewed waiver.
