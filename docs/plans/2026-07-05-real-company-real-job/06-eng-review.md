# 06 — plan-eng-review: findings + revisions (authoritative)

Adversarial multi-agent review (37 agents; 30 verified findings). Decisions below **supersede** the
relevant parts of `01`–`05`. Implementers follow the base plan **as amended here**.

## Blockers (HIGH) — must fix before/while building
1. **Route collision (`/api/companies` already exists in the Phase-1 in-memory port → axum panics).**
   → **Mount the entire new real surface under `/api/paperclip/`**: `POST/GET /api/paperclip/companies`,
   `POST/GET /api/paperclip/companies/:companyId/jobs`. Leave the in-memory `/api/companies` subtree
   untouched. Update `02` + `05` paths accordingly. The new `paperclip.companies` is a *team-scoped*
   concept distinct from the ported in-memory "company".
2. **OpenAI request must send `"stream": true`.** Body = `{"model", "input":[…], "stream": true}`.
   (The proxy streamed anyway in the probe, but be explicit.)
3. **Jobs can hang in `running`.** → Make jobs **synchronous & honest**: drop the `queued` enum value,
   default `jobs.status = 'running'`, `run_job` always reaches `succeeded` or `failed` within the
   request. Add a safety reaper RPC `fail_stale_jobs(p_older_than interval default '10 minutes')`
   (SECURITY DEFINER) callable on startup; document it. Drop `started_at` (== created_at) confusion —
   keep `created_at`/`completed_at` only.
4. **No acceptance row for the LLM-failure → `fail_job` branch.** → Add **E2b**: run a job with a
   rejected model (`gpt-4o`) → assert HTTP 200-with-`status:"failed"` (or 502) and a `jobs` row with
   `status='failed'`, a **sanitized** `error`, and **no** result.

## Security (MEDIUM/LOW) — folded in
5. **API-key path privilege is implicit.** MVP **decision (documented):** *any valid, non-revoked key
   grants full company/job read+write on its team* — this is the intended agent happy path, and key
   *minting* is already role-gated (`create_api_key` needs owner/admin/operator). Cross-team is blocked.
   To keep the door open: `fn_key_principal` **also returns `scopes`, `scope_config`, `subject_type`**;
   a `TODO(scopes)` note marks where per-scope enforcement lands later. Add a test asserting a normally
   minted key performs the intended actions (intra-team least-privilege is a future scope layer).
6. **`fn_key_principal` inherits PUBLIC EXECUTE.** → Immediately after CREATE, `revoke execute … from
   public, anon, authenticated`. Verify `has_function_privilege('anon', …) = false`.
7. **Job `error` text can leak `OPENAI_BASE_URL`/upstream body.** → `run_job` passes a **sanitized**
   message to `fail_job*` (e.g. `"llm_request_failed"` + HTTP status class); the full `reqwest`/`anyhow`
   error is `log`ged server-side only, never through an RPC or response. Unit test asserts persisted
   `error` contains no `://`, no `Bearer`, no `sk-`.
8. **Session RPCs lack the `auth.uid()` null guard** (0009 pattern). → Add
   `if v_uid is null then raise … '28000'` first in `create_company/create_job/complete_job/fail_job`.
9. **Slug suffix only 24 bits.** → widen to `gen_random_bytes(6)`; keep the unique constraint.
10. **Anon-reachable mutation surface.** → Security note in `01`/`05`: rate-limit `/api/paperclip/*` at
    the broker tier; ensure `p_key_hash` is not captured by statement/proxy body logging.

## Correctness (MEDIUM/LOW) — folded in
11. **`run_job` ignored `complete_job`'s boolean** → capture it; on `false`, re-read the job and return
    its actual persisted status, or 409. Never assert `"succeeded"` blindly.
12. **SSE parsing** → make it a **pure function** `fn parse_responses_sse(body: &str) ->
    anyhow::Result<LlmAnswer>`; `respond()` does `resp.text().await?` then calls it. Rules: split on
    `\n`; for `data:` lines strip the prefix + one optional space; ignore non-JSON lines and `[DONE]`;
    accumulate `response.output_text.delta`; **require a terminal event** — `response.completed`
    (→ usage) ; on `response.incomplete` return the partial text + incomplete flag or `Err`; on
    `response.failed`/error `Err`; if the stream ends with no terminal event → `Err`.
13. **LLM client** built with `reqwest::Client::builder().timeout(OPENAI_TIMEOUT_SECS default 120)`;
    a timeout is a job failure. Define `run_job` step-3: if `complete_job*` fails after a good LLM
    answer, attempt `fail_job*` with a persistence note and return 5xx (do not claim success).
14. **Idempotency (double-charge).** Add optional `p_client_token text` to `create_job*`; partial
    unique index `unique(team_id, company_id, client_token) where client_token is not null`; on replay
    return the existing job id and short-circuit `run_job`. Client may omit it (MVP: CLI/MCP send a
    per-invocation nonce).

## Architecture (MEDIUM/LOW) — folded in
15. **Scope P1/P2 to the API-KEY path only** (the actual gate). Drop the session/JWT branch of
    `JobService` from this milestone → removes the "AuthBroker doesn't expose the JWT" problem. The
    human/session path ships in **P3** with the frontend, at which point add
    `AuthBroker::session_access_token(bearer) -> Option<String>` as the seam. (E11 → P3.)
16. **Testable wiring:** add `app_from_env()` (and `Repositories::from_env()`) in `lib.rs` that selects
    real gateways/LLM when `SUPABASE_URL`/`OPENAI_*` are set, else the Disabled/Fake variants; `main.rs`
    calls only that. `Repositories::default()` stays fully in-memory so the hermetic suite is unchanged.
17. **`DataGateway`** stays the seam, but `FakeData` is a **spy** recording each `(name, body, auth)`;
    unit tests assert the **exact** RPC name (`create_job_with_key`, `fail_job_with_key`, …) and JSON
    param keys, so a rename is caught offline (mitigates the stringly-typed risk).
18. **MCP** — specify a **minimal but complete** stdio JSON-RPC server (or depend on `rmcp`): fixed
    `protocolVersion`, tools-only capabilities, `serverInfo`, accept-and-ignore `notifications/*`,
    newline-delimited framing. Not "thin" — this is a real handshake.

## Testability — folded into `05`
19. `parse_responses_sse` gets a committed fixture `backend-rs/tests/fixtures/openai_responses.sse`
    (redacted capture from the live probe) as the offline test input.
20. Tighten **E1** with a DB row assertion (company `team_id` = testuser1 team, `created_by` = key
    principal). Extend **E3** to assert the job's `subject_type`/`created_by` match the key principal.
21. Rewrite **E6 (CLI)/E7 (MCP)**: deterministic prompt ("Reply with only the number 7"),
    nonce-named company, assert result contains `7` + `usage.total_tokens>0` **and** a DB query finds the
    company + job rows on testuser1's team with `status='succeeded'`.
22. Split **E10**: (a) malformed bearer → gate `401`; (b) direct `rpc/create_job_with_key` with a
    bogus/revoked `(prefix,key_hash)` → SQLSTATE `28000`.
23. **Device-login** is **out of scope** for this milestone's acceptance (proven in the prior auth
    acceptance A10); `05` uses a directly-minted key. Reword `00` step 1 accordingly.

## Net effect on the base plan
- All new HTTP under `/api/paperclip/…`.
- `01`: `job_status` = `running|succeeded|failed`; `fn_key_principal` returns scopes/scope_config/
  subject_type + explicit revoke; session RPC null guards; slug 6 bytes; optional `client_token`
  idempotency; `fail_stale_jobs` reaper. Migration name `paperclip_0011_companies_jobs`.
- `02`: `stream:true`; pure `parse_responses_sse` + terminal-event rule + timeout; sanitized failure;
  `run_job` honors `complete_job`; **api-key path only**; `app_from_env()` wiring; spy `FakeData`.
- `03`: real MCP handshake (or `rmcp`); CLI/MCP tests via `wiremock` or in-process router.
- `05`: E1 DB assertion, E2b failure row, E6/E7 tightened + DB checks, E10 split, E11→P3, device-login noted.

**Verdict:** approved to implement with the above amendments. Scope is a real vertical slice (agent →
company → OpenAI job), not creep; the session/frontend path is correctly deferred to P3.
