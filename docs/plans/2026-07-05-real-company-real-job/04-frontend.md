# 04 — Frontend for non-agent users (follow-on)

For users who don't use agents. Nuxt (existing `frontend-nuxt`), hitting the **same broker endpoints**
with the opaque `pcs_` session (never Supabase/OpenAI directly).

## Minimal pages
- `/login` — email+password → `POST /api/auth/login` → store `pcs_` session (httpOnly cookie or memory).
- `/companies` — `GET /api/companies`; "New company" → `POST /api/companies`.
- `/companies/:id` — job console: a prompt box → `POST /api/companies/:id/jobs`, shows the streamed/
  final result + token usage; `GET …/jobs` history.

## Notes
- Session path uses the user JWT server-side (RLS as `auth.uid()`); the browser holds only `pcs_`.
- This is **P3** — not the acceptance gate for "real user → real company → real job" (that runs through
  the CLI/MCP agent path in `05`). Scaffold + wire after P1/P2 are proven.
- TDD: component tests for the login/company/job views against a mock of the broker endpoints.
