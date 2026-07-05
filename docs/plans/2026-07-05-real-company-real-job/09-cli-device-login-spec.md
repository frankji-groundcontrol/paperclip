# 09 — CLI device-login (`paperclip login`) — codex TDD spec

Complete the deferred agent-onboarding flow so an agent can `paperclip login` instead of hand-setting
a key. The DB RPCs already exist and are proven (`cli_start_device_login`, `cli_approve_device_login`,
`cli_poll_device_login`). Implement the **backend proxy endpoints** + the **CLI command**, strict TDD.
Keep the full existing `cargo test` green; the CLI never sends plaintext at rest (it generates its own
key locally and transmits only hashes).

## Design (server-mediated; no Supabase detail at the client)
The CLI generates locally: a `device_secret`, a `user_code`, and the pending API key
`paperclip_<prefix>_<secret>`. It transmits only sha256 **hashes**; the plaintext key stays on the CLI.
A logged-in human (browser/session) approves the `user_code`; the CLI then owns the working key.

### Backend endpoints (add to the broker router)
- `POST /api/cli/start` (no auth) — body `{ secretHash, userCodeHash, pendingKeyPrefix, pendingKeyHash,
  pendingKeyName, deviceName?, requestedAccess?, teamId? }` → `DataGateway::rpc("cli_start_device_login",
  { p_secret_hash, p_user_code_hash, p_pending_key_prefix, p_pending_key_hash, p_pending_key_name,
  p_device_name, p_requested_access, p_team_id }, Auth::Anon)` → `{ ok: true }`.
- `POST /api/cli/poll` (no auth) — body `{ secretHash }` → `rpc("cli_poll_device_login",
  { p_secret_hash }, Auth::Anon)` → return the poll row `{ status, prefix?, teamId?, userId? }`
  (map snake→camel; never leak internal hashes).
- `POST /api/cli/approve` (**session auth**: `Principal::User` via `pcs_` bearer) — body `{ userCodeHash }`
  → resolve the session JWT (`AuthBroker::session_access_token`), then
  `rpc("cli_approve_device_login", { p_user_code_hash }, Auth::Bearer(jwt))` (so `auth.uid()` is the
  approver) → `{ ok: true }`. Reject a non-session bearer with 401.

### Service wiring
Add `CliAuthService { data: Arc<dyn DataGateway>, auth: AuthBroker }` to `Repositories` (Clone; default
uses Disabled gateways). Wire it in `app_from_env()` sharing the SAME `HttpSupabaseGateway` + `AuthBroker`.
Handlers stay thin. Map errors: unknown-session/`28000` → 401, `42501` → 403, else 502 (reuse the
existing `map_job_error` discipline; no internal detail in bodies).

### CLI `paperclip login` (in `src/bin/paperclip.rs`)
```
paperclip login [--name <device>] [--team <TEAM_ID>] [--server URL]
```
1. Generate: `secret` (32 rand bytes, base64url), `user_code` (e.g. 8 hex), `prefix` (`pc_`+8 hex),
   `full = paperclip_<prefix>_<secret2>`. Compute sha256 hex of each hash input.
2. `POST /api/cli/start` with the hashes + `pendingKeyPrefix=prefix`, `pendingKeyHash=sha256(full)`.
3. Print: `Approve this device: run \`paperclip approve <USER_CODE>\` while signed in, or open the
   web approve page. Waiting…` and show the `user_code`.
4. Poll `POST /api/cli/poll { secretHash }` every ~2s (bounded, e.g. 5 min) until `status=="approved"`.
5. On approved: persist `full` to the credentials file (reuse `config set-key` path). Print success.
6. On `expired`/timeout: exit non-zero with a clear message.

(Approval by a signed-in user can be exercised in tests/acceptance via `POST /api/cli/approve` with a
`pcs_` session; a `paperclip approve` subcommand or web page is optional polish.)

## Tests (TDD)
- **Unit (offline, spy `FakeData` + fake `AuthBroker` session):** `start` calls
  `cli_start_device_login` with the exact `p_*` keys and `Auth::Anon`; `poll` maps the row to camelCase;
  `approve` resolves the session JWT and calls `cli_approve_device_login` with `Auth::Bearer(jwt)`;
  a non-session bearer on approve → error. Reuse the `jobs_service.rs` fake patterns.
- **CLI arg/build tests** where practical (hash generation is deterministic given injected randomness —
  or test the request-building given fixed inputs).
- Keep every existing test green; live/end-to-end device-login is exercised by the guiding agent
  against the real project (start → session-approve as testuser1 → poll approved → the minted key
  creates a company).

## Definition of done
- New endpoints + `CliAuthService` + `paperclip login`, TDD.
- `cargo test` fully green; `cargo build --bins` clean.
- No Supabase/OpenAI secret, JWT, or key_hash in any client-facing response.
- Print files changed + passing test count.
