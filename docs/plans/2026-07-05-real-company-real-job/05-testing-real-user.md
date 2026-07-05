# 05 — Real-user acceptance: real company, real OpenAI job

"Make sure it works." Every row runs against the **real** `supabase-franky` DB and the **real**
OpenAI Responses endpoint, as a **real user on the agent (API-key) path**. No mocks in acceptance.

## Setup
1. `main.rs` wires real gateways when env is present: `HttpSupabaseGateway` (SUPABASE_URL/ANON) for
   auth+data, `OpenAiResponsesClient::from_env()` for the LLM. Run the server on `127.0.0.1:8787`.
2. Mint a real API key for **testuser1** on their default team (via PostgREST `create_api_key`, as in
   the auth acceptance) → `paperclip_<prefix>_<secret>`. This is the agent's key.

## Matrix (each must PASS with real data + real LLM)
| # | Scenario | Assertion |
|---|----------|-----------|
| E1 | `POST /api/companies {name:"Acme"}` with the key | 200 `{companyId}`; a real `paperclip.companies` row on testuser1's team |
| E2 | `POST /api/companies/:id/jobs {prompt:"What is 2+2? Reply with only the number."}` | 200 `{jobId, status:"succeeded", result, usage}`; `result` contains `4`; `usage.total_tokens>0` |
| E3 | DB check of the job row | `status='succeeded'`, `result_text` set (real LLM text), `usage` tokens>0, `team_id`=testuser1 team, `model`=`gpt-5.4-mini` |
| E4 | `GET /api/companies/:id/jobs` | lists the job with its result |
| E5 | second prompt, a real task ("Summarize relativity in one sentence.") | 200; `result` is a real, non-trivial sentence (len>20) from OpenAI |
| E6 | **CLI**: `paperclip company create` + `paperclip job run` with the key | prints a real company id + a real LLM answer (same flow, through the binary) |
| E7 | **MCP**: `paperclip_create_company` then `paperclip_run_job` (JSON-RPC over stdio) | returns `{companyId}` then `{result,…}` with a real answer |
| E8 | RLS isolation: **testuser2** key → `GET /api/companies` | Acme **not** present; `list_jobs_with_key` shows none of Acme's jobs |
| E9 | cross-team: `create_job_with_key(testuser2 key, Acme companyId, …)` | `42501` (company not in key's team) |
| E10 | bad/revoked key → `POST /api/companies` | 401/`28000`; revoke the key → subsequent job run fails |
| E11 | **session path** parity (sanity): login testuser1 via broker → `POST /api/companies` (uses JWT + default team) → run a job | 200; real result (proves the human/frontend path too) |
| E12 | leak check | no response body contains the OpenAI key, Supabase URL/anon key, JWT, or `key_hash` |

## Live LLM unit proof (backend)
`PAPERCLIP_LIVE_OPENAI=1 cargo test --test llm_live -- --ignored` — the `OpenAiResponsesClient` hits the
real endpoint and returns a non-empty answer + usage (SSE parse proven against the live stream).

## Cleanup
Delete the Acme test company (cascades its jobs) and revoke the minted key after the run; keep the run
summary in `RESULTS.md`. Never commit the OpenAI key or the test password.
