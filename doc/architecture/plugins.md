# Architecture — Plugin system

Internal architecture reference for the `packages/plugins` subsystem: the plugin
authoring SDK, the out-of-process worker protocol, the manifest schema and
capability model, plugin categories, sandbox-provider environment drivers, and
the trust/isolation expectations of the current runtime.

This document describes the **plugin package side** of the system. The
**host side** (worker spawning, capability enforcement, job scheduling,
environment driver dispatch) lives in `server/src/services/plugin-*` and is
cross-referenced here rather than re-documented; see
[server/src/services/plugin-loader.ts](../../server/src/services/plugin-loader.ts)
and the surrounding services for orchestration detail.

## Purpose / Overview

A Paperclip plugin is an npm package (or local-path checkout) that exports two
things the host cares about:

1. a **manifest** — a `PaperclipPluginManifestV1` object declaring identity,
   categories, capabilities, entrypoints, and every extension it contributes
   (jobs, webhooks, tools, UI slots, API routes, environment drivers, managed
   agents/projects/routines/skills, a database namespace, and external-object
   providers), and
2. a **worker entrypoint** — a module that calls `definePlugin({...})` and then
   `runWorker(plugin, import.meta.url)` so the host can spawn it as a child
   process and drive it over JSON-RPC.

The plugin never links against host internals. Instead the SDK gives the worker
a `PluginContext` (`ctx`) whose client methods serialize into JSON-RPC calls to
the host over stdio. The host answers those calls through capability-gated
handlers. This process boundary is the core architectural seam of the system.

Everything a plugin author needs comes from a single dependency,
`@paperclipai/plugin-sdk`, which re-exports the manifest and constant types from
`@paperclipai/shared` and re-exports `zod` so authors can write config and tool
parameter schemas without extra dependencies
([sdk/src/index.ts](../../packages/plugins/sdk/src/index.ts)).

## Directory layout

| Path | Contents |
|------|----------|
| [`packages/plugins/sdk`](../../packages/plugins/sdk) | The `@paperclipai/plugin-sdk` package: `definePlugin`, worker RPC host, `PluginContext` types, JSON-RPC protocol, host-client factory, bundler presets, dev server, testing harness. |
| [`packages/plugins/create-paperclip-plugin`](../../packages/plugins/create-paperclip-plugin) | Scaffolding CLI (`create-paperclip-plugin`) that generates a typed manifest, worker, UI widget, tests, and bundler config. |
| [`packages/plugins/examples`](../../packages/plugins/examples) | First-party reference plugins used for onboarding and regression: hello-world, file-browser, kitchen-sink, authoring-smoke, orchestration-smoke. |
| [`packages/plugins/plugin-llm-wiki`](../../packages/plugins/plugin-llm-wiki), [`plugin-workspace-diff`](../../packages/plugins/plugin-workspace-diff) | First-party product-shaped plugins (managed agents/skills/routines + DB namespace; workspace Changes tab). |
| [`packages/plugins/paperclip-plugin-fake-sandbox`](../../packages/plugins/paperclip-plugin-fake-sandbox) | Deterministic sandbox provider used to exercise the environment-driver lifecycle without external infrastructure. |
| [`packages/plugins/sandbox-providers`](../../packages/plugins/sandbox-providers) | Real sandbox-provider plugins (`kubernetes`, `cloudflare`, `daytona`, `e2b`, `exe-dev`, `modal`, `novita`). **Excluded from the root workspace / lockfile** (see [Sandbox providers](#sandbox-providers-environment-drivers)). |

## Entry points

**Worker entrypoint (author-facing):** the plugin's `dist/worker.js` calls
`definePlugin()` then `runWorker(plugin, import.meta.url)`.
`definePlugin` simply freezes and wraps the definition
([sdk/src/define-plugin.ts](../../packages/plugins/sdk/src/define-plugin.ts));
`runWorker` starts the RPC host only when the module is the actual process
entrypoint, so importing it in tests does not spin up stdio handling
([sdk/src/worker-rpc-host.ts](../../packages/plugins/sdk/src/worker-rpc-host.ts),
`runWorker` / `isWorkerEntrypoint`).

**SDK package surface** ([sdk/src/index.ts](../../packages/plugins/sdk/src/index.ts)):

| Import specifier | Purpose |
|------------------|---------|
| `@paperclipai/plugin-sdk` | Worker entry: `definePlugin`, `runWorker`, `PluginContext` types, protocol helpers, re-exported manifest/constant types, `z`. |
| `@paperclipai/plugin-sdk/ui` | React hooks (`usePluginData`, `usePluginAction`, `usePluginStream`, `useHostContext`, `useHostNavigation`) and slot prop types. |
| `@paperclipai/plugin-sdk/testing` | `createTestHarness` in-memory host harness. |
| `@paperclipai/plugin-sdk/bundlers` | `createPluginBundlerPresets` (esbuild + rollup worker/manifest/ui builds). |
| `@paperclipai/plugin-sdk/dev-server` | `startPluginDevServer` + `getUiBuildSnapshot` (static UI + SSE reload). |

The full author-facing package documentation lives in
[packages/plugins/sdk/README.md](../../packages/plugins/sdk/README.md), and the
normative spec/guide in
[doc/plugins/PLUGIN_SPEC.md](../../doc/plugins/PLUGIN_SPEC.md) and
[doc/plugins/PLUGIN_AUTHORING_GUIDE.md](../../doc/plugins/PLUGIN_AUTHORING_GUIDE.md).

## Key modules & responsibilities

### `definePlugin` and lifecycle hooks

`definePlugin(definition)` returns a sealed `PaperclipPlugin`. The only required
hook is `setup(ctx)`, called once after `initialize`. All other hooks are
optional and the host applies default behavior when they are absent
([sdk/src/define-plugin.ts](../../packages/plugins/sdk/src/define-plugin.ts)):

| Hook | Trigger / default |
|------|-------------------|
| `setup(ctx)` | **Required.** Register events, jobs, data/action/tool handlers, launchers. Registration must complete synchronously within `setup`. |
| `onHealth()` | Host polls for the health dashboard; default reports `ok` while the worker is alive. |
| `onConfigChanged(newConfig)` | Apply new operator config without restart; default restarts the worker. |
| `onShutdown()` | Cleanup before exit (bounded window, then SIGTERM → SIGKILL). |
| `onValidateConfig(config)` | Powers the settings-form / Test Connection button. |
| `onWebhook(input)` | Handle `POST /api/plugins/:pluginId/webhooks/:endpointKey`; returns 501 if webhooks are declared but unimplemented. |
| `onApiRequest(input)` | Handle manifest-declared scoped routes under `/api/plugins/:pluginId/api/*` after host auth/capability/checkout enforcement. |
| `onDetectExternalObjects` / `onResolveExternalObject` / `onRefreshExternalObjects` | External object reference providers. |
| `onEnvironment*` (validateConfig, probe, acquire/resume/release/destroyLease, realizeWorkspace, execute) | Sandbox-provider / environment-driver lifecycle. |

At `initialize` the worker reports which optional hooks it implements back to the
host in `supportedMethods`, so the host only routes methods the plugin actually
handles ([worker-rpc-host.ts `handleInitialize`](../../packages/plugins/sdk/src/worker-rpc-host.ts)).

### Worker RPC host (`startWorkerRpcHost`)

[sdk/src/worker-rpc-host.ts](../../packages/plugins/sdk/src/worker-rpc-host.ts)
is the worker-side counterpart to the host's `PluginWorkerManager`. It:

- reads newline-delimited JSON-RPC 2.0 messages from **stdin** and writes
  responses/requests on **stdout** (`MESSAGE_DELIMITER` is `"\n"`);
- dispatches host→worker requests (`initialize`, `health`, `shutdown`,
  `validateConfig`, `configChanged`, `onEvent`, `runJob`, `handleWebhook`,
  `handleApiRequest`, `getData`, `performAction`, `executeTool`, the
  `detect/resolve/refreshExternalObjects` set, and the `environment*` set) via
  `dispatchMethod`;
- **builds the `PluginContext`** (`buildContext`): every `ctx.*` client method is
  a thin wrapper that calls `callHost(method, params)` and awaits the response,
  or `notifyHost` for fire-and-forget notifications (logging, streams);
- tracks pending worker→host calls with per-call timeouts (default 30 s) and an
  `AsyncLocalStorage` invocation context so worker→host calls carry the active
  `paperclipInvocationId` for company-scope enforcement;
- keeps handler registries populated during `setup`: `eventHandlers`,
  `jobHandlers`, `launcherRegistrations`, `dataHandlers`, `actionHandlers`,
  `toolHandlers`, plus `sessionEventCallbacks` for streaming agent sessions.

Event dispatch (`handleOnEvent`) matches exact event names plus two wildcard
forms (`plugin.*` and `<prefix>.*`) and applies the optional server-side
`EventFilter` again worker-side; one failing handler is logged and does not stop
the others.

### JSON-RPC protocol

[sdk/src/protocol.ts](../../packages/plugins/sdk/src/protocol.ts) defines the
wire contract shared by both sides: message constructors/guards, the typed
`HostToWorkerMethods` / `WorkerToHostMethods` maps, invocation-scope metadata,
the environment-driver param/result types, and the error-code table
`PLUGIN_RPC_ERROR_CODES`:

| Code | Meaning |
|------|---------|
| `-32000` `WORKER_UNAVAILABLE` | Worker not running/reachable. |
| `-32001` `CAPABILITY_DENIED` | Missing manifest capability for the method. |
| `-32002` `WORKER_ERROR` | Unhandled worker error. |
| `-32003` `TIMEOUT` | Worker response timed out. |
| `-32004` `METHOD_NOT_IMPLEMENTED` | Optional method not implemented. |
| `-32005` `INVOCATION_SCOPE_DENIED` | Worker→host call escaped the invocation's company scope. |
| `-32099` `UNKNOWN` | Catch-all. |

### Host-client factory (capability & scope gate)

[sdk/src/host-client-factory.ts](../../packages/plugins/sdk/src/host-client-factory.ts)
runs on the **host** side (the server imports it). `createHostClientHandlers`
produces a complete handler map for every `WorkerToHostMethods` entry, wrapping
each in `gated(...)` which:

1. **Enforces capability** via `METHOD_CAPABILITY_MAP` — e.g. `state.get` →
   `plugin.state.read`, `http.fetch` → `http.outbound`, `issues.create` →
   `issues.create`; methods like `config.get`, `log`, and `entities.*` require
   none. A missing capability throws `CapabilityDeniedError` (code
   `CAPABILITY_DENIED`).
2. **Enforces company scope** — `requireInvocationCompanyScope` compares the
   `companyId` (or `scopeKind:"company"` scopeId, or an `events.subscribe`
   filter company) requested by the worker against the active invocation's
   authorized company. A mismatch throws `InvocationScopeDeniedError`
   (`INVOCATION_SCOPE_DENIED`). `companies.list` results are additionally
   filtered to the authorized company.

The host wires these handlers to concrete `HostServices` implemented in
[server/src/services/plugin-host-services.ts](../../server/src/services/plugin-host-services.ts).
`getRequiredCapability(method)` is exported for inspection.

### `PluginContext` surface

[sdk/src/types.ts](../../packages/plugins/sdk/src/types.ts) defines every `ctx`
client. Grouped by concern:

- **Config / secrets:** `ctx.config.get()`, `ctx.secrets.resolve(ref)` (resolve
  at call time, never cache).
- **State & entities:** `ctx.state.get/set/delete(scopeKey)` — isolated per
  plugin, partitioned by the five-part key
  `(pluginId, scopeKind, scopeId, namespace, stateKey)` with scope kinds
  `instance | company | project | project_workspace | agent | issue | goal | run`;
  `ctx.entities.upsert/list` for plugin-owned external-entity rows.
- **Events / jobs / streams:** `ctx.events.on/emit`, `ctx.jobs.register`,
  `ctx.streams.open/emit/close` (SSE fan-out to plugin UI).
- **Domain reads/writes:** `ctx.companies`, `ctx.projects` (+ `managed`),
  `ctx.executionWorkspaces`, `ctx.issues` (create/update, comments,
  interactions, `documents`, `relations`, `summaries`, checkout/wakeup),
  `ctx.agents` (+ `managed`, + `sessions` two-way chat), `ctx.goals`,
  `ctx.access`, `ctx.authorization`, `ctx.routines.managed`,
  `ctx.skills.managed`.
- **Restricted DB:** `ctx.db.namespace`, `ctx.db.query` (SELECT), `ctx.db.execute`
  (INSERT/UPDATE/DELETE) against the plugin's host-owned PostgreSQL schema.
- **Trusted local folders:** `ctx.localFolders.configure/status/list/readText/
  writeTextAtomic/deleteFile` — path-safe reads/atomic writes under an
  operator-configured, company-scoped root.
- **UI bridge:** `ctx.data.register` (backs `usePluginData`), `ctx.actions.register`
  (backs `usePluginAction`), `ctx.launchers.register`.
- **Observability:** `ctx.logger`, `ctx.metrics.write`, `ctx.telemetry.track`,
  `ctx.activity.log`.

### Manifest schema

The manifest type `PaperclipPluginManifestV1` and every nested declaration live
in [packages/shared/src/types/plugin.ts](../../packages/shared/src/types/plugin.ts)
and are validated by the Zod schema in
[packages/shared/src/validators/plugin.ts](../../packages/shared/src/validators/plugin.ts).
Required top-level fields: `id`, `apiVersion` (must be `1`), `version`,
`displayName`, `description`, `author`, `categories`, `capabilities`, and
`entrypoints.worker`. `entrypoints.ui` is required when `ui.slots` **or**
`ui.launchers` are declared (enforced by a `superRefine` on the manifest
schema, which also requires the matching capability for each declared feature —
e.g. `tools[]` ⇒ `agent.tools.register`, `environmentDrivers[]` ⇒
`environment.drivers.register`). Optional declaration arrays cover `jobs`,
`webhooks`, `tools`, `database`,
`apiRoutes`, `environmentDrivers`, managed `agents` / `projects` / `routines` /
`skills`, `localFolders`, `objectReferences`, `launchers`, and `ui`.

The host persists an installed plugin as a `plugins` row (`PluginRecord`,
including `pluginKey`, `packageName`, `manifestJson`, `status`, `installOrder`,
`packagePath`) plus companion tables `plugin_config`, `plugin_company_settings`,
`plugin_state`, `plugin_entities`, `plugin_jobs`, `plugin_job_runs`,
`plugin_webhook_deliveries`, and the DB-namespace/migration bookkeeping rows —
all typed in the same file.

## Plugin categories & capabilities

**Categories** (`PLUGIN_CATEGORIES` in
[packages/shared/src/constants.ts](../../packages/shared/src/constants.ts)):
`connector`, `workspace`, `automation`, `ui`. A plugin declares one or more.
Observed usage: `workspace-diff` uses `["workspace","ui"]`; every
sandbox-provider uses `["automation"]`. (The scaffolder additionally accepts an
`environment` template/category alias — see
[create-paperclip-plugin/src/index.ts](../../packages/plugins/create-paperclip-plugin/src/index.ts).)

**Statuses** (`PLUGIN_STATUSES`): `installed → ready → disabled | error |
upgrade_pending → uninstalled`, enforced by the host lifecycle state machine
([server/src/services/plugin-lifecycle.ts](../../server/src/services/plugin-lifecycle.ts)).

**Capabilities** (`PLUGIN_CAPABILITIES`) are the named permissions the host
enforces at every worker→host call. They are grouped into Data Read, Data Write,
Plugin State, Runtime/Integration, Agent Tools, and UI. A representative subset:

- Read: `companies.read`, `projects.read`, `project.workspaces.read`,
  `execution.workspaces.read`, `issues.read`, `issue.documents.read`,
  `issues.orchestration.read`, `database.namespace.read`.
- Write: `issues.create`, `issues.update`, `issue.relations.write`,
  `issues.checkout`, `issues.wakeup`, `issue.interactions.create`,
  `agents.invoke`, `agents.managed`, `routines.managed`, `skills.managed`,
  `database.namespace.write`, `external.objects.*`.
- State: `plugin.state.read`, `plugin.state.write`.
- Runtime: `events.subscribe`, `events.emit`, `jobs.schedule`,
  `webhooks.receive`, `api.routes.register`, `http.outbound`, `secrets.read-ref`,
  `environment.drivers.register`, `local.folders`.
- Agent tools: `agent.tools.register`. UI:
  `ui.page.register`, `ui.sidebar.register`, `ui.detailTab.register`, etc.

The authoritative capability→method mapping is `METHOD_CAPABILITY_MAP` in
[host-client-factory.ts](../../packages/plugins/sdk/src/host-client-factory.ts).

**UI slot types** (`PLUGIN_UI_SLOT_TYPES`) include `page`, `detailTab`,
`taskDetailView`, `dashboardWidget`, `sidebar`, `routeSidebar`, `sidebarPanel`,
`projectSidebarItem`, `globalToolbarButton`, `toolbarButton`, `contextMenuItem`,
`commentAnnotation`, `commentContextMenuItem`, `settingsPage`,
`companySettingsPage`.

## Data flow / execution lifecycle

### Standard host ⇄ worker exchange

```
Host (PluginWorkerManager)                 Worker (startWorkerRpcHost)
  │  request: initialize(manifest,config) →│  → plugin.setup(ctx); returns supportedMethods
  │  notification: onEvent(event) ────────→│  → dispatch to matching event handlers
  │                                        │← request: state.get(scopeKey)   (ctx.state.get)
  │  → gated: capability + scope check     │
  │  response: value ─────────────────────→│
  │  request: runJob / executeTool / … ───→│  → registered handler
  │  request: shutdown ───────────────────→│  → plugin.onShutdown(); process exits
```

Worker→host calls carry the active invocation's `paperclipInvocationId`
(threaded through `AsyncLocalStorage`) so the host can reject cross-company
access. UI `usePluginData` / `usePluginAction` calls travel host→worker as
`getData` / `performAction`; `performAction` handlers additionally receive an
immutable, host-supplied actor context
(`handlePerformAction` / `actionContextFromParams` in
[worker-rpc-host.ts](../../packages/plugins/sdk/src/worker-rpc-host.ts)).

### Real-time streaming

The worker calls `ctx.streams.open/emit/close`, which send fire-and-forget
notifications. The host publishes them to an in-memory `PluginStreamBus`
([server/src/services/plugin-stream-bus.ts](../../server/src/services/plugin-stream-bus.ts))
keyed by `pluginId:channel:companyId`; the SSE endpoint
`GET /api/plugins/:pluginId/bridge/stream/:channel` streams them to
`usePluginStream` in the browser.

### Environment-driver (sandbox) lifecycle

When a plugin declares an `environmentDrivers` entry with the
`environment.drivers.register` capability, the host can route agent-run
environment operations to it. The provider key is `<pluginKey>:<driverKey>`
([server/src/services/plugin-environment-driver.ts](../../server/src/services/plugin-environment-driver.ts)).
Per run, the host calls the worker's `onEnvironment*` hooks in sequence:

```
onEnvironmentValidateConfig → onEnvironmentProbe
   → onEnvironmentAcquireLease  (or onEnvironmentResumeLease)  → PluginEnvironmentLease
   → onEnvironmentRealizeWorkspace → { cwd, metadata }
   → onEnvironmentExecute (repeated)  → { exitCode, signal, timedOut, stdout, stderr }
   → onEnvironmentReleaseLease  (or onEnvironmentDestroyLease)
```

Lease and execute param/result shapes are defined in
[protocol.ts](../../packages/plugins/sdk/src/protocol.ts). A driver declaration's
`kind` is `"environment_driver"` (core `driver: "plugin"` environments) or
`"sandbox_provider"` (core `driver: "sandbox"` whose provider is plugin-backed),
and `supportsReusableLeases` opts a provider into host-retained lease
resume across runs
([packages/shared/src/types/plugin.ts](../../packages/shared/src/types/plugin.ts),
`PluginEnvironmentDriverDeclaration`).

## Sandbox providers (environment drivers)

The [`packages/plugins/sandbox-providers`](../../packages/plugins/sandbox-providers)
tree holds first-party providers — `kubernetes`, `cloudflare`, `daytona`, `e2b`,
`exe-dev`, `modal`, `novita` — each a standalone plugin with `categories:
["automation"]`, a single `environmentDrivers[]` entry of `kind:
"sandbox_provider"`, and the `environment.drivers.register` capability.

**Lockfile / workspace exclusion.** These packages are deliberately **excluded
from the root pnpm workspace** so their heavy third-party dependencies (e.g. the
Kubernetes client) do not churn the root `pnpm-lock.yaml`. See
[pnpm-workspace.yaml](../../pnpm-workspace.yaml):

```yaml
- packages/plugins/*
# Keep sandbox-provider plugins installable as standalone packages without
# forcing root pnpm-lock.yaml churn for their third-party deps.
- "!packages/plugins/sandbox-providers/**"
```

Consequences: they are built, tested, and installed as independent packages
(`pnpm install --ignore-workspace` inside the provider directory), and they are
installed into a running instance via the CLI plugin-install path
([cli/src/commands/client/plugin.ts](../../cli/src/commands/client/plugin.ts)),
not linked through the monorepo. The `plugin-orchestration-smoke-example` fixture
is excluded from the workspace for the same lockfile-hygiene reason.

**Kubernetes provider** ([sandbox-providers/kubernetes](../../packages/plugins/sandbox-providers/kubernetes))
is the richest example and doubles as the reference for the driver lifecycle. It
supports two backends selected by config:

- `sandbox-cr` (default, alpha) — creates a `Sandbox` CR
  (`agents.x-k8s.io/v1alpha1`, from `kubernetes-sigs/agent-sandbox`) whose
  controller provisions a long-lived pod; the host execs individual commands via
  `onEnvironmentExecute` (multi-command adapter-install pattern). The
  `SandboxOrchestrator` interface
  ([src/sandbox-orchestrator.ts](../../packages/plugins/sandbox-providers/kubernetes/src/sandbox-orchestrator.ts))
  is the clean swap point for future backends.
- `job` (stable fallback) — a `batch/v1` Job with a one-shot entrypoint; no
  multi-command exec.

Per company it lazily provisions a tenant namespace with a restricted Pod
Security baseline, ServiceAccount/Role/RoleBinding, ResourceQuota, LimitRange,
and deny-all + explicit-egress NetworkPolicies (or CiliumNetworkPolicy in
`cilium` egress mode). Every agent pod runs non-root, drops all capabilities,
uses a read-only rootFS with explicit `emptyDir` mounts, `seccompProfile:
RuntimeDefault`, and Tini as PID 1; optional `runtimeClassName: kata-fc` adds
Firecracker microVM isolation. Config schema is in
[src/manifest.ts](../../packages/plugins/sandbox-providers/kubernetes/src/manifest.ts);
full rationale and the created-resource inventory are in the provider
[README.md](../../packages/plugins/sandbox-providers/kubernetes/README.md).

The [`paperclip-plugin-fake-sandbox`](../../packages/plugins/paperclip-plugin-fake-sandbox)
package is the deterministic, infrastructure-free provider used to exercise the
same lifecycle in tests — it runs commands in a local temp directory.

## Contracts & cross-layer coupling

The plugin package and the host are coupled through three explicit contracts,
all owned by shared packages so both sides compile against the same source:

- **Wire contract** — [protocol.ts](../../packages/plugins/sdk/src/protocol.ts)
  is imported by both the worker (`worker-rpc-host.ts`) and the host
  (`host-client-factory.ts` and `server/src/services/plugin-*`). It fixes the
  JSON-RPC framing (`MESSAGE_DELIMITER`, message constructors/guards), the
  `HostToWorkerMethods` / `WorkerToHostMethods` type maps, the invocation-scope
  metadata (`paperclipInvocationId`), and `PLUGIN_RPC_ERROR_CODES`. Adding a
  host→worker or worker→host method means editing this file and both sides.
- **Manifest contract** — `PaperclipPluginManifestV1`
  ([types/plugin.ts](../../packages/shared/src/types/plugin.ts)) plus its Zod
  schema ([validators/plugin.ts](../../packages/shared/src/validators/plugin.ts))
  and the enums in [constants.ts](../../packages/shared/src/constants.ts) are the
  shared vocabulary the SDK re-exports and the host validates at install time.
- **Capability contract** — `METHOD_CAPABILITY_MAP` in
  [host-client-factory.ts](../../packages/plugins/sdk/src/host-client-factory.ts)
  maps every `WorkerToHostMethods` entry to the manifest capability the host
  requires before dispatching, and the manifest schema's `superRefine` requires
  a plugin to declare the capabilities its features need. These two must stay in
  sync: a new gated method needs both a `METHOD_CAPABILITY_MAP` entry and (if it
  backs a manifest feature) a `superRefine` check.

On the host side these contracts terminate in concrete services under
`server/src/services/` — `plugin-loader.ts` (install/validate/migrate/register),
`plugin-worker-manager.ts` (spawn/restart/shutdown), `plugin-host-services.ts`
(the `HostServices` implementation `createHostClientHandlers` wraps),
`plugin-database.ts` (namespace SQL validation + migrations),
`plugin-stream-bus.ts`, `plugin-lifecycle.ts`, and `plugin-environment-driver.ts`
(provider-key resolution and sandbox-provider dispatch). The HTTP surface lives
in [server/src/routes/plugins.ts](../../server/src/routes/plugins.ts), which
mounts the `bridge/data`, `bridge/action`, `bridge/stream/:channel`,
`webhooks/:endpointKey`, and scoped `api/*` routes referenced throughout this doc.

## Extension points

A plugin extends Paperclip through manifest declarations plus matching worker
registrations / hooks:

| Extension | Manifest field | Worker side | Gating capability |
|-----------|----------------|-------------|-------------------|
| Scheduled jobs | `jobs[]` (cron `schedule`) | `ctx.jobs.register` | `jobs.schedule` |
| Webhooks | `webhooks[]` | `onWebhook` | `webhooks.receive` |
| Scoped JSON API routes | `apiRoutes[]` | `onApiRequest` | `api.routes.register` |
| Agent tools | `tools[]` | `ctx.tools.register` | `agent.tools.register` |
| UI slots / launchers | `ui.slots` / `ui.launchers` | `ctx.data`/`ctx.actions` handlers | `ui.*.register` |
| Restricted DB namespace | `database` | `ctx.db.query/execute` | `database.namespace.*` |
| Environment drivers | `environmentDrivers[]` | `onEnvironment*` | `environment.drivers.register` |
| Managed agents/projects/routines/skills | `agents`/`projects`/`routines`/`skills` | `ctx.<x>.managed.*` | `agents.managed`, `projects.managed`, `routines.managed`, `skills.managed` |
| External object providers | `objectReferences[]` | `onDetect/Resolve/RefreshExternalObjects` | `external.objects.*` |
| Trusted local folders | `localFolders[]` | `ctx.localFolders.*` | `local.folders` |

The scaffolder ([create-paperclip-plugin](../../packages/plugins/create-paperclip-plugin))
generates a starter package (`default`, `connector`, `workspace`, or
`environment` template) with a typed manifest, worker, example UI widget, tests,
and esbuild/rollup config from the SDK bundler presets.

## Testing

- **SDK unit tests** live in
  [packages/plugins/sdk/tests](../../packages/plugins/sdk/tests) — e.g.
  `worker-rpc-host.test.ts`, `host-client-factory.test.ts`,
  `testing-actions.test.ts`.
- **In-memory harness:** `createTestHarness({ manifest })` from
  `@paperclipai/plugin-sdk/testing`
  ([sdk/src/testing.ts](../../packages/plugins/sdk/src/testing.ts)) provides a
  fake `ctx` and event emitter so a plugin's `setup` and handlers can be tested
  without spawning a process. Environment-driver providers additionally use
  `createEnvironmentTestHarness` / `createFakeEnvironmentDriver` plus the
  lifecycle assertion helpers exported from the same module.
- **Provider tests:** the kubernetes provider has extensive unit tests plus a
  gated kind-cluster integration test
  ([test/integration/end-to-end-run.test.ts](../../packages/plugins/sandbox-providers/kubernetes/test/integration/end-to-end-run.test.ts),
  run only when `RUN_K8S_INTEGRATION_TESTS=1`). Because providers are outside the
  workspace, run their tests from the provider directory with `--ignore-workspace`.
- **Example plugins** double as regression fixtures — `plugin-authoring-smoke-example`
  and `plugin-orchestration-smoke-example` exercise the authoring and
  orchestration host surfaces respectively.

## Gotchas / invariants

- **Trust posture (MVP):** plugin **workers and plugin UI are trusted code
  today**. Plugin UI bundles run as same-origin JavaScript inside the main
  Paperclip app and can call ordinary Paperclip HTTP APIs with the board
  session — **manifest capabilities are not a frontend sandbox**. Capabilities
  gate *worker→host* API calls only. Untrusted/marketplace plugins remain future
  work requiring worker sandboxing plus isolated plugin UI
  ([sdk/README.md](../../packages/plugins/sdk/README.md) "Current deployment
  caveats"; the same note appears in
  [PLUGIN_SPEC.md](../../doc/plugins/PLUGIN_SPEC.md)).
- **Company scope is enforced, not advisory:** worker→host calls that reference a
  `companyId` outside the current invocation's authorized company fail with
  `INVOCATION_SCOPE_DENIED`. Global/instance-scoped invocations may omit
  `companyId`.
- **Never persist resolved secrets.** Store only secret *references* in config
  and call `ctx.secrets.resolve` at execution time; the SDK docs are explicit
  that resolved values must not be cached or logged.
- **`setup` registration must be synchronous.** Register events/jobs/tools/data
  directly in `setup`, not inside async callbacks that resolve after `setup`
  returns — the host reads the registries once `setup` resolves.
- **One worker process per plugin.** Crashes are isolated; the host restarts with
  exponential backoff. Shutdown is a bounded drain then SIGTERM/SIGKILL
  ([server/src/services/plugin-worker-manager.ts](../../server/src/services/plugin-worker-manager.ts)).
- **Restricted DB namespace is genuinely restricted:** migrations run before
  worker startup and are checksum-recorded; runtime `ctx.db.query` is SELECT-only
  from the plugin namespace plus manifest-whitelisted `public` core tables, and
  `ctx.db.execute` writes only the plugin namespace.
- **Origin-namespace enforcement:** plugin-created issues default `originKind` to
  `plugin:<pluginKey>`; a plugin may use sub-kinds
  (`plugin:<pluginKey>:feature`) but the host rejects claiming another plugin's
  namespace.
- **`ctx.assets` is not in the supported runtime** in this build — do not depend
  on asset upload/read APIs.
- **Sandbox providers must be built/installed standalone.** They are outside the
  workspace by design; do not add them back to the root importer set or the root
  lockfile will start tracking their dependencies.

## Key files

- [packages/plugins/sdk/src/define-plugin.ts](../../packages/plugins/sdk/src/define-plugin.ts) — `definePlugin`, lifecycle hook definitions.
- [packages/plugins/sdk/src/worker-rpc-host.ts](../../packages/plugins/sdk/src/worker-rpc-host.ts) — worker-side RPC host, `runWorker`, `PluginContext` construction.
- [packages/plugins/sdk/src/host-client-factory.ts](../../packages/plugins/sdk/src/host-client-factory.ts) — capability & company-scope gating, `METHOD_CAPABILITY_MAP`.
- [packages/plugins/sdk/src/protocol.ts](../../packages/plugins/sdk/src/protocol.ts) — JSON-RPC method maps, error codes, environment-driver params/results.
- [packages/plugins/sdk/src/types.ts](../../packages/plugins/sdk/src/types.ts) — full `PluginContext` client interfaces.
- [packages/plugins/sdk/src/index.ts](../../packages/plugins/sdk/src/index.ts) — SDK public surface and constant re-exports.
- [packages/plugins/sdk/README.md](../../packages/plugins/sdk/README.md) — author-facing SDK docs.
- [packages/shared/src/types/plugin.ts](../../packages/shared/src/types/plugin.ts) — `PaperclipPluginManifestV1` and all declaration/record types.
- [packages/shared/src/validators/plugin.ts](../../packages/shared/src/validators/plugin.ts) — manifest Zod schema.
- [packages/shared/src/constants.ts](../../packages/shared/src/constants.ts) — `PLUGIN_CATEGORIES`, `PLUGIN_CAPABILITIES`, `PLUGIN_STATUSES`, `PLUGIN_UI_SLOT_TYPES`.
- [packages/plugins/sandbox-providers/kubernetes/src/manifest.ts](../../packages/plugins/sandbox-providers/kubernetes/src/manifest.ts) & [plugin.ts](../../packages/plugins/sandbox-providers/kubernetes/src/plugin.ts) — reference environment driver.
- [pnpm-workspace.yaml](../../pnpm-workspace.yaml) — sandbox-provider workspace exclusion.
- [server/src/services/plugin-loader.ts](../../server/src/services/plugin-loader.ts), [plugin-worker-manager.ts](../../server/src/services/plugin-worker-manager.ts), [plugin-host-services.ts](../../server/src/services/plugin-host-services.ts), [plugin-lifecycle.ts](../../server/src/services/plugin-lifecycle.ts), [plugin-database.ts](../../server/src/services/plugin-database.ts), [plugin-stream-bus.ts](../../server/src/services/plugin-stream-bus.ts), [plugin-environment-driver.ts](../../server/src/services/plugin-environment-driver.ts) — host-side counterparts.
- [server/src/routes/plugins.ts](../../server/src/routes/plugins.ts) — HTTP surface (install, health, bridge data/action/stream, webhooks, scoped API routes).
- [doc/plugins/PLUGIN_SPEC.md](../../doc/plugins/PLUGIN_SPEC.md), [doc/plugins/PLUGIN_AUTHORING_GUIDE.md](../../doc/plugins/PLUGIN_AUTHORING_GUIDE.md) — normative spec and authoring guide.
