# Test Strategy

## TDD rule

Each phase begins with failing tests that encode original parity. Production changes are not accepted until those tests pass.

## Test layers

### Unit tests

- Authz predicate matrix.
- Status transition functions.
- Approval decision side effects.
- Adapter config normalization/redaction.
- Secret ref validation.
- Budget hard-stop calculations.

### Route/service tests

- Rust axum route tests for every endpoint.
- Supabase gateway spy tests proving session vs API-key dispatch.
- Error mapping tests for 400/401/403/404/409/422.

### Migration/RLS tests

- Live Supabase introspection for schemas/tables/functions/grants.
- RLS tests for board user, normal member, viewer, agent key, low-trust agent key, cross-company attempts.
- SECURITY DEFINER `search_path=''` checks.

### Frontend tests

- Composable tests for every API client.
- Component/page tests for hire, approvals, lifecycle, runtime, access, budget, tasks.
- No Supabase/OpenAI details in rendered output or responses.

### CLI/MCP tests

- CLI command parsing and server calls.
- MCP tool schemas and JSON-RPC smoke.
- Agent-key vs board-session permissions.

### Live acceptance

- Real users: `testadmin1`, `testuser1`, `testuser2`, `testuser3` per `test_users.json` (gitignored).
- Real company.
- Real agent hire/create.
- Real board approval where required.
- Real OpenAI Responses run.
- DB attribution and lifecycle assertion.

## Required final commands

Backend:

```sh
cd backend-rs
cargo test
cargo build --bins
```

Frontend:

```sh
cd frontend-nuxt
npx vitest run
```

Acceptance:

```sh
python3 docs/plans/2026-07-05-full-paperclip-parity/acceptance/run_original_gap_diff.py
python3 docs/plans/2026-07-05-full-paperclip-parity/acceptance/run_real_user_real_agent.py
```

The final gap diff must print exactly: `gaps_missing=0 gaps_partial=0 gaps_divergent=0`.


## Row-level anti-false-parity gate

Category checks are not enough. Every non-full audit row from the authoritative ledger (`3 full / 50 partial / 145 missing / 22 divergent` across 220 rows in `evidence/rewrite-gap-matrix.jsonl`) must carry:

```text
phase=<owner> acceptance_id=<id> failing_test=<path::name> passing_test=<command> evidence_ref=<source+rewrite> final_status=<full|waived>
```

A row may move to `full` only after the failing test has been observed red, the implementation makes it green, source evidence is cited, and the reviewer updates `evidence/rewrite-gap-matrix.md`. A row may move to `waived` only after `doc/SPEC-implementation.md` or another canonical product contract is updated to say the original behavior is no longer required.
