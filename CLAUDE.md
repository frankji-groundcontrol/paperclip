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
- Default working branch on this host is `franky`. When the user says merge, commit, or push, they mean `franky` unless they name another branch; never target `master` or the upstream default without an explicit instruction.
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
- Internal architecture lives in [`doc/architecture/`](doc/architecture/index.md) (mapped by [`doc/index.md`](doc/index.md)); durable plans/issues/learning/practices live under `doc/` per [`references/repo-records.md`](references/repo-records.md). Keep the relevant architecture doc current when a subsystem changes.
- Public docs navigation lives in `docs/docs.json`.
- For doc moves, use copy-first/link-safe migration: add the replacement, update links/navigation/sources, verify references, then remove old content only when requested.
- Do not expose internal issue IDs, private URLs, local run identifiers, secrets, tailnet links, or `agent://` links in public docs, PRs, commits, comments, or artifact filenames.
