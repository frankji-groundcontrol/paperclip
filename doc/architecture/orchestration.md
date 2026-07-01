# Architecture — Control-plane orchestration

> Internal architecture doc. Grounded in the actual server code under
> [`server/src/routes`](../../server/src/routes) and
> [`server/src/services`](../../server/src/services). Back to the
> [docs map](../index.md).

## Purpose / Overview

The **control plane** is the authoritative brain of a Paperclip instance. It owns
the durable state and the invariants that govern how autonomous agents get work,
execute it, spend money, and hand results back — without any single run being
able to escape its company, its budget, its trust boundary, or its checkout lock.

Concretely, the control plane:

- Models work as **issues** (a.k.a. tasks) in a company-scoped tree, each with at
  most one assignee.
- Drives execution through **heartbeat runs**: queued → running → terminal
  lifecycle records, one per agent invocation.
- Coalesces external triggers into **wakeup requests** that materialize runs.
- Enforces **atomic issue checkout** so two runs never execute the same issue.
- Meters spend into **cost events**, evaluates **budget policies**, and performs
  **hard-stop auto-pause** of agents / projects / companies.
- Gates risky operations behind **approvals** and **governed actions**.
- Realizes **execution workspaces** in **environments** (local / ssh / sandbox)
  via leases, and manages long-lived **workspace runtime services**.
- Stores **secrets** and binds them into runs.
- Hosts **plugins** that observe domain events and extend behavior.
- Writes an append-only **activity log** for every mutation.
- Contains **low-trust** work (hostile/injected input) inside sandbox +
  isolated-workspace boundaries and quarantines its output.

The control plane is an Express app over a single Postgres database (Drizzle
ORM). It is largely **stateless per request**: durable state lives in the DB, and
background timers reconcile it. The heaviest module,
[`services/heartbeat.ts`](../../server/src/services/heartbeat.ts), owns the run
lifecycle end-to-end.

Sibling docs: execution details in
[`../execution-semantics.md`](../execution-semantics.md), watchdog design in
[`../TASK-WATCHDOG.md`](../TASK-WATCHDOG.md), low-trust model in
[`../LOW-TRUST-PRESETS.md`](../LOW-TRUST-PRESETS.md), schema in
[`../DATABASE.md`](../DATABASE.md).

## Entry points

- **HTTP app assembly** — [`server/src/app.ts`](../../server/src/app.ts) is where
  routes are actually mounted: it builds an inner `/api` router, `use`s every
  route factory on it (`api.use(agentRoutes(db, ...))`, etc.), then mounts it with
  `app.use("/api", api)`, and wires the plugin worker manager
  (`createPluginWorkerManager`) and job scheduler (`createPluginJobScheduler`).
  [`routes/index.ts`](../../server/src/routes/index.ts) is a barrel that
  re-exports the route factories. Each factory returns an Express `Router` and
  receives the `Db` handle plus (where needed) a `PluginWorkerManager`.
- **Actor resolution** — [`middleware/auth.ts`](../../server/src/middleware/auth.ts)
  runs first (`actorMiddleware`) and sets `req.actor`. Every actor is one of:
  - `board` — a human/board session (`session`, `board_key`, `local_implicit`,
    or `cloud_tenant` source), optionally `isInstanceAdmin`, scoped to a set of
    `companyIds`.
  - `agent` — an agent key or agent JWT (`agent_key` / `agent_jwt`), pinned to a
    single `companyId` and `agentId`, usually carrying a `runId`.
  - `none` — unauthenticated.
- **Route-level guards** — [`routes/authz.ts`](../../server/src/routes/authz.ts)
  supplies `assertAuthenticated`, `assertBoard`, `assertBoardOrAgent`,
  `assertCompanyAccess`, `assertInstanceAdmin`, and `getActorInfo`. Deeper,
  resource-level decisions go through
  [`accessService`](../../server/src/services/access.ts) (`access.decide(...)`),
  which wraps the lower-level `decide(...)` in
  [`authorizationService`](../../server/src/services/authorization.ts) — routes
  call `access.decide(...)` and the authorization service owns the
  `AuthorizationAction` / `AuthorizationResource` / `AuthorizationDecision`
  contract.
- **Background scheduler** — [`server/src/index.ts`](../../server/src/index.ts)
  installs a single `setInterval` (`config.heartbeatSchedulerIntervalMs`) that
  drives `heartbeat.tickTimers`, `routines.tickScheduledTriggers`, and the whole
  reconciliation suite (`reapOrphanedRuns`, `promoteDueScheduledRetries`,
  `resumeQueuedRuns`, `reconcileStrandedAssignedIssues`,
  `reconcileIssueGraphLiveness`, `reconcileTaskWatchdogs`, `scanSilentActiveRuns`,
  `sweepStaleIssueLocks`, `reconcileProductivityReviews`).

## Key modules & responsibilities

### Issues / tasks / runs

- [`routes/issues.ts`](../../server/src/routes/issues.ts) — the largest route
  surface. Issue CRUD, tree operations, comments, documents, work products,
  thread interactions, checkout/release, feedback, attachments, and recovery
  actions. "Task" is the product-facing name for an issue; the DB table is
  `issues`.
- [`services/issues.ts`](../../server/src/services/issues.ts) — the issue domain
  service. It owns the atomic `checkout`, `release`, `adminForceRelease`, the
  single-assignee constraint, dependency/blocker readiness, and the
  `checkoutRunId` / `executionRunId` lock columns.
- [`services/issue-tree-control.ts`](../../server/src/services/issue-tree-control.ts)
  — subtree pause holds (a checkout can be blocked by an active ancestor pause
  hold gate).
- **Runs** are `heartbeatRuns` rows. There is no separate "run service" — the run
  lifecycle is [`services/heartbeat.ts`](../../server/src/services/heartbeat.ts).
  Each run has `status` (`queued` / `running` / terminal), an `invocationSource`
  (`timer` / `assignment` / `on_demand` / `automation`), a `contextSnapshot`
  (which carries `issueId`, `taskKey`, wake reason, execution policy), and a
  `wakeupRequestId`.

### Heartbeat processing (the run engine)

[`services/heartbeat.ts`](../../server/src/services/heartbeat.ts) is the core.
The `heartbeatService(db, { pluginWorkerManager, environmentRuntime })` factory
returns the run API. Key flow functions:

- **`enqueueWakeup(agentId, opts)`** (exported as `wakeup` / `invoke`) — the sole
  entry for "an agent should think now." It enriches a `contextSnapshot`, checks
  the company is `active` (else writes a `skipped` `agentWakeupRequests` row),
  applies coalescing (bumping `coalescedCount` on an existing queued
  continuation instead of stacking duplicate runs), and inserts an
  `agentWakeupRequests` + `heartbeatRuns` pair inside a transaction.
- **`tickTimers(now)`** — the timer loop. For every invokable agent in an active
  company whose heartbeat interval has elapsed, enqueues a `timer` wakeup; also
  fires due issue execution monitors.
- **`startNextQueuedRunForAgent` / `executeRun`** — claim queued runs (under an
  agent start lock, respecting `policy.maxConcurrentRuns` vs. the running count),
  prioritize them (in-progress issues first, then dependency-ready, then by issue
  priority, then FIFO), then invoke the adapter.
- **Ledgering** — after a run, `updateRuntimeState` normalizes usage
  (`normalizeUsageTotals`), updates `agentRuntimeState` totals, and — when there
  is billed cost or token usage — calls `costService.createEvent`, which cascades
  to budget evaluation.
- **Recovery / liveness** — `reapOrphanedRuns`, `resumeQueuedRuns`,
  `reconcileStrandedAssignedIssues`, `sweepStaleIssueLocks`, `scanSilentActiveRuns`,
  and `reconcileIssueGraphLiveness` heal state the scheduler owns.
  See also [`services/run-liveness.ts`](../../server/src/services/run-liveness.ts)
  and [`services/task-watchdogs.ts`](../../server/src/services/task-watchdogs.ts).

Concurrency uses `withAgentStartLock` (see
[`services/agent-start-lock.ts`](../../server/src/services/agent-start-lock.ts))
and DB row locks (`select ... for update`) so run claiming and issue-lock
transitions are serialized.

### Routines (recurring / triggered work)

- [`routes/routines.ts`](../../server/src/routes/routines.ts) and
  [`services/routines.ts`](../../server/src/services/routines.ts) — routines are
  agent-owned templates that fire on a schedule, manually, via API, or via signed
  **webhook triggers** (`routineTriggers`, with `signingMode` / `replayWindowSec`
  and rotatable secrets). `routineService(db).tickScheduledTriggers(now)` is
  called from the background loop; each fire creates a `routineRuns` row and
  wakes the assignee. A routine that requires a default agent cannot be enabled
  without an `assigneeAgentId` (`assertRoutineCanEnable`).
- Ownership guard: an agent may only manage routines assigned to itself
  (`assertCanManageCompanyRoutine` / `assertCanManageExistingRoutine`); board
  callers additionally need the `tasks:assign` permission to assign to others.

### Approvals & governed actions

- [`routes/approvals.ts`](../../server/src/routes/approvals.ts) +
  [`services/approvals.ts`](../../server/src/services/approvals.ts) — approvals
  are pending decisions on `approvals` rows (`hire_agent`,
  `budget_override_required`, and others). Only `pending` /
  `revision_requested` can be approved/rejected; `request-revision` →
  `resubmit` is the revision cycle.
- **Governed side effects**: approving a `hire_agent` approval activates or
  creates the agent and, if a monthly budget was requested, upserts an agent
  budget policy; rejecting it terminates a pending agent. On approval the
  requesting agent is woken (`heartbeat.wakeup`, reason `approval_approved`).
- **Run-context guard**: an agent whose run is a "cheap status-only recovery"
  run cannot create or modify approvals
  (`assertApprovalMutationAllowedByRunContext`).
- Decision endpoints require `assertBoard`. `hire_agent` payloads are normalized
  through the secret service so secret values are stored as references, not
  inline.

### Costs & budgets (metering + hard-stop auto-pause)

- [`routes/costs.ts`](../../server/src/routes/costs.ts) +
  [`services/costs.ts`](../../server/src/services/costs.ts) — `createEvent`
  inserts a `costEvents` row, recomputes the agent's and company's
  `spentMonthlyCents` for the current UTC month, then calls
  `budgets.evaluateCostEvent(event)`. Aggregation endpoints roll spend up by
  agent, provider, biller, model, project, and rolling time windows. An agent may
  only report **its own** costs.
- [`services/budgets.ts`](../../server/src/services/budgets.ts) — budget policies
  (`budgetPolicies`) are scoped to `company`, `agent`, or `project`, with a
  metric (`billed_cents`), a window (`calendar_month_utc` or `lifetime`), a
  `warnPercent`, and `hardStopEnabled`. On each cost event:
  - At/above the soft threshold (`warnPercent`) → open a **soft** budget
    incident and log `budget.soft_threshold_crossed`.
  - At/above the limit with hard-stop enabled → resolve soft incidents, open a
    **hard** incident with a linked `budget_override_required` approval, then
    **`pauseAndCancelScopeForBudget`**: set the scope to `paused` with
    `pauseReason = "budget"` and cancel in-flight work via the injected
    `cancelWorkForScope` hook (wired to `heartbeat.cancelBudgetScopeWork`).
  - `getInvocationBlock(companyId, agentId, ctx)` is the pre-flight gate: it
    returns a block reason if the company, agent, or project is paused or over a
    hard-stop, and the heartbeat engine consults it before starting work.
  - Resolution is either `raise_budget_and_resume` (must exceed observed spend,
    then `resumeScopeFromBudget`) or dismiss; both mark the linked approval.

### Execution workspaces & workspace runtime services

- [`routes/execution-workspaces.ts`](../../server/src/routes/execution-workspaces.ts)
  + [`services/execution-workspaces.ts`](../../server/src/services/execution-workspaces.ts)
  — an `executionWorkspaces` row records where a run's code lives (its `mode`,
  `config`, and realization metadata). Modes include `isolated_workspace`
  (required for low-trust). Close-readiness endpoints gate teardown; a closed
  isolated workspace blocks new checkouts against its issue.
- [`services/workspace-runtime.ts`](../../server/src/services/workspace-runtime.ts)
  — realizes workspaces on disk. `RealizedExecutionWorkspace.strategy` is either
  `project_primary` (the project's primary checkout) or `git_worktree` (an
  isolated worktree, auto-refreshed to the base ref when unstarted). It also owns
  **workspace runtime services** (`workspaceRuntimeServices`): long-lived
  processes (dev servers, etc.) with a desired-state map (`running` / `stopped` /
  `manual`). Requests to start/stop them are authorized through the low-trust and
  company-access guards in
  [`routes/workspace-runtime-service-authz.ts`](../../server/src/routes/workspace-runtime-service-authz.ts)
  (`assertLowTrustCanManageRuntimeForActor` /
  `assertLowTrustCanManageRuntimeForIssue` /
  `assertCanManageProjectWorkspaceRuntimeServices`).
- [`services/workspace-operations.ts`](../../server/src/services/workspace-operations.ts)
  — a `WorkspaceOperationRecorder` writes every workspace phase (provision,
  worktree prepare, command, cleanup) to `workspaceOperations` with a redacted,
  size-capped log stored via the operation log store. This is the per-workspace
  audit trail, distinct from the domain activity log.
- [`services/workspace-realization.ts`](../../server/src/services/workspace-realization.ts)
  builds the realization request the environment runtime consumes.

### Environment run orchestrator

[`services/environment-run-orchestrator.ts`](../../server/src/services/environment-run-orchestrator.ts)
centralizes the full environment lifecycle for a run so the heartbeat engine does
not inline it. `environmentRunOrchestrator(db, ...)` exposes:

- `acquireForRun` — resolve the selected environment (falling back to the lazily
  created local environment), assert it is `active`, acquire (or resume) an
  `environmentLeases` lease via the runtime driver, log
  `environment.lease_acquired`, and resolve the execution transport.
- `realizeForRun` — build the realization request, realize the workspace for
  `local` / `ssh` / `sandbox` drivers, run any provision command, persist
  realization metadata onto the lease and execution workspace, then resolve the
  adapter execution target (local / ssh / sandbox remote spec).
- `releaseForRun` — release all leases for a run, logging
  `environment.lease_released` per lease; individual release failures never mask
  the original run failure.

Errors are typed via `EnvironmentRunError` with codes like
`environment_inactive`, `lease_acquire_failed`, `workspace_realization_failed`.
The underlying driver is
[`services/environment-runtime.ts`](../../server/src/services/environment-runtime.ts);
higher-level config is
[`services/environments.ts`](../../server/src/services/environments.ts).

### Secrets

- [`routes/secrets.ts`](../../server/src/routes/secrets.ts) +
  [`services/secrets.ts`](../../server/src/services/secrets.ts) — secrets,
  secret-provider configs ("vaults"), remote import/preview, rotation, usage
  (binding references), and access events. **Every** secret route requires
  `assertBoard` + `assertCompanyAccess`: agents cannot manage secrets through the
  HTTP API; they receive resolved bindings at run time. Managed modes let a
  provider (e.g. AWS) own the value while Paperclip stores only an `externalRef`.
  The service is also what the approval and hire paths call to keep secret values
  out of persisted approval payloads.

### Plugin host services

- [`services/plugin-host-services.ts`](../../server/src/services/plugin-host-services.ts)
  is a large aggregating service. It implements the `HostServices` contract exposed to
  plugin code (companies, agents, projects, issues, goals, documents, budgets,
  issue approvals, thread interactions, workspace metadata, live-event
  subscriptions) with per-plugin scoping.
- Domain mutations reach plugins through the event bus. `logActivity` (below)
  translates activity actions into plugin events via `publishPluginDomainEvent`
  and [`services/plugin-event-bus.ts`](../../server/src/services/plugin-event-bus.ts).
  Supporting modules: plugin worker manager, job scheduler/coordinator/store,
  lifecycle, loader, registry, and tool dispatcher/registry under
  [`server/src/services`](../../server/src/services).

### Activity logging for mutations

[`services/activity-log.ts`](../../server/src/services/activity-log.ts) exports
`logActivity(db, input)`. Every mutating route calls it after the write. It:

1. Sanitizes and (optionally) redacts the current-user value in `details`
   (driven by `censorUsernameInLogs` instance setting).
2. Inserts an `activityLog` row (`companyId`, `actorType`, `actorId`, `action`,
   `entityType`, `entityId`, optional `agentId` / `runId`, `details`).
3. Publishes an `activity.logged` live event.
4. If the action maps to a plugin event type (e.g.
   `issue_comment_added → issue.comment.created`,
   `budget_hard_threshold_crossed → budget.incident.opened`), emits it on the
   plugin event bus.

[`routes/activity.ts`](../../server/src/routes/activity.ts) reads the log per
company / issue / run, gated by `company_scope:read` and `issue:read` decisions.

## Data flow / execution lifecycle

The canonical "agent does work on a task" path:

1. **Trigger.** A timer tick, an assignment, an approval, a routine, a webhook,
   or an explicit `POST /agents/:id/wakeup` calls `heartbeat.enqueueWakeup`.
2. **Admission.** `enqueueWakeup` verifies the company is `active`, consults the
   budget invocation block (paused/over-budget scopes are suppressed), coalesces
   against an existing queued continuation, and — on admission — inserts an
   `agentWakeupRequests` + a `queued` `heartbeatRuns` row in one transaction.
3. **Claim.** The scheduler (or an inline path) runs
   `startNextQueuedRunForAgent` under the agent start lock: it checks
   invokability and `maxConcurrentRuns`, prioritizes queued runs, and claims
   slots.
4. **Checkout.** For issue-scoped work the agent calls
   `POST /issues/:id/checkout` (`services/issues.ts#checkout`). This is the
   atomic gate (see invariants). It sets `assigneeAgentId`, `checkoutRunId`,
   `executionRunId`, and `status = "in_progress"` only if the issue is unlocked
   or owned by the same run.
5. **Environment + workspace.** The heartbeat engine delegates to
   `environmentRunOrchestrator.acquireForRun` then `realizeForRun`: acquire a
   lease, realize the workspace (primary checkout or git worktree), resolve the
   execution target.
6. **Adapter execution.** The adapter runs the model in the resolved
   environment; `run` transitions `queued → running`. Runtime status/liveness is
   tracked, and workspace operations are recorded.
7. **Ledger.** Usage is normalized and, when nonzero, `costService.createEvent`
   writes a `costEvents` row, updates `spentMonthlyCents`, and calls
   `budgets.evaluateCostEvent`.
8. **Budget enforcement.** If a hard threshold is crossed, the scope is paused
   (`pauseReason = "budget"`), in-flight work is cancelled, and a
   `budget_override_required` approval is opened.
9. **Release.** On completion, leases are released
   (`environmentRunOrchestrator.releaseForRun`), the issue lock is cleared, and
   the run reaches a terminal status.
10. **Audit + fan-out.** Each step logs an `activityLog` entry and (where
    mapped) emits a plugin domain event.

Budget hard-stop as a standalone flow: cost event → `evaluateCostEvent` → hard
incident + approval + `pauseAndCancelScopeForBudget` → `cancelBudgetScopeWork`
cancels running/queued runs for the scope. Resume requires a board decision
(`raise_budget_and_resume`) which reactivates the policy and calls
`resumeScopeFromBudget`.

## Contracts & cross-layer coupling

- **Route → service → DB.** Routes validate with Zod (`middleware/validate.ts`
  and shared schemas), authorize, call a service, then `logActivity`. Services
  own transactions and invariants. Shared types/schemas come from
  `@paperclipai/shared`; tables from `@paperclipai/db`.
- **Authorization contract.** The `AuthorizationAction` /
  `AuthorizationResource` / `AuthorizationDecision` triple in
  [`services/authorization.ts`](../../server/src/services/authorization.ts) is
  the resource-level decision surface. Route guards handle coarse company access;
  `access.decide(...)` handles per-issue / per-scope reads and the special grant
  reasons (`allow_issue_mention_grant`, task-bridge scope, low-trust boundary).
- **Budget hook injection.** `costService` and `budgetService` accept
  `BudgetServiceHooks.cancelWorkForScope`. Cost/route wiring injects
  `heartbeat.cancelBudgetScopeWork`, so metering can cancel runs without the
  budget module importing the heartbeat engine directly.
- **Environment orchestrator ↔ heartbeat.** The heartbeat engine constructs an
  `environmentRunOrchestrator` and shares the same `environmentRuntime`/
  `pluginWorkerManager`, so leases and workspaces are consistent across the run.
- **Activity → plugins.** `logActivity` is the one-way bridge from domain
  mutations to the plugin event bus; the mapping table lives in
  `activity-log.ts`.
- **Single writer for run state.** Only the heartbeat engine transitions
  `heartbeatRuns.status` and clears issue locks; other modules request work via
  `wakeup` or cancellation APIs.

## Extension points

- **Adapters** decouple the run engine from any specific model runner (see
  `server/src/adapters` and the adapter execution-target utilities).
- **Environment drivers** (`local`, `ssh`, `sandbox`) plug into
  `environment-runtime.ts`; adding a driver means implementing lease acquire /
  workspace realize / execute.
- **Secret providers** are pluggable vaults behind `secretService`
  (`configured-provider`, managed modes, remote import).
- **Plugins** subscribe to domain events and call `HostServices`; managed agents,
  routines, and skills let a plugin own control-plane objects.
- **Budget metrics/scopes** are data-driven (`budgetPolicies` columns), so new
  scopes/windows are additive.
- **Routine triggers** support schedule / manual / api / signed-webhook kinds.

## Testing

Service and route tests live alongside the code and under
`server/src/__tests__`. Relevant examples:

- [`services/heartbeat-run-runtime-status.test.ts`](../../server/src/services/heartbeat-run-runtime-status.test.ts),
  [`services/heartbeat-stop-metadata.test.ts`](../../server/src/services/heartbeat-stop-metadata.test.ts)
  — run status/liveness projection.
- [`services/execution-allowlist.test.ts`](../../server/src/services/execution-allowlist.test.ts),
  [`services/execution-policy-bootstrap.test.ts`](../../server/src/services/execution-policy-bootstrap.test.ts)
  — execution policy gating.
- `server/src/__tests__/issues-service.test.ts` and
  `server/src/__tests__/issue-agent-mutation-ownership-routes.test.ts` — checkout
  ownership and single-assignee behavior.

Run the server test suite from the `server` workspace (see
[`../DEVELOPING.md`](../DEVELOPING.md) for the exact commands and lockfile
policy). Prefer service-level tests for invariant coverage (atomic checkout,
budget hard-stop, low-trust fail-closed) since those are where correctness bugs
are most damaging.

## Gotchas / invariants

- **Single-assignee task model.** An issue has at most one live owner. `checkout`
  only succeeds when `assigneeAgentId` is null, or already this agent under a
  compatible run lock; otherwise it throws `conflict("Issue checkout conflict")`.
  Setting an agent assignee clears any user assignee (`assigneeUserId: null`).
- **Atomic issue checkout.** The lock is a conditional `UPDATE ... WHERE` on
  `issues` guarded by `status IN (expectedStatuses)`,
  `assigneeAgentId IS NULL or same-run`, and the execution-lock condition — so
  two concurrent runs cannot both win. `checkoutRunId` and `executionRunId` are
  the lock columns; stale locks pointing at terminal/missing runs are cleared
  (`clearCheckoutRunIfTerminal` / `clearExecutionRunIfTerminal`) and adoptable by
  the same agent. Board `admin/force-release` is the manual escape hatch.
- **Company / session / run isolation.** An `agent` actor is pinned to one
  `companyId`; `assertCompanyAccess` rejects cross-company agent keys. Board
  users must be members with an active, non-viewer role for mutations. The
  `runId` header binds an agent's writes to its own run; source-trust resolution
  **fails closed** when a run is unknown or its `agentId` mismatches.
- **Budget hard-stop is destructive and sticky.** Crossing a hard threshold both
  pauses the scope and cancels in-flight work; it stays paused (with
  `pauseReason = "budget"`) until a board member raises the budget above observed
  spend. `getInvocationBlock` re-checks live spend, so a scope can be blocked even
  if a stale pause flag was cleared.
- **Approvals gate governed actions, not just notifications.** Approving
  `hire_agent` mutates agents and budgets; approving `budget_override_required`
  resumes a scope. Cheap status-only recovery runs are forbidden from touching
  approvals.
- **Low-trust boundaries fail closed.** For a `low_trust_review` resolution
  ([`services/trust-preset-resolver.ts`](../../server/src/services/trust-preset-resolver.ts)),
  managed execution requires **all** of: isolated workspaces enabled, effective
  mode `isolated_workspace`, the issue inside the resolved boundary, and a
  `sandbox` environment driver — enforced by `assertLowTrustWorkspaceIsolation` in
  [`services/low-trust-runtime-containment.ts`](../../server/src/services/low-trust-runtime-containment.ts).
  Separately, the heartbeat engine restricts secret bindings to the boundary's
  `allowedSecretBindingIds` allow-list (in
  [`services/heartbeat.ts`](../../server/src/services/heartbeat.ts)), and
  `assertLowTrustRuntimeServicesAllowed` denies runtime services unless the
  boundary grants `runtime.manage`.
  Boundaries are intersected across agent/project/issue/run policy sources
  (narrower wins; cross-company or scopeless boundaries are denied). Low-trust
  **output is quarantined**: comments/documents get `sourceTrust.disposition =
  "quarantined"` and are redacted for higher-trust readers until a reviewer
  promotes a sanitized artifact
  ([`services/source-trust.ts`](../../server/src/services/source-trust.ts)).
- **Low-trust actor boundaries beyond issues.**
  - *Agents* — agent keys are single-company, single-agent; they cannot reach
    peer agents, company-wide, runtime, or secret APIs except through explicit
    grants.
  - *Mentions* — an agent mentioned in an issue comment gets a narrow
    `allow_issue_mention_grant` (comment-scoped), not full issue access.
  - *Task bridges* — `task_bridge`-origin keys are the tightest actor: they can
    only create issues inside an approved parent/project boundary and can only
    read/mutate issues they created or are assigned; they are denied company,
    peer-agent, project, runtime, and secret APIs
    ([`services/authorization.ts`](../../server/src/services/authorization.ts)).
- **Activity logging is best-effort but expected.** Mutating routes log after the
  write; a logging failure must not roll back the mutation (e.g. lease-release
  logging is wrapped in try/catch). Do not treat the activity log as
  transactional with the mutation.
- **Wakeups coalesce; runs do not stack.** Repeated triggers for the same agent
  bump `coalescedCount` on a queued continuation rather than creating parallel
  runs; `maxConcurrentRuns` caps genuine concurrency. Reconciliation loops
  (orphan reaping, stranded-issue recovery, stale-lock sweeping) are the safety
  net — the system self-heals on the scheduler interval.
- **Secrets never flow through agent HTTP paths.** All secret management is
  board-only; values are stored as references where a managed provider owns them,
  and are injected into runs as bindings.

## Key files

- [`server/src/app.ts`](../../server/src/app.ts) — route mounting and worker/scheduler wiring.
- [`server/src/index.ts`](../../server/src/index.ts) — background scheduler + reconciliation loop.
- [`server/src/middleware/auth.ts`](../../server/src/middleware/auth.ts) — actor resolution.
- [`server/src/routes/authz.ts`](../../server/src/routes/authz.ts) — coarse route guards.
- [`server/src/routes/issues.ts`](../../server/src/routes/issues.ts) — issue/task/checkout routes.
- [`server/src/services/issues.ts`](../../server/src/services/issues.ts) — atomic checkout + single-assignee.
- [`server/src/services/heartbeat.ts`](../../server/src/services/heartbeat.ts) — run lifecycle, wakeups, ledgering, recovery.
- [`server/src/services/environment-run-orchestrator.ts`](../../server/src/services/environment-run-orchestrator.ts) — lease + workspace realization per run.
- [`server/src/services/budgets.ts`](../../server/src/services/budgets.ts) — budget policies, incidents, hard-stop auto-pause.
- [`server/src/services/costs.ts`](../../server/src/services/costs.ts) — cost metering + aggregation.
- [`server/src/services/approvals.ts`](../../server/src/services/approvals.ts) — approval lifecycle + governed side effects.
- [`server/src/services/secrets.ts`](../../server/src/services/secrets.ts) — secrets + provider vaults.
- [`server/src/services/routines.ts`](../../server/src/services/routines.ts) — routines + triggers.
- [`server/src/services/execution-workspaces.ts`](../../server/src/services/execution-workspaces.ts) / [`workspace-runtime.ts`](../../server/src/services/workspace-runtime.ts) — execution workspaces + runtime services.
- [`server/src/services/authorization.ts`](../../server/src/services/authorization.ts) — resource-level decisions, task bridges, mentions.
- [`server/src/services/trust-preset-resolver.ts`](../../server/src/services/trust-preset-resolver.ts) / [`low-trust-runtime-containment.ts`](../../server/src/services/low-trust-runtime-containment.ts) / [`source-trust.ts`](../../server/src/services/source-trust.ts) — low-trust boundaries + quarantine.
- [`server/src/services/activity-log.ts`](../../server/src/services/activity-log.ts) — mutation audit + plugin fan-out.
- [`server/src/services/plugin-host-services.ts`](../../server/src/services/plugin-host-services.ts) — plugin `HostServices`.
