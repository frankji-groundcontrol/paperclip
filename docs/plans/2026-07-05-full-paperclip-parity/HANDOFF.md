# HANDOFF — Finish Full Paperclip Parity (port to Supabase)

**For the next coding agent.** Read this first, then `README.md` and `01-parity-audit-summary.md` in this dir, then `evidence/rewrite-gap-matrix.jsonl` (the work list).

**Goal (user's words):** "harden and perfect with docs/plans/ plan-eng-review and codex TDD cycle, make it through phase 13, do not stop, make sure everything works." Decision locked: **port everything to Supabase** (retire the SQLite control plane at the data layer).

## Where things stand (honest)

- **HEAD:** `4b7de109` on `franky`, **pushed to origin/franky**. 0 commits ahead locally.
- **Gate:** `gaps_missing=102 gaps_partial=77 gaps_divergent=3 rows_total=220 rows_open=182`. **38/220 closed.**
- **Suites green:** backend `cargo test` 304/0, frontend vitest 166/0.
- **Data layer unified on Supabase:** `paperclip` schema on supabase-franky now has **46 tables** (auth/hiring + full V1 control plane), team-scoped RLS, `my_*` read views, `list_*_with_key` forge-proof RPCs. Migrations `0014`–`0020` applied.

Open rows by phase (open / closed):
```
00 oracle            3 / 1     01 schema/authz    38 / 17
02 goals/issues      10 / 0    03 workspaces/plug   5 / 0
04 deploy/onboard     6 / 0    05 adapters/secrets  12 / 0
06 approvals/budget  12 / 9    07 runtime/lifecycle 10 / 6
08 cli/mcp           29 / 2    09 nuxt ui           39 / 1
10 catalogs/export    5 / 0    11 observability      4 / 1
12 contracts/docs     5 / 0    13 cutover            4 / 1
```

## The architecture (understand before coding)

The rewrite had **two parallel paths**; we are unifying on Supabase:
- **Live Supabase path (KEEP, EXTEND):** `supabase/migrations/paperclip/*.sql` on project `supabase-franky`; live backend `backend-rs/src/supabase/{routes,broker,gateway,data,keys}.rs` + `backend-rs/src/jobs/mod.rs`; binary `paperclip-backend` (`backend-rs/src/main.rs`). Currently mounts only `/api/paperclip/companies|jobs|agents|approvals`.
- **SQLite control plane (RETIRE, DO NOT EXTEND):** `backend-rs/src/{issues,goals,projects,runs,routines,plugins,secrets,budgets,costs,activity,pipelines,workspaces,memberships,adapters,environments,llms,company_skills,teams_catalog,dashboard,inbox,sidebar,...}.rs` + `backend-rs/src/db.rs`. Do NOT add features here. Port their behavior to Supabase RPCs + the live backend instead.

**The dispatch pattern to extend** (see `jobs/mod.rs` `JobService`): every method takes a `bearer`. `AuthBroker::session_access_token(bearer)` resolves `pcs_…` sessions to the GoTrue JWT server-side; `paperclip_…` keys are parsed by `parse_api_key`. So: `pcs_` → `rpc("<name>", params, Auth::Bearer(jwt))`; `paperclip_` → `rpc("<name>_with_key", {p_prefix, p_key_hash, …}, Auth::Anon)`. Mirror this for every new domain method.

## Hard constraints (non-negotiable)

1. **No `public` schema** for app objects — only `paperclip` (PostgREST-exposed, RLS) and `paperclip_private` (SECURITY DEFINER helpers, no grants).
2. **No Supabase service-role key** anywhere in app/CLI/MCP/frontend/tests. Anon key + RLS + SECURITY DEFINER helpers only.
3. **No Supabase/OpenAI detail reaches the client.** Browser holds only opaque `pcs_` sessions; broker holds the JWT.
4. **OpenAI Responses API only** (`${OPENAI_BASE_URL}/v1/responses`), not chat completions.
5. **Use the `supabase-franky` MCP**, not `supabase-community` (project ref `hgyjvkuloaouxwdgromz`).
6. **Default branch `franky`.** Commit/push ONLY when the user explicitly asks.
7. **`test_users.json` is gitignored** — real test accounts live there; password `paperclipPasswd1!2@3#`. Never commit it.
8. **Do not fake-close matrix rows.** A row goes to `final_status=full` only after: source evidence cited + failing test observed red + implementation makes it green + live-verified where relevant. Waivers require a `doc/SPEC-implementation.md` §5.2 clause.

## Verification commands

```sh
# gap gate (must reach gaps_missing=0 gaps_partial=0 gaps_divergent=0 to finish)
python3 docs/plans/2026-07-05-full-paperclip-parity/acceptance/run_original_gap_diff.py

# schema parity assertions (operator DB via supabase-franky MCP execute_sql)
# see acceptance/schema_parity_assertions.sql

# suites
cd backend-rs && cargo test --quiet && cargo build --bins
cd frontend-nuxt && npx vitest run

# live backend smoke (kill by exact PID from ss, NOT broad pkill — see pitfalls)
SUPABASE_URL=https://hgyjvkuloaouxwdgromz.supabase.co \
SUPABASE_ANON_KEY=<anon> OPENAI_BASE_URL=$OPENAI_BASE_URL OPENAI_API_KEY=$OPENAI_API_KEY \
PORT=8787 ./target/debug/paperclip-backend
```

## The exact next chunk to build (highest leverage)

**Phase 02 write path + live-backend wiring.** Closes the densest cluster (10 rows) and unblocks Phase 09 (Nuxt UI re-pointing). Recipe:

1. **Migration `0021_issue_goal_project_write_rpcs.sql`** — SECURITY DEFINER RPCs, each with a `*_with_key` api-key variant re-resolving via `paperclip_private.fn_key_principal`:
   - `create_issue` / `create_issue_with_key(p_prefix,p_key_hash,p_company_id,p_title,p_parent_id?,p_assignee_agent_id?,p_status?)`
   - `update_issue` / `update_issue_with_key(...)` (status transitions; enforce single-assignee + atomic checkout for `in_progress`)
   - `add_issue_comment` / `add_issue_comment_with_key(p_prefix,p_key_hash,p_issue_id,p_body)`
   - `create_goal` / `create_project` (+ `_with_key`), with parent/owner validation
   - Insert an `activity_log` row on each mutating action.
2. **`backend-rs/src/jobs/mod.rs`** — add `create_issue`, `update_issue`, `list_issues`, `add_issue_comment`, `list_issue_comments`, `create_goal`, `list_goals`, `create_project`, `list_projects` methods, dispatching on bearer exactly like `hire_agent`. (Read RPCs already exist as `list_*_with_key` from migration 0020; add session-path `my_*` view reads.)
3. **`backend-rs/src/supabase/routes.rs`** — mount:
   - `POST/GET /api/paperclip/companies/:companyId/issues`, `POST /api/paperclip/issues/:issueId/comments`, `GET /api/paperclip/issues/:issueId/comments`
   - `POST/GET /api/paperclip/companies/:companyId/goals`, `POST/GET /api/paperclip/companies/:companyId/projects`
   Reuse `bearer_token_from_headers` + `map_job_error`.
4. **TDD:** add `backend-rs/tests/issues_parity.rs` (axum router test with a spy `FakeData`, mirroring `tests/paperclip_routes.rs`) — RED first, then GREEN. Add a live check via the broker against supabase-franky.
5. **Close matrix rows** `issues-*`, `comments-*`, `goals-hierarchy-*`, `projects-*` in `evidence/rewrite-gap-matrix.jsonl` with `final_status=full` + evidence.

**Then** Phase 07 runtime (heartbeat_runs lifecycle: `queued→running→idle|error`, wakeup fulfillment, run cancellation) — the next biggest single subsystem. Then Phase 08 (CLI/MCP commands for all new domains) and Phase 09 (re-point the existing Nuxt components at the Supabase RPCs).

## Matrix closure workflow (per chunk)

```sh
# 1. edit evidence/rewrite-gap-matrix.jsonl: set final_status=full + evidence_ref for rows you verified
# 2. regenerate the markdown summary:
python3 - <<'PY'
import json
from pathlib import Path
from collections import Counter, defaultdict
base=Path('docs/plans/2026-07-05-full-paperclip-parity')
rows=[json.loads(l) for l in (base/'evidence/rewrite-gap-matrix.jsonl').read_text().splitlines() if l.strip()]
def closed(r): return r['initial_status']=='full' or r.get('final_status')=='full' or (r.get('final_status')=='waived' and r.get('waiver_ref'))
oc=Counter(r['initial_status'] for r in rows if not closed(r))
print('open:', dict(oc), 'closed:', sum(1 for r in rows if closed(r)), '/', len(rows))
PY
# 3. confirm the gate moved:
python3 docs/plans/2026-07-05-full-paperclip-parity/acceptance/run_original_gap_diff.py
```

## Applying migrations

Use the **`supabase-franky` MCP `apply_migration`** tool (one migration = one transaction). Update `evidence/migration-ledger.md` in the same change. **Postgres gotcha:** `ALTER TYPE … ADD VALUE` cannot be used later in the *same* transaction it's added — split enum-add and enum-use into separate migrations (see how `0014` adds values and `0015` uses them).

## Codex TDD (the user wants this used)

```sh
# bounded prompt (codex hangs on huge reads); write prompt to a file, redirect stdin
codex exec --dangerously-bypass-approvals-and-sandbox -c model_reasoning_effort=high "$(cat /tmp/prompt.txt)" < /dev/null
# keep prompts to ONE concrete deliverable; have codex AUTHOR files, then you apply/verify via MCP
```

## Pitfalls logged (save yourself the time)

- **Codex hangs/times out** on prompts that ask it to read many files or author huge artifacts. Keep prompts tight: one file, one behavior. Have it author; you apply + verify.
- **Workflow tool `args` caps at 4096 array elements.** To pass big data into a workflow, inline it as a `const` literal in the script body (or write the script to a file and invoke via `scriptPath`), not via `args`.
- **Killing the test server:** use `ss -ltnp | grep :8787` → exact PID → `kill <pid>`. Do NOT `pkill -f paperclip-backend` or `pkill -f codex` — it self-matches your shell (exit 144). Don't touch the unrelated `insideout` project's process on port 3000.
- **Company hire-approval default is now `false`** (0014). To test approval flows, flip the company via `execute_sql` first.
- **`agents.status` runtime values are `idle/running/error`**, not `active`. `active` is in the enum but no flow sets it. `fn_agent_active` checks `status in ('idle','running')`.
- **Frontend test fixtures** must use canonical statuses (`idle`/`terminated`, not `active`/`archived`).

## Key file map

```
docs/plans/2026-07-05-full-paperclip-parity/   # THIS plan + matrix + harnesses + handoff
  evidence/rewrite-gap-matrix.jsonl            # THE WORK LIST (220 rows, edit to close)
  evidence/migration-ledger.md                 # update when you add a migration
  acceptance/run_original_gap_diff.py          # the gate
  acceptance/schema_parity_assertions.sql      # schema checks (run via MCP execute_sql)
  phases/02-goals-projects-issues-work-products.md   # NEXT phase spec
supabase/migrations/paperclip/0014..0020.sql   # applied; continue 0021+
backend-rs/src/jobs/mod.rs                     # EXTEND: add domain methods (dispatch pattern)
backend-rs/src/supabase/routes.rs              # EXTEND: mount new routes
backend-rs/src/bin/paperclip.rs                # CLI (Phase 08)
backend-rs/src/bin/paperclip-mcp.rs            # MCP (Phase 08)
frontend-nuxt/components/                      # RE-POINT at Supabase RPCs (Phase 09)
packages/db/src/schema/*.ts                    # original Drizzle schema (column source of truth)
server/src/routes/{agents,approvals,issues,...}.ts  # original behavior (parity target)
```

## Definition of done for the whole goal

`run_original_gap_diff.py` prints `gaps_missing=0 gaps_partial=0 gaps_divergent=0`, both suites green, live real-user/real-agent scenario passes against supabase-franky with a real OpenAI Responses run. No waivers unless tied to a `doc/SPEC-implementation.md` §5.2 update.

— End of handoff. Pick up at "The exact next chunk to build."
