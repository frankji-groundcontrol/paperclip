# Full Paperclip Parity Implementation Plan

> **Goal:** eliminate the verified parity gap between the current Rust/Supabase/Nuxt rewrite and original Paperclip V1 for agent hiring, governance, runtime, interfaces, and real work execution.
>
> **Definition of success:** the rewrite reaches **zero known parity gaps** against `doc/SPEC-implementation.md` and the original TypeScript implementation evidence cited here. A working vertical slice is not enough.

## Directory map

- [`00-overview.md`](00-overview.md) — scope, success criteria, and non-negotiable invariants.
- [`01-parity-audit-summary.md`](01-parity-audit-summary.md) — corrected audit baseline: 3 full / 50 partial / 145 missing / 22 divergent across 220 rows (authoritative ledger: `evidence/rewrite-gap-matrix.jsonl`).
- [`02-target-architecture.md`](02-target-architecture.md) — target Rust/Supabase/Nuxt architecture that preserves original V1 contracts.
- [`03-file-structure.md`](03-file-structure.md) — planned module/file layout for schema, backend, CLI/MCP, Nuxt, and test harnesses.
- [`04-execution-order.md`](04-execution-order.md) — dependency-ordered phases for TDD/Codex workers.
- [`05-test-strategy.md`](05-test-strategy.md) — red/green/unit/integration/live acceptance strategy.
- [`06-risk-register.md`](06-risk-register.md) — security, data, runtime, migration, and external-service risks.
- [`phases/`](phases/) — implementation workstreams, each modular and reviewable.
- [`acceptance/`](acceptance/) — parity harness specs and real-user/real-agent scenarios.
- [`codex/`](codex/) — Codex execution/review prompts.
- [`reviews/`](reviews/) — plan-eng-review and Codex review artifacts.
- [`evidence/`](evidence/) — original-source map and rewrite gap matrix.

## How to use this plan

1. Treat [`01-parity-audit-summary.md`](01-parity-audit-summary.md) as the starting contract; do not erase gaps without source-backed implementation and tests.
2. Implement phases in [`04-execution-order.md`](04-execution-order.md) order. Each phase is designed for Codex TDD workers.
3. After each phase, run the narrow phase checks plus the full parity harness from [`acceptance/full-parity-harness-spec.md`](acceptance/full-parity-harness-spec.md).
4. Completion requires the final Codex parity review to produce no remaining missing/divergent/partial items unless explicitly marked out-of-scope by a revised product contract.

## Non-negotiables

- Use only Supabase schemas `paperclip` and `paperclip_private`; **do not use `public`** for application tables/functions.
- Do not use a Supabase service-role key in app, CLI, MCP, frontend, or test acceptance flows.
- No Supabase/OpenAI credentials or implementation details reach the client.
- Preserve company boundaries, agent low-trust boundaries, approval gates, budgets, activity logs, and secret redaction.
- Use OpenAI **Responses API**, not chat completions, for OpenAI-backed adapter/job paths.

## Plan-design review revision

The first written plan had 7 phases. Ultracode plan-design review found that still under-covered original V1. This revision expands the plan to 14 owned phases, including goals/projects/issues, deployment/auth/onboarding, workspaces/routines/plugins, import/export/catalogs, observability/pipelines, shared contracts/docs, and cutover.


## Row-level ledger note

The first audit synthesis reported `3 full / 48 partial / 98 missing / 24 divergent`. Converting the audit journals into a row-level closure ledger exposed two counter mismatches inside focused retry outputs, yielding `3 full / 50 partial / 100 missing / 22 divergent` across 175 rows for the originally-audited hiring/governance/runtime surface. The plan-design review then added owned rows for the remaining original-V1 control-plane areas (goals/projects/issues, deployment/auth/onboarding, workspaces/routines/plugins, import/export/catalogs, observability/pipelines, shared contracts/docs, cutover, and the parity oracle). The authoritative ledger is `evidence/rewrite-gap-matrix.jsonl`: `3 full / 50 partial / 145 missing / 22 divergent` across 220 rows, with every one of the 14 phases owning rows.

## GSTACK REVIEW REPORT

| Review | Trigger | Why | Runs | Status | Findings |
|--------|---------|-----|------|--------|----------|
| Eng Review | `/plan-eng-review` | Architecture & tests (required) | 1 | CLEAR (PLAN) | 6 issues found across 5 Codex passes, all folded; 0 critical gaps |
| Codex Review | `codex exec` | Independent 2nd opinion | 5 | CLEAR | final PASS in `reviews/codex-plan-rereview-4.md` after 4 FAIL-driven patch cycles |

- **CODEX:** 4 FAIL verdicts each produced concrete blockers (phase-count contradiction, gap-output spelling, row-level matrix closure, live-harness ownership, 14-phase row coverage, authoritative-tally contradiction); all patched and the 5th pass returned PASS.
- **VERDICT:** ENG + CODEX CLEARED — plan is ready to drive implementation toward zero known parity gaps.

NO UNRESOLVED DECISIONS

