# Architecture — Build, test, release & deployment

## Purpose / Overview

This subsystem is the machinery that turns the Paperclip monorepo source tree
into publishable npm packages, Docker images, and a verified running server. It
spans four concerns that share tooling:

- **Build** — compiling the workspace (`pnpm build`), bundling the CLI for npm
  (esbuild), and building the multi-stage server Docker image.
- **Test** — Vitest unit/integration suites (server, workspaces), Playwright e2e
  and release-smoke browser suites, promptfoo agent evals, and dedicated
  script-level tests.
- **Release** — the calver versioning + npm publish + git tag flow driven by
  [`scripts/release.sh`](../../scripts/release.sh), plus GitHub Actions that run
  it for canary (every push to `master`) and stable (manual dispatch).
- **Deploy** — the Docker image, Compose files (quickstart / untrusted-review),
  Podman Quadlet units, and an AWS ECS Fargate task definition.

Everything is orchestrated by the root `package.json` scripts and the workflows
under [`.github/workflows/`](../../.github/workflows). The invariant that ties
it together: **CI owns `pnpm-lock.yaml`** — contributors never commit it, and a
dedicated bot regenerates it on `master`.

For narrative operator docs see [`doc/RELEASING.md`](../../doc/RELEASING.md),
[`doc/PUBLISHING.md`](../../doc/PUBLISHING.md),
[`doc/RELEASE-AUTOMATION-SETUP.md`](../../doc/RELEASE-AUTOMATION-SETUP.md), and
[`doc/DOCKER.md`](../../doc/DOCKER.md). This doc describes how the code is wired.

## Entry points

Every task has a `pnpm` script alias in the root
[`package.json`](../../package.json). The most important ones:

| Command | Backing file | What it does |
|---------|--------------|--------------|
| `pnpm dev` / `pnpm dev:watch` | [`scripts/dev-runner.ts`](../../scripts/dev-runner.ts) | Watch-mode dev server with auto-restart on source change |
| `pnpm dev:once` | [`scripts/dev-runner.ts`](../../scripts/dev-runner.ts) | Single dev-server boot (no watch) |
| `pnpm dev:list` / `pnpm dev:stop` | [`scripts/dev-service.ts`](../../scripts/dev-service.ts) | List / terminate registered dev services for this repo |
| `pnpm build` | `preflight:workspace-links` + `pnpm -r build` | Build every workspace package |
| `pnpm typecheck` | `pnpm -r typecheck` | Typecheck every workspace |
| `pnpm typecheck:build-gaps` | [`scripts/run-typecheck-build-gaps.mjs`](../../scripts/run-typecheck-build-gaps.mjs) | Typecheck workspaces whose `build` skips `tsc` |
| `pnpm test` / `pnpm test:run` | [`scripts/run-vitest-stable.mjs`](../../scripts/run-vitest-stable.mjs) | Full Vitest run (general + serialized) |
| `pnpm test:run:general` / `:serialized` | same, with `--mode` | Group- or shard-scoped Vitest run |
| `pnpm test:e2e` | [`tests/e2e/playwright.config.ts`](../../tests/e2e/playwright.config.ts) | Playwright e2e (single-user, `local_trusted`) |
| `pnpm test:e2e:multiuser-authenticated` | [`tests/e2e/playwright-multiuser-authenticated.config.ts`](../../tests/e2e/playwright-multiuser-authenticated.config.ts) | Authenticated multi-user browser flow |
| `pnpm test:release-smoke` | [`tests/release-smoke/playwright.config.ts`](../../tests/release-smoke/playwright.config.ts) | Smoke a published Docker image end-to-end |
| `pnpm test:release-registry` | `node --test` over release scripts | Unit tests for release tooling |
| `pnpm evals:smoke` | [`evals/promptfoo/promptfooconfig.yaml`](../../evals/promptfoo/promptfooconfig.yaml) | Run promptfoo heartbeat evals |
| `pnpm build:npm` | [`scripts/build-npm.sh`](../../scripts/build-npm.sh) | Bundle the `paperclipai` CLI for publishing |
| `pnpm release:canary` / `:stable` | [`scripts/release.sh`](../../scripts/release.sh) | Version, publish to npm, and tag |
| `pnpm release:github` | [`scripts/create-github-release.sh`](../../scripts/create-github-release.sh) | Create/update the GitHub Release from notes |
| `pnpm release:rollback` | [`scripts/rollback-latest.sh`](../../scripts/rollback-latest.sh) | Repoint the `latest` dist-tag |
| `pnpm check:tokens` / `check:no-git-push` | [`scripts/check-forbidden-tokens.mjs`](../../scripts/check-forbidden-tokens.mjs), [`scripts/check-no-git-push.mjs`](../../scripts/check-no-git-push.mjs) | Static security/privacy gates |
| `pnpm smoke:*` | [`scripts/smoke/`](../../scripts/smoke) | Adapter/gateway end-to-end smoke scripts |

The workspace is pinned to `pnpm@9.15.4` and Node `>=20` (CI uses Node 24; a few
lanes use Node 20). The `postinstall` hook runs
[`scripts/link-plugin-dev-sdk.mjs`](../../scripts/link-plugin-dev-sdk.mjs), and
every build/test script front-runs `preflight:workspace-links`
([`scripts/ensure-workspace-package-links.ts`](../../scripts/ensure-workspace-package-links.ts))
to keep workspace symlinks intact.

## Key modules & responsibilities

### Build

- **`pnpm build`** = `pnpm -r build`. Each package builds itself; the server and
  UI produce `dist/` outputs. The Docker `build` stage explicitly builds
  `@paperclipai/ui`, `@paperclipai/plugin-sdk`, then `@paperclipai/server`, and
  asserts `server/dist/index.js` exists (see the root
  [`Dockerfile`](../../Dockerfile)).
- **CLI npm bundle** — [`scripts/build-npm.sh`](../../scripts/build-npm.sh)
  bundles `cli/` with esbuild into `cli/dist/index.js`, runs the forbidden-token
  check, typechecks (`pnpm -r typecheck`), `node --check`s the bundled
  entrypoint, backs up `cli/package.json` → `cli/package.dev.json`, and
  generates a publishable `package.json` via
  [`scripts/generate-npm-package-json.mjs`](../../scripts/generate-npm-package-json.mjs).
  It accepts `--skip-checks` and `--skip-typecheck` (the release flow passes both
  because it has already verified).
- **Standalone public packages** —
  [`scripts/build-standalone-public-packages.mjs`](../../scripts/build-standalone-public-packages.mjs)
  prepares each publishable workspace package for standalone install.
- **Server UI dist** —
  [`scripts/prepare-server-ui-dist.sh`](../../scripts/prepare-server-ui-dist.sh)
  builds the UI and copies `ui/dist` → `server/ui-dist` so the published
  `@paperclipai/server` can serve the SPA statically. Honors
  `PAPERCLIP_RELEASE_REUSE_UI_DIST=1` to avoid rebuilding the UI twice in one
  release.

### Release tooling

- **[`scripts/release.sh`](../../scripts/release.sh)** — the orchestrator. Takes
  `<canary|stable>` plus `--date`, `--dry-run`, `--skip-verify`,
  `--print-version`. Seven steps: verification gate, workspace build, version
  rewrite, CLI bundle, publish (or dry-run preview), npm/registry convergence
  verification, git tag.
- **[`scripts/release-lib.sh`](../../scripts/release-lib.sh)** — the shared bash
  library: remote resolution (`RELEASE_REMOTE`/`PUBLISH_REMOTE`, else
  `public-gh` → `public` → `origin`), calver computation, npm publish with
  Sigstore transparency-log duplicate handling (`publish_package_to_npm`),
  registry-state polling, and preflight guards
  (`require_clean_worktree`, `require_on_master_branch`,
  `require_npm_publish_auth`).
- **[`scripts/release-package-map.mjs`](../../scripts/release-package-map.mjs)**
  — the single source of truth for *which* packages publish. `list` /
  `set-version` / `check` subcommands. Reconciles discovered public packages
  against
  [`scripts/release-package-manifest.json`](../../scripts/release-package-manifest.json)
  (every public package must appear with `publishFromCi: true|false`), rewrites
  `workspace:` deps to the calver version, topologically sorts by dependency
  order, and rewrites the CLI version string in `cli/src/index.ts`. It fails the
  release if a `publishFromCi:true` package depends on a `publishFromCi:false`
  one (that dep's rewritten version would never exist on npm).
- **[`scripts/verify-release-registry-state.mjs`](../../scripts/verify-release-registry-state.mjs)**
  — post-publish check that npm dist-tags and package metadata converged to the
  target version. Exit code `2` means "wrong / needs operator intervention"
  (release fails immediately, no retry).
- **[`scripts/create-github-release.sh`](../../scripts/create-github-release.sh)**
  — after the stable tag is pushed, creates/updates the GitHub Release from
  `releases/v<version>.md`.
- **[`scripts/rollback-latest.sh`](../../scripts/rollback-latest.sh)** — repoints
  the `latest` dist-tag for every public package to a prior version (does not
  unpublish).

### Test orchestration

- **[`scripts/run-vitest-stable.mjs`](../../scripts/run-vitest-stable.mjs)** —
  the Vitest driver. Partitions suites into **general** and **serialized**
  modes and three general groups: `general-server`, `general-workspaces-a`
  (`@paperclipai/ui`, `paperclipai`), `general-workspaces-b` (everything else).
  Route/authz server suites (matched by `[...]route/routes/authz[...].test.ts`
  plus an explicit `additionalSerializedServerTests` set) run in a dedicated
  **serialized** lane. Supports `--mode`, `--group`, `--shard-index`,
  `--shard-count`, `--dry-run`. Each Vitest invocation runs in a throwaway
  `PAPERCLIP_HOME`/`TMPDIR` under a per-run temp dir with `NODE_ENV=test`.
  Sharding matters because the server Vitest config pins `maxWorkers=1`, so the
  only way to parallelize the server suite is across CI runners.
- **Playwright configs** — three e2e configs plus a release-smoke config, each
  described under [Testing](#testing).

### Security / privacy static gates

- **[`scripts/check-no-git-push.mjs`](../../scripts/check-no-git-push.mjs)** —
  rejects `git push` (and args-array equivalents) inside adapter/runtime source
  (`packages/adapters`, `packages/adapter-utils`, `server/src`, `cli/src`). The
  invariant: adapter/runtime code must never push to a git remote; the local
  execution-workspace cwd is the only persistence boundary between runs. Opt-out
  requires a `paperclip:allow-git-push: <reason>` marker.
- **[`scripts/check-forbidden-tokens.mjs`](../../scripts/check-forbidden-tokens.mjs)**
  — `git grep`s the tree for forbidden tokens (from
  `.git/hooks/forbidden-tokens.txt` plus the active local username) before an npm
  publish, so contributor identities / secrets can't leak into a published
  bundle. Skips `pnpm-lock.yaml` and `.git`.
- **[`scripts/check-docker-deps-stage.mjs`](../../scripts/check-docker-deps-stage.mjs)**
  — verifies the Dockerfile `deps` stage `COPY`s every workspace `package.json`
  (and `patches/`), so a new workspace can't silently break the frozen-lockfile
  install in the image.

### Docker & deploy assets

- Root [`Dockerfile`](../../Dockerfile) — the production server image.
- [`scripts/docker-entrypoint.sh`](../../scripts/docker-entrypoint.sh) — remaps
  the `node` user UID/GID at runtime (or execs directly when unprivileged, e.g.
  Kubernetes restricted PodSecurity / OpenShift).
- Compose files, Quadlet units, and the ECS task definition under
  [`docker/`](../../docker), covered under
  [Docker images & deployment topologies](#docker-images--deployment-topologies).

## Data flow / execution lifecycle

### Release flow (canary and stable)

`scripts/release.sh <channel>` executes:

1. **Resolve & fetch remote**, read current branch/SHA, last stable tag, and the
   list of public packages via `release-package-map.mjs list`.
2. **Compute target version.** Stable is `YYYY.MDD.P` — UTC year, month (no zero
   pad), zero-padded day, and a same-day patch slot `P` derived by querying npm
   for the highest existing slot. Canary appends `-canary.N` (next N from npm)
   and uses git tag `canary/vYYYY.MDD.P-canary.N`; stable uses `vYYYY.MDD.P`.
   `--print-version` prints and exits here.
3. **Preflight guards** — `require_clean_worktree`, `require_npm_publish_auth`
   (npm login, or GitHub Actions trusted publishing), stable requires
   `releases/v<version>.md` to already exist, canary requires branch `master`,
   and the target git tag / npm versions must not already exist.
4. **Step 1/7 verification gate** (skipped by `--skip-verify`): `pnpm -r
   typecheck`, `pnpm test:run`, `pnpm build`.
5. **Step 2/7 build artifacts**: `pnpm build`,
   `build-standalone-public-packages.mjs`, `prepare-server-ui-dist.sh`, and copy
   the top-level `skills/` into `server`, `claude-local`, and `codex-local`.
6. **Step 3/7 version rewrite**: `release-package-map.mjs set-version <version>`
   (also rewrites `workspace:` deps and the CLI version).
7. **Step 4/7 CLI bundle**: `build-npm.sh --skip-checks --skip-typecheck`, then
   assert `cli/package.json` version equals the target (drift guard).
8. **Step 5/7 publish**: for each versioned package (topological order) run
   `publish_package_to_npm <dist-tag>` — dry-run only prints `pnpm publish
   --dry-run`.
9. **Step 6/7 verify**: poll npm until each version is visible
   (`wait_for_npm_package_version`) then confirm dist-tag + metadata convergence
   (`verify-release-registry-state.mjs`).
10. **Step 7/7 tag**: `git tag <tag> <original SHA>` — **tags always point at the
    source commit, never a generated release commit.**

The `EXIT` trap (`cleanup_release_state`) restores the working tree: it moves
`cli/package.dev.json` back, removes generated `server/ui-dist` and copied
`skills/`, and reverts tracked/untracked changes so the version rewrite is never
committed.

Pushing the tag and creating the GitHub Release are separate manual/CI steps
(`git push <remote> refs/tags/<tag>` then `create-github-release.sh`).

### CI release lifecycle

[`.github/workflows/release.yml`](../../.github/workflows/release.yml):

- **On push to `master`** — `verify_canary` (manifest check, install,
  `pnpm -r typecheck`, `pnpm test:run`, `pnpm build`) → `publish_canary`
  (`environment: npm-canary`, `id-token: write` for trusted publishing) runs
  `release.sh canary --skip-verify` then pushes the `canary/v*` tag to `origin`.
- **On `workflow_dispatch`** — `verify_stable` (on the chosen `source_ref`) →
  either `preview_stable` (`release.sh stable --skip-verify --dry-run`) or
  `publish_stable` (`environment: npm-stable`, `id-token: write`) which runs
  `release.sh stable --skip-verify`, pushes the `v*` tag to `origin`, and runs
  `create-github-release.sh`. Both stable jobs pass `--skip-verify` because
  `verify_stable` already ran the gate. The required dispatch inputs are
  `source_ref` (default `master`) and the `dry_run` boolean; the optional
  `stable_date` input is a **UTC date** (e.g. `2026-03-18`) forwarded as
  `--date`, which `release.sh` resolves to a calver — not a version string. When
  omitted, the release uses today's UTC date.

Concurrency group `release-<event>-<ref>` with `cancel-in-progress: false` so a
publish is never cancelled mid-flight.

### Dev-server lifecycle

`pnpm dev` runs `dev-runner.ts` under the server package's `tsx` context. It
watches `cli`, `scripts`, `server`, several `packages/*`, and root config files;
on change it gracefully restarts the server. It registers itself with the local
service supervisor so `pnpm dev:list` / `pnpm dev:stop`
([`scripts/dev-service.ts`](../../scripts/dev-service.ts)) can enumerate and
terminate it. When run inside a linked git worktree it requires a bootstrapped
worktree env (`paperclipai worktree init`).

## Testing

### Vitest (unit / integration)

Driven by `run-vitest-stable.mjs`. In CI
([`.github/workflows/pr.yml`](../../.github/workflows/pr.yml)):

- `general_tests` matrix runs `general-server` (sharded 3×), `general-workspaces-a`,
  and `general-workspaces-b` via `pnpm test:run:general -- --group ...`.
- `verify_serialized_server` matrix runs the route/authz serialized suites
  sharded 4× via `pnpm test:run:serialized`.
- Partition helpers are themselves unit-tested
  ([`scripts/__tests__/run-vitest-stable-shard.test.mjs`](../../scripts/__tests__/run-vitest-stable-shard.test.mjs),
  [`scripts/__tests__/build-standalone-concurrency.test.mjs`](../../scripts/__tests__/build-standalone-concurrency.test.mjs)).

### Playwright e2e — [`tests/e2e/`](../../tests/e2e)

- **Default suite** ([`playwright.config.ts`](../../tests/e2e/playwright.config.ts))
  — `workers: 1`, boots a throwaway instance via `pnpm paperclipai onboard --yes
  --run` on port `3199` in `local_trusted` mode with a fresh temp
  `PAPERCLIP_HOME`, `reuseExistingServer: false`. `testIgnore`s the multi-user
  specs. Set `PAPERCLIP_E2E_SKIP_LLM=true` to skip LLM-dependent assertions and
  `PAPERCLIP_PLAYWRIGHT_CHANNEL=chrome` to use the runner's Chrome.
- **Multi-user (unauthenticated)**
  ([`playwright-multiuser.config.ts`](../../tests/e2e/playwright-multiuser.config.ts))
  — port `3104`, **no `webServer`**; expects an already-running server.
- **Multi-user authenticated**
  ([`playwright-multiuser-authenticated.config.ts`](../../tests/e2e/playwright-multiuser-authenticated.config.ts),
  [`multi-user-authenticated.spec.ts`](../../tests/e2e/multi-user-authenticated.spec.ts))
  — port `3105`, drives the `better-auth` sign-up / invite / membership flow;
  mints a bootstrap invite via
  `packages/db/scripts/create-auth-bootstrap-invite.ts`.
- The `e2e` job in `pr.yml` generates a `local_trusted`, `embedded-postgres`
  config on the runner and runs `pnpm run test:e2e`. The standalone
  [`e2e.yml`](../../.github/workflows/e2e.yml) is `workflow_dispatch`-only and
  can run the LLM-dependent assertions with a real `ANTHROPIC_API_KEY`.

### Release smoke — [`tests/release-smoke/`](../../tests/release-smoke)

[`docker-auth-onboarding.spec.ts`](../../tests/release-smoke/docker-auth-onboarding.spec.ts)
signs into a **published** Docker image, completes the onboarding wizard, and
asserts the CEO agent, task assignment, and first heartbeat run appear through
the real API. Base URL / credentials come from
`PAPERCLIP_RELEASE_SMOKE_*` env vars.
[`.github/workflows/release-smoke.yml`](../../.github/workflows/release-smoke.yml)
launches the harness with `docker-onboard-smoke.sh` (`SMOKE_DETACH=true`,
metadata written to a file) against the `canary` or `latest` dist-tag, then runs
`pnpm run test:release-smoke`. It's `workflow_dispatch` and `workflow_call`.

### Docker smoke scripts

- **[`scripts/docker-onboard-smoke.sh`](../../scripts/docker-onboard-smoke.sh)** —
  builds [`docker/Dockerfile.onboard-smoke`](../../docker/Dockerfile.onboard-smoke)
  (which runs `npx paperclipai@<version> onboard`), runs it, waits for
  `/api/health`, and (for authenticated mode) auto-bootstraps an admin via
  `sign-up/email` + `auth bootstrap-ceo` invite acceptance, verifying the board
  session and `/api/companies`. Emits a metadata env-file for the smoke
  Playwright suite.
- **[`scripts/docker-build-test.sh`](../../scripts/docker-build-test.sh)** —
  builds the `production` target with docker or podman and verifies key binaries
  (`node`, `git`, `gh`, `rg`, `python3`, `curl`) inside the image. Skips
  gracefully when no runtime is available.

### Evals — [`evals/promptfoo/`](../../evals/promptfoo)

`pnpm evals:smoke` runs `promptfoo@0.103.3 eval` against
[`promptfooconfig.yaml`](../../evals/promptfoo/promptfooconfig.yaml): the
heartbeat system prompt
([`prompts/heartbeat-system.txt`](../../evals/promptfoo/prompts/heartbeat-system.txt))
across four OpenRouter providers, with deterministic assertions in
[`tests/core.yaml`](../../evals/promptfoo/tests/core.yaml) and
[`tests/governance.yaml`](../../evals/promptfoo/tests/governance.yaml) (assignment
pickup, progress updates, blocked reporting, clean exit, checkout-before-work,
409 handling, approval-required, company-boundary refusal). Requires
`OPENROUTER_API_KEY` (or per-provider keys); see
[`evals/README.md`](../../evals/README.md).

### Adapter / gateway smoke — [`scripts/smoke/`](../../scripts/smoke)

Bash/Node end-to-end smoke scripts exposed as `pnpm smoke:*`: Hermes gateway
join/e2e, OpenClaw join / docker-ui / SSE-standalone, pipelines tutorial, and a
terminal-bench loop-skill smoke. The Hermes smoke has a matching
`node --test` suite
([`scripts/smoke/hermes-gateway-smoke.test.mjs`](../../scripts/smoke/hermes-gateway-smoke.test.mjs),
`pnpm test:hermes-gateway-smoke`).

### Release-tooling unit tests

`pnpm test:release-registry` runs `node --test` over the release scripts
(`verify-release-registry-state`, `release-package-map`,
`check-release-package-bootstrap`, `check-no-git-push`, `release-lib`,
`link-plugin-dev-sdk`). CI runs this in the `typecheck_release_registry` job.

## CI workflows & PR quality gates

### `.github/workflows/pr.yml` — the PR pipeline

Triggered on PRs to `master`, concurrency-grouped per PR. Jobs:

- **`policy`** — the gatekeeper. Blocks manual `pnpm-lock.yaml` edits (diffing
  merge-base…head, exempting the refresh branch and dependabot), runs the static
  gates (`check-docker-deps-stage`, `check-no-git-push` + its test, the vitest
  shard-partition test, standalone-build-concurrency test), validates the
  release manifest (`release-package-map.mjs check`), verifies release-package
  bootstrap for changed manifests, and — when a manifest changed — regenerates
  the lockfile with `--lockfile-only` and uploads it as the `pr-lockfile`
  artifact so downstream frozen-lockfile installs stay consistent.
- **`typecheck_release_registry`** — `typecheck:build-gaps` +
  `test:release-registry`.
- **`general_tests`** and **`verify_serialized_server`** — the sharded Vitest
  matrices described above.
- **`build`** — `pnpm build`.
- **`canary_dry_run`** — runs `./scripts/release.sh canary --skip-verify
  --dry-run` (on a temporary `master` branch) so the release path is exercised on
  every PR.
- **`e2e`** — the Playwright default suite.
- **`verify`** — a fan-in job (legacy required-check name) that fails unless
  `typecheck_release_registry`, `general_tests`, and `build` all succeeded.

### `.github/workflows/commitperclip-review.yml` — quality & security gates

Runs on `pull_request_target` from the **base branch context** (never executes
PR code) so it can access secrets for fork PRs. Steps:

- **Dependency Review** (`actions/dependency-review-action`).
- **Quality gates** —
  [`.github/scripts/run-quality-gates.mjs`](../../.github/scripts/run-quality-gates.mjs)
  fetches PR data once and runs pure checks, posting one consolidated
  `commitperclip` comment:
  - [`check-pr-template.mjs`](../../.github/scripts/check-pr-template.mjs) —
    required PR-template sections (Thinking Path, What Changed, Verification,
    Risks, Model Used).
  - [`check-pr-linked-issue.mjs`](../../.github/scripts/check-pr-linked-issue.mjs)
    — a linked issue/PR or an inline issue-template description.
  - [`check-pr-dedup-search.mjs`](../../.github/scripts/check-pr-dedup-search.mjs)
    — the "searched for duplicate PRs" affirmation.
  - [`check-pr-test-coverage.mjs`](../../.github/scripts/check-pr-test-coverage.mjs)
    — at least one test file (respecting `docs/chore/build/ci/style/refactor/revert`
    prefixes; also flags `docs:`/`chore:` PRs that touch source).
  - [`check-pr-lockfile.mjs`](../../.github/scripts/check-pr-lockfile.mjs) —
    warns when a PR edits `pnpm-lock.yaml` (only the refresh bot may).
  - [`check-pr-dependencies.mjs`](../../.github/scripts/check-pr-dependencies.mjs)
    — informational new-dependency notice (never fails).
- **Security gates** —
  [`.github/scripts/check-pr-security.mjs`](../../.github/scripts/check-pr-security.mjs)
  runs six silent checks (secret scan, CI-workflow tampering, changes to
  CI/build scripts, supply-chain/lockfile new packages, suspicious patterns in
  test files, and edits to a curated list of security-sensitive source paths).
  It **never posts public comments and always exits 0** — instead it files a
  draft security advisory when a check fires.

Bot tokens are minted by
[`.github/scripts/get-bot-token.mjs`](../../.github/scripts/get-bot-token.mjs);
each gate has a `node --test` suite under
[`.github/scripts/tests/`](../../.github/scripts/tests).

### Other workflows

- **[`docker.yml`](../../.github/workflows/docker.yml)** — on push to `master`
  and `v*` tags, builds the multi-arch (`linux/amd64,linux/arm64`) server image
  and pushes to `ghcr.io/<repo>` with GHA build cache. Tags: `latest` (default
  branch), semver, and `sha`.
- **[`agent-runtime-images.yml`](../../.github/workflows/agent-runtime-images.yml)**
  — builds the agent-runtime image family with `docker buildx bake` and signs
  each digest with cosign keyless OIDC. Default publish scope: `base`,
  `opencode`, `pi`, `codex`, `gemini`, `claude`.
- **[`refresh-lockfile.yml`](../../.github/workflows/refresh-lockfile.yml)** — the
  lockfile owner (see below).
- **[`release.yml`](../../.github/workflows/release.yml)**,
  **[`release-smoke.yml`](../../.github/workflows/release-smoke.yml)**,
  **[`e2e.yml`](../../.github/workflows/e2e.yml)** — covered above.

### Lockfile policy (Actions owns `pnpm-lock.yaml`)

The rule enforced end-to-end:

1. **Contributors never commit `pnpm-lock.yaml`.** The `policy` job in `pr.yml`
   hard-fails any PR (except the refresh branch and dependabot) that changes it,
   and `check-pr-lockfile.mjs` warns proactively in the review comment.
2. **CI regenerates it deterministically.** When a PR changes a manifest
   (`package.json`, `pnpm-workspace.yaml`, `.npmrc`, `pnpmfile.*`), `policy`
   regenerates the lockfile with `pnpm install --lockfile-only --ignore-scripts
   --no-frozen-lockfile` and uploads it as `pr-lockfile`; downstream jobs
   download it so every lane installs against the same hash.
3. **`refresh-lockfile.yml`** runs on every push to `master`: it regenerates the
   lockfile, fails on any non-lockfile change, and opens/updates a
   `chore/refresh-lockfile` PR (authored by `github-actions[bot]`) with
   auto-merge enabled. That branch is the only one allowed to carry a lockfile
   change through `pr.yml`.

Ownership of release/CI/dependency-critical files is enforced by
[`.github/CODEOWNERS`](../../.github/CODEOWNERS) (release scripts, `.github/**`,
`package.json`, `pnpm-lock.yaml`, `pnpm-workspace.yaml`, `.npmrc`, `skills/**`),
and [`.github/dependabot.yml`](../../.github/dependabot.yml) opens weekly npm +
github-actions update PRs (major bumps ignored).

## Docker images & deployment topologies

### Root [`Dockerfile`](../../Dockerfile) — the production server

Four stages: `base` (Node LTS + system tools, corepack, UID/GID-matched `node`
user), `deps` (copies each workspace `package.json` + `patches/` and runs
`pnpm install --frozen-lockfile`), `build` (copies the tree and builds UI →
plugin-sdk → server, asserting `server/dist/index.js`), and `production`
(globally installs the agent CLIs, sets deployment env, wires
`docker-entrypoint.sh`). Runtime defaults: `PORT=3100`, `SERVE_UI=true`,
`PAPERCLIP_DEPLOYMENT_MODE=authenticated`, `PAPERCLIP_DEPLOYMENT_EXPOSURE=private`.
The `deps` stage's `COPY` coverage is enforced by
`check-docker-deps-stage.mjs`.

### Compose files — [`docker/`](../../docker)

- **[`docker-compose.quickstart.yml`](../../docker/docker-compose.quickstart.yml)**
  — one `paperclip` service (embedded storage under a mounted `/paperclip`),
  requires `BETTER_AUTH_SECRET`, defaults to `authenticated`/`private`.
- **[`docker-compose.yml`](../../docker/docker-compose.yml)** — server +
  `postgres:17-alpine` with a healthcheck and `DATABASE_URL` wired in.
- **[`docker-compose.untrusted-review.yml`](../../docker/docker-compose.untrusted-review.yml)**
  — a hardened sandbox (`cap_drop: ALL`, `no-new-privileges`, tmpfs `/tmp`) built
  from [`docker/untrusted-review/Dockerfile`](../../docker/untrusted-review/Dockerfile)
  for reviewing untrusted PR code, with the
  [`review-checkout-pr`](../../docker/untrusted-review/bin/review-checkout-pr)
  helper. See [`doc/UNTRUSTED-PR-REVIEW.md`](../../doc/UNTRUSTED-PR-REVIEW.md).

### Podman Quadlet — [`docker/quadlet/`](../../docker/quadlet)

Rootless systemd units: a
[`paperclip.pod`](../../docker/quadlet/paperclip.pod) publishing `3100`, plus
[`paperclip.container`](../../docker/quadlet/paperclip.container) and
[`paperclip-db.container`](../../docker/quadlet/paperclip-db.container) (Postgres
with a healthcheck), sharing an env-file.

### AWS ECS Fargate — [`docker/ecs-task-definition.json`](../../docker/ecs-task-definition.json)

Fargate task (`2048` CPU / `4096` MB) pulling the server image from ECR,
`PAPERCLIP_DEPLOYMENT_EXPOSURE=public`, `PAPERCLIP_MIGRATION_AUTO_APPLY=true`,
`HEARTBEAT_SCHEDULER_ENABLED=true`, EFS-mounted `/paperclip`, and a
`/api/health` container healthcheck. Secrets (`DATABASE_URL`,
`BETTER_AUTH_SECRET`, provider keys, `GITHUB_TOKEN`) come from AWS Secrets
Manager — the file uses `<ACCOUNT_ID>` / `<REGION>` / `<DOMAIN>` placeholders.
Non-secret env lives in
[`docker/.env.aws.example`](../../docker/.env.aws.example).

### Agent-runtime image family — [`docker/agent-runtime/`](../../docker/agent-runtime)

`agent-runtime-{harness}:{version}` images (base + per-harness) built via
[`buildx-bake.hcl`](../../docker/agent-runtime/buildx-bake.hcl) and published to
`ghcr.io/paperclipai/` for sandbox providers (e.g. the Kubernetes provider). The
base image ships the Go `paperclip-agent-shim` (PID-1 under tini) that reads a
`runtime-command.json` and `exec`s the harness CLI. See
[the family README](../../docker/agent-runtime/README.md).

## Contracts & cross-layer coupling

- **Release manifest ↔ workspace packages.** Every public package must appear in
  `release-package-manifest.json`; the release fails otherwise
  (`release-package-map.mjs` reconciles the two, and `pr.yml` runs the `check`).
  A `publishFromCi:true` package may not depend on a `publishFromCi:false`
  package.
- **Dockerfile `deps` stage ↔ `pnpm-workspace.yaml`.** Adding a workspace without
  updating the Dockerfile `COPY` list fails `check-docker-deps-stage.mjs`.
- **Tags point at source commits.** `release.sh` tags the original SHA and
  restores the tree on exit; there is no release commit. Version numbers only
  ever live in published npm artifacts, not in `master` history.
- **Calver is date-derived and npm-derived.** The same-day patch/canary slot is
  computed by querying npm, so two releases on the same UTC day get distinct
  slots without local state.
- **Server publish artifacts are self-contained.** `server/ui-dist` and the
  copied `skills/` are generated during release and cleaned up afterward, so the
  published `@paperclipai/server` serves the UI and skills without the monorepo.
- **CI trusted publishing.** `publish_canary`/`publish_stable` set
  `id-token: write` and scoped `environment`s; `require_npm_publish_auth` accepts
  the GitHub Actions OIDC path when `GITHUB_ACTIONS=true`.
- Deploy env contracts (`PAPERCLIP_DEPLOYMENT_MODE`/`_EXPOSURE`,
  `BETTER_AUTH_SECRET`, `DATABASE_URL`) are the same across Compose/Quadlet/ECS —
  see [`doc/DEPLOYMENT-MODES.md`](../../doc/DEPLOYMENT-MODES.md) and
  [`doc/DATABASE.md`](../../doc/DATABASE.md). How the server reads them is
  documented in [server.md](./server.md); the schema and auto-apply migration
  path in [data-model.md](./data-model.md).

## Extension points

- **Publish a new package** — add it to `release-package-manifest.json`
  (`publishFromCi: false` first if it needs a manual bootstrap publish via
  [`scripts/bootstrap-npm-package.mjs`](../../scripts/bootstrap-npm-package.mjs) /
  `pnpm release:bootstrap-package`), then flip to `true`. The Docker `deps` stage
  `COPY` must be updated too.
- **Add a Vitest suite** — a route/authz-shaped server test auto-routes into the
  serialized lane; add anything that must serialize to
  `additionalSerializedServerTests` in `run-vitest-stable.mjs`.
- **Add an e2e / smoke suite** — drop a `*.spec.ts` under `tests/e2e` (add a
  Playwright config if it needs a distinct server/port) or a
  `scripts/smoke/*.sh` exposed as `pnpm smoke:*`.
- **Add an eval case** — add a YAML file under `evals/promptfoo/tests/`.
- **Add a PR gate** — add a pure `check-pr-*.mjs` with a `node --test` suite and
  wire it into `run-quality-gates.mjs` (or `check-pr-security.mjs` for a silent
  security signal).
- **Add a deploy target** — new Compose/Quadlet/ECS variants reuse the same image
  and env contract.
- **Add an agent-runtime image** — add a `Dockerfile.<harness>` + a bake target
  and (optionally) the publish scope in `agent-runtime-images.yml`.

## Gotchas / invariants

- **Never commit `pnpm-lock.yaml`.** `pr.yml`'s `policy` job hard-fails it; CI
  and the refresh bot own it. A confusing "manual lockfile edit" failure almost
  always means a stray committed lockfile.
- **`release.sh` requires a clean worktree** and (canary) branch `master`. It
  builds even under `--skip-verify` (Step 2/7 always runs), so CI must present a
  clean tree — hence the `git checkout -- pnpm-lock.yaml` steps before calling it.
- **The release trap restores the tree.** If it's bypassed, `cli/package.dev.json`,
  `server/ui-dist`, copied `skills/`, and rewritten versions can leak into the
  working tree.
- **Sigstore transparency-log duplicates are tolerated.**
  `publish_package_to_npm` treats a `TLOG_CREATE_ENTRY_ERROR` "equivalent entry
  already exists" as success if the version is already on npm, and for canary
  retries once with provenance disabled.
- **Registry verification exit code 2 is fatal, not transient** — it means the
  dist-tag state is wrong and needs an operator, so the release fails
  immediately without retrying.
- **Adapter/runtime code must never `git push`** — enforced by
  `check-no-git-push.mjs` in `pr.yml`; the workspace cwd is the only cross-run
  persistence boundary.
- **Server Vitest is `maxWorkers=1`.** Speed comes only from sharding across
  runners; a single-runner `general-server` run is slow by design.
- **e2e always boots a throwaway instance** (`reuseExistingServer: false`, temp
  `PAPERCLIP_HOME`) so it never attaches to a developer's live server; the
  multi-user (unauthenticated) config is the exception and needs an external
  server.
- **The `verify` job is a legacy required-check name** — it only fans in the
  split lanes; keep it green by keeping those lanes green.
- **Security gates run from the base branch** (`pull_request_target`) and never
  run PR code; they are silent and non-blocking by design (advisories, not
  comments).
- **The eval `.gitignore`** ignores `output/` and `*.json` (except the config),
  so promptfoo run artifacts are never committed.

## Key files

- [`scripts/release.sh`](../../scripts/release.sh) — release orchestrator (7 steps, canary + stable)
- [`scripts/release-lib.sh`](../../scripts/release-lib.sh) — calver, npm publish, registry-state helpers
- [`scripts/release-package-map.mjs`](../../scripts/release-package-map.mjs) + [`scripts/release-package-manifest.json`](../../scripts/release-package-manifest.json) — which packages publish
- [`scripts/build-npm.sh`](../../scripts/build-npm.sh) — CLI esbuild bundle for npm
- [`scripts/run-vitest-stable.mjs`](../../scripts/run-vitest-stable.mjs) — Vitest partitioning / sharding
- [`scripts/check-no-git-push.mjs`](../../scripts/check-no-git-push.mjs), [`scripts/check-forbidden-tokens.mjs`](../../scripts/check-forbidden-tokens.mjs), [`scripts/check-docker-deps-stage.mjs`](../../scripts/check-docker-deps-stage.mjs) — static gates
- [`scripts/docker-onboard-smoke.sh`](../../scripts/docker-onboard-smoke.sh), [`scripts/docker-build-test.sh`](../../scripts/docker-build-test.sh) — Docker smokes
- [`Dockerfile`](../../Dockerfile) + [`scripts/docker-entrypoint.sh`](../../scripts/docker-entrypoint.sh) — production server image
- [`docker/`](../../docker) — Compose, Quadlet, ECS, untrusted-review, agent-runtime
- [`tests/e2e/`](../../tests/e2e), [`tests/release-smoke/`](../../tests/release-smoke) — Playwright suites
- [`evals/promptfoo/`](../../evals/promptfoo) — agent behavior evals
- [`.github/workflows/`](../../.github/workflows) — PR, release, docker, e2e, release-smoke, refresh-lockfile, agent-runtime
- [`.github/scripts/`](../../.github/scripts) — PR quality & security gates

---

Related: [architecture index](./index.md) · [Server](./server.md) ·
[Data model](./data-model.md) · [Board UI](./ui.md) ·
[repo-records](../../references/repo-records.md)
