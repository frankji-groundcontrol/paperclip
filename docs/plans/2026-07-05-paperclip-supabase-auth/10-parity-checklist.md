# 10 — Original-Paperclip Parity Checklist

Grounded in the original TS code (file anchors below). "→" = how the Supabase design preserves it.

## Roles / enums (source: `packages/shared/src/constants.ts`)
- `COMPANY_MEMBERSHIP_ROLES = [owner, admin, operator, viewer, member]` → `paperclip.team_role` enum (same 5 values).
- `HUMAN_COMPANY_MEMBERSHIP_ROLES = [owner, admin, operator, viewer]` (member reserved for agent/system principals) → enforced in invite/member RPCs.
- `INVITE_JOIN_TYPES = [human, agent, both]` → `paperclip.invitations.allowed_join_types`.
- `JOIN_REQUEST_TYPES = [human, agent]`, `JOIN_REQUEST_STATUSES = [pending_approval, approved, rejected]` → `paperclip.join_requests`.
- `instance_user_roles.role default 'instance_admin'` → `paperclip.users.system_role in (user, instance_admin)` + `paperclip.instance_admins` (or a flag).

## API keys (source: `packages/db/src/schema/board_api_keys.ts`, `agent_api_keys.ts`)
- **board_api_keys**: `id, user_id→auth.users, name, key_hash, last_used_at, revoked_at, expires_at, created_at` (user-scoped).
- **agent_api_keys**: `id, agent_id, company_id, name, key_hash, scope_config jsonb, last_used_at, revoked_at, created_at` (agent+company-scoped, with scope config).
- → Unified `paperclip.api_keys` with `subject_type in (user, agent)`, `agent_id` nullable, `team_id`, `scope_config jsonb`, `expires_at`. **Improvement**: add `prefix` for O(1) lookup (original scans by hash only).
- Both store **only `key_hash`** (sha256). → same; plaintext minted server-side, revealed once.

## CLI auth (source: `packages/db/src/schema/cli_auth_challenges.ts`, `server/src/routes/access.ts` cli-auth)
- Columns: `id, secret_hash, command, client_name, requested_access default 'board', requested_company_id, pending_key_hash, pending_key_name, approved_by_user_id, board_api_key_id, approved_at, cancelled_at, expires_at`.
- Flow: **CLI generates the pending key locally**, sends `pending_key_hash` + `secret_hash`; browser-authenticated user approves → server materializes the api key from `pending_key_hash`; CLI polls with challenge secret and already holds the plaintext. **No plaintext ever at rest.**
- → `paperclip.cli_auth` with the same shape (`team_id` replaces `requested_company_id`). This is the **secure device-login** pattern (better than opconfig's `api_key_full`). Adopt as-is.

## Identity / memberships (source: `packages/db/src/schema/{companies,company_memberships,agent_memberships,project_memberships,instance_user_roles}.ts`)
- A **user** = `auth.users` row (Supabase). Original mirrors identity in board flows.
- **company** = tenant. **company_memberships**(company_id, user_id/principal, membership_role, status). agent/project memberships nest under it.
- → In this foundation the **team** is the access-control group (== the original "company membership" layer, renamed per the user's explicit "teams"). `paperclip.teams` + `paperclip.team_members(role)`. Companies/agents/projects layer on top later; team is the root grant.
- **default team per user**: original bootstraps a company/membership for a new user → `handle_new_auth_user` trigger creates a personal team + owner membership.

## Permissions (source: `server/src/services/{access,authorization,company-member-roles}.ts`)
- Action strings observed: `joins:approve, agents:create, pipelines:write, secrets:read, skills:create, tasks:assign, environments:manage, runtime:manage, agent_config:read/update, issue:read/comment/mutate, project:read, company_scope:read, agent:read/wake, upstream_import:{preview,read,write}, dev:once`.
- `access.decide/canUser/hasPermission` with `policySource = oss_default | policy_hook` — OSS default is **allow** for members of the scope; enterprise policy hook can deny.
- → Foundation subset needed now (team management): `team:manage_members, team:manage_keys, team:manage_invites, joins:approve, team:delete, team:read`. Mapped to roles in `03-rpcs.md`. Domain permissions (issues/agents/pipelines) layer on the team grant later. Preserve the **oss-default-allow-for-members** boundary via RLS + role checks.

## Invites & join-requests & registration (source: `packages/db/src/schema/invites.ts`, `server/src/routes/access.ts`)
- **invites**: `company_id, invite_type default 'company_join', token_hash, allowed_join_types default 'both', defaults_payload jsonb, expires_at, invited_by_user_id, revoked_at, accepted_at`. Token hashed; accept materializes a membership (+ optionally an agent).
- **join_requests**: create (pending_approval) → approve/reject; approve materializes membership/agent.
- **registration**: user signs up (auth.users) → bootstrap default team. Instance admin established via `instance_user_roles` (first user / explicit grant).
- → `paperclip.invitations` (team-scoped, **+ role**), `accept_invitation(code)` inserts `team_members`; `paperclip.join_requests` + approve/reject RPCs; register via GoTrue + bootstrap trigger.

## Bootstrap / instance admin
- First user / `instance_user_roles` = instance admin; `bootstrapStatus` gates first-run.
- → `paperclip.users.system_role`, seeded/first-user promotion RPC; server exposes `bootstrapStatus`.

---
## PARITY CHECKLIST (flat — every capability the reimplementation must preserve)
- [ ] User identity backed by `auth.users`; app profile row per user.
- [ ] Team (access group) with 5-role model `owner/admin/operator/viewer/member`.
- [ ] Personal/default team auto-created on user signup (owner membership).
- [ ] Team membership CRUD with role, gated by role (admin+ manages members).
- [ ] User-scoped API keys (parity: board_api_keys): create/list/revoke, hashed, expiry, last_used.
- [ ] Agent-scoped API keys (parity: agent_api_keys): `subject_type=agent`, `agent_id`, `scope_config`, team-scoped.
- [ ] API-key authentication: resolve presented key (by hash) → principal context (user/agent, team, scopes).
- [ ] CLI/MCP device-login (parity: cli_auth_challenges): CLI-generated key, hash-only at rest, browser approval, poll.
- [ ] Invitations: team+role scoped, hashed code, `allowed_join_types`, `defaults_payload`, expiry, revoke, accept→membership.
- [ ] Join-requests: create (pending) → approve/reject → membership materialization; `joins:approve` gate.
- [ ] Instance-admin concept + bootstrapStatus.
- [ ] Permission decisions with OSS-default-allow-for-members boundary (RLS + role checks).
- [ ] Everything hashed at rest (keys, invite codes, challenge secrets); plaintext revealed once.
- [ ] Company scoping ⇒ team scoping: a principal only sees resources for teams it belongs to (RLS).
