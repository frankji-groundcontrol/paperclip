# Risk Register

| Risk | Severity | Mitigation |
|---|---|---|
| Accidentally preserving current non-parity behavior in tests | Critical | Phase 0 parity oracle must fail on current rewrite before fixes. |
| Supabase enum changes on applied project | High | Use additive migrations and safe cast/rename strategy; test on branch/project before production. |
| Service-role temptation for hard authz cases | Critical | Explicitly banned; use SECURITY DEFINER helpers and RLS. |
| Client leaks Supabase/OpenAI details | Critical | Contract tests scan responses/UI for project refs, JWTs, OpenAI keys/base URL. |
| Runtime semantics become fake synchronous jobs | Critical | Phase 5 requires heartbeat run tables and status transitions; OpenAI job is an adapter execution, not replacement lifecycle. |
| Secret refs leak through config/approval payload | Critical | Redaction tests for every config read, activity event, approval payload, CLI/MCP output. |
| Over-broad API keys create companies/cross company | High | Key-principal functions must bind subject/company scopes and deny unsupported actions. |
| UI grows before backend contracts stabilize | Medium | UI phase depends on backend phase acceptance. |
| Codex rate limits block execution | Medium | Codex review can retry; plan records manual fallback, but final sign-off requires an independent review. |
