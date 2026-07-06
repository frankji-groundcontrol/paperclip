Reading additional input from stdin...
2026-07-05T23:14:47.920487Z ERROR codex_models_manager::manager: failed to refresh available models: stream disconnected before completion: failed to decode models response: expected value at line 1 column 1; body: <!doctype html>
<html lang="zh-CN">
  <head>
    <meta charset="UTF-8" />
    <link rel="icon" type="image/png" href="/logo.png" />
    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
    <title>Sub2API - AI API Gateway</title>
    <script type="module" crossorigin src="/assets/index-DdX3yYPo.js"></script>
    <link rel="modulepreload" crossorigin href="/assets/vendor-vue-kBc6jvPi.js">
    <link rel="modulepreload" crossorigin href="/assets/vendor-i18n-BrzyAgMl.js">
    <link rel="modulepreload" crossorigin href="/assets/vendor-misc-CYcWZl9Q.js">
    <link rel="stylesheet" crossorigin href="/assets/vendor-misc-DB0Q8XAf.css">
    <link rel="stylesheet" crossorigin href="/assets/index-BlQao_aQ.css">
  <script nonce="U7WPzUIizBv63dSYBCEX9Q==">window.__APP_CONFIG__={"registration_enabled":false,"email_verify_enabled":false,"registration_email_suffix_whitelist":[],"promo_code_enabled":true,"password_reset_enabled":false,"invitation_code_enabled":false,"totp_enabled":false,"login_agreement_enabled":false,"login_agreement_mode":"modal","login_agreement_updated_at":"2026-03-31","login_agreement_revision":"0eff189d66e0af4e","login_agreement_documents":[{"id":"terms","title":"服务条款","content_md":""},{"id":"usage-policy","title":"使用政策","content_md":""},{"id":"supported-regions","title":"支持的国家和地区","content_md":""},{"id":"service-specific-terms","title":"服务特定条款","content_md":""}],"turnstile_enabled":false,"turnstile_site_key":"","site_name":"Sub2API","site_logo":"","site_subtitle":"Subscription to API Conversion Platform","api_base_url":"","contact_info":"","doc_url":"","home_content":"","hide_ccs_import_button":false,"purchase_subscription_enabled":false,"purchase_subscription_url":"","table_default_page_size":20,"table_page_size_options":[10,20,50],"custom_menu_items":[],"custom_endpoints":[],"linuxdo_oauth_enabled":false,"dingtalk_oauth_enabled":false,"wechat_oauth_enabled":false,"wechat_oauth_open_enabled":false,"wechat_oauth_mp_enabled":false,"wechat_oauth_mobile_enabled":false,"oidc_oauth_enabled":false,"oidc_oauth_provider_name":"OIDC","github_oauth_enabled":false,"google_oauth_enabled":false,"backend_mode_enabled":false,"payment_enabled":false,"version":"","server_timezone":"Asia/Shanghai","server_utc_offset":"+08:00","balance_low_notify_enabled":false,"account_quota_notify_enabled":false,"balance_low_notify_threshold":0,"balance_low_notify_recharge_url":"","channel_monitor_enabled":true,"channel_monitor_default_interval_seconds":60,"available_channels_enabled":false,"affiliate_enabled":false,"risk_control_enabled":false,"allow_user_view_error_requests":false};</script></head>
  <body>
    <div id="app"></div>
  </body>
</html>

OpenAI Codex v0.141.0
--------
workdir: /home/frankji/Projects/paperclip
model: gpt-5.5
provider: OpenAI
approval: never
sandbox: danger-full-access
reasoning effort: low
reasoning summaries: none
session id: 019f3490-1df1-7cd2-a359-06da636b1bbe
--------
user
Final plan re-review (read-only, do not edit). Check only these files: docs/plans/2026-07-05-full-paperclip-parity/README.md, 01-parity-audit-summary.md, 04-execution-order.md, 05-test-strategy.md, phases/00-parity-oracle-and-matrix.md, acceptance/full-parity-harness-spec.md, acceptance/run_original_gap_diff.py, acceptance/run_real_user_real_agent.py, acceptance/run_cli_mcp_surface.py, acceptance/run_frontend_user_flow.py, codex/phase-prompts.md, reviews/review-checklist.md, evidence/rewrite-gap-matrix.md, evidence/rewrite-gap-matrix.jsonl, and the phase filenames under phases/. Prior blockers were: (1) exact gaps_* output, (2) live harness placeholders, (3) matrix not assigning rows to all 14 phases. Confirm whether each is now resolved. The plan is a PLAN artifact: harness scaffolds that intentionally return non-zero are acceptable as Phase 00 RED deliverables as long as Phase 00 explicitly owns implementing them before release. Return BLOCKERS, PATCHES, VERDICT (PASS or FAIL).
2026-07-05T23:14:48.638177Z ERROR rmcp::transport::worker: worker quit with fatal: Transport channel closed, when Auth(AuthorizationRequired)
hook: SessionStart
hook: SessionStart Completed
hook: UserPromptSubmit
hook: UserPromptSubmit Completed
codex
I’ll do a read-only review of only the named plan files and phase filenames, then map the three prior blockers directly to evidence in those files.
hook: PreToolUse
hook: PreToolUse Completed
exec
/bin/bash -lc "sed -n '1,220p' /home/frankji/.codex/superpowers/skills/using-superpowers/SKILL.md && sed -n '1,220p' /home/frankji/.codex/superpowers/skills/receiving-code-review/SKILL.md" in /home/frankji/Projects/paperclip
 succeeded in 0ms:
---
name: using-superpowers
description: Use when starting any conversation - establishes how to find and use skills, requiring Skill tool invocation before ANY response including clarifying questions
---

<SUBAGENT-STOP>
If you were dispatched as a subagent to execute a specific task, skip this skill.
</SUBAGENT-STOP>

<EXTREMELY-IMPORTANT>
If you think there is even a 1% chance a skill might apply to what you are doing, you ABSOLUTELY MUST invoke the skill.

IF A SKILL APPLIES TO YOUR TASK, YOU DO NOT HAVE A CHOICE. YOU MUST USE IT.

This is not negotiable. This is not optional. You cannot rationalize your way out of this.
</EXTREMELY-IMPORTANT>

## Instruction Priority

Superpowers skills override default system prompt behavior, but **user instructions always take precedence**:

1. **User's explicit instructions** (CLAUDE.md, GEMINI.md, AGENTS.md, direct requests) — highest priority
2. **Superpowers skills** — override default system behavior where they conflict
3. **Default system prompt** — lowest priority

If CLAUDE.md, GEMINI.md, or AGENTS.md says "don't use TDD" and a skill says "always use TDD," follow the user's instructions. The user is in control.

## How to Access Skills

**In Claude Code:** Use the `Skill` tool. When you invoke a skill, its content is loaded and presented to you—follow it directly. Never use the Read tool on skill files.

**In Copilot CLI:** Use the `skill` tool. Skills are auto-discovered from installed plugins. The `skill` tool works the same as Claude Code's `Skill` tool.

**In Gemini CLI:** Skills activate via the `activate_skill` tool. Gemini loads skill metadata at session start and activates the full content on demand.

**In other environments:** Check your platform's documentation for how skills are loaded.

## Platform Adaptation

Skills use Claude Code tool names. Non-CC platforms: see `references/copilot-tools.md` (Copilot CLI), `references/codex-tools.md` (Codex) for tool equivalents. Gemini CLI users get the tool mapping loaded automatically via GEMINI.md.

# Using Skills

## The Rule

**Invoke relevant or requested skills BEFORE any response or action.** Even a 1% chance a skill might apply means that you should invoke the skill to check. If an invoked skill turns out to be wrong for the situation, you don't need to use it.

```dot
digraph skill_flow {
    "User message received" [shape=doublecircle];
    "About to EnterPlanMode?" [shape=doublecircle];
    "Already brainstormed?" [shape=diamond];
    "Invoke brainstorming skill" [shape=box];
    "Might any skill apply?" [shape=diamond];
    "Invoke Skill tool" [shape=box];
    "Announce: 'Using [skill] to [purpose]'" [shape=box];
    "Has checklist?" [shape=diamond];
    "Create TodoWrite todo per item" [shape=box];
    "Follow skill exactly" [shape=box];
    "Respond (including clarifications)" [shape=doublecircle];

    "About to EnterPlanMode?" -> "Already brainstormed?";
    "Already brainstormed?" -> "Invoke brainstorming skill" [label="no"];
    "Already brainstormed?" -> "Might any skill apply?" [label="yes"];
    "Invoke brainstorming skill" -> "Might any skill apply?";

    "User message received" -> "Might any skill apply?";
    "Might any skill apply?" -> "Invoke Skill tool" [label="yes, even 1%"];
    "Might any skill apply?" -> "Respond (including clarifications)" [label="definitely not"];
    "Invoke Skill tool" -> "Announce: 'Using [skill] to [purpose]'";
    "Announce: 'Using [skill] to [purpose]'" -> "Has checklist?";
    "Has checklist?" -> "Create TodoWrite todo per item" [label="yes"];
    "Has checklist?" -> "Follow skill exactly" [label="no"];
    "Create TodoWrite todo per item" -> "Follow skill exactly";
}
```

## Red Flags

These thoughts mean STOP—you're rationalizing:

| Thought | Reality |
|---------|---------|
| "This is just a simple question" | Questions are tasks. Check for skills. |
| "I need more context first" | Skill check comes BEFORE clarifying questions. |
| "Let me explore the codebase first" | Skills tell you HOW to explore. Check first. |
| "I can check git/files quickly" | Files lack conversation context. Check for skills. |
| "Let me gather information first" | Skills tell you HOW to gather information. |
| "This doesn't need a formal skill" | If a skill exists, use it. |
| "I remember this skill" | Skills evolve. Read current version. |
| "This doesn't count as a task" | Action = task. Check for skills. |
| "The skill is overkill" | Simple things become complex. Use it. |
| "I'll just do this one thing first" | Check BEFORE doing anything. |
| "This feels productive" | Undisciplined action wastes time. Skills prevent this. |
| "I know what that means" | Knowing the concept ≠ using the skill. Invoke it. |

## Skill Priority

When multiple skills could apply, use this order:

1. **Process skills first** (brainstorming, debugging) - these determine HOW to approach the task
2. **Implementation skills second** (frontend-design, mcp-builder) - these guide execution

"Let's build X" → brainstorming first, then implementation skills.
"Fix this bug" → debugging first, then domain-specific skills.

## Skill Types

**Rigid** (TDD, debugging): Follow exactly. Don't adapt away discipline.

**Flexible** (patterns): Adapt principles to context.

The skill itself tells you which.

## User Instructions

Instructions say WHAT, not HOW. "Add X" or "Fix Y" doesn't mean skip workflows.
---
name: receiving-code-review
description: Use when receiving code review feedback, before implementing suggestions, especially if feedback seems unclear or technically questionable - requires technical rigor and verification, not performative agreement or blind implementation
---

# Code Review Reception

## Overview

Code review requires technical evaluation, not emotional performance.

**Core principle:** Verify before implementing. Ask before assuming. Technical correctness over social comfort.

## The Response Pattern

```
WHEN receiving code review feedback:

1. READ: Complete feedback without reacting
2. UNDERSTAND: Restate requirement in own words (or ask)
3. VERIFY: Check against codebase reality
4. EVALUATE: Technically sound for THIS codebase?
5. RESPOND: Technical acknowledgment or reasoned pushback
6. IMPLEMENT: One item at a time, test each
```

## Forbidden Responses

**NEVER:**
- "You're absolutely right!" (explicit CLAUDE.md violation)
- "Great point!" / "Excellent feedback!" (performative)
- "Let me implement that now" (before verification)

**INSTEAD:**
- Restate the technical requirement
- Ask clarifying questions
- Push back with technical reasoning if wrong
- Just start working (actions > words)

## Handling Unclear Feedback

```
IF any item is unclear:
  STOP - do not implement anything yet
  ASK for clarification on unclear items

WHY: Items may be related. Partial understanding = wrong implementation.
```

**Example:**
```
your human partner: "Fix 1-6"
You understand 1,2,3,6. Unclear on 4,5.

❌ WRONG: Implement 1,2,3,6 now, ask about 4,5 later
✅ RIGHT: "I understand items 1,2,3,6. Need clarification on 4 and 5 before proceeding."
```

## Source-Specific Handling

### From your human partner
- **Trusted** - implement after understanding
- **Still ask** if scope unclear
- **No performative agreement**
- **Skip to action** or technical acknowledgment

### From External Reviewers
```
BEFORE implementing:
  1. Check: Technically correct for THIS codebase?
  2. Check: Breaks existing functionality?
  3. Check: Reason for current implementation?
  4. Check: Works on all platforms/versions?
  5. Check: Does reviewer understand full context?

IF suggestion seems wrong:
  Push back with technical reasoning

IF can't easily verify:
  Say so: "I can't verify this without [X]. Should I [investigate/ask/proceed]?"

IF conflicts with your human partner's prior decisions:
  Stop and discuss with your human partner first
```

**your human partner's rule:** "External feedback - be skeptical, but check carefully"

## YAGNI Check for "Professional" Features

```
IF reviewer suggests "implementing properly":
  grep codebase for actual usage

  IF unused: "This endpoint isn't called. Remove it (YAGNI)?"
  IF used: Then implement properly
```

**your human partner's rule:** "You and reviewer both report to me. If we don't need this feature, don't add it."

## Implementation Order

```
FOR multi-item feedback:
  1. Clarify anything unclear FIRST
  2. Then implement in this order:
     - Blocking issues (breaks, security)
     - Simple fixes (typos, imports)
     - Complex fixes (refactoring, logic)
  3. Test each fix individually
  4. Verify no regressions
```

## When To Push Back

Push back when:
- Suggestion breaks existing functionality
- Reviewer lacks full context
- Violates YAGNI (unused feature)
- Technically incorrect for this stack
- Legacy/compatibility reasons exist
- Conflicts with your human partner's architectural decisions

**How to push back:**
- Use technical reasoning, not defensiveness
- Ask specific questions
- Reference working tests/code
- Involve your human partner if architectural

**Signal if uncomfortable pushing back out loud:** "Strange things are afoot at the Circle K"

## Acknowledging Correct Feedback

When feedback IS correct:
```
✅ "Fixed. [Brief description of what changed]"
✅ "Good catch - [specific issue]. Fixed in [location]."
✅ [Just fix it and show in the code]

❌ "You're absolutely right!"
❌ "Great point!"
❌ "Thanks for catching that!"
❌ "Thanks for [anything]"
❌ ANY gratitude expression
```

**Why no thanks:** Actions speak. Just fix it. The code itself shows you heard the feedback.

**If you catch yourself about to write "Thanks":** DELETE IT. State the fix instead.

## Gracefully Correcting Your Pushback

If you pushed back and were wrong:
```
✅ "You were right - I checked [X] and it does [Y]. Implementing now."
✅ "Verified this and you're correct. My initial understanding was wrong because [reason]. Fixing."

❌ Long apology
❌ Defending why you pushed back
❌ Over-explaining
```

State the correction factually and move on.

## Common Mistakes

| Mistake | Fix |
|---------|-----|
| Performative agreement | State requirement or just act |
| Blind implementation | Verify against codebase first |
| Batch without testing | One at a time, test each |
| Assuming reviewer is right | Check if breaks things |
| Avoiding pushback | Technical correctness > comfort |
| Partial implementation | Clarify all items first |
| Can't verify, proceed anyway | State limitation, ask for direction |

## Real Examples

**Performative Agreement (Bad):**
```
Reviewer: "Remove legacy code"
❌ "You're absolutely right! Let me remove that..."
```

**Technical Verification (Good):**
```
Reviewer: "Remove legacy code"
✅ "Checking... build target is 10.15+, this API needs 13+. Need legacy for backward compat. Current impl has wrong bundle ID - fix it or drop pre-13 support?"
```

**YAGNI (Good):**
```
Reviewer: "Implement proper metrics tracking with database, date filters, CSV export"
✅ "Grepped codebase - nothing calls this endpoint. Remove it (YAGNI)? Or is there usage I'm missing?"
```

**Unclear Item (Good):**
```
your human partner: "Fix items 1-6"
You understand 1,2,3,6. Unclear on 4,5.
✅ "Understand 1,2,3,6. Need clarification on 4 and 5 before implementing."
```

## GitHub Thread Replies

When replying to inline review comments on GitHub, reply in the comment thread (`gh api repos/{owner}/{repo}/pulls/{pr}/comments/{id}/replies`), not as a top-level PR comment.

## The Bottom Line

**External feedback = suggestions to evaluate, not orders to follow.**

Verify. Question. Then implement.

No performative agreement. Technical rigor always.

hook: PostToolUse
hook: PostToolUse Completed
codex
Using `using-superpowers` for required workflow setup and `receiving-code-review` because this is a re-review of prior blockers. I’m keeping this read-only and will not modify files.
hook: PreToolUse
hook: PreToolUse Completed
exec
/bin/bash -lc "find docs/plans/2026-07-05-full-paperclip-parity/phases -maxdepth 1 -type f -printf '%f\\n' | sort" in /home/frankji/Projects/paperclip
 succeeded in 0ms:
00-parity-oracle-and-matrix.md
01-schema-authz-parity.md
02-goals-projects-issues-work-products.md
03-workspaces-routines-plugins-sandbox.md
04-deployment-auth-onboarding-ops.md
05-adapters-secrets-skills-config.md
06-approvals-budgets-costs-activity.md
07-agent-runtime-lifecycle.md
08-interfaces-rest-cli-mcp.md
09-nuxt-board-ui-parity.md
10-import-export-catalogs-portability.md
11-observability-feedback-pipelines.md
12-shared-contract-docs-parity.md
13-migration-compatibility-cutover.md

hook: PreToolUse
hook: PostToolUse
hook: PreToolUse Completed
hook: PreToolUse
hook: PostToolUse Completed
hook: PreToolUse Completed
hook: PreToolUse
hook: PreToolUse
hook: PreToolUse Completed
hook: PreToolUse Completed
exec
/bin/bash -lc "sed -n '1,240p' docs/plans/2026-07-05-full-paperclip-parity/README.md" in /home/frankji/Projects/paperclip
 succeeded in 0ms:
# Full Paperclip Parity Implementation Plan

> **Goal:** eliminate the verified parity gap between the current Rust/Supabase/Nuxt rewrite and original Paperclip V1 for agent hiring, governance, runtime, interfaces, and real work execution.
>
> **Definition of success:** the rewrite reaches **zero known parity gaps** against `doc/SPEC-implementation.md` and the original TypeScript implementation evidence cited here. A working vertical slice is not enough.

## Directory map

- [`00-overview.md`](00-overview.md) — scope, success criteria, and non-negotiable invariants.
- [`01-parity-audit-summary.md`](01-parity-audit-summary.md) — corrected audit baseline: 3 full / 50 partial / 100 missing / 22 divergent.
- [`02-target-architecture.md`](02-target-architecture.md) — target Rust/Supabase/Nuxt architecture that preserves original V1 contracts.
- [`03-file-structure.md`](03-file-structure.md) — planned module/file layout for schema, backend, CLI/MCP, Nuxt, and test harnesses.
- [`04-execution-order.md`](04-execution-order.md) — dependency-ordered phases for TDD/Codex workers.
- [`05-test-strategy.md`](05-test-strategy.md) — red/green/unit/integration/live acceptance strategy.
- [`06-risk-register.md`](06-risk-register.md) — security, data, runtime, migration, and external-service risks.
- [`phases/`](phases/) — implementation workstreams, each modular and reviewable.
- [`acceptance/`](acceptance/) — parity harness specs and real-user/real-agent scenarios.
- [`codex/`](codex/) — Codex execution/review prompts.
- [`reviews/`](reviews/) — plan-eng-review and Codex review artifacts.
- [`evidence/`](evidence/) — original-source map and rewrite gap matrix.

## How to use this plan

1. Treat [`01-parity-audit-summary.md`](01-parity-audit-summary.md) as the starting contract; do not erase gaps without source-backed implementation and tests.
2. Implement phases in [`04-execution-order.md`](04-execution-order.md) order. Each phase is designed for Codex TDD workers.
3. After each phase, run the narrow phase checks plus the full parity harness from [`acceptance/full-parity-harness-spec.md`](acceptance/full-parity-harness-spec.md).
4. Completion requires the final Codex parity review to produce no remaining missing/divergent/partial items unless explicitly marked out-of-scope by a revised product contract.

## Non-negotiables

- Use only Supabase schemas `paperclip` and `paperclip_private`; **do not use `public`** for application tables/functions.
- Do not use a Supabase service-role key in app, CLI, MCP, frontend, or test acceptance flows.
- No Supabase/OpenAI credentials or implementation details reach the client.
- Preserve company boundaries, agent low-trust boundaries, approval gates, budgets, activity logs, and secret redaction.
- Use OpenAI **Responses API**, not chat completions, for OpenAI-backed adapter/job paths.

## Plan-design review revision

The first written plan had 7 phases. Ultracode plan-design review found that still under-covered original V1. This revision expands the plan to 14 owned phases, including goals/projects/issues, deployment/auth/onboarding, workspaces/routines/plugins, import/export/catalogs, observability/pipelines, shared contracts/docs, and cutover.


## Row-level ledger note

The first audit synthesis reported `3 full / 48 partial / 98 missing / 24 divergent`. Converting the audit journals into a row-level closure ledger exposed two counter mismatches inside focused retry outputs. The plan therefore uses `evidence/rewrite-gap-matrix.jsonl` as the authoritative source: `3 full / 50 partial / 100 missing / 22 divergent` across 175 rows.

hook: PostToolUse
hook: PostToolUse Completed
exec
/bin/bash -lc "sed -n '1,260p' docs/plans/2026-07-05-full-paperclip-parity/01-parity-audit-summary.md" in /home/frankji/Projects/paperclip
 succeeded in 0ms:
# Parity Audit Summary

## Corrected audit result

Adversarial workflow audit + focused retries produced an initial synthesized tally of `3 full / 48 partial / 98 missing / 24 divergent`. When converted into a row-level closure ledger, the focused retry item lists were more precise than their declared counters, yielding the authoritative row-level tally below.

| Status | Count |
|---|---:|
| Full | 3 |
| Partial | 50 |
| Missing | 100 |
| Divergent | 22 |

**Authoritative source:** `evidence/rewrite-gap-matrix.jsonl`. Workers close rows, not summary buckets.

**Verdict:** current rewrite is **core-slice-only**, not full parity.

## Critical direct divergences

### Company hire-approval default

Original:

- `packages/db/src/migrations/0071_default_hire_approval_off.sql:1`
- `doc/SPEC-implementation.md:139`

```sql
ALTER TABLE "companies" ALTER COLUMN "require_board_approval_for_new_agents" SET DEFAULT false;
```

Rewrite:

- `supabase/migrations/paperclip/0013_agents_hiring.sql:6-7`

```sql
alter table paperclip.companies
  add column if not exists require_board_approval_for_new_agents boolean not null default true;
```

Plan requirement: default must be `false`; direct-create/hire semantics must match original.

### Agent statuses

Original:

- `packages/shared/src/constants.ts:19-27`

```ts
active | paused | idle | running | error | pending_approval | terminated
```

Rewrite:

- `supabase/migrations/paperclip/0013_agents_hiring.sql:9-12`

```sql
pending_approval | active | paused | archived
```

Plan requirement: restore canonical status vocabulary and lifecycle semantics. Approved/runnable agents are `idle`; execution moves `idle -> running -> idle/error`; rejection/termination uses `terminated` and revokes keys.

### Approval types and statuses

Original:

- `packages/shared/src/constants.ts:538-552`

Types: `hire_agent`, `approve_ceo_strategy`, `budget_override_required`, `request_board_approval`.

Statuses: `pending`, `revision_requested`, `approved`, `rejected`, `cancelled`.

Rewrite:

- `supabase/migrations/paperclip/0013_agents_hiring.sql:13-18`

Only `hire_agent`, `approve_ceo_strategy`; no `revision_requested`.

Plan requirement: restore approval enum/status contract and side-effect behavior.

## Highest-priority gaps

1. **Control-plane contract repairs** — schema defaults/enums/transitions/permission keys.
2. **Original authorization model** — company memberships, board actor semantics, permission grants, low-trust policy checks.
3. **Agent runtime/lifecycle** — heartbeat runs, wakeups, cancellation, logs/events, runtime state, task sessions.
4. **Approval governance** — generic approvals, decision notes, revision flow, budget/request-board approval types, activity side effects.
5. **Adapter/config/secrets/skills** — adapter registry, runtime config, instructions bundles, Codex isolation, secret refs/redaction, desired skills.
6. **Tasks/issues/work products** — assignment, checkout, comments, documents, artifacts, recovery/work product flows.
7. **Interfaces** — UI/CLI/MCP/API parity for the above.
8. **Acceptance** — real users + real agents + real OpenAI Responses + parity diff harness.

hook: PostToolUse
hook: PostToolUse Completed
exec
/bin/bash -lc "sed -n '1,260p' docs/plans/2026-07-05-full-paperclip-parity/04-execution-order.md" in /home/frankji/Projects/paperclip
exec
/bin/bash -lc "sed -n '1,260p' docs/plans/2026-07-05-full-paperclip-parity/05-test-strategy.md" in /home/frankji/Projects/paperclip
 succeeded in 0ms:
# Execution Order

The first draft had seven phases. Plan-design review found that was still too narrow for full parity. The locked plan uses fourteen phases so every original Paperclip control-plane area has an owner.

## Phase order

0. [`00-parity-oracle-and-matrix.md`](phases/00-parity-oracle-and-matrix.md) — build the failing parity oracle and row-level matrix.
1. [`01-schema-authz-parity.md`](phases/01-schema-authz-parity.md) — schema/default/enums/authorization/permission grants.
2. [`02-goals-projects-issues-work-products.md`](phases/02-goals-projects-issues-work-products.md) — company mission, goals, projects, issues, comments, documents, artifacts.
3. [`04-deployment-auth-onboarding-ops.md`](phases/04-deployment-auth-onboarding-ops.md) — local/authenticated modes, onboarding, bootstrap, ops safety.
4. [`05-adapters-secrets-skills-config.md`](phases/05-adapters-secrets-skills-config.md) — adapter config, secrets, skills, config revisions.
5. [`06-approvals-budgets-costs-activity.md`](phases/06-approvals-budgets-costs-activity.md) — approval lifecycle, budgets, cost rollups, activity logs.
6. [`07-agent-runtime-lifecycle.md`](phases/07-agent-runtime-lifecycle.md) — canonical statuses, heartbeat runtime, wakeups, cancellation, logs/events.
7. [`03-workspaces-routines-plugins-sandbox.md`](phases/03-workspaces-routines-plugins-sandbox.md) — runtime workspaces, routines, plugin sandbox/current-addendum surfaces.
8. [`08-interfaces-rest-cli-mcp.md`](phases/08-interfaces-rest-cli-mcp.md) — REST, CLI, MCP parity after domain APIs exist.
9. [`09-nuxt-board-ui-parity.md`](phases/09-nuxt-board-ui-parity.md) — human board UI parity.
10. [`10-import-export-catalogs-portability.md`](phases/10-import-export-catalogs-portability.md) — catalogs, templates, export/import.
11. [`11-observability-feedback-pipelines.md`](phases/11-observability-feedback-pipelines.md) — observability/productivity/pipeline current-addendum classification and implementation.
12. [`12-shared-contract-docs-parity.md`](phases/12-shared-contract-docs-parity.md) — shared constants/validators/docs/OpenAPI contract sync.
13. [`13-migration-compatibility-cutover.md`](phases/13-migration-compatibility-cutover.md) — compatibility, cutover, rollback.

## Dependency rules

- Phase 00 blocks all implementation. No worker can close a gap that is not represented in the matrix.
- Phase 01 blocks all domain phases. The wrong status/default/authz contracts make later work invalid.
- Domain backend phases must land before their UI/CLI/MCP parity rows.
- Live Supabase/OpenAI acceptance is final proof, not a substitute for deterministic local red/green tests.
- A waiver for any V1-core row requires a product-spec update and reviewer signoff.

## Stop condition

The plan is complete only when `acceptance/run_original_gap_diff.py` reports:

```text
gaps_missing=0 gaps_partial=0 gaps_divergent=0
```

or every non-full row is explicitly waived by an updated product contract.


## Row-level closure rule

Each phase owns explicit rows in `evidence/rewrite-gap-matrix.md`. A phase cannot close from a category-level pass. Every row must end with:

```text
phase=<owner> acceptance_id=<id> failing_test=<path::name> passing_test=<command> evidence_ref=<source+rewrite> final_status=<full|waived>
```

The final cutover phase must fail if any blocking row remains `partial`, `missing`, `divergent`, or lacks a reviewed waiver.

 succeeded in 0ms:
# Test Strategy

## TDD rule

Each phase begins with failing tests that encode original parity. Production changes are not accepted until those tests pass.

## Test layers

### Unit tests

- Authz predicate matrix.
- Status transition functions.
- Approval decision side effects.
- Adapter config normalization/redaction.
- Secret ref validation.
- Budget hard-stop calculations.

### Route/service tests

- Rust axum route tests for every endpoint.
- Supabase gateway spy tests proving session vs API-key dispatch.
- Error mapping tests for 400/401/403/404/409/422.

### Migration/RLS tests

- Live Supabase introspection for schemas/tables/functions/grants.
- RLS tests for board user, normal member, viewer, agent key, low-trust agent key, cross-company attempts.
- SECURITY DEFINER `search_path=''` checks.

### Frontend tests

- Composable tests for every API client.
- Component/page tests for hire, approvals, lifecycle, runtime, access, budget, tasks.
- No Supabase/OpenAI details in rendered output or responses.

### CLI/MCP tests

- CLI command parsing and server calls.
- MCP tool schemas and JSON-RPC smoke.
- Agent-key vs board-session permissions.

### Live acceptance

- Real users: `testadmin1`, `testuser1`, `testuser2`, `testuser3` per `test_users.json` (gitignored).
- Real company.
- Real agent hire/create.
- Real board approval where required.
- Real OpenAI Responses run.
- DB attribution and lifecycle assertion.

## Required final commands

Backend:

```sh
cd backend-rs
cargo test
cargo build --bins
```

Frontend:

```sh
cd frontend-nuxt
npx vitest run
```

Acceptance:

```sh
python3 docs/plans/2026-07-05-full-paperclip-parity/acceptance/run_original_gap_diff.py
python3 docs/plans/2026-07-05-full-paperclip-parity/acceptance/run_real_user_real_agent.py
```

The final gap diff must print exactly: `gaps_missing=0 gaps_partial=0 gaps_divergent=0`.


## Row-level anti-false-parity gate

Category checks are not enough. Every non-full audit row from the corrected baseline (`3 full / 50 partial / 100 missing / 22 divergent`) must carry:

```text
phase=<owner> acceptance_id=<id> failing_test=<path::name> passing_test=<command> evidence_ref=<source+rewrite> final_status=<full|waived>
```

A row may move to `full` only after the failing test has been observed red, the implementation makes it green, source evidence is cited, and the reviewer updates `evidence/rewrite-gap-matrix.md`. A row may move to `waived` only after `doc/SPEC-implementation.md` or another canonical product contract is updated to say the original behavior is no longer required.

hook: PostToolUse
hook: PostToolUse
hook: PostToolUse Completed
hook: PostToolUse Completed
codex
The top-level files now identify `rewrite-gap-matrix.jsonl` as authoritative, require exact `gaps_missing=0 gaps_partial=0 gaps_divergent=0`, and list 14 phase files. I’m checking the actual Phase 00 ownership, harness scripts, and row assignments next.
hook: PreToolUse
hook: PreToolUse Completed
hook: PreToolUse
hook: PreToolUse
hook: PreToolUse Completed
hook: PreToolUse Completed
exec
/bin/bash -lc "sed -n '1,280p' docs/plans/2026-07-05-full-paperclip-parity/phases/00-parity-oracle-and-matrix.md" in /home/frankji/Projects/paperclip
 succeeded in 0ms:
# Phase 00 — Parity Oracle and Matrix

## Goal

Freeze the source-backed parity oracle before implementation so workers cannot declare success from a green vertical slice.

## Required artifacts (already scaffolded; Phase 00 must finish them)

- `evidence/rewrite-gap-matrix.md` + `evidence/rewrite-gap-matrix.jsonl`: row-level matrix with original evidence, current rewrite evidence, status, owning phase, red test, green command, acceptance ID, waiver field. Today it carries 220 rows across all 14 phases.
- `evidence/migration-ledger.md`: tracks every Supabase migration number and purpose.
- `acceptance/run_original_gap_diff.py`: the ledger gate. Implemented now; fails on the current rewrite and prints exactly `gaps_missing=0 gaps_partial=0 gaps_divergent=0` only on success.
- `acceptance/run_real_user_real_agent.py`: RED scaffold. Phase 00 must replace the body with the live Supabase + real OpenAI Responses driver.
- `acceptance/run_cli_mcp_surface.py`: RED scaffold. Phase 00 must implement the CLI/MCP surface parity check.
- `acceptance/run_frontend_user_flow.py`: RED scaffold. Phase 00 must implement the Nuxt board-flow parity check.
- No-secret scanner (see `acceptance/no-secret-scanner-spec.md`): Phase 00 must implement it.

## Tasks

1. Confirm matrix rows cover all 14 phases (done: 220 rows). Refine row evidence as later phases cite exact source lines.
2. Classify rows as `v1-core`, `current-addendum`, `post-v1`, or `not-in-v1`.
3. Implement the four acceptance harnesses above so each is a real driver, not a RED scaffold.
4. Implement the no-secret scanner.
5. Add waiver format requiring product-spec update and reviewer signoff.

## Acceptance

- `run_original_gap_diff.py` runs today, fails with non-zero counts, and on eventual success prints exactly `gaps_missing=0 gaps_partial=0 gaps_divergent=0`.
- Phase 00 is not closed until `run_real_user_real_agent.py`, `run_cli_mcp_surface.py`, `run_frontend_user_flow.py`, and the no-secret scanner are real drivers (the current RED scaffolds intentionally return non-zero).
- Final release requires all blocking rows full or explicitly waived by an updated product contract.

hook: PreToolUse
hook: PreToolUse
hook: PreToolUse Completed
hook: PreToolUse Completed
exec
/bin/bash -lc "sed -n '1,260p' docs/plans/2026-07-05-full-paperclip-parity/acceptance/run_original_gap_diff.py" in /home/frankji/Projects/paperclip
 succeeded in 0ms:
#!/usr/bin/env python3
"""Parity gap ledger gate (Phase 00 parity oracle).

Fails until every non-full row in evidence/rewrite-gap-matrix.jsonl is closed
as final_status=full, or final_status=waived with a waiver_ref.

On failure it prints diagnostic detail. On success it prints EXACTLY the
contract line required by 05-test-strategy.md and acceptance/full-parity-harness-spec.md:

    gaps_missing=0 gaps_partial=0 gaps_divergent=0

No other fields are appended on the success line.
"""
from __future__ import annotations
import json
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
MATRIX = ROOT / "evidence" / "rewrite-gap-matrix.jsonl"

rows = [json.loads(line) for line in MATRIX.read_text().splitlines() if line.strip()]

open_rows = []
for row in rows:
    status = row.get("initial_status")
    final = row.get("final_status")
    if status == "full":
        continue
    if final == "full":
        continue
    if final == "waived" and row.get("waiver_ref"):
        continue
    open_rows.append(row)

counts = {"missing": 0, "partial": 0, "divergent": 0}
for row in open_rows:
    if row.get("initial_status") in counts:
        counts[row["initial_status"]] += 1

if any(counts.values()):
    print(
        "gaps_missing=" + str(counts["missing"])
        + " gaps_partial=" + str(counts["partial"])
        + " gaps_divergent=" + str(counts["divergent"])
        + " rows_total=" + str(len(rows))
        + " rows_open=" + str(len(open_rows))
    )
    print("OPEN_ROWS_SAMPLE:")
    for row in open_rows[:20]:
        print("- " + row["row_id"] + " [" + row["initial_status"] + "] " + row["phase"] + " :: " + row["capability"])
    sys.exit(1)

print("gaps_missing=0 gaps_partial=0 gaps_divergent=0")
sys.exit(0)

hook: PostToolUse
hook: PostToolUse
exec
/bin/bash -lc "sed -n '1,280p' docs/plans/2026-07-05-full-paperclip-parity/acceptance/full-parity-harness-spec.md" in /home/frankji/Projects/paperclip
 succeeded in 0ms:
# Full Parity Harness Spec

## Harness inputs

- Supabase URL and anon key for `supabase-franky` only.
- OpenAI-compatible `OPENAI_BASE_URL` and `OPENAI_API_KEY` from environment.
- `test_users.json` with gitignored real test users.
- Rust backend URL.
- Nuxt frontend URL when frontend smoke runs.

## Required assertions

1. Schema introspection: no app objects in `public`; expected tables/functions/enums in `paperclip`; helpers in `paperclip_private`.
2. Defaults/enums match original: company hire approval default false; canonical statuses/types/statuses.
3. Authz matrix: board/session/agent/viewer/low-trust/cross-company behavior.
4. Hire/create: direct create when approval default false; approval flow when enabled.
5. Approval: revision/request/approve/reject/cancel for all types.
6. Runtime: wakeup/run/cancel/logs/status transitions.
7. Budget: cost event, hard-stop auto-pause, budget override approval.
8. Secrets/config: redaction and Codex isolation.
9. Work: task assignment, checkout, comment/work product.
10. Interfaces: HTTP, CLI, MCP, Nuxt.
11. Real OpenAI Responses run: result persisted under run/job/work product and attributed to agent.

## Final pass condition

```text
schema=pass authz=pass lifecycle=pass approvals=pass runtime=pass budgets=pass secrets=pass tasks=pass interfaces=pass live_openai=pass gaps_missing=0 gaps_partial=0 gaps_divergent=0
```


## Row-level matrix proof

The harness must read `evidence/rewrite-gap-matrix.md` (or its generated machine-readable companion) and assert every non-full audit row has:

- `phase`
- `acceptance_id`
- `failing_test`
- `passing_test`
- `evidence_ref`
- `final_status`

Allowed final statuses are only `full` or `waived`. Waived rows require a canonical product-contract reference and reviewer signoff. Category-level checks cannot override a row-level non-full status.

## Exact final output

The final line must include the exact fields:

```text
gaps_missing=0 gaps_partial=0 gaps_divergent=0
```

Any spelling variant, omitted prefix, or category-only summary fails the harness.

hook: PostToolUse Completed
hook: PostToolUse Completed
hook: PostToolUse
hook: PostToolUse Completed
exec
/bin/bash -lc "sed -n '1,260p' docs/plans/2026-07-05-full-paperclip-parity/acceptance/run_real_user_real_agent.py" in /home/frankji/Projects/paperclip
 succeeded in 0ms:
#!/usr/bin/env python3
"""Live real-user / real-agent acceptance harness (Phase 00 deliverable).

This is a deliberately failing RED scaffold. It is owned by Phase 00 of the
full-parity plan: Phase 00 cannot close until this script is implemented to
prove, against the supabase-franky project and a real OpenAI Responses run:

  - real authenticated users from test_users.json (gitignored) get opaque pcs_ sessions;
  - a real company is formed with require_board_approval_for_new_agents=false;
  - a real agent is created/hired, approved where required, and reaches the
    canonical idle runtime state (not the rewrite's divergent 'active');
  - the hired agent runs a REAL OpenAI Responses job attributed to
    subject_type='agent' and the correct agent_id;
  - cross-company / low-trust / agent-cannot-decide denials hold;
  - no Supabase URL/anon key/service role/JWT/OpenAI key reaches any client surface.

While unimplemented it exits non-zero so the release gate cannot pass. The
Phase 00 worker replaces this body with the live driver and the matrix row
phase-00-parity-oracle-run-real-user-real-agent-py-live is closed to final_status=full.
"""
import sys


def main() -> int:
    # RED state: Phase 00 must implement the live driver described above.
    return 2


if __name__ == "__main__":
    sys.exit(main())

hook: PostToolUse
hook: PostToolUse Completed
exec
/bin/bash -lc "sed -n '1,260p' docs/plans/2026-07-05-full-paperclip-parity/acceptance/run_cli_mcp_surface.py" in /home/frankji/Projects/paperclip
 succeeded in 0ms:
#!/usr/bin/env python3
"""CLI / MCP surface parity harness (Phase 00 deliverable).

RED scaffold owned by Phase 00. Phase 00 cannot close until this proves the
rewrite CLI command inventory and MCP tools-list match the original Paperclip
contract for agent/operator surfaces, and that agent keys cannot perform
board-only actions. Exits non-zero while unimplemented.
"""
import sys


def main() -> int:
    return 2


if __name__ == "__main__":
    sys.exit(main())

hook: PostToolUse
hook: PostToolUse Completed
hook: PreToolUse
hook: PreToolUse
hook: PreToolUse Completed
hook: PreToolUse Completed
hook: PreToolUse
hook: PreToolUse
hook: PreToolUse Completed
hook: PreToolUse Failed
hook: PreToolUse
exec
/bin/bash -lc "sed -n '1,240p' docs/plans/2026-07-05-full-paperclip-parity/acceptance/run_frontend_user_flow.py" in /home/frankji/Projects/paperclip
 succeeded in 0ms:
#!/usr/bin/env python3
"""Frontend board-flow parity harness (Phase 00 deliverable).

RED scaffold owned by Phase 00. Phase 00 cannot close until this proves a
non-agent board user can run the full V1 workflow in the Nuxt UI without any
Supabase/OpenAI detail reaching the browser. Exits non-zero while unimplemented.
"""
import sys


def main() -> int:
    return 2


if __name__ == "__main__":
    sys.exit(main())

hook: PostToolUse
hook: PreToolUse Completed
hook: PostToolUse Completed
exec
/bin/bash -lc "sed -n '1,300p' docs/plans/2026-07-05-full-paperclip-parity/codex/phase-prompts.md" in /home/frankji/Projects/paperclip
 succeeded in 0ms:
# Codex Prompts

## Plan review prompt

```text
You are reviewing docs/plans/2026-07-05-full-paperclip-parity for completeness against original Paperclip V1. Read doc/SPEC-implementation.md, packages/shared/src/constants.ts, server/src/routes/agents.ts, server/src/routes/approvals.ts, server/src/routes/access.ts, server/src/services/approvals.ts, server/src/services/heartbeat.ts, ui/src/pages/NewAgent.tsx, cli/src/commands/client/agent.ts, and the plan directory. Identify any missing or under-specified parity area. Do not implement. Return must-fix plan gaps with file references.
```

## Phase implementer prompt template

```text
Before implementing, read PHASE_FILE plus `01-parity-audit-summary.md`, `04-execution-order.md`, `05-test-strategy.md`, `acceptance/full-parity-harness-spec.md`, `evidence/rewrite-gap-matrix.md`, and `reviews/review-checklist.md`. Implement only the rows owned by PHASE_FILE using strict TDD. First write failing tests that demonstrate the current rewrite does not match original Paperclip behavior. Then implement the minimal code/migrations to pass. Preserve: no public schema, no service-role key, no Supabase/OpenAI details client-side, OpenAI Responses API only. Update each owned matrix row with phase, acceptance_id, failing_test, passing_test, evidence_ref, and final_status. Run the phase checks and report exact output.
```

## Final parity review prompt

```text
Adversarially compare the final rewrite against original Paperclip V1 for agent hiring/governance/runtime/interfaces. Use docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.md as the checklist. Confirm gaps_missing=0 gaps_partial=0 gaps_divergent=0 or list blockers. Do not accept a vertical slice as full parity.
```


## Anti-false-parity rule for Codex workers

A phase worker may not report "done" because a category test passed. Done means every owned row in `evidence/rewrite-gap-matrix.md` is `full` or has an approved `waived` product-contract reference. The worker must include the row IDs it closed and the commands that turned each red test green.

hook: PostToolUse
exec
/bin/bash -lc "sed -n '1,280p' docs/plans/2026-07-05-full-paperclip-parity/reviews/review-checklist.md" in /home/frankji/Projects/paperclip
 succeeded in 0ms:
# Review Checklist

Before any phase closes:

- Source evidence cited.
- Matrix rows updated.
- Every non-full baseline row has all closure fields: phase, acceptance_id, failing_test, passing_test, evidence_ref, final_status.
- Red test shown failing before implementation.
- Targeted green command shown passing.
- No unresolved owned gaps.
- No Supabase service-role dependency.
- No Paperclip application objects in `public` schema.
- No company-boundary leak.
- No raw secret exposure.
- Agent status labels are canonical.
- Migration number is unique and recorded in `evidence/migration-ledger.md`.
- Docs/contracts updated when behavior/API/commands changed.
- UI/CLI/MCP rows updated if affected.
- Acceptance IDs linked.
- Category-level acceptance cannot close a row-level gap; row-level matrix must be green or waived.
- Waiver present for any deliberate divergence, with product spec update.

Final review requires:

```text
gaps_missing=0 gaps_partial=0 gaps_divergent=0
```

hook: PostToolUse Completed
hook: PostToolUse
exec
/bin/bash -lc "sed -n '1,260p' docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.md" in /home/frankji/Projects/paperclip
 succeeded in 0ms:
# Rewrite Gap Matrix

Row-level closure ledger. Workers close rows, not categories. Generated from the adversarial parity audit journals plus plan-design coverage rows for every owned phase.

## Corrected audit tally

- full: 3
- partial: 50
- missing: 145
- divergent: 22

Machine-readable companion: [rewrite-gap-matrix.jsonl](rewrite-gap-matrix.jsonl).

## Phase ownership summary

| Phase | Full | Partial | Missing | Divergent | Total |
|---|---:|---:|---:|---:|---:|
| `phases/00-parity-oracle-and-matrix.md` | 0 | 0 | 4 | 0 | 4 |
| `phases/01-schema-authz-parity.md` | 1 | 20 | 24 | 10 | 55 |
| `phases/02-goals-projects-issues-work-products.md` | 0 | 0 | 10 | 0 | 10 |
| `phases/03-workspaces-routines-plugins-sandbox.md` | 0 | 0 | 5 | 0 | 5 |
| `phases/04-deployment-auth-onboarding-ops.md` | 0 | 0 | 6 | 0 | 6 |
| `phases/05-adapters-secrets-skills-config.md` | 0 | 2 | 9 | 1 | 12 |
| `phases/06-approvals-budgets-costs-activity.md` | 1 | 8 | 10 | 2 | 21 |
| `phases/07-agent-runtime-lifecycle.md` | 0 | 3 | 8 | 5 | 16 |
| `phases/08-interfaces-rest-cli-mcp.md` | 0 | 12 | 16 | 3 | 31 |
| `phases/09-nuxt-board-ui-parity.md` | 1 | 5 | 33 | 1 | 40 |
| `phases/10-import-export-catalogs-portability.md` | 0 | 0 | 5 | 0 | 5 |
| `phases/11-observability-feedback-pipelines.md` | 0 | 0 | 5 | 0 | 5 |
| `phases/12-shared-contract-docs-parity.md` | 0 | 0 | 5 | 0 | 5 |
| `phases/13-migration-compatibility-cutover.md` | 0 | 0 | 5 | 0 | 5 |

## Row-level matrix

| Row | Status | Phase | Acceptance | Test | Capability |
|---|---|---|---|---|---|
| `parity-oracle-no-secret-scanner-implementation-across-all-surfaces` | `missing` | `phases/00-parity-oracle-and-matrix.md` | `ACC-00-PARITY-ORACLE-AND-MATRIX-0001` | `acceptance/run_original_gap_diff.py::parity_oracle_no_secret_scanner_implementation_across_all_surfaces` | No-secret scanner implementation across all surfaces |
| `parity-oracle-run-cli-mcp-surface-py-and-run-frontend-user-flow-py-harnesses` | `missing` | `phases/00-parity-oracle-and-matrix.md` | `ACC-00-PARITY-ORACLE-AND-MATRIX-0002` | `acceptance/run_original_gap_diff.py::parity_oracle_run_cli_mcp_surface_py_and_run_frontend_user_flow_py_harnesses` | run_cli_mcp_surface.py and run_frontend_user_flow.py harnesses |
| `parity-oracle-run-original-gap-diff-py-exact-gaps-output-contract` | `missing` | `phases/00-parity-oracle-and-matrix.md` | `ACC-00-PARITY-ORACLE-AND-MATRIX-0003` | `acceptance/run_original_gap_diff.py::parity_oracle_run_original_gap_diff_py_exact_gaps_output_contract` | run_original_gap_diff.py exact gaps_* output contract |
| `parity-oracle-run-real-user-real-agent-py-live-harness-implementation` | `missing` | `phases/00-parity-oracle-and-matrix.md` | `ACC-00-PARITY-ORACLE-AND-MATRIX-0004` | `acceptance/run_original_gap_diff.py::parity_oracle_run_real_user_real_agent_py_live_harness_implementation` | run_real_user_real_agent.py live harness implementation |
| `data-model-schema-agent-adapter-types-enum` | `divergent` | `phases/01-schema-authz-parity.md` | `ACC-01-SCHEMA-AUTHZ-PARITY-0005` | `backend-rs/tests/authz_parity.rs::data_model_schema_agent_adapter_types_enum` | AGENT_ADAPTER_TYPES enum |
| `data-model-schema-agent-budget-fields` | `missing` | `phases/01-schema-authz-parity.md` | `ACC-01-SCHEMA-AUTHZ-PARITY-0006` | `backend-rs/tests/authz_parity.rs::data_model_schema_agent_budget_fields` | per-agent monthly budget accounting |
| `data-model-schema-agent-config-revisions-table` | `missing` | `phases/01-schema-authz-parity.md` | `ACC-01-SCHEMA-AUTHZ-PARITY-0007` | `backend-rs/tests/authz_parity.rs::data_model_schema_agent_config_revisions_table` | agent_config_revisions audit table |
| `data-model-schema-agent-icon-names` | `missing` | `phases/01-schema-authz-parity.md` | `ACC-01-SCHEMA-AUTHZ-PARITY-0008` | `backend-rs/tests/authz_parity.rs::data_model_schema_agent_icon_names` | AGENT_ICON_NAMES enum |
| `data-model-schema-agent-roles-enum` | `divergent` | `phases/01-schema-authz-parity.md` | `ACC-01-SCHEMA-AUTHZ-PARITY-0009` | `backend-rs/tests/authz_parity.rs::data_model_schema_agent_roles_enum` | AGENT_ROLES enum + labels |
| `data-model-schema-agent-statuses-enum` | `divergent` | `phases/01-schema-authz-parity.md` | `ACC-01-SCHEMA-AUTHZ-PARITY-0010` | `backend-rs/tests/authz_parity.rs::data_model_schema_agent_statuses_enum` | AGENT_STATUSES enum values |
| `data-model-schema-agents-create-permission-key` | `partial` | `phases/01-schema-authz-parity.md` | `ACC-01-SCHEMA-AUTHZ-PARITY-0011` | `backend-rs/tests/authz_parity.rs::data_model_schema_agents_create_permission_key` | agents:create permission key |
| `data-model-schema-agents-fk-indexes` | `divergent` | `phases/01-schema-authz-parity.md` | `ACC-01-SCHEMA-AUTHZ-PARITY-0012` | `backend-rs/tests/authz_parity.rs::data_model_schema_agents_fk_indexes` | agents FK constraints and indexes |
| `data-model-schema-agents-orm-extra-columns` | `missing` | `phases/01-schema-authz-parity.md` | `ACC-01-SCHEMA-AUTHZ-PARITY-0013` | `backend-rs/tests/authz_parity.rs::data_model_schema_agents_orm_extra_columns` | agents ORM adds icon/runtime_config/env/pause/error columns |
| `data-model-schema-agents-permissions-column` | `partial` | `phases/01-schema-authz-parity.md` | `ACC-01-SCHEMA-AUTHZ-PARITY-0014` | `backend-rs/tests/authz_parity.rs::data_model_schema_agents_permissions_column` | agents.permissions jsonb column |
| `data-model-schema-agents-table-columns` | `partial` | `phases/01-schema-authz-parity.md` | `ACC-01-SCHEMA-AUTHZ-PARITY-0015` | `backend-rs/tests/authz_parity.rs::data_model_schema_agents_table_columns` | agents table column set |
| `data-model-schema-approval-comments-table` | `missing` | `phases/01-schema-authz-parity.md` | `ACC-01-SCHEMA-AUTHZ-PARITY-0016` | `backend-rs/tests/authz_parity.rs::data_model_schema_approval_comments_table` | approval_comments table |
| `data-model-schema-approval-statuses-enum` | `partial` | `phases/01-schema-authz-parity.md` | `ACC-01-SCHEMA-AUTHZ-PARITY-0017` | `backend-rs/tests/authz_parity.rs::data_model_schema_approval_statuses_enum` | APPROVAL_STATUSES enum |
| `data-model-schema-approval-types-enum` | `partial` | `phases/01-schema-authz-parity.md` | `ACC-01-SCHEMA-AUTHZ-PARITY-0018` | `backend-rs/tests/authz_parity.rs::data_model_schema_approval_types_enum` | APPROVAL_TYPES enum |
| `data-model-schema-approval-validators` | `partial` | `phases/01-schema-authz-parity.md` | `ACC-01-SCHEMA-AUTHZ-PARITY-0019` | `backend-rs/tests/authz_parity.rs::data_model_schema_approval_validators` | approval lifecycle validators |
| `data-model-schema-approvals-index` | `partial` | `phases/01-schema-authz-parity.md` | `ACC-01-SCHEMA-AUTHZ-PARITY-0020` | `backend-rs/tests/authz_parity.rs::data_model_schema_approvals_index` | approvals index |
| `data-model-schema-approvals-table-columns` | `partial` | `phases/01-schema-authz-parity.md` | `ACC-01-SCHEMA-AUTHZ-PARITY-0021` | `backend-rs/tests/authz_parity.rs::data_model_schema_approvals_table_columns` | approvals table column set |
| `data-model-schema-company-board-approval-toggle` | `divergent` | `phases/01-schema-authz-parity.md` | `ACC-01-SCHEMA-AUTHZ-PARITY-0022` | `backend-rs/tests/authz_parity.rs::data_model_schema_company_board_approval_toggle` | companies.require_board_approval_for_new_agents toggle |
| `data-model-schema-company-toggle-validators` | `missing` | `phases/01-schema-authz-parity.md` | `ACC-01-SCHEMA-AUTHZ-PARITY-0023` | `backend-rs/tests/authz_parity.rs::data_model_schema_company_toggle_validators` | board-approval toggle in company validators |
| `data-model-schema-create-agent-validator` | `partial` | `phases/01-schema-authz-parity.md` | `ACC-01-SCHEMA-AUTHZ-PARITY-0024` | `backend-rs/tests/authz_parity.rs::data_model_schema_create_agent_validator` | createAgentSchema fields/defaults |
| `data-model-schema-hire-modeled-via-approval` | `partial` | `phases/01-schema-authz-parity.md` | `ACC-01-SCHEMA-AUTHZ-PARITY-0025` | `backend-rs/tests/authz_parity.rs::data_model_schema_hire_modeled_via_approval` | Hiring modeled through approvals, not a hires table |
| `data-model-schema-issue-approvals-link-table` | `missing` | `phases/01-schema-authz-parity.md` | `ACC-01-SCHEMA-AUTHZ-PARITY-0026` | `backend-rs/tests/authz_parity.rs::data_model_schema_issue_approvals_link_table` | issue_approvals junction table |
| `data-model-schema-pause-reasons-enum` | `missing` | `phases/01-schema-authz-parity.md` | `ACC-01-SCHEMA-AUTHZ-PARITY-0027` | `backend-rs/tests/authz_parity.rs::data_model_schema_pause_reasons_enum` | PAUSE_REASONS enum |
| `data-model-schema-update-agent-permissions-validator` | `missing` | `phases/01-schema-authz-parity.md` | `ACC-01-SCHEMA-AUTHZ-PARITY-0028` | `backend-rs/tests/authz_parity.rs::data_model_schema_update_agent_permissions_validator` | updateAgentPermissionsSchema |
| `data-model-schema-update-agent-validator` | `missing` | `phases/01-schema-authz-parity.md` | `ACC-01-SCHEMA-AUTHZ-PARITY-0029` | `backend-rs/tests/authz_parity.rs::data_model_schema_update_agent_validator` | updateAgentSchema restrictions |
| `hiring-flow-side-effects-approval-gate-status` | `divergent` | `phases/01-schema-authz-parity.md` | `ACC-01-SCHEMA-AUTHZ-PARITY-0030` | `backend-rs/tests/authz_parity.rs::hiring_flow_side_effects_approval_gate_status` | Approval-required vs not: agent status branch |
| `hiring-flow-side-effects-default-task-assign-grant` | `missing` | `phases/01-schema-authz-parity.md` | `ACC-01-SCHEMA-AUTHZ-PARITY-0031` | `backend-rs/tests/authz_parity.rs::hiring_flow_side_effects_default_task_assign_grant` | Default tasks:assign grant on every hire |
| `hiring-flow-side-effects-direct-create-blocked-when-approval` | `missing` | `phases/01-schema-authz-parity.md` | `ACC-01-SCHEMA-AUTHZ-PARITY-0032` | `backend-rs/tests/authz_parity.rs::hiring_flow_side_effects_direct_create_blocked_when_approval` | Direct POST /agents blocked when approvals required |
| `hiring-flow-side-effects-hire-activity-log` | `missing` | `phases/01-schema-authz-parity.md` | `ACC-01-SCHEMA-AUTHZ-PARITY-0033` | `backend-rs/tests/authz_parity.rs::hiring_flow_side_effects_hire_activity_log` | agent.hire_created + approval.created activity logs + telemetry |
| `hiring-flow-side-effects-hire-approve-activation` | `partial` | `phases/01-schema-authz-parity.md` | `ACC-01-SCHEMA-AUTHZ-PARITY-0034` | `backend-rs/tests/authz_parity.rs::hiring_flow_side_effects_hire_approve_activation` | Approving a hire_agent approval activates agent + budget + hook |
| `hiring-flow-side-effects-hire-endpoint-authz` | `partial` | `phases/01-schema-authz-parity.md` | `ACC-01-SCHEMA-AUTHZ-PARITY-0035` | `backend-rs/tests/authz_parity.rs::hiring_flow_side_effects_hire_endpoint_authz` | POST /companies/:companyId/agent-hires with agents:create authz |
| `hiring-flow-side-effects-hire-input-schema` | `partial` | `phases/01-schema-authz-parity.md` | `ACC-01-SCHEMA-AUTHZ-PARITY-0036` | `backend-rs/tests/authz_parity.rs::hiring_flow_side_effects_hire_input_schema` | createAgentHireSchema hire payload fields |
| `hiring-flow-side-effects-hire-reject-terminate` | `partial` | `phases/01-schema-authz-parity.md` | `ACC-01-SCHEMA-AUTHZ-PARITY-0037` | `backend-rs/tests/authz_parity.rs::hiring_flow_side_effects_hire_reject_terminate` | Rejecting a hire_agent approval terminates the pending agent |
| `hiring-flow-side-effects-join-request-hire-hook` | `missing` | `phases/01-schema-authz-parity.md` | `ACC-01-SCHEMA-AUTHZ-PARITY-0038` | `backend-rs/tests/authz_parity.rs::hiring_flow_side_effects_join_request_hire_hook` | Join-request approval also triggers hire hook |
| `hiring-flow-side-effects-legacy-prompt-guard` | `missing` | `phases/01-schema-authz-parity.md` | `ACC-01-SCHEMA-AUTHZ-PARITY-0039` | `backend-rs/tests/authz_parity.rs::hiring_flow_side_effects_legacy_prompt_guard` | Reject legacy promptTemplate on new hires |
| `hiring-flow-side-effects-plugin-managed-agent-hire` | `missing` | `phases/01-schema-authz-parity.md` | `ACC-01-SCHEMA-AUTHZ-PARITY-0040` | `backend-rs/tests/authz_parity.rs::hiring_flow_side_effects_plugin_managed_agent_hire` | Plugin-managed agent creation mirrors the hire+approval flow |
| `hiring-flow-side-effects-source-issue-linking` | `missing` | `phases/01-schema-authz-parity.md` | `ACC-01-SCHEMA-AUTHZ-PARITY-0041` | `backend-rs/tests/authz_parity.rs::hiring_flow_side_effects_source_issue_linking` | Source issues linked to approval |
| `permissions-board-membership-access-activity-audit-side-effects-for-pe` | `missing` | `phases/01-schema-authz-parity.md` | `ACC-01-SCHEMA-AUTHZ-PARITY-0042` | `backend-rs/tests/authz_parity.rs::permissions_board_membership_access_activity_audit_side_effects_for_pe` | Activity/audit side effects for permission and hiring actions |
| `permissions-board-membership-access-agent-api-key-scopes-and-key-subje` | `partial` | `phases/01-schema-authz-parity.md` | `ACC-01-SCHEMA-AUTHZ-PARITY-0043` | `backend-rs/tests/authz_parity.rs::permissions_board_membership_access_agent_api_key_scopes_and_key_subje` | Agent API key scopes and key subject constraints |
| `permissions-board-membership-access-agent-cancreateagents-hiring-autho` | `partial` | `phases/01-schema-authz-parity.md` | `ACC-01-SCHEMA-AUTHZ-PARITY-0044` | `backend-rs/tests/authz_parity.rs::permissions_board_membership_access_agent_cancreateagents_hiring_autho` | Agent `canCreateAgents` / hiring authority |
| `permissions-board-membership-access-agent-configuration-read-update-au` | `missing` | `phases/01-schema-authz-parity.md` | `ACC-01-SCHEMA-AUTHZ-PARITY-0045` | `backend-rs/tests/authz_parity.rs::permissions_board_membership_access_agent_configuration_read_update_au` | Agent configuration read/update authorization and redaction |
| `permissions-board-membership-access-approval-decision-authority-for-hi` | `partial` | `phases/01-schema-authz-parity.md` | `ACC-01-SCHEMA-AUTHZ-PARITY-0046` | `backend-rs/tests/authz_parity.rs::permissions_board_membership_access_approval_decision_authority_for_hi` | Approval decision authority for hires |
| `permissions-board-membership-access-board-identification-and-company-a` | `partial` | `phases/01-schema-authz-parity.md` | `ACC-01-SCHEMA-AUTHZ-PARITY-0047` | `backend-rs/tests/authz_parity.rs::permissions_board_membership_access_board_identification_and_company_a` | Board identification and company access guard |
| `permissions-board-membership-access-company-job-access-under-api-key-v` | `divergent` | `phases/01-schema-authz-parity.md` | `ACC-01-SCHEMA-AUTHZ-PARITY-0048` | `backend-rs/tests/authz_parity.rs::permissions_board_membership_access_company_job_access_under_api_key_v` | Company/job access under API key vs session |
| `permissions-board-membership-access-company-member-management-and-cust` | `missing` | `phases/01-schema-authz-parity.md` | `ACC-01-SCHEMA-AUTHZ-PARITY-0049` | `backend-rs/tests/authz_parity.rs::permissions_board_membership_access_company_member_management_and_cust` | Company member management and custom permission grants |
| `permissions-board-membership-access-frontend-user-visible-board-staffi` | `partial` | `phases/01-schema-authz-parity.md` | `ACC-01-SCHEMA-AUTHZ-PARITY-0050` | `backend-rs/tests/authz_parity.rs::permissions_board_membership_access_frontend_user_visible_board_staffi` | Frontend/user-visible board staffing controls |
| `permissions-board-membership-access-human-board-roles-and-default-gran` | `divergent` | `phases/01-schema-authz-parity.md` | `ACC-01-SCHEMA-AUTHZ-PARITY-0051` | `backend-rs/tests/authz_parity.rs::permissions_board_membership_access_human_board_roles_and_default_gran` | Human board roles and default grants |
| `permissions-board-membership-access-invite-join-request-and-approval-p` | `partial` | `phases/01-schema-authz-parity.md` | `ACC-01-SCHEMA-AUTHZ-PARITY-0052` | `backend-rs/tests/authz_parity.rs::permissions_board_membership_access_invite_join_request_and_approval_p` | Invite, join request, and approval permissions |
| `permissions-board-membership-access-original-permission-key-inventory` | `missing` | `phases/01-schema-authz-parity.md` | `ACC-01-SCHEMA-AUTHZ-PARITY-0053` | `backend-rs/tests/authz_parity.rs::permissions_board_membership_access_original_permission_key_inventory` | Original permission key inventory |
| `permissions-board-membership-access-permission-update-endpoint-for-age` | `missing` | `phases/01-schema-authz-parity.md` | `ACC-01-SCHEMA-AUTHZ-PARITY-0054` | `backend-rs/tests/authz_parity.rs::permissions_board_membership_access_permission_update_endpoint_for_age` | Permission update endpoint for agents (`canCreateAgents`, `canCreateSkills`, `canAssignTasks`) |
| `permissions-board-membership-access-protected-private-agent-authorizat` | `missing` | `phases/01-schema-authz-parity.md` | `ACC-01-SCHEMA-AUTHZ-PARITY-0055` | `backend-rs/tests/authz_parity.rs::permissions_board_membership_access_protected_private_agent_authorizat` | Protected/private agent authorization policies and low-trust boundaries |
| `permissions-board-membership-access-reports-to-same-company-hierarchy-` | `full` | `phases/01-schema-authz-parity.md` | `ACC-01-SCHEMA-AUTHZ-PARITY-0056` | `backend-rs/tests/authz_parity.rs::permissions_board_membership_access_reports_to_same_company_hierarchy_` | Reports-to / same-company hierarchy check for hires |
| `permissions-board-membership-access-require-board-approval-for-new-age` | `divergent` | `phases/01-schema-authz-parity.md` | `ACC-01-SCHEMA-AUTHZ-PARITY-0057` | `backend-rs/tests/authz_parity.rs::permissions_board_membership_access_require_board_approval_for_new_age` | `require_board_approval_for_new_agents` default and direct-hire semantics |
| `permissions-board-membership-access-status-model-for-agent-membership-` | `divergent` | `phases/01-schema-authz-parity.md` | `ACC-01-SCHEMA-AUTHZ-PARITY-0058` | `backend-rs/tests/authz_parity.rs::permissions_board_membership_access_status_model_for_agent_membership_` | Status model for agent membership/hiring lifecycle |
| `permissions-board-membership-access-task-assignment-permissions-tasks-` | `missing` | `phases/01-schema-authz-parity.md` | `ACC-01-SCHEMA-AUTHZ-PARITY-0059` | `backend-rs/tests/authz_parity.rs::permissions_board_membership_access_task_assignment_permissions_tasks_` | Task assignment permissions (`tasks:assign`, scoped assignment, simple default) |
| `comments-issue-comments-as-the-built-in-communication-model` | `missing` | `phases/02-goals-projects-issues-work-products.md` | `ACC-02-GOALS-PROJECTS-ISSUES-WORK-PRODUCTS-0060` | `backend-rs/tests/tasks_parity.rs::comments_issue_comments_as_the_built_in_communication_model` | Issue comments as the built-in communication model |
| `documents-issue-documents-and-attachments` | `missing` | `phases/02-goals-projects-issues-work-products.md` | `ACC-02-GOALS-PROJECTS-ISSUES-WORK-PRODUCTS-0061` | `backend-rs/tests/tasks_parity.rs::documents_issue_documents_and_attachments` | Issue documents and attachments |
| `goals-hierarchy-company-mission-and-goal-hierarchy-company-team-agent-task` | `missing` | `phases/02-goals-projects-issues-work-products.md` | `ACC-02-GOALS-PROJECTS-ISSUES-WORK-PRODUCTS-0062` | `backend-rs/tests/tasks_parity.rs::goals_hierarchy_company_mission_and_goal_hierarchy_company_team_agent_task` | Company mission and goal hierarchy company/team/agent/task |
| `goals-hierarchy-goal-crud-parent-linkage-owner-agent-assignment` | `missing` | `phases/02-goals-projects-issues-work-products.md` | `ACC-02-GOALS-PROJECTS-ISSUES-WORK-PRODUCTS-0063` | `backend-rs/tests/tasks_parity.rs::goals_hierarchy_goal_crud_parent_linkage_owner_agent_assignment` | Goal CRUD, parent linkage, owner agent assignment |
| `issues-blockers-recovery-issues-review-approval-handoffs` | `missing` | `phases/02-goals-projects-issues-work-products.md` | `ACC-02-GOALS-PROJECTS-ISSUES-WORK-PRODUCTS-0064` | `backend-rs/tests/tasks_parity.rs::issues_blockers_recovery_issues_review_approval_handoffs` | Blockers, recovery issues, review/approval handoffs |
| `issues-issue-stable-identifiers-and-parent-sub-issue-tree` | `missing` | `phases/02-goals-projects-issues-work-products.md` | `ACC-02-GOALS-PROJECTS-ISSUES-WORK-PRODUCTS-0065` | `backend-rs/tests/tasks_parity.rs::issues_issue_stable_identifiers_and_parent_sub_issue_tree` | Issue stable identifiers and parent/sub-issue tree |
| `issues-single-assignee-and-atomic-checkout-for-in-progress` | `missing` | `phases/02-goals-projects-issues-work-products.md` | `ACC-02-GOALS-PROJECTS-ISSUES-WORK-PRODUCTS-0066` | `backend-rs/tests/tasks_parity.rs::issues_single_assignee_and_atomic_checkout_for_in_progress` | Single-assignee and atomic checkout for in_progress |
| `projects-project-crud-lead-target-date-goal-linkage` | `missing` | `phases/02-goals-projects-issues-work-products.md` | `ACC-02-GOALS-PROJECTS-ISSUES-WORK-PRODUCTS-0067` | `backend-rs/tests/tasks_parity.rs::projects_project_crud_lead_target_date_goal_linkage` | Project CRUD, lead, target date, goal linkage |
| `projects-project-env-overlay-merging-into-run-environment` | `missing` | `phases/02-goals-projects-issues-work-products.md` | `ACC-02-GOALS-PROJECTS-ISSUES-WORK-PRODUCTS-0068` | `backend-rs/tests/tasks_parity.rs::projects_project_env_overlay_merging_into_run_environment` | Project env overlay merging into run environment |
| `work-products-work-product-artifact-metadata-and-inspectable-outputs` | `missing` | `phases/02-goals-projects-issues-work-products.md` | `ACC-02-GOALS-PROJECTS-ISSUES-WORK-PRODUCTS-0069` | `backend-rs/tests/tasks_parity.rs::work_products_work_product_artifact_metadata_and_inspectable_outputs` | Work product/artifact metadata and inspectable outputs |
| `execution-workspaces-project-execution-workspace-policy-and-provision-command` | `missing` | `phases/03-workspaces-routines-plugins-sandbox.md` | `ACC-03-WORKSPACES-ROUTINES-PLUGINS-SANDBOX-0070` | `backend-rs/tests/runtime_extensions_parity.rs::execution_workspaces_project_execution_workspace_policy_and_provision_command` | Project execution workspace policy and provision command |
| `plugins-plugin-manifest-namespace-trust-boundary` | `missing` | `phases/03-workspaces-routines-plugins-sandbox.md` | `ACC-03-WORKSPACES-ROUTINES-PLUGINS-SANDBOX-0071` | `backend-rs/tests/runtime_extensions_parity.rs::plugins_plugin_manifest_namespace_trust_boundary` | Plugin manifest, namespace, trust boundary |
| `routines-routine-schedule-webhook-api-dispatch-env-overlay-snapshot` | `missing` | `phases/03-workspaces-routines-plugins-sandbox.md` | `ACC-03-WORKSPACES-ROUTINES-PLUGINS-SANDBOX-0072` | `backend-rs/tests/runtime_extensions_parity.rs::routines_routine_schedule_webhook_api_dispatch_env_overlay_snapshot` | Routine schedule, webhook/API dispatch, env overlay snapshot |
| `runtime-services-managed-runtime-preview-dev-service-lifecycle` | `missing` | `phases/03-workspaces-routines-plugins-sandbox.md` | `ACC-03-WORKSPACES-ROUTINES-PLUGINS-SANDBOX-0073` | `backend-rs/tests/runtime_extensions_parity.rs::runtime_services_managed_runtime_preview_dev_service_lifecycle` | Managed runtime/preview/dev service lifecycle |
| `sandbox-sandbox-provider-containment-and-low-trust-boundaries` | `missing` | `phases/03-workspaces-routines-plugins-sandbox.md` | `ACC-03-WORKSPACES-ROUTINES-PLUGINS-SANDBOX-0074` | `backend-rs/tests/runtime_extensions_parity.rs::sandbox_sandbox_provider_containment_and_low_trust_boundaries` | Sandbox provider containment and low-trust boundaries |
| `deployment-modes-authenticated-private-and-public-exposure-policy` | `missing` | `phases/04-deployment-auth-onboarding-ops.md` | `ACC-04-DEPLOYMENT-AUTH-ONBOARDING-OPS-0075` | `backend-rs/tests/deployment_auth_parity.rs::deployment_modes_authenticated_private_and_public_exposure_policy` | authenticated private and public exposure policy |
| `deployment-modes-local-trusted-implicit-board-behavior` | `missing` | `phases/04-deployment-auth-onboarding-ops.md` | `ACC-04-DEPLOYMENT-AUTH-ONBOARDING-OPS-0076` | `backend-rs/tests/deployment_auth_parity.rs::deployment_modes_local_trusted_implicit_board_behavior` | local_trusted implicit board behavior |
| `deployment-modes-public-authenticated-deployment-refuses-embedded-db-fallback` | `missing` | `phases/04-deployment-auth-onboarding-ops.md` | `ACC-04-DEPLOYMENT-AUTH-ONBOARDING-OPS-0077` | `backend-rs/tests/deployment_auth_parity.rs::deployment_modes_public_authenticated_deployment_refuses_embedded_db_fallback` | Public authenticated deployment refuses embedded DB fallback |
| `onboarding-first-admin-bootstrap-and-claim-flow-without-supabase-exposure` | `missing` | `phases/04-deployment-auth-onboarding-ops.md` | `ACC-04-DEPLOYMENT-AUTH-ONBOARDING-OPS-0078` | `backend-rs/tests/deployment_auth_parity.rs::onboarding_first_admin_bootstrap_and_claim_flow_without_supabase_exposure` | First-admin bootstrap and claim flow without Supabase exposure |
| `ops-doctor-onboard-backup-storage-config-admin-surfaces` | `missing` | `phases/04-deployment-auth-onboarding-ops.md` | `ACC-04-DEPLOYMENT-AUTH-ONBOARDING-OPS-0079` | `backend-rs/tests/deployment_auth_parity.rs::ops_doctor_onboard_backup_storage_config_admin_surfaces` | Doctor, onboard, backup, storage, config admin surfaces |
| `ops-instance-settings-and-sidebar-preferences` | `missing` | `phases/04-deployment-auth-onboarding-ops.md` | `ACC-04-DEPLOYMENT-AUTH-ONBOARDING-OPS-0080` | `backend-rs/tests/deployment_auth_parity.rs::ops_instance_settings_and_sidebar_preferences` | Instance settings and sidebar preferences |
| `hiring-flow-side-effects-adapter-defaults-injection` | `missing` | `phases/05-adapters-secrets-skills-config.md` | `ACC-05-ADAPTERS-SECRETS-SKILLS-CONFIG-0081` | `backend-rs/tests/adapter_config_parity.rs::hiring_flow_side_effects_adapter_defaults_injection` | Adapter-type default config injection |
| `hiring-flow-side-effects-adapter-type-validation` | `missing` | `phases/05-adapters-secrets-skills-config.md` | `ACC-05-ADAPTERS-SECRETS-SKILLS-CONFIG-0082` | `backend-rs/tests/adapter_config_parity.rs::hiring_flow_side_effects_adapter_type_validation` | Adapter type must be a registered server adapter |
| `hiring-flow-side-effects-agent-actor-config-guard` | `missing` | `phases/05-adapters-secrets-skills-config.md` | `ACC-05-ADAPTERS-SECRETS-SKILLS-CONFIG-0083` | `backend-rs/tests/adapter_config_parity.rs::hiring_flow_side_effects_agent_actor_config_guard` | Agent-authenticated callers cannot set instructions/workspace-command config |
| `hiring-flow-side-effects-agent-create-side-effects` | `partial` | `phases/05-adapters-secrets-skills-config.md` | `ACC-05-ADAPTERS-SECRETS-SKILLS-CONFIG-0084` | `backend-rs/tests/adapter_config_parity.rs::hiring_flow_side_effects_agent_create_side_effects` | Agent creation side effects (name dedupe, manager validation, secret bindings) |
| `hiring-flow-side-effects-approval-record-creation` | `partial` | `phases/05-adapters-secrets-skills-config.md` | `ACC-05-ADAPTERS-SECRETS-SKILLS-CONFIG-0085` | `backend-rs/tests/adapter_config_parity.rs::hiring_flow_side_effects_approval_record_creation` | hire_agent approval record created with redacted config snapshot |
| `hiring-flow-side-effects-codex-key-isolation` | `missing` | `phases/05-adapters-secrets-skills-config.md` | `ACC-05-ADAPTERS-SECRETS-SKILLS-CONFIG-0086` | `backend-rs/tests/adapter_config_parity.rs::hiring_flow_side_effects_codex_key_isolation` | codex_local per-agent CODEX_HOME isolation |
| `hiring-flow-side-effects-desired-skill-assignment` | `missing` | `phases/05-adapters-secrets-skills-config.md` | `ACC-05-ADAPTERS-SECRETS-SKILLS-CONFIG-0087` | `backend-rs/tests/adapter_config_parity.rs::hiring_flow_side_effects_desired_skill_assignment` | Desired skills resolution + skill sync preference persisted |
| `hiring-flow-side-effects-hire-hook-onhireapproved` | `missing` | `phases/05-adapters-secrets-skills-config.md` | `ACC-05-ADAPTERS-SECRETS-SKILLS-CONFIG-0088` | `backend-rs/tests/adapter_config_parity.rs::hiring_flow_side_effects_hire_hook_onhireapproved` | onHireApproved adapter hook (hire-hook.ts) |
| `hiring-flow-side-effects-instructions-bundle-materialization` | `missing` | `phases/05-adapters-secrets-skills-config.md` | `ACC-05-ADAPTERS-SECRETS-SKILLS-CONFIG-0089` | `backend-rs/tests/adapter_config_parity.rs::hiring_flow_side_effects_instructions_bundle_materialization` | Default instructions bundle materialized on hire |
| `hiring-flow-side-effects-role-status-adapter-enums` | `divergent` | `phases/05-adapters-secrets-skills-config.md` | `ACC-05-ADAPTERS-SECRETS-SKILLS-CONFIG-0090` | `backend-rs/tests/adapter_config_parity.rs::hiring_flow_side_effects_role_status_adapter_enums` | Role / status / adapter-type enum values |
| `hiring-flow-side-effects-runtime-heartbeat-normalization` | `missing` | `phases/05-adapters-secrets-skills-config.md` | `ACC-05-ADAPTERS-SECRETS-SKILLS-CONFIG-0091` | `backend-rs/tests/adapter_config_parity.rs::hiring_flow_side_effects_runtime_heartbeat_normalization` | Runtime config heartbeat defaults |
| `hiring-flow-side-effects-secret-normalization` | `missing` | `phases/05-adapters-secrets-skills-config.md` | `ACC-05-ADAPTERS-SECRETS-SKILLS-CONFIG-0092` | `backend-rs/tests/adapter_config_parity.rs::hiring_flow_side_effects_secret_normalization` | Adapter/runtime config secret normalization + constraints |
| `approvals-system-governance-access-boundary-read` | `partial` | `phases/06-approvals-budgets-costs-activity.md` | `ACC-06-APPROVALS-BUDGETS-COSTS-ACTIVITY-0093` | `backend-rs/tests/approvals_parity.rs::approvals_system_governance_access_boundary_read` | Company-scope read authorization boundary |
| `approvals-system-governance-approval-comments-table` | `missing` | `phases/06-approvals-budgets-costs-activity.md` | `ACC-06-APPROVALS-BUDGETS-COSTS-ACTIVITY-0094` | `backend-rs/tests/approvals_parity.rs::approvals_system_governance_approval_comments_table` | approval_comments table + comment endpoints |
| `approvals-system-governance-approval-status-enum` | `divergent` | `phases/06-approvals-budgets-costs-activity.md` | `ACC-06-APPROVALS-BUDGETS-COSTS-ACTIVITY-0095` | `backend-rs/tests/approvals_parity.rs::approvals_system_governance_approval_status_enum` | Approval status lifecycle enum (5 statuses) |
| `approvals-system-governance-approval-types-enum` | `partial` | `phases/06-approvals-budgets-costs-activity.md` | `ACC-06-APPROVALS-BUDGETS-COSTS-ACTIVITY-0096` | `backend-rs/tests/approvals_parity.rs::approvals_system_governance_approval_types_enum` | Approval type enum (4 governed types) |
| `approvals-system-governance-approvals-table-schema` | `partial` | `phases/06-approvals-budgets-costs-activity.md` | `ACC-06-APPROVALS-BUDGETS-COSTS-ACTIVITY-0097` | `backend-rs/tests/approvals_parity.rs::approvals_system_governance_approvals_table_schema` | approvals table columns |
| `approvals-system-governance-approve-effect-hire-agent` | `partial` | `phases/06-approvals-budgets-costs-activity.md` | `ACC-06-APPROVALS-BUDGETS-COSTS-ACTIVITY-0098` | `backend-rs/tests/approvals_parity.rs::approvals_system_governance_approve_effect_hire_agent` | approve effect: hire_agent activates/creates agent + budget + notify |
| `approvals-system-governance-approve-effect-requester-wakeup` | `missing` | `phases/06-approvals-budgets-costs-activity.md` | `ACC-06-APPROVALS-BUDGETS-COSTS-ACTIVITY-0099` | `backend-rs/tests/approvals_parity.rs::approvals_system_governance_approve_effect_requester_wakeup` | approve effect: wake requesting agent + activity logs |
| `approvals-system-governance-approve-lifecycle` | `divergent` | `phases/06-approvals-budgets-costs-activity.md` | `ACC-06-APPROVALS-BUDGETS-COSTS-ACTIVITY-0100` | `backend-rs/tests/approvals_parity.rs::approvals_system_governance_approve_lifecycle` | Approve transition + idempotency |
| `approvals-system-governance-board-chat-issue-persistence` | `missing` | `phases/06-approvals-budgets-costs-activity.md` | `ACC-06-APPROVALS-BUDGETS-COSTS-ACTIVITY-0101` | `backend-rs/tests/approvals_parity.rs::approvals_system_governance_board_chat_issue_persistence` | Board chat persisted to standing 'Board Operations' issue |
| `approvals-system-governance-board-chat-relay` | `missing` | `phases/06-approvals-budgets-costs-activity.md` | `ACC-06-APPROVALS-BUDGETS-COSTS-ACTIVITY-0102` | `backend-rs/tests/approvals_parity.rs::approvals_system_governance_board_chat_relay` | Board Concierge Chat SSE relay |
| `approvals-system-governance-create-approval` | `partial` | `phases/06-approvals-budgets-costs-activity.md` | `ACC-06-APPROVALS-BUDGETS-COSTS-ACTIVITY-0103` | `backend-rs/tests/approvals_parity.rs::approvals_system_governance_create_approval` | Create approval (pending) |
| `approvals-system-governance-decide-authz-board-only` | `partial` | `phases/06-approvals-budgets-costs-activity.md` | `ACC-06-APPROVALS-BUDGETS-COSTS-ACTIVITY-0104` | `backend-rs/tests/approvals_parity.rs::approvals_system_governance_decide_authz_board_only` | Only board actors may approve/reject/request-revision |
| `approvals-system-governance-find-open-hire-approval` | `missing` | `phases/06-approvals-budgets-costs-activity.md` | `ACC-06-APPROVALS-BUDGETS-COSTS-ACTIVITY-0105` | `backend-rs/tests/approvals_parity.rs::approvals_system_governance_find_open_hire_approval` | findOpenHireApprovalForAgent helper |
| `approvals-system-governance-get-approval` | `missing` | `phases/06-approvals-budgets-costs-activity.md` | `ACC-06-APPROVALS-BUDGETS-COSTS-ACTIVITY-0106` | `backend-rs/tests/approvals_parity.rs::approvals_system_governance_get_approval` | Get single approval / linked issues |
| `approvals-system-governance-list-approvals` | `partial` | `phases/06-approvals-budgets-costs-activity.md` | `ACC-06-APPROVALS-BUDGETS-COSTS-ACTIVITY-0107` | `backend-rs/tests/approvals_parity.rs::approvals_system_governance_list_approvals` | List approvals for a company (status filter) |
| `approvals-system-governance-payload-redaction` | `missing` | `phases/06-approvals-budgets-costs-activity.md` | `ACC-06-APPROVALS-BUDGETS-COSTS-ACTIVITY-0108` | `backend-rs/tests/approvals_parity.rs::approvals_system_governance_payload_redaction` | Approval payload + comment redaction on read |
| `approvals-system-governance-reject-lifecycle-effect` | `partial` | `phases/06-approvals-budgets-costs-activity.md` | `ACC-06-APPROVALS-BUDGETS-COSTS-ACTIVITY-0109` | `backend-rs/tests/approvals_parity.rs::approvals_system_governance_reject_lifecycle_effect` | Reject transition + hire_agent termination |
| `approvals-system-governance-request-revision` | `missing` | `phases/06-approvals-budgets-costs-activity.md` | `ACC-06-APPROVALS-BUDGETS-COSTS-ACTIVITY-0110` | `backend-rs/tests/approvals_parity.rs::approvals_system_governance_request_revision` | Request revision transition (board only, pending→revision_requested) |
| `approvals-system-governance-require-approval-access` | `full` | `phases/06-approvals-budgets-costs-activity.md` | `ACC-06-APPROVALS-BUDGETS-COSTS-ACTIVITY-0111` | `backend-rs/tests/approvals_parity.rs::approvals_system_governance_require_approval_access` | requireApprovalAccess company scoping on decide |
| `approvals-system-governance-resubmit` | `missing` | `phases/06-approvals-budgets-costs-activity.md` | `ACC-06-APPROVALS-BUDGETS-COSTS-ACTIVITY-0112` | `backend-rs/tests/approvals_parity.rs::approvals_system_governance_resubmit` | Resubmit (revision_requested→pending), requester-only |
| `approvals-system-governance-run-context-mutation-guard` | `missing` | `phases/06-approvals-budgets-costs-activity.md` | `ACC-06-APPROVALS-BUDGETS-COSTS-ACTIVITY-0113` | `backend-rs/tests/approvals_parity.rs::approvals_system_governance_run_context_mutation_guard` | Cheap status-only recovery run cannot mutate approvals |
| `agent-lifecycle-status-transitions-agent-key-permission-to-request-hir` | `partial` | `phases/07-agent-runtime-lifecycle.md` | `ACC-07-AGENT-RUNTIME-LIFECYCLE-0114` | `backend-rs/tests/agent_lifecycle_parity.rs::agent_lifecycle_status_transitions_agent_key_permission_to_request_hir` | Agent-key permission to request hires |
| `agent-lifecycle-status-transitions-approval-gated-hiring-flow` | `partial` | `phases/07-agent-runtime-lifecycle.md` | `ACC-07-AGENT-RUNTIME-LIFECYCLE-0115` | `backend-rs/tests/agent_lifecycle_parity.rs::agent_lifecycle_status_transitions_approval_gated_hiring_flow` | Approval-gated hiring flow |
| `agent-lifecycle-status-transitions-approving-pending-hires` | `divergent` | `phases/07-agent-runtime-lifecycle.md` | `ACC-07-AGENT-RUNTIME-LIFECYCLE-0116` | `backend-rs/tests/agent_lifecycle_parity.rs::agent_lifecycle_status_transitions_approving_pending_hires` | Approving pending hires |
| `agent-lifecycle-status-transitions-canonical-agent-status-vocabulary` | `divergent` | `phases/07-agent-runtime-lifecycle.md` | `ACC-07-AGENT-RUNTIME-LIFECYCLE-0117` | `backend-rs/tests/agent_lifecycle_parity.rs::agent_lifecycle_status_transitions_canonical_agent_status_vocabulary` | Canonical agent status vocabulary |
| `agent-lifecycle-status-transitions-company-auth-boundary-for-lifecycle` | `partial` | `phases/07-agent-runtime-lifecycle.md` | `ACC-07-AGENT-RUNTIME-LIFECYCLE-0118` | `backend-rs/tests/agent_lifecycle_parity.rs::agent_lifecycle_status_transitions_company_auth_boundary_for_lifecycle` | Company/auth boundary for lifecycle mutations |
| `agent-lifecycle-status-transitions-error-state-and-clear-error` | `missing` | `phases/07-agent-runtime-lifecycle.md` | `ACC-07-AGENT-RUNTIME-LIFECYCLE-0119` | `backend-rs/tests/agent_lifecycle_parity.rs::agent_lifecycle_status_transitions_error_state_and_clear_error` | Error state and clear-error |
| `agent-lifecycle-status-transitions-frontend-cli-mcp-lifecycle-controls` | `divergent` | `phases/07-agent-runtime-lifecycle.md` | `ACC-07-AGENT-RUNTIME-LIFECYCLE-0120` | `backend-rs/tests/agent_lifecycle_parity.rs::agent_lifecycle_status_transitions_frontend_cli_mcp_lifecycle_controls` | Frontend/CLI/MCP lifecycle controls |
| `agent-lifecycle-status-transitions-heartbeat-run-wakeup-request-log-ev` | `missing` | `phases/07-agent-runtime-lifecycle.md` | `ACC-07-AGENT-RUNTIME-LIFECYCLE-0121` | `backend-rs/tests/agent_lifecycle_parity.rs::agent_lifecycle_status_transitions_heartbeat_run_wakeup_request_log_ev` | Heartbeat run, wakeup request, log/event, live-run, and cancellation model |
| `agent-lifecycle-status-transitions-new-agent-creation-defaults-and-dir` | `divergent` | `phases/07-agent-runtime-lifecycle.md` | `ACC-07-AGENT-RUNTIME-LIFECYCLE-0122` | `backend-rs/tests/agent_lifecycle_parity.rs::agent_lifecycle_status_transitions_new_agent_creation_defaults_and_dir` | New agent creation defaults and direct hire semantics |
| `agent-lifecycle-status-transitions-pause-agent` | `missing` | `phases/07-agent-runtime-lifecycle.md` | `ACC-07-AGENT-RUNTIME-LIFECYCLE-0123` | `backend-rs/tests/agent_lifecycle_parity.rs::agent_lifecycle_status_transitions_pause_agent` | Pause agent |
| `agent-lifecycle-status-transitions-rejecting-pending-hires-firing-pend` | `divergent` | `phases/07-agent-runtime-lifecycle.md` | `ACC-07-AGENT-RUNTIME-LIFECYCLE-0124` | `backend-rs/tests/agent_lifecycle_parity.rs::agent_lifecycle_status_transitions_rejecting_pending_hires_firing_pend` | Rejecting pending hires / firing pending hires |
| `agent-lifecycle-status-transitions-resume-agent` | `missing` | `phases/07-agent-runtime-lifecycle.md` | `ACC-07-AGENT-RUNTIME-LIFECYCLE-0125` | `backend-rs/tests/agent_lifecycle_parity.rs::agent_lifecycle_status_transitions_resume_agent` | Resume agent |
| `agent-lifecycle-status-transitions-runtime-state-task-sessions-and-ses` | `missing` | `phases/07-agent-runtime-lifecycle.md` | `ACC-07-AGENT-RUNTIME-LIFECYCLE-0126` | `backend-rs/tests/agent_lifecycle_parity.rs::agent_lifecycle_status_transitions_runtime_state_task_sessions_and_ses` | Runtime state, task sessions, and session reset |
| `agent-lifecycle-status-transitions-runtime-transition-idle-running-idl` | `missing` | `phases/07-agent-runtime-lifecycle.md` | `ACC-07-AGENT-RUNTIME-LIFECYCLE-0127` | `backend-rs/tests/agent_lifecycle_parity.rs::agent_lifecycle_status_transitions_runtime_transition_idle_running_idl` | Runtime transition idle → running → idle/error |
| `agent-lifecycle-status-transitions-terminate-fire-active-agents` | `missing` | `phases/07-agent-runtime-lifecycle.md` | `ACC-07-AGENT-RUNTIME-LIFECYCLE-0128` | `backend-rs/tests/agent_lifecycle_parity.rs::agent_lifecycle_status_transitions_terminate_fire_active_agents` | Terminate/fire active agents |
| `agent-lifecycle-status-transitions-wakeup-heartbeat-invoke` | `missing` | `phases/07-agent-runtime-lifecycle.md` | `ACC-07-AGENT-RUNTIME-LIFECYCLE-0129` | `backend-rs/tests/agent_lifecycle_parity.rs::agent_lifecycle_status_transitions_wakeup_heartbeat_invoke` | Wakeup / heartbeat invoke |
| `cli-api-surface-agent-users-api-agent-approve` | `partial` | `phases/08-interfaces-rest-cli-mcp.md` | `ACC-08-INTERFACES-REST-CLI-MCP-0130` | `backend-rs/tests/api_surface_parity.rs::cli_api_surface_agent_users_api_agent_approve` | POST /api/agents/:id/approve |
| `cli-api-surface-agent-users-api-agent-hires` | `partial` | `phases/08-interfaces-rest-cli-mcp.md` | `ACC-08-INTERFACES-REST-CLI-MCP-0131` | `backend-rs/tests/api_surface_parity.rs::cli_api_surface_agent_users_api_agent_hires` | POST /api/companies/:companyId/agent-hires |
| `cli-api-surface-agent-users-api-agent-keys` | `missing` | `phases/08-interfaces-rest-cli-mcp.md` | `ACC-08-INTERFACES-REST-CLI-MCP-0132` | `backend-rs/tests/api_surface_parity.rs::cli_api_surface_agent_users_api_agent_keys` | Agent API key endpoints |
| `cli-api-surface-agent-users-api-agent-lifecycle-simple` | `missing` | `phases/08-interfaces-rest-cli-mcp.md` | `ACC-08-INTERFACES-REST-CLI-MCP-0133` | `backend-rs/tests/api_surface_parity.rs::cli_api_surface_agent_users_api_agent_lifecycle_simple` | pause/resume/clear-error/delete |
| `cli-api-surface-agent-users-api-agent-me` | `missing` | `phases/08-interfaces-rest-cli-mcp.md` | `ACC-08-INTERFACES-REST-CLI-MCP-0134` | `backend-rs/tests/api_surface_parity.rs::cli_api_surface_agent_users_api_agent_me` | GET /api/agents/me + inbox |
| `cli-api-surface-agent-users-api-agent-permissions` | `missing` | `phases/08-interfaces-rest-cli-mcp.md` | `ACC-08-INTERFACES-REST-CLI-MCP-0135` | `backend-rs/tests/api_surface_parity.rs::cli_api_surface_agent_users_api_agent_permissions` | PATCH /api/agents/:id/permissions |
| `cli-api-surface-agent-users-api-agent-terminate` | `partial` | `phases/08-interfaces-rest-cli-mcp.md` | `ACC-08-INTERFACES-REST-CLI-MCP-0136` | `backend-rs/tests/api_surface_parity.rs::cli_api_surface_agent_users_api_agent_terminate` | POST /api/agents/:id/terminate |
| `cli-api-surface-agent-users-api-agent-wakeup` | `missing` | `phases/08-interfaces-rest-cli-mcp.md` | `ACC-08-INTERFACES-REST-CLI-MCP-0137` | `backend-rs/tests/api_surface_parity.rs::cli_api_surface_agent_users_api_agent_wakeup` | POST /api/agents/:id/wakeup + /heartbeat/invoke |
| `cli-api-surface-agent-users-api-agents-create` | `divergent` | `phases/08-interfaces-rest-cli-mcp.md` | `ACC-08-INTERFACES-REST-CLI-MCP-0138` | `backend-rs/tests/api_surface_parity.rs::cli_api_surface_agent_users_api_agents_create` | POST /api/companies/:companyId/agents |
| `cli-api-surface-agent-users-api-approvals-create` | `missing` | `phases/08-interfaces-rest-cli-mcp.md` | `ACC-08-INTERFACES-REST-CLI-MCP-0139` | `backend-rs/tests/api_surface_parity.rs::cli_api_surface_agent_users_api_approvals_create` | POST /api/companies/:companyId/approvals |
| `cli-api-surface-agent-users-api-approvals-decisions` | `partial` | `phases/08-interfaces-rest-cli-mcp.md` | `ACC-08-INTERFACES-REST-CLI-MCP-0140` | `backend-rs/tests/api_surface_parity.rs::cli_api_surface_agent_users_api_approvals_decisions` | approve/reject/request-revision |
| `cli-api-surface-agent-users-api-approvals-list-get` | `partial` | `phases/08-interfaces-rest-cli-mcp.md` | `ACC-08-INTERFACES-REST-CLI-MCP-0141` | `backend-rs/tests/api_surface_parity.rs::cli_api_surface_agent_users_api_approvals_list_get` | GET approvals list/get |
| `cli-api-surface-agent-users-api-approvals-resubmit-comment` | `missing` | `phases/08-interfaces-rest-cli-mcp.md` | `ACC-08-INTERFACES-REST-CLI-MCP-0142` | `backend-rs/tests/api_surface_parity.rs::cli_api_surface_agent_users_api_approvals_resubmit_comment` | resubmit + comments |
| `cli-api-surface-agent-users-authz-create-gate` | `partial` | `phases/08-interfaces-rest-cli-mcp.md` | `ACC-08-INTERFACES-REST-CLI-MCP-0143` | `backend-rs/tests/api_surface_parity.rs::cli_api_surface_agent_users_authz_create_gate` | agents:create authorization |
| `cli-api-surface-agent-users-authz-read-config` | `missing` | `phases/08-interfaces-rest-cli-mcp.md` | `ACC-08-INTERFACES-REST-CLI-MCP-0144` | `backend-rs/tests/api_surface_parity.rs::cli_api_surface_agent_users_authz_read_config` | config-read authorization |
| `cli-api-surface-agent-users-cli-agent-create` | `missing` | `phases/08-interfaces-rest-cli-mcp.md` | `ACC-08-INTERFACES-REST-CLI-MCP-0145` | `backend-rs/tests/api_surface_parity.rs::cli_api_surface_agent_users_cli_agent_create` | CLI `agent create` |
| `cli-api-surface-agent-users-cli-agent-crud` | `partial` | `phases/08-interfaces-rest-cli-mcp.md` | `ACC-08-INTERFACES-REST-CLI-MCP-0146` | `backend-rs/tests/api_surface_parity.rs::cli_api_surface_agent_users_cli_agent_crud` | CLI agent read/update/delete |
| `cli-api-surface-agent-users-cli-agent-hire` | `partial` | `phases/08-interfaces-rest-cli-mcp.md` | `ACC-08-INTERFACES-REST-CLI-MCP-0147` | `backend-rs/tests/api_surface_parity.rs::cli_api_surface_agent_users_cli_agent_hire` | CLI `agent hire` |
| `cli-api-surface-agent-users-cli-agent-inbox` | `missing` | `phases/08-interfaces-rest-cli-mcp.md` | `ACC-08-INTERFACES-REST-CLI-MCP-0148` | `backend-rs/tests/api_surface_parity.rs::cli_api_surface_agent_users_cli_agent_inbox` | CLI agent inbox commands |
| `cli-api-surface-agent-users-cli-agent-instructions` | `missing` | `phases/08-interfaces-rest-cli-mcp.md` | `ACC-08-INTERFACES-REST-CLI-MCP-0149` | `backend-rs/tests/api_surface_parity.rs::cli_api_surface_agent_users_cli_agent_instructions` | CLI instructions path/bundle/file |
| `cli-api-surface-agent-users-cli-agent-lifecycle` | `missing` | `phases/08-interfaces-rest-cli-mcp.md` | `ACC-08-INTERFACES-REST-CLI-MCP-0150` | `backend-rs/tests/api_surface_parity.rs::cli_api_surface_agent_users_cli_agent_lifecycle` | CLI agent lifecycle verbs |
| `cli-api-surface-agent-users-cli-agent-local-cli` | `missing` | `phases/08-interfaces-rest-cli-mcp.md` | `ACC-08-INTERFACES-REST-CLI-MCP-0151` | `backend-rs/tests/api_surface_parity.rs::cli_api_surface_agent_users_cli_agent_local_cli` | CLI `agent local-cli` |
| `cli-api-surface-agent-users-cli-agent-permissions-config` | `missing` | `phases/08-interfaces-rest-cli-mcp.md` | `ACC-08-INTERFACES-REST-CLI-MCP-0152` | `backend-rs/tests/api_surface_parity.rs::cli_api_surface_agent_users_cli_agent_permissions_config` | CLI permissions/configuration/revisions |
| `cli-api-surface-agent-users-cli-agent-wake` | `missing` | `phases/08-interfaces-rest-cli-mcp.md` | `ACC-08-INTERFACES-REST-CLI-MCP-0153` | `backend-rs/tests/api_surface_parity.rs::cli_api_surface_agent_users_cli_agent_wake` | CLI `agent wake` |
| `cli-api-surface-agent-users-cli-approval-verbs` | `partial` | `phases/08-interfaces-rest-cli-mcp.md` | `ACC-08-INTERFACES-REST-CLI-MCP-0154` | `backend-rs/tests/api_surface_parity.rs::cli_api_surface_agent_users_cli_approval_verbs` | CLI approval commands |
| `cli-api-surface-agent-users-enum-agent-roles-statuses` | `divergent` | `phases/08-interfaces-rest-cli-mcp.md` | `ACC-08-INTERFACES-REST-CLI-MCP-0155` | `backend-rs/tests/api_surface_parity.rs::cli_api_surface_agent_users_enum_agent_roles_statuses` | Agent role/status enums |
| `cli-api-surface-agent-users-enum-approval-types` | `divergent` | `phases/08-interfaces-rest-cli-mcp.md` | `ACC-08-INTERFACES-REST-CLI-MCP-0156` | `backend-rs/tests/api_surface_parity.rs::cli_api_surface_agent_users_enum_approval_types` | Approval type/status enums |
| `cli-api-surface-agent-users-schema-create-agent` | `partial` | `phases/08-interfaces-rest-cli-mcp.md` | `ACC-08-INTERFACES-REST-CLI-MCP-0157` | `backend-rs/tests/api_surface_parity.rs::cli_api_surface_agent_users_schema_create_agent` | createAgent/createAgentHire schema |
| `cli-api-surface-agent-users-schema-permissions` | `partial` | `phases/08-interfaces-rest-cli-mcp.md` | `ACC-08-INTERFACES-REST-CLI-MCP-0158` | `backend-rs/tests/api_surface_parity.rs::cli_api_surface_agent_users_schema_permissions` | agentPermissionsSchema |
| `cli-api-surface-agent-users-side-effect-hire-approval-payload` | `partial` | `phases/08-interfaces-rest-cli-mcp.md` | `ACC-08-INTERFACES-REST-CLI-MCP-0159` | `backend-rs/tests/api_surface_parity.rs::cli_api_surface_agent_users_side_effect_hire_approval_payload` | Hire approval payload snapshot |
| `cli-api-surface-agent-users-side-effect-task-assign-grant` | `missing` | `phases/08-interfaces-rest-cli-mcp.md` | `ACC-08-INTERFACES-REST-CLI-MCP-0160` | `backend-rs/tests/api_surface_parity.rs::cli_api_surface_agent_users_side_effect_task_assign_grant` | Default tasks:assign grant on hire/create |
| `ui-surface-human-non-agent-users-agent-action-assign-task` | `missing` | `phases/09-nuxt-board-ui-parity.md` | `ACC-09-NUXT-BOARD-UI-PARITY-0161` | `frontend-nuxt/tests/ui-parity.test.ts::ui_surface_human_non_agent_users_agent_action_assign_task` | Assign Task action |
| `ui-surface-human-non-agent-users-agent-action-clear-error` | `missing` | `phases/09-nuxt-board-ui-parity.md` | `ACC-09-NUXT-BOARD-UI-PARITY-0162` | `frontend-nuxt/tests/ui-parity.test.ts::ui_surface_human_non_agent_users_agent_action_clear_error` | Clear error action |
| `ui-surface-human-non-agent-users-agent-action-invoke` | `missing` | `phases/09-nuxt-board-ui-parity.md` | `ACC-09-NUXT-BOARD-UI-PARITY-0163` | `frontend-nuxt/tests/ui-parity.test.ts::ui_surface_human_non_agent_users_agent_action_invoke` | Run heartbeat / invoke action |
| `ui-surface-human-non-agent-users-agent-action-overflow-menu` | `missing` | `phases/09-nuxt-board-ui-parity.md` | `ACC-09-NUXT-BOARD-UI-PARITY-0164` | `frontend-nuxt/tests/ui-parity.test.ts::ui_surface_human_non_agent_users_agent_action_overflow_menu` | Overflow menu: duplicate / copy ID / reset sessions / terminate |
| `ui-surface-human-non-agent-users-agent-action-pause-resume` | `missing` | `phases/09-nuxt-board-ui-parity.md` | `ACC-09-NUXT-BOARD-UI-PARITY-0165` | `frontend-nuxt/tests/ui-parity.test.ts::ui_surface_human_non_agent_users_agent_action_pause_resume` | Pause / Resume action |
| `ui-surface-human-non-agent-users-agent-budget-tab` | `missing` | `phases/09-nuxt-board-ui-parity.md` | `ACC-09-NUXT-BOARD-UI-PARITY-0166` | `frontend-nuxt/tests/ui-parity.test.ts::ui_surface_human_non_agent_users_agent_budget_tab` | Per-agent budget policy |
| `ui-surface-human-non-agent-users-agent-config-secret-redaction` | `missing` | `phases/09-nuxt-board-ui-parity.md` | `ACC-09-NUXT-BOARD-UI-PARITY-0167` | `frontend-nuxt/tests/ui-parity.test.ts::ui_surface_human_non_agent_users_agent_config_secret_redaction` | Run log/env secret + path redaction in UI |
| `ui-surface-human-non-agent-users-agent-dashboard-overview` | `missing` | `phases/09-nuxt-board-ui-parity.md` | `ACC-09-NUXT-BOARD-UI-PARITY-0168` | `frontend-nuxt/tests/ui-parity.test.ts::ui_surface_human_non_agent_users_agent_dashboard_overview` | Agent dashboard overview |
| `ui-surface-human-non-agent-users-agent-detail-tabs` | `missing` | `phases/09-nuxt-board-ui-parity.md` | `ACC-09-NUXT-BOARD-UI-PARITY-0169` | `frontend-nuxt/tests/ui-parity.test.ts::ui_surface_human_non_agent_users_agent_detail_tabs` | Agent detail tabbed page |
| `ui-surface-human-non-agent-users-agent-icon-edit` | `missing` | `phases/09-nuxt-board-ui-parity.md` | `ACC-09-NUXT-BOARD-UI-PARITY-0170` | `frontend-nuxt/tests/ui-parity.test.ts::ui_surface_human_non_agent_users_agent_icon_edit` | Edit agent icon |
| `ui-surface-human-non-agent-users-agent-invalid-org-chain` | `missing` | `phases/09-nuxt-board-ui-parity.md` | `ACC-09-NUXT-BOARD-UI-PARITY-0171` | `frontend-nuxt/tests/ui-parity.test.ts::ui_surface_human_non_agent_users_agent_invalid_org_chain` | Invalid reporting-chain warning + gating |
| `ui-surface-human-non-agent-users-agent-membership-join-leave` | `missing` | `phases/09-nuxt-board-ui-parity.md` | `ACC-09-NUXT-BOARD-UI-PARITY-0172` | `frontend-nuxt/tests/ui-parity.test.ts::ui_surface_human_non_agent_users_agent_membership_join_leave` | Join/leave agent membership |
| `ui-surface-human-non-agent-users-agent-pending-approval-banner` | `partial` | `phases/09-nuxt-board-ui-parity.md` | `ACC-09-NUXT-BOARD-UI-PARITY-0173` | `frontend-nuxt/tests/ui-parity.test.ts::ui_surface_human_non_agent_users_agent_pending_approval_banner` | Pending-approval banner + approve on detail |
| `ui-surface-human-non-agent-users-agent-permissions-update` | `missing` | `phases/09-nuxt-board-ui-parity.md` | `ACC-09-NUXT-BOARD-UI-PARITY-0174` | `frontend-nuxt/tests/ui-parity.test.ts::ui_surface_human_non_agent_users_agent_permissions_update` | Update agent permissions/trust |
| `ui-surface-human-non-agent-users-agents-api-surface` | `partial` | `phases/09-nuxt-board-ui-parity.md` | `ACC-09-NUXT-BOARD-UI-PARITY-0175` | `frontend-nuxt/tests/ui-parity.test.ts::ui_surface_human_non_agent_users_agents_api_surface` | Agents REST API client surface |
| `ui-surface-human-non-agent-users-agents-list-view` | `partial` | `phases/09-nuxt-board-ui-parity.md` | `ACC-09-NUXT-BOARD-UI-PARITY-0176` | `frontend-nuxt/tests/ui-parity.test.ts::ui_surface_human_non_agent_users_agents_list_view` | Agents list (list view) |
| `ui-surface-human-non-agent-users-agents-live-run-indicator` | `missing` | `phases/09-nuxt-board-ui-parity.md` | `ACC-09-NUXT-BOARD-UI-PARITY-0177` | `frontend-nuxt/tests/ui-parity.test.ts::ui_surface_human_non_agent_users_agents_live_run_indicator` | Live run indicator |
| `ui-surface-human-non-agent-users-agents-org-chart` | `missing` | `phases/09-nuxt-board-ui-parity.md` | `ACC-09-NUXT-BOARD-UI-PARITY-0178` | `frontend-nuxt/tests/ui-parity.test.ts::ui_surface_human_non_agent_users_agents_org_chart` | Org chart (tree) view |
| `ui-surface-human-non-agent-users-agents-status-filter-tabs` | `missing` | `phases/09-nuxt-board-ui-parity.md` | `ACC-09-NUXT-BOARD-UI-PARITY-0179` | `frontend-nuxt/tests/ui-parity.test.ts::ui_surface_human_non_agent_users_agents_status_filter_tabs` | Agent status filter tabs |
| `ui-surface-human-non-agent-users-approval-approve-reject` | `full` | `phases/09-nuxt-board-ui-parity.md` | `ACC-09-NUXT-BOARD-UI-PARITY-0180` | `frontend-nuxt/tests/ui-parity.test.ts::ui_surface_human_non_agent_users_approval_approve_reject` | Approve / Reject approval |
| `ui-surface-human-non-agent-users-approval-budget-special-case` | `missing` | `phases/09-nuxt-board-ui-parity.md` | `ACC-09-NUXT-BOARD-UI-PARITY-0181` | `frontend-nuxt/tests/ui-parity.test.ts::ui_surface_human_non_agent_users_approval_budget_special_case` | Budget override approval redirect |
| `ui-surface-human-non-agent-users-approval-comments` | `missing` | `phases/09-nuxt-board-ui-parity.md` | `ACC-09-NUXT-BOARD-UI-PARITY-0182` | `frontend-nuxt/tests/ui-parity.test.ts::ui_surface_human_non_agent_users_approval_comments` | Approval comments thread |
| `ui-surface-human-non-agent-users-approval-delete-disapproved-agent` | `divergent` | `phases/09-nuxt-board-ui-parity.md` | `ACC-09-NUXT-BOARD-UI-PARITY-0183` | `frontend-nuxt/tests/ui-parity.test.ts::ui_surface_human_non_agent_users_approval_delete_disapproved_agent` | Delete disapproved hired agent |
| `ui-surface-human-non-agent-users-approval-detail-view` | `missing` | `phases/09-nuxt-board-ui-parity.md` | `ACC-09-NUXT-BOARD-UI-PARITY-0184` | `frontend-nuxt/tests/ui-parity.test.ts::ui_surface_human_non_agent_users_approval_detail_view` | Approval detail page |
| `ui-surface-human-non-agent-users-approval-payload-renderers` | `missing` | `phases/09-nuxt-board-ui-parity.md` | `ACC-09-NUXT-BOARD-UI-PARITY-0185` | `frontend-nuxt/tests/ui-parity.test.ts::ui_surface_human_non_agent_users_approval_payload_renderers` | Typed approval payload renderers |
| `ui-surface-human-non-agent-users-approval-revision-resubmit` | `missing` | `phases/09-nuxt-board-ui-parity.md` | `ACC-09-NUXT-BOARD-UI-PARITY-0186` | `frontend-nuxt/tests/ui-parity.test.ts::ui_surface_human_non_agent_users_approval_revision_resubmit` | Request revision / Mark resubmitted |
| `ui-surface-human-non-agent-users-board-chat-conference-room` | `missing` | `phases/09-nuxt-board-ui-parity.md` | `ACC-09-NUXT-BOARD-UI-PARITY-0187` | `frontend-nuxt/tests/ui-parity.test.ts::ui_surface_human_non_agent_users_board_chat_conference_room` | Board Concierge chat (Conference Room) |
| `ui-surface-human-non-agent-users-board-chat-feedback-votes` | `missing` | `phases/09-nuxt-board-ui-parity.md` | `ACC-09-NUXT-BOARD-UI-PARITY-0188` | `frontend-nuxt/tests/ui-parity.test.ts::ui_surface_human_non_agent_users_board_chat_feedback_votes` | Board chat message feedback votes |
| `ui-surface-human-non-agent-users-company-archive` | `missing` | `phases/09-nuxt-board-ui-parity.md` | `ACC-09-NUXT-BOARD-UI-PARITY-0189` | `frontend-nuxt/tests/ui-parity.test.ts::ui_surface_human_non_agent_users_company_archive` | Archive company (danger zone) |
| `ui-surface-human-non-agent-users-company-board-approval-toggle` | `partial` | `phases/09-nuxt-board-ui-parity.md` | `ACC-09-NUXT-BOARD-UI-PARITY-0190` | `frontend-nuxt/tests/ui-parity.test.ts::ui_surface_human_non_agent_users_company_board_approval_toggle` | Company setting: require board approval for new hires |
| `ui-surface-human-non-agent-users-company-general-settings` | `missing` | `phases/09-nuxt-board-ui-parity.md` | `ACC-09-NUXT-BOARD-UI-PARITY-0191` | `frontend-nuxt/tests/ui-parity.test.ts::ui_surface_human_non_agent_users_company_general_settings` | Company general/appearance settings |
| `ui-surface-human-non-agent-users-company-packages-links` | `missing` | `phases/09-nuxt-board-ui-parity.md` | `ACC-09-NUXT-BOARD-UI-PARITY-0192` | `frontend-nuxt/tests/ui-parity.test.ts::ui_surface_human_non_agent_users_company_packages_links` | Company import/export/cloud links |
| `ui-surface-human-non-agent-users-first-agent-is-ceo` | `missing` | `phases/09-nuxt-board-ui-parity.md` | `ACC-09-NUXT-BOARD-UI-PARITY-0193` | `frontend-nuxt/tests/ui-parity.test.ts::ui_surface_human_non_agent_users_first_agent_is_ceo` | First agent forced to CEO |
| `ui-surface-human-non-agent-users-hire-adapter-presets` | `missing` | `phases/09-nuxt-board-ui-parity.md` | `ACC-09-NUXT-BOARD-UI-PARITY-0194` | `frontend-nuxt/tests/ui-parity.test.ts::ui_surface_human_non_agent_users_hire_adapter_presets` | Adapter-type presets and validation |
| `ui-surface-human-non-agent-users-hire-company-skills-select` | `missing` | `phases/09-nuxt-board-ui-parity.md` | `ACC-09-NUXT-BOARD-UI-PARITY-0195` | `frontend-nuxt/tests/ui-parity.test.ts::ui_surface_human_non_agent_users_hire_company_skills_select` | Optional company-skill selection on hire |
| `ui-surface-human-non-agent-users-hire-test-agent` | `missing` | `phases/09-nuxt-board-ui-parity.md` | `ACC-09-NUXT-BOARD-UI-PARITY-0196` | `frontend-nuxt/tests/ui-parity.test.ts::ui_surface_human_non_agent_users_hire_test_agent` | Test Agent (adapter environment probe) button |
| `ui-surface-human-non-agent-users-hire-trust-preset` | `missing` | `phases/09-nuxt-board-ui-parity.md` | `ACC-09-NUXT-BOARD-UI-PARITY-0197` | `frontend-nuxt/tests/ui-parity.test.ts::ui_surface_human_non_agent_users_hire_trust_preset` | Trust preset + low-trust boundary picker on hire |
| `ui-surface-human-non-agent-users-invite-external-agent` | `missing` | `phases/09-nuxt-board-ui-parity.md` | `ACC-09-NUXT-BOARD-UI-PARITY-0198` | `frontend-nuxt/tests/ui-parity.test.ts::ui_surface_human_non_agent_users_invite_external_agent` | External agent invite + onboarding prompt |
| `ui-surface-human-non-agent-users-new-agent-dialog-choices` | `missing` | `phases/09-nuxt-board-ui-parity.md` | `ACC-09-NUXT-BOARD-UI-PARITY-0199` | `frontend-nuxt/tests/ui-parity.test.ts::ui_surface_human_non_agent_users_new_agent_dialog_choices` | New Agent dialog with 4 creation paths |
| `ui-surface-human-non-agent-users-new-agent-hire-form` | `partial` | `phases/09-nuxt-board-ui-parity.md` | `ACC-09-NUXT-BOARD-UI-PARITY-0200` | `frontend-nuxt/tests/ui-parity.test.ts::ui_surface_human_non_agent_users_new_agent_hire_form` | Advanced New Agent hire form |
| `catalogs-skills-catalog-validation-and-source-policy` | `missing` | `phases/10-import-export-catalogs-portability.md` | `ACC-10-IMPORT-EXPORT-CATALOGS-PORTABILITY-0201` | `backend-rs/tests/import_export_parity.rs::catalogs_skills_catalog_validation_and_source_policy` | Skills catalog validation and source policy |
| `catalogs-teams-catalog-validation-and-source-policy` | `missing` | `phases/10-import-export-catalogs-portability.md` | `ACC-10-IMPORT-EXPORT-CATALOGS-PORTABILITY-0202` | `backend-rs/tests/import_export_parity.rs::catalogs_teams_catalog_validation_and_source_policy` | Teams catalog validation and source policy |
| `portability-company-export-preview-apply-import-round-trip` | `missing` | `phases/10-import-export-catalogs-portability.md` | `ACC-10-IMPORT-EXPORT-CATALOGS-PORTABILITY-0203` | `backend-rs/tests/import_export_parity.rs::portability_company_export_preview_apply_import_round_trip` | Company export preview/apply/import round trip |
| `portability-export-bundle-secret-redaction-and-no-key-hash-leakage` | `missing` | `phases/10-import-export-catalogs-portability.md` | `ACC-10-IMPORT-EXPORT-CATALOGS-PORTABILITY-0204` | `backend-rs/tests/import_export_parity.rs::portability_export_bundle_secret_redaction_and_no_key_hash_leakage` | Export bundle secret redaction and no key_hash leakage |
| `templates-reusable-company-team-templates-and-onboarding-packages` | `missing` | `phases/10-import-export-catalogs-portability.md` | `ACC-10-IMPORT-EXPORT-CATALOGS-PORTABILITY-0205` | `backend-rs/tests/import_export_parity.rs::templates_reusable_company_team_templates_and_onboarding_packages` | Reusable company/team templates and onboarding packages |
| `observability-dashboard-health-doctor-observability` | `missing` | `phases/11-observability-feedback-pipelines.md` | `ACC-11-OBSERVABILITY-FEEDBACK-PIPELINES-0206` | `backend-rs/tests/observability_parity.rs::observability_dashboard_health_doctor_observability` | Dashboard, health, doctor observability |
| `observability-feedback-exports-and-votes` | `missing` | `phases/11-observability-feedback-pipelines.md` | `ACC-11-OBSERVABILITY-FEEDBACK-PIPELINES-0207` | `backend-rs/tests/observability_parity.rs::observability_feedback_exports_and_votes` | Feedback exports and votes |
| `observability-pipeline-cases-events-outputs-conversation-context` | `missing` | `phases/11-observability-feedback-pipelines.md` | `ACC-11-OBSERVABILITY-FEEDBACK-PIPELINES-0208` | `backend-rs/tests/observability_parity.rs::observability_pipeline_cases_events_outputs_conversation_context` | Pipeline cases, events, outputs, conversation context |
| `observability-productivity-review-states` | `missing` | `phases/11-observability-feedback-pipelines.md` | `ACC-11-OBSERVABILITY-FEEDBACK-PIPELINES-0209` | `backend-rs/tests/observability_parity.rs::observability_productivity_review_states` | Productivity review states |
| `observability-release-and-e2e-smoke-result-ingestion` | `missing` | `phases/11-observability-feedback-pipelines.md` | `ACC-11-OBSERVABILITY-FEEDBACK-PIPELINES-0210` | `backend-rs/tests/observability_parity.rs::observability_release_and_e2e_smoke_result_ingestion` | Release and e2e smoke result ingestion |
| `contracts-openapi-schema-export-and-generated-clients` | `missing` | `phases/12-shared-contract-docs-parity.md` | `ACC-12-SHARED-CONTRACT-DOCS-PARITY-0211` | `backend-rs/tests/shared_contract_parity.rs::contracts_openapi_schema_export_and_generated_clients` | OpenAPI/schema export and generated clients |
| `contracts-packages-shared-constants-validators-api-path-parity-surface` | `missing` | `phases/12-shared-contract-docs-parity.md` | `ACC-12-SHARED-CONTRACT-DOCS-PARITY-0212` | `backend-rs/tests/shared_contract_parity.rs::contracts_packages_shared_constants_validators_api_path_parity_surface` | packages/shared constants, validators, API path parity surface |
| `docs-internal-doc-architecture-records-kept-current` | `missing` | `phases/12-shared-contract-docs-parity.md` | `ACC-12-SHARED-CONTRACT-DOCS-PARITY-0213` | `backend-rs/tests/shared_contract_parity.rs::docs_internal_doc_architecture_records_kept_current` | Internal doc architecture records kept current |
| `docs-no-internal-ids-secrets-tailnet-or-agent-urls-in-public-docs` | `missing` | `phases/12-shared-contract-docs-parity.md` | `ACC-12-SHARED-CONTRACT-DOCS-PARITY-0214` | `backend-rs/tests/shared_contract_parity.rs::docs_no_internal_ids_secrets_tailnet_or_agent_urls_in_public_docs` | No internal IDs, secrets, tailnet, or agent URLs in public docs |
| `docs-public-docs-navigation-docs-docs-json-accuracy` | `missing` | `phases/12-shared-contract-docs-parity.md` | `ACC-12-SHARED-CONTRACT-DOCS-PARITY-0215` | `backend-rs/tests/shared_contract_parity.rs::docs_public_docs_navigation_docs_docs_json_accuracy` | Public docs navigation docs/docs.json accuracy |
| `cutover-compatibility-views-and-rpc-aliases-during-transition` | `missing` | `phases/13-migration-compatibility-cutover.md` | `ACC-13-MIGRATION-COMPATIBILITY-CUTOVER-0216` | `acceptance/run_original_gap_diff.py::cutover_compatibility_views_and_rpc_aliases_during_transition` | Compatibility views and RPC aliases during transition |
| `cutover-feature-flags-and-strangler-routing-between-old-and-new-surfaces` | `missing` | `phases/13-migration-compatibility-cutover.md` | `ACC-13-MIGRATION-COMPATIBILITY-CUTOVER-0217` | `acceptance/run_original_gap_diff.py::cutover_feature_flags_and_strangler_routing_between_old_and_new_surfaces` | Feature flags and strangler routing between old and new surfaces |
| `cutover-forward-only-data-preserving-migrations-and-backfill` | `missing` | `phases/13-migration-compatibility-cutover.md` | `ACC-13-MIGRATION-COMPATIBILITY-CUTOVER-0218` | `acceptance/run_original_gap_diff.py::cutover_forward_only_data_preserving_migrations_and_backfill` | Forward-only data-preserving migrations and backfill |
| `cutover-post-cutover-validation-queries-and-supabase-advisors` | `missing` | `phases/13-migration-compatibility-cutover.md` | `ACC-13-MIGRATION-COMPATIBILITY-CUTOVER-0219` | `acceptance/run_original_gap_diff.py::cutover_post_cutover_validation_queries_and_supabase_advisors` | Post-cutover validation queries and Supabase advisors |
| `cutover-rollback-without-destructive-enum-table-drops` | `missing` | `phases/13-migration-compatibility-cutover.md` | `ACC-13-MIGRATION-COMPATIBILITY-CUTOVER-0220` | `acceptance/run_original_gap_diff.py::cutover_rollback_without_destructive_enum_table_drops` | Rollback without destructive enum/table drops |

## Row schema

| Field | Required | Meaning |

hook: PostToolUse Completed
hook: PostToolUse
hook: PostToolUse Completed
exec
/bin/bash -lc "wc -l docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.jsonl && sed -n '1,20p' docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.jsonl" in /home/frankji/Projects/paperclip
 succeeded in 0ms:
220 docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.jsonl
{"acceptance_id": "ACC-00-PARITY-ORACLE-AND-MATRIX-0001", "capability": "No-secret scanner implementation across all surfaces", "current_rewrite_evidence": "not yet covered by rewrite parity matrix; implementation evidence required", "domain": "Parity oracle", "evidence_ref": "to be filled when phase implements source-backed behavior", "failing_test": "acceptance/run_original_gap_diff.py::parity_oracle_no_secret_scanner_implementation_across_all_surfaces", "final_status": "unclosed", "initial_status": "missing", "original_behavior": "No-secret scanner implementation across all surfaces", "original_evidence": "No-secret scanner implementation across all surfaces", "passing_test": "python3 docs/plans/2026-07-05-full-paperclip-parity/acceptance/run_original_gap_diff.py", "phase": "phases/00-parity-oracle-and-matrix.md", "review_note": "Added after Codex review: plan-design added this V1 area as a phase, so it must own row-level closure rows, not just a filename.", "row_id": "parity-oracle-no-secret-scanner-implementation-across-all-surfaces", "verdict": "PLAN_DESIGN_EXTENSION", "waiver_ref": ""}
{"acceptance_id": "ACC-00-PARITY-ORACLE-AND-MATRIX-0002", "capability": "run_cli_mcp_surface.py and run_frontend_user_flow.py harnesses", "current_rewrite_evidence": "not yet covered by rewrite parity matrix; implementation evidence required", "domain": "Parity oracle", "evidence_ref": "to be filled when phase implements source-backed behavior", "failing_test": "acceptance/run_original_gap_diff.py::parity_oracle_run_cli_mcp_surface_py_and_run_frontend_user_flow_py_harnesses", "final_status": "unclosed", "initial_status": "missing", "original_behavior": "run_cli_mcp_surface.py and run_frontend_user_flow.py harnesses", "original_evidence": "run_cli_mcp_surface.py and run_frontend_user_flow.py harnesses", "passing_test": "python3 docs/plans/2026-07-05-full-paperclip-parity/acceptance/run_original_gap_diff.py", "phase": "phases/00-parity-oracle-and-matrix.md", "review_note": "Added after Codex review: plan-design added this V1 area as a phase, so it must own row-level closure rows, not just a filename.", "row_id": "parity-oracle-run-cli-mcp-surface-py-and-run-frontend-user-flow-py-harnesses", "verdict": "PLAN_DESIGN_EXTENSION", "waiver_ref": ""}
{"acceptance_id": "ACC-00-PARITY-ORACLE-AND-MATRIX-0003", "capability": "run_original_gap_diff.py exact gaps_* output contract", "current_rewrite_evidence": "not yet covered by rewrite parity matrix; implementation evidence required", "domain": "Parity oracle", "evidence_ref": "to be filled when phase implements source-backed behavior", "failing_test": "acceptance/run_original_gap_diff.py::parity_oracle_run_original_gap_diff_py_exact_gaps_output_contract", "final_status": "unclosed", "initial_status": "missing", "original_behavior": "run_original_gap_diff.py exact gaps_* output contract", "original_evidence": "run_original_gap_diff.py exact gaps_* output contract", "passing_test": "python3 docs/plans/2026-07-05-full-paperclip-parity/acceptance/run_original_gap_diff.py", "phase": "phases/00-parity-oracle-and-matrix.md", "review_note": "Added after Codex review: plan-design added this V1 area as a phase, so it must own row-level closure rows, not just a filename.", "row_id": "parity-oracle-run-original-gap-diff-py-exact-gaps-output-contract", "verdict": "PLAN_DESIGN_EXTENSION", "waiver_ref": ""}
{"acceptance_id": "ACC-00-PARITY-ORACLE-AND-MATRIX-0004", "capability": "run_real_user_real_agent.py live harness implementation", "current_rewrite_evidence": "not yet covered by rewrite parity matrix; implementation evidence required", "domain": "Parity oracle", "evidence_ref": "to be filled when phase implements source-backed behavior", "failing_test": "acceptance/run_original_gap_diff.py::parity_oracle_run_real_user_real_agent_py_live_harness_implementation", "final_status": "unclosed", "initial_status": "missing", "original_behavior": "run_real_user_real_agent.py live harness implementation", "original_evidence": "run_real_user_real_agent.py live harness implementation", "passing_test": "python3 docs/plans/2026-07-05-full-paperclip-parity/acceptance/run_original_gap_diff.py", "phase": "phases/00-parity-oracle-and-matrix.md", "review_note": "Added after Codex review: plan-design added this V1 area as a phase, so it must own row-level closure rows, not just a filename.", "row_id": "parity-oracle-run-real-user-real-agent-py-live-harness-implementation", "verdict": "PLAN_DESIGN_EXTENSION", "waiver_ref": ""}
{"acceptance_id": "ACC-01-SCHEMA-AUTHZ-PARITY-0005", "capability": "AGENT_ADAPTER_TYPES enum", "current_rewrite_evidence": "supabase/migrations/paperclip/0013_agents_hiring.sql:30", "domain": "Data model & schema", "evidence_ref": "original_evidence + current_rewrite_evidence", "failing_test": "backend-rs/tests/authz_parity.rs::data_model_schema_agent_adapter_types_enum", "final_status": "unclosed", "initial_status": "divergent", "original_behavior": "", "original_evidence": "", "passing_test": "cd backend-rs && cargo test --test authz_parity", "phase": "phases/01-schema-authz-parity.md", "review_note": "Verified 0013:30 adapter_type text default 'openai', no enum constraint. Original curated set (constants.ts:30-44) defaults 'process' and does not include 'openai'. Free-text with divergent default = divergent. Stands.", "row_id": "data-model-schema-agent-adapter-types-enum", "verdict": "CONFIRMED", "waiver_ref": ""}
{"acceptance_id": "ACC-01-SCHEMA-AUTHZ-PARITY-0006", "capability": "per-agent monthly budget accounting", "current_rewrite_evidence": "none found", "domain": "Data model & schema", "evidence_ref": "original_evidence + current_rewrite_evidence", "failing_test": "backend-rs/tests/authz_parity.rs::data_model_schema_agent_budget_fields", "final_status": "unclosed", "initial_status": "missing", "original_behavior": "", "original_evidence": "", "passing_test": "cd backend-rs && cargo test --test authz_parity", "phase": "phases/01-schema-authz-parity.md", "review_note": "Per-agent verdict correct: agents (0013:21-38) has no budget_monthly_cents/spent_monthly_cents. CAVEAT on the assessment note: its claim 'No budget accounting exists' is overstated \u2014 a COMPANY-level budget hard-stop exists in backend-rs/src/budgets.rs (Budget with monthly_limit_cents/spent_cents, BudgetStore, over-budget auto-pause) plus MCP paperclipGetBudget and a frontend BudgetView. That is an in-memory store, not agents schema columns, so the schema-parity status 'missing' for per-agent budget fields still stands.", "row_id": "data-model-schema-agent-budget-fields", "verdict": "CONFIRMED", "waiver_ref": ""}
{"acceptance_id": "ACC-01-SCHEMA-AUTHZ-PARITY-0007", "capability": "agent_config_revisions audit table", "current_rewrite_evidence": "none found", "domain": "Data model & schema", "evidence_ref": "original_evidence + current_rewrite_evidence", "failing_test": "backend-rs/tests/authz_parity.rs::data_model_schema_agent_config_revisions_table", "final_status": "unclosed", "initial_status": "missing", "original_behavior": "", "original_evidence": "", "passing_test": "cd backend-rs && cargo test --test authz_parity", "phase": "phases/01-schema-authz-parity.md", "review_note": "Confirmed: grep for agent_config_revision found no table or config audit/rollback mechanism; agents have no editable config surface. Original 0006:1-13. Missing stands.", "row_id": "data-model-schema-agent-config-revisions-table", "verdict": "CONFIRMED", "waiver_ref": ""}
{"acceptance_id": "ACC-01-SCHEMA-AUTHZ-PARITY-0008", "capability": "AGENT_ICON_NAMES enum", "current_rewrite_evidence": "none found", "domain": "Data model & schema", "evidence_ref": "original_evidence + current_rewrite_evidence", "failing_test": "backend-rs/tests/authz_parity.rs::data_model_schema_agent_icon_names", "final_status": "unclosed", "initial_status": "missing", "original_behavior": "", "original_evidence": "", "passing_test": "cd backend-rs && cargo test --test authz_parity", "phase": "phases/01-schema-authz-parity.md", "review_note": "Confirmed: no icon column in agents (0013:21-38) and grep for '\\bicon\\b' across the rewrite found no icon enum. Original constants.ts:84-127 is a 41-value set. Missing stands.", "row_id": "data-model-schema-agent-icon-names", "verdict": "CONFIRMED", "waiver_ref": ""}
{"acceptance_id": "ACC-01-SCHEMA-AUTHZ-PARITY-0009", "capability": "AGENT_ROLES enum + labels", "current_rewrite_evidence": "supabase/migrations/paperclip/0013_agents_hiring.sql:26,86,105", "domain": "Data model & schema", "evidence_ref": "original_evidence + current_rewrite_evidence", "failing_test": "backend-rs/tests/authz_parity.rs::data_model_schema_agent_roles_enum", "final_status": "unclosed", "initial_status": "divergent", "original_behavior": "", "original_evidence": "", "passing_test": "cd backend-rs && cargo test --test authz_parity", "phase": "phases/01-schema-authz-parity.md", "review_note": "CHANGED missing->divergent for consistency and accuracy. The role column DOES exist as free-text with the correct default 'general' (0013:26) and is actively used by hire_agent (0013:86 p_role default 'general', 0013:105 coalesce(p_role,'general')). Its sibling item agent-adapter-types-enum was scored 'divergent' for the identical structural situation (free-text column replacing a curated enum). 'missing' under-credits: the field is present in weaker/unconstrained form (no enum, no AGENT_ROLE_LABELS map), which is divergence, not absence.", "row_id": "data-model-schema-agent-roles-enum", "verdict": "CHANGED", "waiver_ref": ""}
{"acceptance_id": "ACC-01-SCHEMA-AUTHZ-PARITY-0010", "capability": "AGENT_STATUSES enum values", "current_rewrite_evidence": "supabase/migrations/paperclip/0013_agents_hiring.sql:11,29", "domain": "Data model & schema", "evidence_ref": "original_evidence + current_rewrite_evidence", "failing_test": "backend-rs/tests/authz_parity.rs::data_model_schema_agent_statuses_enum", "final_status": "unclosed", "initial_status": "divergent", "original_behavior": "", "original_evidence": "", "passing_test": "cd backend-rs && cargo test --test authz_parity", "phase": "phases/01-schema-authz-parity.md", "review_note": "Verified 0013:11 enum = ('pending_approval','active','paused','archived'), DB default 'pending_approval' (0013:29). Original set (constants.ts:19-27) adds idle/running/error/terminated and defaults 'idle'; rewrite adds 'archived' not in original. Different value set + default = divergent. Stands.", "row_id": "data-model-schema-agent-statuses-enum", "verdict": "CONFIRMED", "waiver_ref": ""}
{"acceptance_id": "ACC-01-SCHEMA-AUTHZ-PARITY-0011", "capability": "agents:create permission key", "current_rewrite_evidence": "supabase/migrations/paperclip/0013_agents_hiring.sql:96-98,124-132", "domain": "Data model & schema", "evidence_ref": "original_evidence + current_rewrite_evidence", "failing_test": "backend-rs/tests/authz_parity.rs::data_model_schema_agents_create_permission_key", "final_status": "unclosed", "initial_status": "partial", "original_behavior": "", "original_evidence": "", "passing_test": "cd backend-rs && cargo test --test authz_parity", "phase": "phases/01-schema-authz-parity.md", "review_note": "Verified: no PERMISSION_KEYS RBAC catalog. Hire authz uses fn_has_team_role owner/admin/operator (0013:96), api-key scope 'agents:write' (0013:124), and per-agent permissions.can_create_agents (0013:131). Analogous gate, not the named 'agents:create' key system. Partial stands.", "row_id": "data-model-schema-agents-create-permission-key", "verdict": "CONFIRMED", "waiver_ref": ""}
{"acceptance_id": "ACC-01-SCHEMA-AUTHZ-PARITY-0012", "capability": "agents FK constraints and indexes", "current_rewrite_evidence": "supabase/migrations/paperclip/0013_agents_hiring.sql:23,28,39-40", "domain": "Data model & schema", "evidence_ref": "original_evidence + current_rewrite_evidence", "failing_test": "backend-rs/tests/authz_parity.rs::data_model_schema_agents_fk_indexes", "final_status": "unclosed", "initial_status": "divergent", "original_behavior": "", "original_evidence": "", "passing_test": "cd backend-rs && cargo test --test authz_parity", "phase": "phases/01-schema-authz-parity.md", "review_note": "Verified. company_id FK is ON DELETE cascade (0013:23) and reports_to is ON DELETE set null (0013:28), vs original ON DELETE no action (0000:166-167). Only single-column agents_company_idx and agents_team_idx exist (0013:39-40); the composite agents_company_status_idx and agents_company_reports_to_idx (0000:195-196) are absent. Divergent stands.", "row_id": "data-model-schema-agents-fk-indexes", "verdict": "CONFIRMED", "waiver_ref": ""}
{"acceptance_id": "ACC-01-SCHEMA-AUTHZ-PARITY-0013", "capability": "agents ORM adds icon/runtime_config/env/pause/error columns", "current_rewrite_evidence": "none found", "domain": "Data model & schema", "evidence_ref": "original_evidence + current_rewrite_evidence", "failing_test": "backend-rs/tests/authz_parity.rs::data_model_schema_agents_orm_extra_columns", "final_status": "unclosed", "initial_status": "missing", "original_behavior": "", "original_evidence": "", "passing_test": "cd backend-rs && cargo test --test authz_parity", "phase": "phases/01-schema-authz-parity.md", "review_note": "Confirmed absent: grep across supabase/migrations/paperclip, backend-rs/src, frontend-nuxt for icon, runtime_config, default_environment, pause_reason, paused_at, error_reason returned no hits. No environments concept exists in the rewrite. Missing stands.", "row_id": "data-model-schema-agents-orm-extra-columns", "verdict": "CONFIRMED", "waiver_ref": ""}
{"acceptance_id": "ACC-01-SCHEMA-AUTHZ-PARITY-0014", "capability": "agents.permissions jsonb column", "current_rewrite_evidence": "supabase/migrations/paperclip/0013_agents_hiring.sql:32", "domain": "Data model & schema", "evidence_ref": "original_evidence + current_rewrite_evidence", "failing_test": "backend-rs/tests/authz_parity.rs::data_model_schema_agents_permissions_column", "final_status": "unclosed", "initial_status": "partial", "original_behavior": "", "original_evidence": "", "passing_test": "cd backend-rs && cargo test --test authz_parity", "phase": "phases/01-schema-authz-parity.md", "review_note": "Verified 0013:32 permissions jsonb default '{\"can_create_agents\": false}'. Only carries can_create_agents (snake_case); no canCreateSkills, trustPreset, authorizationPolicy, and no validation schema (orig agent.ts:13-18). Single-field subset = partial. Stands.", "row_id": "data-model-schema-agents-permissions-column", "verdict": "CONFIRMED", "waiver_ref": ""}
{"acceptance_id": "ACC-01-SCHEMA-AUTHZ-PARITY-0015", "capability": "agents table column set", "current_rewrite_evidence": "supabase/migrations/paperclip/0013_agents_hiring.sql:21-38", "domain": "Data model & schema", "evidence_ref": "original_evidence + current_rewrite_evidence", "failing_test": "backend-rs/tests/authz_parity.rs::data_model_schema_agents_table_columns", "final_status": "unclosed", "initial_status": "partial", "original_behavior": "", "original_evidence": "", "passing_test": "cd backend-rs && cargo test --test authz_parity", "phase": "phases/01-schema-authz-parity.md", "review_note": "Verified against 0013:21-38 and 0000:25-43. Rewrite agents = id, company_id, team_id, name, role, title, reports_to, status, adapter_type, model, permissions, capabilities, created_by, created_at, updated_at, meta. Core identity/hierarchy columns present; adapter_config, context_mode, budget_monthly_cents, spent_monthly_cents, last_heartbeat_at all absent; capabilities is jsonb '[]' not text; metadata renamed meta. Subset-with-omissions = partial. Stands.", "row_id": "data-model-schema-agents-table-columns", "verdict": "CONFIRMED", "waiver_ref": ""}
{"acceptance_id": "ACC-01-SCHEMA-AUTHZ-PARITY-0016", "capability": "approval_comments table", "current_rewrite_evidence": "none found", "domain": "Data model & schema", "evidence_ref": "original_evidence + current_rewrite_evidence", "failing_test": "backend-rs/tests/authz_parity.rs::data_model_schema_approval_comments_table", "final_status": "unclosed", "initial_status": "missing", "original_behavior": "", "original_evidence": "", "passing_test": "cd backend-rs && cargo test --test authz_parity", "phase": "phases/01-schema-authz-parity.md", "review_note": "Confirmed: grep for approval_comment across the rewrite found no table or commenting surface. Original 0005:1-19. Missing stands.", "row_id": "data-model-schema-approval-comments-table", "verdict": "CONFIRMED", "waiver_ref": ""}
{"acceptance_id": "ACC-01-SCHEMA-AUTHZ-PARITY-0017", "capability": "APPROVAL_STATUSES enum", "current_rewrite_evidence": "supabase/migrations/paperclip/0013_agents_hiring.sql:17,49", "domain": "Data model & schema", "evidence_ref": "original_evidence + current_rewrite_evidence", "failing_test": "backend-rs/tests/authz_parity.rs::data_model_schema_approval_statuses_enum", "final_status": "unclosed", "initial_status": "partial", "original_behavior": "", "original_evidence": "", "passing_test": "cd backend-rs && cargo test --test authz_parity", "phase": "phases/01-schema-authz-parity.md", "review_note": "Verified 0013:17 enum = ('pending','approved','rejected','cancelled') default pending (0013:49). Missing revision_requested (constants.ts:546-553); no revision lifecycle. Partial stands.", "row_id": "data-model-schema-approval-statuses-enum", "verdict": "CONFIRMED", "waiver_ref": ""}
{"acceptance_id": "ACC-01-SCHEMA-AUTHZ-PARITY-0018", "capability": "APPROVAL_TYPES enum", "current_rewrite_evidence": "supabase/migrations/paperclip/0013_agents_hiring.sql:14", "domain": "Data model & schema", "evidence_ref": "original_evidence + current_rewrite_evidence", "failing_test": "backend-rs/tests/authz_parity.rs::data_model_schema_approval_types_enum", "final_status": "unclosed", "initial_status": "partial", "original_behavior": "", "original_evidence": "", "passing_test": "cd backend-rs && cargo test --test authz_parity", "phase": "phases/01-schema-authz-parity.md", "review_note": "Verified 0013:14 enum = ('hire_agent','approve_ceo_strategy'). Missing budget_override_required and request_board_approval (constants.ts:538-544). hire_agent present. Partial stands.", "row_id": "data-model-schema-approval-types-enum", "verdict": "CONFIRMED", "waiver_ref": ""}
{"acceptance_id": "ACC-01-SCHEMA-AUTHZ-PARITY-0019", "capability": "approval lifecycle validators", "current_rewrite_evidence": "supabase/migrations/paperclip/0013_agents_hiring.sql:154-206", "domain": "Data model & schema", "evidence_ref": "original_evidence + current_rewrite_evidence", "failing_test": "backend-rs/tests/authz_parity.rs::data_model_schema_approval_validators", "final_status": "unclosed", "initial_status": "partial", "original_behavior": "", "original_evidence": "", "passing_test": "cd backend-rs && cargo test --test authz_parity", "phase": "phases/01-schema-authz-parity.md", "review_note": "Verified: only decide_approval/decide_approval_with_key(approval_id, approve:boolean) exist (0013:154-206), a binary approve/reject setting subject agent active/archived. No decisionNote capture, no request-revision, resubmit, or comment; create is implicit via hire. Two-state subset = partial. Stands.", "row_id": "data-model-schema-approval-validators", "verdict": "CONFIRMED", "waiver_ref": ""}
{"acceptance_id": "ACC-01-SCHEMA-AUTHZ-PARITY-0020", "capability": "approvals index", "current_rewrite_evidence": "supabase/migrations/paperclip/0013_agents_hiring.sql:46,58-59", "domain": "Data model & schema", "evidence_ref": "original_evidence + current_rewrite_evidence", "failing_test": "backend-rs/tests/authz_parity.rs::data_model_schema_approvals_index", "final_status": "unclosed", "initial_status": "partial", "original_behavior": "", "original_evidence": "", "passing_test": "cd backend-rs && cargo test --test authz_parity", "phase": "phases/01-schema-authz-parity.md", "review_note": "Verified: only approvals_team_idx and approvals_company_idx (0013:58-59); composite approvals_company_status_type_idx (0000:197) absent. company_id FK is ON DELETE cascade (0013:46) vs orig no action. Some index coverage present but not the board-queue composite = partial. Stands.", "row_id": "data-model-schema-approvals-index", "verdict": "CONFIRMED", "waiver_ref": ""}

hook: PostToolUse
hook: PostToolUse Completed
hook: PreToolUse
hook: PreToolUse Completed
exec
/bin/bash -lc "python3 - <<'PY'
import json, collections, pathlib
p=pathlib.Path('docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.jsonl')
rows=[json.loads(l) for l in p.read_text().splitlines() if l.strip()]
print('rows',len(rows))
print('initial',collections.Counter(r['initial_status'] for r in rows))
ph=collections.Counter(r['phase'] for r in rows)
for k in sorted(ph): print(ph[k], k)
print('phase_count',len(ph))
print('missing fields', sum(not all(r.get(f) for f in ['phase','acceptance_id','failing_test','passing_test','evidence_ref','final_status']) for r in rows))
PY" in /home/frankji/Projects/paperclip
 succeeded in 0ms:
rows 220
initial Counter({'missing': 145, 'partial': 50, 'divergent': 22, 'full': 3})
4 phases/00-parity-oracle-and-matrix.md
55 phases/01-schema-authz-parity.md
10 phases/02-goals-projects-issues-work-products.md
5 phases/03-workspaces-routines-plugins-sandbox.md
6 phases/04-deployment-auth-onboarding-ops.md
12 phases/05-adapters-secrets-skills-config.md
21 phases/06-approvals-budgets-costs-activity.md
16 phases/07-agent-runtime-lifecycle.md
31 phases/08-interfaces-rest-cli-mcp.md
40 phases/09-nuxt-board-ui-parity.md
5 phases/10-import-export-catalogs-portability.md
5 phases/11-observability-feedback-pipelines.md
5 phases/12-shared-contract-docs-parity.md
5 phases/13-migration-compatibility-cutover.md
phase_count 14
missing fields 0

hook: PostToolUse
hook: PostToolUse Completed
codex
The machine-readable matrix assigns rows to all 14 phase filenames, but I found a count inconsistency: the matrix files show 220 rows with 145 missing rows, while the README/audit summary still say 175 rows with 100 missing. I’m running the read-only gap gate once to verify its actual output contract.
hook: PreToolUse
hook: PreToolUse Completed
exec
/bin/bash -lc 'python3 docs/plans/2026-07-05-full-paperclip-parity/acceptance/run_original_gap_diff.py; code=$?; printf '"'\\nEXIT=%s\\n' \""'$code"' in /home/frankji/Projects/paperclip
 succeeded in 0ms:
gaps_missing=145 gaps_partial=50 gaps_divergent=22 rows_total=220 rows_open=217
OPEN_ROWS_SAMPLE:
- parity-oracle-no-secret-scanner-implementation-across-all-surfaces [missing] phases/00-parity-oracle-and-matrix.md :: No-secret scanner implementation across all surfaces
- parity-oracle-run-cli-mcp-surface-py-and-run-frontend-user-flow-py-harnesses [missing] phases/00-parity-oracle-and-matrix.md :: run_cli_mcp_surface.py and run_frontend_user_flow.py harnesses
- parity-oracle-run-original-gap-diff-py-exact-gaps-output-contract [missing] phases/00-parity-oracle-and-matrix.md :: run_original_gap_diff.py exact gaps_* output contract
- parity-oracle-run-real-user-real-agent-py-live-harness-implementation [missing] phases/00-parity-oracle-and-matrix.md :: run_real_user_real_agent.py live harness implementation
- data-model-schema-agent-adapter-types-enum [divergent] phases/01-schema-authz-parity.md :: AGENT_ADAPTER_TYPES enum
- data-model-schema-agent-budget-fields [missing] phases/01-schema-authz-parity.md :: per-agent monthly budget accounting
- data-model-schema-agent-config-revisions-table [missing] phases/01-schema-authz-parity.md :: agent_config_revisions audit table
- data-model-schema-agent-icon-names [missing] phases/01-schema-authz-parity.md :: AGENT_ICON_NAMES enum
- data-model-schema-agent-roles-enum [divergent] phases/01-schema-authz-parity.md :: AGENT_ROLES enum + labels
- data-model-schema-agent-statuses-enum [divergent] phases/01-schema-authz-parity.md :: AGENT_STATUSES enum values
- data-model-schema-agents-create-permission-key [partial] phases/01-schema-authz-parity.md :: agents:create permission key
- data-model-schema-agents-fk-indexes [divergent] phases/01-schema-authz-parity.md :: agents FK constraints and indexes
- data-model-schema-agents-orm-extra-columns [missing] phases/01-schema-authz-parity.md :: agents ORM adds icon/runtime_config/env/pause/error columns
- data-model-schema-agents-permissions-column [partial] phases/01-schema-authz-parity.md :: agents.permissions jsonb column
- data-model-schema-agents-table-columns [partial] phases/01-schema-authz-parity.md :: agents table column set
- data-model-schema-approval-comments-table [missing] phases/01-schema-authz-parity.md :: approval_comments table
- data-model-schema-approval-statuses-enum [partial] phases/01-schema-authz-parity.md :: APPROVAL_STATUSES enum
- data-model-schema-approval-types-enum [partial] phases/01-schema-authz-parity.md :: APPROVAL_TYPES enum
- data-model-schema-approval-validators [partial] phases/01-schema-authz-parity.md :: approval lifecycle validators
- data-model-schema-approvals-index [partial] phases/01-schema-authz-parity.md :: approvals index

EXIT=1

hook: PostToolUse
hook: PostToolUse Completed
codex
BLOCKERS

1. Baseline tally is inconsistent across reviewed files.
   - `README.md` and `01-parity-audit-summary.md` say authoritative tally is `3 full / 50 partial / 100 missing / 22 divergent` across 175 rows.
   - `evidence/rewrite-gap-matrix.md` and `evidence/rewrite-gap-matrix.jsonl` show `3 full / 50 partial / 145 missing / 22 divergent` across 220 rows.
   - `acceptance/run_original_gap_diff.py` actually prints `gaps_missing=145 gaps_partial=50 gaps_divergent=22 rows_total=220 rows_open=217`.
   This makes the plan internally contradictory about the authoritative gap baseline.

PATCHES

- Update `README.md`, `01-parity-audit-summary.md`, and any related summary text to match the matrix if 220 rows is intended: `3 full / 50 partial / 145 missing / 22 divergent`.
- Or reduce/fix the matrix if the intended authoritative baseline is still 175 rows and 100 missing.
- After that, keep `run_original_gap_diff.py` output aligned with the chosen authoritative matrix.

Prior blocker status:
1. Exact `gaps_*` output: resolved structurally. The script emits the exact success line and failure output begins with `gaps_missing=... gaps_partial=... gaps_divergent=...`.
2. Live harness placeholders: resolved for a PLAN artifact. The three live harnesses intentionally return non-zero, and Phase 00 explicitly owns replacing them before release.
3. Matrix row assignment to all 14 phases: resolved. `rewrite-gap-matrix.jsonl` has rows for all 14 phase files under `phases/`.

VERDICT

FAIL, due to the unresolved authoritative tally contradiction between summary files and the matrix/harness output.
hook: Stop
hook: Stop Completed
tokens used
53,683
BLOCKERS

1. Baseline tally is inconsistent across reviewed files.
   - `README.md` and `01-parity-audit-summary.md` say authoritative tally is `3 full / 50 partial / 100 missing / 22 divergent` across 175 rows.
   - `evidence/rewrite-gap-matrix.md` and `evidence/rewrite-gap-matrix.jsonl` show `3 full / 50 partial / 145 missing / 22 divergent` across 220 rows.
   - `acceptance/run_original_gap_diff.py` actually prints `gaps_missing=145 gaps_partial=50 gaps_divergent=22 rows_total=220 rows_open=217`.
   This makes the plan internally contradictory about the authoritative gap baseline.

PATCHES

- Update `README.md`, `01-parity-audit-summary.md`, and any related summary text to match the matrix if 220 rows is intended: `3 full / 50 partial / 145 missing / 22 divergent`.
- Or reduce/fix the matrix if the intended authoritative baseline is still 175 rows and 100 missing.
- After that, keep `run_original_gap_diff.py` output aligned with the chosen authoritative matrix.

Prior blocker status:
1. Exact `gaps_*` output: resolved structurally. The script emits the exact success line and failure output begins with `gaps_missing=... gaps_partial=... gaps_divergent=...`.
2. Live harness placeholders: resolved for a PLAN artifact. The three live harnesses intentionally return non-zero, and Phase 00 explicitly owns replacing them before release.
3. Matrix row assignment to all 14 phases: resolved. `rewrite-gap-matrix.jsonl` has rows for all 14 phase files under `phases/`.

VERDICT

FAIL, due to the unresolved authoritative tally contradiction between summary files and the matrix/harness output.
2026-07-05T23:16:36.863675Z ERROR rmcp::transport::streamable_http_client: fail to delete session: unexpected server response: DELETE returned HTTP 404 session_id="eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpZCI6IjkzM2ZlNzVhLTU0MmItNDhlMy05MjNjLTg4MzFhNmM2NDg5ZCIsImNsaWVudEluZm8iOnsibmFtZSI6ImNvZGV4LW1jcC1jbGllbnQiLCJ0aXRsZSI6IkNvZGV4IiwidmVyc2lvbiI6IjAuMTQxLjAifSwiY2xpZW50Q2FwYWJpbGl0aWVzIjp7ImVsaWNpdGF0aW9uIjp7fX0sImlhdCI6MTc4MzI5MzI4OH0.zjuCzEkr5ArMeQ0g6yZMFI3AY9mytZV2jBm3u2yISto"
