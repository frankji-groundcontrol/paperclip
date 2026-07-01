# Architecture — Server (HTTP API & composition)

Internal architecture doc for the `@paperclipai/server` subsystem — the Express
HTTP API, its middleware pipeline, request/actor model, and the process-level
startup that boots the database and mounts everything together.

See also: [plugins](./plugins.md) · [adapters](./adapters.md) ·
[data model](./data-model.md) · [shared contracts](./shared-contracts.md).
The long-running work this layer exposes over HTTP (issues, runs, heartbeat,
routines, approvals, budgets) lives in the `server/src/services/` layer.

> Scope note: this document covers the HTTP/composition layer. The heartbeat
> scheduler, routine ticks, run reconciliation, and other long-running
> orchestration lifecycles are *kicked off* from
> [server/src/index.ts](../../server/src/index.ts) after the app is built, but
> their semantics live in the service modules under
> [server/src/services/](../../server/src/services) (e.g. `heartbeatService`,
> the routine scheduler, and the `reconcile*OnStartup` helpers).

---

## Purpose / Overview

`@paperclipai/server` is the single Node/Express process that serves the
Paperclip control-plane REST API (mounted under `/api`), the better-auth
endpoints (`/api/auth/*`), a websocket live-events channel, plugin worker
orchestration, and — depending on config — the built UI (static or Vite dev
middleware). It is a workspace package
([server/package.json](../../server/package.json)) whose public entry re-exports
`startServer` from [server/src/index.ts](../../server/src/index.ts).

The subsystem has two clearly separated concerns:

1. **Process bootstrap** ([server/src/index.ts](../../server/src/index.ts)) —
   resolve config, gate/apply migrations, bring up (embedded or external)
   PostgreSQL, wire auth, construct the app, start schedulers, listen, and
   handle graceful shutdown.
2. **HTTP composition** ([server/src/app.ts](../../server/src/app.ts)) —
   `createApp(db, opts)` builds the Express `app`: middleware order, the `/api`
   sub-router, auth handler mounting, plugin host wiring, and UI serving.

Everything is dependency-injected around a single `Db` handle
(`@paperclipai/db`, a Drizzle client) — there is no global DB singleton, which is
what makes the whole app trivially instantiable inside tests via
[supertest](../../server/src/__tests__/setup-supertest.ts).

---

## Entry points

- **`startServer()`** — [server/src/index.ts](../../server/src/index.ts). The
  process-level orchestrator. Exported for embedding; also self-invoked when the
  module is the main module (`isMainModule(import.meta.url)`).
- **`createApp(db, opts)`** — [server/src/app.ts](../../server/src/app.ts).
  Returns a fully-wired Express `app`. `opts` carries everything the app cannot
  discover on its own: `uiMode`, `serverPort`, `storageService`,
  `deploymentMode`, `deploymentExposure`, `allowedHostnames`, `bindHost`,
  `authReady`, `betterAuthHandler`, `resolveSession`, `pluginWorkerManager`,
  and optional `feedbackExportService` / `databaseBackupService`.
- **Package export** — [server/package.json](../../server/package.json)
  `exports["."]` points at `./src/index.ts` (dev) / `./dist/index.js` (packed).

### Startup / migration gating & DB safety

[server/src/index.ts](../../server/src/index.ts) enforces a strict ordering so
the server never accepts traffic against a stale or wrong database:

- **Instrumentation first.** `await instrumentationReady` blocks before any DB
  connection or the HTTP server exists, so OTel trace coverage does not depend
  on incidental timing ([server/src/instrumentation.ts](../../server/src/instrumentation.ts)).
- **Cloud DB contract.** `assertCloudDatabaseContract()` refuses to boot an
  `authenticated` + `public` deployment without a real postgres `DATABASE_URL`
  (no embedded-postgres fallback in that mode).
- **Migration gating** via `ensureMigrations(connectionString, label, opts)`,
  built on `inspectMigrations` / `applyPendingMigrations` /
  `reconcilePendingMigrationHistory` from `@paperclipai/db`. Behavior:
  - `upToDate` → returns `already applied`.
  - Drifted journal (`pending-migrations`) → attempts
    `reconcilePendingMigrationHistory` to repair journal entries from existing
    schema, then re-inspects.
  - Otherwise it either auto-applies (first-run embedded setup, or
    `PAPERCLIP_MIGRATION_AUTO_APPLY=true`, or a non-TTY stdin/stdout), prompts
    interactively on a TTY, or **throws and refuses to start** against a stale
    schema. `PAPERCLIP_MIGRATION_PROMPT=never` forces a refusal in non-auto
    mode.
- **Embedded vs external Postgres.** With no `DATABASE_URL`, the server lazily
  imports `embedded-postgres`, prepares the native runtime, detects a free port
  (reusing an already-running cluster by pid file or reachability probe), and
  applies first-run migrations automatically. External postgres uses
  `config.databaseUrl` (with an optional separate `databaseMigrationUrl` for the
  migration connection).
- **Deployment-mode invariants** (fail-loud, before listening):
  `local_trusted` requires a loopback `host` and `private` exposure;
  `authenticated` requires coherent `authBaseUrlMode` / `authPublicBaseUrl`
  (explicit base URL for public exposure).
- **Local-trusted board principal.** In `local_trusted` mode,
  `ensureLocalTrustedBoardPrincipal(db)` seeds the `local-board` user, an
  `instance_admin` role, and `owner` memberships for every company so the
  implicit board actor is authorized. `backfillPrincipalAccessCompatibility`
  seeds default grants on every boot.
- **Schedulers & recovery** are started *after* the app is built and *before*
  `server.listen(...)`, gated on `config.heartbeatSchedulerEnabled`: heartbeat
  orphan-run reaping (`heartbeat.reapOrphanedRuns()`), queued-run resumption
  (`heartbeat.resumeQueuedRuns()`), stranded-issue reconciliation
  (`heartbeat.reconcileStrandedAssignedIssues()`), heartbeat + routine scheduler
  timer ticks, adapter-availability reconciliation
  (`reconcileAdapterAvailability`), and the `reconcile*OnStartup` helpers
  (persisted runtime services, cloud-upstream runs, codex-local managed homes).
  These are orchestration concerns whose semantics live in the
  [server/src/services/](../../server/src/services) layer; this doc only covers
  the fact that they are *launched* from
  [server/src/index.ts](../../server/src/index.ts).
- **Keep-alive tuning.** `server.keepAliveTimeout = 185000` /
  `headersTimeout = 186000` to outlive common reverse-proxy idle timeouts and
  avoid intermittent 502/ECONNRESET.
- **Graceful shutdown.** `SIGINT`/`SIGTERM` flush telemetry, call
  `app.locals.paperclipShutdown` (plugin/dev-watcher/feedback cleanup), stop
  embedded postgres only if this process started it, and
  `shutdownInstrumentation()` to flush OTel spans.

---

## Key modules & responsibilities

### App composition — [server/src/app.ts](../../server/src/app.ts)

`createApp` is where the entire request pipeline is assembled. Middleware order
is load-bearing (see [Data flow](#data-flow--request-lifecycle)). The router
strategy:

- Top-level `app.use(...)` for cross-cutting middleware, auth, and UI/static.
- A dedicated `api = Router()` mounted at `/api`, holding every REST route
  factory, followed by an `/api` 404 fallthrough that returns
  `{ error: "API route not found" }`.

> `llmRoutes(db)` is mounted **twice** on purpose: once top-level
> (`app.use(llmRoutes(db))`, serving `/llms/*` for adapter agent-configuration
> reflection) and once inside the `api` router (`api.use(llmRoutes(db))`,
> serving the same handlers under `/api/llms/*`). The plugin host wiring
> (worker manager, job store, scheduler, lifecycle, tool dispatcher, event bus,
> host services, dev watcher) is also constructed inline in `createApp` before
> `pluginRoutes` is mounted.

Body parsing is split: the company-import path (`COMPANY_IMPORT_API_PATH`, from
[server/src/routes/company-import-paths.ts](../../server/src/routes/company-import-paths.ts))
gets the larger `PORTABLE_JSON_BODY_LIMIT` (`64mb`), everything else the
`DEFAULT_JSON_BODY_LIMIT` (`10mb`) — see
[server/src/http/body-limits.ts](../../server/src/http/body-limits.ts). Both
capture the raw body onto `req.rawBody` for signature-style verification.

### Route factory pattern — [server/src/routes/](../../server/src/routes)

Every route module exports a **factory function**, not a router instance. The
convention:

```ts
export function goalRoutes(db: Db) {
  const router = Router();
  const svc = goalService(db);          // service constructed per-factory
  router.get("/companies/:companyId/goals", async (req, res) => { ... });
  return router;
}
```

(from [server/src/routes/goals.ts](../../server/src/routes/goals.ts)). The
factory receives the injected `Db` (and sometimes extra deps like
`storageService` or `pluginWorkerManager`), constructs the services it needs,
and returns a `Router`. `createApp` mounts each returned router onto the `api`
sub-router. Most factories mount their own full path prefixes
(e.g. `/companies/:companyId/goals`, `/goals/:id`) rather than being mounted
under a base path, so a single factory can span several resource roots.

> **`server/src/routes/index.ts` is only a re-export surface.** See
> [server/src/routes/index.ts](../../server/src/routes/index.ts) — it re-exports
> the factory functions for convenience but is **not** what `app.ts` imports;
> `app.ts` imports each factory from its concrete module. It is also not
> exhaustive (e.g. `cloudUpstreamRoutes` is exported there but mounted elsewhere;
> `openApiRoutes`, `adapterRoutes`, `pluginRoutes` are imported directly). Treat
> `index.ts` as a courtesy barrel, not the source of truth for what is mounted.

Authorization inside handlers is done with helpers from
[server/src/routes/authz.ts](../../server/src/routes/authz.ts):
`assertAuthenticated`, `assertBoard`, `assertBoardOrgAccess`,
`assertBoardOrAgent`, `assertInstanceAdmin`, `assertCompanyAccess`, and
`getActorInfo`. These read the normalized `req.actor` (below) and throw
`HttpError`s. `assertCompanyAccess` is the workhorse: it scopes agents to their
own company, checks board membership/instance-admin, and enforces
read-only/`viewer` restrictions on mutating methods.

### Middleware — [server/src/middleware/](../../server/src/middleware)

- **Actor / auth normalization** —
  [server/src/middleware/auth.ts](../../server/src/middleware/auth.ts).
  `actorMiddleware(db, opts)` runs on every request and populates `req.actor`
  (typed in [server/src/types/express.d.ts](../../server/src/types/express.d.ts))
  to one discriminated shape: `board` | `agent` | `none`, with a `source` field
  (`local_implicit`, `session`, `board_key`, `agent_key`, `agent_jwt`,
  `cloud_tenant`, `none`). Resolution order:
  1. `local_trusted` → implicit `local-board` admin actor.
  2. No `Bearer` token, `authenticated` mode → trusted Cloud-tenant header
     actor (`resolveCloudTenantActor`, guarded by a constant-time compare of
     `x-paperclip-cloud-tenant-token`) → else better-auth session via
     `resolveSession`.
  3. `Bearer <token>` → board API key → agent API key (sha256 hash lookup in
     `agentApiKeys`) → local agent JWT (`verifyLocalAgentJwt`).
  An `x-paperclip-run-id` header is threaded onto `req.actor.runId` for run
  attribution. `resolveCloudTenantActor` also *provisions* the tenant user,
  company, and membership on the fly and actively purges stale
  `instance_admin` rows for cloud-tenant users.
- **`validate(schema)`** —
  [server/src/middleware/validate.ts](../../server/src/middleware/validate.ts).
  A one-liner that runs `schema.parse(req.body)` (Zod) and reassigns
  `req.body`. Thrown `ZodError`s are formatted by the error handler.
- **Error handler** —
  [server/src/middleware/error-handler.ts](../../server/src/middleware/error-handler.ts).
  Terminal middleware. `HttpError` → its `status` + `{ error, code?, details? }`;
  `ZodError` → `400 { error: "Validation error", details }`; anything else →
  `500 { error: "Internal server error" }`. For `>= 500` it attaches an
  `__errorContext` (used by the HTTP logger) and fires
  `trackErrorHandlerCrash` telemetry. It only leaks the raw message for the
  narrow trusted cloud-tenant company-import case.
- **Board mutation guard** —
  [server/src/middleware/board-mutation-guard.ts](../../server/src/middleware/board-mutation-guard.ts).
  CSRF-style protection: mounted at the head of the `/api` router. For unsafe
  methods by a browser-session board actor, it requires a trusted
  `Origin`/`Referer` (dev origins, the request Host, or `PAPERCLIP_PUBLIC_URL`).
  Non-browser board sources (`local_implicit`, `board_key`, `cloud_tenant`) and
  non-board actors bypass it.
- **Private hostname guard** —
  [server/src/middleware/private-hostname-guard.ts](../../server/src/middleware/private-hostname-guard.ts).
  Blocks requests whose Host header is not loopback or on the allow-set. Enabled
  only for `private` exposure in `local_trusted`/`authenticated` modes
  (`shouldEnablePrivateHostnameGuard`). The allow-set is loopback + `bindHost` +
  configured `allowedHostnames`; the error message points operators at
  `pnpm paperclipai allowed-hostname <host>`. In `vite-dev` mode the same
  allow-set is passed to Vite's `allowedHosts`.
- **Trust proxy** —
  [server/src/middleware/trust-proxy.ts](../../server/src/middleware/trust-proxy.ts).
  `parseTrustProxyEnv(process.env.TRUST_PROXY)` + `applyTrustProxy(app, …)`.
  Default is **unset** → Express trusts nothing (so `req.ip` /
  `X-Forwarded-For` cannot be spoofed). Accepts `true`, a hop count, or a
  comma-separated subnet/CIDR list; throws at startup on obvious garbage.
- **HTTP logger** —
  [server/src/middleware/logger.ts](../../server/src/middleware/logger.ts).
  Exports both the shared `logger` (pino, dual pino-pretty transports to stdout
  + a `server.log` file) and `httpLogger` (pino-http). Success logs can be
  silenced via [http-log-policy.ts](../../server/src/middleware/http-log-policy.ts);
  error props are redacted through
  [redact-sensitive.ts](../../server/src/middleware/redact-sensitive.ts). The
  base logger redacts `req.headers.authorization`.

### Errors — [server/src/errors.ts](../../server/src/errors.ts)

A tiny `HttpError extends Error { status; details? }` plus factory helpers:
`badRequest` (400), `unauthorized` (401), `forbidden` (403), `notFound` (404),
`conflict` (409), `unprocessable` (422). Handlers throw these; the error handler
translates them. `details.code` (when present) is surfaced as a top-level
`code` on the JSON body.

### Auth wiring — [server/src/auth/](../../server/src/auth)

- [server/src/auth/better-auth.ts](../../server/src/auth/better-auth.ts) builds
  the better-auth instance over the Drizzle adapter (`provider: "pg"`), mapping
  better-auth's `user`/`session`/`account`/`verification` schema keys onto the
  `authUsers`/`authSessions`/`authAccounts`/`authVerifications` tables from
  `@paperclipai/db`, with email+password enabled (`disableSignUp` from config),
  an instance-scoped cookie prefix (`deriveAuthCookiePrefix`), derived trusted
  origins (`deriveAuthTrustedOrigins`), and secure-cookie gating
  (`shouldDisableSecureAuthCookies`). It exports `createBetterAuthHandler`
  (wrapping `toNodeHandler`), `resolveBetterAuthSession(req)`, and
  `resolveBetterAuthSessionFromHeaders(headers)` (the latter reused by the
  websocket server).
- **Mounting order in `app.ts`:** the app first mounts its own
  [server/src/routes/auth.ts](../../server/src/routes/auth.ts) at `/api/auth`
  (`/get-session`, `/profile`) which reads the *already-normalized* `req.actor`,
  then mounts the better-auth catch-all handler at `/api/auth/{*authPath}` for
  sign-in/up/session management. `betterAuthHandler` and `resolveSession` are
  only constructed in `authenticated` mode (in `index.ts`) and injected into
  `createApp`.
- The secret for better-auth comes from `BETTER_AUTH_SECRET` (falling back to
  `PAPERCLIP_AGENT_JWT_SECRET`); startup throws if neither is set.

### OpenAPI — [server/src/routes/openapi.ts](../../server/src/routes/openapi.ts)

The OpenAPI document is **hand-maintained**, not generated from the routers.
The file defines a small local `OpenAPIRegistry` class plus a `zodToOpenApiSchema`
converter, then makes ~300+ explicit `registry.registerPath({ … })` calls that
mirror the real routes and reuse the Zod request schemas from
`@paperclipai/shared`. `buildOpenApiDocument()` assembles
`{ openapi: "3.0.0", info, servers, components, paths }` and applies
`applyDocumentFixups` (which injects `securitySchemes` and per-operation
`BOARD_SECURITY` / `AUTHENTICATED_SECURITY`). `openApiRoutes()` serves it at
`GET /api/openapi.json`.

> **Invariant:** because these registrations are written by hand, adding or
> changing a route does *not* update the spec automatically — the
> `registerPath` entry must be edited in lockstep. This is the intended
> trade-off (no decorator/reflection machinery), but it means the spec can
> silently drift if edits are skipped.

### Service dependency-injection style — [server/src/services/](../../server/src/services)

Services follow the same factory convention as routes: each is a function like
`goalService(db)` / `heartbeatService(db, { pluginWorkerManager })` that closes
over its dependencies and returns an object of methods. There is no DI
container and no service singletons — a fresh service instance is built wherever
it is needed (usually once per route factory). Shared services are re-exported
from [server/src/services/index.ts](../../server/src/services/index.ts) (again a
barrel, not a registry). This keeps every unit trivially constructable with a
test `Db` and makes the dependency graph explicit at each call site.

### Realtime — [server/src/realtime/live-events-ws.ts](../../server/src/realtime/live-events-ws.ts)

`setupLiveEventsWebSocketServer(server, db, { deploymentMode, resolveSessionFromHeaders })`
attaches a websocket server to the same HTTP server in `index.ts`, reusing the
better-auth header-session resolver for auth parity with the REST layer.

---

## Data flow / request lifecycle

For a typical `POST /api/companies/:companyId/goals`:

1. **`trust proxy`** setting applied at app construction (governs `req.ip`).
2. **Body parsing** — portable-limit JSON for the company-import path, else the
   default 10mb limit; raw bytes captured to `req.rawBody`.
3. **`httpLogger`** — request logging (pino-http); may be silenced for some
   success paths.
4. **`privateHostnameGuard`** — reject disallowed Host headers (when enabled).
5. **`actorMiddleware`** — normalize credentials into `req.actor`.
6. **`/api/auth`** app routes, then the **better-auth** catch-all (only relevant
   for `/api/auth/*`), then top-level `llmRoutes`.
7. **`/api` router**: first `boardMutationGuard()` (CSRF for browser board
   mutations), then the resource route factories in registration order.
8. **Route handler**: `validate(schema)` (Zod) → `assert*Access(req, …)`
   authorization from [authz.ts](../../server/src/routes/authz.ts) → service
   call (`svc.create(...)`) → optional `logActivity(...)` / telemetry →
   `res.status(201).json(...)`.
9. **Unmatched `/api`** → `404 { error: "API route not found" }`.
10. **Non-API paths** → plugin UI static, then (per `uiMode`) static UI with an
    SPA fallback or the Vite dev HTML renderer.
11. **`errorHandler`** (mounted last) converts any thrown `HttpError` /
    `ZodError` / unknown error into a JSON response and, for 5xx, records error
    context + telemetry.

Startup lifecycle (process level) is described under
[Entry points → Startup](#startup--migration-gating--db-safety); the
scheduler/recovery loops it launches live in the
[server/src/services/](../../server/src/services) layer (heartbeat, routine
scheduler, `reconcile*OnStartup` helpers).

---

## Contracts & cross-layer coupling

- **`req.actor` is the single auth contract.** Declared in
  [server/src/types/express.d.ts](../../server/src/types/express.d.ts), produced
  by [auth.ts](../../server/src/middleware/auth.ts), consumed by
  [authz.ts](../../server/src/routes/authz.ts),
  [board-mutation-guard.ts](../../server/src/middleware/board-mutation-guard.ts),
  and many handlers. Any change to its shape ripples across all three.
- **`Db` injection.** The whole subsystem is parameterized by a single
  `@paperclipai/db` handle; nothing reaches for a global connection.
- **Shared Zod schemas.** Request validation
  ([validate.ts](../../server/src/middleware/validate.ts)) and the OpenAPI spec
  ([openapi.ts](../../server/src/routes/openapi.ts)) both consume schemas from
  `@paperclipai/shared`, keeping the wire contract centralized.
- **Deployment mode / exposure.** `DeploymentMode` and `DeploymentExposure`
  (from `@paperclipai/shared`, resolved in
  [server/src/config.ts](../../server/src/config.ts)) gate auth resolution,
  the private-hostname guard, health-detail exposure, and the cloud DB
  contract. See [DEPLOYMENT-MODES.md](../DEPLOYMENT-MODES.md).
- **Plugin host.** `createApp` constructs the plugin worker manager, job store,
  scheduler, lifecycle manager, tool dispatcher, event bus, and host services,
  and mounts [pluginRoutes](../../server/src/routes/plugins.ts). Plugin
  internals are out of scope here — see [plugins](./plugins.md).
- **Storage.** Routes that handle uploads receive an injected `StorageService`
  (`createStorageServiceFromConfig`) rather than importing a concrete backend.

---

## Extension points

- **Add a REST resource:** create `server/src/routes/<name>.ts` exporting a
  `xRoutes(db, …deps)` factory, build a `Router`, use `validate(...)` +
  `assert*Access(...)` in handlers, delegate to a `xService(db)` service, and
  mount it on the `api` router in
  [app.ts](../../server/src/app.ts). Optionally add the re-export to
  [routes/index.ts](../../server/src/routes/index.ts) (barrel only) and, if the
  route is public API, add matching `registry.registerPath(...)` entries in
  [openapi.ts](../../server/src/routes/openapi.ts).
- **Add middleware:** implement in `server/src/middleware/`, export from
  [middleware/index.ts](../../server/src/middleware/index.ts) if broadly used,
  and insert it at the correct point in the `app.ts` pipeline (order matters —
  before `actorMiddleware` for pre-auth concerns, inside the `api` router for
  API-scoped concerns).
- **Add a service:** `server/src/services/<name>.ts` exporting an
  `xService(db, deps?)` factory; re-export from
  [services/index.ts](../../server/src/services/index.ts).
- **New error class:** add a factory to
  [errors.ts](../../server/src/errors.ts) returning an `HttpError`; the handler
  already renders any `HttpError`.

---

## Testing

- **Runner:** [vitest](../../server/vitest.config.ts) with
  `pool: "forks"`, `isolate: true`, `maxWorkers: 1` / `minWorkers: 1`,
  `maxConcurrency: 1`, and non-concurrent sequencing. This deliberately runs
  tests **serially in isolated forked processes** — most server tests touch a
  real (embedded) Postgres and shared module state, so parallelism would cause
  cross-test interference.
- **HTTP tests use [supertest](../../server/src/__tests__/setup-supertest.ts).**
  The `setupFiles` entry monkey-patches supertest's `serverAddress` to always
  bind on an explicit loopback TCP port (`[::1]` / `127.0.0.1`) instead of
  `0.0.0.0`/`::`, so tests stay on the loopback interface and work under the
  private-hostname guard. Tests typically call `createApp(testDb, opts)` and
  drive it with `request(app)`.
- **Colocated unit tests** live next to their subject (e.g.
  [cloud-tenant-actor.test.ts](../../server/src/middleware/cloud-tenant-actor.test.ts),
  [execution-allowlist.test.ts](../../server/src/services/execution-allowlist.test.ts)).
- **Scripts** ([server/package.json](../../server/package.json)):
  `dev` (`tsx src/index.ts`), `dev:watch` (auto-apply migrations,
  non-interactive prompt, via [scripts/dev-watch.ts](../../server/scripts/dev-watch.ts)),
  `build` (`tsc` + copy `onboarding-assets`), `start` (`node dist/index.js`),
  `typecheck`, `clean`, and `prepare:ui-dist` / `prepack` / `postpack` for
  bundling the built UI into the published package. Tests run via the
  workspace-level test command against this `vitest.config.ts`.

---

## Gotchas / invariants

- **Middleware order is a security boundary.** `boardMutationGuard` sits at the
  head of the `/api` router (after `actorMiddleware`, which it depends on);
  `privateHostnameGuard` and body limits run before auth; `errorHandler` must
  be last. Reordering can silently disable CSRF or hostname protection.
- **`trust proxy` defaults to trusting nothing.** Behind a real LB you *must*
  set `TRUST_PROXY`, or `req.ip` and forwarded headers will be the proxy's, not
  the client's. Setting `TRUST_PROXY=true` behind an untrusted LB is a spoofing
  risk (documented inline in
  [trust-proxy.ts](../../server/src/middleware/trust-proxy.ts)).
- **The server refuses to start against a stale schema** unless auto-apply is
  enabled or the operator confirms — a safety feature, not a bug. CI/containers
  set `PAPERCLIP_MIGRATION_AUTO_APPLY=true`.
- **`authenticated` + `public` demands external Postgres.** Embedded postgres is
  rejected in that combination (`assertCloudDatabaseContract`).
- **The OpenAPI spec is hand-maintained** — route changes require matching
  edits in [openapi.ts](../../server/src/routes/openapi.ts) or the spec drifts.
- **`routes/index.ts` is a re-export barrel, not the mount list.** The
  authoritative mounting lives in
  [app.ts](../../server/src/app.ts); the barrel is incomplete and importing from
  it is not what wires the app.
- **Bundled kubernetes plugin auto-install is fail-safe.** In `app.ts`,
  `ensureBundledKubernetesPlugin()` swallows all errors — a degraded boot (no
  k8s provider) is preferred over a crash loop.
- **Embedded Postgres is stopped on shutdown only if this process started it**
  (`embeddedPostgresStartedByThisProcess`), so reusing an externally-managed
  cluster does not accidentally kill it.
- **Local-trusted mode grants an implicit admin (`local-board`).** Every request
  is an instance admin; do not run `local_trusted` on a non-loopback bind — the
  startup guard enforces this.

---

## Key files

- [server/src/index.ts](../../server/src/index.ts) — process bootstrap: config,
  migrations, DB, auth, schedulers, listen, shutdown.
- [server/src/app.ts](../../server/src/app.ts) — `createApp`; middleware order,
  `/api` router, auth mounting, UI serving, plugin host wiring.
- [server/src/errors.ts](../../server/src/errors.ts) — `HttpError` + factory
  helpers.
- [server/src/middleware/auth.ts](../../server/src/middleware/auth.ts) — actor
  normalization across board/agent/cloud/session credentials.
- [server/src/middleware/error-handler.ts](../../server/src/middleware/error-handler.ts)
  — terminal error translation + telemetry.
- [server/src/middleware/board-mutation-guard.ts](../../server/src/middleware/board-mutation-guard.ts)
  — CSRF-style guard for browser board mutations.
- [server/src/middleware/private-hostname-guard.ts](../../server/src/middleware/private-hostname-guard.ts)
  — Host-header allow-listing.
- [server/src/middleware/trust-proxy.ts](../../server/src/middleware/trust-proxy.ts)
  — `TRUST_PROXY` parsing/application.
- [server/src/middleware/validate.ts](../../server/src/middleware/validate.ts) —
  Zod body validation.
- [server/src/auth/better-auth.ts](../../server/src/auth/better-auth.ts) —
  better-auth instance, handler, session resolvers.
- [server/src/routes/auth.ts](../../server/src/routes/auth.ts) — `/api/auth`
  session/profile routes over `req.actor`.
- [server/src/routes/authz.ts](../../server/src/routes/authz.ts) — in-handler
  authorization helpers.
- [server/src/routes/openapi.ts](../../server/src/routes/openapi.ts) —
  hand-maintained OpenAPI spec + `/api/openapi.json`.
- [server/src/routes/index.ts](../../server/src/routes/index.ts) — route factory
  re-export barrel (not the mount list).
- [server/src/routes/health.ts](../../server/src/routes/health.ts) — health /
  bootstrap-status probe with actor-scoped detail.
- [server/src/routes/goals.ts](../../server/src/routes/goals.ts) —
  representative route-factory + service-DI example.
- [server/src/types/express.d.ts](../../server/src/types/express.d.ts) —
  `req.actor` type contract.
- [server/src/http/body-limits.ts](../../server/src/http/body-limits.ts) — JSON
  body-size limits.
- [server/vitest.config.ts](../../server/vitest.config.ts) &
  [server/src/__tests__/setup-supertest.ts](../../server/src/__tests__/setup-supertest.ts)
  — isolated-forks test config + supertest loopback patch.
- [server/package.json](../../server/package.json) — scripts, deps, package
  exports.
