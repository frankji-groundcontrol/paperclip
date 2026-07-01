# Architecture — Team & skill catalogs

Internal architecture documentation for the **catalogs** subsystem: the two workspace packages that curate, validate, and ship the first‑party library of reusable agent *teams* and *skills* that Paperclip companies can browse and install.

- [`packages/teams-catalog`](../../packages/teams-catalog) — `@paperclipai/teams-catalog`
- [`packages/skills-catalog`](../../packages/skills-catalog) — `@paperclipai/skills-catalog`

The two packages are near‑mirror images of each other by design. This doc describes them together, calling out where they diverge.

See also: [architecture map](./index.md) · the wider [doc map](../index.md) · [server](./server.md) (hosts the catalog services and their HTTP routes) · [shared contracts](./shared-contracts.md) (the `@paperclipai/shared` copy of the team manifest types and the frontmatter parser skills‑catalog re‑exports) · [data model](./data-model.md) (agent `metadata` where installed‑team provenance is persisted).

## Purpose / Overview

Each catalog package is the authoritative source of a curated set of reusable content:

- **teams-catalog** ships whole *agent teams* (an org slice: a manager agent, subordinate agents, projects, tasks/routines, optionally local skills) as [Agent Companies spec](../../docs/companies/companies-spec.md) fragments (`schema: agentcompanies/v1`; the spec version string is `agentcompanies/v1-draft`).
- **skills-catalog** ships individual *skills* (`SKILL.md` plus optional references/scripts/assets), either vendored locally or referenced from a pinned external GitHub repo.

A package has two logically distinct halves:

1. **Source catalog content** under `catalog/` — hand‑authored Markdown packages the humans curate.
2. A **generated runtime artifact** at `generated/catalog.json` — a machine‑readable *manifest* built from that source by a deterministic builder. The manifest is the only thing the rest of the monorepo consumes at runtime; nothing reads the raw `catalog/` tree except the builder and the file‑serving path in the server services.

The builder is the trust boundary: it walks the source tree, parses frontmatter, validates cross‑references, computes SHA‑256 content hashes and a `trustLevel`, and emits a stable, sorted manifest. A CI/validate step asserts that the committed `generated/catalog.json` matches what the builder would produce, so the manifest can never silently drift from the source content.

Downstream, the Paperclip server exposes these manifests through catalog services that feed **company creation** (installing a team into an existing company) and **skill creation** (installing a skill into a company, then materializing it under `.agents/skills/`).

### Directory layout (both packages)

```
packages/<name>-catalog/
  catalog/                     # SOURCE content (hand-authored)
    bundled/<category>/<slug>/ # default-trust, shipped-by-default eligible
    optional/<category>/<slug>/# opt-in entries
  generated/catalog.json       # GENERATED manifest (committed, validated)
  scripts/
    build-catalog-manifest.ts  # regenerate the manifest
    validate-catalog.ts        # assert manifest is fresh + content is valid
  src/
    catalog-builder.ts         # builder/validator core
    frontmatter.ts             # YAML frontmatter parsing helpers
    types.ts                   # manifest + entry types
    index.ts                   # runtime entry: import catalog.json, resolve refs
```

The canonical path shape is strict: every entry lives at exactly `catalog/<bundled|optional>/<category>/<slug>/` with a required entrypoint file. teams-catalog requires `TEAM.md`; skills-catalog requires exactly one of `SKILL.md` (local) **or** `catalog-ref.json` (external reference). Any `TEAM.md` / `SKILL.md` / `catalog-ref.json` found at a non‑conforming depth is flagged as a "misplaced" error (see `collectMisplacedTeamFiles` / `collectMisplacedSkillFiles` in the builders).

## Entry points

- **Build the manifest:** `pnpm --filter @paperclipai/teams-catalog build:manifest` / `pnpm --filter @paperclipai/skills-catalog build:manifest`. Runs [`scripts/build-catalog-manifest.ts`](../../packages/teams-catalog/scripts/build-catalog-manifest.ts) → `writeCatalogManifest()`.
- **Validate:** `pnpm --filter @paperclipai/teams-catalog validate` / `... skills-catalog validate`. Runs [`scripts/validate-catalog.ts`](../../packages/teams-catalog/scripts/validate-catalog.ts) → `validateCatalog()`, which fails (exit 1) on any content error or a stale manifest.
- **Full build:** `pnpm ... build` = `build:manifest` then `tsc -p tsconfig.json` (emits `dist/`).
- **Runtime consumption (in-process):** import the package. [`src/index.ts`](../../packages/teams-catalog/src/index.ts) statically imports `../generated/catalog.json` and re‑exports it as typed data plus resolver helpers (`getCatalogTeam`, `resolveCatalogTeamRef` / `getCatalogSkill`, `resolveCatalogSkillRef`).
- **Runtime consumption (server):** the server does **not** import the package's `index.ts` for the live path; it reads `generated/catalog.json` directly off disk with mtime/size caching — see [`server/src/services/teams-catalog.ts`](../../server/src/services/teams-catalog.ts) and [`server/src/services/skills-catalog.ts`](../../server/src/services/skills-catalog.ts).

## Key modules & responsibilities

### `src/catalog-builder.ts` — builder / validator core

Both builders share the same skeleton (see teams‑catalog [`src/catalog-builder.ts`](../../packages/teams-catalog/src/catalog-builder.ts) and skills‑catalog [`src/catalog-builder.ts`](../../packages/skills-catalog/src/catalog-builder.ts)):

- `buildCatalogManifest({ packageDir, generatedAt })` — the pure build: discover candidates, build each entry, sort by `id`, collect uniqueness errors, return `{ manifest, errors }`.
- `buildExpectedCatalogManifest(packageDir)` — wraps `buildCatalogManifest` with a `generatedAt` stabilization trick: it reuses the existing manifest's `generatedAt` when nothing else changed, so re‑running the build on unchanged content is a no‑op diff. `sameManifestExceptGeneratedAt()` compares everything except the timestamp.
- `writeCatalogManifest(packageDir)` — build then, **only if there are zero errors**, write `generated/catalog.json` via `formatCatalogManifest()` (2‑space `JSON.stringify` + trailing newline). Errors abort the write.
- `validateCatalog(packageDir)` — build expected manifest, then compare byte‑for‑byte against the committed `generated/catalog.json`; append a "manifest is stale" error if they differ, plus any build errors.
- `formatCatalogManifest(manifest)` — the single canonical serializer. This is load‑bearing: `validateCatalog` does an exact string comparison, so the committed file must be produced by exactly this formatter.

Per‑entry construction:

- **Discovery** (`discoverTeamCandidates` / `discoverSkillCandidates`) walks `catalog/{bundled,optional}/<category>/<slug>/`, entries sorted by `name` at every level (`sortedDirEntries`) so the manifest order is deterministic.
- **Parse & validate frontmatter** — reads the entrypoint, requires `name` + `description` (+ `schema: agentcompanies/v1` for teams); cross‑checks any explicit `key`/`slug`/`category` frontmatter against the values derived from the directory path. Canonical identity is always derived from `kind`/`category`/`slug`:
  - `id = paperclipai:<kind>:<category>:<slug>`
  - `key = paperclipai/<kind>/<category>/<slug>`
- **File inventory** (`collectTeamFiles` / `collectSkillFiles`) recursively hashes every file (SHA‑256), classifies it (`classifyCatalogFile`), enforces `MAX_CATALOG_FILE_BYTES` (1 MiB) per file, and resolves symlinks defensively — broken symlinks, directory symlinks, and symlinks/paths escaping the entry directory are all rejected. The entrypoint is sorted first.
- **Content hash** (`buildContentHash`) — `sha256` over the ordered list of `{ path, sha256 }`, prefixed `sha256:`. This is the entry's stable identity for update detection downstream.
- **Trust level** (`deriveTrustLevel`) — see [Trust levels](#trust-levels).

### teams‑catalog: team package graph

teams‑catalog does substantially more than skills‑catalog because a team is a graph of entities. [`readTeamPackageGraph`](../../packages/teams-catalog/src/catalog-builder.ts) buckets every `AGENTS.md` / `PROJECT.md` / `TASK.md` / `SKILL.md` in the package, then:

- `validateLocalReferences` — requires a `manager` that resolves to an in‑package `AGENTS.md` (its slug becomes the `rootAgentSlugs`), validates `includes`, and checks that every `reportsTo`, project `owner`/`leadAgent`, task `assignee`, and task `project` resolves to a real slug in the package.
- `collectRequiredSkills` — merges skills declared on each agent (`skills:` frontmatter) with team‑level `requiredSkills`, dedupes by requirement identity, and resolves each ref to one of the `CatalogTeamSkillRequirementType`s (`catalog`, `local`, `skills_sh`, `github`, `url`, `local_path`, `agent_package`). Catalog refs are resolved against the sibling **skills‑catalog** manifest (see cross‑package coupling below); unresolved refs produce an error and `resolved: false`.
- `collectEnvInputs` — flattens `inputs.env` frontmatter across agents/projects into `CatalogTeamEnvInputSummary[]` (`secret`|`plain`, `required`|`optional`). Values are never captured — only key/shape metadata.
- `collectSourceRefs` — surfaces external `includes` and external skill sources with a `pinned` flag (`isPinnedExternalRef` = 40‑hex commit or `sha256:` digest).
- `counts` — agents/projects/tasks/routines/localSkills/catalogSkills/externalSkillSources. `tasks` vs `routines` is split on the task's `recurring` frontmatter flag.

### skills‑catalog: local vs referenced skills

A skill candidate is either `source: "local"` (has `SKILL.md`) or `source: "reference"` (has `catalog-ref.json`). Referenced skills are built by `buildReferencedCatalogSkill`:

- `readReferencedSkillDescriptor` parses `catalog-ref.json`, which must carry a `source: { type: "github", owner, repo, ref, commit, path }` plus optional `files`, `defaultInstall`, `recommendedForRoles`, `requires`, `tags`. `commit` **must** be a 40‑char SHA (`buildCatalogSkillSource` enforces it) — external skills are always commit‑pinned.
- `collectReferencedSkillFiles` fetches the repo's git tree from the GitHub API (`fetchGitHubTree`, honoring `hostname` for GHE via `githubApiBase`), filters blobs by the `files` include patterns (`referencedPathMatches`, supporting `dir/**`), and downloads each file's raw bytes (`fetchReferencedFileBytes` → `rawGitHubUrl`) to hash it. Network fetch is part of the build.
- **Resilience:** if a pinned reference is temporarily unreachable, `canFallbackToExistingReferencedSkill` reuses the previous manifest entry when the only errors are *recoverable* fetch failures (HTTP 403/408/409/425/429/≥500 or network) and the existing entry's source coordinates still match (`canReuseExistingReferencedSkill`). This keeps a flaky GitHub outage from dropping a shipped skill from the manifest.

See the example descriptor at [`catalog/optional/research/last30days/catalog-ref.json`](../../packages/skills-catalog/catalog/optional/research/last30days/catalog-ref.json).

### `src/frontmatter.ts` — the notable divergence

This is the clearest place the two packages differ:

- teams‑catalog vendors a **self‑contained minimal YAML frontmatter parser** in [`src/frontmatter.ts`](../../packages/teams-catalog/src/frontmatter.ts) (`parseFrontmatterMarkdown`, `asString`, `asBoolean`, `asStringArray`, `isPlainRecord`). It has no runtime dependencies — the package's `package.json` declares no `dependencies` — supporting the indent‑based subset of YAML the catalog frontmatter actually uses.
- skills‑catalog's [`src/frontmatter.ts`](../../packages/skills-catalog/src/frontmatter.ts) is a thin **re‑export of `@paperclipai/shared`** (`workspace:*` dependency), so it inherits the richer shared parser — including YAML block scalars (`>`, `>-`) used by some `SKILL.md` descriptions (covered by [`src/frontmatter.test.ts`](../../packages/skills-catalog/src/frontmatter.test.ts)).

Keep this in mind when authoring frontmatter: teams‑catalog frontmatter must stay within the vendored parser's capabilities.

### `src/types.ts` & `src/index.ts` — types and in‑process API

- `CatalogManifest` = `{ schemaVersion: 1, packageName, packageVersion, generatedAt, teams|skills }`.
- `CatalogTeam` / `CatalogSkill` are the per‑entry shapes ([teams types](../../packages/teams-catalog/src/types.ts), [skills types](../../packages/skills-catalog/src/types.ts)). The team shape is mirrored in [`packages/shared/src/types/teams-catalog.ts`](../../packages/shared/src/types/teams-catalog.ts) so the server and UI can consume it without importing the catalog package's internals.
- `index.ts` imports the JSON manifest with an import attribute (`with { type: "json" }`), exposes `catalogTeams` / `catalogSkills`, and builds `byId` / `byKey` lookup maps. `resolveCatalogTeamRef` / `resolveCatalogSkillRef` resolve a ref by exact id, then exact key, then a **unique** slug match (ambiguous slug → `null`).

## Data flow / execution lifecycle

### Author → manifest (build time)

```
edit catalog/<kind>/<category>/<slug>/...        (source content)
        │
        ▼  pnpm --filter <pkg> build:manifest
writeCatalogManifest(packageDir)
        │  buildExpectedCatalogManifest → buildCatalogManifest
        │    • discover candidates (sorted, path-shape checked)
        │    • per entry: parse frontmatter, hash file inventory,
        │      validate references, derive trustLevel + contentHash
        │    • sort by id, collect uniqueness errors
        ▼
generated/catalog.json                           (committed artifact)
        │
        ▼  pnpm --filter <pkg> validate  (CI)
validateCatalog → rebuild + byte-compare vs committed file
        → "stale" error if the committed manifest doesn't match
```

The `build:manifest` scripts invoke `tsx` from `cli/node_modules` (`node ../../cli/node_modules/tsx/dist/cli.mjs ...`) rather than depending on `tsx` directly.

### Manifest → company / skill (runtime, server)

**Teams → company creation** ([`server/src/services/teams-catalog.ts`](../../server/src/services/teams-catalog.ts), `teamsCatalogService`):

1. `getCatalogManifest()` reads `generated/catalog.json` off disk (locations tried in order: `PAPERCLIP_TEAMS_CATALOG_DIR`, `cwd/packages/teams-catalog`, and a path relative to the compiled service), cached by mtime+size.
2. `prepareCatalogTeamSource(companyId, catalogRef, options)` resolves the team, hard‑blocks `scripts_executables` trust (the safe importer refuses executable teams) and gates `external_sources`, then builds an **inline portable bundle**: it reads every source file into memory (`readCatalogTeamSourceFiles`), synthesizes a `COMPANY.md` (`renderSyntheticCompanyMarkdown`), and generates a `.paperclip.yaml` extension carrying **catalog provenance** (`renderCatalogProvenanceYaml`) under `metadata.paperclip.catalogTeam` for every agent/project/task, merged over any existing `.paperclip.yaml`.
3. Agent skill refs in the bundle are rewritten to canonical catalog keys (`rewriteAgentCatalogSkillRefs`).
4. `previewCatalogTeamImport` / `installCatalogTeam` hand the inline bundle to the **company‑portability** importer in `agent_safe` mode. Bundled catalog agents declare no adapter, so `withSafeCatalogAdapterDefaults` injects a safe default adapter (`claude_local`, overridable via `PAPERCLIP_TEAMS_CATALOG_DEFAULT_ADAPTER_TYPE`) for any agent the caller didn't override.
5. After the team imports, `prepareSkillInstalls` walks the team's skill *preparations* and installs each: `catalog_install_required` → `companySkills.installFromCatalog`, `external_import_required` → `companySkills.importFromSource`, `blocked` → warning only.
6. `listInstalledCatalogTeams` reads each company agent's `metadata.paperclip.catalogTeam` provenance (`readCatalogTeamProvenance`) and compares the installed `originHash` to the live `contentHash` to compute the `outOfDate` flag driving the catalog UI's "installed / update available" state.

`collectCatalogTeamSkillPreparations` is the policy gate for a team's skills: unresolved → blocked; `catalog`/`local` → install‑from‑package; `local_path`/`agent_package` → blocked (dev‑only / no safe resolver); external `github`/`skills_sh` must be commit‑pinned for **bundled** teams and, for **optional** teams, either pinned or explicitly allowed via `sourcePolicy.allowUnpinnedOptionalSources`.

**Skills → skill creation** ([`server/src/services/skills-catalog.ts`](../../server/src/services/skills-catalog.ts)):

- `listCatalogSkills` / `resolveCatalogSkillReference` / `getCatalogSkillOrThrow` mirror the team resolvers (id → key → unique slug). `listCatalogSkillsOrEmpty` degrades to `[]` (logged once) when the manifest is unavailable, so the product surface survives a missing catalog.
- File reads go through `readCatalogFileBytes`, which **re‑verifies the SHA‑256** of every file against the manifest's recorded hash before returning bytes — a local file whose bytes drift from the manifest, or a pinned external file whose content changed, is rejected. Local skills read from the package dir; referenced skills fetch pinned raw GitHub bytes (`fetchCatalogSourceFile`).
- Company skill install (in `company-skills.ts`) copies each catalog file via `copyCatalogSkillFile` into a runtime skill directory named by `buildSkillRuntimeName(key, slug)` and records `catalogSkill.contentHash` as the tracking/origin hash for later update checks.

### Relationship to `.agents/skills`

`catalog/` is **source**, not a runtime skills directory. When a catalog skill is installed into a company, its files are materialized into that company's runtime skills tree — `.agents/skills` is one of the runtime scan roots the skills service recognizes (`PROJECT_SCAN_DIRECTORY_ROOTS` in `server/src/services/company-skills.ts`, alongside `skills/`, `.claude/skills`, etc.). So the catalog is the curated upstream; `.agents/skills` (and siblings) is the per‑company downstream where installed copies live. There is no symlink or live link between them — install is a hash‑verified copy, and the recorded `contentHash` is what later flags a company's installed skill as out of date versus the catalog.

## Contracts & cross‑layer coupling

- **Manifest is the contract.** Everything downstream (server services, UI, the shared types package) depends on the JSON shape in `generated/catalog.json`, versioned by `schemaVersion: 1`. The team shape is duplicated in `@paperclipai/shared` — changes to `CatalogTeam` must be made in both [`packages/teams-catalog/src/types.ts`](../../packages/teams-catalog/src/types.ts) and [`packages/shared/src/types/teams-catalog.ts`](../../packages/shared/src/types/teams-catalog.ts).
- **teams‑catalog → skills‑catalog (build‑time).** `loadCatalogSkills` in the teams builder resolves `catalog` skill requirements against the skills‑catalog manifest: first by importing `@paperclipai/skills-catalog`, falling back to reading the sibling `../skills-catalog/generated/catalog.json` from disk. A bundled team that references a catalog skill key that no longer exists fails validation. (There is no reverse dependency: skills‑catalog knows nothing about teams.)
- **Canonical id/key derivation** is enforced, not authored: explicit `key`/`slug`/`category` frontmatter is allowed only if it matches the directory‑derived value. `shipped-catalog.test.ts` re‑asserts this for every shipped entry.
- **Content hash is the update signal.** `contentHash` (teams) / `contentHash` (skills) is what the server persists as `originHash` and later diffs to compute `outOfDate`. Any change to `buildContentHash`'s hash input would invalidate every installed entry's update detection.
- **Trust gates the importer.** `deriveTrustLevel` output is a hard gate: the safe team importer refuses `scripts_executables` and gates `external_sources`; the shipped‑catalog tests keep every shipped entry at `markdown_only` / `assets` today.
- **Package exports:** `.` (index), `./types`, and `./catalog.json` are exported; `publishConfig.exports` remaps them to `dist/`. `files` ships `catalog`, `dist`, and `generated`, so consumers get both the manifest and the raw source content.

### Trust levels

`deriveTrustLevel(files[, sourceRefs])`:

| Level | teams‑catalog | skills‑catalog |
| --- | --- | --- |
| `external_sources` | any external `sourceRefs` present | (n/a — skills express external via `source`) |
| `scripts_executables` | any file under `scripts/` | any file under `scripts/` |
| `assets` | has `assets/`, `other`, or `extension` file | has `assets/` or `other` file |
| `markdown_only` | otherwise | otherwise |

File classification (`classifyCatalogFile`) keys off well‑known names/prefixes: entrypoint → `team`/`skill`; `AGENTS.md`/`PROJECT.md`/`TASK.md`/`SKILL.md` → their kinds; `.paperclip.yaml` → `extension` (teams only); `README.md` → `readme` (teams only; stays `markdown_only`); `references/*` → `reference`; `scripts/*` → `script`; `assets/*` → `asset`; other `.md`/`.mdx` → `markdown`; else `other`. Note skills‑catalog's classifier is the smaller set (`skill`/`reference`/`script`/`asset`/`markdown`/`other`) with no `team`/`agent`/`project`/`task`/`extension`/`readme` kinds.

## Packaging

- Both are ESM (`"type": "module"`) TypeScript packages published as `@paperclipai/teams-catalog` (v0.1.0) and `@paperclipai/skills-catalog` (v0.3.1).
- `build` = regenerate the manifest, then `tsc` to `dist/`. The published `exports` (via `publishConfig`) point at compiled `dist/` outputs and `dist/generated/catalog.json`.
- `files` includes the raw `catalog/` tree, `dist/`, and `generated/` — so a published consumer can both read the manifest and copy source files.
- `packaged-artifacts.test.ts` (skills‑catalog) runs `npm pack --json` and asserts the tarball contains `dist/generated/catalog.json`, `generated/catalog.json`, representative `catalog/.../SKILL.md` files, and `package.json` — protecting the published artifact shape. See [`src/packaged-artifacts.test.ts`](../../packages/skills-catalog/src/packaged-artifacts.test.ts).
- Release wiring lives in [`scripts/release-package-manifest.json`](../../scripts/release-package-manifest.json).

## Extension points

- **Add a team:** create `catalog/<bundled|optional>/<category>/<slug>/TEAM.md` (frontmatter `name`, `description`, `schema: agentcompanies/v1`, `manager`, optional `includes`/`requiredSkills`/`tags`/`recommendedForCompanyTypes`/`defaultInstall`) plus the referenced `agents/*/AGENTS.md`, `projects/*/PROJECT.md`, `tasks/*/TASK.md`; run `build:manifest`, then `validate` + `test`. Example: [`catalog/bundled/company-defaults/core-exec-team/TEAM.md`](../../packages/teams-catalog/catalog/bundled/company-defaults/core-exec-team/TEAM.md).
- **Add a local skill:** create `catalog/<kind>/<category>/<slug>/SKILL.md` (+ optional `references/`, `scripts/`, `assets/`). Example: [`catalog/bundled/software-development/github-pr-workflow/SKILL.md`](../../packages/skills-catalog/catalog/bundled/software-development/github-pr-workflow/SKILL.md).
- **Add an external (referenced) skill:** create `catalog/<kind>/<category>/<slug>/catalog-ref.json` with a commit‑pinned GitHub `source` and a `files` include list. The builder fetches and hashes at build time.
- **Adapter default for imported teams:** `PAPERCLIP_TEAMS_CATALOG_DEFAULT_ADAPTER_TYPE` overrides the safe fallback adapter used during install.
- **Alternate catalog location at runtime:** `PAPERCLIP_TEAMS_CATALOG_DIR` points the server at a different teams‑catalog package root.
- New file kinds / trust rules are added in `classifyCatalogFile` + `deriveTrustLevel`; new external skill‑source types in `CatalogTeamSkillRequirementType` + `resolveDeclaredSkillRequirement` and the server's `collectCatalogTeamSkillPreparations` policy gate.

## Testing

Run per package with `pnpm --filter @paperclipai/teams-catalog test` / `... skills-catalog test` (Vitest, node env, `src/**/*.test.ts`).

- **`catalog-builder.test.ts`** (both) — builds a fixture package in a temp dir and asserts stable manifest entries; asserts that frontmatter/directory/uniqueness/reference/skill (teams) and inventory (skills) errors are all reported together; asserts `validateCatalog` detects a stale committed manifest. Skills version additionally covers building from pinned GitHub references and the two fallback‑reuse paths when GitHub is unavailable. [teams](../../packages/teams-catalog/src/catalog-builder.test.ts) · [skills](../../packages/skills-catalog/src/catalog-builder.test.ts).
- **`shipped-catalog.test.ts`** (both) — pins the exact set of shipped bundled/optional keys, asserts every shipped entry is `markdown_only`/`assets` (no scripts/external), asserts browse/search fields are populated, re‑asserts canonical id/key derivation, and checks resolver behavior. teams additionally asserts every recurring task declares a valid in‑team project. [teams](../../packages/teams-catalog/src/shipped-catalog.test.ts) · [skills](../../packages/skills-catalog/src/shipped-catalog.test.ts).
- **`frontmatter.test.ts`** (skills only) — YAML block‑scalar handling via the shared parser.
- **`packaged-artifacts.test.ts`** (skills only) — `npm pack` artifact shape.
- Server‑side integration: `server/src/__tests__/skills-catalog-service.test.ts`, `company-skills-catalog-service.test.ts`, `company-portability.test.ts` exercise the consuming services.

## Gotchas / invariants

- **The committed `generated/catalog.json` must be regenerated and committed with any `catalog/` change.** `validate` does an exact string compare against `formatCatalogManifest` output; a stale manifest fails CI. `generatedAt` is intentionally *not* bumped when nothing else changed (`buildExpectedCatalogManifest`), so unrelated content edits don't churn the timestamp.
- **`writeCatalogManifest` refuses to write when there are errors** — a broken catalog leaves the last good manifest in place rather than shipping a partial one.
- **skills‑catalog builds hit the network.** Referenced‑skill entries fetch GitHub at build time; a build with no network can only succeed by reusing prior manifest entries (recoverable‑error fallback), and only for entries whose pinned coordinates are unchanged.
- **External skills are always commit‑pinned;** `catalog-ref.json` `commit` must be a 40‑hex SHA or the entry is dropped. The server also **re‑hashes** every fetched/local file against the manifest before use, so a mutated pinned ref or drifted local file is rejected at read time, not just build time.
- **teams‑catalog and skills‑catalog have separate, non‑identical frontmatter parsers** (vendored vs shared) — see the divergence note above.
- **Trust is enforced end to end.** A team that gains a `scripts/` file becomes `scripts_executables` and is refused by the safe importer; the shipped‑catalog tests fail if any shipped entry exceeds `markdown_only`/`assets`. Introducing scripts/external sources is deliberately gated behind a security review (see the migration notes).
- **Symlinks are hostile input.** The inventory walker rejects broken symlinks, directory symlinks, and any symlink/relative path that escapes the entry directory — do not use symlinks to share content between entries; copy files in.
- **Adapter‑less agents.** Bundled catalog agents omit `adapterType` on purpose so operators pick an adapter at import time; the safe importer would otherwise reject the `process` default, which is why the install path injects a `claude_local` default and emits a warning listing the defaulted agents.

## Migration notes

teams‑catalog carries a live [`MIGRATION.md`](../../packages/teams-catalog/MIGRATION.md) recording the state of migrating legacy onboarding content into the catalog:

- The current onboarding assets under `server/src/onboarding-assets/ceo/` and `.../default/` are **kept as‑is**; the catalog does not yet replace onboarding. `bundled/company-defaults/core-exec-team` is the catalog mirror of the historical CEO onboarding flow (`defaultInstall: true`).
- Legacy per‑role agent templates (`skills/paperclip-create-agent/references/agents/*.md`) had their content migrated into catalog `AGENTS.md` files (senior‑coder, qa, ux‑designer) but are **kept** as authoritative templates for ad‑hoc hiring until onboarding switches to the catalog service. Some role templates (e.g. security engineer) are intentionally **not** migrated.
- Rich persona files (`SOUL.md`/`HEARTBEAT.md`/`TOOLS.md`) are intentionally collapsed into a single `AGENTS.md` per agent for now; per‑agent adapter overrides are intentionally omitted from frontmatter so the import preview lets operators choose at install time.
- Executable/script‑bearing entries and several optional teams are **deferred** pending a security review of external‑source handling — the shipped‑catalog test enforces `markdown_only`/`assets` trust until then.
- `MIGRATION.md` also lists the compatibility tests (onboarding parity, slug stability, skill‑resolution drift, adapter‑default fallback, routine import) that must land before any legacy onboarding source can be deleted.

## Key files

- [`packages/teams-catalog/src/catalog-builder.ts`](../../packages/teams-catalog/src/catalog-builder.ts) — teams builder/validator + team package graph.
- [`packages/skills-catalog/src/catalog-builder.ts`](../../packages/skills-catalog/src/catalog-builder.ts) — skills builder/validator + referenced‑skill fetching.
- [`packages/teams-catalog/src/types.ts`](../../packages/teams-catalog/src/types.ts) / [`packages/skills-catalog/src/types.ts`](../../packages/skills-catalog/src/types.ts) — manifest and entry types.
- [`packages/teams-catalog/src/index.ts`](../../packages/teams-catalog/src/index.ts) / [`packages/skills-catalog/src/index.ts`](../../packages/skills-catalog/src/index.ts) — in‑process manifest + resolvers.
- [`packages/teams-catalog/src/frontmatter.ts`](../../packages/teams-catalog/src/frontmatter.ts) (vendored) / [`packages/skills-catalog/src/frontmatter.ts`](../../packages/skills-catalog/src/frontmatter.ts) (shared re‑export).
- [`packages/teams-catalog/generated/catalog.json`](../../packages/teams-catalog/generated/catalog.json) / [`packages/skills-catalog/generated/catalog.json`](../../packages/skills-catalog/generated/catalog.json) — generated runtime manifests.
- [`packages/teams-catalog/scripts/build-catalog-manifest.ts`](../../packages/teams-catalog/scripts/build-catalog-manifest.ts) / [`packages/teams-catalog/scripts/validate-catalog.ts`](../../packages/teams-catalog/scripts/validate-catalog.ts) — build/validate entry scripts.
- [`packages/teams-catalog/MIGRATION.md`](../../packages/teams-catalog/MIGRATION.md) — legacy‑onboarding migration state.
- [`server/src/services/teams-catalog.ts`](../../server/src/services/teams-catalog.ts) / [`server/src/services/skills-catalog.ts`](../../server/src/services/skills-catalog.ts) — server consumers feeding company/skill creation.
- [`packages/shared/src/types/teams-catalog.ts`](../../packages/shared/src/types/teams-catalog.ts) — shared copy of the team manifest types.
