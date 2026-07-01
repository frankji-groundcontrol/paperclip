# AGENTS.md

Guidance for human and AI contributors working in this repository.

## 1. Purpose and Authority

Paperclip is a control plane for AI-agent companies.
The current implementation target is V1 and is defined in `doc/SPEC-implementation.md`.

This file is the canonical cross-agent contributor guide for repository work. Keep it grounded in direct repository evidence: before broad documentation or architecture changes, inspect the relevant files and cite existing docs/source paths in the change or hand-off. Do not scaffold generic agent instructions that are not traceable to this repo.

Instruction precedence:

1. Direct user/task instructions narrow the immediate scope.
2. This root `AGENTS.md` defines repository-wide rules.
3. `CLAUDE.md`, when present, is a Claude-specific entry point that must defer to this file.
4. Package-local/generated `AGENTS.md` files under onboarding assets, catalogs, plugin templates, or shipped examples are product content unless the task is explicitly editing those templates.
5. Project skills provide workflow details when invoked; follow them without copying their full text here.

## 2. Read This First

Before making changes, read in this order:

1. `doc/GOAL.md`
2. `doc/PRODUCT.md`
3. `doc/SPEC-implementation.md`
4. `doc/DEVELOPING.md`
5. `doc/DATABASE.md`

`doc/SPEC.md` is long-horizon product context.
`doc/SPEC-implementation.md` is the concrete V1 build contract.

Documentation namespaces:

- `doc/`: internal product, developer, operations, release, database, plugin, adapter, and agent-artifact documentation.
- `docs/`: public user/admin/deploy/API documentation; navigation is maintained in `docs/docs.json`.
- `report/`, loose screenshots, and ad-hoc notes are evidence or archival material unless linked from canonical docs or attached through the Paperclip artifact workflow.

When updating docs, update the narrow module you touched and its index/navigation/source references. Do not rescan or rewrite unrelated docs. For doc moves, use copy-first/link-safe migration: create the replacement, update links/navigation/sources, verify references, then remove the old file only when the task explicitly calls for deletion.

Internal docs map: [`doc/index.md`](doc/index.md). Modular records live under [`doc/architecture/`](doc/architecture/index.md) (subsystem design), [`doc/plans/`](doc/plans/README.md), [`doc/issues/`](doc/issues/README.md), [`doc/learning/`](doc/learning/README.md), and [`doc/practices/`](doc/practices/README.md). Maintain them per [`references/repo-records.md`](references/repo-records.md), and update the relevant architecture doc in the same change that alters a subsystem's structure.

## 3. Repo Map

- `server/`: Express REST API, auth, orchestration services, workspace/runtime services, plugin host services, and server tests.
- `ui/`: React + Vite board UI, routing, API client usage, design guide page, and Storybook.
- `cli/`: Paperclip CLI commands and API client/config handling.
- `packages/db/`: Drizzle schema, migrations, DB clients, embedded PostgreSQL/runtime migration helpers.
- `packages/shared/`: shared types, constants, validators, API path constants, and cross-layer contracts.
- `packages/adapters/`: built-in and external agent adapter implementations plus adapter authoring docs.
- `packages/adapter-utils/`: shared adapter utilities.
- `packages/plugins/`: plugin system packages, SDK, manifests, sandbox providers, examples, and plugin tests.
- `packages/teams-catalog/`: generated/cataloged agent company packages and related builder/validation code.
- `packages/skills-catalog/`: generated/cataloged skills and skill package tooling.
- `packages/mcp-server/`: MCP/control-plane server package.
- `skills/`: repository skills for Paperclip coordination and related workflows.
- `.claude/skills/`: Claude-specific project skills, including `design-guide`.
- `.agents/`: distributable agent/skill content and templates; do not treat shipped examples as root contributor instructions.
- `doc/`: internal product/developer/ops docs.
- `docs/`: public docs site content and navigation.
- `tests/`: Playwright e2e and release-smoke suites.
- `evals/`: promptfoo and prompt/eval assets.
- `scripts/`: dev-service, release, Docker smoke, migration, security, and operational helpers.
- `docker/`: compose files, deployment examples, untrusted review, quadlet, ECS, runtime images, and Docker helper docs.
- `tools/`: local tooling such as agent shims.
- `releases/`: release changelog files.
- `patches/`: dependency patches required by the repo.
- `screenshots/`, `docs/pr-screenshots/`, `doc/screenshots/`: visual evidence/doc assets; keep public filenames free of internal IDs.
- `.github/`: PR templates, CI workflows, policy checks, and issue templates.

## 4. Working Style

Use these behavior rules for non-trivial work:

1. Think before coding.
   - Restate the requested outcome as observable success criteria.
   - State assumptions when scope is clear.
   - Ask only when ambiguity changes security, privacy, data scope, API contracts, user-visible behavior, cost, or external side effects.

2. Simplicity first.
   - Prefer the smallest solution that satisfies today's requirement.
   - Do not add speculative flexibility, caching, configuration, strategy layers, or impossible-case error handling.
   - Paperclip invariants still win over minimalism: company scoping, authz, audit/activity logging, approval gates, budgets, schema/type sync, and public deployment safety are mandatory.

3. Surgical changes.
   - Touch only lines tied to the request.
   - Match surrounding style, naming, and comment density.
   - Do not reformat unrelated code or rewrite adjacent comments because they look imperfect.
   - Mention unrelated cleanup opportunities instead of doing them silently.

4. Goal-driven execution.
   - Bug fixes should reproduce the bug with a failing test, log, or minimal scenario when practical.
   - Features should define acceptance checks before implementation.
   - Refactors should verify before/after behavior and preserve public contracts unless the task explicitly changes them.

5. Evidence-backed docs.
   - Documentation changes must be based on file reads, source references, or canonical docs, not memory.
   - Root docs should link to detailed manuals instead of duplicating route tables, env matrices, Docker recipes, plugin specs, or release procedures.

## 5. Dev Setup, Runtime Services, and Lockfile Policy

Default local setup:

```sh
pnpm install
pnpm dev
```

This starts the API and UI at `http://localhost:3100` in the normal integrated dev flow.

Quick checks:

```sh
curl http://localhost:3100/api/health
curl http://localhost:3100/api/companies
```

Database defaults:

- Leave `DATABASE_URL` unset for the local embedded PostgreSQL development path.
- Public authenticated deployments must use a real `postgres`/`postgresql` `DATABASE_URL` and the required auth/secrets configuration; server startup refuses unsafe embedded fallback in that context.
- Read `doc/DATABASE.md` before changing DB runtime, migration, or deployment behavior.

Runtime-service discipline:

- Prefer the repo's managed dev/runtime service controls for reusable preview services instead of unmanaged long-running background processes.
- Relevant root commands include `pnpm dev:list` and `pnpm dev:stop`.
- `scripts/dev-service.ts` is the local dev-service anchor.

Lockfile policy:

- GitHub Actions owns `pnpm-lock.yaml` refreshes.
- Do not commit `pnpm-lock.yaml` in ordinary PRs.
- Manifest changes are validated in CI and the lockfile is refreshed on `master` by the dedicated workflow.
- `pnpm-workspace.yaml` intentionally excludes standalone sandbox provider/example dependency trees to reduce lockfile churn.

## 6. Core Engineering Rules

1. Keep changes company-scoped.
   Every domain entity should be scoped to a company, and company boundaries must be enforced in routes, services, joins, UI state, and CLI/API interactions.

2. Keep contracts synchronized.
   If you change schema/API behavior, update all impacted layers:
   - `packages/db` schema, migrations, and exports
   - `packages/shared` types/constants/validators
   - `server` routes/services/OpenAPI/error behavior
   - `ui` API clients, query keys, routes, and pages
   - `cli` command/client expectations
   - `docs/` or `doc/` documentation when behavior or commands change

3. Preserve control-plane invariants.
   - Single-assignee task model
   - Atomic issue checkout semantics
   - Approval gates for governed actions
   - Budget hard-stop auto-pause behavior
   - Activity logging for mutating actions
   - Company/session/run isolation
   - Low-trust boundaries for agents, mentions, task bridges, and plugin/runtime surfaces

4. Do not replace strategic docs wholesale unless explicitly asked.
   Prefer additive, source-backed updates. Keep `doc/SPEC.md` and `doc/SPEC-implementation.md` aligned when product scope changes.

5. Keep repo plan docs dated and centralized.
   When creating a plan file in the repository itself, new plan documents belong in `doc/plans/` and should use `YYYY-MM-DD-slug.md` filenames. This does not replace Paperclip issue planning: if a Paperclip issue asks for a plan, update the issue `plan` document per the `paperclip` skill instead of creating a repo markdown file.

6. Attach inspectable generated artifacts.
   When a task produces a user-inspectable deliverable file, follow the Paperclip skill's Generated Artifacts and Work Products workflow before final disposition. Prefer `skills/paperclip/scripts/paperclip-upload-artifact.sh` so the file is available through the Paperclip API, create or update an artifact work product when the file is the deliverable, link the uploaded artifact in the final issue comment, and then set status. Do not rely on local filesystem paths as the only access path. If an important file intentionally remains workspace-only, create or update a work product with `metadata.resourceRef.kind: "workspace_file"` and a workspace-relative path, then name that work product and path in the final comment. See `doc/AGENT-ARTIFACTS.md` and `skills/paperclip/references/artifacts.md`.

## 7. Server, API, Auth, and Database Expectations

Server composition:

- `server/src/app.ts` is the authoritative Express composition and route mount order.
- `server/src/routes/index.ts` is a re-export surface; do not assume adding an export there registers a route.
- Global middleware includes JSON raw-body capture, HTTP logging, private hostname guard, actor middleware, auth routes, board mutation guard on `/api`, plugin UI routes, optional UI/static or Vite fallback, and `errorHandler`.

Auth/authz:

- `server/src/middleware/auth.ts` normalizes `req.actor`.
- `server/src/routes/authz.ts` contains route-level helpers such as `assertAuthenticated`, `assertBoard`, `assertBoardOrAgent`, `assertInstanceAdmin`, and `assertCompanyAccess`.
- `server/src/services/authorization.ts` contains fine-grained authorization decisions, including explicit permission grants, task assignment scopes, task bridge restrictions, low-trust boundaries, mention-scoped grants, manager-chain allowances, and legacy compatibility.
- Agent API keys are hashed at rest and must not cross company boundaries.
- Board access is full-control operator context but still passes through route guards and mutation safety.

Routes and errors:

- Base path: `/api`.
- Validate inputs with Zod schemas via `server/src/middleware/validate.ts` or inline parsing.
- Throw `HttpError` helpers from `server/src/errors.ts` for expected errors.
- `server/src/middleware/error-handler.ts` standardizes JSON errors and maps Zod errors to 400s.
- New endpoints should return consistent HTTP errors: `400/401/403/404/409/422/500`.
- `server/src/routes/openapi.ts` is manually maintained; public REST contract changes may require OpenAPI/schema updates.

Database and migrations:

- Edit schema in `packages/db/src/schema/*.ts` and export new tables from `packages/db/src/schema/index.ts`.
- DB access should use injected `Db` dependencies and table objects from `@paperclipai/db`; do not construct ad-hoc connections inside route code.
- Query joins must include `companyId` predicates where company isolation is relevant.
- Migrations live under `packages/db/src/migrations` with Drizzle metadata. Do not casually hand-edit migration journal metadata.
- `server/src/index.ts` checks pending migrations at startup, can prompt or auto-apply, repairs some drifted migration journal entries, refuses stale schema when not approved, and enforces public authenticated deployment DB safety.

When changing data model:

```sh
pnpm db:generate
pnpm -r typecheck
```

Notes:

- `packages/db/drizzle.config.ts` reads compiled schema from `dist/schema/*.js`.
- `pnpm db:generate` compiles `packages/db` first.

## 8. Orchestration, Routines, Workspaces, Secrets, and Plugins

Key orchestration areas include:

- Issues/tasks/runs: `server/src/routes/issues.ts`, run/task services, heartbeat processing, and activity logs.
- Routines: `server/src/routes/routines.ts` and `server/src/services/routines.ts`.
- Approvals and governed actions: `server/src/routes/approvals.ts` plus related services.
- Costs and budgets: `server/src/routes/costs.ts`, `server/src/services/budgets.ts`, and budget hard-stop behavior.
- Secrets: `server/src/routes/secrets.ts` and `server/src/services/secrets.ts`; never expose secret values in logs, docs, comments, or public PRs.
- Execution workspaces and runtime services: `server/src/routes/execution-workspaces.ts`, `server/src/services/workspace-runtime.ts`, and `server/src/services/environment-run-orchestrator.ts`.
- Plugins: `server/src/routes/plugins.ts`, `server/src/services/plugin-host-services.ts`, `doc/plugins/*`, and `packages/plugins/*`.
- OpenClaw/Hermes gateway onboarding: `doc/OPENCLAW_ONBOARDING.md`, `doc/HERMES_GATEWAY_ONBOARDING.md`, and related adapter/gateway docs.

Rules:

- Governed actions, broad trust changes, secret handling, budget changes, destructive migrations, production releases, and cross-company data access need explicit approval.
- Third-party adapters/plugins/skills are untrusted until inspected; prefer least privilege and sandboxing.
- Keep built-in adapter behavior, external adapter overrides, plugin manifests, UI parsers, and docs synchronized.

## 9. UI, CLI, Packages, Adapters, and Catalogs

UI:

- `ui/` is a React + Vite single-page app served by the API in integrated dev.
- Keep routes, navigation, query keys, and API clients aligned with server/shared contracts.
- Use company selection context for company-scoped pages.
- Surface failures clearly; do not silently ignore API errors.
- For UI component/page/styling work, use `.claude/skills/design-guide/SKILL.md` when applicable.
- When adding reusable UI components, update `ui/src/pages/DesignGuide.tsx` and `.claude/skills/design-guide/references/component-index.md` when the component belongs in the design guide.
- Use Storybook commands only when the change touches component docs or visual states.

CLI:

- `cli/` is the Paperclip command-line surface and should stay aligned with public API/auth/config contracts.
- Update CLI docs in `docs/cli/` and package README when commands or flags change.
- Do not hardcode environment-specific server URLs or secrets in CLI examples.

Shared packages:

- `packages/shared` is the cross-layer contract surface; update it with DB/API/UI/CLI behavior changes.
- `packages/db` owns schema, migrations, clients, migration status, and embedded DB runtime helpers.
- Catalog packages (`packages/teams-catalog`, `packages/skills-catalog`) contain generated/distributable content; distinguish source catalogs from runtime-generated artifacts.

Adapters and plugins:

- Adapter authoring details belong in `packages/adapters/AUTHORING.md` and `docs/adapters/*`.
- Plugin authoring details belong in `doc/plugins/*`, `packages/plugins/sdk/README.md`, and package-local docs.
- Root docs should summarize trust, contract-sync, and verification expectations; do not duplicate full manifest schemas or adapter implementation recipes.
- Sandbox provider packages may have standalone dependencies intentionally excluded from the root lockfile.

## 10. Docs, Skills, Templates, and Artifact Hygiene

Docs:

- Public docs changes under `docs/` must keep `docs/docs.json` navigation accurate.
- Internal docs changes under `doc/` should link to canonical product/developer/ops docs instead of duplicating them in root files.
- Internal architecture and the record system: [`doc/index.md`](doc/index.md) maps [`doc/architecture/`](doc/architecture/index.md), [`doc/plans/`](doc/plans/README.md), [`doc/issues/`](doc/issues/README.md), [`doc/learning/`](doc/learning/README.md), and [`doc/practices/`](doc/practices/README.md); follow [`references/repo-records.md`](references/repo-records.md) and keep the relevant architecture doc current when a subsystem's structure changes.
- Keep stale-doc discoveries as explicit follow-up notes unless the task includes fixing them.

Skills and templates:

- `skills/paperclip/SKILL.md` is the operational guide for Paperclip API coordination.
- `skills/paperclip-board/SKILL.md`, `skills/paperclip-create-agent/SKILL.md`, `.claude/skills/design-guide/SKILL.md`, and `.agents/skills/**/SKILL.md` are workflow or distributable skill surfaces; follow relevant skills when invoked.
- Do not rewrite shipped skill/template examples unless the task explicitly targets them.
- Generated output should go to the target workspace/artifact path, not back into source templates.

Artifacts and screenshots:

- Use `doc/AGENT-ARTIFACTS.md` and `skills/paperclip/references/artifacts.md` for work-product handling.
- Screenshot directories are evidence assets unless linked from docs or uploaded as artifacts.
- Keep public artifact/screenshot filenames free of internal issue IDs, private hostnames, tailnet URLs, or agent-run identifiers.

## 11. Verification and Testing

Start with the smallest relevant verification command. Do not default to repo-wide typecheck/build/test on every heartbeat when a narrower check proves the change.

Cheap default:

```sh
pnpm test
```

PR-ready or broad change check:

```sh
pnpm -r typecheck
pnpm test:run
pnpm build
```

Browser and release suites are opt-in unless touched or explicitly requested:

```sh
pnpm test:e2e
pnpm test:e2e:multiuser-authenticated
pnpm test:release-smoke
```

Prompt/eval checks:

- Use `evals/promptfoo/` and the root promptfoo scripts when changes affect heartbeat prompts, governance prompts, agent behavioral contracts, or eval-covered prompt surfaces.
- Record eval outputs/artifacts in the relevant report or work product path.

Server tests:

- Server Vitest config runs node tests with isolated forks and low concurrency; respect that when debugging server tests.
- Supertest setup patches loopback server addresses in `server/src/__tests__/setup-supertest.ts`.

Docs-only changes:

- Run Markdown/link/grep sanity checks when available.
- Runtime tests are usually not required for docs-only changes; state `Not run: docs-only change` when you skip them.

Always report exactly what was run, what passed, and what was skipped with the reason.

## 12. Operational Scripts, Release, Docker, and Deployment

Operational scripts:

- `scripts/release.sh`, `scripts/build-npm.sh`, release note tooling, and package publication helpers are stateful; do not run them unless explicitly requested.
- Docker/onboarding smoke scripts, OpenClaw/Hermes smokes, publish tooling, and destructive migration scripts are not routine verification commands.
- Security/helper scripts such as forbidden-token and no-git-push checks are evidence sources for policy wording; do not bypass them in CI-facing work.

Docker/deployment:

- Root `Dockerfile` builds production assets and installs agent CLIs in the runtime image.
- `docker/docker-compose.quickstart.yml` is the quickstart compose path; do not invent a root `docker-compose.yml` for docs.
- `docker/docker-compose.yml`, untrusted-review compose files, quadlet, ECS, runtime images, and deployment docs are detailed references; link to them instead of copying recipes.
- Deployment docs live in `docs/deploy/` and `doc/DOCKER.md`.

Release docs:

- Release and publishing details belong in `doc/RELEASING.md`, `doc/PUBLISHING.md`, release automation docs, scripts, and `.github/workflows/release*.yml`.
- Do not include npm dist-tag retry logic, full env matrices, or every release-script branch in root docs.

## 13. Pull Request and Public-Safety Requirements

Before creating a PR:

- Search GitHub for duplicate/in-flight PRs and related issues.
- Review `CONTRIBUTING.md` and `.github/PULL_REQUEST_TEMPLATE.md`.
- Use descriptive branch names without internal ticket IDs.
- Do not put internal Paperclip issue IDs, private URLs, localhost/tailnet links, `agent://` links, secrets, or instance-local identifiers in public PR titles, bodies, commits, comments, or artifact filenames.

When creating a pull request, read and fill in every section of `.github/PULL_REQUEST_TEMPLATE.md`. Required sections include:

- Thinking Path
- What Changed
- Verification
- Risks
- Model Used
- Checklist

Security vulnerabilities must be reported via GitHub Security Advisories, not public issues.

## 14. Definition of Done

A change is done when all are true:

1. Behavior matches `doc/SPEC-implementation.md` or the task's explicitly narrowed scope.
2. Company scoping, authz, budgets, approvals, activity logs, and low-trust boundaries are preserved.
3. Contracts are synced across db/shared/server/ui/cli/docs where affected.
4. Relevant verification has passed, or skipped checks are explicitly named with reasons.
5. Docs are updated when behavior, commands, public contracts, deployment requirements, or workflow expectations change.
6. Generated deliverables are uploaded or recorded through the Paperclip artifact/work-product workflow when they are user-inspectable outputs.
7. PR description follows `.github/PULL_REQUEST_TEMPLATE.md` when a PR is created.
8. No secrets, private URLs, internal IDs, or unrelated company/session context are exposed.

## 15. Fork-Specific and Local-Branch Notes

Fork/local notes are not universal upstream guidance. Verify the current branch and package state before applying them outside this checkout.

- Default working branch: on this host/fork the maintained default branch is `franky`. Unless the maintainer names another branch, "merge", "commit", and "push" mean on `franky`. Do not commit or push to `master` or the upstream default branch without an explicit instruction.
- This checkout may carry QoL patches and built-in adapter stories that differ from upstream docs.
- Keep Hermes/OpenClaw gateway wording aligned with current adapter IDs and onboarding docs; do not preserve stale generic `adapterType=openclaw` examples when current docs require gateway-specific IDs.
- Older branches may document plugin-only Hermes; current built-ins and override behavior must be verified against the branch being edited.
- Port, NTFS, cache, and process-kill workarounds are local troubleshooting notes. Do not copy them into public docs or make them default instructions for all contributors.
- If re-copying upstream source into a fork with local UI patches, re-check local diffs such as transcript grouping and dashboard excerpt behavior before overwriting.
