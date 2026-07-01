# Repo Records

Reference for keeping Paperclip's internal repository documentation modular and
indexed. It adapts the `clean-repo-org` record system to this repo's layout:
durable internal records live under [`doc/`](../doc/index.md); the published
product site under [`docs/`](../docs/docs.json) is a separate surface and is not
governed by these rules.

Keep records modular and indexed; do not build a giant dated ledger, and do not
move operating knowledge into long `CLAUDE.md`/`AGENTS.md` files.

## Doc surfaces in this repo

- `doc/` — internal product, developer, and operations docs, plus the modular
  record system below. Not published.
- `docs/` — public Mintlify site (`docs/docs.json` nav). Public-facing.
- `AGENTS.md` / `CLAUDE.md` — thin routers. They link to `doc/` records; they do
  not contain the records.

## Architecture

- Maintain [`doc/architecture/`](../doc/architecture/index.md) when source
  layout, scripts, folders, public entry points, or major workflows change.
- Keep `CLAUDE.md` and `AGENTS.md` as thin routers that cite architecture docs
  instead of duplicating them.
- Keep a short docs-maintenance reminder in both router files, linking to the
  architecture, learning, plans, and practices indexes.
- After each commit-sized task, check whether the architecture docs still match
  the changed structure.
- Prefer one architecture file per concern, with
  [`doc/architecture/index.md`](../doc/architecture/index.md) as the soft index.

## Learning

- Record task learnings under [`doc/learning/`](../doc/learning/README.md).
- Add or update a learning record when work produced a reusable lesson, decision
  rule, or failure mode.
- Small lessons: `doc/learning/YYYY-MM-DD-title.md`. If a lesson can grow, use
  `doc/learning/YYYY-MM-DD-title/` with `index.md` and focused child files.
- Update `doc/learning/README.md` after adding, moving, splitting, or retiring a
  record.
- Include what was learned, evidence, scope, and when to apply it again.
- Redact private identifiers (tokens, emails, account hashes, private/tailnet
  URLs, internal Paperclip issue IDs, local runtime paths) to placeholders unless
  publication was explicitly approved.

## Plans

- Store plans under [`doc/plans/`](../doc/plans/README.md). This matches the
  existing `AGENTS.md` rule that repo plan files use `doc/plans/YYYY-MM-DD-slug.md`.
- Open the dated plan record at the start of a multi-step task and drive the task
  from it; keep it current as work proceeds, not only at the end.
- For plans with phases, logs, or decisions, use `doc/plans/YYYY-MM-DD-title/`
  with `index.md` plus child files (`plan.md`, `progress.md`, `decisions.md`).
- Update `doc/plans/README.md` with links and status summaries only.
- If a Paperclip issue owns the plan, update that issue's `plan` document via the
  `paperclip` skill instead of creating a repo file.

## Practices

- Store reusable practices under [`doc/practices/`](../doc/practices/README.md).
- Write a practice when a task reveals a reusable setup, module pattern, command
  sequence, or operational method.
- Compact practice: `doc/practices/YYYY-MM-DD-title.md`. Growing practice:
  `doc/practices/YYYY-MM-DD-title/`.
- Update `doc/practices/README.md` after adding or changing practices.

## Issues

- Record concrete implementation problems and their resolutions under
  [`doc/issues/`](../doc/issues/README.md) when the detail is worth keeping but
  does not belong in architecture, learning, or practice records.
- Keep issue records focused on the problem, root cause, fix, and verification.

## Splitting long records

When a dated file grows past a single coherent topic:

1. Create `doc/<aspect>/YYYY-MM-DD-title/`.
2. Move the original content into `index.md` or a focused child file.
3. Split unrelated sections into named child files.
4. Replace the old long file with a link, or remove it once references are updated.
5. Update the aspect README.

## Citations

- Use clickable relative markdown links for local citations. From an
  architecture doc under `doc/architecture/`, cite a source file with a
  `../../`-prefixed path — for example, link `server/src/app.ts` as
  `../../server/src/app.ts`.
- Prefer relative links so they survive repo moves.
- Verify every added local link points to an existing target.
- Use stable external URLs only when the source is genuinely external.

## Reorganizing folders or scripts

When asked to reorganize:

- Group files by responsibility, lifecycle, or invocation path.
- Add local README files only when they help navigation.
- Add docstrings/comments for non-obvious modules, public functions, and scripts.
- Preserve behavior unless a behavior change was explicitly requested.
- Update citations, imports, commands, and architecture docs in the same change.
