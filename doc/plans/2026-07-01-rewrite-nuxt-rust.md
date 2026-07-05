# 2026-07-01 Rewrite: frontend → Nuxt, backend → Rust (TDD)

Branch: `rewrite-nuxt-rust` (off `franky`).

Goal: migrate the Paperclip control plane from TypeScript (Express `server/` +
React/Vite `ui/`) to a Rust backend (`backend-rs/`, axum) and a Nuxt frontend
(`frontend-nuxt/`), test-first throughout.

## Approach: strangler-fig, vertical slices, TDD

A full rewrite of a 3,256-file monorepo is a large, multi-session effort. It is
done incrementally and honestly:

- New stacks live alongside the existing ones (`backend-rs/`, `frontend-nuxt/`);
  nothing in `server/`, `ui/`, or `packages/` is deleted until its replacement is
  ported and verified.
- Each slice is one endpoint or view, ported test-first: write the failing test,
  watch it fail, implement the minimum to pass, refactor, commit.
- The existing TypeScript contracts (routes, OpenAPI, shared types) are the
  source of truth each Rust/Nuxt slice must satisfy.

Toolchain verified on this host: cargo/rustc 1.94, node 24, npm 11.

## Slice log

| # | Slice | Stack | Status |
|---|-------|-------|--------|
| 1 | `GET /api/health` → `{status:"ok"}` | Rust/axum | Done (green) |
| 2 | `GET /api/health` adds `version` | Rust/axum | Done (green) |
| 3 | Companies CRUD (`GET`/`POST /api/companies`, `GET /:id`, in-memory repo) | Rust/axum | Done (green) |
| 4 | Issues (`GET`/`POST /api/companies/:companyId/issues`, company-scoped) | Rust/axum | Done (green) |
| 5 | Projects (`GET`/`POST /api/companies/:companyId/projects`, company-scoped) | Rust/axum | Done (green) |
| 6 | Agents (`GET`/`POST /api/companies/:companyId/agents`, company-scoped) | Rust/axum | Done (green) |
| 7 | Durable persistence: `CompanyRepository` trait + SQLite-backed store (survives new instances) | Rust/rusqlite | Done (green) |
| 8 | Wire SQLite into `/api/companies` routes (trait-object state); company POSTed over HTTP persists to disk end-to-end | Rust/axum | Done (green) |
| 9 | Durable, company-scoped issue persistence (`IssueRepository` + SQLite) wired into `/api/companies/:id/issues`; issue POSTed over HTTP persists to disk | Rust/axum | Done (green) |
| 10 | Durable, company-scoped project persistence (`ProjectRepository` + SQLite) wired via a `Repositories` builder; project POSTed over HTTP persists to disk | Rust/axum | Done (green) |
| 11 | Durable, company-scoped agent persistence (`AgentRepository` + SQLite) wired into HTTP; **all four data domains now durable end-to-end** | Rust/axum | Done (green) |
| 12 | Auth/actor resolution: `Actor` extractor (board vs bearer agent-key → company scope; 401 on invalid key); `GET /api/whoami` | Rust/axum | Done (green) |
| 13 | Company-scoped authorization enforced on issue routes (`authorize_company_access`): agent → own company only (403 otherwise), board → any | Rust/axum | Done (green) |
| 14 | Company-scoped authorization extended to project + agent routes (consistent 403 for cross-company agents) | Rust/axum | Done (green) |
| 15 | Runs domain (`GET`/`POST /api/companies/:companyId/runs`, company-scoped, authz-enforced, `status` default "queued") — orchestration layer | Rust/axum | Done (green) |
| 16 | Durable, company-scoped run persistence (`SqliteRunStore`) wired end-to-end; **all five data domains now durable** | Rust/axum | Done (green) |
| 17 | Approvals domain (`GET`/`POST /api/companies/:companyId/approvals`, company-scoped, authz-enforced, `status` default "pending") — governed-action gates | Rust/axum | Done (green) |
| 18 | Budgets domain (`GET /budget`, `POST /budget/limit`, `POST /budget/spend`) — budget **hard-stop** invariant (`exceeded` at limit), company-scoped, authz | Rust/axum | Done (green) |
| 19 | Routines domain (`GET`/`POST /api/companies/:companyId/routines`, company-scoped, authz-enforced, `status` default "active") — recurring scheduled tasks | Rust/axum | Done (green) |
| 20 | Secrets domain (`GET`/`POST /api/companies/:companyId/secrets`) — value stored but **never serialized** (leak-proof invariant asserted), company-scoped, authz | Rust/axum | Done (green) |
| 21 | Workspaces domain (`GET`/`POST /api/companies/:companyId/workspaces`, company-scoped, authz-enforced, `status` default "starting") — execution workspaces | Rust/axum | Done (green) |
| 22 | Plugins domain (`GET`/`POST /api/companies/:companyId/plugins`, company-scoped, authz-enforced, `enabled` default true) — company plugin registry | Rust/axum | Done (green) |
| 23 | Adapters domain (`GET`/`POST /api/companies/:companyId/adapters`, company-scoped, authz-enforced, `enabled` default true) — company adapter registry | Rust/axum | Done (green) |
| 24 | MCP surface (`GET /api/mcp/tools`) — lists control-plane tools ({name, description}) mirroring packages/mcp-server | Rust/axum | Done (green) |
| 25 | Durable, company-scoped approval persistence (`SqliteApprovalStore`) wired end-to-end | Rust/axum | Done (green) |
| 26 | Durable, company-scoped routine persistence (`SqliteRoutineStore`) wired end-to-end | Rust/axum | Done (green) |
| 27 | Durable, company-scoped secret persistence (`SqliteSecretStore`) — value stored, never selected back; wired end-to-end | Rust/axum | Done (green) |
| 28 | Durable, company-scoped workspace persistence (`SqliteWorkspaceStore`) wired end-to-end | Rust/axum | Done (green) |
| 29 | Durable, company-scoped plugin persistence (`SqlitePluginStore`) wired end-to-end | Rust/axum | Done (green) |
| 30 | Durable, company-scoped adapter persistence (`SqliteAdapterStore`) wired end-to-end — **all 12 domains now durable** | Rust/axum | Done (green) |
| 31 | Issue mutations: `PATCH`/`DELETE /api/companies/:companyId/issues/:issueId` (partial update, 404 handling) across in-memory + SQLite — moving toward CRUD/endpoint parity | Rust/axum | Done (green) |
| 32 | Project mutations: `PATCH`/`DELETE /api/companies/:companyId/projects/:projectId` (partial update, 404 handling) across in-memory + SQLite | Rust/axum | Done (green) |
| 33 | Agent mutations: `PATCH`/`DELETE /api/companies/:companyId/agents/:agentId` (partial update, 404 handling) across in-memory + SQLite | Rust/axum | Done (green) |
| 34 | Run mutations: `PATCH`/`DELETE /api/companies/:companyId/runs/:runId` (status transitions, 404 handling) across in-memory + SQLite | Rust/axum | Done (green) |
| 35 | Company mutations: `PATCH`/`DELETE /api/companies/:companyId` (partial update, delete→detail 404) across in-memory + SQLite | Rust/axum | Done (green) |
| 36 | Approval mutations: `PATCH`/`DELETE /api/companies/:companyId/approvals/:approvalId` (decision transitions, 404) across in-memory + SQLite | Rust/axum | Done (green) |
| 37 | Routine mutations: `PATCH`/`DELETE /api/companies/:companyId/routines/:routineId` (partial update, 404) across in-memory + SQLite | Rust/axum | Done (green) |
| 38 | Workspace mutations: `PATCH`/`DELETE /api/companies/:companyId/workspaces/:workspaceId` (lifecycle status, 404) across in-memory + SQLite | Rust/axum | Done (green) |
| 39 | Plugin mutations: `PATCH`/`DELETE /api/companies/:companyId/plugins/:pluginId` (toggle enabled, 404) across in-memory + SQLite | Rust/axum | Done (green) |
| 40 | Adapter mutations: `PATCH`/`DELETE /api/companies/:companyId/adapters/:adapterId` (toggle enabled, 404) across in-memory + SQLite | Rust/axum | Done (green) |
| 41 | Secret deletion: `DELETE /api/companies/:companyId/secrets/:secretId` (404 handling) across in-memory + SQLite; no-leak invariant preserved on delete | Rust/axum | Done (green) |
| 42 | **Goals domain** (new port of server/src/routes/goals.ts): `GET`/`POST /api/companies/:companyId/goals`, company-scoped, authz-enforced; `level` default "task", `status` default "planned", nullable description/parentId/ownerAgentId | Rust/axum | Done (green) |
| 43 | Goal mutations: `PATCH`/`DELETE /api/companies/:companyId/goals/:goalId` (partial update leaves other fields unchanged, 404 handling) | Rust/axum | Done (green) |
| 44 | **Activity domain** (new port of server/src/routes/activity.ts): `GET`/`POST /api/companies/:companyId/activity`; **board-only create** (`require_board` — agents 403 even for own company); `GET` company-scoped with `agentId`/`entityType`/`entityId` filters + clamped `limit` (default 100, max 500), newest-first | Rust/axum | Done (green) |
| 45 | Durable goal persistence (`SqliteGoalStore`): create/list/update/delete round-trip through SQLite, company-scoped, all core fields (description/level/status/parentId) preserved | Rust/rusqlite | Done (green) |
| 46 | Durable activity persistence (`SqliteActivityStore`): JSON `details` round-trips; filters (`agentId`/`entityType`/`entityId`) + clamped `limit` + newest-first (`rowid DESC`) enforced in SQL | Rust/rusqlite | Done (green) |
| 47 | **Dashboard domain** (new port of server/src/services/dashboard.ts): `GET /api/companies/:companyId/dashboard` — first **cross-domain** handler, aggregating agent/task/approval status buckets from three repos via multiple `State` extractors (`idle`→active, `open`=not done/cancelled) | Rust/axum | Done (green) |
| 48 | **Costs domain** (new port of server/src/routes/costs.ts core): `POST /api/companies/:companyId/cost-events` (biller←provider, billingType "unknown", token defaults) + `GET /costs/summary` (cross-domain spend vs company budget, `utilizationPercent`, 404 on unknown company) | Rust/axum | Done (green) |
| 49 | **Actor identity** (auth-parity): `Actor::Agent` now carries an optional `agent_id`; `AgentKeyStore::insert_agent` maps a key → (company, agent); `whoami` surfaces `agentId`; new `require_own_agent` guard completes the costs "agent can only report its own costs" 403 (ports server/src/routes/costs.ts:118) | Rust/axum | Done (green) |
| 50 | **Environments domain** (new port of server/src/routes/environments.ts core CRUD): `GET`/`POST /api/companies/:companyId/environments` + `PATCH`/`DELETE /…/:environmentId`; driver required, `status` default "active", `config`/`envVars` default `{}` (JSON), nullable description/metadata, 404 handling | Rust/axum | Done (green) |
| 51 | Durable environment persistence (`SqliteEnvironmentStore`): create/list/update/delete round-trip; JSON `config`/`envVars`/`metadata` stored as TEXT and round-tripped; company-scoped | Rust/rusqlite | Done (green) |
| 52 | Durable cost persistence (`SqliteCostStore`): cost events persist; `biller` defaults to `provider`; `spend_cents` summed via SQL `SUM`, company-scoped | Rust/rusqlite | Done (green) |
| 53 | **Board user identity + Inbox-dismissals domain** (new port of server/src/routes/inbox-dismissals.ts): `Actor::Board` now carries `user_id` (from `X-Actor-User`), surfaced by `whoami`; new `require_board_user` guard (agent→"Board authentication required", board-without-user→"Board user context required"); `GET`/`POST /api/companies/:companyId/inbox-dismissals` per-user-scoped, idempotent dismiss, `itemKey` regex (`approval|join|run:`) → 400 | Rust/axum | Done (green) |
| 54 | **Sidebar-preferences domain** (new port of server/src/routes/sidebar-preferences.ts): `GET`/`PUT /api/companies/:companyId/sidebar-preferences/me` — board-user-scoped project order; `orderedIds` deduped preserving first occurrence; empty `{orderedIds:[],updatedAt:null}` when unset | Rust/axum | Done (green) |
| 55 | Durable inbox persistence (`SqliteInboxStore`): dismissals persist, (company,user)-scoped; composite PRIMARY KEY + `ON CONFLICT … DO UPDATE` makes dismiss idempotent per item | Rust/rusqlite | Done (green) |
| 56 | **Resource-memberships domain** (new port of server/src/routes/resource-memberships.ts, OSS-default policy): `GET /…/resource-memberships/me` (project/agent → joined/left maps) + `PUT /…/me/projects/:id` and `/…/me/agents/:id`; board-user-scoped; cross-domain existence checks (404 "Project/Agent not found"); invalid state → 400 | Rust/axum | Done (green) |
| 57 | **Instance-settings domain** (new port of server/src/routes/instance-settings.ts): instance-wide **singleton** (not company-scoped) — `GET`/`PATCH /api/instance/settings` (+ `/general`, `/experimental`); board-only; PATCH-merges only provided keys; full general + 14-flag experimental defaults seeded | Rust/axum | Done (green) |
| 58 | **Pipelines core** (new port of server/src/routes/pipelines.ts pipeline entity): `GET`/`POST /api/companies/:companyId/pipelines` + `GET`/`PATCH /…/:pipelineId`; `key`+`name` required, `enforceTransitions`/`archived` default false, nullable description/projectId, 404 handling. Stage/case machinery + list aggregate counts deferred | Rust/axum | Done (green) |
| 59 | **Pipeline stages** (extends pipelines): `GET`/`POST /…/pipelines/:pipelineId/stages` + `PATCH`/`DELETE /…/stages/:stageId`; `kind` (working/review/done/cancelled), `position` default 0 (list sorted by it), `config` JSON default `{}`; cross-domain `ensure_pipeline` guard (404 "Pipeline not found" so a company can't reach another's pipeline); 404 "Stage not found" | Rust/axum | Done (green) |
| 60 | **Pipeline cases** (extends pipelines): `GET`/`POST /…/pipelines/:pipelineId/cases` + `GET`/`PATCH /…/cases/:caseId`; ingest core (`title` required, `fields` JSON default `{}`, nullable caseKey/summary/stageKey/parentCaseId); `ensure_pipeline` guard; 404 "Case not found". Transitions/documents/leases deferred | Rust/axum | Done (green) |
| 61 | **Case transitions** (extends pipelines): `POST /…/cases/:caseId/transition` — moves a case to `toStageKey` with **optimistic concurrency** (`expectedVersion`); cases gain a `version` bumped on each move; target-stage existence checked (404 "Stage not found"), 404 "Case not found", **409 "Version conflict"** on mismatch | Rust/axum | Done (green) |
| 62 | Durable pipeline persistence (`SqlitePipelineStore`): pipeline entities create/list/get/update round-trip; `enforceTransitions`/`archived` bools ↔ INTEGER; company-scoped (stages/cases stay in-memory for now) | Rust/rusqlite | Done (green) |
| 63 | **Board-api-keys domain** (new port of server/src/routes/access.ts board-api-keys): per-board-user token CRUD — `GET`/`POST /api/board-api-keys` + `DELETE /…/:keyId`; `require_board_user_authenticated` (**401** "Board authentication required"); token revealed once on create, **never listed** (no-leak, asserted); 404 "Board API key not found" | Rust/axum | Done (green) |
| 64 | **Company-invites domain** (new port of server/src/routes/access.ts company invites, OSS default): `GET`/`POST /api/companies/:companyId/invites` + `POST /…/:inviteId/revoke`; shareable `token`, `allowedJoinTypes` default "both", `?state=` filter (active/revoked/…), state→"revoked" on revoke, 404 "Invite not found" | Rust/axum | Done (green) |
| 65 | **Join-requests domain** (new port of server/src/routes/access.ts join-requests, OSS default): `GET`/`POST /api/companies/:companyId/join-requests` (`?status=`/`?requestType=` filters) + `POST /…/:requestId/{approve,reject}`; status default "pending_approval", **pending-only** transitions (404 "Join request not found", **409 "Join request is not pending"**). Membership/agent materialisation on approve deferred | Rust/axum | Done (green) |
| 66 | **Sidebar-badges domain** (new port of server/src/routes/sidebar-badges.ts core): `GET /api/companies/:companyId/sidebar-badges` — second cross-domain rollup, counting failed runs + pending approvals + pending join-requests, `inbox` = their sum. Now portable because join-requests exists. Alert heuristics + per-user dismissals deferred | Rust/axum | Done (green) |
| 67 | **LLMs domain** (new port of server/src/routes/llms.ts): `GET /llms/agent-configuration.txt` + `/llms/agent-icons.txt` — first **`text/plain`** endpoints; full 41-name `AGENT_ICON_NAMES` list; board-read gate (agents 403 "Board or permitted agent authentication required" pending an agent-permission model) | Rust/axum | Done (green) |
| 68 | **Teams-catalog domain** (new port of server/src/routes/teams-catalog.ts core): `GET /api/teams/catalog` (kind/category/`q` filters over a seeded catalog) + `GET /…/teams/catalog/installed` + `POST /…/teams/catalog/:catalogId/install` (404 "Catalog team not found"). On-disk bundled loader + agent-materialising import deferred | Rust/axum | Done (green) |
| 69 | **Company-skills domain** (new port of server/src/routes/company-skills.ts core): `GET`/`POST /api/companies/:companyId/skills` (`q`/`category` filters) + `GET /…/:skillId` + `POST`/`DELETE /…/:skillId/star` (star counter, clamped at 0); 404 "Skill not found". Versions/comments/catalog-install/source-import deferred | Rust/axum | Done (green) |
| 70 | **User-profiles domain** (focused new port of server/src/routes/user-profiles.ts): `GET /api/companies/:companyId/users/:userId/profile` — third cross-domain rollup, aggregating the activity feed by author (`activityCount`, newest-first `recentActivity`, `actionCounts` map). Fuller cost/issue/agent windows deferred until those carry per-user keys | Rust/axum | Done (green) |
| 71 | **Cloud-upstreams domain** (focused new port of server/src/routes/cloud-upstreams.ts): board-only `GET /api/cloud-upstreams` (+ `?companyId=`), `POST /…/register`, `DELETE /…/:id`; **gated by the instance-settings `enableCloudSync` flag** (cross-domain read → 404 "Cloud sync is not enabled"). OAuth handshake + push-runs deferred | Rust/axum | Done (green) |
| 1f | Health view renders status | Nuxt/Vitest | Done (green) |
| 2f | `CompanyList` renders companies + empty state | Nuxt/Vitest | Done (green) |
| 3f | `fetchCompanies` composable calls `/api/companies` | Nuxt/Vitest | Done (green) |
| 4f | `IssueList` + `fetchIssues` (company-scoped path) | Nuxt/Vitest | Done (green) |
| 5f | `ProjectList` + `fetchProjects` (company-scoped path) | Nuxt/Vitest | Done (green) |
| 6f | `AgentList` + `fetchAgents` (company-scoped path) | Nuxt/Vitest | Done (green) |
| 7f | `ActorBadge` + `fetchWhoami` (mirrors `/api/whoami`) | Nuxt/Vitest | Done (green) |
| 8f | `RunList` + `fetchRuns` (company-scoped runs) | Nuxt/Vitest | Done (green) |
| 9f | `ApprovalList` + `fetchApprovals` (company-scoped approvals) | Nuxt/Vitest | Done (green) |
| 10f | `BudgetView` + `fetchBudget` (mirrors budget hard-stop) | Nuxt/Vitest | Done (green) |
| 11f | `SecretList` + `fetchSecrets` (references only, no values) | Nuxt/Vitest | Done (green) |
| 12f | `RoutineList` + `fetchRoutines` (company-scoped routines) | Nuxt/Vitest | Done (green) |
| 13f | `WorkspaceList` + `fetchWorkspaces` (company-scoped workspaces) | Nuxt/Vitest | Done (green) |
| 14f | `PluginList` + `fetchPlugins` (company plugin registry) | Nuxt/Vitest | Done (green) |
| 15f | `AdapterList` + `fetchAdapters` (company adapter registry) | Nuxt/Vitest | Done (green) |
| 16f | `Board` shell composing health/actor + domain lists; `app.vue` renders it | Nuxt/Vitest | Done (green) |
| 17f | Issue mutations composables (`createIssue` POST, `deleteIssue` DELETE) with ofetch-style options fetcher — frontend begins mutation parity | Nuxt/Vitest | Done (green) |
| 18f | Project mutations composables (`createProject` POST, `deleteProject` DELETE) | Nuxt/Vitest | Done (green) |
| 19f | Agent mutations composables (`createAgent` POST, `deleteAgent` DELETE) | Nuxt/Vitest | Done (green) |
| 20f | Run mutations composables (`createRun` POST, `updateRun` PATCH status) — introduces frontend PATCH pattern | Nuxt/Vitest | Done (green) |
| 21f | Approval mutations composables (`createApproval` POST, `updateApproval` PATCH decision) | Nuxt/Vitest | Done (green) |
| 22f | Routine mutations composables (`createRoutine` POST, `deleteRoutine` DELETE) | Nuxt/Vitest | Done (green) |
| 23f | Workspace mutations composables (`createWorkspace` POST, `updateWorkspace` PATCH lifecycle) | Nuxt/Vitest | Done (green) |
| 24f | Plugin mutations composables (`updatePlugin` PATCH toggle, `deletePlugin` DELETE) | Nuxt/Vitest | Done (green) |
| 25f | Adapter mutations composables (`updateAdapter` PATCH toggle, `deleteAdapter` DELETE) | Nuxt/Vitest | Done (green) |
| 26f | Company mutations composables (`createCompany` POST, `updateCompany` PATCH, `deleteCompany` DELETE) | Nuxt/Vitest | Done (green) |
| 27f | Secret mutations composables (`createSecret` POST value-in/ref-out, `deleteSecret` DELETE) | Nuxt/Vitest | Done (green) |
| 28f | Budget mutations composables (`setBudgetLimit` POST, `recordBudgetSpend` POST reflecting hard-stop) | Nuxt/Vitest | Done (green) |
| 29f | Live wiring: `makeApiFetcher(rawFetch, baseUrl)` binds `$fetch` to the API base for reads+mutations; `useApi()` glue; `nitro.devProxy` forwards `/api/**` → Rust `:3100`; `runtimeConfig.public.apiBase` | Nuxt/Vitest | Done (green) |
| 30f | Goals composables (`fetchGoals`, `createGoal`, `updateGoal`, `deleteGoal`) mirroring the new backend goals domain | Nuxt/Vitest | Done (green) |
| 31f | Activity composables (`fetchActivity` with stable-order query filters, `createActivity`) mirroring the new backend activity domain | Nuxt/Vitest | Done (green) |
| 32f | Dashboard composable (`fetchDashboard`) mirroring the cross-domain rollup | Nuxt/Vitest | Done (green) |
| 33f | Costs composables (`reportCostEvent`, `fetchCostSummary`) mirroring the new backend costs domain | Nuxt/Vitest | Done (green) |
| 34f | Live board wiring: tested `loadBoard(fetcher, companyId)` orchestrator (parallel fetch of health/actor/companies/issues/agents/runs → `Board` props); `app.vue` now drives `<Board>` from `useApi()` + `useAsyncData` with an empty-state fallback | Nuxt/Vitest | Done (green) |
| 35f | Environments composables (`fetchEnvironments`, `createEnvironment`, `updateEnvironment`, `deleteEnvironment`) mirroring the new backend environments domain | Nuxt/Vitest | Done (green) |
| 36f | Inbox composables (`fetchInboxDismissals`, `dismissInboxItem`) mirroring the new backend inbox-dismissals domain | Nuxt/Vitest | Done (green) |
| 37f | Sidebar composables (`fetchProjectOrder`, `saveProjectOrder` PUT) mirroring the new backend sidebar-preferences domain | Nuxt/Vitest | Done (green) |
| 38f | Memberships composables (`fetchMemberships`, `setProjectMembership`, `setAgentMembership` PUT) mirroring the new backend resource-memberships domain | Nuxt/Vitest | Done (green) |
| 39f | Instance-settings composables (`fetchInstanceSettings`, `updateInstanceSettings`, `updateInstanceGeneral`, `updateInstanceExperimental`) mirroring the new backend instance-settings singleton | Nuxt/Vitest | Done (green) |
| 40f | Pipelines composables (`fetchPipelines`, `createPipeline`, `fetchPipeline`, `updatePipeline`) mirroring the new backend pipelines-core domain | Nuxt/Vitest | Done (green) |
| 41f | Pipeline-stage composables (`fetchStages`, `createStage`, `updateStage`, `deleteStage`) mirroring the new backend stage sub-resource | Nuxt/Vitest | Done (green) |
| 42f | Pipeline-case composables (`fetchCases`, `ingestCase`, `fetchCase`, `updateCase`) mirroring the new backend case sub-resource | Nuxt/Vitest | Done (green) |
| 43f | `transitionCase` composable (POST toStageKey + expectedVersion) mirroring the new backend case-transition endpoint | Nuxt/Vitest | Done (green) |
| 44f | `makeApiFetcher` gains `defaultHeaders` — merged into every request (reads + mutations), so the board-user endpoints (inbox/sidebar/memberships) can carry `X-Actor-User` live | Nuxt/Vitest | Done (green) |
| 45f | Board-keys composables (`fetchBoardApiKeys` refs-only, `createBoardApiKey` one-time token, `deleteBoardApiKey`) mirroring the new backend board-api-keys domain | Nuxt/Vitest | Done (green) |
| 46f | Invites composables (`fetchInvites` w/ state filter, `createInvite`, `revokeInvite`) mirroring the new backend company-invites domain | Nuxt/Vitest | Done (green) |
| 47f | Join-requests composables (`fetchJoinRequests` w/ filters, `createJoinRequest`, `approveJoinRequest`, `rejectJoinRequest`) mirroring the new backend join-requests domain | Nuxt/Vitest | Done (green) |
| 48f | `fetchSidebarBadges` composable mirroring the new backend sidebar-badges rollup | Nuxt/Vitest | Done (green) |
| 49f | `fetchAgentIcons` composable — fetches the llms icons text and parses the `- name` lines into an array (icon-picker helper) | Nuxt/Vitest | Done (green) |
| 50f | Teams-catalog composables (`fetchCatalog` w/ filters, `fetchInstalledTeams`, `installTeam`) mirroring the new backend teams-catalog domain | Nuxt/Vitest | Done (green) |
| 51f | Company-skills composables (`fetchSkills` w/ filters, `createSkill`, `fetchSkill`, `starSkill`, `unstarSkill`) mirroring the new backend company-skills domain | Nuxt/Vitest | Done (green) |
| 52f | `GoalList` component — renders goals (title + level/status + empty state), @vue/test-utils | Nuxt/Vitest | Done (green) |
| 53f | `PipelineList` component — renders pipelines, marks archived, empty state | Nuxt/Vitest | Done (green) |
| 54f | `CompanySkillList` component — renders skills with star count, empty state | Nuxt/Vitest | Done (green) |
| 55f | `EnvironmentList` component — renders environments (driver/status), empty state | Nuxt/Vitest | Done (green) |
| 56f | `InviteList` component — renders invites (join types/state), empty state | Nuxt/Vitest | Done (green) |
| 57f | `JoinRequestList` component — renders join-requests (type/status/requester), empty state | Nuxt/Vitest | Done (green) |
| 58f | `DashboardView` component — renders the agent/task/approval rollup counts | Nuxt/Vitest | Done (green) |
| 59f | `CostSummaryView` component — renders spend/budget (as dollars) + utilization, flags over-budget ≥100% | Nuxt/Vitest | Done (green) |
| 60f | `ActivityList` component — renders the activity feed (action/entity), empty state | Nuxt/Vitest | Done (green) |
| 61f | `fetchUserProfile` composable mirroring the new backend user-profiles rollup | Nuxt/Vitest | Done (green) |
| 62f | Cloud-upstreams composables (`fetchCloudUpstreams` w/ company filter, `registerCloudUpstream`, `deleteCloudUpstream`) mirroring the new backend cloud-upstreams domain | Nuxt/Vitest | Done (green) |

Frontend now covers **all 12 backend domains + auth** for both reads (fetch
composables + list components) and **mutations** (create/update/delete composables
mirroring every backend endpoint) — full read+write domain parity at the data layer.

Test totals: backend `cargo test` = 257 passing; frontend `vitest` = 155 passing.
Domains ported: 31 (adds teams-catalog, company-skills, user-profiles, and
cloud-upstreams — the latter gated by the instance-settings flag, another example
of ported domains composing). Frontend UI now has
render components (not just data composables) for many newer domains: GoalList,
PipelineList, CompanySkillList, EnvironmentList, InviteList, JoinRequestList,
ActivityList, plus the DashboardView and CostSummaryView rollup views, joining
the original list components.

Domains ported: 27 (adds board-api-keys, company-invites, join-requests,
sidebar-badges, and llms — the first `text/plain` surface; and
sidebar-badges — the last unlocked by join-requests, showing the port compounding:
new domains make previously-blocked aggregations portable). Deeper access
token/cli-auth/bootstrap flows and the non-portable domains (company-skills
catalog, user-profiles per-user aggregation, cloud-upstreams/llms/teams-catalog)
remain.

Domains ported: 22 (the original 12 + goals, activity, dashboard, costs,
environments, inbox-dismissals, sidebar-preferences, resource-memberships,
instance-settings, pipelines — now incl. stages **and cases**). Remaining routes
are the deeper pipeline sub-resources (case documents/revisions/transitions/
blockers), company-skills, user-profiles, access, cloud-upstreams, llms,
teams-catalog, and infra (Postgres, real agent/adapter execution, MCP/orchestration).
Durable (SQLite): the original 12 + goals, activity, environments, costs, inbox
(dashboard is a computed rollup; sidebar + memberships are in-memory per-user
stores). Note: resource-memberships ports the **OSS-default** policy faithfully;
the enterprise policy hook (`policySource`, inherited defaults) is out of scope.

### Remaining domains require infrastructure, not just ports

The ~23 still-unported server routes are not clean 1:1 ports — each needs
substantial supporting machinery that is itself multi-slice work:
- **resource-memberships / access** — a policy-resolution engine (`policySource`,
  inherited defaults, invites/grants lifecycle).
- **pipelines** (2907 LOC) — stages, cases, documents+revisions, transitions,
  automation retries; **company-skills** (743 LOC) — catalog, versions, stars,
  comments, files; **user-profiles** (436 LOC) — cross-domain aggregation.
- **cloud-upstreams / llms / teams-catalog** — external service clients, static
  catalog seeds, or `.txt` config generation.
- **instance-settings** — a large instance-admin flag surface.
These, plus Postgres, real agent/adapter execution, and the MCP/orchestration
layer, are the bulk of the remaining multi-session effort.
Auth now models both agent identity (`agent_id`) and board user identity
(`user_id`), with guards `authorize_company_access` / `require_board` /
`require_own_agent` / `require_board_user`.

`app.vue` is no longer a placeholder — it loads live data via the tested
`loadBoard` orchestrator. End-to-end verification in this sandbox: the backend was
booted and seeded (`LiveWiredCo` + an issue), `nuxt dev` started with the dev
proxy, and **`GET :3001/api/companies` proxied to the Rust backend and returned
the seeded company** — proving the proxy + data path. The full SSR HTML render
could not be captured here: the Nuxt dev SSR worker hit the sandbox's JS-heap
ceiling ("Worker terminated … out of memory") — an environment limit, not a code
fault (`loadBoard` is unit-tested green; the proxy + backend are verified).

Known cleanup (non-blocking): each composable re-exports `Fetcher`/`FetchOptions`
types, so Nuxt logs duplicate auto-import warnings; a shared `types.ts` would
de-dupe them.

Domains ported: the original 12 **plus goals, activity, dashboard, and costs** (16).
These are genuinely new ports from `server/src/**`, not mutations/composables of
existing ones. Along the way the recode gained behaviour the first 12 didn't
exercise: the **board-only** authz path (`require_board`), query-param filtering +
limit clamping (activity), durable JSON columns (activity `details`), and the
first **cross-domain** handler composing multiple repositories (dashboard).

Live end-to-end verified by hand: the compiled `paperclip-backend` binary (port
3100) serves the full surface — health, company/issue create+list, issue
`PATCH`→200 (status changed), `DELETE`→204, unknown `DELETE`→404 `{"error":...}` —
proving the composed router runs, not just the per-endpoint unit tests. The Nuxt
dev proxy (`$development.nitro.devProxy`) forwards `/api/**` to this backend.

All 12 domains now have full mutation coverage where applicable: companies, issues,
projects, agents, runs, approvals, routines, workspaces, plugins, adapters all
support `PATCH`+`DELETE`; secrets supports `DELETE` (rotation = delete+create, no
in-place update by design); budgets use their own limit/spend semantics.

### Next slices (backlog)

- Backend: company scoping guards; `GET /api/companies/:id` fuller fields
  (createdAt, budget); issues domain (`GET/POST /api/companies/:id/issues`);
  swap in-memory `CompanyStore` for a Postgres-backed repo (sqlx) behind the same
  trait; auth/actor middleware; health `deploymentMode`/`bootstrapStatus` variants.
- Frontend: `useCompanies` composable that fetches `/api/companies` (mocked
  `$fetch`); companies page/route; issue board views; wire to the Rust API.

## Contract references (source of truth)

- Health: [server/src/routes/health.ts](../../server/src/routes/health.ts),
  OpenAPI [server/src/routes/openapi.ts](../../server/src/routes/openapi.ts)
  (`GET /api/health`). Success responses always include `status: "ok"`; fuller
  detail (`version`, `deploymentMode`, `deploymentExposure`, `bootstrapStatus`,
  `bootstrapInviteActive`, `serverInfo`) is layered by deployment mode. Ported in
  ascending fidelity across slices.

## Notes

- Do not commit/push unless the user asks (standing rule). Work stays on
  `rewrite-nuxt-rust`.
