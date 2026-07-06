# Codex Prompts

## Plan review prompt

```text
You are reviewing docs/plans/2026-07-05-full-paperclip-parity for completeness against original Paperclip V1. Read doc/SPEC-implementation.md, packages/shared/src/constants.ts, server/src/routes/agents.ts, server/src/routes/approvals.ts, server/src/routes/access.ts, server/src/services/approvals.ts, server/src/services/heartbeat.ts, ui/src/pages/NewAgent.tsx, cli/src/commands/client/agent.ts, and the plan directory. Identify any missing or under-specified parity area. Do not implement. Return must-fix plan gaps with file references.
```

## Phase implementer prompt template

```text
Before implementing, read PHASE_FILE plus `01-parity-audit-summary.md`, `04-execution-order.md`, `05-test-strategy.md`, `acceptance/full-parity-harness-spec.md`, `evidence/rewrite-gap-matrix.md`, and `reviews/review-checklist.md`. Implement only the rows owned by PHASE_FILE using strict TDD. First write failing tests that demonstrate the current rewrite does not match original Paperclip behavior. Then implement the minimal code/migrations to pass. Preserve: no public schema, no service-role key, no Supabase/OpenAI details client-side, OpenAI Responses API only. Update each owned matrix row with phase, acceptance_id, failing_test, passing_test, evidence_ref, and final_status. Run the phase checks and report exact output.
```

## Final parity review prompt

```text
Adversarially compare the final rewrite against original Paperclip V1 for agent hiring/governance/runtime/interfaces. Use docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.md as the checklist. Confirm gaps_missing=0 gaps_partial=0 gaps_divergent=0 or list blockers. Do not accept a vertical slice as full parity.
```


## Anti-false-parity rule for Codex workers

A phase worker may not report "done" because a category test passed. Done means every owned row in `evidence/rewrite-gap-matrix.md` is `full` or has an approved `waived` product-contract reference. The worker must include the row IDs it closed and the commands that turned each red test green.
