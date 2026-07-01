# Architecture — Shared contracts (packages/shared)

`@paperclipai/shared` is the single source of truth for the cross-layer contracts of
the Paperclip monorepo. Outside the telemetry subpath it contains no business logic and
performs no I/O — the main barrel is only the vocabulary that every other layer agrees on:
enum-style constant tuples, TypeScript domain types, Zod request/response validators, API
path constants, and a handful of pure helpers (mention hrefs, document anchors, routine
variables, config parsing). The one exception is the `@paperclipai/shared/telemetry`
subpath, which does perform I/O (`node:fs` state file, `node:crypto` hashing, `fetch` to
the ingest endpoint) and is quarantined behind its own export so the main barrel stays
pure. If a value crosses a boundary between the server, the UI, the CLI, or the database
schema, its shape is defined here.

> Scope: this doc covers the `packages/shared` subsystem. For the layers that consume it,
> see [server.md](./server.md), [data-model.md](./data-model.md), [adapters.md](./adapters.md),
> and [mcp-server.md](./mcp-server.md).

## Purpose / overview

The package exists to keep four independently-built consumers structurally in lockstep:

- **server** ([server/src](../../server/src)) — validates every request body/query with the
  Zod schemas exported here and returns responses typed by the shared domain types.
- **ui** ([ui/src](../../ui/src)) — imports the same domain types for its API client and
  React components, so the wire format is checked at compile time on both ends.
- **cli** ([cli/src](../../cli/src)) — parses `config.json` with `paperclipConfigSchema`,
  resolves instance paths with the `home-paths` helpers, and emits telemetry.
- **packages/db** ([packages/db/src](../../packages/db/src)) — types Drizzle `jsonb` columns
  with shared interfaces so persisted blobs match the types the API serves.

Because all four import from the same package (via the `workspace:*` protocol), a change to
a contract that is not propagated fails to type-check in the consumers rather than drifting
silently at runtime. That invariant is the entire reason the package exists.

Runtime dependency: `zod` only. Everything else is standard-library — the pure modules use
`node:os` / `node:path`, and the telemetry subsystem additionally reaches for `node:fs` and
`node:crypto` plus the global `fetch`.

## Entry points

Consumers resolve the package through the `exports` map in
[packages/shared/package.json](../../packages/shared/package.json):

| Specifier | Resolves to | Contents |
| --- | --- | --- |
| `@paperclipai/shared` | [src/index.ts](../../packages/shared/src/index.ts) | The aggregate barrel — constants, types, validators, API paths, helpers |
| `@paperclipai/shared/telemetry` | [src/telemetry/index.ts](../../packages/shared/src/telemetry/index.ts) | Telemetry client, config resolver, and `track*` event emitters |
| `@paperclipai/shared/*` | `src/*.ts` | Direct deep imports of any top-level module |

The wildcard subpath export is why a handful of call sites deep-import specific modules
instead of the barrel, e.g. `@paperclipai/shared/home-paths`,
`@paperclipai/shared/external-objects-server`, and
`@paperclipai/shared/validators/adapter-skills`. `external-objects-server` is a deliberately
separate entry so browser bundles do not pull server-only URL-canonicalization code through
the main barrel.

Development vs. published resolution: in-repo, `exports` points straight at the `.ts`
sources (no build step is needed for workspace consumers). The `publishConfig` block
rewrites `exports`/`main`/`types` to the compiled `dist/` output produced by
`pnpm build` (`tsc`, per [tsconfig.json](../../packages/shared/tsconfig.json)).

The barrel is large by design — [src/index.ts](../../packages/shared/src/index.ts) is ~1,500
lines of pure re-exports so that the common case (`import { … } from "@paperclipai/shared"`)
reaches everything. Over 600 files across the consumers import from the bare specifier.

## Key modules & responsibilities

### Constants — [src/constants.ts](../../packages/shared/src/constants.ts)

The canonical enumerations. Each is a `readonly` tuple plus a derived union type, e.g.:

```ts
export const AGENT_STATUSES = ["active","paused","idle","running","error","pending_approval","terminated"] as const;
export type AgentStatus = (typeof AGENT_STATUSES)[number];
```

This `const`-tuple + `[number]` pattern is the load-bearing idiom of the whole package: the
tuple is used at runtime (to build `z.enum(...)`, to render dropdowns, to seed DB checks) and
the union type is used at compile time. Adding a value in one place updates validators, types,
and UI simultaneously. Covered domains include agents (`AGENT_STATUSES`, `AGENT_ADAPTER_TYPES`,
`AGENT_ROLES` + `AGENT_ROLE_LABELS`), issues (`ISSUE_STATUSES`, `ISSUE_PRIORITIES`,
`ISSUE_WORK_MODES`, the large `ISSUE_EXECUTION_*` and `ISSUE_THREAD_INTERACTION_*` families),
routines, environments, secrets, budgets/finance, plugins (`PLUGIN_*`), memberships/permissions,
and deployment (`DEPLOYMENT_MODES`, `BIND_MODES`). `AgentAdapterType` is intentionally an open
union (`(typeof AGENT_ADAPTER_TYPES)[number] | (string & {})`) so plugin-supplied adapters are
accepted without editing the tuple.

### Domain types — [src/types/](../../packages/shared/src/types)

One file per domain aggregate (e.g. [types/issue.ts](../../packages/shared/src/types/issue.ts),
[types/agent.ts](../../packages/shared/src/types/agent.ts),
[types/pipeline.ts](../../packages/shared/src/types/pipeline.ts),
[types/plugin.ts](../../packages/shared/src/types/plugin.ts)), re-exported through
[types/index.ts](../../packages/shared/src/types/index.ts). These are the response DTOs the
API returns and the UI consumes. They reference the constant unions for status/kind fields
(see the `import type { … } from "../constants.js"` block at the top of
[types/issue.ts](../../packages/shared/src/types/issue.ts)), keeping enum fields honest.

### Validators — [src/validators/](../../packages/shared/src/validators)

One file per domain (e.g. [validators/issue.ts](../../packages/shared/src/validators/issue.ts),
[validators/agent.ts](../../packages/shared/src/validators/agent.ts),
[validators/routine.ts](../../packages/shared/src/validators/routine.ts)), re-exported through
[validators/index.ts](../../packages/shared/src/validators/index.ts). These are the Zod schemas
for request bodies/queries. Each schema is paired with an inferred input type via
`z.infer<...>` (exported as e.g. `CreateIssue`, `UpdateAgent`), so a route handler and its
caller share one definition. Validators import the constant tuples and wrap them in `z.enum`,
so the accepted values track the enums automatically. Shared refinements live in
[validators/text.ts](../../packages/shared/src/validators/text.ts) (`multilineTextSchema`,
`normalizeEscapedLineBreaks`).

### API path constants — [src/api.ts](../../packages/shared/src/api.ts)

`API_PREFIX = "/api"` and the `API` record of route roots
(`API.companies`, `API.issues`, `API.approvals`, …). Server mounts and UI fetch calls read
from this single object, so a path rename is a one-line change that both sides pick up.

### Config schema — [src/config-schema.ts](../../packages/shared/src/config-schema.ts)

`paperclipConfigSchema` and its sub-schemas (`serverConfigSchema`, `databaseConfigSchema`,
`storageConfigSchema`, `secretsConfigSchema`, `authConfigSchema`, `telemetryConfigSchema`, …)
describe the on-disk `config.json` written during onboarding. The CLI parses raw JSON through
this schema; the server reads the same validated shape. It reuses `DEPLOYMENT_MODES`,
`BIND_MODES`, `STORAGE_PROVIDERS`, `SECRET_PROVIDERS` from constants and delegates bind-mode
consistency to `validateConfiguredBindMode` in
[src/network-bind.ts](../../packages/shared/src/network-bind.ts).

### Home paths — [src/home-paths.ts](../../packages/shared/src/home-paths.ts)

Pure filesystem-path resolution for the `~/.paperclip/instances/<id>` layout
(`resolvePaperclipHomeDir`, `resolvePaperclipInstanceRoot`,
`resolvePaperclipInstanceConfigPath`, `expandHomePrefix`). Deep-imported by the CLI and by
[packages/db](../../packages/db/src) (`backup.ts`, `runtime-config.ts`) so the CLI, server, and
DB agree on where instance data lives. `resolvePaperclipInstanceId` enforces a safe path
segment.

### Pure domain helpers

Dependency-free logic that must produce identical results on client and server:

- [src/agent-eligibility.ts](../../packages/shared/src/agent-eligibility.ts) — org-chain and
  work-eligibility predicates (`getAgentWorkEligibility`, `isAgentAssignableToWork`).
- [src/pipeline-health.ts](../../packages/shared/src/pipeline-health.ts) /
  [src/pipeline-case-type.ts](../../packages/shared/src/pipeline-case-type.ts) — pipeline
  warning computation and case-type derivation.
- [src/project-mentions.ts](../../packages/shared/src/project-mentions.ts) — the mention URI
  schemes (`agent://`, `project://`, `user://`, `skill://`, `routine://`, `pipeline://`) with
  matched build/parse/extract helpers, so mention links round-trip identically wherever they
  are rendered or parsed.
- [src/issue-references.ts](../../packages/shared/src/issue-references.ts) — issue-identifier
  parsing/href helpers.
- [src/document-anchors.ts](../../packages/shared/src/document-anchors.ts) — text-quote anchor
  creation/remap/verify for document annotations.
- [src/routine-variables.ts](../../packages/shared/src/routine-variables.ts) — routine template
  variable extraction/interpolation and built-ins (`date`, `timestamp`).
- [src/frontmatter.ts](../../packages/shared/src/frontmatter.ts) — markdown frontmatter parsing.
- [src/environment-support.ts](../../packages/shared/src/environment-support.ts) — which
  environment drivers / sandbox providers an adapter supports.
- [src/trust-policy.ts](../../packages/shared/src/trust-policy.ts) — trust presets and
  low-trust review policy constants/types.

### Telemetry — [src/telemetry/](../../packages/shared/src/telemetry)

A self-contained subsystem exported under the `/telemetry` subpath (and *only* that subpath —
it is never re-exported from the main barrel, precisely because it performs I/O):
[client.ts](../../packages/shared/src/telemetry/client.ts) (`TelemetryClient`, which `fetch`es
the ingest endpoint and hashes private refs via `node:crypto`),
[config.ts](../../packages/shared/src/telemetry/config.ts) (`resolveTelemetryConfig`),
[state.ts](../../packages/shared/src/telemetry/state.ts) (`loadOrCreateState`, which reads/writes
an anonymous-id state file with `node:fs`), and
[events.ts](../../packages/shared/src/telemetry/events.ts) (the `track*` emitters such as
`trackAgentTaskCompleted`, `trackRoutineRun`, `trackCompanyImported`). Server and CLI both emit
through it. Event names/types live in
[telemetry/types.ts](../../packages/shared/src/telemetry/types.ts).

## Data flow / how a contract is used

A create-issue request illustrates the full round trip through the package:

1. **UI** builds a request body typed by `CreateIssue` (the `z.infer` of
   `createIssueSchema`) and POSTs to a path derived from `API.issues` /
   `API_PREFIX` — both from `@paperclipai/shared`.
2. **Server** mounts the route and runs the body through the generic Zod middleware
   [server/src/middleware/validate.ts](../../server/src/middleware/validate.ts), which calls
   `schema.parse(req.body)`. The route in
   [server/src/routes/issues.ts](../../server/src/routes/issues.ts) passes
   `validate(createIssueSchema)` (imported from `@paperclipai/shared`). Invalid bodies are
   rejected before any handler logic runs; query params use `.safeParse(...)` directly.
3. **DB** persists rows into tables whose enum-like `text` columns default to values from the
   same constant tuples, and whose `jsonb` columns are `$type<>`-annotated with shared
   interfaces — e.g. [packages/db/src/schema/issues.ts](../../packages/db/src/schema/issues.ts)
   types `sourceTrust`-style blobs as `SourceTrustMetadata` imported from `@paperclipai/shared`.
4. **Server** serializes the row back into an `Issue` domain type.
5. **UI** receives it typed as `Issue`, closing the loop with no hand-written duplicate
   interface anywhere in the chain.

The CLI path is analogous for configuration: raw `config.json` → `paperclipConfigSchema.parse`
→ typed `PaperclipConfig` consumed identically by CLI and server, with paths resolved through
`home-paths`.

## Contracts & cross-layer coupling

The core rule this package enforces:

> **Schema and API changes must stay synchronized through `packages/shared`.** A field, enum
> value, route path, or config key that any two layers share is edited here first; the change
> then propagates by type-checking into every consumer.

Concretely:

- **Enum values** are added to the constant tuple in `constants.ts`. Validators
  (`z.enum(TUPLE)`), domain types (`(typeof TUPLE)[number]`), and DB defaults all follow from
  that one edit. Forgetting a consumer surfaces as a compile error, not a runtime 500.
- **Request/response shape** changes live in `validators/` + `types/` as a schema/type pair.
  Because the server derives its handler input from `z.infer` and the UI imports the type, a
  breaking change to one side breaks the other's build.
- **Route paths** are edited only in `api.ts`; server mounts and UI clients dereference the
  same `API` object.
- **Persisted JSON** (`jsonb` columns) is coupled by `$type<import("@paperclipai/shared").X>()`
  annotations in [packages/db/src/schema](../../packages/db/src/schema), so the stored blob and
  the served DTO cannot diverge in type. See [data-model.md](./data-model.md) for the full DB
  view.
- **Directory conventions** (`~/.paperclip/...`) come exclusively from `home-paths`, so the CLI
  (which creates instances) and the server/DB (which read them) never hardcode divergent paths.

Because `packages/shared` sits at the bottom of the dependency graph, it must not import from
`server`, `ui`, `cli`, or `db` — the coupling is strictly one-directional (consumers depend on
shared, never the reverse).

## Extension points

- **New enum**: add a `const` tuple + derived union in `constants.ts`, re-export from
  `index.ts`, then reference it from the relevant validator and type.
- **New endpoint contract**: add `<domain>.ts` under `validators/` (schema + `z.infer` type)
  and, if a response DTO is needed, a matching type under `types/`; wire both into the
  respective `index.ts` barrels and the top-level `index.ts`.
- **New route root**: add a key to the `API` record in `api.ts`.
- **New config key**: extend the appropriate sub-schema in `config-schema.ts` (give it a
  `.default(...)` so existing config files stay valid).
- **New telemetry event**: add a `track*` emitter in `telemetry/events.ts` and export it from
  `telemetry/index.ts`.
- **New adapter/environment support**: extend `AGENT_ADAPTER_TYPES` and the capability tables
  in `environment-support.ts`; the open `AgentAdapterType` union also admits plugin-defined
  adapter strings without a tuple edit.
- **Adapter registry**: `adapterRegistrySchema` /
  [validators/adapter-registry.ts](../../packages/shared/src/validators/adapter-registry.ts)
  validates externally-supplied adapter registry entries.

## Testing

Tests are colocated `*.test.ts` files run with Vitest
([vitest.config.ts](../../packages/shared/vitest.config.ts), `include: ["src/**/*.test.ts"]`);
there are ~24 across the package. They concentrate on the pure helpers and validators where the
logic is non-trivial — for example
[document-anchors.test.ts](../../packages/shared/src/document-anchors.test.ts),
[routine-variables.test.ts](../../packages/shared/src/routine-variables.test.ts),
[agent-eligibility.test.ts](../../packages/shared/src/agent-eligibility.test.ts),
[config-schema.test.ts](../../packages/shared/src/config-schema.test.ts),
[external-objects.test.ts](../../packages/shared/src/external-objects.test.ts), and several
validator suites under [validators/](../../packages/shared/src/validators)
(`issue.test.ts`, `routine.test.ts`, `approval.test.ts`, `work-product.test.ts`, …). Pure
re-export/type files carry no tests — TypeScript's compiler is their check. `pnpm typecheck`
(`tsc --noEmit`) guards the whole package.

## Gotchas / invariants

- **The main barrel must stay pure and browser-safe.** The `@paperclipai/shared` barrel must
  stay importable from both browser and Node bundles, so it carries no I/O. Two kinds of
  side-effecting code are therefore quarantined behind their own subpaths and never re-exported
  from the barrel: server-only URL canonicalization in
  [external-objects-server.ts](../../packages/shared/src/external-objects-server.ts)
  (`@paperclipai/shared/external-objects-server`) and the I/O-bearing telemetry subsystem
  (`@paperclipai/shared/telemetry`, which touches `node:fs` / `node:crypto` / `fetch`). Adding
  either kind of code to the barrel breaks browser consumers.
- **`.js` import specifiers.** Source files import siblings with `.js` extensions
  (`from "./constants.js"`) even though the files are `.ts` — this is the NodeNext/ESM convention
  the repo's `tsconfig.base.json` enforces. New files must follow it.
- **The `as const` tuple is load-bearing.** Do not replace a constant tuple with a bare TS
  `type` union; the runtime array is consumed by `z.enum(...)`, DB seeding, and UI rendering.
- **Barrel completeness.** A new export must be added to the domain `index.ts` *and* the
  top-level [index.ts](../../packages/shared/src/index.ts) to be reachable via the bare
  `@paperclipai/shared` specifier (the default for consumers).
- **Config defaults must be backward-compatible.** `config-schema.ts` fields are consumed
  against config files written by older versions; new fields need `.default(...)` or `.optional()`
  or onboarding parsing breaks.
- **Version/publish coupling.** The package is versioned (Changesets;
  [CHANGELOG.md](../../packages/shared/CHANGELOG.md)) and published; `publishConfig` swaps
  `exports` to `dist/`. In-repo consumers use `workspace:*` and the raw `src/` sources, so a
  contract change is visible to them immediately without a rebuild.

## Key files

- [packages/shared/package.json](../../packages/shared/package.json) — exports map, subpath
  entries, `zod` dependency, build/typecheck scripts.
- [src/index.ts](../../packages/shared/src/index.ts) — the aggregate barrel re-exporting
  everything.
- [src/constants.ts](../../packages/shared/src/constants.ts) — canonical enum tuples + union
  types.
- [src/api.ts](../../packages/shared/src/api.ts) — `API_PREFIX` and `API` route-path record.
- [src/types/index.ts](../../packages/shared/src/types/index.ts) — domain-type barrel.
- [src/validators/index.ts](../../packages/shared/src/validators/index.ts) — Zod-schema barrel.
- [src/config-schema.ts](../../packages/shared/src/config-schema.ts) — `paperclipConfigSchema`
  for `config.json`.
- [src/home-paths.ts](../../packages/shared/src/home-paths.ts) — `~/.paperclip` instance path
  resolution.
- [src/telemetry/index.ts](../../packages/shared/src/telemetry/index.ts) — telemetry client and
  event emitters.
- [server/src/middleware/validate.ts](../../server/src/middleware/validate.ts) — the consumer
  seam where a shared Zod schema becomes request validation.
