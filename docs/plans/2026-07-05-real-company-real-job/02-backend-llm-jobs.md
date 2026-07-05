# 02 — Backend: OpenAI Responses client + job runner + endpoints

All new Rust in `backend-rs`. Server holds both the Supabase creds and the OpenAI key; **neither
reaches the client**. codex builds this **TDD** (unit tests with fakes; live tests env-gated).

## Module layout
- `src/llm/mod.rs`, `src/llm/openai.rs` — `LlmClient` trait + `OpenAiResponsesClient` (real, SSE) + config.
- `src/supabase/data.rs` — `DataGateway` trait (`rpc(name, body, Auth)`) + impl on `HttpSupabaseGateway`.
- `src/jobs/mod.rs` — `JobService`: orchestrates auth-principal → data RPC → LLM → data RPC.
- `src/supabase/routes.rs` — add company/job routes; handlers stay thin (pass bearer + body to `JobService`).

## LLM client (OpenAI Responses, SSE)
```rust
pub struct LlmUsage { pub input_tokens: i64, pub output_tokens: i64, pub total_tokens: i64 }
pub struct LlmAnswer { pub text: String, pub model: String, pub usage: LlmUsage }

#[async_trait::async_trait]
pub trait LlmClient: Send + Sync {
    async fn respond(&self, model: &str, prompt: &str) -> anyhow::Result<LlmAnswer>;
}
```
`OpenAiResponsesClient::from_env()` reads `OPENAI_BASE_URL`, `OPENAI_API_KEY`, optional
`OPENAI_MODEL` (default `gpt-5.4-mini`). `respond`:
1. `POST {base}/v1/responses` with `Authorization: Bearer {key}`, body
   `{"model": model, "input": [{"role":"user","content": prompt}]}` — **input is a list**.
2. The endpoint **streams SSE** (streaming is forced). Read the body as text (or a byte stream) and
   parse line-by-line: for every `data:` line, JSON-parse it; if `type == "response.output_text.delta"`
   append `delta` to the answer; if `type == "response.completed"` read `response.usage`
   (`input_tokens`, `output_tokens`, `total_tokens`) and `response.model`.
3. Return `LlmAnswer { text, model, usage }`. On a `response.failed`/`error` event or HTTP≥400, `Err`.
   (Robustness: ignore non-JSON `data:` lines and the `[DONE]` sentinel if present.)

**Test:** a `FakeLlm` returning a canned answer for unit tests; a live test (env-gated
`PAPERCLIP_LIVE_OPENAI=1`) that calls the real endpoint and asserts a non-empty answer + usage.

## Data gateway (PostgREST RPC, paperclip schema)
```rust
pub enum Auth { Anon, Bearer(String) }   // Bearer = a user JWT
#[async_trait::async_trait]
pub trait DataGateway: Send + Sync {
    async fn rpc(&self, name: &str, body: serde_json::Value, auth: Auth) -> anyhow::Result<serde_json::Value>;
}
```
`HttpSupabaseGateway` implements it: POST `{url}/rest/v1/rpc/{name}` with `apikey: anon`,
`Authorization: Bearer {anon|jwt}`, `Content-Profile: paperclip`, `Accept-Profile: paperclip`, JSON body.
(Reuses the existing client; mirrors `whoami`/`resolve_api_key`.)

## JobService — the orchestration (dispatch by principal)
Holds `auth: AuthBroker`, `data: Arc<dyn DataGateway>`, `llm: Arc<dyn LlmClient>`. Every method takes
the **raw bearer** so it can pick the path:
- **User** principal → look up the session's stored JWT (via `AuthBroker`), call the **session RPCs**
  with `Auth::Bearer(jwt)`. Company creation needs a team: use the user's `default_team_id` from whoami.
- **ApiKey** principal → parse the bearer into `(prefix, key_hash)` (existing `parse_api_key`), call the
  **`*_with_key` RPCs** with `Auth::Anon`.

Methods:
- `create_company(bearer, name) -> company_id`
- `list_companies(bearer) -> [company]`
- `run_job(bearer, company_id, prompt, model?) -> { job_id, status, result, usage }`:
  1. create job (running) via the right RPC → `job_id`.
  2. `llm.respond(model, prompt)` → answer (or, on error, `fail_job*` and return failed).
  3. `complete_job*(job_id, answer.text, answer.usage)`.
  4. return `{ job_id, status:"succeeded", result: answer.text, usage }`.
- `list_jobs(bearer, company_id) -> [job]`

**Never** returns Supabase or OpenAI internals — only company/job/result fields.

## Routes (added to the broker router; `Principal` extractor already gates auth)
| Method | Path | Body | Returns |
|---|---|---|---|
| POST | `/api/companies` | `{name}` | `{companyId}` |
| GET  | `/api/companies` | — | `{companies:[…]}` |
| POST | `/api/companies/:companyId/jobs` | `{prompt, model?}` | `{jobId, status, result, usage}` |
| GET  | `/api/companies/:companyId/jobs` | — | `{jobs:[…]}` |

Handlers extract the bearer (existing `bearer_token`) + `Principal` (auth gate) and delegate to
`JobService`. Map `42501`/`28000` errors → 403/401; keep bodies free of internal detail.

## Wiring
- `Repositories` gains `jobs: JobService` (or `AppServices`); `main.rs` constructs it from env
  (`HttpSupabaseGateway` for auth+data, `OpenAiResponsesClient::from_env()` for llm). Default/in-memory
  build uses disabled gateways so the existing suite stays hermetic.

## Tests (codex, TDD)
- **Unit** (fakes, offline): `run_job` happy path (create→llm→complete, returns result+usage);
  llm-error path (job failed, no result); api-key vs session dispatch; company creation; SSE parser
  unit test over a captured sample stream (delta accumulation + usage).
- **Live** (`#[ignore]` + `PAPERCLIP_LIVE_SUPABASE=1` + `PAPERCLIP_LIVE_OPENAI=1`): the real end-to-end
  is driven by the acceptance harness (`05`), not a Rust live test, to keep secrets in the shell.
