# Phase 4 — Adapters, Secrets, Skills, and Config

## Goal

Make hired agents configurable and safe like original Paperclip agents.

## Source evidence

- `doc/SPEC-implementation.md:145-166` agent config model.
- `doc/DEVELOPING.md:270-286` Codex local isolation.
- `doc/DATABASE.md:173-218` secret storage.
- `server/src/routes/agents.ts:2194-2371` hire config normalization/redaction.
- `server/src/services/secrets.ts`
- `packages/shared/src/validators/agent.ts`

## Target files

- `supabase/migrations/paperclip/0021_secrets_skills_config_revisions.sql`
- `backend-rs/src/adapters/*`
- `backend-rs/src/secrets/*`
- `backend-rs/src/agents/config.rs`
- `backend-rs/tests/adapter_config_parity.rs`
- `backend-rs/tests/secrets_redaction.rs`

## Tasks

1. RED: tests for rejecting unsafe Codex/shared `CODEX_HOME` and host OpenAI key inheritance.
2. Add adapter config/runtime config/default environment fields and config revision tables.
3. Implement adapter registry validation for built-ins and plugin placeholders.
4. Implement OpenAI Responses adapter under runtime contract.
5. Implement secret ref format, redaction on reads/events/approvals, and strict-mode validation.
6. Implement desired skills input and materialization hooks.
7. Implement config revisions and rollback.

## Acceptance

- No secret material appears in API/CLI/MCP/UI outputs.
- Codex local config creates isolated per-agent homes and blocks shared paths.
- Adapter config changes create revisions and rollback works.
