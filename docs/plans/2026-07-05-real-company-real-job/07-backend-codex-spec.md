# 07 — Backend codex spec (TDD)

Implement in `backend-rs` per `02-backend-llm-jobs.md` **as amended by `06-eng-review.md`**. Strict TDD
(RED→GREEN→REFACTOR). **API-key path only** (session path is P3). Keep the full existing `cargo test`
green; live tests `#[ignore]` + env-gated. Nothing may leak Supabase or OpenAI internals to a client.

## Deps (add to Cargo.toml if missing)
`reqwest` (already), `serde`/`serde_json` (already), `async-trait` (already), `anyhow` (already).
Dev: `wiremock = "0.6"` only if you choose the mock-HTTP route for the data/llm live-less tests
(prefer pure functions + fakes so no new dep is needed).

## Modules
- `src/llm/mod.rs`, `src/llm/openai.rs`
- `src/supabase/data.rs`
- `src/jobs/mod.rs`
- extend `src/supabase/routes.rs` (routes under `/api/paperclip`) and `src/lib.rs` (wiring + `app_from_env`).

## 1. LLM client — pure SSE parser + real client
```rust
pub struct LlmUsage { pub input_tokens: i64, pub output_tokens: i64, pub total_tokens: i64 }
pub struct LlmAnswer { pub text: String, pub model: String, pub usage: LlmUsage }
#[async_trait::async_trait]
pub trait LlmClient: Send + Sync { async fn respond(&self, model: &str, prompt: &str) -> anyhow::Result<LlmAnswer>; }
```
- **Pure function** `pub fn parse_responses_sse(body: &str) -> anyhow::Result<LlmAnswer>`:
  split on `\n`; for lines starting `data:` strip the prefix + one optional leading space; skip `[DONE]`
  and non-JSON lines; accumulate `type=="response.output_text.delta"` `.delta` into text; on
  `type=="response.completed"` read `response.model` and `response.usage`
  (`input_tokens`,`output_tokens`,`total_tokens`) and mark terminal; on `response.failed`/`response.error`
  → `Err`; on `response.incomplete` → `Err` (or Ok with a flag — choose Err for MVP). If no terminal
  event seen → `Err("no terminal event")`.
- **Unit test (offline, MUST use the committed fixture):**
  `let s = include_str!("../../tests/fixtures/openai_responses.sse"); let a = parse_responses_sse(s).unwrap();`
  assert `a.text == "7"`, `a.usage.total_tokens == 18`, `a.model` starts with `gpt-5.4-mini`.
- `OpenAiResponsesClient::from_env()` reads `OPENAI_BASE_URL`, `OPENAI_API_KEY`, optional `OPENAI_MODEL`
  (default `gpt-5.4-mini`), optional `OPENAI_TIMEOUT_SECS` (default 120). `respond`:
  `POST {base}/v1/responses` (`{base}` already ends without `/v1`; append it), header
  `Authorization: Bearer {key}`, body `{"model":model,"input":[{"role":"user","content":prompt}],"stream":true}`;
  `let body = resp.text().await?; parse_responses_sse(&body)`. Built with
  `reqwest::Client::builder().timeout(Duration::from_secs(timeout)).build()`.
- **Live test** `tests/llm_live.rs` (`#[ignore]`, gated `PAPERCLIP_LIVE_OPENAI=1`): real `respond` returns
  non-empty text + `total_tokens>0`.

## 2. Data gateway
```rust
pub enum Auth { Anon, Bearer(String) }
#[async_trait::async_trait]
pub trait DataGateway: Send + Sync {
    async fn rpc(&self, name: &str, body: serde_json::Value, auth: Auth) -> anyhow::Result<serde_json::Value>;
}
```
`HttpSupabaseGateway` implements it: POST `{url}/rest/v1/rpc/{name}`, headers `apikey: anon`,
`Authorization: Bearer {anon|jwt}`, `Content-Profile: paperclip`, `Accept-Profile: paperclip`, JSON body;
return parsed JSON (reuse `json_response`). Errors from PostgREST (4xx) → `Err`.

## 3. JobService (api-key path only)
Holds `data: Arc<dyn DataGateway>`, `llm: Arc<dyn LlmClient>`. Each method takes the **raw bearer**;
require it parse as an api key via existing `parse_api_key(bearer)` → `(prefix, key_hash)`; if it does
not (e.g. a `pcs_` session) → `Err` (endpoints will map to 401 "api key required").
- `create_company(bearer, name) -> String` → `rpc("create_company_with_key", {p_prefix,p_key_hash,p_name}, Anon)` → company id (a JSON string).
- `list_companies(bearer) -> Value` → `rpc("list_companies_with_key", {...}, Anon)` (array).
- `run_job(bearer, company_id, prompt, model, client_token) -> Value`:
  1. `job_id = rpc("create_job_with_key", {p_prefix,p_key_hash,p_company_id,p_prompt,p_model,p_client_token}, Anon)`.
  2. `match llm.respond(model, prompt).await { Ok(a) => complete, Err(e) => fail }`.
     - complete: `ok = rpc("complete_job_with_key", {..,p_job_id,p_result:a.text,p_usage:{input_tokens,output_tokens,total_tokens}}, Anon)`.
       If `ok==false`, re-read via `list_jobs_with_key` and return the job's actual status.
       Return `{jobId, status:"succeeded", result:a.text, usage:{…}}`.
     - fail: **sanitize** — `let msg = "llm_request_failed";` (do NOT put `e` in the RPC); `log`
       `e` server-side (e.g. `eprintln!`); `rpc("fail_job_with_key", {..,p_error:msg}, Anon)`; return
       `{jobId, status:"failed", error:msg}`.
- `list_jobs(bearer, company_id) -> Value` → `rpc("list_jobs_with_key", {..,p_company_id}, Anon)`.
- Model defaulting: if the request omits `model`, use the client's default (`gpt-5.4-mini`).

**Unit tests (offline):** a `FakeLlm` (canned answer / forced error) and a **spy `FakeData`** recording
every `(name, body, auth)`:
- happy path: asserts the RPC call sequence is exactly `create_job_with_key` then `complete_job_with_key`
  with the **exact** JSON param keys, and the returned JSON has `status:"succeeded"`, `result`, `usage`.
- llm-error path: asserts `fail_job_with_key` was called and the `p_error` value contains no `://`,
  `Bearer`, or `sk-`; returned `status:"failed"`.
- `create_company`: asserts `create_company_with_key` called with `p_name`.
- non-api-key bearer (`pcs_…`) → `run_job` returns `Err`.

## 4. Routes (under `/api/paperclip` — DO NOT collide with the in-memory `/api/companies`)
Add to the broker router:
| Method | Path | Body | Returns |
|---|---|---|---|
| POST | `/api/paperclip/companies` | `{name}` | `{companyId}` |
| GET  | `/api/paperclip/companies` | — | `{companies:[…]}` |
| POST | `/api/paperclip/companies/:companyId/jobs` | `{prompt, model?, clientToken?}` | `{jobId,status,result,usage}` |
| GET  | `/api/paperclip/companies/:companyId/jobs` | — | `{jobs:[…]}` |

Handlers extract the bearer (existing `bearer_token`) and delegate to `JobService`. Map JobService errors:
api-key-required / invalid key → 401; `42501` in PostgREST error → 403; else 502 with a generic body.
**Never** echo the OpenAI key, Supabase URL/anon key, JWT, or `key_hash`.

## 5. Wiring (testable)
- `Repositories` gains `jobs: JobService` (Clone; default constructed with Disabled/Fake gateways so the
  hermetic suite is unaffected).
- `pub fn app_from_env() -> Router` in `lib.rs`: if `SUPABASE_URL`+`SUPABASE_ANON_KEY` set, build
  `HttpSupabaseGateway` for auth **and** data; if `OPENAI_API_KEY`+`OPENAI_BASE_URL` set, build
  `OpenAiResponsesClient::from_env()`; assemble `Repositories` with a real `AuthBroker` + `JobService`;
  else fall back to disabled/in-memory. `main.rs` calls `app_from_env()`.
- Keep `app()`/`Repositories::default()` fully in-memory (disabled gateways) for existing tests.

## Definition of done
- New unit tests written test-first (parser-over-fixture, run_job happy + llm-error + dispatch, create/list).
- `cargo test` fully green (existing suite unaffected); `cargo build` clean.
- Live tests compile and pass with the env set (run by the guiding agent).
- No client-facing response contains a Supabase or OpenAI secret, URL, JWT, or key_hash.
- Print a summary of files changed + passing test count.
