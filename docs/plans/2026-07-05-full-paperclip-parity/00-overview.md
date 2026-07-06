# Full Paperclip Parity Implementation Plan

> **Goal:** eliminate the verified parity gap between the current Rust/Supabase/Nuxt rewrite and original Paperclip V1 for agent hiring, governance, runtime, interfaces, and real work execution.
>
> **Definition of success:** the rewrite reaches **zero known parity gaps** against `doc/SPEC-implementation.md` and the original TypeScript implementation evidence cited here. A working vertical slice is not enough.

## Observable success criteria

A board user can run the original Paperclip V1 loop in the rewrite:

1. Create/sign into a company in authenticated mode.
2. Configure company settings, including `require_board_approval_for_new_agents` with the original default `false`.
3. Hire/create agents with original schema richness: org tree, adapter type/config, runtime config, instructions bundle, desired skills, budgets, capabilities, secret refs, trust/authorization policy.
4. When approval is required, create a full `hire_agent` approval payload, board decision flow, activity log, notifications/hooks, and original status transitions.
5. Approved/runnable agents enter the original lifecycle (`idle -> running -> idle/error`) through heartbeat/wakeup/run records, not a one-off synchronous job only.
6. Board can pause, resume, terminate, clear error, inspect runtime state, reset sessions, view live runs, cancel runs, and inspect logs/events.
7. Agents can use scoped API keys to read/claim/execute assigned work without crossing company or trust boundaries.
8. Costs, budgets, hard-stop auto-pause, and approval-required budget overrides behave like original V1.
9. UI, CLI, MCP, and HTTP APIs expose equivalent user/operator capabilities.
10. Acceptance harness proves real users, real companies, real hired agents, real OpenAI Responses jobs, and parity with no known missing/divergent items.

## Current baseline

The rewrite currently proves a valuable vertical slice:

- Supabase custom-schema auth foundation.
- Company/job RPCs with session and API-key paths.
- OpenAI Responses job execution.
- Hiring with `pending_approval`, board decision, agent-key minting, and agent-attributed real job.
- Minimal Nuxt/CLI/MCP surfaces for that slice.

That is not full parity. The corrected audit baseline is in [`01-parity-audit-summary.md`](01-parity-audit-summary.md).

## Out-of-plan non-goals

This plan does not redefine V1. If a capability in original V1 is too expensive or no longer desired, first update `doc/SPEC-implementation.md` and record a product decision. Until then, this plan treats original V1 behavior as required parity.
