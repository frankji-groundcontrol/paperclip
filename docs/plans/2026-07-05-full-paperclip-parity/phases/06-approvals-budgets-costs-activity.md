# Phase 3 — Approvals, Budgets, Costs, and Activity

## Goal

Recreate original governance side effects instead of binary hire-only decisions.

## Source evidence

- `packages/shared/src/constants.ts:538-552` approval types/statuses.
- `server/src/routes/approvals.ts`
- `server/src/services/approvals.ts`
- `server/src/routes/board-chat.ts`
- `server/src/services/budgets.ts`
- `server/src/routes/costs.ts`

## Target files

- `supabase/migrations/paperclip/0017_approval_governance.sql`
- `supabase/migrations/paperclip/0018_activity_cost_budget.sql`
- `backend-rs/src/approvals/*`
- `backend-rs/src/budgets/*`
- `backend-rs/src/activity/*`
- `backend-rs/tests/approvals_parity.rs`

## Tasks

1. RED: tests for `revision_requested`, decision notes, generic approval types.
2. Add approval comments/decision metadata/source issue links.
3. Implement create/list/detail/decide/revision/resubmit/cancel.
4. Implement side effects for hire approval: activate to `idle`, budget policy creation, activity events, notifications/hooks placeholder.
5. Implement budget override approvals and hard-stop auto-pause.
6. Add cost events/rollups by company/project/task/agent/run.
7. Add activity log table/API and write events for every mutating action.

## Acceptance

- Every mutating action in phases 1-3 writes an activity event.
- Approval lifecycle supports pending -> revision_requested -> pending -> approved/rejected/cancelled.
- Budget hard-stop pauses agents/tasks per original V1 contract.
