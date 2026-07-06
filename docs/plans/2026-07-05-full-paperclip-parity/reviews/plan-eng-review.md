# plan-eng-review — Full Paperclip Parity Plan

Skill: `plan-eng-review`. Target: `docs/plans/2026-07-05-full-paperclip-parity/`.
Mode: FULL_REVIEW. The active `/goal` instructed me not to interrupt with per-issue
AskUserQuestion prompts, so obvious fixes were applied directly and recorded here.

## Step 0 — Scope challenge

The plan is a *plan artifact*, not an implementation. The goal is that the plan itself
is complete and modular enough to drive the Rust/Supabase/Nuxt rewrite to **zero known
parity gaps** against original Paperclip V1, with Codex/TDD workers and acceptance gates
that cannot be satisfied by a vertical slice. Scope accepted as-is; no reduction.

## Architecture review

- The plan preserves the mandatory invariants: `paperclip`/`paperclip_private` schemas
  only, no `public` app objects, no service-role key, opaque `pcs_` client sessions,
  OpenAI Responses API only, company/low-trust boundaries, approvals, budgets, activity.
- Initial 7-phase draft was too narrow. Plan-design review (Ultracode workflow) expanded
  to 14 owned phases covering goals/projects/issues, deployment/auth/onboarding,
  workspaces/routines/plugins, import/export/catalogs, observability, shared contracts,
  and cutover.
- Phase ownership is now row-level: every phase file owns matrix rows.

## Code quality / modularity review

- File structure is modular: top-level overview/architecture/file-structure/execution-order/
  test-strategy/risk-register, a `phases/` directory, `acceptance/`, `codex/`, `reviews/`,
  and `evidence/`.
- Removed duplicate early phase files that conflicted with the renumbered 14-phase order.
- Migration ledger introduced so no two phases propose conflicting migration numbers.

## Test review

- Test strategy is layered: unit, route/service, migration/RLS, frontend, CLI/MCP, live.
- The row-level anti-false-parity gate requires every non-full row to carry
  `{phase, acceptance_id, failing_test, passing_test, evidence_ref, final_status}`.
- `acceptance/run_original_gap_diff.py` is implemented and fails today
  (`gaps_missing=145 gaps_partial=50 gaps_divergent=22 rows_total=220 rows_open=217`),
  printing exactly `gaps_missing=0 gaps_partial=0 gaps_divergent=0` only on success.
- Live harnesses (`run_real_user_real_agent.py`, `run_cli_mcp_surface.py`,
  `run_frontend_user_flow.py`) are intentional RED scaffolds owned by Phase 00; they
  return non-zero until implemented, so the release gate cannot pass early.

## Performance review

No issues for a plan artifact. N+1/caching concerns are phase-implementation concerns and
are flagged in the per-phase tasks where relevant (runtime/run listing, cost rollups).

## Outside voice — Codex

Codex reviewed the plan across four passes (`codex-plan-review.md`,
`codex-plan-rereview.md`, `codex-plan-rereview-2.md`, `codex-plan-rereview-3.md`,
`codex-plan-rereview-4.md`). Each FAIL was followed by concrete patches:

1. Phase count contradiction (17 vs 14) → reconciled to 14.
2. Inconsistent gap-output spelling → standardized to `gaps_missing=0 gaps_partial=0 gaps_divergent=0`.
3. Category-level acceptance could pass without row-level closure → added row-level matrix
   and anti-false-parity gate.
4. Codex implementer prompts did not load global context → strengthened.
5. Live harness placeholders and matrix not covering all 14 phases → added Phase 00
   ownership and added coverage rows for every phase.
6. Authoritative tally contradiction (175/100 vs 220/145) → reconciled to 220 rows.

Final Codex verdict: **PASS** (`codex-plan-rereview-4.md`).

## NOT in scope

- Implementing the rewrite itself. This plan only defines the zero-gap roadmap,
  acceptance gates, and Codex/TDD prompts. Implementation is the subsequent multi-phase
  work tracked by the matrix.
- Redefining V1. Any waiver requires updating `doc/SPEC-implementation.md`.

## What already exists

- The rewrite's vertical slice (auth foundation, companies/jobs RPCs, OpenAI Responses
  execution, hiring + approvals, CLI/MCP/Nuxt surfaces) is reused as the starting point;
  phases migrate/correct it rather than rebuild from zero.
- The corrected parity audit journals are reused as the row-level matrix seed.

## Failure modes flagged

- False-parity claim from a green vertical slice: mitigated by the row-level gate.
- Supabase service-role temptation: banned; flagged in non-negotiables and checklist.
- Client leak of Supabase/OpenAI detail: mitigated by the no-secret scanner spec and
  per-phase redaction tests.
- Migration enum changes on the live project: mitigated by forward-only migration rules
  and the migration ledger.

## Implementation tasks (plan-level)

- T1: Phase 00 — finish the four acceptance harnesses + no-secret scanner (RED → real).
- T2: Phase 01 — schema/default/enums/authz parity migration + tests.
- T3: Phase 06 — approval governance, budgets, costs, activity side effects.
- T4: Phase 07 — agent runtime/lifecycle (idle/running/error/terminated, heartbeat).
- T5: Phases 02–05, 08–13 — remaining domain, interface, portability, docs, cutover work.

## Completion summary

- Step 0: scope accepted as-is.
- Architecture: 1 issue (narrow phase set) → expanded to 14 phases.
- Code quality: duplicate phase files removed; ledger added.
- Test review: row-level matrix + gate produced; live harness RED scaffolds owned by Phase 00.
- Performance: no plan-level issues.
- NOT in scope: written.
- What already exists: written.
- Outside voice: Codex PASS after 5 passes.
- Verdict: ENG REVIEW CLEARED (PLAN).
