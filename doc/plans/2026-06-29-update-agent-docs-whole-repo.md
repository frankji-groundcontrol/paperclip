# Update Agent Docs from Whole-Repo Evidence Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Update `AGENTS.md` and create a thin `CLAUDE.md` so Paperclip's agent guidance reflects direct whole-repo evidence, integrates transferable guidance from `frankji-groundcontrol/franky-frank` and `multica-ai/andrej-karpathy-skills`, and avoids generic scaffolded docs.

**Architecture:** Keep `AGENTS.md` as the authoritative cross-agent contributor guide. Add `CLAUDE.md` only as a Claude-specific entry point that points back to `AGENTS.md`, records Claude workflow defaults, and does not duplicate domain manuals. Use repo-relative links and concise rules; reference canonical domain docs instead of copying full server, API, deployment, plugin, or release manuals.

**Tech Stack:** Markdown docs in a pnpm 9.15 / Node >=20 TypeScript monorepo with Express, React/Vite, Drizzle, Vitest, Playwright, promptfoo, Docker, adapter/plugin packages, project skills, and Paperclip control-plane artifact workflows.

---

## Source Evidence Behind This Plan

This plan is not a scaffold. It is based on these completed read-only audits:

- Local root/config/docs audit: `AGENTS.md`, `README.md`, `CONTRIBUTING.md`, `SECURITY.md`, `ROADMAP.md`, `package.json`, `pnpm-workspace.yaml`, `tsconfig*.json`, `vitest.config.ts`, `.github/**`, `Dockerfile`, `.env.example`, `.npmrc`, `.gitignore`.
- Whole-repo domain audit: `server/`, `ui/`, `cli/`, `packages/shared`, `packages/db`, `packages/adapters`, `packages/plugins`, `packages/teams-catalog`, `packages/skills-catalog`, `packages/mcp-server`, `skills/`, `.agents/`, `doc/`, `docs/`, `tests/`, `scripts/`, `docker/`, `evals/`, `tools/`, `releases/`, `patches/`, `report/`, screenshots directories, and `.claude/`.
- Follow-up server audit anchors: `server/src/app.ts`, `server/src/middleware/auth.ts`, `server/src/routes/authz.ts`, `server/src/services/authorization.ts`, `server/src/errors.ts`, `server/src/middleware/error-handler.ts`, `server/src/middleware/validate.ts`, `server/src/routes/openapi.ts`, `server/src/index.ts`, orchestration routes/services, and DB runtime/migration files.
- Follow-up docs/testing/ops audits: `doc/GOAL.md`, `doc/PRODUCT.md`, `doc/SPEC-implementation.md`, `doc/DEVELOPING.md`, `doc/DATABASE.md`, `doc/DOCKER.md`, release/publishing docs, plugin and adapter docs, `docs/docs.json`, `tests/e2e/**`, `tests/release-smoke/**`, `evals/promptfoo/**`, operational scripts, Docker compose files, runtime service scripts, artifacts docs, and project skills.
- External source audit: `frankji-groundcontrol/franky-frank` README and `_meta` knowledge-base docs; `multica-ai/andrej-karpathy-skills` `README.md`, `CLAUDE.md`, and `skills/karpathy-guidelines/SKILL.md`.

Important audit conclusions:

- `AGENTS.md` exists and is the root contributor guide; `CLAUDE.md` does not exist yet.
- Existing `AGENTS.md` has stale `PGlite` wording, an incomplete repo map, duplicate `## 11` headings, and a self-referential fork note.
- Server route composition is in `server/src/app.ts`; `server/src/routes/index.ts` is a re-export surface, not the main registration point.
- Public authenticated deployments require a real `postgres`/`postgresql` `DATABASE_URL`; embedded PostgreSQL is for local/non-cloud contexts.
- `docs/` is public/user/admin documentation governed by `docs/docs.json`; `doc/` is internal developer/product/operations documentation.
- Root docs should link to detailed domain docs rather than duplicate route tables, endpoint schemas, Docker recipes, release procedures, plugin specs, or skill manuals.
- Transferable external guidance should be paraphrased into Paperclip-local workflow rules: whole-repo evidence, incremental docs maintenance, copy-first doc moves, Think Before Coding, Simplicity First, Surgical Changes, and Goal-Driven Execution.

## File Structure

- Modify: `/home/frankji/Projects/paperclip/AGENTS.md`
  - Responsibility: canonical cross-agent contributor guide for this repository.
  - Change type: full-file Markdown update that preserves useful current content, removes stale wording, fixes heading numbering, adds audited repo-wide guidance, and keeps fork-only details isolated.
- Create: `/home/frankji/Projects/paperclip/CLAUDE.md`
  - Responsibility: thin Claude-specific entry point that delegates repo policy to `AGENTS.md` and records Claude workflow expectations.
- Do not modify: `doc/`, `docs/`, package docs, skills, generated catalog content, `.github/`, scripts, tests, or code in this implementation. Link to those files only.
- Do not commit unless the user explicitly asks; Claude Code session instructions require commits/pushes only on request.

---

### Task 1: Preflight and Current-State Confirmation

**Files:**
- Read: `/home/frankji/Projects/paperclip/AGENTS.md`
- Check: `/home/frankji/Projects/paperclip/CLAUDE.md`

- [ ] **Step 1: Confirm branch and working tree state**

Run:

```bash
git -C /home/frankji/Projects/paperclip status --short --branch
```

Expected:

- Current branch should be the user's maintained branch, usually `franky` on this host.
- If uncommitted changes exist, inspect them before editing and avoid overwriting unrelated work.

- [ ] **Step 2: Confirm target file existence**

Run:

```bash
test -f /home/frankji/Projects/paperclip/AGENTS.md && printf 'AGENTS.md exists\n'
test -e /home/frankji/Projects/paperclip/CLAUDE.md && printf 'CLAUDE.md exists\n' || printf 'CLAUDE.md missing\n'
```

Expected:

```text
AGENTS.md exists
CLAUDE.md missing
```

If `CLAUDE.md` exists, read it before editing and merge the target content from Task 3 instead of blindly overwriting.

- [ ] **Step 3: Re-read existing `AGENTS.md` before editing**

Use the file read tool on:

```text
/home/frankji/Projects/paperclip/AGENTS.md
```

Expected observations from the current file:

- It starts with `# AGENTS.md`.
- It uses `Use embedded PGlite in dev by leaving DATABASE_URL unset`, which is stale.
- It has duplicate `## 11` headings.
- It has a fork-specific section ending with `See root AGENTS.md for full details`, which is self-referential.

---

### Task 2: Replace `AGENTS.md` with Whole-Repo Grounded Root Guidance

**Files:**
- Modify: `/home/frankji/Projects/paperclip/AGENTS.md`

- [ ] **Step 1: Replace the file content with this complete Markdown**

Write this exact content to `/home/frankji/Projects/paperclip/AGENTS.md`:

````markdown
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

- This checkout may carry QoL patches and built-in adapter stories that differ from upstream docs.
- Keep Hermes/OpenClaw gateway wording aligned with current adapter IDs and onboarding docs; do not preserve stale generic `adapterType=openclaw` examples when current docs require gateway-specific IDs.
- Older branches may document plugin-only Hermes; current built-ins and override behavior must be verified against the branch being edited.
- Port, NTFS, cache, and process-kill workarounds are local troubleshooting notes. Do not copy them into public docs or make them default instructions for all contributors.
- If re-copying upstream source into a fork with local UI patches, re-check local diffs such as transcript grouping and dashboard excerpt behavior before overwriting.
````

- [ ] **Step 2: Confirm stale strings and duplicate numbering are gone**

Run:

```bash
rg -n "PGlite|embedded SQLite|adapterType=openclaw|See root AGENTS.md|## 11\." /home/frankji/Projects/paperclip/AGENTS.md
```

Expected:

- No matches for `PGlite`, `embedded SQLite`, `adapterType=openclaw`, or `See root AGENTS.md`.
- No duplicate `## 11.` section; the only `## 11.` heading should be `## 11. Verification and Testing`.

---

### Task 3: Create Thin Claude-Specific `CLAUDE.md`

**Files:**
- Create: `/home/frankji/Projects/paperclip/CLAUDE.md`

- [ ] **Step 1: Write this complete file**

Write this exact content to `/home/frankji/Projects/paperclip/CLAUDE.md`:

````markdown
# CLAUDE.md

Claude-specific entry point for working in this Paperclip checkout.

## Start Here

Read `AGENTS.md` first. It is the authoritative cross-agent guide for this repository.
This file only adds Claude-specific operating defaults and must not drift into a second copy of the full repo manual.

Required first reads for non-trivial repository work:

1. `AGENTS.md`
2. `doc/GOAL.md`
3. `doc/PRODUCT.md`
4. `doc/SPEC-implementation.md`
5. `doc/DEVELOPING.md`
6. `doc/DATABASE.md`

For public docs work, also inspect `docs/docs.json` and the affected `docs/**` pages.
For internal developer/product/ops docs work, inspect the affected `doc/**` files.

## Whole-Repo Evidence Rule

Do not scaffold generic guidance for this repo. Before updating root docs, architecture guidance, or cross-cutting behavior, read the relevant repository areas and ground the change in existing files.

For broad doc updates, check the affected domains explicitly:

- root config and `.github/`
- `server/`
- `ui/`
- `cli/`
- `packages/`
- `doc/` and `docs/`
- `tests/` and `evals/`
- `scripts/`, `docker/`, `tools/`, and `releases/`
- `skills/`, `.claude/skills/`, and `.agents/`

When coverage is partial, say so and limit the change to the evidence read.

## Claude Workflow Defaults

- Use relevant skills before acting when a skill applies.
- For non-trivial implementation, plan from source evidence before editing.
- Use subagents or workflows for independent repo-wide research, then inspect and synthesize their outputs before editing.
- Dispatch is not completion: verify worker outputs, diffs, tests, and artifact links yourself.
- Keep visible task tracking current; do not finish while work is still marked in progress.
- Do not commit or push unless the user explicitly asks.

## Behavioral Guardrails

Use the local adaptation of the Karpathy-style workflow:

1. Think before coding: state assumptions and success criteria for non-trivial work.
2. Simplicity first: solve today's requirement without speculative abstractions.
3. Surgical changes: touch only lines tied to the request and match existing style.
4. Goal-driven execution: verify against the stated outcome; reproduce bugs before fixing when practical.

Ask for clarification only when ambiguity changes security, privacy, data scope, API contracts, user-visible behavior, cost, or external side effects. Otherwise, state a reasonable assumption and proceed.

## Paperclip-Specific Priorities

Keep these invariants in mind while applying `AGENTS.md`:

- Company boundaries are mandatory across server routes, services, UI state, CLI/API interactions, artifacts, and docs examples.
- Schema/API changes must stay synchronized across `packages/db`, `packages/shared`, `server`, `ui`, `cli`, and docs.
- Governed actions, approvals, budgets, activity logs, secrets, runtime workspaces, plugin trust, and low-trust agent boundaries are control-plane contracts, not optional polish.
- Public authenticated deployments require real PostgreSQL configuration and secrets; do not document embedded DB fallback as production-safe.
- Generated deliverables should go through the Paperclip artifact/work-product workflow when they are user-inspectable outputs.

## Documentation Maintenance

- `AGENTS.md` stays concise and repo-wide.
- Detailed manuals stay in `doc/`, `docs/`, package READMEs, plugin/adapter docs, or skills.
- Public docs navigation lives in `docs/docs.json`.
- For doc moves, use copy-first/link-safe migration: add the replacement, update links/navigation/sources, verify references, then remove old content only when requested.
- Do not expose internal issue IDs, private URLs, local run identifiers, secrets, tailnet links, or `agent://` links in public docs, PRs, commits, comments, or artifact filenames.
````

- [ ] **Step 2: Confirm `CLAUDE.md` delegates to `AGENTS.md` and is not a duplicate manual**

Run:

```bash
wc -l /home/frankji/Projects/paperclip/CLAUDE.md /home/frankji/Projects/paperclip/AGENTS.md
rg -n "Read `AGENTS.md` first|Do not scaffold generic guidance|Do not commit or push unless" /home/frankji/Projects/paperclip/CLAUDE.md
```

Expected:

- `CLAUDE.md` is substantially shorter than `AGENTS.md`.
- The grep command finds all three phrases.

---

### Task 4: Review Forbidden Content and Repo Evidence Coverage

**Files:**
- Review: `/home/frankji/Projects/paperclip/AGENTS.md`
- Review: `/home/frankji/Projects/paperclip/CLAUDE.md`

- [ ] **Step 1: Check required evidence anchors are present**

Run:

```bash
rg -n "whole-repo|scaffold|SPEC-implementation|docs/docs\.json|server/src/app\.ts|server/src/routes/authz\.ts|server/src/services/authorization\.ts|server/src/routes/openapi\.ts|doc/AGENT-ARTIFACTS\.md|BETTER_AUTH_SECRET|DATABASE_URL|promptfoo|test:e2e:multiuser-authenticated|Karpathy|frankji-groundcontrol|franky-frank" /home/frankji/Projects/paperclip/AGENTS.md /home/frankji/Projects/paperclip/CLAUDE.md
```

Expected:

- Matches for whole-repo/scaffold guidance.
- Matches for `SPEC-implementation`, `docs/docs.json`, server auth/OpenAPI anchors, `doc/AGENT-ARTIFACTS.md`, `DATABASE_URL`, promptfoo/e2e guidance, and Karpathy-style principles.
- `BETTER_AUTH_SECRET` and external repo names may be absent from the final docs if they would be too specific; if absent, confirm the docs still cover auth/secrets and external guidance in concise paraphrased form. Do not add names just to satisfy grep.

- [ ] **Step 2: Check forbidden or stale content is absent**

Run:

```bash
rg -n "PGlite|embedded SQLite|adapterType=openclaw|See root AGENTS.md|TODO|TBD|fill in|localhost.*public PR|tailnet|agent://" /home/frankji/Projects/paperclip/AGENTS.md /home/frankji/Projects/paperclip/CLAUDE.md
```

Expected:

- No matches.
- If `localhost` appears only in local dev instructions and not public PR guidance, that is acceptable.

- [ ] **Step 3: Check headings are unique and ordered**

Run:

```bash
rg -n "^## " /home/frankji/Projects/paperclip/AGENTS.md /home/frankji/Projects/paperclip/CLAUDE.md
```

Expected `AGENTS.md` headings:

```text
## 1. Purpose and Authority
## 2. Read This First
## 3. Repo Map
## 4. Working Style
## 5. Dev Setup, Runtime Services, and Lockfile Policy
## 6. Core Engineering Rules
## 7. Server, API, Auth, and Database Expectations
## 8. Orchestration, Routines, Workspaces, Secrets, and Plugins
## 9. UI, CLI, Packages, Adapters, and Catalogs
## 10. Docs, Skills, Templates, and Artifact Hygiene
## 11. Verification and Testing
## 12. Operational Scripts, Release, Docker, and Deployment
## 13. Pull Request and Public-Safety Requirements
## 14. Definition of Done
## 15. Fork-Specific and Local-Branch Notes
```

Expected `CLAUDE.md` headings:

```text
## Start Here
## Whole-Repo Evidence Rule
## Claude Workflow Defaults
## Behavioral Guardrails
## Paperclip-Specific Priorities
## Documentation Maintenance
```

---

### Task 5: Markdown and Diff Verification

**Files:**
- Verify: `/home/frankji/Projects/paperclip/AGENTS.md`
- Verify: `/home/frankji/Projects/paperclip/CLAUDE.md`

- [ ] **Step 1: Inspect the exact diff**

Run:

```bash
git -C /home/frankji/Projects/paperclip diff -- AGENTS.md CLAUDE.md
```

Expected:

- Only `AGENTS.md` and `CLAUDE.md` are shown.
- `AGENTS.md` changes are documentation-only.
- `CLAUDE.md` is a new documentation file.
- No secrets, private URLs, internal issue IDs, or unrelated local paths are introduced.

- [ ] **Step 2: Run markdown lint if the repo tool is available**

Run:

```bash
pnpm -C /home/frankji/Projects/paperclip exec markdownlint-cli2 AGENTS.md CLAUDE.md
```

Expected:

- PASS if `markdownlint-cli2` is available through the workspace.
- If the command fails because the tool is not installed, record the exact failure and perform manual Markdown review instead. Do not add a new dependency just for this docs change.

- [ ] **Step 3: Run docs-only status check**

Run:

```bash
git -C /home/frankji/Projects/paperclip status --short
```

Expected:

```text
 M AGENTS.md
?? CLAUDE.md
```

This plan file may also appear if it was saved in `doc/plans/`; include it in the handoff if it remains in the working tree.

- [ ] **Step 4: Decide whether runtime tests are necessary**

For this docs-only change, do not run stateful or broad runtime commands by default.

Do not run these unless explicitly requested:

```bash
/home/frankji/Projects/paperclip/scripts/release.sh
pnpm -C /home/frankji/Projects/paperclip test:e2e
pnpm -C /home/frankji/Projects/paperclip test:release-smoke
/home/frankji/Projects/paperclip/scripts/docker-onboard-smoke.sh
/home/frankji/Projects/paperclip/scripts/docker-build-test.sh
```

Expected handoff wording:

```text
Not run: runtime tests/builds; change is Markdown-only. Performed diff, heading, stale-string, and markdown lint/manual review instead.
```

---

### Task 6: Final Handoff

**Files:**
- Summarize: `/home/frankji/Projects/paperclip/AGENTS.md`
- Summarize: `/home/frankji/Projects/paperclip/CLAUDE.md`
- Summarize: `/home/frankji/Projects/paperclip/doc/plans/2026-06-29-update-agent-docs-whole-repo.md`

- [ ] **Step 1: Prepare the final summary**

Use this structure:

```markdown
Implemented docs updates grounded in the repo-wide audit.

Changed:
- `AGENTS.md`: expanded root guidance from whole-repo evidence; fixed stale embedded DB wording, repo map gaps, duplicate heading numbering, and self-referential fork note; added concise server/auth/db, orchestration, docs, testing, ops, PR, artifact, and external-principle guidance.
- `CLAUDE.md`: added a thin Claude-specific entry point that delegates to `AGENTS.md` and captures whole-repo evidence, skill/workflow, and behavioral defaults.
- `doc/plans/2026-06-29-update-agent-docs-whole-repo.md`: saved the implementation plan.

Verification:
- Ran `rg` checks for stale strings and required anchors.
- Ran heading/order sanity check.
- Ran `git diff -- AGENTS.md CLAUDE.md`.
- Ran markdown lint if available, or manually reviewed Markdown if not installed.

Not run:
- Runtime tests/builds; change is Markdown-only.
```

- [ ] **Step 2: Do not commit unless asked**

Do not run `git commit` or `git push` unless the user explicitly asks for it.

If the user asks for a commit later, use a commit message like:

```bash
git add AGENTS.md CLAUDE.md doc/plans/2026-06-29-update-agent-docs-whole-repo.md
git commit -m "docs: update agent guidance from repo audit"
```

Remember that Claude Code's commit attribution requirements apply if committing.
