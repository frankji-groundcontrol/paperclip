# Phase 2 — Agent Runtime and Lifecycle

## Goal

Restore original agent lifecycle and runtime semantics: hired agents are not just API-key subjects; they are managed employees with heartbeat runs and board controls.

## Source evidence

- `packages/shared/src/constants.ts:19-27` statuses.
- `server/src/routes/agents.ts:2129-2192` runtime state/session reset.
- `server/src/routes/agents.ts:2948-3419` pause/resume/terminate/clear-error/wakeup.
- `server/src/routes/agents.ts:3453-3589` live runs/run detail/cancel.
- `server/src/services/heartbeat.ts` run transitions.

## Target files

- `supabase/migrations/paperclip/0019_heartbeat_runtime.sql`
- `backend-rs/src/runtime/*`
- `backend-rs/src/agents/lifecycle.rs`
- `backend-rs/tests/agent_lifecycle_parity.rs`
- `backend-rs/tests/runtime_parity.rs`

## Tasks

1. RED: tests for approved hire becoming `idle`, not `active`.
2. RED: pause/resume/terminate/clear-error route tests fail.
3. Add heartbeat run/wakeup/log/event/runtime-session schema and RPCs.
4. Implement lifecycle transition functions with invalid-state errors: terminated cannot resume; pending cannot invoke; paused/terminated/pending excluded from scheduler.
5. Implement wakeup/invoke/cancel/live-run endpoints and service methods.
6. Wire OpenAI Responses execution through runtime run records so status mutates `idle -> running -> idle/error`.
7. Revoke keys and cancel active runs on termination.

## Acceptance

- Runtime acceptance proves a hired agent enters `idle`, runs a real OpenAI Responses run, transitions through `running`, then `idle` with `last_heartbeat_at`.
- Failed run moves agent to `error`; clear-error moves to `idle`.
- Terminated agent cannot run and keys are invalid.
