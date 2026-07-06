# Codex Plan Review

## BLOCKERS

1. `README.md` said the revision expands to **17 owned phases**, but `04-execution-order.md` says the locked plan uses **fourteen phases**, and the `phases/` directory contains 14 files: `00` through `13`. This is a plan-control failure. Phase ownership cannot be trusted until the count and scope are reconciled.

2. Gap count closure was inconsistent. `04-execution-order.md` requires `gaps_missing=0 gaps_partial=0 gaps_divergent=0`, while `05-test-strategy.md` said the final diff must print `missing=0 partial=0 divergent=0`. The harness spec uses the `gaps_*` names. That mismatch creates room for false pass conditions.

3. The harness spec listed high-level assertions but did not require every audit baseline row from `3 full / 50 partial / 100 missing / 22 divergent` to have an acceptance ID, owner phase, red test, green test, and final evidence row. Category-level checks could pass without proving row-level zero-gap parity.

4. `codex/phase-prompts.md` did not require phase implementers to load the audit summary, matrix, execution order, harness spec, and review checklist before coding. That weakens Codex/TDD executability and lets phase workers miss inherited invariants.

## PATCHES APPLIED

1. Reconciled phase count to 14 everywhere.
2. Standardized final diff contract to `gaps_missing=0 gaps_partial=0 gaps_divergent=0`.
3. Added hard row-level acceptance rule requiring `{phase, acceptance_id, failing_test, passing_test, evidence_ref, final_status}` for every non-full audit row.
4. Strengthened Codex phase prompts so implementers must read global plan context, write failing tests first, update matrix/evidence, and run targeted green checks.
5. Added final anti-false-parity gate: category checks are insufficient unless the row-level matrix has zero partial/missing/divergent rows and no unreviewed waivers.

## VERDICT

Initial verdict: FAIL. Patches applied in this review cycle; re-review expected after patch verification.
