# 12 — backend-rs Supabase Auth Broker (codex TDD spec)

**For the implementing agent (codex, gpt-5.5, xhigh).** Implement the server-mediated auth broker in `backend-rs` **strictly test-first** (RED → GREEN → REFACTOR). This is the "later slice" the existing `src/auth.rs` promises: *"a hashed, Postgres-backed lookup replaces [the in-memory AgentKeyStore]."* Here that backing store is the `paperclip` Supabase schema, reached only through this server. **Clients must never see Supabase URLs, keys, or JWTs** — only an opaque `pcs_…` session token.

## Non-negotiable invariants
1. No Supabase detail crosses the client boundary. Responses expose only: the opaque `pcs_` session token and the `whoami` payload (user + teams). Never the GoTrue JWT, refresh token, project URL, or anon key.
2. Server uses **only** the anon/publishable key + user JWT. No service-role key. (Matches the DB layer: `docs/plans/2026-07-05-paperclip-supabase-auth/`.)
3. All PostgREST calls target the **`paperclip`** schema via `Content-Profile: paperclip` / `Accept-Profile: paperclip`.
4. `cargo test` stays green with **no network** (unit tests use a fake gateway). Live tests are `#[ignore]` AND gated on `PAPERCLIP_LIVE_SUPABASE=1`.
5. Match existing style: axum `0.7`, handlers returning `Json<Value>`, `FromRequestParts` extractors, `tower::ServiceExt::oneshot` tests, module-per-file under `src/`.

## Dependencies to add (Cargo.toml)
- `reqwest = { version = "0.12", default-features = false, features = ["json", "rustls-tls"] }`
- `sha2 = "0.10"`, `hex = "0.4"`
- `async-trait` (already transitively available via axum? add explicitly: `async-trait = "0.1"`)
- dev: `base64` if needed for token gen (or use existing `uuid` + `rand`); prefer `getrandom`/`rand` for 32 random bytes. Add `rand = "0.8"`.

## Module layout
- `src/supabase/mod.rs` — re-exports.
- `src/supabase/gateway.rs` — `SupabaseGateway` trait + DTOs + `HttpSupabaseGateway`.
- `src/supabase/broker.rs` — `AuthBroker`, `SessionStore`, `InMemorySessionStore`, token minting, `Principal`.
- `src/supabase/keys.rs` — `parse_api_key`, `hash_full_key` (sha256 hex).
- `src/supabase/routes.rs` — axum handlers + `Principal` extractor + `auth_routes()` returning a `Router`.
- Wire `auth_routes()` into `app_with` (nest under existing router; keep current routes intact).

## DTOs
```rust
pub struct GoTrueSession { pub access_token: String, pub refresh_token: String,
  pub expires_in: i64, pub user_id: String }
pub struct SignUpOutcome { pub user_id: Option<String>, pub needs_confirmation: bool }
pub struct ResolvedKey { pub api_key_id: String, pub team_id: String,
  pub created_by: Option<String>, pub subject_type: String,
  pub agent_id: Option<String>, pub scopes: serde_json::Value, pub scope_config: serde_json::Value }
```

## Trait
```rust
#[async_trait::async_trait]
pub trait SupabaseGateway: Send + Sync {
    async fn sign_up(&self, email: &str, password: &str) -> anyhow::Result<SignUpOutcome>;
    async fn sign_in_password(&self, email: &str, password: &str) -> anyhow::Result<GoTrueSession>;
    async fn refresh(&self, refresh_token: &str) -> anyhow::Result<GoTrueSession>;
    async fn sign_out(&self, access_token: &str) -> anyhow::Result<()>;
    async fn whoami(&self, access_token: &str) -> anyhow::Result<serde_json::Value>; // rpc/whoami
    async fn resolve_api_key(&self, prefix: &str, key_hash: &str) -> anyhow::Result<Option<ResolvedKey>>;
}
```
(Use `anyhow` — add `anyhow = "1"`. Map transport/4xx to `Err`; `resolve_api_key` returns `Ok(None)` on empty.)

### HttpSupabaseGateway
- `new(url, anon_key)` from `SupabaseConfig::from_env()` (`SUPABASE_URL`, `SUPABASE_ANON_KEY`).
- GoTrue: `POST {url}/auth/v1/signup`, `POST {url}/auth/v1/token?grant_type=password`, `.../token?grant_type=refresh_token`, `POST {url}/auth/v1/logout`. Header `apikey: <anon>`; for logout `Authorization: Bearer <access>`.
- PostgREST RPC: `POST {url}/rest/v1/rpc/whoami` with `apikey`, `Authorization: Bearer <access>`, `Content-Profile: paperclip`. `resolve_api_key` uses `apikey`+`Authorization: Bearer <anon>` (anon path) + `Content-Profile: paperclip`, body `{p_prefix, p_key_hash}`, returns first row or None.

## Broker
- `mint_session_token()` → `"pcs_" + base64url_nopad(32 random bytes)`. MUST NOT be a JWT and MUST NOT equal the access token.
- `StoredSession { access_token, refresh_token, expires_at: std::time::SystemTime, user_id }`.
- `SessionStore` trait (put/get/delete) + `InMemorySessionStore` (`Arc<Mutex<HashMap<String, StoredSession>>>`).
- `AuthBroker<G: SupabaseGateway, S: SessionStore>`:
  - `register(email,password) -> SignUpOutcome`.
  - `login(email,password) -> (session_token, whoami_json)`: gateway sign-in → store session (expires_at = now + expires_in) → gateway.whoami → return opaque token + whoami.
  - `logout(session_token)`: gateway.sign_out(access) + store.delete.
  - `authenticate(bearer: &str) -> Result<Principal>`:
    - `pcs_…` → load session; if `expires_at` within 60s of now, `gateway.refresh` and update store; `Principal::User { user_id, whoami }` (call whoami with current access).
    - `paperclip_…` → `parse_api_key` → `gateway.resolve_api_key` → `Principal::ApiKey{..}` or 401 on None.
    - else → 401.
- `Principal`:
```rust
pub enum Principal {
  User { user_id: String, whoami: serde_json::Value },
  ApiKey { team_id: String, subject_type: String, agent_id: Option<String>, scopes: serde_json::Value, scope_config: serde_json::Value },
}
```

## keys.rs
- Full key format: `paperclip_<prefix>_<secret>` where `<prefix>` itself is like `pc_ab12cd34`. `parse_api_key(full) -> Option<ApiKeyParts{ prefix, key_hash }>`: require the `paperclip_` marker; `prefix` = the `pc_…` segment (the substring the DB stores in `api_keys.prefix`); `key_hash` = `sha256_hex(full)`. Include a known-answer sha256 test vector.

## Routes
- `POST /api/auth/register` `{email,password}` → `{ needs_confirmation, user_id? }`.
- `POST /api/auth/login` `{email,password}` → `{ session: "pcs_…", whoami: {...} }`; 401 on bad creds.
- `POST /api/auth/logout` (Bearer `pcs_…`) → `{ ok: true }`.
- `GET  /api/auth/session` (Bearer `pcs_…` **or** `paperclip_…`) → the `Principal` as JSON: `{ "kind":"user", ...whoami }` or `{ "kind":"apiKey", "teamId":..., "subjectType":..., ... }`; 401 otherwise.
- `Principal` extractor via `FromRequestParts` reading `Authorization: Bearer …`, using an `AuthBroker` from router state. Keep the broker generic OR box it: `Arc<dyn SupabaseGateway>` + `Arc<dyn SessionStore>` so it fits axum state cleanly.

## Tests — write these FIRST (unit; fake gateway; must pass offline)
`tests/supabase_auth.rs`:
1. `login_issues_opaque_session_token` — token starts `pcs_`, `!= fake.access_token`, not three JWT segments.
2. `session_bearer_authenticates_as_user` — `GET /api/auth/session` with the login token → `kind:"user"`, whoami email present.
3. `api_key_bearer_authenticates_as_team` — fake resolves `paperclip_pc_x_y` → `kind:"apiKey"`, correct `teamId`/`subjectType`.
4. `unknown_api_key_is_unauthorized` — 401.
5. `logout_invalidates_session` — after logout, session bearer → 401; fake recorded `sign_out`.
6. `expired_session_is_refreshed` — seed a session expiring now; authenticate → fake `refresh` called once, store updated, still `kind:"user"`.
7. `register_reports_confirmation` — fake sign_up `needs_confirmation:true` → body reflects it.
8. `bad_password_is_unauthorized` — fake sign-in Err → login 401.
9. `no_supabase_leak` — login/session responses contain neither the fake access token nor `SUPABASE`/URL strings.
10. keys: `parse_api_key` happy + reject non-`paperclip_`; sha256 known-answer.

Fake gateway records calls (e.g. `Arc<Mutex<Vec<String>>>`), returns canned `GoTrueSession { access_token:"fake.jwt.tok", ... }`, canned whoami `{"user":{"email":"x@y.z"},"teams":[...]}`, and a programmable `resolve_api_key` map.

## Tests — live (real Supabase; `#[ignore]` + env-gated)
`tests/supabase_auth_live.rs` — each begins:
```rust
if std::env::var("PAPERCLIP_LIVE_SUPABASE").ok().as_deref() != Some("1") { return; }
```
Reads `SUPABASE_URL`, `SUPABASE_ANON_KEY`, `PC_TEST_EMAIL`, `PC_TEST_PASSWORD` from env. Cases:
- `live_login_and_whoami` — real login of `PC_TEST_EMAIL` → session → `GET /api/auth/session` → `kind:"user"`, email matches, at least one team.
- `live_bad_password_rejected` — wrong password → 401.
Run manually:
```
PAPERCLIP_LIVE_SUPABASE=1 SUPABASE_URL=… SUPABASE_ANON_KEY=… \
  PC_TEST_EMAIL=testuser1_paperclip@gmail.com PC_TEST_PASSWORD='…' \
  cargo test --test supabase_auth_live -- --ignored --nocapture
```

## Definition of done
- New unit tests written first, each seen to fail, then made to pass.
- `cargo test` fully green (unit + existing suite unaffected).
- `cargo build` clean; no clippy-obvious dead code.
- Live tests compile and pass when run with the env above (the guiding agent runs these against the real project).
- No Supabase secret or JWT reachable from any client-facing response.
