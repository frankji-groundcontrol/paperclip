# 05 — API Keys + CLI/MCP Device-Login

## Key format
```
full_key = "paperclip_" + hex(4 rand bytes)        <- prefix (stored)
         + "_" + base64url(32 rand bytes, no '=')   <- body (never stored)
key_hash = sha256_hex(full_key)                     <- stored
```
- **Server mints** `full_key`, computes `prefix`+`key_hash`, calls `create_api_key(...)`. Returns `full_key` to the user **exactly once**. DB stores only `prefix`+`key_hash`.
- Presented key auth: server splits `prefix`, computes `sha256(full_key)`, calls `resolve_api_key(prefix, hash)`.

## Agent vs user keys (parity board_api_keys / agent_api_keys)
- `subject_type='user'` → board-key equivalent (human/session-substitute).
- `subject_type='agent'` + `agent_id` + `scope_config` → agent-key equivalent (MCP agents). `scopes`/`scope_config` carried into the `Principal`.

## CLI/MCP device-login sequence (no plaintext at rest anywhere)
```
CLI                          Server                        Supabase(RPC via anon)
 |  generate locally:          |                              |
 |   secret, user_code,        |                              |
 |   pending_key(full),        |                              |
 |   pending_key_prefix        |                              |
 |-- POST /cli-auth/start ---->| hash all -> cli_start_device_login(hashes) -->|
 |<-- {user_code, verify_url, poll_token=secret} --|          | (row stored, hashes only)
 |  print user_code + url      |                              |
 |                             |   (user opens url, logs in)  |
 |                             |<-- POST /cli-auth/approve {user_code} (session)|
 |                             | hash user_code -> cli_approve_device_login --->| (insert api_keys w/ pending_key_hash)
 |-- GET /cli-auth/poll ------>| hash secret -> cli_poll_device_login -------->|
 |<-- {status:approved} -------|                              |
 |  CLI already HAS pending_key(full) -> use it as its API key |
```
- The minted key's **plaintext exists only on the CLI**, which generated it. Server + DB only ever hold hashes. This beats opconfig's `api_key_full` plaintext-at-rest.
- Poll returns `status` (+ prefix for confirmation), never a secret.
- `user_code` is short/human (e.g. `ABCD-1234`); shown by CLI, typed in browser. `secret` (poll token) is high-entropy, never shown.
