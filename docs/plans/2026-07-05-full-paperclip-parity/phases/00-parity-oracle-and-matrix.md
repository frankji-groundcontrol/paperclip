# Phase 00 — Parity Oracle and Matrix

## Goal

Freeze the source-backed parity oracle before implementation so workers cannot declare success from a green vertical slice.

## Required artifacts (already scaffolded; Phase 00 must finish them)

- `evidence/rewrite-gap-matrix.md` + `evidence/rewrite-gap-matrix.jsonl`: row-level matrix with original evidence, current rewrite evidence, status, owning phase, red test, green command, acceptance ID, waiver field. Today it carries 220 rows across all 14 phases.
- `evidence/migration-ledger.md`: tracks every Supabase migration number and purpose.
- `acceptance/run_original_gap_diff.py`: the ledger gate. Implemented now; fails on the current rewrite and prints exactly `gaps_missing=0 gaps_partial=0 gaps_divergent=0` only on success.
- `acceptance/run_real_user_real_agent.py`: RED scaffold. Phase 00 must replace the body with the live Supabase + real OpenAI Responses driver.
- `acceptance/run_cli_mcp_surface.py`: RED scaffold. Phase 00 must implement the CLI/MCP surface parity check.
- `acceptance/run_frontend_user_flow.py`: RED scaffold. Phase 00 must implement the Nuxt board-flow parity check.
- No-secret scanner (see `acceptance/no-secret-scanner-spec.md`): Phase 00 must implement it.

## Tasks

1. Confirm matrix rows cover all 14 phases (done: 220 rows). Refine row evidence as later phases cite exact source lines.
2. Classify rows as `v1-core`, `current-addendum`, `post-v1`, or `not-in-v1`.
3. Implement the four acceptance harnesses above so each is a real driver, not a RED scaffold.
4. Implement the no-secret scanner.
5. Add waiver format requiring product-spec update and reviewer signoff.

## Acceptance

- `run_original_gap_diff.py` runs today, fails with non-zero counts, and on eventual success prints exactly `gaps_missing=0 gaps_partial=0 gaps_divergent=0`.
- Phase 00 is not closed until `run_real_user_real_agent.py`, `run_cli_mcp_surface.py`, `run_frontend_user_flow.py`, and the no-secret scanner are real drivers (the current RED scaffolds intentionally return non-zero).
- Final release requires all blocking rows full or explicitly waived by an updated product contract.
