# Full Parity Harness Spec

## Harness inputs

- Supabase URL and anon key for `supabase-franky` only.
- OpenAI-compatible `OPENAI_BASE_URL` and `OPENAI_API_KEY` from environment.
- `test_users.json` with gitignored real test users.
- Rust backend URL.
- Nuxt frontend URL when frontend smoke runs.

## Required assertions

1. Schema introspection: no app objects in `public`; expected tables/functions/enums in `paperclip`; helpers in `paperclip_private`.
2. Defaults/enums match original: company hire approval default false; canonical statuses/types/statuses.
3. Authz matrix: board/session/agent/viewer/low-trust/cross-company behavior.
4. Hire/create: direct create when approval default false; approval flow when enabled.
5. Approval: revision/request/approve/reject/cancel for all types.
6. Runtime: wakeup/run/cancel/logs/status transitions.
7. Budget: cost event, hard-stop auto-pause, budget override approval.
8. Secrets/config: redaction and Codex isolation.
9. Work: task assignment, checkout, comment/work product.
10. Interfaces: HTTP, CLI, MCP, Nuxt.
11. Real OpenAI Responses run: result persisted under run/job/work product and attributed to agent.

## Final pass condition

```text
schema=pass authz=pass lifecycle=pass approvals=pass runtime=pass budgets=pass secrets=pass tasks=pass interfaces=pass live_openai=pass gaps_missing=0 gaps_partial=0 gaps_divergent=0
```


## Row-level matrix proof

The harness must read `evidence/rewrite-gap-matrix.md` (or its generated machine-readable companion) and assert every non-full audit row has:

- `phase`
- `acceptance_id`
- `failing_test`
- `passing_test`
- `evidence_ref`
- `final_status`

Allowed final statuses are only `full` or `waived`. Waived rows require a canonical product-contract reference and reviewer signoff. Category-level checks cannot override a row-level non-full status.

## Exact final output

The final line must include the exact fields:

```text
gaps_missing=0 gaps_partial=0 gaps_divergent=0
```

Any spelling variant, omitted prefix, or category-only summary fails the harness.
