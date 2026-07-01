# Architecture Map

Subsystem-by-subsystem architecture for the Paperclip monorepo, written from
direct reads of the code. Paperclip is a control plane for AI-agent companies: a
Node/Express API and orchestration layer, a React + Vite board UI, a CLI, and a
set of workspace packages (schema, shared contracts, adapters, plugins, catalogs,
MCP server).

Router files ([`AGENTS.md`](../../AGENTS.md), [`CLAUDE.md`](../../CLAUDE.md)) link
here instead of embedding architecture. Keep these docs current when structure
changes (see [repo-records](../../references/repo-records.md)).

## Subsystems

| Doc | Scope | Source root |
|-----|-------|-------------|
| [Server](server.md) | HTTP API composition, middleware, auth wiring, error handling, startup | [`server/`](../../server) |
| [Orchestration](orchestration.md) | Issues/runs, heartbeat, routines, approvals, budgets, workspaces, secrets | [`server/src`](../../server/src) |
| [Board UI](ui.md) | React + Vite SPA, routing, data fetching, design system | [`ui/`](../../ui) |
| [CLI](cli.md) | Command tree, API client, config, parity | [`cli/`](../../cli) |
| [Data model](data-model.md) | Drizzle schema, migrations, company scoping | [`packages/db`](../../packages/db) |
| [Shared contracts](shared-contracts.md) | Cross-layer types, constants, validators, API paths | [`packages/shared`](../../packages/shared) |
| [Adapters](adapters.md) | Agent adapter contract, built-ins, external loading | [`packages/adapters`](../../packages/adapters) |
| [Plugins](plugins.md) | Plugin SDK, host, manifests, sandbox providers | [`packages/plugins`](../../packages/plugins) |
| [Catalogs](catalogs.md) | Team & skill catalogs, generated vs source | [`packages/teams-catalog`](../../packages/teams-catalog) |
| [MCP server](mcp-server.md) | MCP tools/transport for agents | [`packages/mcp-server`](../../packages/mcp-server) |
| [Build/test/release](build-test-release.md) | Scripts, tests, evals, Docker, CI, release | [`scripts/`](../../scripts) |

## Cross-cutting invariants

These hold across subsystems (see per-doc "Gotchas / invariants" for specifics):

- Company scoping is enforced in routes, services, queries, and UI state.
- Contracts stay synchronized across `packages/db`, `packages/shared`, `server`,
  `ui`, and `cli`.
- Governed actions, approvals, budgets, activity logging, secrets, and low-trust
  boundaries are control-plane contracts, not optional.
- Public authenticated deployments require real PostgreSQL configuration.

## Maintenance

Update the relevant subsystem doc in the same change that alters its structure,
and keep this index's table in sync when a subsystem is added or removed.
