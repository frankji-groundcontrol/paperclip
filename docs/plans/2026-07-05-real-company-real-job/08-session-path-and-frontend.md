# 08 — Session path + frontend for non-agent users (P3, now in scope)

The goal names a **frontend for users who don't use agents**. They log in with email/password and get
an opaque `pcs_` session — so the `/api/paperclip/*` endpoints must also work on the **session path**
(P1/P2 built only the api-key path). This doc adds that path and the Nuxt UI.

## Backend: session path (enables the frontend)
- `AuthBroker::session_access_token(bearer) -> anyhow::Result<Option<String>>`: for a `pcs_` token,
  refresh if near expiry (reusing the existing logic), return the current GoTrue access token; `None`
  for a non-session bearer; evict + `Err` on refresh failure. The JWT stays server-side.
- `JobService` gains an `AuthBroker` handle and **dispatches on the bearer**:
  - `pcs_…` (session): get the JWT via `session_access_token`; read `default_team_id` from `whoami`
    (via `DataGateway`); call the **session RPCs** with `Auth::Bearer(jwt)`:
    `create_company(p_team_id, p_name)`, `create_job(...)`, `complete_job(...)`/`fail_job(...)`; list
    via the `my_companies` / `my_jobs` views (add `DataGateway::get(path, auth)`).
  - `paperclip_…` (api key): the existing key-credential path, unchanged.
- Endpoints under `/api/paperclip/*` are unchanged; they already pass the raw bearer to `JobService`.
- **TDD:** unit tests with a fake broker/session + spy `DataGateway` assert the session path calls
  `whoami` then `create_company`/`create_job`/`complete_job` with `Auth::Bearer(jwt)` and the right
  params; api-key path still uses `*_with_key`. Live acceptance = **E11** below.

## Frontend (Nuxt, `frontend-nuxt`)
New pages/composables (separate from the legacy in-memory components), calling the broker via the
same-origin `/api` proxy; the browser holds only the opaque `pcs_` token (a cookie).
- `useSession()` composable: `login(email,password)` → `POST /api/auth/login` → store `pcs_`;
  `logout()`; `whoami()` from the login payload.
- `usePaperclip()` composable: `createCompany(name)`, `listCompanies()`, `runJob(companyId, prompt,
  model?)`, `listJobs(companyId)` → the `/api/paperclip/*` endpoints with `Authorization: Bearer pcs_`.
- Pages: `/login` (form), `/paperclip/companies` (list + create), `/paperclip/companies/[id]`
  (job console: prompt box → run → shows result + token usage; job history).
- **Tests (vitest + @vue/test-utils + happy-dom):** component tests render the login and job-console
  views against a mocked `$fetch`, asserting the right endpoints/payloads and that a returned job
  result renders. (No Supabase/OpenAI detail ever reaches the browser — only `pcs_` + job results.)

## Acceptance (real, added to `05`)
- **E11 (session path, backend):** real GoTrue login of testuser1 via the broker → `pcs_` → `POST
  /api/paperclip/companies` (session) creates a real company on the user's default team → `POST
  …/jobs` runs a **real OpenAI job** → `succeeded` + real result; `GET …/companies` & `…/jobs` list
  them; a second user cannot see them (RLS). Driven by `run_session_acceptance.py`.
- **Frontend build/tests:** `nuxt build` succeeds; vitest component tests pass.
