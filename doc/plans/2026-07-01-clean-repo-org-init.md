# 2026-07-01 Clean Repo Org Init (paperclip)

Mode: `init` (applied to the paperclip repository).

Goal: stand up the modular internal-docs structure under `doc/` described by the
[clean-repo-org skill](https://github.com/frankji-groundcontrol/franky-frank) and
[references/repo-records.md](../../references/repo-records.md), keep `AGENTS.md`
and `CLAUDE.md` as thin routers, and write comprehensive architecture docs
("reports") for every subsystem by reading the whole codebase.

Location decision: modular records live under internal `doc/`. The public
Mintlify site under `docs/` (governed by [docs/docs.json](../../docs/docs.json))
is intentionally left untouched.

Status: **complete.**

## Scope

- Read the whole repository subsystem-by-subsystem and produce accurate
  `doc/architecture/*` docs with clickable relative links to real source files.
- Bootstrap record indexes for architecture, issues, learning, plans, practices,
  and a usage index that routes to the existing public docs.
- Add `references/repo-records.md` adapted to paperclip's `doc/` layout.
- Keep `AGENTS.md` and `CLAUDE.md` thin; add a docs-maintenance reminder that
  links to the new `doc/` indexes.

## Docs bootstrap checklist

| Surface | Status | Notes |
|---------|--------|-------|
| `doc/plans/2026-07-01-clean-repo-org-init.md` | Created | This live plan record. |
| [`doc/index.md`](../index.md) | Created | Internal documentation map. |
| [`doc/architecture/index.md`](../architecture/index.md) | Created | Architecture map + 11 subsystem links. |
| [`doc/architecture/server.md`](../architecture/server.md) | Created | Express API, middleware, composition (509 lines). |
| [`doc/architecture/orchestration.md`](../architecture/orchestration.md) | Created | Issues/runs/heartbeat/routines/approvals/budgets/workspaces/secrets (488). |
| [`doc/architecture/ui.md`](../architecture/ui.md) | Created | React + Vite board UI (561). |
| [`doc/architecture/cli.md`](../architecture/cli.md) | Created | CLI commands, client, config, parity (420). |
| [`doc/architecture/data-model.md`](../architecture/data-model.md) | Created | packages/db schema, migrations, scoping (410). |
| [`doc/architecture/shared-contracts.md`](../architecture/shared-contracts.md) | Created | packages/shared contracts (309). |
| [`doc/architecture/adapters.md`](../architecture/adapters.md) | Created | packages/adapters + adapter-utils (471). |
| [`doc/architecture/plugins.md`](../architecture/plugins.md) | Created | Plugin SDK, host, sandbox providers (514). |
| [`doc/architecture/catalogs.md`](../architecture/catalogs.md) | Created | teams-catalog + skills-catalog (241). |
| [`doc/architecture/mcp-server.md`](../architecture/mcp-server.md) | Created | packages/mcp-server (210). |
| [`doc/architecture/build-test-release.md`](../architecture/build-test-release.md) | Created | scripts, tests, evals, docker, CI, release (602). |
| [`doc/issues/README.md`](../issues/README.md) | Created | Issue record index. |
| [`doc/learning/README.md`](../learning/README.md) | Created | Learning record index. |
| [`doc/practices/README.md`](../practices/README.md) | Created | Practice record index. |
| [`doc/plans/README.md`](README.md) | Created | Plan record index. |
| [`doc/usage/README.md`](../usage/README.md) | Created | Usage index routing to public docs + dev usage. |
| [`references/repo-records.md`](../../references/repo-records.md) | Created | Record-system rules adapted to `doc/`. |
| `AGENTS.md` router | Updated | Docs-maintenance reminder + links to `doc/` indexes. |
| `CLAUDE.md` router | Updated | Docs-maintenance reminder + links to `doc/` indexes. |

## How the architecture docs were produced

Each subsystem doc was written by an agent that read the actual code, then an
independent adversarial verifier re-read the doc against the source and fixed
inaccuracies (e.g. `accessService` lives in `services/access.ts`, MCP tool count
41 not ~39, CI release flags, `doctor --repair` vs `--fix`). All 11 docs were
returned `accurate: true` and `privacyClean: true`.

## Verification

- 1047 relative links across the new docs and routers resolve (0 broken).
- Architecture index links match the 11 files on disk exactly.
- Privacy scan clean: no tokens, real emails, tailnet/private URLs, local
  runtime paths, or internal issue IDs (the one example was redacted to
  `PAP-XXXX`).

## Privacy

- Architecture docs cite real repo-relative file paths only.
- No private identifiers introduced.

## Completion

- [x] All architecture docs written from direct code reads and links verified.
- [x] All record indexes created and cross-linked.
- [x] `references/repo-records.md` present and accurate for `doc/`.
- [x] `AGENTS.md` and `CLAUDE.md` route to `doc/` indexes without bloating.
- [x] Internal links resolve; no private identifiers introduced.
- [x] This plan record marked complete with no unchecked items.
