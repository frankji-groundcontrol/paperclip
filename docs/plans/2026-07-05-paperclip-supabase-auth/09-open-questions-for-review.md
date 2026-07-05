# 09 — Open Questions to Lock with plan-eng-review

Decisions I lean toward (with rationale); review to confirm or override.

1. **API-key request execution without a service role.** After `resolve_api_key` yields a principal, how do api-key (MCP/CLI) requests read/write data under RLS without a user JWT (which we can't mint — no JWT secret)?
   - **Lean: all api-key-authenticated data access goes through `SECURITY DEFINER` RPCs** that accept the resolved `(user_id/agent_id, team_id, scopes)` explicitly and self-authorize (re-check membership/scope). No raw RLS reads for api-key sessions. Keeps us entirely off the service key.
   - Alt: a dedicated low-priv Postgres role + `set_config('request.jwt.claims',…)` — rejected (fragile, edges toward impersonation).

2. **Instance admin representation.** `users.system_role in (user,instance_admin)` vs a `paperclip.instance_admins` table. Lean: keep `system_role` column (simpler); add table only if we need per-scope admin grants. First-user auto-promote? Lean: **no auto-promote** (explicit grant via a seeded admin), to avoid a signup race granting instance admin.

3. **Teams vs Companies naming.** User asked for "teams"; original uses "companies". Lean: **teams** = the access-control root now; companies/agents/projects layer on top later as sub-scopes referencing `team_id`. Keeps the foundation clean and matches the request. Confirm we won't need a company layer *inside* this milestone.

4. **`join_requests` scope.** Included for parity but the primary path is direct invitations. Lean: **implement create+approve+reject** (parity) but keep it minimal. Confirm it's in-scope for this milestone.

5. **Email confirmation for register test.** If the project requires email confirmation, `testuser3` can't password-login without confirming, and we won't use the service role. Lean: verify the project's setting; assert bootstrap regardless; document login state honestly. Confirm acceptable.

6. **Server language for the broker.** `backend-rs` (axum) vs the Nuxt server. Lean: **backend-rs** (aligns with the recode; strong typing for the principal/crypto). Confirm.

7. **Where crypto lives.** Server-side minting (Rust) vs DB-side (`gen_random_bytes`). Lean: **server mints api keys + invite codes** (plaintext never enters the DB); DB generates only the non-secret parts. CLI generates its own device-login key. Confirm.

8. **Hardening (optional, note only):** HMAC-with-pepper instead of bare `sha256` for api-key hashing (defense if a key were low-entropy — not the case here, keys are 256-bit). Lean: **plain sha256 is sufficient** for 256-bit random keys; skip pepper to match house style. Flag if reviewer wants HMAC.

9. **Codex scope.** Codex writes: migration SQL (`0001–0005`) + pgTAP/SQL tests + `backend-rs/src/auth/*` + Rust tests, TDD. I apply migrations to the real project and run the real-user acceptance matrix (`07`). Confirm the split.
