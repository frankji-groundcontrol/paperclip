# Architecture — Data model & migrations (packages/db)

> Internal architecture doc for the `@paperclipai/db` package. Grounded in the code under
> [packages/db](../../packages/db). For operator-facing setup (embedded vs external Postgres,
> `pnpm db:migrate`, resetting local data) see [doc/DATABASE.md](../../doc/DATABASE.md); this
> document covers *how the package is built* rather than *how to run it*.

## Purpose / Overview

`@paperclipai/db` is the single source of truth for Paperclip's relational data model. It owns:

- The **Drizzle ORM schema** — one TypeScript module per table (or small cluster of related
  tables) under [packages/db/src/schema](../../packages/db/src/schema), re-exported through a
  flat barrel ([src/schema/index.ts](../../packages/db/src/schema/index.ts)).
- The **Db client factory** ([src/client.ts](../../packages/db/src/client.ts)) — wraps the
  `postgres` driver in a typed `drizzle` instance and provides all migration inspection /
  application primitives.
- **Migration generation** via `drizzle-kit` ([drizzle.config.ts](../../packages/db/drizzle.config.ts))
  and a large, append-only SQL migration set under
  [src/migrations](../../packages/db/src/migrations) with Drizzle journal metadata.
- **Embedded vs external PostgreSQL** resolution and lifecycle
  ([src/runtime-config.ts](../../packages/db/src/runtime-config.ts),
  [src/migration-runtime.ts](../../packages/db/src/migration-runtime.ts),
  [src/embedded-postgres-native.ts](../../packages/db/src/embedded-postgres-native.ts)).
- Backup/restore ([src/backup-lib.ts](../../packages/db/src/backup-lib.ts)) and a test-only
  embedded-Postgres harness ([src/test-embedded-postgres.ts](../../packages/db/src/test-embedded-postgres.ts)).

The package targets **PostgreSQL only** (dialect `postgresql`). "Embedded Postgres" is a real
PostgreSQL cluster started in-process via the `embedded-postgres` npm package — not a SQL
emulation. A legacy `pglite` mode is silently migrated to `embedded-postgres` at config-read
time (see [runtime-config.ts](../../packages/db/src/runtime-config.ts) `migrateLegacyConfig`).

The public API surface is defined by [src/index.ts](../../packages/db/src/index.ts): it re-exports
the client + migration functions, the backup library, embedded-Postgres helpers, and
`export * from "./schema/index.js"` (every table object).

## Entry points

The package is consumed by the server, CLI, plugin runtime, and tests. The important entry
points, all exported from [src/index.ts](../../packages/db/src/index.ts):

| Export | Kind | Purpose |
| --- | --- | --- |
| `createDb(url)` | factory | Returns a typed Drizzle client (`Db`) bound to a `postgres` connection and the full `schema`. |
| `type Db` | type | `ReturnType<typeof createDb>`; the handle passed around the server. |
| `inspectMigrations(url)` | fn | Reports `MigrationState` (`upToDate` / `needsMigrations` + reason). |
| `applyPendingMigrations(url)` | fn | Idempotently applies pending migrations (bootstrap + manual paths). |
| `migratePostgresIfEmpty(url)` | fn | One-shot bootstrap for an empty DB only. |
| `reconcilePendingMigrationHistory(url)` | fn | Repairs the Drizzle history table when the schema is already present. |
| `ensurePostgresDatabase(url, name)` | fn | `CREATE DATABASE` if missing (used before migrating). |
| `getPostgresDataDirectory(url)` | fn | Reads `data_directory` GUC to identify an embedded cluster. |
| Backup: `runDatabaseBackup`, `runDatabaseRestore`, `formatDatabaseBackupResult` | fns | pg_dump/JS backup + restore. |
| Test: `startEmbeddedPostgresTestDatabase`, `getEmbeddedPostgresTestSupport` | fns | Spin up a throwaway embedded cluster with migrations applied. |
| All schema tables (e.g. `companies`, `agents`, `issues`, …) | table objects | Drizzle `pgTable` handles for queries. |

**CLI script entry points** (from [package.json](../../packages/db/package.json) `scripts`, and
the repo-root aliases `db:generate` / `db:migrate` in the root `package.json`):

- `pnpm --filter @paperclipai/db generate` (root: `pnpm db:generate`) →
  `check:migrations` then `tsc -p tsconfig.json` then `drizzle-kit generate`.
- `pnpm --filter @paperclipai/db migrate` (root: `pnpm db:migrate`) →
  `check:migrations` then `tsx src/migrate.ts`.
- `pnpm --filter @paperclipai/db seed` → `tsx src/seed.ts`.
- Migration status CLI: `src/migration-status.ts` (`--json` supported).

At **server runtime**, the server (not this package's CLI) drives migrations: `server/src/index.ts`
imports `createDb`, `ensurePostgresDatabase`, `inspectMigrations`, `applyPendingMigrations`, and
`reconcilePendingMigrationHistory` and runs them during startup before serving requests.

## Key modules & responsibilities

### Schema modules — `src/schema/*.ts`

Each file declares one or more tables with Drizzle's `pgTable`. There are **93 schema modules**
declaring roughly **111 tables** (some files bundle related tables — e.g.
[routines.ts](../../packages/db/src/schema/routines.ts) exports `routines`, `routineRevisions`,
`routineTriggers`, `routineRuns`; [pipeline_cases.ts](../../packages/db/src/schema/pipeline_cases.ts)
exports six pipeline tables). Conventions observed across the schema:

- **UUID primary keys** with `uuid("id").primaryKey().defaultRandom()` on domain tables. The
  Better-Auth tables in [auth.ts](../../packages/db/src/schema/auth.ts) are the exception: they use
  **text** primary keys (`user`, `session`, `account`, `verification`) to match the external auth
  library's ID format.
- **Timezone-aware timestamps** (`timestamp(..., { withTimezone: true })`) with `defaultNow()` for
  `created_at` / `updated_at`.
- **JSONB payloads** typed via `.$type<...>()` against shared types from `@paperclipai/shared`
  (e.g. `AgentEnvConfig` on [projects.ts](../../packages/db/src/schema/projects.ts),
  `SourceTrustMetadata` on [issues.ts](../../packages/db/src/schema/issues.ts)).
- **Foreign keys** declared with `.references(() => other.id)`, including **self-references** via
  `AnyPgColumn` (e.g. `agents.reportsTo → agents.id`, `issues.parentId → issues.id`).
- **Composite / partial / expression indexes** live in the table's second callback argument. See
  [issues.ts](../../packages/db/src/schema/issues.ts) for GIN trigram search indexes
  (`.using("gin", table.title.op("gin_trgm_ops"))`) and many `where(...)`-guarded unique indexes
  that enforce "at most one active row of kind X per company" invariants.

Representative anchor tables:

- [companies.ts](../../packages/db/src/schema/companies.ts) — the tenant root. Owns
  `issue_prefix` / `issue_counter` (per-company issue identifiers), budget cents, brand color, and
  feedback data-sharing consent columns. `companies_issue_prefix_idx` is a unique index on
  `issue_prefix`.
- [agents.ts](../../packages/db/src/schema/agents.ts) — `company_id` FK, `reports_to` self-FK,
  `adapter_config` / `runtime_config` / `permissions` JSONB, `default_environment_id` FK to
  environments.
- [issues.ts](../../packages/db/src/schema/issues.ts) — the largest and most index-dense table;
  the unit of work in the control plane.
- [company_secrets.ts](../../packages/db/src/schema/company_secrets.ts) — secret metadata
  (values live in `company_secret_versions`); note this table is company-scoped while the versions
  table is keyed through the secret, not a direct `company_id`.

### Company scoping — the multi-tenant column

Paperclip is multi-tenant with **`companies` as the tenant root**. **80 of the 93 schema modules
carry a `company_id` column**, and most domain reads/writes filter by it. The convention:

```ts
companyId: uuid("company_id").notNull().references(() => companies.id),
// ... plus a leading-column index:
companyStatusIdx: index("agents_company_status_idx").on(table.companyId, table.status),
```

Composite indexes almost always **lead with `company_id`** so tenant-scoped queries stay on an
index (see [issues.ts](../../packages/db/src/schema/issues.ts):
`issues_company_status_idx`, `issues_company_assignee_status_idx`, etc.).

Tables **without** `company_id` fall into clear categories:

- **Tenant root / instance-global**: [companies.ts](../../packages/db/src/schema/companies.ts),
  [instance_settings.ts](../../packages/db/src/schema/instance_settings.ts),
  [instance_user_roles.ts](../../packages/db/src/schema/instance_user_roles.ts),
  [auth.ts](../../packages/db/src/schema/auth.ts) (identity is instance-wide),
  [user_sidebar_preferences.ts](../../packages/db/src/schema/user_sidebar_preferences.ts).
- **Instance-scoped infrastructure**: [environments.ts](../../packages/db/src/schema/environments.ts),
  [board_api_keys.ts](../../packages/db/src/schema/board_api_keys.ts).
- **Plugin-owned tables** where the plugin, not the company, is the scoping root:
  [plugins.ts](../../packages/db/src/schema/plugins.ts),
  [plugin_config.ts](../../packages/db/src/schema/plugin_config.ts),
  [plugin_state.ts](../../packages/db/src/schema/plugin_state.ts),
  [plugin_database.ts](../../packages/db/src/schema/plugin_database.ts).
- **Child rows reached via a parent FK** (e.g.
  [company_secret_versions.ts](../../packages/db/src/schema/company_secret_versions.ts) — scoped
  through its `company_secrets` parent).

Some plugin tables carry a **nullable** `company_id` where `NULL` means instance-level. See
[plugin_entities.ts](../../packages/db/src/schema/plugin_entities.ts): the per-tenant unique index
uses `.nullsNotDistinct()` (Postgres 15+) precisely because `company_id` can be `NULL` and two
`NULL`s would otherwise be treated as distinct, breaking dedup. Migration
[0101_plugin_company_id_tenant_isolation.sql](../../packages/db/src/migrations/0101_plugin_company_id_tenant_isolation.sql)
retrofitted `company_id` onto `plugin_entities`, `plugin_job_runs`, `plugin_logs`, and
`plugin_webhook_deliveries` for tenant isolation.

### The Db client — `src/client.ts`

[client.ts](../../packages/db/src/client.ts) is the heart of the package. It has two roles:

1. **Client construction** — `createDb(url)` builds `drizzlePg(postgres(url), { schema })`. The
   exported `type Db = ReturnType<typeof createDb>` is the handle threaded through the server.

2. **Migration engine** — a hand-rolled, defensive layer *on top of* Drizzle's own migrator,
   because Paperclip must handle databases in inconsistent states (fresh, adopted, partially
   migrated, or with a divergent history table). Key pieces:

   - `createUtilitySql(url)` — a single-connection (`max: 1`) `postgres` client with notices
     silenced, used for all inspection/DDL.
   - `MIGRATIONS_FOLDER`, `MIGRATIONS_JOURNAL_JSON` — resolved relative to the module URL so they
     work both from `src/` (tsx) and the compiled `dist/` (see build note below).
   - `DRIZZLE_MIGRATIONS_TABLE = "__drizzle_migrations"` and `discoverMigrationTableSchema()` — the
     history table may live in the `drizzle` schema (Drizzle default) *or* `public`; discovery
     prefers `drizzle`, then `public`, then any schema.
   - **SQL safety helpers** — `isSafeIdentifier`, `quoteIdentifier`, `quoteLiteral`; identifiers are
     validated against `/^[A-Za-z_][A-Za-z0-9_]*$/` before interpolation.
   - `splitMigrationStatements()` — splits a `.sql` file on the Drizzle
     `--> statement-breakpoint` marker; each statement is run individually inside a transaction.

   Public migration API:

   - `inspectMigrations(url): MigrationState` — counts `public` base tables, discovers the history
     table, resolves applied migrations, and returns `upToDate` or `needsMigrations` with a
     `reason` of `no-migration-journal-empty-db`, `no-migration-journal-non-empty-db`, or
     `pending-migrations`.
   - `applyPendingMigrations(url)` — the orchestrator. For an **empty DB with no journal**, it runs
     Drizzle's `migrate()` to bootstrap, then reconciles + manually applies any stragglers. For a
     **non-empty DB with no journal**, it *throws* (auto-migration is unsafe). For **pending
     migrations**, it first calls `reconcilePendingMigrationHistory`, then
     `applyPendingMigrationsManually`.
   - `applyPendingMigrationsManually(url, pending)` — orders pending files by the journal, hashes
     each (`sha256`), skips files already recorded, and applies each remaining file's statements in
     a single `BEGIN/COMMIT` transaction, recording a history row (`hash` / `name` / `created_at`
     as the columns permit).
   - `reconcilePendingMigrationHistory(url)` — the "self-healing" path. For each pending file it
     checks whether the DDL is **already applied** (`migrationContentAlreadyApplied` inspects
     `information_schema` / `pg_class` / `pg_constraint` for `CREATE TABLE`, `ADD COLUMN`,
     `CREATE INDEX`, `ADD CONSTRAINT` statements). If so, it records/repairs the history row instead
     of re-running the DDL. Statements it cannot reason about safely force a real migration.
   - `migratePostgresIfEmpty(url)` — narrower bootstrap: migrates **only** when there is no history
     table *and* zero `public` tables; otherwise returns `already-migrated` /
     `not-empty-no-migration-journal` without acting.
   - `ensurePostgresDatabase(url, name)` — validates the DB name, checks `pg_database`, and issues
     `CREATE DATABASE ... encoding 'UTF8' lc_collate 'C' lc_ctype 'C' template template0`.

   `loadAppliedMigrations()` handles history tables that key on `name`, on `hash`, or (fallback)
   ordinal `id` — reconciling against `mapHashesToMigrationFiles` and the journal so a DB created by
   any Drizzle version can be understood.

### Migration generation — `drizzle.config.ts` + `db:generate`

[drizzle.config.ts](../../packages/db/drizzle.config.ts) points `schema` at **`./dist/schema/*.js`
(compiled output), not `./src`**, `out` at `./src/migrations`, dialect `postgresql`, and reads
`DATABASE_URL` for credentials. This is why the `generate` script
([package.json](../../packages/db/package.json)) runs **`tsc -p tsconfig.json` first** — Drizzle
introspects the *built* JS schema, so `dist/schema` must be regenerated before `drizzle-kit generate`
diffs it against the recorded snapshots. `check:migrations` runs before both `generate` and `migrate`.

### Migration runtime & config resolution — `runtime-config.ts`, `migration-runtime.ts`

[runtime-config.ts](../../packages/db/src/runtime-config.ts) `resolveDatabaseTarget()` decides
where the database lives, in priority order:

1. `process.env.DATABASE_URL` → external `postgres` (`source: "DATABASE_URL"`).
2. `DATABASE_URL` from the Paperclip env file → external `postgres` (`source: "paperclip-env"`).
3. `config.database.mode === "postgres"` + `connectionString` → external
   (`source: "config.database.connectionString"`).
4. Otherwise → **embedded-postgres**, using `embeddedPostgresDataDir` /
   `embeddedPostgresPort` from config (defaults: a resolved home-aware data dir and **port 54329**).

Config is discovered by walking up from `cwd` for `.paperclip/config.json` (or `PAPERCLIP_CONFIG`),
and legacy `pglite` config is rewritten to `embedded-postgres` on read.

[migration-runtime.ts](../../packages/db/src/migration-runtime.ts) `resolveMigrationConnection()`
turns that target into a live `MigrationConnection` (`connectionString`, `source`, `stop()`):

- **External postgres** → returns the connection string directly with a no-op `stop()`.
- **Embedded** → `ensureEmbeddedPostgresConnection(dataDir, port)`:
  - Prepares the native runtime (`prepareEmbeddedPostgresNativeRuntime`, see below).
  - Finds a free port near the preferred one (`findAvailablePort`, 20-port lookahead).
  - **Adopts an already-running cluster** when `postmaster.pid` is missing but a reachable Postgres
    on the preferred port uses the expected data dir; or when a live postmaster PID is found it
    connects to the port from the PID file. Otherwise it constructs a persistent `EmbeddedPostgres`
    (user/password `paperclip`, `initdbFlags: --encoding=UTF8 --locale=C --lc-messages=C`),
    `initialise()`s if `PG_VERSION` is absent, clears a stale `postmaster.pid`, and `start()`s.
  - Always calls `ensurePostgresDatabase(adminUrl, "paperclip")` so the `paperclip` database exists,
    then returns a connection string to it.

`src/migrate.ts` and `src/migration-status.ts` are thin CLIs over these: both call
`resolveMigrationConnection()`, do their work, then `await connection.stop()` in a `finally`.

### Embedded-Postgres native shims & errors

- [embedded-postgres-native.ts](../../packages/db/src/embedded-postgres-native.ts) — on Linux,
  prepends the `@embedded-postgres/<platform>` native `lib` dir to `LD_LIBRARY_PATH` and creates
  `libfoo.so.N` → `libfoo.so.N.M` symlink aliases so the bundled server binaries load. No-op on
  other platforms / when the package isn't present.
- [embedded-postgres-error.ts](../../packages/db/src/embedded-postgres-error.ts) — a rolling log
  buffer (`createEmbeddedPostgresLogBuffer`) plus `formatEmbeddedPostgresError`, which appends a
  shared-memory hint (macOS `kern.sysv.shm*`) and the last few log lines onto startup failures.

## Data flow / execution lifecycle

**Schema change → migration → apply:**

```
edit src/schema/<table>.ts
  → pnpm db:generate
      → check:migrations (validate numbering + journal)
      → tsc  (emit dist/schema/*.js)
      → drizzle-kit generate  (diff dist/schema vs meta/*_snapshot.json)
          → writes src/migrations/NNNN_<name>.sql + meta/NNNN_snapshot.json
          → appends an entry to meta/_journal.json
```

**Applying migrations (server startup or `pnpm db:migrate`):**

```
resolveDatabaseTarget()                 # env → env-file → config → embedded
  → resolveMigrationConnection()        # start/adopt embedded cluster if needed
      → ensurePostgresDatabase(..., "paperclip")
      → inspectMigrations(url)          # upToDate? / needsMigrations + reason
          ├─ empty + no journal   → Drizzle migrate() bootstrap, then reconcile + manual
          ├─ non-empty + no journal → throw (unsafe)
          └─ pending              → reconcilePendingMigrationHistory() then
                                     applyPendingMigrationsManually() (per-file txn, hashed)
      → inspectMigrations(url)          # must be upToDate, else throw
  → connection.stop()                   # stop embedded cluster (no-op for external)
```

**Query path at runtime:** the server calls `createDb(url)` once, holds the `Db`, and issues typed
Drizzle queries against the imported table objects — e.g. `db.insert(issues).values({ companyId, ... })`
as in [seed.ts](../../packages/db/src/seed.ts). Almost every domain query includes a `company_id`
predicate.

## Contracts & cross-layer coupling

- **`@paperclipai/shared`** — schema JSONB columns are typed against shared types
  (`AgentEnvConfig`, `SourceTrustMetadata`, plugin status enums, `PluginStateScopeKind`, …), and
  `runtime-config.ts` imports home-path helpers from `@paperclipai/shared/home-paths`. Changing a
  shared type changes the compile-time shape of stored JSONB.
- **Server startup** owns migration orchestration in production; this package only *provides* the
  primitives. `server/src/index.ts` calls `createDb`, `ensurePostgresDatabase`, `inspectMigrations`,
  `applyPendingMigrations`, and `reconcilePendingMigrationHistory`. It also supports a separate
  `databaseMigrationUrl` (a distinct `createDb` for plugin migrations).
- **Package publishing** — `exports` map ships from `./dist/*` (types + ESM). The `build` script is
  `check:migrations && tsc && cp -r src/migrations dist/migrations`, so the SQL files are copied
  next to the compiled JS; `client.ts` resolves the migrations folder via `import.meta.url`, so it
  finds them under `dist/migrations` at runtime and `src/migrations` under tsx.
- **`__drizzle_migrations` history table** is the durable contract between generation and
  application. The manual applier and reconciler both read/write it and tolerate `name`-, `hash`-,
  or `id`-keyed variants and either the `drizzle` or `public` schema.
- **Drizzle journal** (`meta/_journal.json`) defines canonical ordering; both
  `check-migration-numbering.ts` and `client.ts` (`orderMigrationsByJournal`) depend on it.

## Extension points

- **Add a table**: create `src/schema/<table>.ts` with a `pgTable`, re-export it from
  [src/schema/index.ts](../../packages/db/src/schema/index.ts) (the barrel is what `createDb`
  registers and what consumers import), then `pnpm db:generate`. Include a `company_id` FK + a
  `company_id`-leading index unless the table is genuinely instance- or plugin-scoped.
- **Add a data-only / backfill migration**: hand-author a numbered `.sql` under
  `src/migrations` and add the matching `meta/_journal.json` entry. Data migrations (e.g.
  [0004_issue_identifiers.sql](../../packages/db/src/migrations/0004_issue_identifiers.sql), which
  backfills `issue_number`/`identifier` and syncs `issue_counter`) mix DDL and `UPDATE`s;
  statements the reconciler can't classify will always force a real apply — which is the safe
  default for data changes.
- **Change the storage target**: extend `ResolvedDatabaseTarget` / `resolveDatabaseTarget()` in
  [runtime-config.ts](../../packages/db/src/runtime-config.ts) and the matching branch in
  `resolveMigrationConnection()`.
- **Plugin-owned data**: plugins get their own namespace + migration ledger via
  [plugin_database.ts](../../packages/db/src/schema/plugin_database.ts)
  (`plugin_database_namespaces`, `plugin_migrations`) — a separate mechanism from the host
  migration set documented here.

## Testing

- **Unit / integration tests** live alongside sources (Vitest, config
  [vitest.config.ts](../../packages/db/vitest.config.ts)): e.g.
  [client.test.ts](../../packages/db/src/client.test.ts),
  [runtime-config.test.ts](../../packages/db/src/runtime-config.test.ts),
  [embedded-postgres-error.test.ts](../../packages/db/src/embedded-postgres-error.test.ts),
  [external-objects-schema.test.ts](../../packages/db/src/external-objects-schema.test.ts),
  [pipelines-schema.test.ts](../../packages/db/src/pipelines-schema.test.ts),
  [backup-lib.test.ts](../../packages/db/src/backup-lib.test.ts).
- **Real-DB tests** use [test-embedded-postgres.ts](../../packages/db/src/test-embedded-postgres.ts):
  `startEmbeddedPostgresTestDatabase(prefix)` spins a throwaway embedded cluster in a `mkdtemp`
  dir on a random free port, runs `ensurePostgresDatabase` + `applyPendingMigrations`, and returns
  `{ connectionString, cleanup }`. `getEmbeddedPostgresTestSupport()` probes once whether the host
  can run embedded Postgres at all (memoized), so tests can skip gracefully. Reserved ports (default
  `54329` plus configured ones) are avoided so tests never collide with a dev instance.
- **Migration-integrity gate**: `pnpm --filter @paperclipai/db check:migrations`
  ([check-migration-numbering.ts](../../packages/db/src/check-migration-numbering.ts)) enforces that
  every migration filename starts with a unique, strictly-ordered 4-digit number and that the
  `.sql` files and `_journal.json` entries match 1:1 in count and order. It runs before `build`,
  `typecheck`, `generate`, and `migrate`.

## Gotchas / invariants

- **`drizzle.config.ts` reads compiled output** (`dist/schema/*.js`). Running `drizzle-kit generate`
  without a preceding `tsc` diffs stale JS. The `generate` script chains them; don't bypass it.
- **Migrations are append-only.** `check:migrations` forbids duplicate numbers and out-of-order
  files/journal entries. Editing an already-applied `.sql` changes its `sha256` and can desync the
  history table — the reconciler tolerates *some* drift but do not rely on it for edits.
- **Non-empty DB with no migration journal throws.** `applyPendingMigrations` refuses to
  auto-migrate a populated database that has no `__drizzle_migrations` table; history must be
  initialized manually. Only a truly empty DB is auto-bootstrapped.
- **History table location & keying vary.** It may be in `drizzle` or `public`, and keyed by
  `name`, `hash`, or ordinal `id`. All reads go through `discoverMigrationTableSchema` /
  `loadAppliedMigrations`; never assume `drizzle.__drizzle_migrations(name)`.
- **`nullsNotDistinct()` requires Postgres 15+** and is load-bearing for plugin dedup where
  `company_id` is nullable ([plugin_entities.ts](../../packages/db/src/schema/plugin_entities.ts),
  same pattern in `plugin_state`).
- **Embedded clusters can be adopted, not just started.** If a matching cluster is already running
  (or reachable on the preferred port with the right data dir), `migration-runtime.ts` reuses it and
  returns a no-op `stop()` — so callers must not assume they own the cluster lifecycle.
- **`company_id`-leading indexes are a performance contract.** New tenant-scoped tables should
  index `company_id` first (usually composite with `status`/FKs) to keep tenant queries index-only.
- **`__drizzle_migrations` is included in backups.** The `includeMigrationJournal` backup option is
  deprecated and no longer changes behavior ([backup-lib.ts](../../packages/db/src/backup-lib.ts)).
- **Auth tables use text PKs**, not UUIDs, because they mirror the external auth library's schema
  ([auth.ts](../../packages/db/src/schema/auth.ts)).

## Key files

- [packages/db/src/index.ts](../../packages/db/src/index.ts) — public API barrel.
- [packages/db/src/client.ts](../../packages/db/src/client.ts) — `createDb`, `Db`, and the full
  migration engine.
- [packages/db/src/schema/index.ts](../../packages/db/src/schema/index.ts) — schema barrel (all
  tables).
- [packages/db/src/schema/companies.ts](../../packages/db/src/schema/companies.ts) — tenant root.
- [packages/db/src/schema/issues.ts](../../packages/db/src/schema/issues.ts) — richest table +
  index patterns.
- [packages/db/src/schema/plugin_entities.ts](../../packages/db/src/schema/plugin_entities.ts) —
  nullable-`company_id` tenant-isolation pattern.
- [packages/db/drizzle.config.ts](../../packages/db/drizzle.config.ts) — drizzle-kit config
  (reads `dist/schema`, writes `src/migrations`).
- [packages/db/src/runtime-config.ts](../../packages/db/src/runtime-config.ts) — DB target
  resolution (embedded vs external).
- [packages/db/src/migration-runtime.ts](../../packages/db/src/migration-runtime.ts) — embedded
  cluster lifecycle + `MigrationConnection`.
- [packages/db/src/migrate.ts](../../packages/db/src/migrate.ts) /
  [migration-status.ts](../../packages/db/src/migration-status.ts) — CLIs.
- [packages/db/src/check-migration-numbering.ts](../../packages/db/src/check-migration-numbering.ts)
  — migration-integrity gate.
- [packages/db/src/migrations/](../../packages/db/src/migrations) +
  [meta/_journal.json](../../packages/db/src/migrations/meta/_journal.json) — SQL migrations and
  Drizzle journal metadata.
- [packages/db/src/test-embedded-postgres.ts](../../packages/db/src/test-embedded-postgres.ts) —
  throwaway embedded DB for tests.

## Related docs

- [doc/DATABASE.md](../../doc/DATABASE.md) — operator-facing DB setup and run modes.
- [doc/DEPLOYMENT-MODES.md](../../doc/DEPLOYMENT-MODES.md) — embedded vs external deployment.
