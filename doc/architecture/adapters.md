# Architecture — Agent adapters

> Internal architecture reference for the **adapters** subsystem: the packages
> under [`packages/adapters/`](../../packages/adapters) and the shared runtime
> library [`packages/adapter-utils/`](../../packages/adapter-utils). Grounded in
> the code as of this writing; when in doubt, the cited source is authoritative.

## Purpose / Overview

An **adapter** is the bridge between Paperclip's control plane (the heartbeat /
run machinery in the `server`) and a concrete agent runtime — a CLI like Claude
Code, Codex, Gemini, Cursor, OpenCode, Pi, Grok, ACPX, or a networked service
like the Hermes API gateway, Cursor Cloud, or an OpenClaw gateway. Each adapter
knows how to:

- launch (or call) its runtime for one **run**,
- inject Paperclip context (prompt, workspace, env, skills),
- parse the runtime's output into usage/cost/session/error signals,
- report structured diagnostics for the settings "Test environment" flow, and
- (optionally) render the runtime's stdout into a structured UI transcript.

The whole subsystem is organized around **one interface**,
[`ServerAdapterModule`](../../packages/adapter-utils/src/types.ts), plus a
library of shared helpers in `@paperclipai/adapter-utils` that keep every
adapter's process spawning, env shaping, remote transport, and workspace
round-tripping consistent.

Two kinds of adapters exist, distinguished only by how they get registered:

| | Built-in | External / plugin |
|---|---|---|
| Location | `packages/adapters/<name>/` (in-repo) | separate npm package or local dir |
| Registration | statically imported into three registries | loaded at startup / hot-installed via the plugin store |
| UI parser | static import at build time | fetched from the server and run in a browser sandbox |
| Trust | first-party | must not replace built-in types; parser sandboxed |

The authoritative in-repo contract notes live in
[`packages/adapters/AUTHORING.md`](../../packages/adapters/AUTHORING.md); the
user-facing author guides live under
[`docs/adapters/`](../../docs/adapters/overview.md).

## Entry points

- **The interface**: [`ServerAdapterModule`](../../packages/adapter-utils/src/types.ts)
  — `type`, `execute()`, `testEnvironment()`, plus many optional fields.
- **Shared library barrel**:
  [`packages/adapter-utils/src/index.ts`](../../packages/adapter-utils/src/index.ts)
  re-exports the types and the browser-safe helpers; heavier server helpers are
  reached through subpath exports (`@paperclipai/adapter-utils/server-utils`,
  `/ssh`, `/execution-target`).
- **Server registry**:
  [`server/src/adapters/registry.ts`](../../server/src/adapters/registry.ts) —
  instantiates every built-in `ServerAdapterModule`, registers them by `type`,
  and merges in external plugins.
- **Plugin loader**:
  [`server/src/adapters/plugin-loader.ts`](../../server/src/adapters/plugin-loader.ts)
  — dynamically imports external packages via their `createServerAdapter()`
  factory and extracts their UI parser source.
- **UI registry**:
  [`ui/src/adapters/registry.ts`](../../ui/src/adapters/registry.ts) — the
  browser-side registry of transcript parsers and config-form modules, with
  override handling for external adapters.
- **Set of first-party type keys**:
  [`server/src/adapters/builtin-adapter-types.ts`](../../server/src/adapters/builtin-adapter-types.ts)
  — the reserved names external plugins may override but never permanently
  replace.

## Key modules & responsibilities

### The adapter contract — `ServerAdapterModule`

Defined in
[`packages/adapter-utils/src/types.ts`](../../packages/adapter-utils/src/types.ts).
Only two members are required:

```ts
type: string;                                             // globally unique, snake_case
execute(ctx: AdapterExecutionContext): Promise<AdapterExecutionResult>;
testEnvironment(ctx: AdapterEnvironmentTestContext): Promise<AdapterEnvironmentTestResult>;
```

Everything else is optional and progressively unlocks host behavior:

- **Session continuity** — `sessionCodec` (`deserialize` / `serialize` /
  `getDisplayId`) validates and normalizes persisted session params;
  `sessionManagement` (an `AdapterSessionManagement`) declares resume support
  and native context management so the host knows whether to apply
  threshold-based compaction.
- **Skills** — `listSkills` / `syncSkills` return an `AdapterSkillSnapshot`;
  `requiresMaterializedRuntimeSkills` forces the host to write skill entries to
  disk before the run.
- **Models** — `models` (static), `listModels` (discovered), `modelProfiles` /
  `listModelProfiles` (e.g. the `cheap` profile key), and `refreshModels` (a
  cache-bypass hook so the UI can pull newly released models without a Paperclip
  release).
- **`detectModel()`** — optional; reads the runtime's local config file and
  returns `{ model, provider, source, candidates? }` so the UI can pre-fill the
  configured model. Only Hermes implements it today
  ([`hermes/src/server/detect-model.ts`](../../packages/adapters/hermes/src/server/detect-model.ts)),
  which reads `~/.hermes/config.yaml`; the registry surfaces it through
  `detectAdapterModel()`.
- **`getConfigSchema()`** — returns a declarative `AdapterConfigSchema` (a list
  of `ConfigFieldSchema` fields) so the UI can render adapter-specific form
  fields without shipping React. Used by ACPX
  ([`acpx-local/src/server/config-schema.ts`](../../packages/adapters/acpx-local/src/server/config-schema.ts))
  and Cursor Cloud.
- **`getRuntimeCommandSpec(config)`** — returns
  `{ command, detectCommand?, installCommand? }` describing how to launch and
  provision the runtime command in a fresh remote environment (sandbox).
- **`getQuotaWindows()`** — fetches provider rate-limit/quota windows for the
  quota UI (Claude and Codex implement it).
- **`onHireApproved(payload, adapterConfig)`** — a non-fatal lifecycle hook fired
  when an agent is hired/approved (used by cloud adapters to call back).
- **Capability flags** — `supportsLocalAgentJwt`, `supportsInstructionsBundle`
  (+ `instructionsPathKey`, default `"instructionsFilePath"`), and
  `requiresMaterializedRuntimeSkills`. When unset, the server falls back to
  legacy hardcoded lists for built-in types; external plugins must opt in. These
  are exposed through `GET /api/adapters` alongside a derived `supportsSkills`.

`execute()` receives an
[`AdapterExecutionContext`](../../packages/adapter-utils/src/types.ts): `runId`,
`agent`, `runtime` (prior `sessionParams` / `sessionId`), the agent's `config`,
the run `context` (task, wake reason, workspace hints…), an optional
`executionTarget`, and callbacks — `onLog(stream, chunk)`, `onMeta(meta)`,
`onSpawn(meta)`, `onRuntimeProgress`, and an `authToken`. It returns an
[`AdapterExecutionResult`](../../packages/adapter-utils/src/types.ts): exit
signals (`exitCode`, `signal`, `timedOut`), error classification (`errorCode`,
`errorFamily` of `transient_upstream | model_refusal`, `retryNotBefore`),
`usage`, `costUsd`, `provider` / `biller` / `model` / `billingType`,
`sessionParams` / `sessionDisplayId`, `clearSession`, `runtimeServices`, and an
optional agent-asked `question`.

### `adapter-utils` — the shared runtime library

[`packages/adapter-utils/`](../../packages/adapter-utils) is the dependency every
adapter shares (`@paperclipai/adapter-utils`, `workspace:*`). Its root export is
kept **browser-safe** because the UI imports the types and a few helpers from it;
server-only code lives behind subpath exports. Highlights:

- **[`types.ts`](../../packages/adapter-utils/src/types.ts)** — the full contract
  surface: `ServerAdapterModule`, execution context/result, skills, models,
  config schema, quota, `TranscriptEntry` / `StdoutLineParser`, `CLIAdapterModule`,
  and `CreateConfigValues` (the UI config-form value shape).
- **[`server-utils.ts`](../../packages/adapter-utils/src/server-utils.ts)** — the
  workhorse: `runChildProcess()` (spawn with timeout/grace/streaming and a
  running-process registry), safe config coercion (`asString`, `asNumber`,
  `asBoolean`, `asStringArray`, `parseObject`, `parseJson`), `buildPaperclipEnv()`
  (inject `PAPERCLIP_*` vars), the workspace env shapers
  (`applyPaperclipWorkspaceEnv`, `shapePaperclipWorkspaceEnvForExecution`,
  `rewriteWorkspaceCwdEnvVarsForExecution`, `refreshPaperclipWorkspaceEnvForExecution`),
  prompt rendering (`renderTemplate`, `renderPaperclipWakePrompt`,
  `joinPromptSections`, `DEFAULT_PAPERCLIP_AGENT_PROMPT_TEMPLATE`), skills
  materialization/symlinking, and log-safe env/command builders.
- **[`execution-target.ts`](../../packages/adapter-utils/src/execution-target.ts)**
  — the provider-neutral remote-execution contract. `AdapterExecutionTarget` is
  `local | ssh | sandbox`; helpers such as `readAdapterExecutionTarget`,
  `adapterExecutionTargetIsRemote`, `runAdapterExecutionTargetProcess`,
  `runAdapterExecutionTargetShellCommand`, `prepareAdapterExecutionTargetRuntime`,
  `ensureAdapterExecutionTargetRuntimeCommandInstalled`,
  `startAdapterExecutionTargetPaperclipBridge`, and the session-identity matchers
  let an adapter run the same logic locally, over SSH, or inside a sandbox
  without branching on transport.
- **[`ssh.ts`](../../packages/adapter-utils/src/ssh.ts)** — the SSH transport and
  the **workspace round-trip**: `prepareWorkspaceForSshExecution` git-bundles the
  local cwd to the remote dir before a run;
  `restoreWorkspaceFromSshExecution` syncs remote-side changes (including new
  commits) back into the local cwd after. Both run with **no git remote
  configured**.
- **[`remote-managed-runtime.ts`](../../packages/adapter-utils/src/remote-managed-runtime.ts)**
  — `prepareRemoteManagedRuntime` wraps the prepare/restore pair for adapters
  that want a per-run remote workspace plus an automatic `restoreWorkspace()`
  finally hook.
- **[`session-compaction.ts`](../../packages/adapter-utils/src/session-compaction.ts)**
  — `ADAPTER_SESSION_MANAGEMENT` maps each known type to its resume + context
  policy; adapters with `nativeContextManagement: "confirmed"` (Claude, Codex,
  ACPX) opt out of host-side threshold rotation. `getAdapterSessionManagement()`
  is what the registry passes into each module.
- **[`sandbox-callback-bridge.ts`](../../packages/adapter-utils/src/sandbox-callback-bridge.ts)**
  — a proxy so a sandboxed agent can call back to the Paperclip API through a
  narrow, routed channel instead of holding host credentials.
- **[`runtime-progress.ts`](../../packages/adapter-utils/src/runtime-progress.ts)**,
  **[`log-redaction.ts`](../../packages/adapter-utils/src/log-redaction.ts)**,
  **[`command-redaction.ts`](../../packages/adapter-utils/src/command-redaction.ts)**,
  **[`billing.ts`](../../packages/adapter-utils/src/billing.ts)** — progress
  reporting, home-path/command redaction for logs, and OpenAI-compatible biller
  inference.

### Representative built-in adapters

Each built-in adapter is its own package under `packages/adapters/<name>/`, split
into `src/index.ts` (dependency-free metadata: `type`, `label`, `models`,
`agentConfigurationDoc`), `src/server/` (execute/test/parse/skills/session), and
`src/ui/` + `src/cli/` for transcript rendering.

- **`claude_local`** — the fullest example.
  [`claude-local/src/server/execute.ts`](../../packages/adapters/claude-local/src/server/execute.ts)
  builds env + workspace shaping, resolves whether a stored session can be resumed
  (UUID validity, prompt-bundle key match, cwd/remote-identity match), spawns
  `claude --print - --output-format stream-json …` via
  `runAdapterExecutionTargetProcess`, and classifies results — max-turns,
  refusals (Fable policy refusals exit `0` yet must not look successful),
  transient upstream errors with `retryNotBefore`, poisoned `previous_message_id`
  (drops the session so the next wake starts clean), and login-required. Its
  `finally` block restores the remote workspace. The
  [`server/index.ts`](../../packages/adapters/claude-local/src/server/index.ts)
  exports a `sessionCodec` that normalizes `sessionId` + `cwd` + `promptBundleKey`
  + workspace/repo identity.
- **`codex_local`** — mirrors Claude's structure with its own JSONL parser and
  quota probe; ships a `sessionCodec`
  ([`codex-local/src/server/index.ts`](../../packages/adapters/codex-local/src/server/index.ts)).
- **`hermes` package** — a single package owning **two** stable type keys:
  `hermes_local` (spawns the local Hermes CLI) and `hermes_gateway` (calls an
  already-running Hermes API server over HTTP/SSE). It is the one adapter that
  implements `detectModel()` and ships two UI parsers
  ([`hermes/ui-parser.cjs`](../../packages/adapters/hermes/ui-parser.cjs) and
  `gateway-ui-parser.cjs`). See
  [`hermes/README.md`](../../packages/adapters/hermes/README.md).
- **`acpx_local`, `cursor` / `cursor_cloud`, `gemini_local`, `grok_local`,
  `opencode_local`, `pi_local`, `openclaw_gateway`** — the remaining first-party
  adapters, each registered in
  [`registry.ts`](../../server/src/adapters/registry.ts).
- **`process` and `http`** — the two generic built-ins that live inside the server
  rather than a package:
  [`server/src/adapters/process/index.ts`](../../server/src/adapters/process/index.ts)
  runs an arbitrary command, and
  [`server/src/adapters/http/index.ts`](../../server/src/adapters/http/index.ts)
  posts to a webhook. `process` is also the default fallback when a type is
  unknown (`getServerAdapter` returns it), and neither can be unregistered.

### The three registries

Adapters must be visible in three contexts, wired independently:

1. **Server** —
   [`server/src/adapters/registry.ts`](../../server/src/adapters/registry.ts)
   constructs each `ServerAdapterModule` literal (stitching a package's exported
   `execute` / `testEnvironment` / `sessionCodec` / capability flags together,
   e.g. `claudeLocalAdapter`), registers them into `adaptersByType`, then merges
   external plugins.
2. **UI** —
   [`ui/src/adapters/registry.ts`](../../ui/src/adapters/registry.ts) holds
   transcript parsers + config-form modules and manages dynamic overrides.
3. **CLI** — [`cli/src/adapters/registry.ts`](../../cli/src/adapters/registry.ts)
   wires each `CLIAdapterModule.formatStdoutEvent`; it is consumed by the
   `paperclipai heartbeat run` command
   ([`cli/src/commands/heartbeat-run.ts`](../../cli/src/commands/heartbeat-run.ts)),
   which streams live adapter stdout during a single heartbeat.

## Data flow / execution lifecycle

### Registration at server startup

`registry.ts` calls `registerBuiltInAdapters()` synchronously, then kicks off an
async IIFE (`externalAdaptersReady`) that calls `buildExternalAdapters()` from the
plugin loader. Because plugins load asynchronously, callers await
`waitForExternalAdapters()` before validating an adapter type, so a valid external
type is not rejected during the startup window.

### External plugin loading

[`plugin-loader.ts`](../../server/src/adapters/plugin-loader.ts) reads records
from the adapter-plugin store
([`server/src/services/adapter-plugin-store.ts`](../../server/src/services/adapter-plugin-store.ts)),
resolves each package's entry point from its `exports["."]`, dynamically
`import()`s it, and validates via `validateAdapterModule()` — which requires a
`createServerAdapter()` export that returns a module with a `type`. It also
extracts the UI parser source (`extractUiParserSource`) and caches it in memory,
enforcing the `paperclip.adapterUiParser` contract major version
(`SUPPORTED_PARSER_CONTRACT = "1"`) and rejecting parser paths that escape the
package directory. `reloadExternalAdapter()` busts the ESM cache for dev
iteration without a restart. Hot install/uninstall/reinstall/reload run through
[`server/src/routes/adapters.ts`](../../server/src/routes/adapters.ts)
(`POST /api/adapters/install` for an npm package name or local `file:` path,
`DELETE /api/adapters/:type`, `POST /api/adapters/:type/reinstall`, and
`POST /api/adapters/:type/reload`), which persist a store record and call
`registerServerAdapter()` via `resolveExternalAdapterRegistration()`.

### One run (`execute`)

1. Host resolves the active module (`findActiveServerAdapter(type)` /
   `getServerAdapter`) and calls `execute(ctx)`.
2. The adapter coerces `config`, builds env via `buildPaperclipEnv(agent)` plus
   `PAPERCLIP_*` context vars, and shapes the workspace cwd.
3. It resolves session state from `runtime.sessionParams` (guarded by
   `sessionCodec` and cwd/remote-identity matching) to decide resume vs. fresh.
4. It renders the prompt (`renderTemplate`, wake prompt, task/handoff notes) and
   emits `onMeta` (command, args, redacted env, prompt metrics) before spawning.
5. It spawns through `runAdapterExecutionTargetProcess` (local, SSH, or sandbox
   per `executionTarget`), streaming output through `onLog`.
6. It parses stdout for usage, cost, session id, and error family, and returns an
   `AdapterExecutionResult`.
7. On remote targets, a `finally` block calls `restoreRemoteWorkspace()` so any
   changes/commits made remotely land back in the local cwd.

### Environment test (`testEnvironment`)

The settings "Test environment" flow calls `testEnvironment(ctx)` with an
optional `executionTarget` (so probes run inside the environment the agent will
actually use) and returns `{ status: pass|warn|fail, checks[] }`, where each check
carries `level` (`info | warn | error`), `code`, `message`, and optional
`hint` — `error` blocks execution.

## Contracts & cross-layer coupling

### The AUTHORING contract (no-remote-git)

[`packages/adapters/AUTHORING.md`](../../packages/adapters/AUTHORING.md) pins the
subsystem's hardest invariant: **the local execution-workspace cwd is the only
cross-run persistence boundary; no adapter may depend on a git remote.** Code
carried forward between runs must live in the local cwd by the time `execute()`
returns. Adapters running the agent elsewhere must use the SSH round-trip
(`prepareWorkspaceForSshExecution` → `restoreWorkspaceFromSshExecution`, both with
no remote configured); a failed sync-back is a **run-level error** that records
`workspace_finalize=failed` and gates dependent issue wakes — restore errors must
not be swallowed. The rule is:

- pinned by the `no-remote-git contract` case in
  [`ssh-fixture.test.ts`](../../packages/adapter-utils/src/ssh-fixture.test.ts)
  (a remote-only commit must propagate to the local worktree through the
  round-trip with no remote at any point), and
- statically enforced by
  [`scripts/check-no-git-push.mjs`](../../scripts/check-no-git-push.mjs), which
  fails the `policy` CI job on any unapproved `git push` in adapter/runtime
  source; a legitimate operator-configured push must carry a
  `// paperclip:allow-git-push: <reason>` opt-in comment.

### Trust & contract-sync expectations

- **Reserved type keys.**
  [`BUILTIN_ADAPTER_TYPES`](../../server/src/adapters/builtin-adapter-types.ts)
  lists the first-party names. An external plugin may *override* a built-in type
  (its module replaces the built-in in `adaptersByType`), but the original is
  saved as a `builtinFallback` and can be paused/restored
  (`setOverridePaused` / `unregisterServerAdapter`); `process` and `http` can
  never be removed. This keeps a foreign package from permanently displacing a
  core adapter.
- **Untrusted agent output.** Both author guides state the invariant plainly:
  parse agent output defensively, never `eval()` it, inject secrets via env not
  prompts, and always enforce timeout + grace.
- **Sandboxed UI parser.** An external adapter's parser JS is fetched from
  `GET /api/adapters/:type/ui-parser.js`
  ([routes/adapters.ts](../../server/src/routes/adapters.ts) →
  `getOrExtractUiParserSource`) and executed inside a dedicated Web Worker
  ([`ui/src/adapters/dynamic-loader.ts`](../../ui/src/adapters/dynamic-loader.ts),
  [`sandboxed-parser-worker.ts`](../../ui/src/adapters/sandboxed-parser-worker.ts))
  so it cannot touch the board's same-origin state (cookies, localStorage, DOM,
  authenticated fetch). The parser contract requires zero runtime imports, no DOM
  / Node APIs, no side effects, determinism, and error-tolerance (never throw).
- **Contract-sync between layers.** The `types.ts` contract is shared by the
  server, UI, and CLI; the three registries must be kept in lockstep for
  built-ins (the author guide's "edit 3 registries" step). Session policy is kept
  in sync by threading `getAdapterSessionManagement(type)` from
  `session-compaction.ts` through both the built-in registry literals and
  `resolveExternalAdapterRegistration()` — so init-time and hot-install paths
  resolve `sessionManagement` identically, and an external override of a built-in
  type inherits the built-in's policy when it declares none.

### UI parser / override behavior

Built-in transcript parsers are static modules. Each adapter package exports
package-scoped functions that the UI tree wires into the `UIAdapterModule` slots
`parseStdoutLine` / `createStdoutParser` / `buildAdapterConfig` / `ConfigFields`
(see [`ui/src/adapters/types.ts`](../../ui/src/adapters/types.ts)). Grok, for
example, exports `parseGrokStdoutLine` + `createGrokStdoutParser` from
[`ui/parse-stdout.ts`](../../packages/adapters/grok-local/src/ui/parse-stdout.ts)
and `buildGrokLocalConfig` (`CreateConfigValues` → adapter-config object) from
[`ui/build-config.ts`](../../packages/adapters/grok-local/src/ui/build-config.ts);
[`ui/src/adapters/grok-local/index.ts`](../../ui/src/adapters/grok-local/index.ts)
binds them into a `grokLocalUIAdapter` literal. The UI registry
([`ui/src/adapters/registry.ts`](../../ui/src/adapters/registry.ts)) records which
types are built-in vs. externally overridden, keeps `builtinAdaptersByType` for
restoration, and uses an `overrideGeneration` counter to discard stale dynamic
loads when an override is deactivated mid-flight. A parser may be static
(`parseStdoutLine(line, ts)`) or a stateful factory (`createStdoutParser()`); when
both are present the factory wins. If no parser is available the UI falls back to
the generic process parser (every non-system line becomes `assistant` text).

## Extension points

- **Build an external adapter** (preferred): a package that exports
  `createServerAdapter(): ServerAdapterModule` from its root, per
  [`docs/adapters/external-adapters.md`](../../docs/adapters/external-adapters.md).
  Optional add-ons: `sessionCodec`, `listSkills`/`syncSkills`, `detectModel`,
  `getConfigSchema`, a self-contained `ui-parser.ts`, and a `format-event.ts` CLI
  formatter. Install by npm package name or local path via the UI/API, or by
  editing `~/.paperclip/adapter-plugins.json`.
- **Add a built-in adapter**: create `packages/adapters/<name>/`, then register it
  in the server, UI, and CLI registries
  ([`docs/adapters/creating-an-adapter.md`](../../docs/adapters/creating-an-adapter.md)).
- **Declarative config UI**: implement `getConfigSchema()` (see
  [ACPX](../../packages/adapters/acpx-local/src/server/config-schema.ts)) instead
  of shipping a React config component.
- **Remote/sandbox execution**: route process launches through
  [`execution-target.ts`](../../packages/adapter-utils/src/execution-target.ts)
  and use the workspace round-trip in
  [`ssh.ts`](../../packages/adapter-utils/src/ssh.ts) /
  [`remote-managed-runtime.ts`](../../packages/adapter-utils/src/remote-managed-runtime.ts).

## Testing

- **Contract invariant**:
  [`ssh-fixture.test.ts`](../../packages/adapter-utils/src/ssh-fixture.test.ts)
  pins the no-remote-git round-trip; do not regress it.
- **adapter-utils** ships focused unit tests for command-managed and
  sandbox-managed runtimes, execution targets, git workspace sync,
  runtime progress, billing, and the sandbox callback bridge (the `*.test.ts`
  files next to each module).
- **Per-adapter** tests live under each package's `src/server/` (`parse.test.ts`,
  `execute.test.ts`, `execute.remote.test.ts`, `models.test.ts`, etc.) and
  `src/ui/parse-stdout.test.ts`; packages carry their own `vitest.config.ts`.
- **Registry / plugin / route** behavior is covered by the server's
  `__tests__/adapter-routes*.test.ts`, `plugin-database.test.ts`, and
  `plugin-install-autobuild.test.ts`.
- The static `git push` guard runs in the `policy` CI job via
  [`scripts/check-no-git-push.mjs`](../../scripts/check-no-git-push.mjs).

## Gotchas / invariants

- **Never `git push`, never assume a remote.** The local cwd is the source of
  truth between runs; surface (do not swallow) restore failures.
- **`process` is the ultimate fallback.** `getServerAdapter(type)` returns the
  `process` adapter for any unknown type — a run will not simply error out on a
  missing adapter.
- **External adapters load asynchronously.** Validate types only after
  `waitForExternalAdapters()` resolves.
- **Overrides are pause/restore, not delete.** Overriding a built-in stashes the
  original as a fallback; already-running sessions keep the module reference they
  started with, so pausing only affects new runs.
- **Refusals and clean exits.** A model policy refusal can exit `0` with
  `is_error=false`; adapters must classify it (`errorFamily: "model_refusal"`) so
  the heartbeat does not treat it as success (see claude-local's `execute.ts`).
- **Session poison is a drop, not a retry-forever.** A poisoned
  `previous_message_id` must drop the session (`clearSession: true`) so the next
  wake starts clean.
- **Keep the root `adapter-utils` export browser-safe.** The UI imports it;
  server-only code must stay behind subpath exports (`/server-utils`, `/ssh`,
  `/execution-target`, `/sandbox-callback-bridge`).
- **Capability flags gate UI features.** When a built-in omits a flag the server
  uses legacy hardcoded lists; external adapters default every capability to
  `false` unless they opt in.
- **UI parser contract is major-versioned.** `paperclip.adapterUiParser` must
  match `SUPPORTED_PARSER_CONTRACT` (`"1"`); mismatches fall back to the generic
  parser, and parser paths escaping the package dir are rejected.

## Key files

- [`packages/adapter-utils/src/types.ts`](../../packages/adapter-utils/src/types.ts) — the `ServerAdapterModule` contract and execution/result/skills/model/config types.
- [`packages/adapter-utils/src/server-utils.ts`](../../packages/adapter-utils/src/server-utils.ts) — process spawning, env/prompt/skills helpers.
- [`packages/adapter-utils/src/execution-target.ts`](../../packages/adapter-utils/src/execution-target.ts) — local/ssh/sandbox transport contract.
- [`packages/adapter-utils/src/ssh.ts`](../../packages/adapter-utils/src/ssh.ts) — SSH transport + workspace round-trip.
- [`packages/adapter-utils/src/session-compaction.ts`](../../packages/adapter-utils/src/session-compaction.ts) — per-adapter session/context policy.
- [`packages/adapters/AUTHORING.md`](../../packages/adapters/AUTHORING.md) — the no-remote-git contract for authors.
- [`packages/adapters/claude-local/src/server/execute.ts`](../../packages/adapters/claude-local/src/server/execute.ts) — the reference `execute()` implementation.
- [`server/src/adapters/registry.ts`](../../server/src/adapters/registry.ts) — built-in construction + external merge + override handling.
- [`server/src/adapters/plugin-loader.ts`](../../server/src/adapters/plugin-loader.ts) — external package loading + UI parser extraction.
- [`server/src/adapters/builtin-adapter-types.ts`](../../server/src/adapters/builtin-adapter-types.ts) — reserved first-party type keys.
- [`ui/src/adapters/dynamic-loader.ts`](../../ui/src/adapters/dynamic-loader.ts) / [`sandboxed-parser-worker.ts`](../../ui/src/adapters/sandboxed-parser-worker.ts) — sandboxed UI parser execution.
- [`docs/adapters/creating-an-adapter.md`](../../docs/adapters/creating-an-adapter.md), [`external-adapters.md`](../../docs/adapters/external-adapters.md), [`adapter-ui-parser.md`](../../docs/adapters/adapter-ui-parser.md) — user-facing author guides.

### Related architecture docs

- [Doc map](./index.md) — the subsystem index for this directory.
- [Server](./server.md) — HTTP API composition; hosts the adapter routes
  ([`server/src/routes/adapters.ts`](../../server/src/routes/adapters.ts)) and the
  three server registries.
- [Board UI](./ui.md) — the React SPA that renders transcripts through the UI
  adapter registry and sandboxed parsers.
- [Shared contracts](./shared-contracts.md) — cross-layer types/constants that the
  `types.ts` adapter contract sits alongside.
- [Plugins](./plugins.md) — the general plugin SDK/host, distinct from the adapter
  plugin store but sharing the hot-install/sandbox philosophy.
