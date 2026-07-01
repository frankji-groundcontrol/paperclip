# Architecture — Command-line interface

The `paperclipai` CLI is the operator- and agent-facing command-line surface for a
Paperclip instance. It serves two distinct roles from one binary:

1. **Instance setup & diagnostics** — bootstrap local config, run health checks,
   print deployment env, and start the server (`onboard`, `doctor`, `configure`,
   `env`, `run`, `allowed-hostname`, `db:backup`, `env-lab`, `worktree`).
2. **Control-plane client** — a thin, typed wrapper over the Paperclip server REST
   API for companies, issues, agents, runs, budgets, secrets, skills, plugins, and
   more. These commands talk to a running server over HTTP; they do not touch the
   database directly.

The CLI is published to npm as the `paperclipai` package and is also the entry
point invoked by `pnpm paperclipai` in the monorepo. It is the same code path the
heartbeat/adapter runtime uses to stream a single agent run (`heartbeat run`),
which is why it bundles the CLI-side adapter stream formatters.

Related subsystems: [server](./server.md) (the API this client wraps),
[adapters](./adapters.md) (stream formatters and heartbeat semantics),
[shared contracts](./shared-contracts.md) (config + entity schemas),
[plugins](./plugins.md), and [build/test/release](./build-test-release.md). See
the architecture map in [index.md](./index.md). The full user-facing command
reference lives at [doc/CLI.md](../../doc/CLI.md); this document describes the
internal structure.

## Entry points

- **Binary**: `paperclipai`, mapped to `./dist/index.js` via the `bin` field in
  [cli/package.json](../../cli/package.json).
- **Source entry**: [cli/src/index.ts](../../cli/src/index.ts). It constructs a
  single Commander `program`, sets `name("paperclipai")`, wires the version from
  [cli/src/version.ts](../../cli/src/version.ts) (read from `package.json`), and
  registers every command group. `main()` calls `program.parseAsync()`, prints the
  error `message` on failure, always flushes telemetry, and exits non-zero on
  error.
- **Dev vs. built**: `pnpm dev` runs `tsx src/index.ts`; the shipped artifact is a
  single esbuild bundle with a `#!/usr/bin/env node` banner.

### Global `preAction` hook

Before any command action runs, a `program.hook("preAction", …)` in
[cli/src/index.ts](../../cli/src/index.ts) performs three cross-cutting steps
using the resolved (global) options:

1. `applyDataDirOverride(...)` — honors `--data-dir` / `-d`
   ([cli/src/config/data-dir.ts](../../cli/src/config/data-dir.ts)). It resolves
   the path, sets `PAPERCLIP_HOME`, and — only when a command declares `--config`
   / `--context` options and none is set — derives default `PAPERCLIP_CONFIG` and
   `PAPERCLIP_CONTEXT` paths and `PAPERCLIP_INSTANCE_ID` for that data dir. This
   is how the whole CLI is isolated from `~/.paperclip` for tests and multi-instance
   dev.
2. `loadPaperclipEnvFile(options.config)` — loads the instance `.env` next to the
   config file (see [Config & auth resolution](#config--auth-resolution)).
3. `initTelemetryFromConfigFile(options.config)` — initializes anonymous
   telemetry, honoring `telemetry.enabled` in config
   ([cli/src/telemetry.ts](../../cli/src/telemetry.ts)).

## Command tree & key modules

Commands are registered in [cli/src/index.ts](../../cli/src/index.ts) either
inline (setup/diagnostic commands) or via `register*` functions exported from
per-domain modules under `cli/src/commands/`. Client-facing command modules live
under `cli/src/commands/client/` and share a common option/auth/output helper set.

### Setup, diagnostics, and lifecycle (registered inline)

| Command | Module | Notes |
| --- | --- | --- |
| `onboard` | [commands/onboard.ts](../../cli/src/commands/onboard.ts) | Interactive first-run wizard. `--bind <loopback\|lan\|tailnet>`, `--yes`, `--run`. Quickstart defaults to trusted local loopback. |
| `doctor` | [commands/doctor.ts](../../cli/src/commands/doctor.ts) | Runs the checks in [checks/index.ts](../../cli/src/checks/index.ts). `--repair` (auto-repair), `--yes` (skip repair confirmation prompts). |
| `env` | [commands/env.ts](../../cli/src/commands/env.ts) | Prints deployment env vars. |
| `configure` | [commands/configure.ts](../../cli/src/commands/configure.ts) | `--section <llm\|database\|logging\|server\|storage\|secrets>`. |
| `db:backup` | [commands/db-backup.ts](../../cli/src/commands/db-backup.ts) | One-off DB backup; `--dir`, `--retention-days`, `--filename-prefix`, `--json`. |
| `allowed-hostname <host>` | [commands/allowed-hostname.ts](../../cli/src/commands/allowed-hostname.ts) | Adds a hostname to `server.allowedHostnames`. |
| `run` (+ subcommands) | [commands/run.ts](../../cli/src/commands/run.ts) | Bootstraps (onboard+doctor) and starts the server. Client run subcommands are attached via `registerRunCommands(run)`. |
| `heartbeat run` | [commands/heartbeat-run.ts](../../cli/src/commands/heartbeat-run.ts) | Invokes one agent wakeup and streams live logs. |
| `auth bootstrap-ceo` | [commands/auth-bootstrap-ceo.ts](../../cli/src/commands/auth-bootstrap-ceo.ts) | One-time bootstrap invite URL for the first instance admin. |

The `run` command with no subcommand (`runCommand`) resolves the instance root,
runs onboarding if no config exists, runs `doctor` (auto-repair on by default),
then dynamically imports the server entrypoint. In the monorepo it imports
`server/src/index.ts` via `tsx`; in a published install it dynamically imports the
`@paperclipai/server` package. It calls the module's exported `startServer()` and,
for authenticated + embedded-postgres deployments, generates a bootstrap CEO
invite. This dynamic import is why `@paperclipai/server` is *excluded* from the
CLI bundle (see [Build & packaging](#build--packaging)).

### Local maintenance / fixtures (registered via `register*`)

- `env-lab` — [commands/env-lab.ts](../../cli/src/commands/env-lab.ts):
  `up`, `status`, `down`, `doctor` for deterministic local SSH environment
  fixtures used to test the environment/execution-workspace features.
- `worktree` — [commands/worktree.ts](../../cli/src/commands/worktree.ts):
  worktree-local instance helpers (`worktree:make`, `init`, `env`,
  `worktree:list`, `worktree:merge-history`, `reseed`, `repair`,
  `worktree:cleanup`), backed by
  [commands/worktree-lib.ts](../../cli/src/commands/worktree-lib.ts) and
  [commands/worktree-merge-history-lib.ts](../../cli/src/commands/worktree-merge-history-lib.ts).
- `routines` — [commands/routines.ts](../../cli/src/commands/routines.ts): local
  maintenance (`routines disable-all`), distinct from the API-backed singular
  `routine` group.
- `pipelines` — [commands/pipelines.ts](../../cli/src/commands/pipelines.ts):
  pipeline/case operations and guidance documents.

### Control-plane client groups (registered via `register*`)

Each of these is a `register<Group>Commands(program)` function in
`cli/src/commands/client/`. All are wired in
[cli/src/index.ts](../../cli/src/index.ts). Representative groups and their
modules:

| Group(s) | Module |
| --- | --- |
| `context` (profiles) | [client/context.ts](../../cli/src/commands/client/context.ts) |
| `connect` (interactive board/agent setup) | [client/connect.ts](../../cli/src/commands/client/connect.ts) |
| `company` | [client/company.ts](../../cli/src/commands/client/company.ts) |
| `issue` (+ documents, work-products, interactions, tree-holds, attachments, labels) | [client/issue.ts](../../cli/src/commands/client/issue.ts) |
| `agent`, `token` | [client/agent.ts](../../cli/src/commands/client/agent.ts), [client/token.ts](../../cli/src/commands/client/token.ts) |
| `project`, `goal` | [client/project.ts](../../cli/src/commands/client/project.ts), [client/goal.ts](../../cli/src/commands/client/goal.ts) |
| `approval`, `activity`, `dashboard` | [client/approval.ts](../../cli/src/commands/client/approval.ts), [client/activity.ts](../../cli/src/commands/client/activity.ts), [client/dashboard.ts](../../cli/src/commands/client/dashboard.ts) |
| `run` (heartbeat run inspection/control) | [client/run.ts](../../cli/src/commands/client/run.ts) |
| `agent-prompt`, `agent prompt`, `board prompt` | [client/prompt.ts](../../cli/src/commands/client/prompt.ts) |
| `cost`, `finance`, `budget` | [client/cost.ts](../../cli/src/commands/client/cost.ts) |
| `workspace`, `environment`, `project-workspace`, `org`, `agent-config` | [client/workspace.ts](../../cli/src/commands/client/workspace.ts) |
| `whoami`, `openapi`, `profile`, `invite`, `join`, `member`, `admin`, `instance`, `sidebar`, `inbox`, `board-claim`, `available-skill`, `llm` | [client/access.ts](../../cli/src/commands/client/access.ts) |
| `routine` (API), `plugin` | [client/routine-api.ts](../../cli/src/commands/client/routine-api.ts), [client/plugin.ts](../../cli/src/commands/client/plugin.ts) |
| `adapter`, `asset`, `skill` | [client/adapter.ts](../../cli/src/commands/client/adapter.ts), [client/asset.ts](../../cli/src/commands/client/asset.ts), [client/skill.ts](../../cli/src/commands/client/skill.ts) |
| `skills` (catalog/library/attach), `teams` | [client/skills.ts](../../cli/src/commands/client/skills.ts), [client/teams.ts](../../cli/src/commands/client/teams.ts) |
| `feedback`, `secrets`, `cloud` | [client/feedback.ts](../../cli/src/commands/client/feedback.ts), [client/secrets.ts](../../cli/src/commands/client/secrets.ts), [client/cloud.ts](../../cli/src/commands/client/cloud.ts) |
| client auth (`auth whoami`, CLI-auth challenge lifecycle) | [client/auth.ts](../../cli/src/commands/client/auth.ts) |

Shared conventions across client commands come from
[client/common.ts](../../cli/src/commands/client/common.ts):

- `addCommonClientOptions(command, { includeCompany })` adds the standard flags:
  `-c/--config`, `-d/--data-dir`, `--context`, `--profile`, `--api-base`,
  `--api-key`, `--run-id`, `--json`, and optionally `-C/--company-id`.
- `resolveCommandContext(options, { requireCompany })` builds a
  `PaperclipApiClient` plus the resolved company id, profile, and JSON flag.
- `apiPath` is a tagged-template helper that URL-encodes path segments and
  refuses empty segments.
- `printOutput` / `formatInlineRecord` render either raw JSON (`--json`) or a
  compact human table; `handleCommandError` maps `ApiRequestError` to a red
  `API error <status>: <message>` and exits 1.
- Commands that wrap broad server schemas take a `--payload-json '{…}'` (or
  domain-specific `--*-json`) argument that is validated against shared Zod
  schemas before sending.

## HTTP client & API interaction

All server communication goes through `PaperclipApiClient` in
[cli/src/client/http.ts](../../cli/src/client/http.ts). Key behavior:

- **Verb helpers**: `get/post/patch/put/delete<T>` return `T | null` (204 and
  empty bodies map to `null`); JSON bodies are stringified automatically and
  `content-type: application/json` is set.
- **Headers**: always sends `accept: application/json`. When an API key is
  present it adds `authorization: Bearer <token>`. When a run id is present it
  adds `x-paperclip-run-id: <runId>` — required by the server for
  agent-authenticated mutations (checkout/release/interactions/in-progress issue
  updates).
- **URL building**: `buildUrl` joins the normalized `apiBase` (trailing slashes
  stripped) with the path, preserving any query string. Paths are expected to
  include the `/api/...` prefix (client commands pass e.g.
  `/api/companies/{id}/...`).
- **Errors**: non-2xx responses become `ApiRequestError` (carrying `status`,
  parsed `message`/`error`, `details`, and raw `body`). Network failures become
  `ApiConnectionError`, whose message includes the attempted `METHOD URL`, a
  probable-cause hint, and a `curl <base>/api/health` suggestion.
- **`ignoreNotFound`**: callers can opt into returning `null` on 404 instead of
  throwing (used e.g. by `heartbeat run` when polling logs).
- **Auth recovery**: an optional `recoverAuth` callback lets a request that fails
  with 401 (or a board/instance-admin 403) trigger an interactive board login and
  retry once with the recovered token. `resolveCommandContext` wires this to the
  board CLI-auth flow, but only in a TTY and only when no explicit API key was
  supplied.

The parity tests assert exact `[method, url]` pairs against this client (see
[CLI/API parity](#cliapi-parity)).

## Config & auth resolution

The CLI distinguishes three kinds of local state, each with its own file and
resolver:

### Instance config (`config.json`)

- Schema: [cli/src/config/schema.ts](../../cli/src/config/schema.ts) re-exports the
  canonical Zod schema from
  [packages/shared/src/config-schema.ts](../../packages/shared/src/config-schema.ts).
  Top-level sections: `$meta`, `llm`, `database`, `logging`, `server`,
  `telemetry`, `auth`, `storage`, `secrets`. `server` carries `deploymentMode`
  (`local_trusted` / `authenticated`), `exposure` (`private` / `public`), `bind`
  (`loopback`/`lan`/`tailnet`/`custom`), `host`, `port` (default `3100`),
  `allowedHostnames`, and `serveUi`. The schema `superRefine` enforces
  cross-field invariants (e.g. `local_trusted` must be `private`; public exposure
  requires an explicit `auth.baseUrlMode` and `auth.publicBaseUrl`).
- Read/write: [cli/src/config/store.ts](../../cli/src/config/store.ts).
  `resolveConfigPath` order is: `--config` override → `PAPERCLIP_CONFIG` env →
  nearest `.paperclip/config.json` walking up from cwd → the default
  instance path. `readConfig` migrates legacy configs (e.g. `pglite` →
  `embedded-postgres`), validates against the schema, and surfaces field-level
  validation errors. `writeConfig` backs up the prior file to `*.backup` and
  writes with mode `0600`.
- Path helpers: [cli/src/config/home.ts](../../cli/src/config/home.ts) delegates to
  `@paperclipai/shared/home-paths` to resolve the home dir, instance id, and the
  per-instance config/db/logs/storage/secrets/backup paths (honoring
  `PAPERCLIP_HOME` and `PAPERCLIP_INSTANCE_ID`).

### Instance env file (`.env`)

[cli/src/config/env.ts](../../cli/src/config/env.ts) manages a `.env` next to the
config file. It loads it once (via `dotenv`, `override: false`), and manages the
`PAPERCLIP_AGENT_JWT_SECRET` (generate-if-missing via `ensureAgentJwtSecret`,
written with mode `0600`). `doctor`'s agent-JWT check uses this.

### CLI client context (`context.json`) and API-base / API-key resolution

- Profiles: [cli/src/client/context.ts](../../cli/src/client/context.ts) stores a
  versioned `context.json` (`version: 2`) with named profiles. A profile holds
  `apiBase`, `companyId`, `persona` (`board`/`agent`), `agentId`, `agentName`,
  `apiKeyEnvVarName`, and token metadata. Crucially, profiles store the *name of
  the env var* holding the API key, **not** the token itself. `resolveContextPath`
  mirrors the config resolution order (`--context` → `PAPERCLIP_CONTEXT` →
  nearest `.paperclip/context.json` → default home path). Context files are
  written with mode `0600`.
- API base resolution (`resolveApiBase` in
  [client/common.ts](../../cli/src/commands/client/common.ts)):
  `--api-base` → `PAPERCLIP_API_URL` → profile `apiBase` →
  `inferApiBaseFromConfig` (uses `PAPERCLIP_SERVER_HOST`/`PAPERCLIP_SERVER_PORT`
  or the local config `server.port`, defaulting to `http://localhost:3100`).
- API key resolution (`resolveApiKey`): `--api-key` (source `explicit`) →
  `PAPERCLIP_API_KEY` (`env`) → the env var named by the profile's
  `apiKeyEnvVarName` (`profile_env`) → none. Company id resolves from
  `--company-id` → `PAPERCLIP_COMPANY_ID` → profile `companyId`.
- Run id for agent mutations: `--run-id` → `PAPERCLIP_RUN_ID`.

### Board CLI auth (browser-approval flow)

[cli/src/client/board-auth.ts](../../cli/src/client/board-auth.ts) implements a
device-style board login. `loginBoardCli` POSTs a challenge to
`/api/cli-auth/challenges`, prints/opens an approval URL, polls the challenge
until `approved`, calls `/api/cli-auth/me`, and stores a per-`apiBase` board
credential in `auth.json` (mode `0600`, keyed by normalized API base). Stored
credentials are read by `resolveCommandContext` as the fallback when no explicit
key is given (`authSource: "stored_board"`), and are the token the `recoverAuth`
retry produces. `command-label.ts`
([client/command-label.ts](../../cli/src/client/command-label.ts)) builds the
human-readable command string shown on the approval screen. `openUrl` shells out
to the platform opener and can be suppressed with `PAPERCLIP_NO_BROWSER`.

The interactive `connect` wizard
([client/connect.ts](../../cli/src/commands/client/connect.ts)) ties this
together: verify `/api/health`, board-login, choose a persona/company/agent,
create a scoped board or agent API key server-side, save a profile, and print
`export …` lines for the token (never persisting the token into `context.json`).

## Build & packaging

- **Bundler**: [cli/esbuild.config.mjs](../../cli/esbuild.config.mjs). It bundles
  `src/index.ts` to `dist/index.js` (ESM, `node20` target, sourcemaps,
  tree-shaking) with a `#!/usr/bin/env node` banner. All non-`@paperclipai/*` npm
  deps stay external; the listed workspace packages (`cli`, `packages/db`,
  `packages/shared`, `adapter-utils`, and the bundled adapters) are inlined.
- **`@paperclipai/server` is intentionally external** and resolved at runtime via
  dynamic import, so the CLI can start either the monorepo dev server or the
  published server package (see `importServerEntry` in
  [commands/run.ts](../../cli/src/commands/run.ts)).
- **npm bundle script**: [scripts/build-npm.sh](../../scripts/build-npm.sh) runs a
  forbidden-token check ([scripts/check-forbidden-tokens.mjs](../../scripts/check-forbidden-tokens.mjs))
  before bundling, then esbuilds and prepares the publishable `dist/`. The
  release automation rewrites the CLI version string as part of
  [build/test/release](./build-test-release.md).
- **`package.json`**: [cli/package.json](../../cli/package.json) declares
  `bin.paperclipai → ./dist/index.js`, `files: ["dist"]`, and the workspace
  adapter/`server`/`shared`/`db` dependencies. `pnpm dev` uses `tsx`.

## Testing

- **Runner**: Vitest, configured in
  [cli/vitest.config.ts](../../cli/vitest.config.ts) (Node environment). Run via
  `pnpm test` from the repo root (Playwright is separate).
- **Location**: tests live in `cli/src/__tests__/`. Coverage spans the HTTP client
  ([http.test.ts](../../cli/src/__tests__/http.test.ts)), config/home/data-dir
  resolution, board auth, context, onboarding, doctor, telemetry, and per-command
  behavior.
- **Fetch stubbing**: command tests build an isolated Commander program per group
  (`program.exitOverride()`, silenced output), pass `--api-base`/`--api-key`
  explicitly, and stub the global `fetch` to assert both the request URL/method
  and the printed output — see the parity tests below.
- **Embedded Postgres helper**:
  [__tests__/helpers/embedded-postgres.ts](../../cli/src/__tests__/helpers/embedded-postgres.ts)
  supports end-to-end company import/export tests.

## CLI/API parity

The control-plane client is meant to be a faithful, complete wrapper of the server
REST API: every relevant endpoint has a corresponding command, and each command
hits exactly the documented method + path. This is enforced two ways:

- **Parity tests** assert the precise `[method, url]` sequence the client sends
  for a group of commands:
  [operations-parity.test.ts](../../cli/src/__tests__/operations-parity.test.ts)
  (cost/finance/budget, org, workspaces, environments, project-workspaces),
  [routine-plugin-parity.test.ts](../../cli/src/__tests__/routine-plugin-parity.test.ts),
  [access-parity.test.ts](../../cli/src/__tests__/access-parity.test.ts),
  [activity-parity.test.ts](../../cli/src/__tests__/activity-parity.test.ts), and
  [admin-asset-skill-parity.test.ts](../../cli/src/__tests__/admin-asset-skill-parity.test.ts).
  For example, `cost summary --company-id <id>` must produce
  `GET /api/companies/<id>/costs/summary`.
- **OpenAPI reference**: the server publishes an OpenAPI document
  (`GET /api/openapi.json`, wrapped by the `openapi` command) generated in
  [server/src/routes/openapi.ts](../../server/src/routes/openapi.ts). The parity
  effort and endpoint inventory are tracked in
  [doc/plans/2026-05-23-cli-api-parity-openapi-reference.ts](../../doc/plans/2026-05-23-cli-api-parity-openapi-reference.ts).
  The authoritative command list is [doc/CLI.md](../../doc/CLI.md).

When adding or changing an endpoint on the [server](./server.md), the expectation
is: add/update the matching CLI command, its parity test, and the entry in
[doc/CLI.md](../../doc/CLI.md).

## Relationship to server, adapters, and heartbeats

- **Server**: client commands are pure HTTP callers; they never open the DB. The
  `run` command is the exception that *starts* the [server](./server.md) in-process
  via `startServer()`.
- **Adapters**: `heartbeat run`
  ([commands/heartbeat-run.ts](../../cli/src/commands/heartbeat-run.ts)) invokes a
  single agent wakeup (`POST /api/agents/{id}/wakeup`), then polls run events and
  logs and renders adapter stdout using the CLI-side stream formatter selected by
  the agent's `adapterType`. The formatter registry is
  [cli/src/adapters/registry.ts](../../cli/src/adapters/registry.ts), which maps
  adapter types (`claude_local`, `codex_local`, `cursor`, `gemini_local`,
  `hermes_gateway`, `openclaw_gateway`, …) to `printStreamEvent` functions from the
  `@paperclipai/adapter-*` packages, falling back to the generic process/HTTP
  adapters ([adapters/process/index.ts](../../cli/src/adapters/process/index.ts),
  [adapters/http/index.ts](../../cli/src/adapters/http/index.ts)). See
  [adapters](./adapters.md).
- **Prompt handoff** creates real Paperclip work (an issue + wakeup), not a chat
  session — see the `prompt` commands in
  [client/prompt.ts](../../cli/src/commands/client/prompt.ts).

## Gotchas / invariants

- **`--data-dir` must be a global-level concern.** The `preAction` hook only
  derives default `PAPERCLIP_CONFIG`/`PAPERCLIP_CONTEXT` for commands that
  actually declare `--config`/`--context` options; it always sets
  `PAPERCLIP_HOME`. Commands added without the common options may not inherit the
  isolated config/context path.
- **API paths include `/api`.** `PaperclipApiClient` does not add the `/api`
  prefix; command code passes full `/api/...` paths (via `apiPath`). `apiBase` is
  the origin (e.g. `http://localhost:3100`), not `.../api`.
- **`x-paperclip-run-id` is required for agent mutations.** Checkout, release,
  interactions, and in-progress issue PATCHes 401 with "Agent run id required"
  unless `--run-id`/`PAPERCLIP_RUN_ID` is present.
- **Secrets never live in `context.json`.** Profiles store `apiKeyEnvVarName`; the
  token is read from that env var at call time. `connect`/`token` print plaintext
  tokens once. Board credentials are stored in `auth.json` (mode `0600`) keyed by
  API base; `secrets` commands never print secret values.
- **Interactive auth recovery only fires in a TTY** and only when no explicit
  `--api-key` was passed; non-interactive/CI callers must supply a key or a stored
  credential.
- **Config write-through invariants.** `local_trusted` implies `private` exposure;
  `public` exposure requires an explicit `auth.baseUrlMode`/`publicBaseUrl`. These
  are enforced by the schema `superRefine`, so a hand-edited `config.json` can fail
  `readConfig`/`doctor`.
- **`@paperclipai/server` stays external in the bundle.** Do not import it
  statically from CLI code paths that must run in a published (server-less) install
  — it is resolved by dynamic import in `run` only.
- **Forbidden-token check gates publish.** [scripts/build-npm.sh](../../scripts/build-npm.sh)
  runs [scripts/check-forbidden-tokens.mjs](../../scripts/check-forbidden-tokens.mjs)
  to keep local/private values (tailnet hosts, machine-specific tokens) out of the
  published bundle.

## Key files

- [cli/src/index.ts](../../cli/src/index.ts) — Commander program, command
  registration, global `preAction` hook, `main()`.
- [cli/src/version.ts](../../cli/src/version.ts) — CLI version from `package.json`.
- [cli/src/client/http.ts](../../cli/src/client/http.ts) — `PaperclipApiClient`,
  `ApiRequestError`, `ApiConnectionError`, auth-recovery retry.
- [cli/src/client/context.ts](../../cli/src/client/context.ts) — CLI context
  profiles (`context.json`).
- [cli/src/client/board-auth.ts](../../cli/src/client/board-auth.ts) — browser
  board-login challenge flow + `auth.json` credential store.
- [cli/src/client/command-label.ts](../../cli/src/client/command-label.ts) —
  approval-screen command label.
- [cli/src/commands/client/common.ts](../../cli/src/commands/client/common.ts) —
  shared options, `resolveCommandContext`, `apiPath`, output/error helpers.
- [cli/src/config/schema.ts](../../cli/src/config/schema.ts) — config schema
  re-export; canonical schema in
  [packages/shared/src/config-schema.ts](../../packages/shared/src/config-schema.ts).
- [cli/src/config/store.ts](../../cli/src/config/store.ts) — config read/write,
  resolution order, legacy migration.
- [cli/src/config/home.ts](../../cli/src/config/home.ts) — instance/home path
  resolution.
- [cli/src/config/data-dir.ts](../../cli/src/config/data-dir.ts) — `--data-dir`
  override logic.
- [cli/src/config/env.ts](../../cli/src/config/env.ts) — instance `.env` and agent
  JWT secret management.
- [cli/src/commands/run.ts](../../cli/src/commands/run.ts) — bootstrap + start
  server (dynamic server import).
- [cli/src/commands/heartbeat-run.ts](../../cli/src/commands/heartbeat-run.ts) —
  single-agent wakeup + live log streaming.
- [cli/src/commands/doctor.ts](../../cli/src/commands/doctor.ts) /
  [cli/src/checks/index.ts](../../cli/src/checks/index.ts) — diagnostics.
- [cli/src/commands/onboard.ts](../../cli/src/commands/onboard.ts) /
  [cli/src/commands/configure.ts](../../cli/src/commands/configure.ts) — setup
  wizards.
- [cli/src/adapters/registry.ts](../../cli/src/adapters/registry.ts) — CLI-side
  adapter stream-formatter registry.
- [cli/src/telemetry.ts](../../cli/src/telemetry.ts) — anonymous telemetry init /
  flush.
- [cli/esbuild.config.mjs](../../cli/esbuild.config.mjs) — bundle config.
- [cli/vitest.config.ts](../../cli/vitest.config.ts) — test runner config.
- [cli/package.json](../../cli/package.json) — `bin`, deps, scripts.
- [doc/CLI.md](../../doc/CLI.md) — full user-facing command reference.
- [doc/DEPLOYMENT-MODES.md](../../doc/DEPLOYMENT-MODES.md) — deployment-mode
  taxonomy referenced by onboarding/configure.
