# Original Source Map

## Product/spec

- `doc/GOAL.md` — control plane vision.
- `doc/PRODUCT.md` — companies, employees/agents, adapter config, tasks, board abstraction.
- `doc/SPEC-implementation.md` — V1 contract.

## Constants/contracts

- `packages/shared/src/constants.ts:19-27` — `AGENT_STATUSES`.
- `packages/shared/src/constants.ts:538-552` — approval types/statuses.
- `packages/shared/src/constants.ts:786-798` — permission keys.
- `packages/shared/src/validators/agent.ts` — agent payload/config validators.

## Schema/migrations

- `packages/db/src/migrations/0071_default_hire_approval_off.sql` — hire approval default false.
- `packages/db/src/schema/*` — Drizzle schema source.

## Server

- `server/src/routes/authz.ts` — board/company guards.
- `server/src/routes/access.ts` — members, invites, permissions, joins.
- `server/src/routes/agents.ts` — hire/create/config/lifecycle/runtime routes.
- `server/src/routes/approvals.ts` — approval APIs.
- `server/src/services/approvals.ts` — approval side effects.
- `server/src/services/agents.ts` — lifecycle persistence/key revocation.
- `server/src/services/heartbeat.ts` — runtime execution/status transitions.
- `server/src/services/secrets.ts` — secret handling.
- `server/src/services/company-member-roles.ts` — role grants.

## UI

- `ui/src/pages/NewAgent.tsx`
- `ui/src/pages/Agents.tsx`
- `ui/src/pages/AgentDetail.tsx`
- `ui/src/pages/ApprovalDetail.tsx`
- `ui/src/pages/CompanySettings.tsx`
- `ui/src/pages/BoardChat.tsx`
- `ui/src/components/AgentActionButtons.tsx`
- `ui/src/components/ApprovalPayload.tsx`

## CLI

- `cli/src/commands/client/agent.ts`
- `cli/src/commands/client/approval.ts`
