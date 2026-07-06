Reading additional input from stdin...
2026-07-05T23:18:50.178873Z ERROR codex_models_manager::manager: failed to refresh available models: stream disconnected before completion: failed to decode models response: expected value at line 1 column 1; body: <!doctype html>
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
  <script nonce="kHL2/CWtE5sCYa0QM+Brbg==">window.__APP_CONFIG__={"registration_enabled":false,"email_verify_enabled":false,"registration_email_suffix_whitelist":[],"promo_code_enabled":true,"password_reset_enabled":false,"invitation_code_enabled":false,"totp_enabled":false,"login_agreement_enabled":false,"login_agreement_mode":"modal","login_agreement_updated_at":"2026-03-31","login_agreement_revision":"0eff189d66e0af4e","login_agreement_documents":[{"id":"terms","title":"服务条款","content_md":""},{"id":"usage-policy","title":"使用政策","content_md":""},{"id":"supported-regions","title":"支持的国家和地区","content_md":""},{"id":"service-specific-terms","title":"服务特定条款","content_md":""}],"turnstile_enabled":false,"turnstile_site_key":"","site_name":"Sub2API","site_logo":"","site_subtitle":"Subscription to API Conversion Platform","api_base_url":"","contact_info":"","doc_url":"","home_content":"","hide_ccs_import_button":false,"purchase_subscription_enabled":false,"purchase_subscription_url":"","table_default_page_size":20,"table_page_size_options":[10,20,50],"custom_menu_items":[],"custom_endpoints":[],"linuxdo_oauth_enabled":false,"dingtalk_oauth_enabled":false,"wechat_oauth_enabled":false,"wechat_oauth_open_enabled":false,"wechat_oauth_mp_enabled":false,"wechat_oauth_mobile_enabled":false,"oidc_oauth_enabled":false,"oidc_oauth_provider_name":"OIDC","github_oauth_enabled":false,"google_oauth_enabled":false,"backend_mode_enabled":false,"payment_enabled":false,"version":"","server_timezone":"Asia/Shanghai","server_utc_offset":"+08:00","balance_low_notify_enabled":false,"account_quota_notify_enabled":false,"balance_low_notify_threshold":0,"balance_low_notify_recharge_url":"","channel_monitor_enabled":true,"channel_monitor_default_interval_seconds":60,"available_channels_enabled":false,"affiliate_enabled":false,"risk_control_enabled":false,"allow_user_view_error_requests":false};</script></head>
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
session id: 019f3493-d043-7e60-b1ad-b0d0cef2f4b4
--------
user
Final plan re-review (read-only, do not edit). Check only these files: docs/plans/2026-07-05-full-paperclip-parity/README.md, 01-parity-audit-summary.md, 04-execution-order.md, 05-test-strategy.md, phases/00-parity-oracle-and-matrix.md, acceptance/full-parity-harness-spec.md, acceptance/run_original_gap_diff.py, acceptance/run_real_user_real_agent.py, acceptance/run_cli_mcp_surface.py, acceptance/run_frontend_user_flow.py, codex/phase-prompts.md, reviews/review-checklist.md, evidence/rewrite-gap-matrix.md, evidence/rewrite-gap-matrix.jsonl, and phase filenames under phases/. Prior blocker was an authoritative-tally contradiction between summary files (175 rows / 100 missing) and the matrix (220 rows / 145 missing). Confirm whether the tallies now agree across README, audit summary, test strategy, matrix files, and the run_original_gap_diff.py output. The plan is a PLAN artifact: live harness scaffolds that intentionally return non-zero are acceptable as Phase 00 RED deliverables because Phase 00 explicitly owns replacing them before release. Return BLOCKERS, PATCHES, VERDICT (PASS or FAIL).
2026-07-05T23:18:50.869816Z ERROR rmcp::transport::worker: worker quit with fatal: Transport channel closed, when Auth(AuthorizationRequired)
hook: SessionStart
hook: SessionStart Completed
hook: UserPromptSubmit
hook: UserPromptSubmit Completed
codex
I’ll do this as a read-only plan review, scoped to the listed files and the phase filenames only. I’m using the repo instructions plus verification-before-completion because the request is specifically to validate a prior blocker before giving a verdict.
hook: PreToolUse
hook: PreToolUse Completed
hook: PreToolUse
hook: PreToolUse
hook: PreToolUse Completed
hook: PreToolUse
hook: PreToolUse Completed
hook: PreToolUse Completed
exec
/bin/bash -lc "sed -n '1,220p' /home/frankji/.codex/superpowers/skills/verification-before-completion/SKILL.md" in /home/frankji/Projects/paperclip
 succeeded in 0ms:
---
name: verification-before-completion
description: Use when about to claim work is complete, fixed, or passing, before committing or creating PRs - requires running verification commands and confirming output before making any success claims; evidence before assertions always
---

# Verification Before Completion

## Overview

Claiming work is complete without verification is dishonesty, not efficiency.

**Core principle:** Evidence before claims, always.

**Violating the letter of this rule is violating the spirit of this rule.**

## The Iron Law

```
NO COMPLETION CLAIMS WITHOUT FRESH VERIFICATION EVIDENCE
```

If you haven't run the verification command in this message, you cannot claim it passes.

## The Gate Function

```
BEFORE claiming any status or expressing satisfaction:

1. IDENTIFY: What command proves this claim?
2. RUN: Execute the FULL command (fresh, complete)
3. READ: Full output, check exit code, count failures
4. VERIFY: Does output confirm the claim?
   - If NO: State actual status with evidence
   - If YES: State claim WITH evidence
5. ONLY THEN: Make the claim

Skip any step = lying, not verifying
```

## Common Failures

| Claim | Requires | Not Sufficient |
|-------|----------|----------------|
| Tests pass | Test command output: 0 failures | Previous run, "should pass" |
| Linter clean | Linter output: 0 errors | Partial check, extrapolation |
| Build succeeds | Build command: exit 0 | Linter passing, logs look good |
| Bug fixed | Test original symptom: passes | Code changed, assumed fixed |
| Regression test works | Red-green cycle verified | Test passes once |
| Agent completed | VCS diff shows changes | Agent reports "success" |
| Requirements met | Line-by-line checklist | Tests passing |

## Red Flags - STOP

- Using "should", "probably", "seems to"
- Expressing satisfaction before verification ("Great!", "Perfect!", "Done!", etc.)
- About to commit/push/PR without verification
- Trusting agent success reports
- Relying on partial verification
- Thinking "just this once"
- Tired and wanting work over
- **ANY wording implying success without having run verification**

## Rationalization Prevention

| Excuse | Reality |
|--------|---------|
| "Should work now" | RUN the verification |
| "I'm confident" | Confidence ≠ evidence |
| "Just this once" | No exceptions |
| "Linter passed" | Linter ≠ compiler |
| "Agent said success" | Verify independently |
| "I'm tired" | Exhaustion ≠ excuse |
| "Partial check is enough" | Partial proves nothing |
| "Different words so rule doesn't apply" | Spirit over letter |

## Key Patterns

**Tests:**
```
✅ [Run test command] [See: 34/34 pass] "All tests pass"
❌ "Should pass now" / "Looks correct"
```

**Regression tests (TDD Red-Green):**
```
✅ Write → Run (pass) → Revert fix → Run (MUST FAIL) → Restore → Run (pass)
❌ "I've written a regression test" (without red-green verification)
```

**Build:**
```
✅ [Run build] [See: exit 0] "Build passes"
❌ "Linter passed" (linter doesn't check compilation)
```

**Requirements:**
```
✅ Re-read plan → Create checklist → Verify each → Report gaps or completion
❌ "Tests pass, phase complete"
```

**Agent delegation:**
```
✅ Agent reports success → Check VCS diff → Verify changes → Report actual state
❌ Trust agent report
```

## Why This Matters

From 24 failure memories:
- your human partner said "I don't believe you" - trust broken
- Undefined functions shipped - would crash
- Missing requirements shipped - incomplete features
- Time wasted on false completion → redirect → rework
- Violates: "Honesty is a core value. If you lie, you'll be replaced."

## When To Apply

**ALWAYS before:**
- ANY variation of success/completion claims
- ANY expression of satisfaction
- ANY positive statement about work state
- Committing, PR creation, task completion
- Moving to next task
- Delegating to agents

**Rule applies to:**
- Exact phrases
- Paraphrases and synonyms
- Implications of success
- ANY communication suggesting completion/correctness

## The Bottom Line

**No shortcuts for verification.**

Run the command. Read the output. THEN claim the result.

This is non-negotiable.

exec
/bin/bash -lc "sed -n '1,220p' doc/GOAL.md" in /home/frankji/Projects/paperclip
 succeeded in 0ms:
# Paperclip

**Paperclip is the backbone of the autonomous economy.** We are building the infrastructure that autonomous AI companies run on. Our goal is for Paperclip-powered companies to collectively generate economic output that rivals the GDP of the world's largest countries. Every decision we make should serve that: make autonomous companies more capable, more governable, more scalable, and more real.

## The Vision

Autonomous companies — AI workforces organized with real structure, governance, and accountability — will become a major force in the global economy. Not one company. Thousands. Millions. An entire economic layer that runs on AI labor, coordinated through Paperclip.

Paperclip is not the company. Paperclip is what makes the companies possible. We are the control plane, the nervous system, the operating layer. Every autonomous company needs structure, task management, cost control, goal alignment, and human governance. That's us. We are to autonomous companies what the corporate operating system is to human ones — except this time, the operating system is real software, not metaphor.

The measure of our success is not whether one company works. It's whether Paperclip becomes the default foundation that autonomous companies are built on — and whether those companies, collectively, become a serious economic force that rivals the output of nations.

## The Problem

Task management software doesn't go far enough. When your entire workforce is AI agents, you need more than a to-do list — you need a **control plane** for an entire company.

## What This Is

Paperclip is the command, communication, and control plane for a company of AI agents. It is the single place where you:

- **Manage agents as employees** — hire, organize, and track who does what
- **Define org structure** — org charts that agents themselves operate within
- **Track work in real time** — see at any moment what every agent is working on
- **Control costs** — token salary budgets per agent, spend tracking, burn rate
- **Align to goals** — agents see how their work serves the bigger mission
- **Preserve work context** — comments, documents, work products, attachments, and company state stay attached to the work

## Architecture

Two layers:

### 1. Control Plane (this software)

The central nervous system. Manages:

- Agent registry and org chart
- Task assignment and status
- Budget and token spend tracking
- Issue comments, documents, work products, attachments, and company state
- Goal hierarchy (company → team → agent → task)
- Heartbeat monitoring — know when agents are alive, idle, or stuck

It also enforces execution-control semantics such as single-assignee issues, atomic checkout and execution locks, blockers, recovery issues, and workspace/runtime controls.

### 2. Execution Services (adapters)

Agents run externally and report into the control plane. Adapters connect different execution environments and define how a heartbeat is invoked, observed, and cancelled:

- **Local CLI/session adapters** — built-in adapters for tools such as Claude Code, Codex, Gemini, OpenCode, Pi, and Cursor
- **HTTP/process-style adapters** — command or webhook/API integrations for custom runtimes
- **OpenClaw gateway** — integration for OpenClaw-style remote agents
- **External adapter plugins** — dynamically loaded adapters installed outside the core app

The control plane doesn't run agents. It orchestrates them. Agents run wherever they run and phone home.

## Core Principle

You should be able to look at Paperclip and understand your entire company at a glance — who's doing what, how much it costs, and whether it's working.

hook: PostToolUse
hook: PostToolUse
exec
/bin/bash -lc "sed -n '1,220p' doc/SPEC-implementation.md" in /home/frankji/Projects/paperclip
 succeeded in 0ms:
# Paperclip V1 Implementation Spec

Status: Implementation contract for first release (V1)
Date: 2026-04-28
Audience: Product, engineering, and agent-integration authors
Source inputs: `GOAL.md`, `PRODUCT.md`, `SPEC.md`, `DATABASE.md`, current monorepo code

## 1. Document Role

`SPEC.md` remains the long-horizon product spec.
This document is the concrete, build-ready V1 contract.
When there is a conflict, `SPEC-implementation.md` controls V1 behavior.

## 2. V1 Outcomes

Paperclip V1 must provide a full control-plane loop for autonomous agents:

1. A human board creates a company and defines goals.
2. The board creates and manages agents in an org tree.
3. Agents receive and execute tasks via heartbeat invocations.
4. All work is tracked through tasks/comments with audit visibility.
5. Token/cost usage is reported and budget limits can stop work.
6. The board can intervene anywhere (pause agents/tasks, override decisions).

Success means one operator can run a small AI-native company end-to-end with clear visibility and control.

## 3. Explicit V1 Product Decisions

These decisions close open questions from `SPEC.md` for V1.

| Topic | V1 Decision |
|---|---|
| Tenancy | Single-tenant deployment, multi-company data model |
| Company model | Company is first-order; all business entities are company-scoped |
| Board | Single human board operator per deployment |
| Org graph | Strict tree (`reports_to` nullable root); no multi-manager reporting |
| Visibility | Company-scoped visibility: board + all in-company agents can see all work objects by default; public/private deployment flags affect external exposure only and do **not** imply project/issue privacy |
| Communication | Tasks + comments only (no separate chat system) |
| Task ownership | Single assignee; atomic checkout required for `in_progress` transition |
| Task watchdogs | A task watchdog is an explicitly configured, issue-subtree-scoped verification and recovery capacity. It may restore live task paths inside the watched subtree and resolve only eligible task-level plan confirmations; it is not board authority, active-run output monitoring, or general liveness recovery. |
| Recovery | Liveness/watchdog recovery preserves explicit ownership: retry lost execution continuity where safe, otherwise open visible source-scoped recovery actions by default, use issue-backed recovery only for independent repair work, or require human escalation (see `doc/execution-semantics.md`) |
| Agent adapters | Built-in `process`, `http`, local CLI/session adapters, and OpenClaw gateway support; external adapters can also be loaded through the adapter plugin flow |
| Plugin framework | Local/self-hosted early plugin runtime is in scope; cloud marketplace and packaged public distribution remain out of scope |
| Auth | Mode-dependent human auth (`local_trusted` implicit board in current code; authenticated mode uses sessions), API keys for agents |
| Budget period | Monthly UTC calendar window |
| Budget enforcement | Soft alerts + hard limit auto-pause |
| Deployment modes | Canonical model is `local_trusted` + `authenticated` with `private/public` exposure policy (see `doc/DEPLOYMENT-MODES.md`) |

Low-trust agent presets are containment controls for hostile automated work, not
general project or issue privacy controls. The core preset resolver contract is
documented in `doc/LOW-TRUST-PRESETS.md`.

## 4. Current Baseline (Repo Snapshot)

As of 2026-02-17, the repo already includes:

- Node + TypeScript backend with REST CRUD for `agents`, `projects`, `goals`, `issues`, `activity`
- React UI pages for dashboard/agents/projects/goals/issues lists
- PostgreSQL schema via Drizzle with embedded PostgreSQL fallback when `DATABASE_URL` is unset

V1 implementation extends this baseline into a company-centric, governance-aware control plane.

## 5. V1 Scope

## 5.1 In Scope

- Company lifecycle (create/list/get/update/archive)
- Goal hierarchy linked to company mission
- Agent lifecycle with org structure and adapter configuration
- Task lifecycle with parent/child hierarchy and comments
- Atomic task checkout and explicit task status transitions
- Board approvals for hires and CEO strategy proposal
- Heartbeat invocation, status tracking, and cancellation
- Cost event ingestion and rollups (agent/task/project/company)
- Budget settings and hard-stop enforcement
- Board web UI for dashboard, org chart, tasks, agents, approvals, costs
- Agent-facing API contract (task read/write, heartbeat report, cost report)
- Auditable activity log for all mutating actions

## 5.2 Out of Scope (V1)

- Cloud-grade plugin marketplace/distribution beyond the local/self-hosted plugin runtime
- Revenue/expense accounting beyond model/token costs
- Knowledge base subsystem
- Public marketplace (ClipHub)
- Multi-board governance or role-based human permission granularity
- Automatic self-healing orchestration (auto-reassign/retry planners)

## 6. Architecture

## 6.1 Runtime Components

- `server/`: REST API, auth, orchestration services
- `ui/`: Board operator interface
- `packages/db/`: Drizzle schema, migrations, DB clients (Postgres)
- `packages/shared/`: Shared API types, validators, constants

## 6.2 Data Stores

- Primary: PostgreSQL
- Local default: embedded PostgreSQL at `~/.paperclip/instances/default/db`
- Optional local prod-like: Docker Postgres
- Optional hosted: Supabase/Postgres-compatible
- File/object storage:
  - local default: `~/.paperclip/instances/default/data/storage` (`local_disk`)
  - cloud: S3-compatible object storage (`s3`)

## 6.3 Background Processing

A lightweight scheduler/worker in the server process handles:

- heartbeat trigger checks
- stuck run detection
- budget threshold checks

Separate queue infrastructure is not required for V1.

## 7. Canonical Data Model (V1)

All core tables include `id`, `created_at`, `updated_at` unless noted.

## 7.0 Auth Tables

Human auth tables (`users`, `sessions`, and provider-specific auth artifacts) are managed by the selected auth library. This spec treats them as required dependencies and references `users.id` where user attribution is needed.

## 7.1 `companies`

- `id` uuid pk
- `name` text not null
- `description` text null
- `status` enum: `active | paused | archived`
- `pause_reason` text null
- `paused_at` timestamptz null
- `issue_prefix` text not null
- `issue_counter` int not null
- `budget_monthly_cents` int not null default 0
- `spent_monthly_cents` int not null default 0
- `attachment_max_bytes` int not null
- `require_board_approval_for_new_agents` boolean not null default false
- feedback sharing consent fields
- branding fields such as `brand_color`

Invariant: every business record belongs to exactly one company.

## 7.2 `agents`

- `id` uuid pk
- `company_id` uuid fk `companies.id` not null
- `name` text not null
- `role` text not null
- `title` text null
- `icon` text null
- `status` enum: `active | paused | idle | running | error | pending_approval | terminated`
- `reports_to` uuid fk `agents.id` null
- `capabilities` text null
- `adapter_type` text; built-ins include `process`, `http`, `claude_local`, `codex_local`, `gemini_local`, `opencode_local`, `pi_local`, `cursor`, `hermes_local`, `hermes_gateway`, and `openclaw_gateway`
- `adapter_config` jsonb not null
- `runtime_config` jsonb not null default `{}`; may include Paperclip runtime policy such as `modelProfiles.cheap.adapterConfig` for an optional low-cost model lane that does not change the primary adapter config
- `default_environment_id` uuid fk `environments.id` null
- `context_mode` enum: `thin | fat` default `thin`
- `budget_monthly_cents` int not null default 0
- `spent_monthly_cents` int not null default 0
- pause fields: `pause_reason`, `paused_at`
- `permissions` jsonb not null default `{}`
- `last_heartbeat_at` timestamptz null
- `metadata` jsonb null

Invariants:

- agent and manager must be in same company
- no cycles in reporting tree
- `terminated` agents cannot be resumed

## 7.3 `agent_api_keys`

- `id` uuid pk
- `agent_id` uuid fk `agents.id` not null
- `company_id` uuid fk `companies.id` not null
- `name` text not null
- `key_hash` text not null
- `last_used_at` timestamptz null
- `revoked_at` timestamptz null

Invariant: plaintext key shown once at creation; only hash stored.

## 7.4 `goals`

- `id` uuid pk
- `company_id` uuid fk not null
- `title` text not null
- `description` text null
- `level` enum: `company | team | agent | task`
- `parent_id` uuid fk `goals.id` null
- `owner_agent_id` uuid fk `agents.id` null
- `status` enum: `planned | active | achieved | cancelled`

Invariant: at least one root `company` level goal per company.

## 7.5 `projects`

- `id` uuid pk
- `company_id` uuid fk not null
- `goal_id` uuid fk `goals.id` null
- `name` text not null
- `description` text null
- `status` enum: `backlog | planned | in_progress | completed | cancelled`
- `lead_agent_id` uuid fk `agents.id` null
- `target_date` date null
- `env` jsonb null (same secret-aware env binding format used by agent config)

Invariant:

- project env is merged into run environment for issues in that project and overrides conflicting agent env keys before Paperclip runtime-owned keys are injected

Routine execution issues add a routine-scoped env overlay after project env and before Paperclip runtime-owned keys. Routine env uses the same secret-aware binding format, is stored on `routines.env`, is snapshotted in routine revisions, and resolves secret refs against the routine binding target so routine-owned secrets do not require direct bindings on the executing agent.

## 7.6 `issues` (core task entity)

- `id` uuid pk
- `company_id` uuid fk not null

hook: PostToolUse
hook: PostToolUse Completed
hook: PostToolUse Completed
hook: PostToolUse Completed
exec
/bin/bash -lc "sed -n '1,220p' doc/PRODUCT.md" in /home/frankji/Projects/paperclip
 succeeded in 0ms:
# Paperclip — Product Definition

## What It Is

Paperclip is the control plane for autonomous AI companies. One instance of Paperclip can run multiple companies. A **company** is a first-order object.

## Core Concepts

### Company

A company has:

- A **goal** — the reason it exists ("Create the #1 AI note-taking app that does $1M MRR within 3 months")
- **Employees** — every employee is an AI agent
- **Org structure** — who reports to whom
- **Revenue & expenses** — tracked at the company level
- **Task hierarchy** — all work traces back to the company goal

### Employees & Agents

Every employee is an agent. When you create a company, you start by defining the CEO, then build out from there.

Each employee has:

- **Adapter type + config** — how this agent runs and what defines its identity/behavior. This is adapter-specific (e.g., an OpenClaw agent might use SOUL.md and HEARTBEAT.md files; a Claude Code agent might use CLAUDE.md; a bare script might use CLI args). Paperclip doesn't prescribe the format — the adapter does.
- **Role & reporting** — their title, who they report to, who reports to them
- **Capabilities description** — a short paragraph on what this agent does and when they're relevant (helps other agents discover who can help with what)

Example: A CEO agent's adapter config tells it to "review what your executives are doing, check company metrics, reprioritize if needed, assign new strategic initiatives" on each heartbeat. An engineer's config tells it to "check assigned tasks, pick the highest priority, and work it."

Then you define who reports to the CEO: a CTO managing programmers, a CMO managing the marketing team, and so on. Every agent in the tree gets their own adapter configuration.

### Agent Execution

Paperclip supports several ways to run an agent's heartbeat:

1. **Local CLI/session adapters** — Paperclip starts or resumes local coding-tool sessions such as Claude Code, Codex, Gemini, OpenCode, Pi, and Cursor, then tracks the run.
2. **Run a command** — Paperclip kicks off a process (shell command, Python script, etc.) and tracks it. The heartbeat is "execute this and monitor it."
3. **Fire and forget a request** — Paperclip sends a webhook/API call to an externally running agent. The heartbeat is "notify this agent to wake up." OpenClaw-style hooks work this way.
4. **External adapter plugins** — Paperclip loads adapter packages through the plugin/adapter flow so self-hosted installs can add runtimes without hardcoding them in core.

Agent runs can use project and execution workspaces, managed runtime services such as preview/dev servers, adapter-specific session state, and HTTP/webhook-style execution. We provide sensible defaults, but the adapter is still the boundary: if a runtime can be invoked, observed, and authorized, Paperclip can coordinate it.

### Task Management

Task management is hierarchical. At any moment, every piece of work must trace back to the company's top-level goal through a chain of parent tasks:

```
I am researching the Facebook ads Granola uses (current task)
  because → I need to create Facebook ads for our software (parent)
    because → I need to grow new signups by 100 users (parent)
      because → I need to get revenue to $2,000 this week (parent)
        because → ...
          because → We're building the #1 AI note-taking app to $1M MRR in 3 months
```

Tasks have parentage. Every task exists in service of a parent task, all the way up to the company goal. This is what keeps autonomous agents aligned — they can always answer "why am I doing this?"

The current issue model includes stable issue identifiers, parent/sub-issues, blockers, a single assignee, comments, issue documents, attachments and work products, and review/approval handoffs. That structure keeps work inspectable by both the board and agents while still allowing agents to decompose work into smaller tasks.

## Principles

1. **Unopinionated about how you run your agents.** Your agents could be OpenClaw bots, Python scripts, Node scripts, Claude Code sessions, Codex instances — we don't care. Paperclip defines the control plane for communication and provides utility infrastructure for heartbeats. It does not mandate an agent runtime.

2. **Company is the unit of organization.** Everything lives under a company. One Paperclip instance, many companies.

3. **Adapter config defines the agent.** Every agent has an adapter type and configuration that controls its identity and behavior. The minimum contract is just "be callable."

4. **All work traces to the goal.** Hierarchical task management means nothing exists in isolation. If you can't explain why a task matters to the company goal, it shouldn't exist.

5. **Control plane, not execution plane.** Paperclip orchestrates. Agents run wherever they run and phone home.

## User Flow (Dream Scenario)

1. Open Paperclip, create a new company
2. Define the company's goal: "Create the #1 AI note-taking app, $1M MRR in 3 months"
3. Create the CEO
   - Choose an adapter (e.g., process adapter for Claude Code, HTTP adapter for OpenClaw)
   - Configure the adapter (agent identity, loop behavior, execution settings)
   - CEO proposes strategic breakdown → board approves
4. Define the CEO's reports: CTO, CMO, CFO, etc.
   - Each gets their own adapter config and role definition
5. Define their reports: engineers under CTO, marketers under CMO, etc.
6. Set budgets, define initial strategic tasks
7. Hit go — agents start their heartbeats and the company runs

## Guidelines

There are two runtime modes Paperclip must support:

- `local_trusted` (default): single-user local trusted deployment with no login friction
- `authenticated`: login-required mode that supports both private-network and public deployment exposure policies

Canonical mode design and command expectations live in `doc/DEPLOYMENT-MODES.md`.

## Further Detail

See [SPEC.md](./SPEC.md) for the full technical specification and [TASKS.md](./TASKS.md) for the task management data model.

---

Paperclip’s core identity is a **control plane for autonomous AI companies**, centered on **companies, org charts, goals, issues/comments, heartbeats, budgets, approvals, and board governance**. The public docs are also explicit about the current boundaries: **tasks/comments are the built-in communication model**, Paperclip is **not a chatbot**, and it is **not a code review tool**. The roadmap already points toward **easier onboarding, cloud agents, easier agent configuration, plugins, better docs, and ClipMart/ClipHub-style reusable companies/templates**.

## What Paperclip should do vs. not do

**Do**

- Stay **board-level and company-level**. Users should manage goals, orgs, budgets, approvals, and outputs.
- Make the first five minutes feel magical: install, answer a few questions, see a CEO do something real.
- Keep work anchored to **issues/comments/projects/goals**, even if the surface feels conversational.
- Treat **agency / internal team / startup** as the same underlying abstraction with different templates and labels.
- Make outputs first-class: files, docs, reports, previews, links, screenshots.
- Provide **hooks into engineering workflows**: worktrees, preview servers, PR links, external review tools.
- Use **plugins** for edge cases like rich chat, knowledge bases, doc editors, custom tracing.

**Do not**

- Do not make the core product a general chat app. The current product definition is explicitly task/comment-centric and “not a chatbot,” and that boundary is valuable.
- Do not build a complete Jira/GitHub replacement. The repo/docs already position Paperclip as organization orchestration, not focused on pull-request review.
- Do not build enterprise-grade RBAC first. Paperclip now has authenticated mode, company memberships, instance roles, and permission grants, but fine-grained enterprise governance should remain secondary to the core company control plane.
- Do not interpret agent-level privacy flags as a project/issue privacy feature in V1; work visibility stays company-scoped.
- Do not lead with raw bash logs and transcripts. Default view should be human-readable intent/progress, with raw detail beneath.
- Do not force users to understand provider/API-key plumbing unless absolutely necessary. There are active onboarding/auth issues already; friction here is clearly real.

## Specific design goals

1. **Time-to-first-success under 5 minutes**
   A fresh user should go from install to “my CEO completed a first task” in one sitting.

2. **Board-level abstraction always wins**
   The default UI should answer: what is the company doing, who is doing it, why does it matter, what did it cost, and what needs my approval.

3. **Conversation stays attached to work objects**
   “Chat with CEO” should still resolve to strategy threads, decisions, tasks, or approvals.

4. **Progressive disclosure**
   Top layer: human-readable summary. Middle layer: checklist/steps/artifacts. Bottom layer: raw logs/tool calls/transcript.

5. **Output-first**
   Work is not done until the user can see the result: file, document, preview link, screenshot, plan, or PR.

6. **Execution visibility without log worship**
   Active runs, recovery issues, productivity review states, blockers, and work products should be first-class surfaces. Raw transcripts are available when needed, but they are not the primary product surface.

7. **Local-first, cloud-ready**
   The mental model should not change between local solo use and shared/private or public/cloud deployment.

8. **Safe autonomy**
   Auto mode is allowed; hidden token burn is not.

9. **Thin core, rich edges**
   Put optional chat, knowledge, and special surfaces into plugins/extensions rather than bloating the control plane.

hook: PostToolUse
hook: PostToolUse Completed
codex
I’ve loaded the required repo context and the verification skill. Next I’m reading only the named plan files plus phase filenames, then I’ll run the diff script read-only to compare its reported counts against the docs and matrix.
hook: PreToolUse
hook: PreToolUse Completed
hook: PreToolUse
hook: PreToolUse
hook: PreToolUse Completed
hook: PreToolUse Completed
exec
/bin/bash -lc "sed -n '1,220p' doc/DEVELOPING.md" in /home/frankji/Projects/paperclip
 succeeded in 0ms:
# Developing

This project can run fully in local dev without setting up PostgreSQL manually.

## Deployment Modes

For mode definitions and intended CLI behavior, see `doc/DEPLOYMENT-MODES.md`.

Current implementation status:

- canonical model: `local_trusted` and `authenticated` (with `private/public` exposure)

## Prerequisites

- Node.js 20+
- pnpm 9+

## Dependency Lockfile Policy

GitHub Actions owns `pnpm-lock.yaml`.

- Do not commit `pnpm-lock.yaml` in pull requests.
- Pull request CI validates dependency resolution when manifests change.
- Pushes to `master` regenerate `pnpm-lock.yaml` with `pnpm install --lockfile-only --no-frozen-lockfile`, commit it back if needed, and then run verification with `--frozen-lockfile`.

## Branch Convention

On this host/fork, the default working branch is `franky`.

- Unless a branch is named explicitly, "merge", "commit", and "push" mean on `franky`.
- Do not commit or push to `master` or the upstream default branch without an explicit instruction.
- This is a local maintainer convention for this checkout, not upstream guidance.

## Start Dev

From repo root:

```sh
pnpm install
pnpm dev
```

This starts:

- API server: `http://localhost:3100`
- UI: served by the API server in dev middleware mode (same origin as API)

`pnpm dev` runs the server in watch mode and restarts on changes from workspace packages (including adapter packages). Use `pnpm dev:once` to run without file watching.

`pnpm dev:once` auto-applies pending local migrations by default before starting the dev server.

`pnpm dev` and `pnpm dev:once` are now idempotent for the current repo and instance: if the matching Paperclip dev runner is already alive, Paperclip reports the existing process instead of starting a duplicate.

Issue execution may also use project execution workspace policies and workspace runtime services for per-project worktrees, preview servers, and managed dev commands. Configure those through the project workspace/runtime surfaces rather than starting long-running unmanaged processes when a task needs a reusable service.

## Storybook

The board UI Storybook keeps stories and Storybook config under `ui/storybook/` so component review files stay out of the app source routes.

```sh
pnpm storybook
pnpm build-storybook
```

These run the `@paperclipai/ui` Storybook on port `6006` and build the static output to `ui/storybook-static/`.

Inspect or stop the current repo's managed dev runner:

```sh
pnpm dev:list
pnpm dev:stop
```

`pnpm dev:once` now tracks backend-relevant file changes and pending migrations. When the current boot is stale, the board UI shows a `Restart required` banner. You can also enable guarded auto-restart in `Instance Settings > Experimental`, which waits for queued/running local agent runs to finish before restarting the dev server.

Tailscale/private-auth dev mode:

```sh
pnpm dev --bind lan
```

This runs dev as `authenticated/private` with a private-network bind preset.
On a fresh authenticated/private instance, open the app, sign in or create an
account, and use the setup screen to claim the first instance admin from the
browser. The CLI fallback remains:

```sh
pnpm paperclipai auth bootstrap-ceo
```

For Tailscale-only reachability on a detected tailnet address:

```sh
pnpm dev --bind tailnet
```

Legacy aliases still map to the old broad private-network behavior:

```sh
pnpm dev --tailscale-auth
pnpm dev --authenticated-private
```

Allow additional private hostnames (for example custom Tailscale hostnames):

```sh
pnpm paperclipai allowed-hostname dotta-macbook-pro
```

## Test Commands

Use the cheap local default unless you are specifically working on browser flows:

```sh
pnpm test
```

`pnpm test` runs the Vitest suite only. For interactive Vitest watch mode use:

```sh
pnpm test:watch
```

Browser suites stay separate:

```sh
pnpm test:e2e
pnpm test:release-smoke
```

These browser suites are intended for targeted local verification and CI, not the default agent/human test command.

For normal issue work, start with the smallest targeted check that proves the change. Reserve repo-wide typecheck/build/test runs for PR-ready handoff or changes broad enough that narrow checks do not cover the risk.

## One-Command Local Run

For a first-time local install, you can bootstrap and run in one command:

```sh
pnpm paperclipai run
```

> **Note: private npm registry `.npmrc` + first-run onboarding**
>
> The first-run experience often starts with `npx paperclipai onboard --yes` (before you have a repo checkout). If your global `~/.npmrc` sets `registry` to a private registry (for example GitHub Packages), `npx` may try to resolve `paperclipai` from that private registry and fail with `E404`.
>
> Diagnostic:
>
> ```sh
> npm config get registry
> ```
>
> Workaround (cross-platform; force the public npm registry for this command):
>
> ```sh
> npx --registry https://registry.npmjs.org paperclipai onboard --yes
> ```

`paperclipai run` does:

1. auto-onboard if config is missing
2. `paperclipai doctor` with repair enabled
3. starts the server when checks pass

## Docker Quickstart (No local Node install)

Build and run Paperclip in Docker:

```sh
docker build -t paperclip-local .
docker run --name paperclip \
  -p 3100:3100 \
  -e HOST=0.0.0.0 \
  -e PAPERCLIP_HOME=/paperclip \
  -v "$(pwd)/data/docker-paperclip:/paperclip" \
  paperclip-local
```

Or use Compose:

```sh
docker compose -f docker/docker-compose.quickstart.yml up --build
```

See `doc/DOCKER.md` for API key wiring (`OPENAI_API_KEY` / `ANTHROPIC_API_KEY`) and persistence details.

## Docker For Untrusted PR Review

For a separate review-oriented container that keeps `codex`/`claude` login state in Docker volumes and checks out PRs into an isolated scratch workspace, see `doc/UNTRUSTED-PR-REVIEW.md`.

## Local Instance Layout

Every local install keeps runtime state directly under the selected instance root:

```text
~/.paperclip/instances/default/                  # instance root
  config.json                                    # runtime config
  .env                                           # instance env file
  db/                                            # embedded PostgreSQL data
  data/
    storage/                                     # local_disk uploads
    backups/                                     # automatic DB backups
  logs/
  secrets/master.key                             # local_encrypted master key
  workspaces/<agent-id>/                         # default agent workspaces
  projects/                                      # project execution workspaces
  companies/<company-id>/agents/<agent-id>/codex-home/
                                                   # per-agent codex_local home
```

`PAPERCLIP_HOME` and `PAPERCLIP_INSTANCE_ID` override the home root and instance id respectively. `paperclipai onboard` echoes the resolved values in its banner (`Local home: <home> | instance: <id> | config: <path>`) so you can confirm where state will land before continuing.

## Database in Dev (Auto-Handled)

For local development, leave `DATABASE_URL` unset.
The server will automatically use embedded PostgreSQL and persist data at:

- `~/.paperclip/instances/default/db`

Override home or instance:

hook: PreToolUse
hook: PreToolUse Completed
hook: PostToolUse
hook: PostToolUse Completed
exec
/bin/bash -lc "sed -n '1,220p' doc/DATABASE.md" in /home/frankji/Projects/paperclip
 succeeded in 0ms:
# Database

Paperclip uses PostgreSQL via [Drizzle ORM](https://orm.drizzle.team/). There are three ways to run the database, from simplest to most production-ready.

## 1. Embedded PostgreSQL — zero config

If you don't set `DATABASE_URL`, the server automatically starts an embedded PostgreSQL instance and manages a local data directory.

```sh
pnpm dev
```

That's it. On first start the server:

1. Creates a `~/.paperclip/instances/default/db/` directory for storage
2. Ensures the `paperclip` database exists
3. Runs migrations automatically for empty databases
4. Starts serving requests

Data persists across restarts in `~/.paperclip/instances/default/db/`. To reset local dev data, delete that directory.

If you need to apply pending migrations manually, run:

```sh
pnpm db:migrate
```

When `DATABASE_URL` is unset, this command targets the current embedded PostgreSQL instance for your active Paperclip config/instance.

Issue reference mentions follow the normal migration path: the schema migration creates the tracking table, but it does not backfill historical issue titles, descriptions, comments, or documents automatically.

To backfill existing content manually after migrating, run:

```sh
pnpm issue-references:backfill
# optional: limit to one company
pnpm issue-references:backfill -- --company <company-id>
```

Future issue, comment, and document writes sync references automatically without running the backfill command.

This mode is ideal for local development and one-command installs.

Docker note: the Docker quickstart image also uses embedded PostgreSQL by default. Persist `/paperclip` to keep DB state across container restarts (see `doc/DOCKER.md`).

## 2. Local PostgreSQL (Docker)

For a full PostgreSQL server locally, use the included Docker Compose setup:

```sh
docker compose up -d
```

This starts PostgreSQL 17 on `localhost:5432`. Then set the connection string:

```sh
cp .env.example .env
# .env already contains:
# DATABASE_URL=postgres://paperclip:paperclip@localhost:5432/paperclip
```

Run migrations:

```sh
DATABASE_URL=postgres://paperclip:paperclip@localhost:5432/paperclip \
  pnpm db:migrate
```

Start the server:

```sh
pnpm dev
```

## 3. Hosted PostgreSQL (Supabase)

For production, use a hosted PostgreSQL provider. [Supabase](https://supabase.com/) is a good option with a free tier.

### Setup

1. Create a project at [database.new](https://database.new)
2. Go to **Project Settings > Database > Connection string**
3. Copy the URI and replace the password placeholder with your database password

### Connection string

Supabase offers two connection modes:

**Direct connection** (port 5432) — use for migrations and one-off scripts:

```
postgres://postgres.[PROJECT-REF]:[PASSWORD]@aws-0-[REGION].pooler.supabase.com:5432/postgres
```

**Connection pooling via Supavisor** (port 6543) — use for the application:

```
postgres://postgres.[PROJECT-REF]:[PASSWORD]@aws-0-[REGION].pooler.supabase.com:6543/postgres
```

### Configure

For the application runtime, use a direct PostgreSQL connection unless the database client has explicit prepared-statement configuration for your pooling mode:

```sh
DATABASE_URL=postgres://postgres.[PROJECT-REF]:[PASSWORD]@aws-0-[REGION].pooler.supabase.com:5432/postgres
```

If you later run the app with a pooled runtime URL, set `DATABASE_MIGRATION_URL` to the direct connection URL. Paperclip uses it for startup schema checks/migrations and plugin namespace migrations, while the app continues to use `DATABASE_URL` for runtime queries:

```sh
DATABASE_URL=postgres://postgres.[PROJECT-REF]:[PASSWORD]@aws-0-[REGION].pooler.supabase.com:6543/postgres
DATABASE_MIGRATION_URL=postgres://postgres.[PROJECT-REF]:[PASSWORD]@aws-0-[REGION].pooler.supabase.com:5432/postgres
```

If your hosted database requires transaction-pooling-only connections, use a direct or session-pooled connection for Paperclip until runtime pooling support is documented in this guide. Do not edit database client source files as part of deployment setup.

### Push the schema

```sh
# Use the direct connection (port 5432) for schema changes
DATABASE_URL=postgres://postgres.[PROJECT-REF]:[PASSWORD]@...5432/postgres \
  pnpm db:migrate
```

### Free tier limits

- 500 MB database storage
- 200 concurrent connections
- Projects pause after 1 week of inactivity

See [Supabase pricing](https://supabase.com/pricing) for current details.

## Switching between modes

The database mode is controlled by `DATABASE_URL`:

| `DATABASE_URL` | Mode |
|---|---|
| Not set | Embedded PostgreSQL (`~/.paperclip/instances/default/db/`) |
| `postgres://...localhost...` | Local Docker PostgreSQL |
| `postgres://...supabase.com...` | Hosted Supabase |

Your Drizzle schema (`packages/db/src/schema/`) stays the same regardless of mode.

## Resource membership tables

Paperclip stores current-user sidebar membership state in:

- `project_memberships`
- `agent_memberships`

These rows are company-scoped and user-scoped. A missing row means the user is joined, so existing users keep seeing projects and agents in the sidebar until they explicitly leave them. Rows only control sidebar visibility; they do not affect project/agent detail access, all-pages, selectors, assignment flows, or existing company permissions.

Both tables use a unique key on `(company_id, user_id, resource_id)` and keep `state` as `joined` or `left`. Join/leave mutations are idempotent board-user `/me` operations and write activity entries when the effective state changes.

## Plugin database namespaces

The plugin runtime tracks plugin-owned database namespaces and migrations in `plugin_database_namespaces` and `plugin_migrations`. Hosted deployments that separate runtime and migration connections should set `DATABASE_MIGRATION_URL`; plugin namespace migration work uses the migration connection when present.

## Backups

Paperclip supports automatic and manual logical database backups. These dumps include
non-system database schemas such as `public`, the Drizzle migration journal, and
plugin-owned database schemas. See `doc/DEVELOPING.md` for the current
`paperclipai db:backup` / `pnpm db:backup` commands and backup retention
configuration.

Database backups do not include non-database instance files such as local-disk
uploads, workspace files, or the local encrypted secrets master key. Back those paths
up separately when you need full instance disaster recovery.

## Secret storage

Paperclip stores secret metadata and versions in:

- `company_secrets`
- `company_secret_versions`
- `company_secret_bindings`
- `secret_access_events`

Secret-aware env bindings are supported by agents, projects, and routines. Routine env lives in `routines.env`, is captured in `routine_revisions.snapshot`, and routine dispatches store `routine_runs.routine_revision_id` so runtime secret resolution uses the env snapshot that existed when the run was created. Routine secret refs bind with `target_type = 'routine'`, `target_id = routines.id`, and `config_path` values under `env.*`.

For local/default installs, the active provider is `local_encrypted`:

- Secret material is encrypted at rest with a local master key.
- Default key file: `~/.paperclip/instances/default/secrets/master.key` (auto-created if missing).
- CLI config location: `~/.paperclip/instances/default/config.json` under `secrets.localEncrypted.keyFilePath`.
- Backup/restore requires both the database metadata and the local master key file; either artifact alone is insufficient.
- The server best-effort enforces `0600` key file permissions and provider health reports permission warnings.

Optional overrides:

- `PAPERCLIP_SECRETS_MASTER_KEY` (32-byte key as base64, hex, or raw 32-char string)
- `PAPERCLIP_SECRETS_MASTER_KEY_FILE` (custom key file path)

Strict mode to block new inline sensitive env values:

```sh
PAPERCLIP_SECRETS_STRICT_MODE=true
```

You can set strict mode and provider defaults via:

```sh
pnpm paperclipai configure --section secrets
```

Inline secret migration command:

```sh
pnpm paperclipai secrets migrate-inline-env --company-id <company-id> --apply

# direct database maintenance fallback
pnpm secrets:migrate-inline-env --apply
```

Hosted AWS provider notes live in [SECRETS-AWS-PROVIDER.md](./SECRETS-AWS-PROVIDER.md).

hook: PostToolUse
hook: PostToolUse Completed
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

hook: PostToolUse
hook: PostToolUse Completed
exec
/bin/bash -lc 'rg -n "175|100|220|145|missing|rows|total|gap|matrix|authoritative|Phase 00|RED|run_original_gap_diff" docs/plans/2026-07-05-full-paperclip-parity/README.md docs/plans/2026-07-05-full-paperclip-parity/01-parity-audit-summary.md docs/plans/2026-07-05-full-paperclip-parity/04-execution-order.md docs/plans/2026-07-05-full-paperclip-parity/05-test-strategy.md docs/plans/2026-07-05-full-paperclip-parity/phases/00-parity-oracle-and-matrix.md docs/plans/2026-07-05-full-paperclip-parity/acceptance/full-parity-harness-spec.md docs/plans/2026-07-05-full-paperclip-parity/acceptance/run_original_gap_diff.py docs/plans/2026-07-05-full-paperclip-parity/acceptance/run_real_user_real_agent.py docs/plans/2026-07-05-full-paperclip-parity/acceptance/run_cli_mcp_surface.py docs/plans/2026-07-05-full-paperclip-parity/acceptance/run_frontend_user_flow.py docs/plans/2026-07-05-full-paperclip-parity/codex/phase-prompts.md docs/plans/2026-07-05-full-paperclip-parity/reviews/review-checklist.md docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.md docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.jsonl' in /home/frankji/Projects/paperclip
 succeeded in 0ms:
docs/plans/2026-07-05-full-paperclip-parity/01-parity-audit-summary.md:7:1. The adversarial parity audit + focused retries produced an initial synthesized tally of `3 full / 48 partial / 98 missing / 24 divergent`. When converted into a row-level closure ledger, the focused retry item lists were more precise than their declared counters, yielding `3 full / 50 partial / 100 missing / 22 divergent` across 175 rows for the originally-audited hiring/governance/runtime surface.
docs/plans/2026-07-05-full-paperclip-parity/01-parity-audit-summary.md:8:2. The plan-design review then added the remaining original-V1 control-plane areas (goals/projects/issues, deployment/auth/onboarding, workspaces/routines/plugins, import/export/catalogs, observability/pipelines, shared contracts/docs, cutover, and the parity oracle itself) as additional owned rows. With those coverage rows the authoritative ledger is `3 full / 50 partial / 145 missing / 22 divergent` across 220 rows.
docs/plans/2026-07-05-full-paperclip-parity/01-parity-audit-summary.md:14:| Missing | 145 |
docs/plans/2026-07-05-full-paperclip-parity/01-parity-audit-summary.md:17:**Authoritative source:** `evidence/rewrite-gap-matrix.jsonl`. Workers close rows, not summary buckets.
docs/plans/2026-07-05-full-paperclip-parity/01-parity-audit-summary.md:83:## Highest-priority gaps
docs/plans/2026-07-05-full-paperclip-parity/README.md:3:> **Goal:** eliminate the verified parity gap between the current Rust/Supabase/Nuxt rewrite and original Paperclip V1 for agent hiring, governance, runtime, interfaces, and real work execution.
docs/plans/2026-07-05-full-paperclip-parity/README.md:5:> **Definition of success:** the rewrite reaches **zero known parity gaps** against `doc/SPEC-implementation.md` and the original TypeScript implementation evidence cited here. A working vertical slice is not enough.
docs/plans/2026-07-05-full-paperclip-parity/README.md:10:- [`01-parity-audit-summary.md`](01-parity-audit-summary.md) — corrected audit baseline: 3 full / 50 partial / 145 missing / 22 divergent across 220 rows (authoritative ledger: `evidence/rewrite-gap-matrix.jsonl`).
docs/plans/2026-07-05-full-paperclip-parity/README.md:20:- [`evidence/`](evidence/) — original-source map and rewrite gap matrix.
docs/plans/2026-07-05-full-paperclip-parity/README.md:24:1. Treat [`01-parity-audit-summary.md`](01-parity-audit-summary.md) as the starting contract; do not erase gaps without source-backed implementation and tests.
docs/plans/2026-07-05-full-paperclip-parity/README.md:27:4. Completion requires the final Codex parity review to produce no remaining missing/divergent/partial items unless explicitly marked out-of-scope by a revised product contract.
docs/plans/2026-07-05-full-paperclip-parity/README.md:44:The first audit synthesis reported `3 full / 48 partial / 98 missing / 24 divergent`. Converting the audit journals into a row-level closure ledger exposed two counter mismatches inside focused retry outputs, yielding `3 full / 50 partial / 100 missing / 22 divergent` across 175 rows for the originally-audited hiring/governance/runtime surface. The plan-design review then added owned rows for the remaining original-V1 control-plane areas (goals/projects/issues, deployment/auth/onboarding, workspaces/routines/plugins, import/export/catalogs, observability/pipelines, shared contracts/docs, cutover, and the parity oracle). The authoritative ledger is `evidence/rewrite-gap-matrix.jsonl`: `3 full / 50 partial / 145 missing / 22 divergent` across 220 rows, with every one of the 14 phases owning rows.
docs/plans/2026-07-05-full-paperclip-parity/reviews/review-checklist.md:6:- Matrix rows updated.
docs/plans/2026-07-05-full-paperclip-parity/reviews/review-checklist.md:10:- No unresolved owned gaps.
docs/plans/2026-07-05-full-paperclip-parity/reviews/review-checklist.md:18:- UI/CLI/MCP rows updated if affected.
docs/plans/2026-07-05-full-paperclip-parity/reviews/review-checklist.md:20:- Category-level acceptance cannot close a row-level gap; row-level matrix must be green or waived.
docs/plans/2026-07-05-full-paperclip-parity/reviews/review-checklist.md:26:gaps_missing=0 gaps_partial=0 gaps_divergent=0
docs/plans/2026-07-05-full-paperclip-parity/codex/phase-prompts.md:6:You are reviewing docs/plans/2026-07-05-full-paperclip-parity for completeness against original Paperclip V1. Read doc/SPEC-implementation.md, packages/shared/src/constants.ts, server/src/routes/agents.ts, server/src/routes/approvals.ts, server/src/routes/access.ts, server/src/services/approvals.ts, server/src/services/heartbeat.ts, ui/src/pages/NewAgent.tsx, cli/src/commands/client/agent.ts, and the plan directory. Identify any missing or under-specified parity area. Do not implement. Return must-fix plan gaps with file references.
docs/plans/2026-07-05-full-paperclip-parity/codex/phase-prompts.md:12:Before implementing, read PHASE_FILE plus `01-parity-audit-summary.md`, `04-execution-order.md`, `05-test-strategy.md`, `acceptance/full-parity-harness-spec.md`, `evidence/rewrite-gap-matrix.md`, and `reviews/review-checklist.md`. Implement only the rows owned by PHASE_FILE using strict TDD. First write failing tests that demonstrate the current rewrite does not match original Paperclip behavior. Then implement the minimal code/migrations to pass. Preserve: no public schema, no service-role key, no Supabase/OpenAI details client-side, OpenAI Responses API only. Update each owned matrix row with phase, acceptance_id, failing_test, passing_test, evidence_ref, and final_status. Run the phase checks and report exact output.
docs/plans/2026-07-05-full-paperclip-parity/codex/phase-prompts.md:18:Adversarially compare the final rewrite against original Paperclip V1 for agent hiring/governance/runtime/interfaces. Use docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.md as the checklist. Confirm gaps_missing=0 gaps_partial=0 gaps_divergent=0 or list blockers. Do not accept a vertical slice as full parity.
docs/plans/2026-07-05-full-paperclip-parity/codex/phase-prompts.md:24:A phase worker may not report "done" because a category test passed. Done means every owned row in `evidence/rewrite-gap-matrix.md` is `full` or has an approved `waived` product-contract reference. The worker must include the row IDs it closed and the commands that turned each red test green.
docs/plans/2026-07-05-full-paperclip-parity/acceptance/run_frontend_user_flow.py:2:"""Frontend board-flow parity harness (Phase 00 deliverable).
docs/plans/2026-07-05-full-paperclip-parity/acceptance/run_frontend_user_flow.py:4:RED scaffold owned by Phase 00. Phase 00 cannot close until this proves a
docs/plans/2026-07-05-full-paperclip-parity/acceptance/run_frontend_user_flow.py:6:Supabase/OpenAI detail reaching the browser. Exits non-zero while unimplemented.
docs/plans/2026-07-05-full-paperclip-parity/acceptance/run_cli_mcp_surface.py:2:"""CLI / MCP surface parity harness (Phase 00 deliverable).
docs/plans/2026-07-05-full-paperclip-parity/acceptance/run_cli_mcp_surface.py:4:RED scaffold owned by Phase 00. Phase 00 cannot close until this proves the
docs/plans/2026-07-05-full-paperclip-parity/acceptance/run_real_user_real_agent.py:2:"""Live real-user / real-agent acceptance harness (Phase 00 deliverable).
docs/plans/2026-07-05-full-paperclip-parity/acceptance/run_real_user_real_agent.py:4:This is a deliberately failing RED scaffold. It is owned by Phase 00 of the
docs/plans/2026-07-05-full-paperclip-parity/acceptance/run_real_user_real_agent.py:5:full-parity plan: Phase 00 cannot close until this script is implemented to
docs/plans/2026-07-05-full-paperclip-parity/acceptance/run_real_user_real_agent.py:18:Phase 00 worker replaces this body with the live driver and the matrix row
docs/plans/2026-07-05-full-paperclip-parity/acceptance/run_real_user_real_agent.py:25:    # RED state: Phase 00 must implement the live driver described above.
docs/plans/2026-07-05-full-paperclip-parity/acceptance/full-parity-harness-spec.md:15:3. Authz matrix: board/session/agent/viewer/low-trust/cross-company behavior.
docs/plans/2026-07-05-full-paperclip-parity/acceptance/full-parity-harness-spec.md:28:schema=pass authz=pass lifecycle=pass approvals=pass runtime=pass budgets=pass secrets=pass tasks=pass interfaces=pass live_openai=pass gaps_missing=0 gaps_partial=0 gaps_divergent=0
docs/plans/2026-07-05-full-paperclip-parity/acceptance/full-parity-harness-spec.md:32:## Row-level matrix proof
docs/plans/2026-07-05-full-paperclip-parity/acceptance/full-parity-harness-spec.md:34:The harness must read `evidence/rewrite-gap-matrix.md` (or its generated machine-readable companion) and assert every non-full audit row has:
docs/plans/2026-07-05-full-paperclip-parity/acceptance/full-parity-harness-spec.md:43:Allowed final statuses are only `full` or `waived`. Waived rows require a canonical product-contract reference and reviewer signoff. Category-level checks cannot override a row-level non-full status.
docs/plans/2026-07-05-full-paperclip-parity/acceptance/full-parity-harness-spec.md:50:gaps_missing=0 gaps_partial=0 gaps_divergent=0
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.jsonl:1:{"acceptance_id": "ACC-00-PARITY-ORACLE-AND-MATRIX-0001", "capability": "No-secret scanner implementation across all surfaces", "current_rewrite_evidence": "not yet covered by rewrite parity matrix; implementation evidence required", "domain": "Parity oracle", "evidence_ref": "to be filled when phase implements source-backed behavior", "failing_test": "acceptance/run_original_gap_diff.py::parity_oracle_no_secret_scanner_implementation_across_all_surfaces", "final_status": "unclosed", "initial_status": "missing", "original_behavior": "No-secret scanner implementation across all surfaces", "original_evidence": "No-secret scanner implementation across all surfaces", "passing_test": "python3 docs/plans/2026-07-05-full-paperclip-parity/acceptance/run_original_gap_diff.py", "phase": "phases/00-parity-oracle-and-matrix.md", "review_note": "Added after Codex review: plan-design added this V1 area as a phase, so it must own row-level closure rows, not just a filename.", "row_id": "parity-oracle-no-secret-scanner-implementation-across-all-surfaces", "verdict": "PLAN_DESIGN_EXTENSION", "waiver_ref": ""}
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.jsonl:2:{"acceptance_id": "ACC-00-PARITY-ORACLE-AND-MATRIX-0002", "capability": "run_cli_mcp_surface.py and run_frontend_user_flow.py harnesses", "current_rewrite_evidence": "not yet covered by rewrite parity matrix; implementation evidence required", "domain": "Parity oracle", "evidence_ref": "to be filled when phase implements source-backed behavior", "failing_test": "acceptance/run_original_gap_diff.py::parity_oracle_run_cli_mcp_surface_py_and_run_frontend_user_flow_py_harnesses", "final_status": "unclosed", "initial_status": "missing", "original_behavior": "run_cli_mcp_surface.py and run_frontend_user_flow.py harnesses", "original_evidence": "run_cli_mcp_surface.py and run_frontend_user_flow.py harnesses", "passing_test": "python3 docs/plans/2026-07-05-full-paperclip-parity/acceptance/run_original_gap_diff.py", "phase": "phases/00-parity-oracle-and-matrix.md", "review_note": "Added after Codex review: plan-design added this V1 area as a phase, so it must own row-level closure rows, not just a filename.", "row_id": "parity-oracle-run-cli-mcp-surface-py-and-run-frontend-user-flow-py-harnesses", "verdict": "PLAN_DESIGN_EXTENSION", "waiver_ref": ""}
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.jsonl:3:{"acceptance_id": "ACC-00-PARITY-ORACLE-AND-MATRIX-0003", "capability": "run_original_gap_diff.py exact gaps_* output contract", "current_rewrite_evidence": "not yet covered by rewrite parity matrix; implementation evidence required", "domain": "Parity oracle", "evidence_ref": "to be filled when phase implements source-backed behavior", "failing_test": "acceptance/run_original_gap_diff.py::parity_oracle_run_original_gap_diff_py_exact_gaps_output_contract", "final_status": "unclosed", "initial_status": "missing", "original_behavior": "run_original_gap_diff.py exact gaps_* output contract", "original_evidence": "run_original_gap_diff.py exact gaps_* output contract", "passing_test": "python3 docs/plans/2026-07-05-full-paperclip-parity/acceptance/run_original_gap_diff.py", "phase": "phases/00-parity-oracle-and-matrix.md", "review_note": "Added after Codex review: plan-design added this V1 area as a phase, so it must own row-level closure rows, not just a filename.", "row_id": "parity-oracle-run-original-gap-diff-py-exact-gaps-output-contract", "verdict": "PLAN_DESIGN_EXTENSION", "waiver_ref": ""}
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.jsonl:4:{"acceptance_id": "ACC-00-PARITY-ORACLE-AND-MATRIX-0004", "capability": "run_real_user_real_agent.py live harness implementation", "current_rewrite_evidence": "not yet covered by rewrite parity matrix; implementation evidence required", "domain": "Parity oracle", "evidence_ref": "to be filled when phase implements source-backed behavior", "failing_test": "acceptance/run_original_gap_diff.py::parity_oracle_run_real_user_real_agent_py_live_harness_implementation", "final_status": "unclosed", "initial_status": "missing", "original_behavior": "run_real_user_real_agent.py live harness implementation", "original_evidence": "run_real_user_real_agent.py live harness implementation", "passing_test": "python3 docs/plans/2026-07-05-full-paperclip-parity/acceptance/run_original_gap_diff.py", "phase": "phases/00-parity-oracle-and-matrix.md", "review_note": "Added after Codex review: plan-design added this V1 area as a phase, so it must own row-level closure rows, not just a filename.", "row_id": "parity-oracle-run-real-user-real-agent-py-live-harness-implementation", "verdict": "PLAN_DESIGN_EXTENSION", "waiver_ref": ""}
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.jsonl:6:{"acceptance_id": "ACC-01-SCHEMA-AUTHZ-PARITY-0006", "capability": "per-agent monthly budget accounting", "current_rewrite_evidence": "none found", "domain": "Data model & schema", "evidence_ref": "original_evidence + current_rewrite_evidence", "failing_test": "backend-rs/tests/authz_parity.rs::data_model_schema_agent_budget_fields", "final_status": "unclosed", "initial_status": "missing", "original_behavior": "", "original_evidence": "", "passing_test": "cd backend-rs && cargo test --test authz_parity", "phase": "phases/01-schema-authz-parity.md", "review_note": "Per-agent verdict correct: agents (0013:21-38) has no budget_monthly_cents/spent_monthly_cents. CAVEAT on the assessment note: its claim 'No budget accounting exists' is overstated \u2014 a COMPANY-level budget hard-stop exists in backend-rs/src/budgets.rs (Budget with monthly_limit_cents/spent_cents, BudgetStore, over-budget auto-pause) plus MCP paperclipGetBudget and a frontend BudgetView. That is an in-memory store, not agents schema columns, so the schema-parity status 'missing' for per-agent budget fields still stands.", "row_id": "data-model-schema-agent-budget-fields", "verdict": "CONFIRMED", "waiver_ref": ""}
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.jsonl:7:{"acceptance_id": "ACC-01-SCHEMA-AUTHZ-PARITY-0007", "capability": "agent_config_revisions audit table", "current_rewrite_evidence": "none found", "domain": "Data model & schema", "evidence_ref": "original_evidence + current_rewrite_evidence", "failing_test": "backend-rs/tests/authz_parity.rs::data_model_schema_agent_config_revisions_table", "final_status": "unclosed", "initial_status": "missing", "original_behavior": "", "original_evidence": "", "passing_test": "cd backend-rs && cargo test --test authz_parity", "phase": "phases/01-schema-authz-parity.md", "review_note": "Confirmed: grep for agent_config_revision found no table or config audit/rollback mechanism; agents have no editable config surface. Original 0006:1-13. Missing stands.", "row_id": "data-model-schema-agent-config-revisions-table", "verdict": "CONFIRMED", "waiver_ref": ""}
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.jsonl:8:{"acceptance_id": "ACC-01-SCHEMA-AUTHZ-PARITY-0008", "capability": "AGENT_ICON_NAMES enum", "current_rewrite_evidence": "none found", "domain": "Data model & schema", "evidence_ref": "original_evidence + current_rewrite_evidence", "failing_test": "backend-rs/tests/authz_parity.rs::data_model_schema_agent_icon_names", "final_status": "unclosed", "initial_status": "missing", "original_behavior": "", "original_evidence": "", "passing_test": "cd backend-rs && cargo test --test authz_parity", "phase": "phases/01-schema-authz-parity.md", "review_note": "Confirmed: no icon column in agents (0013:21-38) and grep for '\\bicon\\b' across the rewrite found no icon enum. Original constants.ts:84-127 is a 41-value set. Missing stands.", "row_id": "data-model-schema-agent-icon-names", "verdict": "CONFIRMED", "waiver_ref": ""}
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.jsonl:9:{"acceptance_id": "ACC-01-SCHEMA-AUTHZ-PARITY-0009", "capability": "AGENT_ROLES enum + labels", "current_rewrite_evidence": "supabase/migrations/paperclip/0013_agents_hiring.sql:26,86,105", "domain": "Data model & schema", "evidence_ref": "original_evidence + current_rewrite_evidence", "failing_test": "backend-rs/tests/authz_parity.rs::data_model_schema_agent_roles_enum", "final_status": "unclosed", "initial_status": "divergent", "original_behavior": "", "original_evidence": "", "passing_test": "cd backend-rs && cargo test --test authz_parity", "phase": "phases/01-schema-authz-parity.md", "review_note": "CHANGED missing->divergent for consistency and accuracy. The role column DOES exist as free-text with the correct default 'general' (0013:26) and is actively used by hire_agent (0013:86 p_role default 'general', 0013:105 coalesce(p_role,'general')). Its sibling item agent-adapter-types-enum was scored 'divergent' for the identical structural situation (free-text column replacing a curated enum). 'missing' under-credits: the field is present in weaker/unconstrained form (no enum, no AGENT_ROLE_LABELS map), which is divergence, not absence.", "row_id": "data-model-schema-agent-roles-enum", "verdict": "CHANGED", "waiver_ref": ""}
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.jsonl:13:{"acceptance_id": "ACC-01-SCHEMA-AUTHZ-PARITY-0013", "capability": "agents ORM adds icon/runtime_config/env/pause/error columns", "current_rewrite_evidence": "none found", "domain": "Data model & schema", "evidence_ref": "original_evidence + current_rewrite_evidence", "failing_test": "backend-rs/tests/authz_parity.rs::data_model_schema_agents_orm_extra_columns", "final_status": "unclosed", "initial_status": "missing", "original_behavior": "", "original_evidence": "", "passing_test": "cd backend-rs && cargo test --test authz_parity", "phase": "phases/01-schema-authz-parity.md", "review_note": "Confirmed absent: grep across supabase/migrations/paperclip, backend-rs/src, frontend-nuxt for icon, runtime_config, default_environment, pause_reason, paused_at, error_reason returned no hits. No environments concept exists in the rewrite. Missing stands.", "row_id": "data-model-schema-agents-orm-extra-columns", "verdict": "CONFIRMED", "waiver_ref": ""}
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.jsonl:16:{"acceptance_id": "ACC-01-SCHEMA-AUTHZ-PARITY-0016", "capability": "approval_comments table", "current_rewrite_evidence": "none found", "domain": "Data model & schema", "evidence_ref": "original_evidence + current_rewrite_evidence", "failing_test": "backend-rs/tests/authz_parity.rs::data_model_schema_approval_comments_table", "final_status": "unclosed", "initial_status": "missing", "original_behavior": "", "original_evidence": "", "passing_test": "cd backend-rs && cargo test --test authz_parity", "phase": "phases/01-schema-authz-parity.md", "review_note": "Confirmed: grep for approval_comment across the rewrite found no table or commenting surface. Original 0005:1-19. Missing stands.", "row_id": "data-model-schema-approval-comments-table", "verdict": "CONFIRMED", "waiver_ref": ""}
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.jsonl:23:{"acceptance_id": "ACC-01-SCHEMA-AUTHZ-PARITY-0023", "capability": "board-approval toggle in company validators", "current_rewrite_evidence": "none found", "domain": "Data model & schema", "evidence_ref": "original_evidence + current_rewrite_evidence", "failing_test": "backend-rs/tests/authz_parity.rs::data_model_schema_company_toggle_validators", "final_status": "unclosed", "initial_status": "missing", "original_behavior": "", "original_evidence": "", "passing_test": "cd backend-rs && cargo test --test authz_parity", "phase": "phases/01-schema-authz-parity.md", "review_note": "Confirmed: grep for update_company/updateCompany/require_board in backend-rs and frontend found no update-company RPC or portability export/import. The toggle is fixed at company-creation default and cannot be round-tripped. Missing stands.", "row_id": "data-model-schema-company-toggle-validators", "verdict": "CONFIRMED", "waiver_ref": ""}
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.jsonl:25:{"acceptance_id": "ACC-01-SCHEMA-AUTHZ-PARITY-0025", "capability": "Hiring modeled through approvals, not a hires table", "current_rewrite_evidence": "supabase/migrations/paperclip/0013_agents_hiring.sql:104-112,142-150", "domain": "Data model & schema", "evidence_ref": "original_evidence + current_rewrite_evidence", "failing_test": "backend-rs/tests/authz_parity.rs::data_model_schema_hire_modeled_via_approval", "final_status": "unclosed", "initial_status": "partial", "original_behavior": "", "original_evidence": "", "passing_test": "cd backend-rs && cargo test --test authz_parity", "phase": "phases/01-schema-authz-parity.md", "review_note": "Verified: no hires table; when v_require, hire_agent inserts a type 'hire_agent' approval referencing the agent (0013:107-111,145-149). But the agent row is created up front as pending_approval (0013:104-106) rather than materialized from payload on approval, and payload is only {name,role,model} (0013:109). No sourceIssueId(s) linkage (orig agent.ts:91-96). Approval-gated hire present but mechanically weaker = partial. Stands.", "row_id": "data-model-schema-hire-modeled-via-approval", "verdict": "CONFIRMED", "waiver_ref": ""}
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.jsonl:26:{"acceptance_id": "ACC-01-SCHEMA-AUTHZ-PARITY-0026", "capability": "issue_approvals junction table", "current_rewrite_evidence": "none found", "domain": "Data model & schema", "evidence_ref": "original_evidence + current_rewrite_evidence", "failing_test": "backend-rs/tests/authz_parity.rs::data_model_schema_issue_approvals_link_table", "final_status": "unclosed", "initial_status": "missing", "original_behavior": "", "original_evidence": "", "passing_test": "cd backend-rs && cargo test --test authz_parity", "phase": "phases/01-schema-authz-parity.md", "review_note": "Confirmed: grep for issue_approval found no hits; no issues table exists in the rewrite so no junction is possible. Original 0006:15-23. Missing stands.", "row_id": "data-model-schema-issue-approvals-link-table", "verdict": "CONFIRMED", "waiver_ref": ""}
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.jsonl:27:{"acceptance_id": "ACC-01-SCHEMA-AUTHZ-PARITY-0027", "capability": "PAUSE_REASONS enum", "current_rewrite_evidence": "none found", "domain": "Data model & schema", "evidence_ref": "original_evidence + current_rewrite_evidence", "failing_test": "backend-rs/tests/authz_parity.rs::data_model_schema_pause_reasons_enum", "final_status": "unclosed", "initial_status": "missing", "original_behavior": "", "original_evidence": "", "passing_test": "cd backend-rs && cargo test --test authz_parity", "phase": "phases/01-schema-authz-parity.md", "review_note": "Confirmed: no pause_reason column on agents/companies and no PAUSE_REASONS constraint anywhere (grep hit nothing). Status enum lacks a paused-reason concept. Missing stands.", "row_id": "data-model-schema-pause-reasons-enum", "verdict": "CONFIRMED", "waiver_ref": ""}
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.jsonl:28:{"acceptance_id": "ACC-01-SCHEMA-AUTHZ-PARITY-0028", "capability": "updateAgentPermissionsSchema", "current_rewrite_evidence": "none found", "domain": "Data model & schema", "evidence_ref": "original_evidence + current_rewrite_evidence", "failing_test": "backend-rs/tests/authz_parity.rs::data_model_schema_update_agent_permissions_validator", "final_status": "unclosed", "initial_status": "missing", "original_behavior": "", "original_evidence": "", "passing_test": "cd backend-rs && cargo test --test authz_parity", "phase": "phases/01-schema-authz-parity.md", "review_note": "Confirmed: grep for canAssignTasks/canCreateSkills/updateAgentPermissions found no hits in the rewrite. can_create_agents is set once at hire (0013:102 session board-only; 0013:143 always false via key) with no later mutation path. Missing stands.", "row_id": "data-model-schema-update-agent-permissions-validator", "verdict": "CONFIRMED", "waiver_ref": ""}
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.jsonl:29:{"acceptance_id": "ACC-01-SCHEMA-AUTHZ-PARITY-0029", "capability": "updateAgentSchema restrictions", "current_rewrite_evidence": "none found", "domain": "Data model & schema", "evidence_ref": "original_evidence + current_rewrite_evidence", "failing_test": "backend-rs/tests/authz_parity.rs::data_model_schema_update_agent_validator", "final_status": "unclosed", "initial_status": "missing", "original_behavior": "", "original_evidence": "", "passing_test": "cd backend-rs && cargo test --test authz_parity", "phase": "phases/01-schema-authz-parity.md", "review_note": "Confirmed: grep for update_agent/updateAgent across migrations, backend-rs/src, frontend-nuxt found no general update RPC/endpoint. Agents only hired and transitioned active/archived via decide_approval (0013:164-176). No replaceAdapterConfig/status/spentMonthlyCents mutation surface. Missing stands.", "row_id": "data-model-schema-update-agent-validator", "verdict": "CONFIRMED", "waiver_ref": ""}
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.jsonl:31:{"acceptance_id": "ACC-01-SCHEMA-AUTHZ-PARITY-0031", "capability": "Default tasks:assign grant on every hire", "current_rewrite_evidence": "supabase/migrations/paperclip/0013_agents_hiring.sql:104-113", "domain": "Hiring flow & side effects", "evidence_ref": "original_evidence + current_rewrite_evidence", "failing_test": "backend-rs/tests/authz_parity.rs::hiring_flow_side_effects_default_task_assign_grant", "final_status": "unclosed", "initial_status": "missing", "original_behavior": "", "original_evidence": "", "passing_test": "cd backend-rs && cargo test --test authz_parity", "phase": "phases/01-schema-authz-parity.md", "review_note": "Verified: no tasks concept (rewrite has 'jobs' not 'tasks'), no membership/principal-permission grant side effect on hire. hire_agent inserts only the agent (+approval). Missing stands.", "row_id": "hiring-flow-side-effects-default-task-assign-grant", "verdict": "CONFIRMED", "waiver_ref": ""}
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.jsonl:32:{"acceptance_id": "ACC-01-SCHEMA-AUTHZ-PARITY-0032", "capability": "Direct POST /agents blocked when approvals required", "current_rewrite_evidence": "backend-rs/src/supabase/routes.rs:51-53; supabase/migrations/paperclip/0013_agents_hiring.sql:103,141", "domain": "Hiring flow & side effects", "evidence_ref": "original_evidence + current_rewrite_evidence", "failing_test": "backend-rs/tests/authz_parity.rs::hiring_flow_side_effects_direct_create_blocked_when_approval", "final_status": "unclosed", "initial_status": "missing", "original_behavior": "", "original_evidence": "", "passing_test": "cd backend-rs && cargo test --test authz_parity", "phase": "phases/01-schema-authz-parity.md", "review_note": "Verified: there is no separate non-hire direct-create endpoint \u2014 POST .../agents IS the hire path and is itself approval-gated (0013:103,141). So no bypass exists, but the specific conflict-throwing guard behavior described is genuinely absent (the invariant is upheld structurally, not by an explicit guard). Considered flipping to divergent, but there is no alternative implementation of a 'block' to point at \u2014 only the absence of a bypass \u2014 so 'missing' with this nuance is the honest label. Stands.", "row_id": "hiring-flow-side-effects-direct-create-blocked-when-approval", "verdict": "CONFIRMED", "waiver_ref": ""}
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.jsonl:33:{"acceptance_id": "ACC-01-SCHEMA-AUTHZ-PARITY-0033", "capability": "agent.hire_created + approval.created activity logs + telemetry", "current_rewrite_evidence": "supabase/migrations/paperclip/0013_agents_hiring.sql:104-113,142-151; 0011_companies_jobs.sql:6-49", "domain": "Hiring flow & side effects", "evidence_ref": "original_evidence + current_rewrite_evidence", "failing_test": "backend-rs/tests/authz_parity.rs::hiring_flow_side_effects_hire_activity_log", "final_status": "unclosed", "initial_status": "missing", "original_behavior": "", "original_evidence": "", "passing_test": "cd backend-rs && cargo test --test authz_parity", "phase": "phases/01-schema-authz-parity.md", "review_note": "Verified: no events/activity table exists in the entire paperclip schema (0011 = companies+jobs, 0013 = agents+approvals; grep of all migrations shows no events table), and hire_agent/hire_agent_with_key/decide_approval write only to agents+approvals. No telemetry emission in jobs/mod.rs. Missing stands.", "row_id": "hiring-flow-side-effects-hire-activity-log", "verdict": "CONFIRMED", "waiver_ref": ""}
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.jsonl:38:{"acceptance_id": "ACC-01-SCHEMA-AUTHZ-PARITY-0038", "capability": "Join-request approval also triggers hire hook", "current_rewrite_evidence": "supabase/migrations/paperclip/0013_agents_hiring.sql:13-15,154-206", "domain": "Hiring flow & side effects", "evidence_ref": "original_evidence + current_rewrite_evidence", "failing_test": "backend-rs/tests/authz_parity.rs::hiring_flow_side_effects_join_request_hire_hook", "final_status": "unclosed", "initial_status": "missing", "original_behavior": "", "original_evidence": "", "passing_test": "cd backend-rs && cargo test --test authz_parity", "phase": "phases/01-schema-authz-parity.md", "review_note": "Verified: no join-request flow (approval_type enum is only 'hire_agent','approve_ceo_strategy', 0013:14) and no hire hook. Missing stands.", "row_id": "hiring-flow-side-effects-join-request-hire-hook", "verdict": "CONFIRMED", "waiver_ref": ""}
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.jsonl:39:{"acceptance_id": "ACC-01-SCHEMA-AUTHZ-PARITY-0039", "capability": "Reject legacy promptTemplate on new hires", "current_rewrite_evidence": "supabase/migrations/paperclip/0013_agents_hiring.sql:84-151 (no prompt fields)", "domain": "Hiring flow & side effects", "evidence_ref": "original_evidence + current_rewrite_evidence", "failing_test": "backend-rs/tests/authz_parity.rs::hiring_flow_side_effects_legacy_prompt_guard", "final_status": "unclosed", "initial_status": "missing", "original_behavior": "", "original_evidence": "", "passing_test": "cd backend-rs && cargo test --test authz_parity", "phase": "phases/01-schema-authz-parity.md", "review_note": "Verified: no promptTemplate/bootstrapPromptTemplate/instructionsBundle/adapterConfig concept exists anywhere in the rewrite hiring path (0013, jobs/mod.rs, routes.rs). Nothing to guard. Missing stands.", "row_id": "hiring-flow-side-effects-legacy-prompt-guard", "verdict": "CONFIRMED", "waiver_ref": ""}
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.jsonl:40:{"acceptance_id": "ACC-01-SCHEMA-AUTHZ-PARITY-0040", "capability": "Plugin-managed agent creation mirrors the hire+approval flow", "current_rewrite_evidence": "supabase/migrations/paperclip/0013_agents_hiring.sql:84-151", "domain": "Hiring flow & side effects", "evidence_ref": "original_evidence + current_rewrite_evidence", "failing_test": "backend-rs/tests/authz_parity.rs::hiring_flow_side_effects_plugin_managed_agent_hire", "final_status": "unclosed", "initial_status": "missing", "original_behavior": "", "original_evidence": "", "passing_test": "cd backend-rs && cargo test --test authz_parity", "phase": "phases/01-schema-authz-parity.md", "review_note": "Verified: no plugin subsystem and no managed-agent path exist in the rewrite. Missing stands.", "row_id": "hiring-flow-side-effects-plugin-managed-agent-hire", "verdict": "CONFIRMED", "waiver_ref": ""}
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.jsonl:41:{"acceptance_id": "ACC-01-SCHEMA-AUTHZ-PARITY-0041", "capability": "Source issues linked to approval", "current_rewrite_evidence": "supabase/migrations/paperclip/0013_agents_hiring.sql:84-151", "domain": "Hiring flow & side effects", "evidence_ref": "original_evidence + current_rewrite_evidence", "failing_test": "backend-rs/tests/authz_parity.rs::hiring_flow_side_effects_source_issue_linking", "final_status": "unclosed", "initial_status": "missing", "original_behavior": "", "original_evidence": "", "passing_test": "cd backend-rs && cargo test --test authz_parity", "phase": "phases/01-schema-authz-parity.md", "review_note": "Verified: no source-issue concept, no issue table, no linkManyForApproval; hire payload has no issue fields. Missing stands.", "row_id": "hiring-flow-side-effects-source-issue-linking", "verdict": "CONFIRMED", "waiver_ref": ""}
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.jsonl:42:{"acceptance_id": "ACC-01-SCHEMA-AUTHZ-PARITY-0042", "capability": "Activity/audit side effects for permission and hiring actions", "current_rewrite_evidence": "Audited rewrite migrations define no activity log table in 0011-0013 and the Rust service calls RPCs without recording audit events in /home/frankji/Projects/paperclip/backend-rs/src/jobs/mod.rs:231 and /home/frankji/Projects/paperclip/backend-rs/src/jobs/mod.rs:316.", "domain": "Permissions, board membership & access", "evidence_ref": "original_evidence + current_rewrite_evidence", "failing_test": "backend-rs/tests/authz_parity.rs::permissions_board_membership_access_activity_audit_side_effects_for_pe", "final_status": "unclosed", "initial_status": "missing", "original_behavior": "Missing. This matters because original governance treats activity log writes as part of the permission side effect contract.", "original_evidence": "Original mutating access/agent routes write activity, for example `agent.hire_created` and `approval.created` at /home/frankji/Projects/paperclip/server/src/routes/agents.ts:2328 and /home/frankji/Projects/paperclip/server/src/routes/agents.ts:2357, `agent.permissions_updated` at /home/frankji/Projects/paperclip/server/src/routes/agents.ts:2535, and `company_member.permissions_updated` at /home/frankji/Projects/paperclip/server/src/routes/access.ts:4696.", "passing_test": "cd backend-rs && cargo test --test authz_parity", "phase": "phases/01-schema-authz-parity.md", "review_note": "Missing. This matters because original governance treats activity log writes as part of the permission side effect contract.", "row_id": "permissions-board-membership-access-activity-audit-side-effects-for-pe", "verdict": "CONFIRMED", "waiver_ref": ""}
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.jsonl:45:{"acceptance_id": "ACC-01-SCHEMA-AUTHZ-PARITY-0045", "capability": "Agent configuration read/update authorization and redaction", "current_rewrite_evidence": "Rewrite agent model has only `adapter_type` and `model`, not adapter_config/runtime_config, in /home/frankji/Projects/paperclip/supabase/migrations/paperclip/0013_agents_hiring.sql:30. Backend exposes list/hire agents only, not config read/update, at /home/frankji/Projects/paperclip/backend-rs/src/supabase/routes.rs:51 and /home/frankji/Projects/paperclip/backend-rs/src/jobs/mod.rs:231.", "domain": "Permissions, board membership & access", "evidence_ref": "original_evidence + current_rewrite_evidence", "failing_test": "backend-rs/tests/authz_parity.rs::permissions_board_membership_access_agent_configuration_read_update_au", "final_status": "unclosed", "initial_status": "missing", "original_behavior": "Missing for this domain. The rewrite intentionally simplified agents, but it does not preserve original agent config permission behavior, secret-redacted config reads, config revisions, or update guards.", "original_evidence": "Original `agent_config:read`/`agent_config:update` map to `agents:create` at /home/frankji/Projects/paperclip/server/src/services/authorization.ts:116. Board members can read redacted configs; agents require explicit `agents:create` or legacy canCreateAgents in /home/frankji/Projects/paperclip/server/src/routes/agents.ts:686. Redaction is done in `redactAgentConfiguration` at /home/frankji/Projects/paperclip/server/src/routes/agents.ts:1490.", "passing_test": "cd backend-rs && cargo test --test authz_parity", "phase": "phases/01-schema-authz-parity.md", "review_note": "Missing for this domain. The rewrite intentionally simplified agents, but it does not preserve original agent config permission behavior, secret-redacted config reads, config revisions, or update guards.", "row_id": "permissions-board-membership-access-agent-configuration-read-update-au", "verdict": "CONFIRMED", "waiver_ref": ""}
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.jsonl:46:{"acceptance_id": "ACC-01-SCHEMA-AUTHZ-PARITY-0046", "capability": "Approval decision authority for hires", "current_rewrite_evidence": "Rewrite `decide_approval` is owner/admin session-only and sets hired agent to active or archived at /home/frankji/Projects/paperclip/supabase/migrations/paperclip/0013_agents_hiring.sql:153. `decide_approval_with_key` denies agent-subject keys and requires the key minter owner/admin at /home/frankji/Projects/paperclip/supabase/migrations/paperclip/0013_agents_hiring.sql:179. Rust exposes POST `/api/paperclip/approvals/:approvalId/decide` at /home/frankji/Projects/paperclip/backend-rs/src/supabase/routes.rs:59.", "domain": "Permissions, board membership & access", "evidence_ref": "original_evidence + current_rewrite_evidence", "failing_test": "backend-rs/tests/authz_parity.rs::permissions_board_membership_access_approval_decision_authority_for_hi", "final_status": "unclosed", "initial_status": "partial", "original_behavior": "Partial. The safety property that agents cannot decide approvals is present, but authority is hard-coded owner/admin rather than original `joins:approve`/board grant model, and original side effects such as activity log, issue links, notifyHireApproved, and decision notes are absent.", "original_evidence": "Original hire approval rows are created in `/agent-hires`, with requested_by fields and linked issue approvals, then decision logging/notification side effects occur elsewhere; creation evidence at /home/frankji/Projects/paperclip/server/src/routes/agents.ts:2284 and activity at /home/frankji/Projects/paperclip/server/src/routes/agents.ts:2357. Original join request approval also forbids non-`joins:approve` callers at /home/frankji/Projects/paperclip/server/src/routes/access.ts:4121.", "passing_test": "cd backend-rs && cargo test --test authz_parity", "phase": "phases/01-schema-authz-parity.md", "review_note": "Partial. The safety property that agents cannot decide approvals is present, but authority is hard-coded owner/admin rather than original `joins:approve`/board grant model, and original side effects such as activity log, issue links, notifyHireApproved, and decision notes are absent.", "row_id": "permissions-board-membership-access-approval-decision-authority-for-hi", "verdict": "CONFIRMED", "waiver_ref": ""}
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.jsonl:49:{"acceptance_id": "ACC-01-SCHEMA-AUTHZ-PARITY-0049", "capability": "Company member management and custom permission grants", "current_rewrite_evidence": "Rewrite has team member primitives in earlier migrations, with owner/admin role checks and last-owner protection at /home/frankji/Projects/paperclip/supabase/migrations/paperclip/0004_rpcs.sql:57 and /home/frankji/Projects/paperclip/supabase/migrations/paperclip/0009_authz_hardening.sql:66, but the audited Rust/Nuxt surfaces expose no member or custom grant management endpoints; routes are listed at /home/frankji/Projects/paperclip/backend-rs/src/supabase/routes.rs:36.", "domain": "Permissions, board membership & access", "evidence_ref": "original_evidence + current_rewrite_evidence", "failing_test": "backend-rs/tests/authz_parity.rs::permissions_board_membership_access_company_member_management_and_cust", "final_status": "unclosed", "initial_status": "missing", "original_behavior": "Missing in the audited rewrite surface. Some lower-level team membership exists, but it is not the original company membership/grants model and is not exposed in the requested backend/frontend files.", "original_evidence": "Original access routes require `users:manage_permissions` for member list/update/archive/permissions at /home/frankji/Projects/paperclip/server/src/routes/access.ts:4394, /home/frankji/Projects/paperclip/server/src/routes/access.ts:4414, and /home/frankji/Projects/paperclip/server/src/routes/access.ts:4679. Original protects self-removal, instance admins, owners/admins, role rank, and last active owner in /home/frankji/Projects/paperclip/server/src/routes/access.ts:1256 and /home/frankji/Projects/paperclip/server/src/routes/access.ts:4454.", "passing_test": "cd backend-rs && cargo test --test authz_parity", "phase": "phases/01-schema-authz-parity.md", "review_note": "Missing in the audited rewrite surface. Some lower-level team membership exists, but it is not the original company membership/grants model and is not exposed in the requested backend/frontend files.", "row_id": "permissions-board-membership-access-company-member-management-and-cust", "verdict": "CONFIRMED", "waiver_ref": ""}
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.jsonl:53:{"acceptance_id": "ACC-01-SCHEMA-AUTHZ-PARITY-0053", "capability": "Original permission key inventory", "current_rewrite_evidence": "Rewrite API-key scopes are a separate coarse set: `companies:write`, `companies:read`, `jobs:write`, `jobs:read`, `*` in /home/frankji/Projects/paperclip/supabase/migrations/paperclip/0012_api_key_scopes.sql:1. Hiring adds `agents:write`/`agents:read` checks at /home/frankji/Projects/paperclip/supabase/migrations/paperclip/0013_agents_hiring.sql:124 and /home/frankji/Projects/paperclip/supabase/migrations/paperclip/0013_agents_hiring.sql:214. No rewrite table/RPC equivalent of `principal_permission_grants` or original permission keys appears in the audited files.", "domain": "Permissions, board membership & access", "evidence_ref": "original_evidence + current_rewrite_evidence", "failing_test": "backend-rs/tests/authz_parity.rs::permissions_board_membership_access_original_permission_key_inventory", "final_status": "unclosed", "initial_status": "missing", "original_behavior": "Missing parity. The rewrite enforces a few endpoint scopes but does not preserve the original policy key space or agent policy fields beyond snake_case `can_create_agents`.", "original_evidence": "Original explicit keys: `agents:create`, `skills:create`, `environments:manage`, `users:invite`, `users:manage_permissions`, `tasks:assign`, `tasks:assign_scope`, `tasks:manage_active_checkouts`, `pipelines:write`, `joins:approve` at /home/frankji/Projects/paperclip/packages/shared/src/constants.ts:786. Agent policy fields include `canCreateAgents`, `canCreateSkills`, `trustPreset`, `authorizationPolicy` at /home/frankji/Projects/paperclip/packages/shared/src/validators/agent.ts:13 and /home/frankji/Projects/paperclip/packages/shared/src/types/agent.ts:19.", "passing_test": "cd backend-rs && cargo test --test authz_parity", "phase": "phases/01-schema-authz-parity.md", "review_note": "Missing parity. The rewrite enforces a few endpoint scopes but does not preserve the original policy key space or agent policy fields beyond snake_case `can_create_agents`.", "row_id": "permissions-board-membership-access-original-permission-key-inventory", "verdict": "CONFIRMED", "waiver_ref": ""}
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.jsonl:54:{"acceptance_id": "ACC-01-SCHEMA-AUTHZ-PARITY-0054", "capability": "Permission update endpoint for agents (`canCreateAgents`, `canCreateSkills`, `canAssignTasks`)", "current_rewrite_evidence": "Rewrite has no endpoint or RPC to update agent permissions in audited files. `p_can_create_agents` is only a create-time session parameter and is not exposed by Rust route payload at /home/frankji/Projects/paperclip/supabase/migrations/paperclip/0013_agents_hiring.sql:85 and /home/frankji/Projects/paperclip/backend-rs/src/supabase/routes.rs:191.", "domain": "Permissions, board membership & access", "evidence_ref": "original_evidence + current_rewrite_evidence", "failing_test": "backend-rs/tests/authz_parity.rs::permissions_board_membership_access_permission_update_endpoint_for_age", "final_status": "unclosed", "initial_status": "missing", "original_behavior": "Missing. Agent permission management after hire is absent; `canCreateSkills`, `canAssignTasks`, trust policy, and authorization policy have no rewrite equivalent.", "original_evidence": "Original `/agents/:id/permissions` accepts `canCreateAgents`, optional `canCreateSkills`, `canAssignTasks`, `trustPreset`, and `authorizationPolicy` at /home/frankji/Projects/paperclip/packages/shared/src/validators/agent.ts:200. CEO agents or board with manage-agent authority may update permissions; route syncs effective `tasks:assign` grant and logs details at /home/frankji/Projects/paperclip/server/src/routes/agents.ts:2493 and /home/frankji/Projects/paperclip/server/src/routes/agents.ts:2522.", "passing_test": "cd backend-rs && cargo test --test authz_parity", "phase": "phases/01-schema-authz-parity.md", "review_note": "Missing. Agent permission management after hire is absent; `canCreateSkills`, `canAssignTasks`, trust policy, and authorization policy have no rewrite equivalent.", "row_id": "permissions-board-membership-access-permission-update-endpoint-for-age", "verdict": "CONFIRMED", "waiver_ref": ""}
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.jsonl:55:{"acceptance_id": "ACC-01-SCHEMA-AUTHZ-PARITY-0055", "capability": "Protected/private agent authorization policies and low-trust boundaries", "current_rewrite_evidence": "Rewrite `agents.permissions` stores only JSON default `{\"can_create_agents\": false}` and no trust/authorization policy schema at /home/frankji/Projects/paperclip/supabase/migrations/paperclip/0013_agents_hiring.sql:32. Rust routes do not evaluate trust policies in /home/frankji/Projects/paperclip/backend-rs/src/supabase/routes.rs:36.", "domain": "Permissions, board membership & access", "evidence_ref": "original_evidence + current_rewrite_evidence", "failing_test": "backend-rs/tests/authz_parity.rs::permissions_board_membership_access_protected_private_agent_authorizat", "final_status": "unclosed", "initial_status": "missing", "original_behavior": "Missing. The rewrite has no equivalent of original low-trust containment or protected assignment policies.", "original_evidence": "Original agent permissions include `trustPreset` and `authorizationPolicy` at /home/frankji/Projects/paperclip/packages/shared/src/validators/agent.ts:13. Assignment policy evaluates private/protected/approval-required settings at /home/frankji/Projects/paperclip/server/src/services/authorization.ts:222. Low-trust denies broad capabilities and restricts agent/project/issue access at /home/frankji/Projects/paperclip/server/src/services/authorization.ts:708.", "passing_test": "cd backend-rs && cargo test --test authz_parity", "phase": "phases/01-schema-authz-parity.md", "review_note": "Missing. The rewrite has no equivalent of original low-trust containment or protected assignment policies.", "row_id": "permissions-board-membership-access-protected-private-agent-authorizat", "verdict": "CONFIRMED", "waiver_ref": ""}
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.jsonl:59:{"acceptance_id": "ACC-01-SCHEMA-AUTHZ-PARITY-0059", "capability": "Task assignment permissions (`tasks:assign`, scoped assignment, simple default)", "current_rewrite_evidence": "Rewrite audited files have no issues/tasks table or task assignment RPC. The only job operations are company-scoped run/list/complete/fail in /home/frankji/Projects/paperclip/backend-rs/src/jobs/mod.rs:103 and /home/frankji/Projects/paperclip/backend-rs/src/jobs/mod.rs:211; API-key job scopes are `jobs:*` in /home/frankji/Projects/paperclip/supabase/migrations/paperclip/0012_api_key_scopes.sql:38.", "domain": "Permissions, board membership & access", "evidence_ref": "original_evidence + current_rewrite_evidence", "failing_test": "backend-rs/tests/authz_parity.rs::permissions_board_membership_access_task_assignment_permissions_tasks_", "final_status": "unclosed", "initial_status": "missing", "original_behavior": "Missing. Jobs are not task assignment, no scoped assignment constraints, no manager subtree assignment policy, and no default task assignment grant side effect.", "original_evidence": "Original grants include `tasks:assign` and `tasks:assign_scope` at /home/frankji/Projects/paperclip/packages/shared/src/constants.ts:792. Original authorization evaluates project/agent/subtree scope constraints for `tasks:assign_scope` at /home/frankji/Projects/paperclip/server/src/services/authorization.ts:325 and tries broad then scoped assignment grants at /home/frankji/Projects/paperclip/server/src/services/authorization.ts:1099. Newly created agents get membership plus `tasks:assign` at /home/frankji/Projects/paperclip/server/src/routes/agents.ts:640.", "passing_test": "cd backend-rs && cargo test --test authz_parity", "phase": "phases/01-schema-authz-parity.md", "review_note": "Missing. Jobs are not task assignment, no scoped assignment constraints, no manager subtree assignment policy, and no default task assignment grant side effect.", "row_id": "permissions-board-membership-access-task-assignment-permissions-tasks-", "verdict": "CONFIRMED", "waiver_ref": ""}
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.jsonl:60:{"acceptance_id": "ACC-02-GOALS-PROJECTS-ISSUES-WORK-PRODUCTS-0060", "capability": "Issue comments as the built-in communication model", "current_rewrite_evidence": "not yet covered by rewrite parity matrix; implementation evidence required", "domain": "Comments", "evidence_ref": "to be filled when phase implements source-backed behavior", "failing_test": "backend-rs/tests/tasks_parity.rs::comments_issue_comments_as_the_built_in_communication_model", "final_status": "unclosed", "initial_status": "missing", "original_behavior": "Issue comments as the built-in communication model", "original_evidence": "Issue comments as the built-in communication model", "passing_test": "cd backend-rs && cargo test --test tasks_parity", "phase": "phases/02-goals-projects-issues-work-products.md", "review_note": "Added after Codex review: plan-design added this V1 area as a phase, so it must own row-level closure rows, not just a filename.", "row_id": "comments-issue-comments-as-the-built-in-communication-model", "verdict": "PLAN_DESIGN_EXTENSION", "waiver_ref": ""}
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.jsonl:61:{"acceptance_id": "ACC-02-GOALS-PROJECTS-ISSUES-WORK-PRODUCTS-0061", "capability": "Issue documents and attachments", "current_rewrite_evidence": "not yet covered by rewrite parity matrix; implementation evidence required", "domain": "Documents", "evidence_ref": "to be filled when phase implements source-backed behavior", "failing_test": "backend-rs/tests/tasks_parity.rs::documents_issue_documents_and_attachments", "final_status": "unclosed", "initial_status": "missing", "original_behavior": "Issue documents and attachments", "original_evidence": "Issue documents and attachments", "passing_test": "cd backend-rs && cargo test --test tasks_parity", "phase": "phases/02-goals-projects-issues-work-products.md", "review_note": "Added after Codex review: plan-design added this V1 area as a phase, so it must own row-level closure rows, not just a filename.", "row_id": "documents-issue-documents-and-attachments", "verdict": "PLAN_DESIGN_EXTENSION", "waiver_ref": ""}
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.jsonl:62:{"acceptance_id": "ACC-02-GOALS-PROJECTS-ISSUES-WORK-PRODUCTS-0062", "capability": "Company mission and goal hierarchy company/team/agent/task", "current_rewrite_evidence": "not yet covered by rewrite parity matrix; implementation evidence required", "domain": "Goals hierarchy", "evidence_ref": "to be filled when phase implements source-backed behavior", "failing_test": "backend-rs/tests/tasks_parity.rs::goals_hierarchy_company_mission_and_goal_hierarchy_company_team_agent_task", "final_status": "unclosed", "initial_status": "missing", "original_behavior": "Company mission and goal hierarchy company/team/agent/task", "original_evidence": "Company mission and goal hierarchy company/team/agent/task", "passing_test": "cd backend-rs && cargo test --test tasks_parity", "phase": "phases/02-goals-projects-issues-work-products.md", "review_note": "Added after Codex review: plan-design added this V1 area as a phase, so it must own row-level closure rows, not just a filename.", "row_id": "goals-hierarchy-company-mission-and-goal-hierarchy-company-team-agent-task", "verdict": "PLAN_DESIGN_EXTENSION", "waiver_ref": ""}
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.jsonl:63:{"acceptance_id": "ACC-02-GOALS-PROJECTS-ISSUES-WORK-PRODUCTS-0063", "capability": "Goal CRUD, parent linkage, owner agent assignment", "current_rewrite_evidence": "not yet covered by rewrite parity matrix; implementation evidence required", "domain": "Goals hierarchy", "evidence_ref": "to be filled when phase implements source-backed behavior", "failing_test": "backend-rs/tests/tasks_parity.rs::goals_hierarchy_goal_crud_parent_linkage_owner_agent_assignment", "final_status": "unclosed", "initial_status": "missing", "original_behavior": "Goal CRUD, parent linkage, owner agent assignment", "original_evidence": "Goal CRUD, parent linkage, owner agent assignment", "passing_test": "cd backend-rs && cargo test --test tasks_parity", "phase": "phases/02-goals-projects-issues-work-products.md", "review_note": "Added after Codex review: plan-design added this V1 area as a phase, so it must own row-level closure rows, not just a filename.", "row_id": "goals-hierarchy-goal-crud-parent-linkage-owner-agent-assignment", "verdict": "PLAN_DESIGN_EXTENSION", "waiver_ref": ""}
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.jsonl:64:{"acceptance_id": "ACC-02-GOALS-PROJECTS-ISSUES-WORK-PRODUCTS-0064", "capability": "Blockers, recovery issues, review/approval handoffs", "current_rewrite_evidence": "not yet covered by rewrite parity matrix; implementation evidence required", "domain": "Issues", "evidence_ref": "to be filled when phase implements source-backed behavior", "failing_test": "backend-rs/tests/tasks_parity.rs::issues_blockers_recovery_issues_review_approval_handoffs", "final_status": "unclosed", "initial_status": "missing", "original_behavior": "Blockers, recovery issues, review/approval handoffs", "original_evidence": "Blockers, recovery issues, review/approval handoffs", "passing_test": "cd backend-rs && cargo test --test tasks_parity", "phase": "phases/02-goals-projects-issues-work-products.md", "review_note": "Added after Codex review: plan-design added this V1 area as a phase, so it must own row-level closure rows, not just a filename.", "row_id": "issues-blockers-recovery-issues-review-approval-handoffs", "verdict": "PLAN_DESIGN_EXTENSION", "waiver_ref": ""}
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.jsonl:65:{"acceptance_id": "ACC-02-GOALS-PROJECTS-ISSUES-WORK-PRODUCTS-0065", "capability": "Issue stable identifiers and parent/sub-issue tree", "current_rewrite_evidence": "not yet covered by rewrite parity matrix; implementation evidence required", "domain": "Issues", "evidence_ref": "to be filled when phase implements source-backed behavior", "failing_test": "backend-rs/tests/tasks_parity.rs::issues_issue_stable_identifiers_and_parent_sub_issue_tree", "final_status": "unclosed", "initial_status": "missing", "original_behavior": "Issue stable identifiers and parent/sub-issue tree", "original_evidence": "Issue stable identifiers and parent/sub-issue tree", "passing_test": "cd backend-rs && cargo test --test tasks_parity", "phase": "phases/02-goals-projects-issues-work-products.md", "review_note": "Added after Codex review: plan-design added this V1 area as a phase, so it must own row-level closure rows, not just a filename.", "row_id": "issues-issue-stable-identifiers-and-parent-sub-issue-tree", "verdict": "PLAN_DESIGN_EXTENSION", "waiver_ref": ""}
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.jsonl:66:{"acceptance_id": "ACC-02-GOALS-PROJECTS-ISSUES-WORK-PRODUCTS-0066", "capability": "Single-assignee and atomic checkout for in_progress", "current_rewrite_evidence": "not yet covered by rewrite parity matrix; implementation evidence required", "domain": "Issues", "evidence_ref": "to be filled when phase implements source-backed behavior", "failing_test": "backend-rs/tests/tasks_parity.rs::issues_single_assignee_and_atomic_checkout_for_in_progress", "final_status": "unclosed", "initial_status": "missing", "original_behavior": "Single-assignee and atomic checkout for in_progress", "original_evidence": "Single-assignee and atomic checkout for in_progress", "passing_test": "cd backend-rs && cargo test --test tasks_parity", "phase": "phases/02-goals-projects-issues-work-products.md", "review_note": "Added after Codex review: plan-design added this V1 area as a phase, so it must own row-level closure rows, not just a filename.", "row_id": "issues-single-assignee-and-atomic-checkout-for-in-progress", "verdict": "PLAN_DESIGN_EXTENSION", "waiver_ref": ""}
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.jsonl:67:{"acceptance_id": "ACC-02-GOALS-PROJECTS-ISSUES-WORK-PRODUCTS-0067", "capability": "Project CRUD, lead, target date, goal linkage", "current_rewrite_evidence": "not yet covered by rewrite parity matrix; implementation evidence required", "domain": "Projects", "evidence_ref": "to be filled when phase implements source-backed behavior", "failing_test": "backend-rs/tests/tasks_parity.rs::projects_project_crud_lead_target_date_goal_linkage", "final_status": "unclosed", "initial_status": "missing", "original_behavior": "Project CRUD, lead, target date, goal linkage", "original_evidence": "Project CRUD, lead, target date, goal linkage", "passing_test": "cd backend-rs && cargo test --test tasks_parity", "phase": "phases/02-goals-projects-issues-work-products.md", "review_note": "Added after Codex review: plan-design added this V1 area as a phase, so it must own row-level closure rows, not just a filename.", "row_id": "projects-project-crud-lead-target-date-goal-linkage", "verdict": "PLAN_DESIGN_EXTENSION", "waiver_ref": ""}
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.jsonl:68:{"acceptance_id": "ACC-02-GOALS-PROJECTS-ISSUES-WORK-PRODUCTS-0068", "capability": "Project env overlay merging into run environment", "current_rewrite_evidence": "not yet covered by rewrite parity matrix; implementation evidence required", "domain": "Projects", "evidence_ref": "to be filled when phase implements source-backed behavior", "failing_test": "backend-rs/tests/tasks_parity.rs::projects_project_env_overlay_merging_into_run_environment", "final_status": "unclosed", "initial_status": "missing", "original_behavior": "Project env overlay merging into run environment", "original_evidence": "Project env overlay merging into run environment", "passing_test": "cd backend-rs && cargo test --test tasks_parity", "phase": "phases/02-goals-projects-issues-work-products.md", "review_note": "Added after Codex review: plan-design added this V1 area as a phase, so it must own row-level closure rows, not just a filename.", "row_id": "projects-project-env-overlay-merging-into-run-environment", "verdict": "PLAN_DESIGN_EXTENSION", "waiver_ref": ""}
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.jsonl:69:{"acceptance_id": "ACC-02-GOALS-PROJECTS-ISSUES-WORK-PRODUCTS-0069", "capability": "Work product/artifact metadata and inspectable outputs", "current_rewrite_evidence": "not yet covered by rewrite parity matrix; implementation evidence required", "domain": "Work products", "evidence_ref": "to be filled when phase implements source-backed behavior", "failing_test": "backend-rs/tests/tasks_parity.rs::work_products_work_product_artifact_metadata_and_inspectable_outputs", "final_status": "unclosed", "initial_status": "missing", "original_behavior": "Work product/artifact metadata and inspectable outputs", "original_evidence": "Work product/artifact metadata and inspectable outputs", "passing_test": "cd backend-rs && cargo test --test tasks_parity", "phase": "phases/02-goals-projects-issues-work-products.md", "review_note": "Added after Codex review: plan-design added this V1 area as a phase, so it must own row-level closure rows, not just a filename.", "row_id": "work-products-work-product-artifact-metadata-and-inspectable-outputs", "verdict": "PLAN_DESIGN_EXTENSION", "waiver_ref": ""}
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.jsonl:70:{"acceptance_id": "ACC-03-WORKSPACES-ROUTINES-PLUGINS-SANDBOX-0070", "capability": "Project execution workspace policy and provision command", "current_rewrite_evidence": "not yet covered by rewrite parity matrix; implementation evidence required", "domain": "Execution workspaces", "evidence_ref": "to be filled when phase implements source-backed behavior", "failing_test": "backend-rs/tests/runtime_extensions_parity.rs::execution_workspaces_project_execution_workspace_policy_and_provision_command", "final_status": "unclosed", "initial_status": "missing", "original_behavior": "Project execution workspace policy and provision command", "original_evidence": "Project execution workspace policy and provision command", "passing_test": "cd backend-rs && cargo test --test runtime_extensions_parity", "phase": "phases/03-workspaces-routines-plugins-sandbox.md", "review_note": "Added after Codex review: plan-design added this V1 area as a phase, so it must own row-level closure rows, not just a filename.", "row_id": "execution-workspaces-project-execution-workspace-policy-and-provision-command", "verdict": "PLAN_DESIGN_EXTENSION", "waiver_ref": ""}
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.jsonl:71:{"acceptance_id": "ACC-03-WORKSPACES-ROUTINES-PLUGINS-SANDBOX-0071", "capability": "Plugin manifest, namespace, trust boundary", "current_rewrite_evidence": "not yet covered by rewrite parity matrix; implementation evidence required", "domain": "Plugins", "evidence_ref": "to be filled when phase implements source-backed behavior", "failing_test": "backend-rs/tests/runtime_extensions_parity.rs::plugins_plugin_manifest_namespace_trust_boundary", "final_status": "unclosed", "initial_status": "missing", "original_behavior": "Plugin manifest, namespace, trust boundary", "original_evidence": "Plugin manifest, namespace, trust boundary", "passing_test": "cd backend-rs && cargo test --test runtime_extensions_parity", "phase": "phases/03-workspaces-routines-plugins-sandbox.md", "review_note": "Added after Codex review: plan-design added this V1 area as a phase, so it must own row-level closure rows, not just a filename.", "row_id": "plugins-plugin-manifest-namespace-trust-boundary", "verdict": "PLAN_DESIGN_EXTENSION", "waiver_ref": ""}
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.jsonl:72:{"acceptance_id": "ACC-03-WORKSPACES-ROUTINES-PLUGINS-SANDBOX-0072", "capability": "Routine schedule, webhook/API dispatch, env overlay snapshot", "current_rewrite_evidence": "not yet covered by rewrite parity matrix; implementation evidence required", "domain": "Routines", "evidence_ref": "to be filled when phase implements source-backed behavior", "failing_test": "backend-rs/tests/runtime_extensions_parity.rs::routines_routine_schedule_webhook_api_dispatch_env_overlay_snapshot", "final_status": "unclosed", "initial_status": "missing", "original_behavior": "Routine schedule, webhook/API dispatch, env overlay snapshot", "original_evidence": "Routine schedule, webhook/API dispatch, env overlay snapshot", "passing_test": "cd backend-rs && cargo test --test runtime_extensions_parity", "phase": "phases/03-workspaces-routines-plugins-sandbox.md", "review_note": "Added after Codex review: plan-design added this V1 area as a phase, so it must own row-level closure rows, not just a filename.", "row_id": "routines-routine-schedule-webhook-api-dispatch-env-overlay-snapshot", "verdict": "PLAN_DESIGN_EXTENSION", "waiver_ref": ""}
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.jsonl:73:{"acceptance_id": "ACC-03-WORKSPACES-ROUTINES-PLUGINS-SANDBOX-0073", "capability": "Managed runtime/preview/dev service lifecycle", "current_rewrite_evidence": "not yet covered by rewrite parity matrix; implementation evidence required", "domain": "Runtime services", "evidence_ref": "to be filled when phase implements source-backed behavior", "failing_test": "backend-rs/tests/runtime_extensions_parity.rs::runtime_services_managed_runtime_preview_dev_service_lifecycle", "final_status": "unclosed", "initial_status": "missing", "original_behavior": "Managed runtime/preview/dev service lifecycle", "original_evidence": "Managed runtime/preview/dev service lifecycle", "passing_test": "cd backend-rs && cargo test --test runtime_extensions_parity", "phase": "phases/03-workspaces-routines-plugins-sandbox.md", "review_note": "Added after Codex review: plan-design added this V1 area as a phase, so it must own row-level closure rows, not just a filename.", "row_id": "runtime-services-managed-runtime-preview-dev-service-lifecycle", "verdict": "PLAN_DESIGN_EXTENSION", "waiver_ref": ""}
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.jsonl:74:{"acceptance_id": "ACC-03-WORKSPACES-ROUTINES-PLUGINS-SANDBOX-0074", "capability": "Sandbox provider containment and low-trust boundaries", "current_rewrite_evidence": "not yet covered by rewrite parity matrix; implementation evidence required", "domain": "Sandbox", "evidence_ref": "to be filled when phase implements source-backed behavior", "failing_test": "backend-rs/tests/runtime_extensions_parity.rs::sandbox_sandbox_provider_containment_and_low_trust_boundaries", "final_status": "unclosed", "initial_status": "missing", "original_behavior": "Sandbox provider containment and low-trust boundaries", "original_evidence": "Sandbox provider containment and low-trust boundaries", "passing_test": "cd backend-rs && cargo test --test runtime_extensions_parity", "phase": "phases/03-workspaces-routines-plugins-sandbox.md", "review_note": "Added after Codex review: plan-design added this V1 area as a phase, so it must own row-level closure rows, not just a filename.", "row_id": "sandbox-sandbox-provider-containment-and-low-trust-boundaries", "verdict": "PLAN_DESIGN_EXTENSION", "waiver_ref": ""}
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.jsonl:75:{"acceptance_id": "ACC-04-DEPLOYMENT-AUTH-ONBOARDING-OPS-0075", "capability": "authenticated private and public exposure policy", "current_rewrite_evidence": "not yet covered by rewrite parity matrix; implementation evidence required", "domain": "Deployment modes", "evidence_ref": "to be filled when phase implements source-backed behavior", "failing_test": "backend-rs/tests/deployment_auth_parity.rs::deployment_modes_authenticated_private_and_public_exposure_policy", "final_status": "unclosed", "initial_status": "missing", "original_behavior": "authenticated private and public exposure policy", "original_evidence": "authenticated private and public exposure policy", "passing_test": "cd backend-rs && cargo test --test deployment_auth_parity", "phase": "phases/04-deployment-auth-onboarding-ops.md", "review_note": "Added after Codex review: plan-design added this V1 area as a phase, so it must own row-level closure rows, not just a filename.", "row_id": "deployment-modes-authenticated-private-and-public-exposure-policy", "verdict": "PLAN_DESIGN_EXTENSION", "waiver_ref": ""}
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.jsonl:76:{"acceptance_id": "ACC-04-DEPLOYMENT-AUTH-ONBOARDING-OPS-0076", "capability": "local_trusted implicit board behavior", "current_rewrite_evidence": "not yet covered by rewrite parity matrix; implementation evidence required", "domain": "Deployment modes", "evidence_ref": "to be filled when phase implements source-backed behavior", "failing_test": "backend-rs/tests/deployment_auth_parity.rs::deployment_modes_local_trusted_implicit_board_behavior", "final_status": "unclosed", "initial_status": "missing", "original_behavior": "local_trusted implicit board behavior", "original_evidence": "local_trusted implicit board behavior", "passing_test": "cd backend-rs && cargo test --test deployment_auth_parity", "phase": "phases/04-deployment-auth-onboarding-ops.md", "review_note": "Added after Codex review: plan-design added this V1 area as a phase, so it must own row-level closure rows, not just a filename.", "row_id": "deployment-modes-local-trusted-implicit-board-behavior", "verdict": "PLAN_DESIGN_EXTENSION", "waiver_ref": ""}
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.jsonl:77:{"acceptance_id": "ACC-04-DEPLOYMENT-AUTH-ONBOARDING-OPS-0077", "capability": "Public authenticated deployment refuses embedded DB fallback", "current_rewrite_evidence": "not yet covered by rewrite parity matrix; implementation evidence required", "domain": "Deployment modes", "evidence_ref": "to be filled when phase implements source-backed behavior", "failing_test": "backend-rs/tests/deployment_auth_parity.rs::deployment_modes_public_authenticated_deployment_refuses_embedded_db_fallback", "final_status": "unclosed", "initial_status": "missing", "original_behavior": "Public authenticated deployment refuses embedded DB fallback", "original_evidence": "Public authenticated deployment refuses embedded DB fallback", "passing_test": "cd backend-rs && cargo test --test deployment_auth_parity", "phase": "phases/04-deployment-auth-onboarding-ops.md", "review_note": "Added after Codex review: plan-design added this V1 area as a phase, so it must own row-level closure rows, not just a filename.", "row_id": "deployment-modes-public-authenticated-deployment-refuses-embedded-db-fallback", "verdict": "PLAN_DESIGN_EXTENSION", "waiver_ref": ""}
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.jsonl:78:{"acceptance_id": "ACC-04-DEPLOYMENT-AUTH-ONBOARDING-OPS-0078", "capability": "First-admin bootstrap and claim flow without Supabase exposure", "current_rewrite_evidence": "not yet covered by rewrite parity matrix; implementation evidence required", "domain": "Onboarding", "evidence_ref": "to be filled when phase implements source-backed behavior", "failing_test": "backend-rs/tests/deployment_auth_parity.rs::onboarding_first_admin_bootstrap_and_claim_flow_without_supabase_exposure", "final_status": "unclosed", "initial_status": "missing", "original_behavior": "First-admin bootstrap and claim flow without Supabase exposure", "original_evidence": "First-admin bootstrap and claim flow without Supabase exposure", "passing_test": "cd backend-rs && cargo test --test deployment_auth_parity", "phase": "phases/04-deployment-auth-onboarding-ops.md", "review_note": "Added after Codex review: plan-design added this V1 area as a phase, so it must own row-level closure rows, not just a filename.", "row_id": "onboarding-first-admin-bootstrap-and-claim-flow-without-supabase-exposure", "verdict": "PLAN_DESIGN_EXTENSION", "waiver_ref": ""}
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.jsonl:79:{"acceptance_id": "ACC-04-DEPLOYMENT-AUTH-ONBOARDING-OPS-0079", "capability": "Doctor, onboard, backup, storage, config admin surfaces", "current_rewrite_evidence": "not yet covered by rewrite parity matrix; implementation evidence required", "domain": "Ops", "evidence_ref": "to be filled when phase implements source-backed behavior", "failing_test": "backend-rs/tests/deployment_auth_parity.rs::ops_doctor_onboard_backup_storage_config_admin_surfaces", "final_status": "unclosed", "initial_status": "missing", "original_behavior": "Doctor, onboard, backup, storage, config admin surfaces", "original_evidence": "Doctor, onboard, backup, storage, config admin surfaces", "passing_test": "cd backend-rs && cargo test --test deployment_auth_parity", "phase": "phases/04-deployment-auth-onboarding-ops.md", "review_note": "Added after Codex review: plan-design added this V1 area as a phase, so it must own row-level closure rows, not just a filename.", "row_id": "ops-doctor-onboard-backup-storage-config-admin-surfaces", "verdict": "PLAN_DESIGN_EXTENSION", "waiver_ref": ""}
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.jsonl:80:{"acceptance_id": "ACC-04-DEPLOYMENT-AUTH-ONBOARDING-OPS-0080", "capability": "Instance settings and sidebar preferences", "current_rewrite_evidence": "not yet covered by rewrite parity matrix; implementation evidence required", "domain": "Ops", "evidence_ref": "to be filled when phase implements source-backed behavior", "failing_test": "backend-rs/tests/deployment_auth_parity.rs::ops_instance_settings_and_sidebar_preferences", "final_status": "unclosed", "initial_status": "missing", "original_behavior": "Instance settings and sidebar preferences", "original_evidence": "Instance settings and sidebar preferences", "passing_test": "cd backend-rs && cargo test --test deployment_auth_parity", "phase": "phases/04-deployment-auth-onboarding-ops.md", "review_note": "Added after Codex review: plan-design added this V1 area as a phase, so it must own row-level closure rows, not just a filename.", "row_id": "ops-instance-settings-and-sidebar-preferences", "verdict": "PLAN_DESIGN_EXTENSION", "waiver_ref": ""}
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.jsonl:81:{"acceptance_id": "ACC-05-ADAPTERS-SECRETS-SKILLS-CONFIG-0081", "capability": "Adapter-type default config injection", "current_rewrite_evidence": "supabase/migrations/paperclip/0013_agents_hiring.sql:105,143", "domain": "Hiring flow & side effects", "evidence_ref": "original_evidence + current_rewrite_evidence", "failing_test": "backend-rs/tests/adapter_config_parity.rs::hiring_flow_side_effects_adapter_defaults_injection", "final_status": "unclosed", "initial_status": "missing", "original_behavior": "", "original_evidence": "", "passing_test": "cd backend-rs && cargo test --test adapter_config_parity", "phase": "phases/05-adapters-secrets-skills-config.md", "review_note": "Verified: no per-adapter default-config or gateway device-key injection. Only a fixed scalar model default 'gpt-5.4-mini' (0013:105,143). Missing stands.", "row_id": "hiring-flow-side-effects-adapter-defaults-injection", "verdict": "CONFIRMED", "waiver_ref": ""}
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.jsonl:82:{"acceptance_id": "ACC-05-ADAPTERS-SECRETS-SKILLS-CONFIG-0082", "capability": "Adapter type must be a registered server adapter", "current_rewrite_evidence": "backend-rs/src/supabase/routes.rs:191-198; supabase/migrations/paperclip/0013_agents_hiring.sql:105,143", "domain": "Hiring flow & side effects", "evidence_ref": "original_evidence + current_rewrite_evidence", "failing_test": "backend-rs/tests/adapter_config_parity.rs::hiring_flow_side_effects_adapter_type_validation", "final_status": "unclosed", "initial_status": "missing", "original_behavior": "", "original_evidence": "", "passing_test": "cd backend-rs && cargo test --test adapter_config_parity", "phase": "phases/05-adapters-secrets-skills-config.md", "review_note": "Verified: adapter_type is never accepted on the HTTP/CLI/MCP surface (HirePaperclipAgent has no adapterType field, routes.rs:191-198), and the RPC merely coalesces p_adapter_type to 'openai' (0013:105,143) with zero registry validation. No adapter registry exists in the rewrite. Missing stands.", "row_id": "hiring-flow-side-effects-adapter-type-validation", "verdict": "CONFIRMED", "waiver_ref": ""}
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.jsonl:83:{"acceptance_id": "ACC-05-ADAPTERS-SECRETS-SKILLS-CONFIG-0083", "capability": "Agent-authenticated callers cannot set instructions/workspace-command config", "current_rewrite_evidence": "supabase/migrations/paperclip/0013_agents_hiring.sql:84-151,186", "domain": "Hiring flow & side effects", "evidence_ref": "original_evidence + current_rewrite_evidence", "failing_test": "backend-rs/tests/adapter_config_parity.rs::hiring_flow_side_effects_agent_actor_config_guard", "final_status": "unclosed", "initial_status": "missing", "original_behavior": "", "original_evidence": "", "passing_test": "cd backend-rs && cargo test --test adapter_config_parity", "phase": "phases/05-adapters-secrets-skills-config.md", "review_note": "Verified: hires carry no adapterConfig, so the specific instructions-bundle/workspace-command key guard is absent. The rewrite's agent-actor guards are a different concern (can_create_agents self-propagation blocked 0013:131-132,143; agents may never decide 0013:186) and do not implement this capability. Missing stands.", "row_id": "hiring-flow-side-effects-agent-actor-config-guard", "verdict": "CONFIRMED", "waiver_ref": ""}
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.jsonl:84:{"acceptance_id": "ACC-05-ADAPTERS-SECRETS-SKILLS-CONFIG-0084", "capability": "Agent creation side effects (name dedupe, manager validation, secret bindings)", "current_rewrite_evidence": "supabase/migrations/paperclip/0013_agents_hiring.sql:21-38,99-100,105,139-140,143", "domain": "Hiring flow & side effects", "evidence_ref": "original_evidence + current_rewrite_evidence", "failing_test": "backend-rs/tests/adapter_config_parity.rs::hiring_flow_side_effects_agent_create_side_effects", "final_status": "unclosed", "initial_status": "partial", "original_behavior": "", "original_evidence": "", "passing_test": "cd backend-rs && cargo test --test adapter_config_parity", "phase": "phases/05-adapters-secrets-skills-config.md", "review_note": "Verified: only manager validation (reports_to must be an agent in same company, 0013:99-100,139-140) and a fixed adapter default (0013:105,143) are present \u2014 and reports_to is never wired through HTTP/CLI/MCP. No name deduplication (agents.name has no unique constraint, 0013:21-38), no per-role permission normalization, no secret-binding sync. Partial stands.", "row_id": "hiring-flow-side-effects-agent-create-side-effects", "verdict": "CONFIRMED", "waiver_ref": ""}
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.jsonl:85:{"acceptance_id": "ACC-05-ADAPTERS-SECRETS-SKILLS-CONFIG-0085", "capability": "hire_agent approval record created with redacted config snapshot", "current_rewrite_evidence": "supabase/migrations/paperclip/0013_agents_hiring.sql:44-57,107-111,145-149", "domain": "Hiring flow & side effects", "evidence_ref": "original_evidence + current_rewrite_evidence", "failing_test": "backend-rs/tests/adapter_config_parity.rs::hiring_flow_side_effects_approval_record_creation", "final_status": "unclosed", "initial_status": "partial", "original_behavior": "", "original_evidence": "", "passing_test": "cd backend-rs && cargo test --test adapter_config_parity", "phase": "phases/05-adapters-secrets-skills-config.md", "review_note": "Verified: approval row inserted only when v_require (0013:107-111,145-149) with type='hire_agent', status='pending', requested_by, subject_agent_id, and a minimal payload jsonb_build_object('name','role','model'). No redacted adapterConfig/runtimeConfig/metadata snapshot, no requestedConfigurationSnapshot; agent linkage via subject_agent_id column not payload.agentId. Row+core fields present, rich payload absent = partial. Stands.", "row_id": "hiring-flow-side-effects-approval-record-creation", "verdict": "CONFIRMED", "waiver_ref": ""}
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.jsonl:86:{"acceptance_id": "ACC-05-ADAPTERS-SECRETS-SKILLS-CONFIG-0086", "capability": "codex_local per-agent CODEX_HOME isolation", "current_rewrite_evidence": "supabase/migrations/paperclip/0013_agents_hiring.sql:84-151 (no env/workspace)", "domain": "Hiring flow & side effects", "evidence_ref": "original_evidence + current_rewrite_evidence", "failing_test": "backend-rs/tests/adapter_config_parity.rs::hiring_flow_side_effects_codex_key_isolation", "final_status": "unclosed", "initial_status": "missing", "original_behavior": "", "original_evidence": "", "passing_test": "cd backend-rs && cargo test --test adapter_config_parity", "phase": "phases/05-adapters-secrets-skills-config.md", "review_note": "Verified: no codex_local adapter and no env/workspace/CODEX_HOME handling anywhere in the rewrite. Missing stands.", "row_id": "hiring-flow-side-effects-codex-key-isolation", "verdict": "CONFIRMED", "waiver_ref": ""}
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.jsonl:87:{"acceptance_id": "ACC-05-ADAPTERS-SECRETS-SKILLS-CONFIG-0087", "capability": "Desired skills resolution + skill sync preference persisted", "current_rewrite_evidence": "supabase/migrations/paperclip/0013_agents_hiring.sql:21-38 (agents table has no skills)", "domain": "Hiring flow & side effects", "evidence_ref": "original_evidence + current_rewrite_evidence", "failing_test": "backend-rs/tests/adapter_config_parity.rs::hiring_flow_side_effects_desired_skill_assignment", "final_status": "unclosed", "initial_status": "missing", "original_behavior": "", "original_evidence": "", "passing_test": "cd backend-rs && cargo test --test adapter_config_parity", "phase": "phases/05-adapters-secrets-skills-config.md", "review_note": "Verified: no skills subsystem, no desiredSkills field, no runtime-skill materialization or sync-preference persistence in migrations 0011/0013 or the Rust services. Missing stands.", "row_id": "hiring-flow-side-effects-desired-skill-assignment", "verdict": "CONFIRMED", "waiver_ref": ""}
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.jsonl:88:{"acceptance_id": "ACC-05-ADAPTERS-SECRETS-SKILLS-CONFIG-0088", "capability": "onHireApproved adapter hook (hire-hook.ts)", "current_rewrite_evidence": "supabase/migrations/paperclip/0013_agents_hiring.sql:154-206", "domain": "Hiring flow & side effects", "evidence_ref": "original_evidence + current_rewrite_evidence", "failing_test": "backend-rs/tests/adapter_config_parity.rs::hiring_flow_side_effects_hire_hook_onhireapproved", "final_status": "unclosed", "initial_status": "missing", "original_behavior": "", "original_evidence": "", "passing_test": "cd backend-rs && cargo test --test adapter_config_parity", "phase": "phases/05-adapters-secrets-skills-config.md", "review_note": "Verified: no adapter-hook mechanism and no onboarding notification anywhere in the rewrite; decide_approval only updates agent+approval rows. Missing stands.", "row_id": "hiring-flow-side-effects-hire-hook-onhireapproved", "verdict": "CONFIRMED", "waiver_ref": ""}
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.jsonl:89:{"acceptance_id": "ACC-05-ADAPTERS-SECRETS-SKILLS-CONFIG-0089", "capability": "Default instructions bundle materialized on hire", "current_rewrite_evidence": "supabase/migrations/paperclip/0013_agents_hiring.sql:84-151", "domain": "Hiring flow & side effects", "evidence_ref": "original_evidence + current_rewrite_evidence", "failing_test": "backend-rs/tests/adapter_config_parity.rs::hiring_flow_side_effects_instructions_bundle_materialization", "final_status": "unclosed", "initial_status": "missing", "original_behavior": "", "original_evidence": "", "passing_test": "cd backend-rs && cargo test --test adapter_config_parity", "phase": "phases/05-adapters-secrets-skills-config.md", "review_note": "Verified: no instructions-bundle concept or AGENTS.md materialization in the hire path. Missing stands.", "row_id": "hiring-flow-side-effects-instructions-bundle-materialization", "verdict": "CONFIRMED", "waiver_ref": ""}
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.jsonl:91:{"acceptance_id": "ACC-05-ADAPTERS-SECRETS-SKILLS-CONFIG-0091", "capability": "Runtime config heartbeat defaults", "current_rewrite_evidence": "supabase/migrations/paperclip/0013_agents_hiring.sql:21-38", "domain": "Hiring flow & side effects", "evidence_ref": "original_evidence + current_rewrite_evidence", "failing_test": "backend-rs/tests/adapter_config_parity.rs::hiring_flow_side_effects_runtime_heartbeat_normalization", "final_status": "unclosed", "initial_status": "missing", "original_behavior": "", "original_evidence": "", "passing_test": "cd backend-rs && cargo test --test adapter_config_parity", "phase": "phases/05-adapters-secrets-skills-config.md", "review_note": "Verified: no runtimeConfig/heartbeat/maxConcurrentRuns concept; agents table has no such columns (0013:21-38). Missing stands.", "row_id": "hiring-flow-side-effects-runtime-heartbeat-normalization", "verdict": "CONFIRMED", "waiver_ref": ""}
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.jsonl:92:{"acceptance_id": "ACC-05-ADAPTERS-SECRETS-SKILLS-CONFIG-0092", "capability": "Adapter/runtime config secret normalization + constraints", "current_rewrite_evidence": "supabase/migrations/paperclip/0013_agents_hiring.sql:84-151", "domain": "Hiring flow & side effects", "evidence_ref": "original_evidence + current_rewrite_evidence", "failing_test": "backend-rs/tests/adapter_config_parity.rs::hiring_flow_side_effects_secret_normalization", "final_status": "unclosed", "initial_status": "missing", "original_behavior": "", "original_evidence": "", "passing_test": "cd backend-rs && cargo test --test adapter_config_parity", "phase": "phases/05-adapters-secrets-skills-config.md", "review_note": "Verified: no adapterConfig and no secret extraction/ref/constraint logic anywhere in the rewrite hire path. Missing stands.", "row_id": "hiring-flow-side-effects-secret-normalization", "verdict": "CONFIRMED", "waiver_ref": ""}
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.jsonl:94:{"acceptance_id": "ACC-06-APPROVALS-BUDGETS-COSTS-ACTIVITY-0094", "capability": "approval_comments table + comment endpoints", "current_rewrite_evidence": "none found \u2014 no approval_comments in any of 0001-0013; routes.rs:54-61 wires only list + decide", "domain": "Approvals system & governance", "evidence_ref": "original_evidence + current_rewrite_evidence", "failing_test": "backend-rs/tests/approvals_parity.rs::approvals_system_governance_approval_comments_table", "final_status": "unclosed", "initial_status": "missing", "original_behavior": "", "original_evidence": "", "passing_test": "cd backend-rs && cargo test --test approvals_parity", "phase": "phases/06-approvals-budgets-costs-activity.md", "review_note": "CONFIRMED absent. Verified original has the table (packages/db/src/schema/approval_comments.ts) and endpoints (approvals.ts:383,395). No comment table or endpoint anywhere in the rewrite; grep for approval_comment across migrations/backend-rs/frontend-nuxt returns nothing.", "row_id": "approvals-system-governance-approval-comments-table", "verdict": "CONFIRMED", "waiver_ref": ""}
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.jsonl:95:{"acceptance_id": "ACC-06-APPROVALS-BUDGETS-COSTS-ACTIVITY-0095", "capability": "Approval status lifecycle enum (5 statuses)", "current_rewrite_evidence": "supabase/migrations/paperclip/0013_agents_hiring.sql:17 (enum = pending,approved,rejected,cancelled), 49 (default pending), 163/168/174 (only pending decidable; only approved/rejected written)", "domain": "Approvals system & governance", "evidence_ref": "original_evidence + current_rewrite_evidence", "failing_test": "backend-rs/tests/approvals_parity.rs::approvals_system_governance_approval_status_enum", "final_status": "unclosed", "initial_status": "divergent", "original_behavior": "", "original_evidence": "", "passing_test": "cd backend-rs && cargo test --test approvals_parity", "phase": "phases/06-approvals-budgets-costs-activity.md", "review_note": "CONFIRMED. Rewrite enum is 4 members missing 'revision_requested'; verified original APPROVAL_STATUSES includes revision_requested (constants.ts:548). Default pending matches; cancelled present-but-unwritten matches; the entire revision lifecycle is structurally absent and only 'pending' is decidable (0013:163). Different lifecycle model = divergent is correct.", "row_id": "approvals-system-governance-approval-status-enum", "verdict": "CONFIRMED", "waiver_ref": ""}
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.jsonl:96:{"acceptance_id": "ACC-06-APPROVALS-BUDGETS-COSTS-ACTIVITY-0096", "capability": "Approval type enum (4 governed types)", "current_rewrite_evidence": "supabase/migrations/paperclip/0013_agents_hiring.sql:14 (enum has only hire_agent, approve_ceo_strategy); 0013:108-109,146-147 (only hire_agent ever inserted); 0013:165-166,171-172 (hire_agent decide effect)", "domain": "Approvals system & governance", "evidence_ref": "original_evidence + current_rewrite_evidence", "failing_test": "backend-rs/tests/approvals_parity.rs::approvals_system_governance_approval_types_enum", "final_status": "unclosed", "initial_status": "partial", "original_behavior": "", "original_evidence": "", "passing_test": "cd backend-rs && cargo test --test approvals_parity", "phase": "phases/06-approvals-budgets-costs-activity.md", "review_note": "CONFIRMED. paperclip.approval_type = {hire_agent, approve_ceo_strategy} \u2014 a strict subset of the original 4; budget_override_required and request_board_approval absent. hire_agent (the only type with a decide-time effect) is faithfully reproduced (activate/archive subject agent). approve_ceo_strategy is in the enum but never insertable (only hire RPCs create rows). Some enum + full hire_agent behavior present = partial is correct.", "row_id": "approvals-system-governance-approval-types-enum", "verdict": "CONFIRMED", "waiver_ref": ""}
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.jsonl:98:{"acceptance_id": "ACC-06-APPROVALS-BUDGETS-COSTS-ACTIVITY-0098", "capability": "approve effect: hire_agent activates/creates agent + budget + notify", "current_rewrite_evidence": "supabase/migrations/paperclip/0013_agents_hiring.sql:164-167,194-196", "domain": "Approvals system & governance", "evidence_ref": "original_evidence + current_rewrite_evidence", "failing_test": "backend-rs/tests/approvals_parity.rs::approvals_system_governance_approve_effect_hire_agent", "final_status": "unclosed", "initial_status": "partial", "original_behavior": "", "original_evidence": "", "passing_test": "cd backend-rs && cargo test --test approvals_parity", "phase": "phases/06-approvals-budgets-costs-activity.md", "review_note": "CONFIRMED (status unchanged) but reviewer's evidence has a factual error to correct. Activation is reproduced: on approve, UPDATE agents SET status='active' WHERE id=subject_agent_id AND status='pending_approval' (0013:166,195). No create-from-payload branch (agent pre-created at hire, 0013:104-106); no notify/hire hook. IMPORTANT CORRECTION: the note's 'no budgets table/policy anywhere' is FALSE \u2014 backend-rs/src/budgets.rs exists (in-memory BudgetStore porting the hard-stop invariant, get/set_limit/record_spend). HOWEVER it is an in-memory HashMap with NO budget table in any migration and is completely UNWIRED from decide_approval/hire (grep of budgets.rs for approval|hire|decide = 0 hits), so the approve-time budget upsertPolicy effect is still genuinely absent. Activation present; budget+notify effects absent = partial stays correct.", "row_id": "approvals-system-governance-approve-effect-hire-agent", "verdict": "CONFIRMED", "waiver_ref": ""}
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.jsonl:99:{"acceptance_id": "ACC-06-APPROVALS-BUDGETS-COSTS-ACTIVITY-0099", "capability": "approve effect: wake requesting agent + activity logs", "current_rewrite_evidence": "none found \u2014 decide_approval (0013:164-176) only does status/agent UPDATEs", "domain": "Approvals system & governance", "evidence_ref": "original_evidence + current_rewrite_evidence", "failing_test": "backend-rs/tests/approvals_parity.rs::approvals_system_governance_approve_effect_requester_wakeup", "final_status": "unclosed", "initial_status": "missing", "original_behavior": "", "original_evidence": "", "passing_test": "cd backend-rs && cargo test --test approvals_parity", "phase": "phases/06-approvals-budgets-costs-activity.md", "review_note": "CONFIRMED absent. No heartbeat/wakeup subsystem, no requestedByAgentId (requester is always a user uuid, 0013:52), and no activity logging in the rewrite. decide_approval does nothing beyond the status + agent status UPDATE.", "row_id": "approvals-system-governance-approve-effect-requester-wakeup", "verdict": "CONFIRMED", "waiver_ref": ""}
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.jsonl:100:{"acceptance_id": "ACC-06-APPROVALS-BUDGETS-COSTS-ACTIVITY-0100", "capability": "Approve transition + idempotency", "current_rewrite_evidence": "supabase/migrations/paperclip/0013_agents_hiring.sql:159,163,164-169", "domain": "Approvals system & governance", "evidence_ref": "original_evidence + current_rewrite_evidence", "failing_test": "backend-rs/tests/approvals_parity.rs::approvals_system_governance_approve_lifecycle", "final_status": "unclosed", "initial_status": "divergent", "original_behavior": "", "original_evidence": "", "passing_test": "cd backend-rs && cargo test --test approvals_parity", "phase": "phases/06-approvals-budgets-costs-activity.md", "review_note": "CONFIRMED. FOR UPDATE gives concurrency safety, but idempotency diverges: re-deciding a non-pending approval raises 22023 (0013:163), which is unmapped in routes.rs:440-467 and thus surfaces as 502 paperclip_backend_unavailable rather than an idempotent {applied:false} no-op. Return is the approval_status enum string, not {approval,applied}. Cannot approve revision_requested (status gone). Divergent semantics = correct.", "row_id": "approvals-system-governance-approve-lifecycle", "verdict": "CONFIRMED", "waiver_ref": ""}
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.jsonl:101:{"acceptance_id": "ACC-06-APPROVALS-BUDGETS-COSTS-ACTIVITY-0101", "capability": "Board chat persisted to standing 'Board Operations' issue", "current_rewrite_evidence": "none found", "domain": "Approvals system & governance", "evidence_ref": "original_evidence + current_rewrite_evidence", "failing_test": "backend-rs/tests/approvals_parity.rs::approvals_system_governance_board_chat_issue_persistence", "final_status": "unclosed", "initial_status": "missing", "original_behavior": "", "original_evidence": "", "passing_test": "cd backend-rs && cargo test --test approvals_parity", "phase": "phases/06-approvals-budgets-costs-activity.md", "review_note": "CONFIRMED absent. No board chat, no issues table/concept, and no 'Board Operations' persistence in the rewrite. Entirely absent.", "row_id": "approvals-system-governance-board-chat-issue-persistence", "verdict": "CONFIRMED", "waiver_ref": ""}
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.jsonl:102:{"acceptance_id": "ACC-06-APPROVALS-BUDGETS-COSTS-ACTIVITY-0102", "capability": "Board Concierge Chat SSE relay", "current_rewrite_evidence": "none found \u2014 routes.rs:36-62 has no board-chat route", "domain": "Approvals system & governance", "evidence_ref": "original_evidence + current_rewrite_evidence", "failing_test": "backend-rs/tests/approvals_parity.rs::approvals_system_governance_board_chat_relay", "final_status": "unclosed", "initial_status": "missing", "original_behavior": "", "original_evidence": "", "passing_test": "cd backend-rs && cargo test --test approvals_parity", "phase": "phases/06-approvals-budgets-costs-activity.md", "review_note": "CONFIRMED absent. Verified original exists (server/src/routes/board-chat.ts). No /board/chat/stream endpoint, no SSE, no CLI-spawn concierge anywhere in the rewrite route table.", "row_id": "approvals-system-governance-board-chat-relay", "verdict": "CONFIRMED", "waiver_ref": ""}
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.jsonl:103:{"acceptance_id": "ACC-06-APPROVALS-BUDGETS-COSTS-ACTIVITY-0103", "capability": "Create approval (pending)", "current_rewrite_evidence": "supabase/migrations/paperclip/0013_agents_hiring.sql:107-111,145-149", "domain": "Approvals system & governance", "evidence_ref": "original_evidence + current_rewrite_evidence", "failing_test": "backend-rs/tests/approvals_parity.rs::approvals_system_governance_create_approval", "final_status": "unclosed", "initial_status": "partial", "original_behavior": "", "original_evidence": "", "passing_test": "cd backend-rs && cargo test --test approvals_parity", "phase": "phases/06-approvals-budgets-costs-activity.md", "review_note": "CONFIRMED. No generic POST /companies/:id/approvals endpoint; a pending hire_agent approval is created only as an internal side effect of hire_agent/hire_agent_with_key when require_board_approval_for_new_agents (default true, 0013:7). Consequently no schema validation, no arbitrary-type creation (approve_ceo_strategy never requestable), no issue linking, no activity log, no secret normalization (payload is name/role/model only, 0013:109). Only the hire-driven pending-approval creation is reproduced = partial correct.", "row_id": "approvals-system-governance-create-approval", "verdict": "CONFIRMED", "waiver_ref": ""}
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.jsonl:105:{"acceptance_id": "ACC-06-APPROVALS-BUDGETS-COSTS-ACTIVITY-0105", "capability": "findOpenHireApprovalForAgent helper", "current_rewrite_evidence": "none found \u2014 hire RPCs always insert fresh (0013:104-111,142-149)", "domain": "Approvals system & governance", "evidence_ref": "original_evidence + current_rewrite_evidence", "failing_test": "backend-rs/tests/approvals_parity.rs::approvals_system_governance_find_open_hire_approval", "final_status": "unclosed", "initial_status": "missing", "original_behavior": "", "original_evidence": "", "passing_test": "cd backend-rs && cargo test --test approvals_parity", "phase": "phases/06-approvals-budgets-costs-activity.md", "review_note": "CONFIRMED absent. Verified original helper exists (services/approvals.ts:95). Rewrite hire RPCs unconditionally insert a fresh agent and (if required) a fresh pending approval with no dedupe check for an existing open hire approval; subject tracked via subject_agent_id, not payload.agentId.", "row_id": "approvals-system-governance-find-open-hire-approval", "verdict": "CONFIRMED", "waiver_ref": ""}
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.jsonl:106:{"acceptance_id": "ACC-06-APPROVALS-BUDGETS-COSTS-ACTIVITY-0106", "capability": "Get single approval / linked issues", "current_rewrite_evidence": "none found \u2014 routes.rs:54-61 exposes only list-per-company and /approvals/:id/decide", "domain": "Approvals system & governance", "evidence_ref": "original_evidence + current_rewrite_evidence", "failing_test": "backend-rs/tests/approvals_parity.rs::approvals_system_governance_get_approval", "final_status": "unclosed", "initial_status": "missing", "original_behavior": "", "original_evidence": "", "passing_test": "cd backend-rs && cargo test --test approvals_parity", "phase": "phases/06-approvals-budgets-costs-activity.md", "review_note": "CONFIRMED absent. No GET /approvals/:id and no /approvals/:id/issues; the issue-linking concept does not exist in the rewrite at all. Only list + decide are wired.", "row_id": "approvals-system-governance-get-approval", "verdict": "CONFIRMED", "waiver_ref": ""}
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.jsonl:108:{"acceptance_id": "ACC-06-APPROVALS-BUDGETS-COSTS-ACTIVITY-0108", "capability": "Approval payload + comment redaction on read", "current_rewrite_evidence": "none found \u2014 list returns raw rows (jobs/mod.rs:294-314; 0013:224)", "domain": "Approvals system & governance", "evidence_ref": "original_evidence + current_rewrite_evidence", "failing_test": "backend-rs/tests/approvals_parity.rs::approvals_system_governance_payload_redaction", "final_status": "unclosed", "initial_status": "missing", "original_behavior": "", "original_evidence": "", "passing_test": "cd backend-rs && cargo test --test approvals_parity", "phase": "phases/06-approvals-budgets-costs-activity.md", "review_note": "CONFIRMED absent. Grep for 'redact' across the rewrite returns nothing. list_approvals returns raw rows including payload; hire payload holds only name/role/model (0013:109) with no secret-normalization step, and there are no comments to censor. The secret-handling/redaction governance layer is absent.", "row_id": "approvals-system-governance-payload-redaction", "verdict": "CONFIRMED", "waiver_ref": ""}
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.jsonl:109:{"acceptance_id": "ACC-06-APPROVALS-BUDGETS-COSTS-ACTIVITY-0109", "capability": "Reject transition + hire_agent termination", "current_rewrite_evidence": "supabase/migrations/paperclip/0013_agents_hiring.sql:170-176,199-204", "domain": "Approvals system & governance", "evidence_ref": "original_evidence + current_rewrite_evidence", "failing_test": "backend-rs/tests/approvals_parity.rs::approvals_system_governance_reject_lifecycle_effect", "final_status": "unclosed", "initial_status": "partial", "original_behavior": "", "original_evidence": "", "passing_test": "cd backend-rs && cargo test --test approvals_parity", "phase": "phases/06-approvals-budgets-costs-activity.md", "review_note": "CONFIRMED. Reject is folded into decide_approval(p_approve=false); on reject of hire_agent it sets the subject agent to status='archived' WHERE pending_approval (0013:172,201) \u2014 the termination analogue. But no idempotency (non-pending raises 22023, 0013:163), no separate reject endpoint (shared /decide), no activity log. Core transition + hire termination reproduced with weaker/divergent semantics = partial correct.", "row_id": "approvals-system-governance-reject-lifecycle-effect", "verdict": "CONFIRMED", "waiver_ref": ""}
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.jsonl:110:{"acceptance_id": "ACC-06-APPROVALS-BUDGETS-COSTS-ACTIVITY-0110", "capability": "Request revision transition (board only, pending\u2192revision_requested)", "current_rewrite_evidence": "none found", "domain": "Approvals system & governance", "evidence_ref": "original_evidence + current_rewrite_evidence", "failing_test": "backend-rs/tests/approvals_parity.rs::approvals_system_governance_request_revision", "final_status": "unclosed", "initial_status": "missing", "original_behavior": "", "original_evidence": "", "passing_test": "cd backend-rs && cargo test --test approvals_parity", "phase": "phases/06-approvals-budgets-costs-activity.md", "review_note": "CONFIRMED absent. Verified original route exists (approvals.ts:318, service:205-224). Rewrite has no revision_requested status (0013:17), no request-revision route (routes.rs only has decide), no decisionNote column. Entirely absent.", "row_id": "approvals-system-governance-request-revision", "verdict": "CONFIRMED", "waiver_ref": ""}
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.jsonl:111:{"acceptance_id": "ACC-06-APPROVALS-BUDGETS-COSTS-ACTIVITY-0111", "capability": "requireApprovalAccess company scoping on decide", "current_rewrite_evidence": "supabase/migrations/paperclip/0013_agents_hiring.sql:159-162,187-191", "domain": "Approvals system & governance", "evidence_ref": "original_evidence + current_rewrite_evidence", "failing_test": "backend-rs/tests/approvals_parity.rs::approvals_system_governance_require_approval_access", "final_status": "full", "initial_status": "full", "original_behavior": "", "original_evidence": "", "passing_test": "cd backend-rs && cargo test --test approvals_parity", "phase": "phases/06-approvals-budgets-costs-activity.md", "review_note": "CONFIRMED with caveat. decide_approval loads the row FOR UPDATE (0013:159), raises P0002 if absent (0013:160), and gates fn_has_team_role on the APPROVAL's own team_id (0013:161); the key path additionally requires v_ap.team_id=k.team_id (0013:189). The security-relevant scoping \u2014 decision authority tied to membership in the approval's own company/team \u2014 is fully and arguably more strictly reproduced. Caveat (already noted by reviewer): missing\u2192404 actually surfaces as 502 (P0002 unmapped in routes.rs:440-467) and cross-team is 403 not 404; these are error-surface deltas, not scoping holes, so full stands.", "row_id": "approvals-system-governance-require-approval-access", "verdict": "CONFIRMED", "waiver_ref": ""}
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.jsonl:112:{"acceptance_id": "ACC-06-APPROVALS-BUDGETS-COSTS-ACTIVITY-0112", "capability": "Resubmit (revision_requested\u2192pending), requester-only", "current_rewrite_evidence": "none found", "domain": "Approvals system & governance", "evidence_ref": "original_evidence + current_rewrite_evidence", "failing_test": "backend-rs/tests/approvals_parity.rs::approvals_system_governance_resubmit", "final_status": "unclosed", "initial_status": "missing", "original_behavior": "", "original_evidence": "", "passing_test": "cd backend-rs && cargo test --test approvals_parity", "phase": "phases/06-approvals-budgets-costs-activity.md", "review_note": "CONFIRMED absent. Verified original route+service exist (approvals.ts:344; service:226-246). No resubmit route/RPC and no revision_requested state to resubmit from in the rewrite.", "row_id": "approvals-system-governance-resubmit", "verdict": "CONFIRMED", "waiver_ref": ""}
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.jsonl:113:{"acceptance_id": "ACC-06-APPROVALS-BUDGETS-COSTS-ACTIVITY-0113", "capability": "Cheap status-only recovery run cannot mutate approvals", "current_rewrite_evidence": "none found", "domain": "Approvals system & governance", "evidence_ref": "original_evidence + current_rewrite_evidence", "failing_test": "backend-rs/tests/approvals_parity.rs::approvals_system_governance_run_context_mutation_guard", "final_status": "unclosed", "initial_status": "missing", "original_behavior": "", "original_evidence": "", "passing_test": "cd backend-rs && cargo test --test approvals_parity", "phase": "phases/06-approvals-budgets-costs-activity.md", "review_note": "CONFIRMED absent. Verified original guard exists (approvals.ts:76 assertApprovalMutationAllowedByRunContext, referenced at 132/352/403). No heartbeat runs, run context snapshots, or recovery-profile concept anywhere in the rewrite \u2014 nothing to guard and no guard present.", "row_id": "approvals-system-governance-run-context-mutation-guard", "verdict": "CONFIRMED", "waiver_ref": ""}
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.jsonl:115:{"acceptance_id": "ACC-07-AGENT-RUNTIME-LIFECYCLE-0115", "capability": "Approval-gated hiring flow", "current_rewrite_evidence": "/home/frankji/Projects/paperclip/supabase/migrations/paperclip/0013_agents_hiring.sql:84-151 implements hire_agent and hire_agent_with_key with name/role/title/reports_to/model only; /home/frankji/Projects/paperclip/backend-rs/src/jobs/mod.rs:231-271 calls those RPCs; /home/frankji/Projects/paperclip/frontend-nuxt/composables/usePaperclipSession.ts:81-99 exposes only name/role/model.", "domain": "Agent lifecycle & status transitions", "evidence_ref": "original_evidence + current_rewrite_evidence", "failing_test": "backend-rs/tests/agent_lifecycle_parity.rs::agent_lifecycle_status_transitions_approval_gated_hiring_flow", "final_status": "unclosed", "initial_status": "partial", "original_behavior": "The rewrite has an approval-gated hire flow with session and API-key paths, same-company manager validation, and board-only approval decisions. It does not match original side effects and data richness: no adapterConfig/runtimeConfig/capabilities/budget/default task assignment grant/instructions bundle/secret binding parity, and approved status is active rather than idle.", "original_evidence": "/home/frankji/Projects/paperclip/server/src/routes/agents.ts:2194-2371 creates pending hire rows, redacts config into hire approval payload, links source issues, logs agent.hire_created and approval.created, applies default task assignment grant, and materializes default instructions; /home/frankji/Projects/paperclip/server/src/services/approvals.ts:125-180 activates pending hires, creates budget policy, and notifies hire approval.", "passing_test": "cd backend-rs && cargo test --test agent_lifecycle_parity", "phase": "phases/07-agent-runtime-lifecycle.md", "review_note": "The rewrite has an approval-gated hire flow with session and API-key paths, same-company manager validation, and board-only approval decisions. It does not match original side effects and data richness: no adapterConfig/runtimeConfig/capabilities/budget/default task assignment grant/instructions bundle/secret binding parity, and approved status is active rather than idle.", "row_id": "agent-lifecycle-status-transitions-approval-gated-hiring-flow", "verdict": "CONFIRMED", "waiver_ref": ""}
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.jsonl:118:{"acceptance_id": "ACC-07-AGENT-RUNTIME-LIFECYCLE-0118", "capability": "Company/auth boundary for lifecycle mutations", "current_rewrite_evidence": "/home/frankji/Projects/paperclip/supabase/migrations/paperclip/0011_companies_jobs.sql:51-68 enables RLS read policies for companies/jobs; /home/frankji/Projects/paperclip/supabase/migrations/paperclip/0012_api_key_scopes.sql:7-16 implements key-scope checks; /home/frankji/Projects/paperclip/supabase/migrations/paperclip/0013_agents_hiring.sql:153-206 implements board-only approval decisions and forbids agent approval decisions; /home/frankji/Projects/paperclip/backend-rs/src/supabase/routes.rs:440-467 maps many DB errors to 401/403/502 only.", "domain": "Agent lifecycle & status transitions", "evidence_ref": "original_evidence + current_rewrite_evidence", "failing_test": "backend-rs/tests/agent_lifecycle_parity.rs::agent_lifecycle_status_transitions_company_auth_boundary_for_lifecycle", "final_status": "unclosed", "initial_status": "partial", "original_behavior": "Rewrite has meaningful team-level RLS and forge-proof key RPCs, but lifecycle parity requires the original board-versus-agent route semantics for each transition. Because most transitions are missing, auth parity exists only for create/list/hire/approval/job slices. Rewrite errors are also coarse compared with original 409/422 state conflicts.", "original_evidence": "/home/frankji/Projects/paperclip/server/src/routes/agents.ts:674-684 requires board and agents:create for board management; /home/frankji/Projects/paperclip/server/src/routes/agents.ts:3279-3286 allows agent self-invoke but board management otherwise; /home/frankji/Projects/paperclip/server/src/routes/agents.ts:2948, 2974, 3005, 3037, 3091, and 3567 call assertBoard for privileged transitions.", "passing_test": "cd backend-rs && cargo test --test agent_lifecycle_parity", "phase": "phases/07-agent-runtime-lifecycle.md", "review_note": "Rewrite has meaningful team-level RLS and forge-proof key RPCs, but lifecycle parity requires the original board-versus-agent route semantics for each transition. Because most transitions are missing, auth parity exists only for create/list/hire/approval/job slices. Rewrite errors are also coarse compared with original 409/422 state conflicts.", "row_id": "agent-lifecycle-status-transitions-company-auth-boundary-for-lifecycle", "verdict": "CONFIRMED", "waiver_ref": ""}
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.jsonl:119:{"acceptance_id": "ACC-07-AGENT-RUNTIME-LIFECYCLE-0119", "capability": "Error state and clear-error", "current_rewrite_evidence": "/home/frankji/Projects/paperclip/supabase/migrations/paperclip/0013_agents_hiring.sql:10-12 omits error from agent_status; /home/frankji/Projects/paperclip/backend-rs/src/supabase/routes.rs:36-62 registers no clear-error endpoint.", "domain": "Agent lifecycle & status transitions", "evidence_ref": "original_evidence + current_rewrite_evidence", "failing_test": "backend-rs/tests/agent_lifecycle_parity.rs::agent_lifecycle_status_transitions_error_state_and_clear_error", "final_status": "unclosed", "initial_status": "missing", "original_behavior": "Rewrite has no error agent status and no clear-error endpoint. Original failed/timed-out runs put the agent into error with a bounded errorReason, and clear-error is board-only, only valid from error, returns to idle, and logs agent.error_cleared.", "original_evidence": "/home/frankji/Projects/paperclip/server/src/services/heartbeat.ts:7816-7854 stores errorReason and sets error on failed/timed_out; /home/frankji/Projects/paperclip/server/src/routes/agents.ts:3005-3035 implements POST /agents/:id/clear-error; /home/frankji/Projects/paperclip/server/src/services/agents.ts:592-620 enforces only error agents can be cleared and sets idle.", "passing_test": "cd backend-rs && cargo test --test agent_lifecycle_parity", "phase": "phases/07-agent-runtime-lifecycle.md", "review_note": "Rewrite has no error agent status and no clear-error endpoint. Original failed/timed-out runs put the agent into error with a bounded errorReason, and clear-error is board-only, only valid from error, returns to idle, and logs agent.error_cleared.", "row_id": "agent-lifecycle-status-transitions-error-state-and-clear-error", "verdict": "CONFIRMED", "waiver_ref": ""}
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.jsonl:121:{"acceptance_id": "ACC-07-AGENT-RUNTIME-LIFECYCLE-0121", "capability": "Heartbeat run, wakeup request, log/event, live-run, and cancellation model", "current_rewrite_evidence": "/home/frankji/Projects/paperclip/supabase/migrations/paperclip/0011_companies_jobs.sql:21-45 defines only job_status running/succeeded/failed and jobs table; /home/frankji/Projects/paperclip/backend-rs/src/supabase/routes.rs:47-48 exposes only /jobs for execution.", "domain": "Agent lifecycle & status transitions", "evidence_ref": "original_evidence + current_rewrite_evidence", "failing_test": "backend-rs/tests/agent_lifecycle_parity.rs::agent_lifecycle_status_transitions_heartbeat_run_wakeup_request_log_ev", "final_status": "unclosed", "initial_status": "missing", "original_behavior": "Rewrite job records are not parity for heartbeat runs. Original has queued/running/succeeded/failed/cancelled/timed_out heartbeat runs, wakeup requests, live-run listing, run detail, cancel, events, logs, workspace operations, and issue active-run lookup. Rewrite has only company jobs with running/succeeded/failed and synchronous completion/failure.", "original_evidence": "/home/frankji/Projects/paperclip/packages/shared/src/constants.ts:681-713 defines heartbeat invocation sources, wakeup request statuses, and heartbeat run statuses; /home/frankji/Projects/paperclip/server/src/routes/agents.ts:3453-3589 exposes heartbeat-runs, live-runs, run detail, and cancel; /home/frankji/Projects/paperclip/server/src/routes/agents.ts:3627-3667 exposes run events and logs.", "passing_test": "cd backend-rs && cargo test --test agent_lifecycle_parity", "phase": "phases/07-agent-runtime-lifecycle.md", "review_note": "Rewrite job records are not parity for heartbeat runs. Original has queued/running/succeeded/failed/cancelled/timed_out heartbeat runs, wakeup requests, live-run listing, run detail, cancel, events, logs, workspace operations, and issue active-run lookup. Rewrite has only company jobs with running/succeeded/failed and synchronous completion/failure.", "row_id": "agent-lifecycle-status-transitions-heartbeat-run-wakeup-request-log-ev", "verdict": "CONFIRMED", "waiver_ref": ""}
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.jsonl:123:{"acceptance_id": "ACC-07-AGENT-RUNTIME-LIFECYCLE-0123", "capability": "Pause agent", "current_rewrite_evidence": "/home/frankji/Projects/paperclip/backend-rs/src/supabase/routes.rs:36-62 registers no pause endpoint; /home/frankji/Projects/paperclip/backend-rs/src/bin/paperclip.rs:272-279 CLI usage lists no pause command; /home/frankji/Projects/paperclip/backend-rs/src/bin/paperclip-mcp.rs:28-56 lists no pause tool.", "domain": "Agent lifecycle & status transitions", "evidence_ref": "original_evidence + current_rewrite_evidence", "failing_test": "backend-rs/tests/agent_lifecycle_parity.rs::agent_lifecycle_status_transitions_pause_agent", "final_status": "unclosed", "initial_status": "missing", "original_behavior": "No rewrite route, CLI/MCP tool, frontend action, or RPC pauses an agent. Original pause is board-only, sets pause metadata, clears error reason, cancels active runs, and logs activity.", "original_evidence": "/home/frankji/Projects/paperclip/server/src/routes/agents.ts:2948-2972 implements POST /agents/:id/pause with assertBoard, heartbeat.cancelActiveForAgent, and agent.paused activity; /home/frankji/Projects/paperclip/server/src/services/agents.ts:549-567 sets status paused, pauseReason, pausedAt, clears errorReason.", "passing_test": "cd backend-rs && cargo test --test agent_lifecycle_parity", "phase": "phases/07-agent-runtime-lifecycle.md", "review_note": "No rewrite route, CLI/MCP tool, frontend action, or RPC pauses an agent. Original pause is board-only, sets pause metadata, clears error reason, cancels active runs, and logs activity.", "row_id": "agent-lifecycle-status-transitions-pause-agent", "verdict": "CONFIRMED", "waiver_ref": ""}
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.jsonl:124:{"acceptance_id": "ACC-07-AGENT-RUNTIME-LIFECYCLE-0124", "capability": "Rejecting pending hires / firing pending hires", "current_rewrite_evidence": "/home/frankji/Projects/paperclip/supabase/migrations/paperclip/0013_agents_hiring.sql:171-175 and 199-204 set pending agents to archived when approval is rejected.", "domain": "Agent lifecycle & status transitions", "evidence_ref": "original_evidence + current_rewrite_evidence", "failing_test": "backend-rs/tests/agent_lifecycle_parity.rs::agent_lifecycle_status_transitions_rejecting_pending_hires_firing_pend", "final_status": "unclosed", "initial_status": "divergent", "original_behavior": "Original rejection of a linked pending hire delegates to termination semantics, making the agent terminated and revoking API keys. Rewrite marks rejected hires archived, a status absent from original AGENT_STATUSES, and does not revoke keys/cancel invocations.", "original_evidence": "/home/frankji/Projects/paperclip/server/src/routes/agents.ts:3091-3115 delegates pending-hire termination through approvalsSvc.reject where possible; /home/frankji/Projects/paperclip/server/src/services/approvals.ts:186-199 rejects hire approvals by calling agentsSvc.terminate; /home/frankji/Projects/paperclip/server/src/services/agents.ts:622-642 sets terminated and revokes agent API keys.", "passing_test": "cd backend-rs && cargo test --test agent_lifecycle_parity", "phase": "phases/07-agent-runtime-lifecycle.md", "review_note": "Original rejection of a linked pending hire delegates to termination semantics, making the agent terminated and revoking API keys. Rewrite marks rejected hires archived, a status absent from original AGENT_STATUSES, and does not revoke keys/cancel invocations.", "row_id": "agent-lifecycle-status-transitions-rejecting-pending-hires-firing-pend", "verdict": "CONFIRMED", "waiver_ref": ""}
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.jsonl:125:{"acceptance_id": "ACC-07-AGENT-RUNTIME-LIFECYCLE-0125", "capability": "Resume agent", "current_rewrite_evidence": "/home/frankji/Projects/paperclip/backend-rs/src/supabase/routes.rs:36-62 registers no resume endpoint; /home/frankji/Projects/paperclip/frontend-nuxt/components/PaperclipConsole.vue:53-82 renders hiring and approvals only, with no resume control.", "domain": "Agent lifecycle & status transitions", "evidence_ref": "original_evidence + current_rewrite_evidence", "failing_test": "backend-rs/tests/agent_lifecycle_parity.rs::agent_lifecycle_status_transitions_resume_agent", "final_status": "unclosed", "initial_status": "missing", "original_behavior": "No rewrite surface resumes a paused agent. Original resume is board-only, rejects terminated and pending_approval agents, checks invalid org chains in the route, clears pause/error metadata, sets idle, and logs activity.", "original_evidence": "/home/frankji/Projects/paperclip/server/src/routes/agents.ts:2974-3003 implements board-only resume with invalid-org-chain conflict handling; /home/frankji/Projects/paperclip/server/src/services/agents.ts:569-590 sets status idle and clears pause/error fields.", "passing_test": "cd backend-rs && cargo test --test agent_lifecycle_parity", "phase": "phases/07-agent-runtime-lifecycle.md", "review_note": "No rewrite surface resumes a paused agent. Original resume is board-only, rejects terminated and pending_approval agents, checks invalid org chains in the route, clears pause/error metadata, sets idle, and logs activity.", "row_id": "agent-lifecycle-status-transitions-resume-agent", "verdict": "CONFIRMED", "waiver_ref": ""}
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.jsonl:126:{"acceptance_id": "ACC-07-AGENT-RUNTIME-LIFECYCLE-0126", "capability": "Runtime state, task sessions, and session reset", "current_rewrite_evidence": "/home/frankji/Projects/paperclip/backend-rs/src/supabase/routes.rs:36-62 has no runtime-state or task-session routes; /home/frankji/Projects/paperclip/frontend-nuxt/composables/usePaperclipSession.ts:76-125 exposes only hiring and approval calls for agents.", "domain": "Agent lifecycle & status transitions", "evidence_ref": "original_evidence + current_rewrite_evidence", "failing_test": "backend-rs/tests/agent_lifecycle_parity.rs::agent_lifecycle_status_transitions_runtime_state_task_sessions_and_ses", "final_status": "unclosed", "initial_status": "missing", "original_behavior": "Rewrite has no persisted runtime-state/session API. Original lets board users inspect runtime state and task sessions and reset saved sessions with activity logging.", "original_evidence": "/home/frankji/Projects/paperclip/server/src/routes/agents.ts:2129-2142 implements GET /agents/:id/runtime-state; /home/frankji/Projects/paperclip/server/src/routes/agents.ts:2144-2162 implements GET /agents/:id/task-sessions; /home/frankji/Projects/paperclip/server/src/routes/agents.ts:2164-2192 implements POST /agents/:id/runtime-state/reset-session and logs agent.runtime_session_reset.", "passing_test": "cd backend-rs && cargo test --test agent_lifecycle_parity", "phase": "phases/07-agent-runtime-lifecycle.md", "review_note": "Rewrite has no persisted runtime-state/session API. Original lets board users inspect runtime state and task sessions and reset saved sessions with activity logging.", "row_id": "agent-lifecycle-status-transitions-runtime-state-task-sessions-and-ses", "verdict": "CONFIRMED", "waiver_ref": ""}
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.jsonl:127:{"acceptance_id": "ACC-07-AGENT-RUNTIME-LIFECYCLE-0127", "capability": "Runtime transition idle \u2192 running \u2192 idle/error", "current_rewrite_evidence": "/home/frankji/Projects/paperclip/backend-rs/src/jobs/mod.rs:118-209 calls create_job/complete_job/fail_job RPCs; /home/frankji/Projects/paperclip/supabase/migrations/paperclip/0011_companies_jobs.sql:28-45 defines job status running/succeeded/failed separate from agents; no agent status update appears in these job RPCs at /home/frankji/Projects/paperclip/supabase/migrations/paperclip/0011_companies_jobs.sql:95-137 and 152-194.", "domain": "Agent lifecycle & status transitions", "evidence_ref": "original_evidence + current_rewrite_evidence", "failing_test": "backend-rs/tests/agent_lifecycle_parity.rs::agent_lifecycle_status_transitions_runtime_transition_idle_running_idl", "final_status": "unclosed", "initial_status": "missing", "original_behavior": "Rewrite does not mutate agent status during execution. Original changes the agent to running at execution start only if still invokable, then finalizes to idle for succeeded/cancelled runs or error for failed/timed_out runs, preserving running if another run remains active.", "original_evidence": "/home/frankji/Projects/paperclip/server/src/services/heartbeat.ts:9463-9506 atomically sets agent status running and publishes agent.status; /home/frankji/Projects/paperclip/server/src/services/heartbeat.ts:7823-7879 finalizes status to running, idle, or error and updates lastHeartbeatAt.", "passing_test": "cd backend-rs && cargo test --test agent_lifecycle_parity", "phase": "phases/07-agent-runtime-lifecycle.md", "review_note": "Rewrite does not mutate agent status during execution. Original changes the agent to running at execution start only if still invokable, then finalizes to idle for succeeded/cancelled runs or error for failed/timed_out runs, preserving running if another run remains active.", "row_id": "agent-lifecycle-status-transitions-runtime-transition-idle-running-idl", "verdict": "CONFIRMED", "waiver_ref": ""}
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.jsonl:128:{"acceptance_id": "ACC-07-AGENT-RUNTIME-LIFECYCLE-0128", "capability": "Terminate/fire active agents", "current_rewrite_evidence": "/home/frankji/Projects/paperclip/backend-rs/src/supabase/routes.rs:36-62 registers no terminate/fire endpoint; /home/frankji/Projects/paperclip/supabase/migrations/paperclip/0013_agents_hiring.sql:10-12 does not define terminated.", "domain": "Agent lifecycle & status transitions", "evidence_ref": "original_evidence + current_rewrite_evidence", "failing_test": "backend-rs/tests/agent_lifecycle_parity.rs::agent_lifecycle_status_transitions_terminate_fire_active_agents", "final_status": "unclosed", "initial_status": "missing", "original_behavior": "Rewrite has no general terminate/fire endpoint. Original termination is irreversible, board-only, revokes keys, cancels invocations for the agent and descendants invalidated by org-chain termination, and logs detailed activity.", "original_evidence": "/home/frankji/Projects/paperclip/server/src/routes/agents.ts:3091-3159 implements POST /agents/:id/terminate, cancels invocations for terminated agent plus invalid descendants, and logs agent.terminated; /home/frankji/Projects/paperclip/server/src/services/agents.ts:622-642 persists status terminated and revokes API keys.", "passing_test": "cd backend-rs && cargo test --test agent_lifecycle_parity", "phase": "phases/07-agent-runtime-lifecycle.md", "review_note": "Rewrite has no general terminate/fire endpoint. Original termination is irreversible, board-only, revokes keys, cancels invocations for the agent and descendants invalidated by org-chain termination, and logs detailed activity.", "row_id": "agent-lifecycle-status-transitions-terminate-fire-active-agents", "verdict": "CONFIRMED", "waiver_ref": ""}
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.jsonl:129:{"acceptance_id": "ACC-07-AGENT-RUNTIME-LIFECYCLE-0129", "capability": "Wakeup / heartbeat invoke", "current_rewrite_evidence": "/home/frankji/Projects/paperclip/backend-rs/src/supabase/routes.rs:36-62 registers no wakeup or heartbeat route; /home/frankji/Projects/paperclip/backend-rs/src/jobs/mod.rs:103-209 runs synchronous jobs instead of enqueueing heartbeat runs.", "domain": "Agent lifecycle & status transitions", "evidence_ref": "original_evidence + current_rewrite_evidence", "failing_test": "backend-rs/tests/agent_lifecycle_parity.rs::agent_lifecycle_status_transitions_wakeup_heartbeat_invoke", "final_status": "unclosed", "initial_status": "missing", "original_behavior": "Rewrite has no wakeup queue, heartbeat run creation, idempotency, trigger details, or agent self-invocation semantics. Original exposes both modern /wakeup and legacy /heartbeat/invoke with company access, agent self-only invoke, board management checks, org-chain checks, idempotency/payload/context handling, skipped responses, and heartbeat.invoked activity.", "original_evidence": "/home/frankji/Projects/paperclip/server/src/routes/agents.ts:3252-3335 implements shared wakeup route behavior and POST /agents/:id/wakeup; /home/frankji/Projects/paperclip/server/src/routes/agents.ts:3337-3419 implements legacy POST /agents/:id/heartbeat/invoke.", "passing_test": "cd backend-rs && cargo test --test agent_lifecycle_parity", "phase": "phases/07-agent-runtime-lifecycle.md", "review_note": "Rewrite has no wakeup queue, heartbeat run creation, idempotency, trigger details, or agent self-invocation semantics. Original exposes both modern /wakeup and legacy /heartbeat/invoke with company access, agent self-only invoke, board management checks, org-chain checks, idempotency/payload/context handling, skipped responses, and heartbeat.invoked activity.", "row_id": "agent-lifecycle-status-transitions-wakeup-heartbeat-invoke", "verdict": "CONFIRMED", "waiver_ref": ""}
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.jsonl:130:{"acceptance_id": "ACC-08-INTERFACES-REST-CLI-MCP-0130", "capability": "POST /api/agents/:id/approve", "current_rewrite_evidence": "supabase/migrations/paperclip/0013_agents_hiring.sql:154-177; backend-rs/src/supabase/routes.rs:58-61", "domain": "CLI / API surface (agent users)", "evidence_ref": "original_evidence + current_rewrite_evidence", "failing_test": "backend-rs/tests/api_surface_parity.rs::cli_api_surface_agent_users_api_agent_approve", "final_status": "unclosed", "initial_status": "partial", "original_behavior": "", "original_evidence": "", "passing_test": "cd backend-rs && cargo test --test api_surface_parity", "phase": "phases/08-interfaces-rest-cli-mcp.md", "review_note": "Verified: no /agents/:id/approve route. Effect exists but keyed on the approval: decide_approval(true) is board-only (0013:161), sets subject agent status='active' where 'pending_approval' (166), terminal on non-pending (163, errcode 22023). No findOpenHireApprovalForAgent, no agent.approved log. Partial.", "row_id": "cli-api-surface-agent-users-api-agent-approve", "verdict": "CONFIRMED", "waiver_ref": ""}
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.jsonl:132:{"acceptance_id": "ACC-08-INTERFACES-REST-CLI-MCP-0132", "capability": "Agent API key endpoints", "current_rewrite_evidence": "backend-rs/src/supabase/routes.rs:36-74; supabase/migrations/paperclip/0013_agents_hiring.sql:229-245", "domain": "CLI / API surface (agent users)", "evidence_ref": "original_evidence + current_rewrite_evidence", "failing_test": "backend-rs/tests/api_surface_parity.rs::cli_api_surface_agent_users_api_agent_keys", "final_status": "unclosed", "initial_status": "missing", "original_behavior": "", "original_evidence": "", "passing_test": "cd backend-rs && cargo test --test api_surface_parity", "phase": "phases/08-interfaces-rest-cli-mcp.md", "review_note": "Verified: the broker exposes no /keys routes (routes.rs has none). create_api_key RPC (0013:229-245) requires an active agent for agent-subject keys but is not surfaced to agent users via any HTTP endpoint or CLI verb. Missing.", "row_id": "cli-api-surface-agent-users-api-agent-keys", "verdict": "CONFIRMED", "waiver_ref": ""}
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.jsonl:133:{"acceptance_id": "ACC-08-INTERFACES-REST-CLI-MCP-0133", "capability": "pause/resume/clear-error/delete", "current_rewrite_evidence": "supabase/migrations/paperclip/0013_agents_hiring.sql:11; backend-rs/src/supabase/routes.rs:36-62", "domain": "CLI / API surface (agent users)", "evidence_ref": "original_evidence + current_rewrite_evidence", "failing_test": "backend-rs/tests/api_surface_parity.rs::cli_api_surface_agent_users_api_agent_lifecycle_simple", "final_status": "unclosed", "initial_status": "missing", "original_behavior": "", "original_evidence": "", "passing_test": "cd backend-rs && cargo test --test api_surface_parity", "phase": "phases/08-interfaces-rest-cli-mcp.md", "review_note": "Verified: no pause/resume/clear-error/delete endpoints or RPCs on the agent-user surface. agent_status enum includes 'paused' (0013:11) but nothing sets it; there is no error state. (The browser dashboard.rs merely counts paused/error agents; it does not mutate them.) Missing.", "row_id": "cli-api-surface-agent-users-api-agent-lifecycle-simple", "verdict": "CONFIRMED", "waiver_ref": ""}
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.jsonl:134:{"acceptance_id": "ACC-08-INTERFACES-REST-CLI-MCP-0134", "capability": "GET /api/agents/me + inbox", "current_rewrite_evidence": "backend-rs/src/supabase/routes.rs:33,159-161,469-502", "domain": "CLI / API surface (agent users)", "evidence_ref": "original_evidence + current_rewrite_evidence", "failing_test": "backend-rs/tests/api_surface_parity.rs::cli_api_surface_agent_users_api_agent_me", "final_status": "unclosed", "initial_status": "missing", "original_behavior": "", "original_evidence": "", "passing_test": "cd backend-rs && cargo test --test api_surface_parity", "phase": "phases/08-interfaces-rest-cli-mcp.md", "review_note": "Verified: no /agents/me endpoint, no trust-preset views, no inbox. Principal introspection exists only via GET /api/auth/session (routes.rs:33,159-161), which echoes raw key/user identity (principal_json 469-502), not an agent self-view. Missing.", "row_id": "cli-api-surface-agent-users-api-agent-me", "verdict": "CONFIRMED", "waiver_ref": ""}
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.jsonl:135:{"acceptance_id": "ACC-08-INTERFACES-REST-CLI-MCP-0135", "capability": "PATCH /api/agents/:id/permissions", "current_rewrite_evidence": "supabase/migrations/paperclip/0013_agents_hiring.sql:32,102; backend-rs/src/supabase/routes.rs:36-62 (no permissions route)", "domain": "CLI / API surface (agent users)", "evidence_ref": "original_evidence + current_rewrite_evidence", "failing_test": "backend-rs/tests/api_surface_parity.rs::cli_api_surface_agent_users_api_agent_permissions", "final_status": "unclosed", "initial_status": "missing", "original_behavior": "", "original_evidence": "", "passing_test": "cd backend-rs && cargo test --test api_surface_parity", "phase": "phases/08-interfaces-rest-cli-mcp.md", "review_note": "Verified: no permissions endpoint/RPC. can_create_agents is the only permission, set once at hire and grantable only by the board (0013:102); no post-hire mutation path, no CEO gate, no tasks:assign membership. Missing.", "row_id": "cli-api-surface-agent-users-api-agent-permissions", "verdict": "CONFIRMED", "waiver_ref": ""}
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.jsonl:137:{"acceptance_id": "ACC-08-INTERFACES-REST-CLI-MCP-0137", "capability": "POST /api/agents/:id/wakeup + /heartbeat/invoke", "current_rewrite_evidence": "backend-rs/src/supabase/routes.rs:36-62; backend-rs/src/jobs/mod.rs:103-209", "domain": "CLI / API surface (agent users)", "evidence_ref": "original_evidence + current_rewrite_evidence", "failing_test": "backend-rs/tests/api_surface_parity.rs::cli_api_surface_agent_users_api_agent_wakeup", "final_status": "unclosed", "initial_status": "missing", "original_behavior": "", "original_evidence": "", "passing_test": "cd backend-rs && cargo test --test api_surface_parity", "phase": "phases/08-interfaces-rest-cli-mcp.md", "review_note": "Verified: no wakeup/heartbeat routes or RPCs. Work is dispatched via synchronous jobs (run_job, jobs/mod.rs:103-209). Missing.", "row_id": "cli-api-surface-agent-users-api-agent-wakeup", "verdict": "CONFIRMED", "waiver_ref": ""}
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.jsonl:139:{"acceptance_id": "ACC-08-INTERFACES-REST-CLI-MCP-0139", "capability": "POST /api/companies/:companyId/approvals", "current_rewrite_evidence": "backend-rs/src/supabase/routes.rs:54-61 (only list+decide); supabase/migrations/paperclip/0013_agents_hiring.sql:14,107-110", "domain": "CLI / API surface (agent users)", "evidence_ref": "original_evidence + current_rewrite_evidence", "failing_test": "backend-rs/tests/api_surface_parity.rs::cli_api_surface_agent_users_api_approvals_create", "final_status": "unclosed", "initial_status": "missing", "original_behavior": "", "original_evidence": "", "passing_test": "cd backend-rs && cargo test --test api_surface_parity", "phase": "phases/08-interfaces-rest-cli-mcp.md", "review_note": "Verified: no generic approval-create endpoint. Approvals are created only implicitly as a side effect of hire_agent (0013:107-110). approve_ceo_strategy is a declared enum value (0013:14) but nothing can create such an approval. Missing.", "row_id": "cli-api-surface-agent-users-api-approvals-create", "verdict": "CONFIRMED", "waiver_ref": ""}
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.jsonl:140:{"acceptance_id": "ACC-08-INTERFACES-REST-CLI-MCP-0140", "capability": "approve/reject/request-revision", "current_rewrite_evidence": "backend-rs/src/supabase/routes.rs:58-61; supabase/migrations/paperclip/0013_agents_hiring.sql:154-206", "domain": "CLI / API surface (agent users)", "evidence_ref": "original_evidence + current_rewrite_evidence", "failing_test": "backend-rs/tests/api_surface_parity.rs::cli_api_surface_agent_users_api_approvals_decisions", "final_status": "unclosed", "initial_status": "partial", "original_behavior": "", "original_evidence": "", "passing_test": "cd backend-rs && cargo test --test api_surface_parity", "phase": "phases/08-interfaces-rest-cli-mcp.md", "review_note": "Verified: POST /approvals/:id/decide {approve:bool} (routes.rs:58-61 -> decide_approval / decide_approval_with_key, 0013:154-206) covers approve+reject only, board-only, terminal, records decided_by/decided_at. request-revision absent (no revision_requested status, 0013:17); the post-approval requester wakeup side effect is entirely missing; agent-subject keys can never decide (0013:186). Partial.", "row_id": "cli-api-surface-agent-users-api-approvals-decisions", "verdict": "CONFIRMED", "waiver_ref": ""}
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.jsonl:142:{"acceptance_id": "ACC-08-INTERFACES-REST-CLI-MCP-0142", "capability": "resubmit + comments", "current_rewrite_evidence": "supabase/migrations/paperclip/0013_agents_hiring.sql:16-17,154-206", "domain": "CLI / API surface (agent users)", "evidence_ref": "original_evidence + current_rewrite_evidence", "failing_test": "backend-rs/tests/api_surface_parity.rs::cli_api_surface_agent_users_api_approvals_resubmit_comment", "final_status": "unclosed", "initial_status": "missing", "original_behavior": "", "original_evidence": "", "passing_test": "cd backend-rs && cargo test --test api_surface_parity", "phase": "phases/08-interfaces-rest-cli-mcp.md", "review_note": "Verified: no resubmit and no approval comments in schema, broker, or CLI. Rejected approvals are terminal (approval_status enum has no revision path, 0013:17). Missing.", "row_id": "cli-api-surface-agent-users-api-approvals-resubmit-comment", "verdict": "CONFIRMED", "waiver_ref": ""}
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.jsonl:144:{"acceptance_id": "ACC-08-INTERFACES-REST-CLI-MCP-0144", "capability": "config-read authorization", "current_rewrite_evidence": "backend-rs/src/supabase/routes.rs:36-62 (no config/skills routes)", "domain": "CLI / API surface (agent users)", "evidence_ref": "original_evidence + current_rewrite_evidence", "failing_test": "backend-rs/tests/api_surface_parity.rs::cli_api_surface_agent_users_authz_read_config", "final_status": "unclosed", "initial_status": "missing", "original_behavior": "", "original_evidence": "", "passing_test": "cd backend-rs && cargo test --test api_surface_parity", "phase": "phases/08-interfaces-rest-cli-mcp.md", "review_note": "Verified: no configuration/skills/config-revisions read surface on the agent-user broker, hence no corresponding authorization layer. (A company skills read surface exists on the separate browser app, lib.rs:507-513, but it is not agent-user reachable.) Missing.", "row_id": "cli-api-surface-agent-users-authz-read-config", "verdict": "CONFIRMED", "waiver_ref": ""}
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.jsonl:145:{"acceptance_id": "ACC-08-INTERFACES-REST-CLI-MCP-0145", "capability": "CLI `agent create`", "current_rewrite_evidence": "backend-rs/src/bin/paperclip.rs:490-519; backend-rs/src/supabase/routes.rs:51-53", "domain": "CLI / API surface (agent users)", "evidence_ref": "original_evidence + current_rewrite_evidence", "failing_test": "backend-rs/tests/api_surface_parity.rs::cli_api_surface_agent_users_cli_agent_create", "final_status": "unclosed", "initial_status": "missing", "original_behavior": "", "original_evidence": "", "passing_test": "cd backend-rs && cargo test --test api_surface_parity", "phase": "phases/08-interfaces-rest-cli-mcp.md", "review_note": "Verified: the `agent` match arm (paperclip.rs:490-519) exposes only `hire` and `list`; there is no direct-create verb. The single POST /agents always routes through the approval-gated hire_agent RPC. Missing stands.", "row_id": "cli-api-surface-agent-users-cli-agent-create", "verdict": "CONFIRMED", "waiver_ref": ""}
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.jsonl:148:{"acceptance_id": "ACC-08-INTERFACES-REST-CLI-MCP-0148", "capability": "CLI agent inbox commands", "current_rewrite_evidence": "backend-rs/src/bin/paperclip.rs:490-519 (no inbox verb); backend-rs/src/inbox.rs:1-50 + lib.rs:577 (separate browser surface only)", "domain": "CLI / API surface (agent users)", "evidence_ref": "original_evidence + current_rewrite_evidence", "failing_test": "backend-rs/tests/api_surface_parity.rs::cli_api_surface_agent_users_cli_agent_inbox", "final_status": "unclosed", "initial_status": "missing", "original_behavior": "", "original_evidence": "", "passing_test": "cd backend-rs && cargo test --test api_surface_parity", "phase": "phases/08-interfaces-rest-cli-mcp.md", "review_note": "Agent-user status confirmed missing: no inbox verb/endpoint on the broker or CLI. NOTE the note's 'no inbox concept anywhere in the rewrite / no issues/tasks tables' is overbroad: the separate browser-facing app has an inbox module (backend-rs/src/inbox.rs; route /api/companies/:id/inbox-dismissals lib.rs:577) and an IssueRepo (lib.rs app_with_repos). Those are Actor/browser-user routes, unrelated to the original /agents/me/inbox-lite and unreachable by agent users, so the agent-user verdict (missing) still holds.", "row_id": "cli-api-surface-agent-users-cli-agent-inbox", "verdict": "CONFIRMED", "waiver_ref": ""}
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.jsonl:149:{"acceptance_id": "ACC-08-INTERFACES-REST-CLI-MCP-0149", "capability": "CLI instructions path/bundle/file", "current_rewrite_evidence": "backend-rs/src/bin/paperclip.rs:490-519; supabase/migrations/paperclip/0013_agents_hiring.sql:21-38 (agents table has no instructions columns)", "domain": "CLI / API surface (agent users)", "evidence_ref": "original_evidence + current_rewrite_evidence", "failing_test": "backend-rs/tests/api_surface_parity.rs::cli_api_surface_agent_users_cli_agent_instructions", "final_status": "unclosed", "initial_status": "missing", "original_behavior": "", "original_evidence": "", "passing_test": "cd backend-rs && cargo test --test api_surface_parity", "phase": "phases/08-interfaces-rest-cli-mcp.md", "review_note": "Verified: no instructions path/bundle/file concept in the migrations (0011-0013), broker (routes.rs), or CLI. Missing.", "row_id": "cli-api-surface-agent-users-cli-agent-instructions", "verdict": "CONFIRMED", "waiver_ref": ""}
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.jsonl:150:{"acceptance_id": "ACC-08-INTERFACES-REST-CLI-MCP-0150", "capability": "CLI agent lifecycle verbs", "current_rewrite_evidence": "backend-rs/src/bin/paperclip.rs:490-545", "domain": "CLI / API surface (agent users)", "evidence_ref": "original_evidence + current_rewrite_evidence", "failing_test": "backend-rs/tests/api_surface_parity.rs::cli_api_surface_agent_users_cli_agent_lifecycle", "final_status": "unclosed", "initial_status": "missing", "original_behavior": "", "original_evidence": "", "passing_test": "cd backend-rs && cargo test --test api_surface_parity", "phase": "phases/08-interfaces-rest-cli-mcp.md", "review_note": "Verified: no agent-scoped lifecycle verbs. approve/reject exist only as `approval approve|reject` (paperclip.rs:520-545), not per-agent; no pause/resume/terminate/heartbeat/claude-login anywhere in the CLI or broker. Missing.", "row_id": "cli-api-surface-agent-users-cli-agent-lifecycle", "verdict": "CONFIRMED", "waiver_ref": ""}
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.jsonl:151:{"acceptance_id": "ACC-08-INTERFACES-REST-CLI-MCP-0151", "capability": "CLI `agent local-cli`", "current_rewrite_evidence": "backend-rs/src/bin/paperclip.rs:219-270,490-519; supabase/migrations/paperclip/0013_agents_hiring.sql:229-245", "domain": "CLI / API surface (agent users)", "evidence_ref": "original_evidence + current_rewrite_evidence", "failing_test": "backend-rs/tests/api_surface_parity.rs::cli_api_surface_agent_users_cli_agent_local_cli", "final_status": "unclosed", "initial_status": "missing", "original_behavior": "", "original_evidence": "", "passing_test": "cd backend-rs && cargo test --test api_surface_parity", "phase": "phases/08-interfaces-rest-cli-mcp.md", "review_note": "Verified: no local-cli verb, no skill symlinking. `login` (paperclip.rs:219-270) mints a team device-login key, not a per-agent key. create_api_key RPC (0013:229-245) exists but is not surfaced to agents via any broker route or CLI verb. Missing.", "row_id": "cli-api-surface-agent-users-cli-agent-local-cli", "verdict": "CONFIRMED", "waiver_ref": ""}
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.jsonl:152:{"acceptance_id": "ACC-08-INTERFACES-REST-CLI-MCP-0152", "capability": "CLI permissions/configuration/revisions", "current_rewrite_evidence": "backend-rs/src/bin/paperclip.rs:490-519; backend-rs/src/company_skills.rs + lib.rs:507-513 (browser surface only)", "domain": "CLI / API surface (agent users)", "evidence_ref": "original_evidence + current_rewrite_evidence", "failing_test": "backend-rs/tests/api_surface_parity.rs::cli_api_surface_agent_users_cli_agent_permissions_config", "final_status": "unclosed", "initial_status": "missing", "original_behavior": "", "original_evidence": "", "passing_test": "cd backend-rs && cargo test --test api_surface_parity", "phase": "phases/08-interfaces-rest-cli-mcp.md", "review_note": "Agent-user status confirmed missing: none of permissions:update/configuration/config-revisions/runtime-state/reset-session/task-sessions/skills[:sync] exist on the CLI or broker. NOTE 'no skills machinery' is overbroad \u2014 a company skills library exists on the browser surface (backend-rs/src/company_skills.rs; routes /api/companies/:id/skills lib.rs:507-513) \u2014 but it is not agent-scoped and not agent-user reachable, so missing stands for this domain.", "row_id": "cli-api-surface-agent-users-cli-agent-permissions-config", "verdict": "CONFIRMED", "waiver_ref": ""}
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.jsonl:153:{"acceptance_id": "ACC-08-INTERFACES-REST-CLI-MCP-0153", "capability": "CLI `agent wake`", "current_rewrite_evidence": "backend-rs/src/bin/paperclip.rs:416-489; backend-rs/src/jobs/mod.rs:103-209", "domain": "CLI / API surface (agent users)", "evidence_ref": "original_evidence + current_rewrite_evidence", "failing_test": "backend-rs/tests/api_surface_parity.rs::cli_api_surface_agent_users_cli_agent_wake", "final_status": "unclosed", "initial_status": "missing", "original_behavior": "", "original_evidence": "", "passing_test": "cd backend-rs && cargo test --test api_surface_parity", "phase": "phases/08-interfaces-rest-cli-mcp.md", "review_note": "Verified: no wakeup/heartbeat verb; the agent-user unit of work is a synchronous job (`job run` -> run_job jobs/mod.rs:103-209). Missing.", "row_id": "cli-api-surface-agent-users-cli-agent-wake", "verdict": "CONFIRMED", "waiver_ref": ""}
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.jsonl:160:{"acceptance_id": "ACC-08-INTERFACES-REST-CLI-MCP-0160", "capability": "Default tasks:assign grant on hire/create", "current_rewrite_evidence": "supabase/migrations/paperclip/0013_agents_hiring.sql:104-106,142-144; backend-rs/src/lib.rs (resource-memberships routes, browser surface only)", "domain": "CLI / API surface (agent users)", "evidence_ref": "original_evidence + current_rewrite_evidence", "failing_test": "backend-rs/tests/api_surface_parity.rs::cli_api_surface_agent_users_side_effect_task_assign_grant", "final_status": "unclosed", "initial_status": "missing", "original_behavior": "", "original_evidence": "", "passing_test": "cd backend-rs && cargo test --test api_surface_parity", "phase": "phases/08-interfaces-rest-cli-mcp.md", "review_note": "Verified: hire_agent/hire_agent_with_key (0013:104-106,142-144) insert only the agent row \u2014 no tasks:assign grant, no agent membership row, no task-assignment concept in the audited surface. NOTE the browser app does have resource-membership routes incl. /resource-memberships/me/agents/:agentId (lib.rs) and an IssueRepo, so 'no task-assignment concept in the rewrite' is overbroad \u2014 but nothing wires a tasks:assign grant on hire and none of it is agent-user reachable, so missing stands.", "row_id": "cli-api-surface-agent-users-side-effect-task-assign-grant", "verdict": "CONFIRMED", "waiver_ref": ""}
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.jsonl:161:{"acceptance_id": "ACC-09-NUXT-BOARD-UI-PARITY-0161", "capability": "Assign Task action", "current_rewrite_evidence": "backend-rs/src/supabase/routes.rs:36-62 (no issues routes)", "domain": "UI surface (human non-agent users)", "evidence_ref": "original_evidence + current_rewrite_evidence", "failing_test": "frontend-nuxt/tests/ui-parity.test.ts::ui_surface_human_non_agent_users_agent_action_assign_task", "final_status": "unclosed", "initial_status": "missing", "original_behavior": "", "original_evidence": "", "passing_test": "cd frontend-nuxt && npx vitest run tests/ui-parity.test.ts", "phase": "phases/09-nuxt-board-ui-parity.md", "review_note": "Confirmed missing. No issues/tasks concept, no New Issue dialog, no assigneeAgentId anywhere in the rewrite.", "row_id": "ui-surface-human-non-agent-users-agent-action-assign-task", "verdict": "CONFIRMED", "waiver_ref": ""}
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.jsonl:162:{"acceptance_id": "ACC-09-NUXT-BOARD-UI-PARITY-0162", "capability": "Clear error action", "current_rewrite_evidence": "supabase/migrations/paperclip/0013_agents_hiring.sql:11", "domain": "UI surface (human non-agent users)", "evidence_ref": "original_evidence + current_rewrite_evidence", "failing_test": "frontend-nuxt/tests/ui-parity.test.ts::ui_surface_human_non_agent_users_agent_action_clear_error", "final_status": "unclosed", "initial_status": "missing", "original_behavior": "", "original_evidence": "", "passing_test": "cd frontend-nuxt && npx vitest run tests/ui-parity.test.ts", "phase": "phases/09-nuxt-board-ui-parity.md", "review_note": "Confirmed missing. agent_status enum has no 'error' or 'idle' value (only pending_approval/active/paused/archived, 0013:11); no clear-error RPC or UI.", "row_id": "ui-surface-human-non-agent-users-agent-action-clear-error", "verdict": "CONFIRMED", "waiver_ref": ""}
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.jsonl:163:{"acceptance_id": "ACC-09-NUXT-BOARD-UI-PARITY-0163", "capability": "Run heartbeat / invoke action", "current_rewrite_evidence": "backend-rs/src/jobs/mod.rs:103-177", "domain": "UI surface (human non-agent users)", "evidence_ref": "original_evidence + current_rewrite_evidence", "failing_test": "frontend-nuxt/tests/ui-parity.test.ts::ui_surface_human_non_agent_users_agent_action_invoke", "final_status": "unclosed", "initial_status": "missing", "original_behavior": "", "original_evidence": "", "passing_test": "cd frontend-nuxt && npx vitest run tests/ui-parity.test.ts", "phase": "phases/09-nuxt-board-ui-parity.md", "review_note": "Confirmed missing. Jobs are created against a company_id (jobs/mod.rs:103-177, run_paperclip_job route), never invoked on an agent; no heartbeat/invoke endpoint and no runs to navigate to.", "row_id": "ui-surface-human-non-agent-users-agent-action-invoke", "verdict": "CONFIRMED", "waiver_ref": ""}
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.jsonl:164:{"acceptance_id": "ACC-09-NUXT-BOARD-UI-PARITY-0164", "capability": "Overflow menu: duplicate / copy ID / reset sessions / terminate", "current_rewrite_evidence": "frontend-nuxt/components/PaperclipConsole.vue:60-64", "domain": "UI surface (human non-agent users)", "evidence_ref": "original_evidence + current_rewrite_evidence", "failing_test": "frontend-nuxt/tests/ui-parity.test.ts::ui_surface_human_non_agent_users_agent_action_overflow_menu", "final_status": "unclosed", "initial_status": "missing", "original_behavior": "", "original_evidence": "", "passing_test": "cd frontend-nuxt && npx vitest run tests/ui-parity.test.ts", "phase": "phases/09-nuxt-board-ui-parity.md", "review_note": "Confirmed missing. No duplicate, reset-session, or terminate RPC/route and no runtime-state model; the console has no per-agent action controls (PaperclipConsole.vue:60-64).", "row_id": "ui-surface-human-non-agent-users-agent-action-overflow-menu", "verdict": "CONFIRMED", "waiver_ref": ""}
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.jsonl:165:{"acceptance_id": "ACC-09-NUXT-BOARD-UI-PARITY-0165", "capability": "Pause / Resume action", "current_rewrite_evidence": "supabase/migrations/paperclip/0013_agents_hiring.sql:11", "domain": "UI surface (human non-agent users)", "evidence_ref": "original_evidence + current_rewrite_evidence", "failing_test": "frontend-nuxt/tests/ui-parity.test.ts::ui_surface_human_non_agent_users_agent_action_pause_resume", "final_status": "unclosed", "initial_status": "missing", "original_behavior": "", "original_evidence": "", "passing_test": "cd frontend-nuxt && npx vitest run tests/ui-parity.test.ts", "phase": "phases/09-nuxt-board-ui-parity.md", "review_note": "Confirmed missing. agent_status has a 'paused' value (0013:11) but no pause/resume RPC (none in 0013) and no UI control to reach it.", "row_id": "ui-surface-human-non-agent-users-agent-action-pause-resume", "verdict": "CONFIRMED", "waiver_ref": ""}
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.jsonl:166:{"acceptance_id": "ACC-09-NUXT-BOARD-UI-PARITY-0166", "capability": "Per-agent budget policy", "current_rewrite_evidence": "supabase/migrations/paperclip/0013_agents_hiring.sql (no budget entities)", "domain": "UI surface (human non-agent users)", "evidence_ref": "original_evidence + current_rewrite_evidence", "failing_test": "frontend-nuxt/tests/ui-parity.test.ts::ui_surface_human_non_agent_users_agent_budget_tab", "final_status": "unclosed", "initial_status": "missing", "original_behavior": "", "original_evidence": "", "passing_test": "cd frontend-nuxt && npx vitest run tests/ui-parity.test.ts", "phase": "phases/09-nuxt-board-ui-parity.md", "review_note": "Confirmed missing. No budgets/budget-policy table, RPC, or UI in the rewrite.", "row_id": "ui-surface-human-non-agent-users-agent-budget-tab", "verdict": "CONFIRMED", "waiver_ref": ""}
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.jsonl:167:{"acceptance_id": "ACC-09-NUXT-BOARD-UI-PARITY-0167", "capability": "Run log/env secret + path redaction in UI", "current_rewrite_evidence": "backend-rs/src/jobs/mod.rs:369-391", "domain": "UI surface (human non-agent users)", "evidence_ref": "original_evidence + current_rewrite_evidence", "failing_test": "frontend-nuxt/tests/ui-parity.test.ts::ui_surface_human_non_agent_users_agent_config_secret_redaction", "final_status": "unclosed", "initial_status": "missing", "original_behavior": "", "original_evidence": "", "passing_test": "cd frontend-nuxt && npx vitest run tests/ui-parity.test.ts", "phase": "phases/09-nuxt-board-ui-parity.md", "review_note": "Confirmed missing. There is no run invocation/env/log rendering (jobs return only result text + token usage, jobs/mod.rs:369-391), so there is no redaction layer.", "row_id": "ui-surface-human-non-agent-users-agent-config-secret-redaction", "verdict": "CONFIRMED", "waiver_ref": ""}
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.jsonl:168:{"acceptance_id": "ACC-09-NUXT-BOARD-UI-PARITY-0168", "capability": "Agent dashboard overview", "current_rewrite_evidence": "supabase/migrations/paperclip/0011_companies_jobs.sql:28-45", "domain": "UI surface (human non-agent users)", "evidence_ref": "original_evidence + current_rewrite_evidence", "failing_test": "frontend-nuxt/tests/ui-parity.test.ts::ui_surface_human_non_agent_users_agent_dashboard_overview", "final_status": "unclosed", "initial_status": "missing", "original_behavior": "", "original_evidence": "", "passing_test": "cd frontend-nuxt && npx vitest run tests/ui-parity.test.ts", "phase": "phases/09-nuxt-board-ui-parity.md", "review_note": "Confirmed missing. No dashboard, charts, tasks, runs, or cost data exist; jobs carry only a usage jsonb (0011:39) never charted.", "row_id": "ui-surface-human-non-agent-users-agent-dashboard-overview", "verdict": "CONFIRMED", "waiver_ref": ""}
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.jsonl:169:{"acceptance_id": "ACC-09-NUXT-BOARD-UI-PARITY-0169", "capability": "Agent detail tabbed page", "current_rewrite_evidence": "frontend-nuxt/components/PaperclipConsole.vue:60-64", "domain": "UI surface (human non-agent users)", "evidence_ref": "original_evidence + current_rewrite_evidence", "failing_test": "frontend-nuxt/tests/ui-parity.test.ts::ui_surface_human_non_agent_users_agent_detail_tabs", "final_status": "unclosed", "initial_status": "missing", "original_behavior": "", "original_evidence": "", "passing_test": "cd frontend-nuxt && npx vitest run tests/ui-parity.test.ts", "phase": "phases/09-nuxt-board-ui-parity.md", "review_note": "Confirmed missing. There are no frontend routes/pages at all; agents surface only as list items in PaperclipConsole.vue:60-64. No Dashboard/Instructions/Skills/Configuration/Runs/Budget tabs, no editable header.", "row_id": "ui-surface-human-non-agent-users-agent-detail-tabs", "verdict": "CONFIRMED", "waiver_ref": ""}
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.jsonl:170:{"acceptance_id": "ACC-09-NUXT-BOARD-UI-PARITY-0170", "capability": "Edit agent icon", "current_rewrite_evidence": "supabase/migrations/paperclip/0013_agents_hiring.sql:21-38", "domain": "UI surface (human non-agent users)", "evidence_ref": "original_evidence + current_rewrite_evidence", "failing_test": "frontend-nuxt/tests/ui-parity.test.ts::ui_surface_human_non_agent_users_agent_icon_edit", "final_status": "unclosed", "initial_status": "missing", "original_behavior": "", "original_evidence": "", "passing_test": "cd frontend-nuxt && npx vitest run tests/ui-parity.test.ts", "phase": "phases/09-nuxt-board-ui-parity.md", "review_note": "Confirmed missing. agents table has no icon column (0013:21-38) and there is no PATCH /agents/{id} update route (routes.rs:36-62).", "row_id": "ui-surface-human-non-agent-users-agent-icon-edit", "verdict": "CONFIRMED", "waiver_ref": ""}
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.jsonl:171:{"acceptance_id": "ACC-09-NUXT-BOARD-UI-PARITY-0171", "capability": "Invalid reporting-chain warning + gating", "current_rewrite_evidence": "supabase/migrations/paperclip/0013_agents_hiring.sql:99-100", "domain": "UI surface (human non-agent users)", "evidence_ref": "original_evidence + current_rewrite_evidence", "failing_test": "frontend-nuxt/tests/ui-parity.test.ts::ui_surface_human_non_agent_users_agent_invalid_org_chain", "final_status": "unclosed", "initial_status": "missing", "original_behavior": "", "original_evidence": "", "passing_test": "cd frontend-nuxt && npx vitest run tests/ui-parity.test.ts", "phase": "phases/09-nuxt-board-ui-parity.md", "review_note": "Confirmed missing. hire_agent only checks reports_to is an agent in the same company (0013:99-100); there is no chain-validity/cycle computation, no invalid_org_chain banner, and no gating of assign/run.", "row_id": "ui-surface-human-non-agent-users-agent-invalid-org-chain", "verdict": "CONFIRMED", "waiver_ref": ""}
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.jsonl:172:{"acceptance_id": "ACC-09-NUXT-BOARD-UI-PARITY-0172", "capability": "Join/leave agent membership", "current_rewrite_evidence": "supabase/migrations/paperclip/0013_agents_hiring.sql:21-38 (no membership)", "domain": "UI surface (human non-agent users)", "evidence_ref": "original_evidence + current_rewrite_evidence", "failing_test": "frontend-nuxt/tests/ui-parity.test.ts::ui_surface_human_non_agent_users_agent_membership_join_leave", "final_status": "unclosed", "initial_status": "missing", "original_behavior": "", "original_evidence": "", "passing_test": "cd frontend-nuxt && npx vitest run tests/ui-parity.test.ts", "phase": "phases/09-nuxt-board-ui-parity.md", "review_note": "Confirmed missing. No per-agent membership model, join/leave RPC, or left-agent/rejoin UI anywhere in the rewrite.", "row_id": "ui-surface-human-non-agent-users-agent-membership-join-leave", "verdict": "CONFIRMED", "waiver_ref": ""}
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.jsonl:174:{"acceptance_id": "ACC-09-NUXT-BOARD-UI-PARITY-0174", "capability": "Update agent permissions/trust", "current_rewrite_evidence": "supabase/migrations/paperclip/0013_agents_hiring.sql:102", "domain": "UI surface (human non-agent users)", "evidence_ref": "original_evidence + current_rewrite_evidence", "failing_test": "frontend-nuxt/tests/ui-parity.test.ts::ui_surface_human_non_agent_users_agent_permissions_update", "final_status": "unclosed", "initial_status": "missing", "original_behavior": "", "original_evidence": "", "passing_test": "cd frontend-nuxt && npx vitest run tests/ui-parity.test.ts", "phase": "phases/09-nuxt-board-ui-parity.md", "review_note": "Confirmed missing. permissions are set once at hire (can_create_agents only, 0013:102) with no update RPC and no permissions route/UI.", "row_id": "ui-surface-human-non-agent-users-agent-permissions-update", "verdict": "CONFIRMED", "waiver_ref": ""}
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.jsonl:175:{"acceptance_id": "ACC-09-NUXT-BOARD-UI-PARITY-0175", "capability": "Agents REST API client surface", "current_rewrite_evidence": "frontend-nuxt/composables/usePaperclipSession.ts:83-125; backend-rs/src/supabase/routes.rs:50-62", "domain": "UI surface (human non-agent users)", "evidence_ref": "original_evidence + current_rewrite_evidence", "failing_test": "frontend-nuxt/tests/ui-parity.test.ts::ui_surface_human_non_agent_users_agents_api_surface", "final_status": "unclosed", "initial_status": "partial", "original_behavior": "", "original_evidence": "", "passing_test": "cd frontend-nuxt && npx vitest run tests/ui-parity.test.ts", "phase": "phases/09-nuxt-board-ui-parity.md", "review_note": "Confirmed partial. usePaperclipSession.ts:83-125 and routes.rs:50-62 expose only listAgents (GET .../agents), hireAgent (POST .../agents), listApprovals, and decideApproval. Missing: org, get-by-shortname, create, update, updatePermissions, pause/resume/clearError/per-agent approve/terminate/remove, invoke, resetSession, runtimeState, instructions, keys, skills, and all adapter-model endpoints. A thin slice exists -> partial.", "row_id": "ui-surface-human-non-agent-users-agents-api-surface", "verdict": "CONFIRMED", "waiver_ref": ""}
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.jsonl:176:{"acceptance_id": "ACC-09-NUXT-BOARD-UI-PARITY-0176", "capability": "Agents list (list view)", "current_rewrite_evidence": "frontend-nuxt/components/PaperclipConsole.vue:60-64", "domain": "UI surface (human non-agent users)", "evidence_ref": "original_evidence + current_rewrite_evidence", "failing_test": "frontend-nuxt/tests/ui-parity.test.ts::ui_surface_human_non_agent_users_agents_list_view", "final_status": "unclosed", "initial_status": "partial", "original_behavior": "", "original_evidence": "", "passing_test": "cd frontend-nuxt && npx vitest run tests/ui-parity.test.ts", "phase": "phases/09-nuxt-board-ui-parity.md", "review_note": "Confirmed. PaperclipConsole.vue:60-64 renders a bare <li> of name + status only, over the unfiltered my_agents view (jobs/mod.rs:274-292). No role/title subtitle, model/adapter/env/heartbeat meta, live-run chip, action buttons, membership, or dimming of pending_approval rows. A minimal list exists -> partial.", "row_id": "ui-surface-human-non-agent-users-agents-list-view", "verdict": "CONFIRMED", "waiver_ref": ""}
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.jsonl:177:{"acceptance_id": "ACC-09-NUXT-BOARD-UI-PARITY-0177", "capability": "Live run indicator", "current_rewrite_evidence": "backend-rs/src/jobs/mod.rs:103-209", "domain": "UI surface (human non-agent users)", "evidence_ref": "original_evidence + current_rewrite_evidence", "failing_test": "frontend-nuxt/tests/ui-parity.test.ts::ui_surface_human_non_agent_users_agents_live_run_indicator", "final_status": "unclosed", "initial_status": "missing", "original_behavior": "", "original_evidence": "", "passing_test": "cd frontend-nuxt && npx vitest run tests/ui-parity.test.ts", "phase": "phases/09-nuxt-board-ui-parity.md", "review_note": "Confirmed missing. Jobs are synchronous one-shot LLM calls (jobs/mod.rs:103-209) with a running|succeeded|failed status and no runs/heartbeats table or live-run polling.", "row_id": "ui-surface-human-non-agent-users-agents-live-run-indicator", "verdict": "CONFIRMED", "waiver_ref": ""}
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.jsonl:178:{"acceptance_id": "ACC-09-NUXT-BOARD-UI-PARITY-0178", "capability": "Org chart (tree) view", "current_rewrite_evidence": "supabase/migrations/paperclip/0013_agents_hiring.sql:28; routes.rs:36-62", "domain": "UI surface (human non-agent users)", "evidence_ref": "original_evidence + current_rewrite_evidence", "failing_test": "frontend-nuxt/tests/ui-parity.test.ts::ui_surface_human_non_agent_users_agents_org_chart", "final_status": "unclosed", "initial_status": "missing", "original_behavior": "", "original_evidence": "", "passing_test": "cd frontend-nuxt && npx vitest run tests/ui-parity.test.ts", "phase": "phases/09-nuxt-board-ui-parity.md", "review_note": "Confirmed missing. reports_to column exists (0013:28) but there is no /org endpoint in routes.rs (36-62), no tree rendering, and no list/org toggle in the console.", "row_id": "ui-surface-human-non-agent-users-agents-org-chart", "verdict": "CONFIRMED", "waiver_ref": ""}
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.jsonl:179:{"acceptance_id": "ACC-09-NUXT-BOARD-UI-PARITY-0179", "capability": "Agent status filter tabs", "current_rewrite_evidence": "frontend-nuxt/components/PaperclipConsole.vue:60-64; 0013:11", "domain": "UI surface (human non-agent users)", "evidence_ref": "original_evidence + current_rewrite_evidence", "failing_test": "frontend-nuxt/tests/ui-parity.test.ts::ui_surface_human_non_agent_users_agents_status_filter_tabs", "final_status": "unclosed", "initial_status": "missing", "original_behavior": "", "original_evidence": "", "passing_test": "cd frontend-nuxt && npx vitest run tests/ui-parity.test.ts", "phase": "phases/09-nuxt-board-ui-parity.md", "review_note": "Confirmed missing. The list (PaperclipConsole.vue:60-64) is unfiltered and includes pending_approval agents (which the original always hides). No All/Active/Paused/Error tabs; agent_status enum has no 'error' value (0013:11).", "row_id": "ui-surface-human-non-agent-users-agents-status-filter-tabs", "verdict": "CONFIRMED", "waiver_ref": ""}
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.jsonl:181:{"acceptance_id": "ACC-09-NUXT-BOARD-UI-PARITY-0181", "capability": "Budget override approval redirect", "current_rewrite_evidence": "supabase/migrations/paperclip/0013_agents_hiring.sql:14", "domain": "UI surface (human non-agent users)", "evidence_ref": "original_evidence + current_rewrite_evidence", "failing_test": "frontend-nuxt/tests/ui-parity.test.ts::ui_surface_human_non_agent_users_approval_budget_special_case", "final_status": "unclosed", "initial_status": "missing", "original_behavior": "", "original_evidence": "", "passing_test": "cd frontend-nuxt && npx vitest run tests/ui-parity.test.ts", "phase": "phases/09-nuxt-board-ui-parity.md", "review_note": "Confirmed missing. approval_type is only hire_agent/approve_ceo_strategy (0013:14); no budget_override_required type and no /costs page to redirect to.", "row_id": "ui-surface-human-non-agent-users-approval-budget-special-case", "verdict": "CONFIRMED", "waiver_ref": ""}
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.jsonl:182:{"acceptance_id": "ACC-09-NUXT-BOARD-UI-PARITY-0182", "capability": "Approval comments thread", "current_rewrite_evidence": "supabase/migrations/paperclip/0013_agents_hiring.sql:44-57", "domain": "UI surface (human non-agent users)", "evidence_ref": "original_evidence + current_rewrite_evidence", "failing_test": "frontend-nuxt/tests/ui-parity.test.ts::ui_surface_human_non_agent_users_approval_comments", "final_status": "unclosed", "initial_status": "missing", "original_behavior": "", "original_evidence": "", "passing_test": "cd frontend-nuxt && npx vitest run tests/ui-parity.test.ts", "phase": "phases/09-nuxt-board-ui-parity.md", "review_note": "Confirmed missing. No approval comments table or UI; approvals carry only a payload/meta jsonb (0013:44-57) with no thread.", "row_id": "ui-surface-human-non-agent-users-approval-comments", "verdict": "CONFIRMED", "waiver_ref": ""}
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.jsonl:184:{"acceptance_id": "ACC-09-NUXT-BOARD-UI-PARITY-0184", "capability": "Approval detail page", "current_rewrite_evidence": "frontend-nuxt/components/PaperclipConsole.vue:66-81", "domain": "UI surface (human non-agent users)", "evidence_ref": "original_evidence + current_rewrite_evidence", "failing_test": "frontend-nuxt/tests/ui-parity.test.ts::ui_surface_human_non_agent_users_approval_detail_view", "final_status": "unclosed", "initial_status": "missing", "original_behavior": "", "original_evidence": "", "passing_test": "cd frontend-nuxt && npx vitest run tests/ui-parity.test.ts", "phase": "phases/09-nuxt-board-ui-parity.md", "review_note": "Confirmed missing. No approval detail route/page; approvals render as inline list rows showing only ap.type (PaperclipConsole.vue:66-81). No comments, linked issues, typed payload, or raw JSON view.", "row_id": "ui-surface-human-non-agent-users-approval-detail-view", "verdict": "CONFIRMED", "waiver_ref": ""}
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.jsonl:185:{"acceptance_id": "ACC-09-NUXT-BOARD-UI-PARITY-0185", "capability": "Typed approval payload renderers", "current_rewrite_evidence": "frontend-nuxt/components/PaperclipConsole.vue:74; 0013:109", "domain": "UI surface (human non-agent users)", "evidence_ref": "original_evidence + current_rewrite_evidence", "failing_test": "frontend-nuxt/tests/ui-parity.test.ts::ui_surface_human_non_agent_users_approval_payload_renderers", "final_status": "unclosed", "initial_status": "missing", "original_behavior": "", "original_evidence": "", "passing_test": "cd frontend-nuxt && npx vitest run tests/ui-parity.test.ts", "phase": "phases/09-nuxt-board-ui-parity.md", "review_note": "Confirmed missing. The approval payload (name/role/model, stored 0013:109) is never rendered; the list shows only the raw enum type string via {{ ap.type }} (PaperclipConsole.vue:74). No per-type renderers, chips, or friendly labels.", "row_id": "ui-surface-human-non-agent-users-approval-payload-renderers", "verdict": "CONFIRMED", "waiver_ref": ""}
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.jsonl:186:{"acceptance_id": "ACC-09-NUXT-BOARD-UI-PARITY-0186", "capability": "Request revision / Mark resubmitted", "current_rewrite_evidence": "supabase/migrations/paperclip/0013_agents_hiring.sql:17", "domain": "UI surface (human non-agent users)", "evidence_ref": "original_evidence + current_rewrite_evidence", "failing_test": "frontend-nuxt/tests/ui-parity.test.ts::ui_surface_human_non_agent_users_approval_revision_resubmit", "final_status": "unclosed", "initial_status": "missing", "original_behavior": "", "original_evidence": "", "passing_test": "cd frontend-nuxt && npx vitest run tests/ui-parity.test.ts", "phase": "phases/09-nuxt-board-ui-parity.md", "review_note": "Confirmed missing. approval_status enum has no 'revision_requested' value (only pending/approved/rejected/cancelled, 0013:17); decide_approval is binary approve/reject with no revision or resubmit path.", "row_id": "ui-surface-human-non-agent-users-approval-revision-resubmit", "verdict": "CONFIRMED", "waiver_ref": ""}
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.jsonl:187:{"acceptance_id": "ACC-09-NUXT-BOARD-UI-PARITY-0187", "capability": "Board Concierge chat (Conference Room)", "current_rewrite_evidence": "frontend-nuxt/components/PaperclipConsole.vue:38-51", "domain": "UI surface (human non-agent users)", "evidence_ref": "original_evidence + current_rewrite_evidence", "failing_test": "frontend-nuxt/tests/ui-parity.test.ts::ui_surface_human_non_agent_users_board_chat_conference_room", "final_status": "unclosed", "initial_status": "missing", "original_behavior": "", "original_evidence": "", "passing_test": "cd frontend-nuxt && npx vitest run tests/ui-parity.test.ts", "phase": "phases/09-nuxt-board-ui-parity.md", "review_note": "Confirmed missing. No board chat, SSE stream, or CEO welcome. The job console is a single non-streaming prompt->result box (PaperclipConsole.vue:38-51); run_job is a one-shot synchronous LLM call (jobs/mod.rs:103-209).", "row_id": "ui-surface-human-non-agent-users-board-chat-conference-room", "verdict": "CONFIRMED", "waiver_ref": ""}
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.jsonl:188:{"acceptance_id": "ACC-09-NUXT-BOARD-UI-PARITY-0188", "capability": "Board chat message feedback votes", "current_rewrite_evidence": "frontend-nuxt/components/PaperclipConsole.vue:38-82", "domain": "UI surface (human non-agent users)", "evidence_ref": "original_evidence + current_rewrite_evidence", "failing_test": "frontend-nuxt/tests/ui-parity.test.ts::ui_surface_human_non_agent_users_board_chat_feedback_votes", "final_status": "unclosed", "initial_status": "missing", "original_behavior": "", "original_evidence": "", "passing_test": "cd frontend-nuxt && npx vitest run tests/ui-parity.test.ts", "phase": "phases/09-nuxt-board-ui-parity.md", "review_note": "Confirmed missing. No chat bubbles, comments, or feedback-vote model exists in the rewrite.", "row_id": "ui-surface-human-non-agent-users-board-chat-feedback-votes", "verdict": "CONFIRMED", "waiver_ref": ""}
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.jsonl:189:{"acceptance_id": "ACC-09-NUXT-BOARD-UI-PARITY-0189", "capability": "Archive company (danger zone)", "current_rewrite_evidence": "supabase/migrations/paperclip/0011_companies_jobs.sql:6-15", "domain": "UI surface (human non-agent users)", "evidence_ref": "original_evidence + current_rewrite_evidence", "failing_test": "frontend-nuxt/tests/ui-parity.test.ts::ui_surface_human_non_agent_users_company_archive", "final_status": "unclosed", "initial_status": "missing", "original_behavior": "", "original_evidence": "", "passing_test": "cd frontend-nuxt && npx vitest run tests/ui-parity.test.ts", "phase": "phases/09-nuxt-board-ui-parity.md", "review_note": "Confirmed missing. companies has no status/archived column (0011:6-15) and no archive RPC or danger-zone UI.", "row_id": "ui-surface-human-non-agent-users-company-archive", "verdict": "CONFIRMED", "waiver_ref": ""}
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.jsonl:191:{"acceptance_id": "ACC-09-NUXT-BOARD-UI-PARITY-0191", "capability": "Company general/appearance settings", "current_rewrite_evidence": "supabase/migrations/paperclip/0011_companies_jobs.sql:6-15", "domain": "UI surface (human non-agent users)", "evidence_ref": "original_evidence + current_rewrite_evidence", "failing_test": "frontend-nuxt/tests/ui-parity.test.ts::ui_surface_human_non_agent_users_company_general_settings", "final_status": "unclosed", "initial_status": "missing", "original_behavior": "", "original_evidence": "", "passing_test": "cd frontend-nuxt && npx vitest run tests/ui-parity.test.ts", "phase": "phases/09-nuxt-board-ui-parity.md", "review_note": "Confirmed missing. companies has only name/slug/created_by/meta (0011:6-15); no description, brand color, logo, or attachment-limit columns, no settings page, and no company update route (routes.rs:36-62).", "row_id": "ui-surface-human-non-agent-users-company-general-settings", "verdict": "CONFIRMED", "waiver_ref": ""}
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.jsonl:192:{"acceptance_id": "ACC-09-NUXT-BOARD-UI-PARITY-0192", "capability": "Company import/export/cloud links", "current_rewrite_evidence": "backend-rs/src/supabase/routes.rs:36-62", "domain": "UI surface (human non-agent users)", "evidence_ref": "original_evidence + current_rewrite_evidence", "failing_test": "frontend-nuxt/tests/ui-parity.test.ts::ui_surface_human_non_agent_users_company_packages_links", "final_status": "unclosed", "initial_status": "missing", "original_behavior": "", "original_evidence": "", "passing_test": "cd frontend-nuxt && npx vitest run tests/ui-parity.test.ts", "phase": "phases/09-nuxt-board-ui-parity.md", "review_note": "Confirmed missing. No import/export/cloud-upstream routes or UI anywhere in the rewrite.", "row_id": "ui-surface-human-non-agent-users-company-packages-links", "verdict": "CONFIRMED", "waiver_ref": ""}
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.jsonl:193:{"acceptance_id": "ACC-09-NUXT-BOARD-UI-PARITY-0193", "capability": "First agent forced to CEO", "current_rewrite_evidence": "supabase/migrations/paperclip/0013_agents_hiring.sql:26,105", "domain": "UI surface (human non-agent users)", "evidence_ref": "original_evidence + current_rewrite_evidence", "failing_test": "frontend-nuxt/tests/ui-parity.test.ts::ui_surface_human_non_agent_users_first_agent_is_ceo", "final_status": "unclosed", "initial_status": "missing", "original_behavior": "", "original_evidence": "", "passing_test": "cd frontend-nuxt && npx vitest run tests/ui-parity.test.ts", "phase": "phases/09-nuxt-board-ui-parity.md", "review_note": "Confirmed missing. hire_agent (0013:85-113) has no first-agent/count special-casing; role defaults 'general' (0013:26,105). The approve_ceo_strategy enum value (0013:14) is declared but unused by any RPC, and no UI auto-fills/disables pickers for a CEO. Nothing forces the first agent to CEO.", "row_id": "ui-surface-human-non-agent-users-first-agent-is-ceo", "verdict": "CONFIRMED", "waiver_ref": ""}
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.jsonl:194:{"acceptance_id": "ACC-09-NUXT-BOARD-UI-PARITY-0194", "capability": "Adapter-type presets and validation", "current_rewrite_evidence": "supabase/migrations/paperclip/0013_agents_hiring.sql:30", "domain": "UI surface (human non-agent users)", "evidence_ref": "original_evidence + current_rewrite_evidence", "failing_test": "frontend-nuxt/tests/ui-parity.test.ts::ui_surface_human_non_agent_users_hire_adapter_presets", "final_status": "unclosed", "initial_status": "missing", "original_behavior": "", "original_evidence": "", "passing_test": "cd frontend-nuxt && npx vitest run tests/ui-parity.test.ts", "phase": "phases/09-nuxt-board-ui-parity.md", "review_note": "Confirmed missing. adapter_type has a fixed DB default 'openai' (0013:30,105) and the human UI never sends or selects it (PaperclipConsole.vue:56-59). No ?adapterType preset, per-adapter defaults, or OpenCode provider/model validation anywhere in the rewrite.", "row_id": "ui-surface-human-non-agent-users-hire-adapter-presets", "verdict": "CONFIRMED", "waiver_ref": ""}
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.jsonl:195:{"acceptance_id": "ACC-09-NUXT-BOARD-UI-PARITY-0195", "capability": "Optional company-skill selection on hire", "current_rewrite_evidence": "supabase/migrations/paperclip/0013_agents_hiring.sql:33", "domain": "UI surface (human non-agent users)", "evidence_ref": "original_evidence + current_rewrite_evidence", "failing_test": "frontend-nuxt/tests/ui-parity.test.ts::ui_surface_human_non_agent_users_hire_company_skills_select", "final_status": "unclosed", "initial_status": "missing", "original_behavior": "", "original_evidence": "", "passing_test": "cd frontend-nuxt && npx vitest run tests/ui-parity.test.ts", "phase": "phases/09-nuxt-board-ui-parity.md", "review_note": "Confirmed missing. No skills table, endpoint, or UI exists in any read file. agents has a 'capabilities' jsonb (0013:33) but it is never populated or surfaced, and there is no company skill library.", "row_id": "ui-surface-human-non-agent-users-hire-company-skills-select", "verdict": "CONFIRMED", "waiver_ref": ""}
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.jsonl:196:{"acceptance_id": "ACC-09-NUXT-BOARD-UI-PARITY-0196", "capability": "Test Agent (adapter environment probe) button", "current_rewrite_evidence": "backend-rs/src/supabase/routes.rs:36-62 (no probe route)", "domain": "UI surface (human non-agent users)", "evidence_ref": "original_evidence + current_rewrite_evidence", "failing_test": "frontend-nuxt/tests/ui-parity.test.ts::ui_surface_human_non_agent_users_hire_test_agent", "final_status": "unclosed", "initial_status": "missing", "original_behavior": "", "original_evidence": "", "passing_test": "cd frontend-nuxt && npx vitest run tests/ui-parity.test.ts", "phase": "phases/09-nuxt-board-ui-parity.md", "review_note": "Confirmed missing. No adapter/environment concept exists; no probe endpoint in routes.rs and no test button in PaperclipConsole.vue.", "row_id": "ui-surface-human-non-agent-users-hire-test-agent", "verdict": "CONFIRMED", "waiver_ref": ""}
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.jsonl:197:{"acceptance_id": "ACC-09-NUXT-BOARD-UI-PARITY-0197", "capability": "Trust preset + low-trust boundary picker on hire", "current_rewrite_evidence": "supabase/migrations/paperclip/0013_agents_hiring.sql:32,102", "domain": "UI surface (human non-agent users)", "evidence_ref": "original_evidence + current_rewrite_evidence", "failing_test": "frontend-nuxt/tests/ui-parity.test.ts::ui_surface_human_non_agent_users_hire_trust_preset", "final_status": "unclosed", "initial_status": "missing", "original_behavior": "", "original_evidence": "", "passing_test": "cd frontend-nuxt && npx vitest run tests/ui-parity.test.ts", "phase": "phases/09-nuxt-board-ui-parity.md", "review_note": "Confirmed missing. permissions jsonb defaults to {can_create_agents:false} (0013:32) and hire only sets can_create_agents (0013:102). No trust presets, authorization policy, or low-trust boundary; projects/issues do not exist in the rewrite to scope a boundary against.", "row_id": "ui-surface-human-non-agent-users-hire-trust-preset", "verdict": "CONFIRMED", "waiver_ref": ""}
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.jsonl:198:{"acceptance_id": "ACC-09-NUXT-BOARD-UI-PARITY-0198", "capability": "External agent invite + onboarding prompt", "current_rewrite_evidence": "backend-rs/src/supabase/routes.rs:64-74", "domain": "UI surface (human non-agent users)", "evidence_ref": "original_evidence + current_rewrite_evidence", "failing_test": "frontend-nuxt/tests/ui-parity.test.ts::ui_surface_human_non_agent_users_invite_external_agent", "final_status": "unclosed", "initial_status": "missing", "original_behavior": "", "original_evidence": "", "passing_test": "cd frontend-nuxt && npx vitest run tests/ui-parity.test.ts", "phase": "phases/09-nuxt-board-ui-parity.md", "review_note": "Confirmed missing. No company-invite RPC/route and no onboarding-prompt generation. The CLI device-login (routes.rs:64-74, bin/paperclip.rs:219-270) is human-CLI auth, unrelated to inviting an external agent into a company.", "row_id": "ui-surface-human-non-agent-users-invite-external-agent", "verdict": "CONFIRMED", "waiver_ref": ""}
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.jsonl:199:{"acceptance_id": "ACC-09-NUXT-BOARD-UI-PARITY-0199", "capability": "New Agent dialog with 4 creation paths", "current_rewrite_evidence": "frontend-nuxt/components/PaperclipConsole.vue:53-64", "domain": "UI surface (human non-agent users)", "evidence_ref": "original_evidence + current_rewrite_evidence", "failing_test": "frontend-nuxt/tests/ui-parity.test.ts::ui_surface_human_non_agent_users_new_agent_dialog_choices", "final_status": "unclosed", "initial_status": "missing", "original_behavior": "", "original_evidence": "", "passing_test": "cd frontend-nuxt && npx vitest run tests/ui-parity.test.ts", "phase": "phases/09-nuxt-board-ui-parity.md", "review_note": "Confirmed missing. PaperclipConsole.vue has only the single name-form hire path (56-59); no modal, no ask-CEO/manual-runtime/invite-external/prompt modes, no adapter grid.", "row_id": "ui-surface-human-non-agent-users-new-agent-dialog-choices", "verdict": "CONFIRMED", "waiver_ref": ""}
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.jsonl:201:{"acceptance_id": "ACC-10-IMPORT-EXPORT-CATALOGS-PORTABILITY-0201", "capability": "Skills catalog validation and source policy", "current_rewrite_evidence": "not yet covered by rewrite parity matrix; implementation evidence required", "domain": "Catalogs", "evidence_ref": "to be filled when phase implements source-backed behavior", "failing_test": "backend-rs/tests/import_export_parity.rs::catalogs_skills_catalog_validation_and_source_policy", "final_status": "unclosed", "initial_status": "missing", "original_behavior": "Skills catalog validation and source policy", "original_evidence": "Skills catalog validation and source policy", "passing_test": "cd backend-rs && cargo test --test import_export_parity", "phase": "phases/10-import-export-catalogs-portability.md", "review_note": "Added after Codex review: plan-design added this V1 area as a phase, so it must own row-level closure rows, not just a filename.", "row_id": "catalogs-skills-catalog-validation-and-source-policy", "verdict": "PLAN_DESIGN_EXTENSION", "waiver_ref": ""}
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.jsonl:202:{"acceptance_id": "ACC-10-IMPORT-EXPORT-CATALOGS-PORTABILITY-0202", "capability": "Teams catalog validation and source policy", "current_rewrite_evidence": "not yet covered by rewrite parity matrix; implementation evidence required", "domain": "Catalogs", "evidence_ref": "to be filled when phase implements source-backed behavior", "failing_test": "backend-rs/tests/import_export_parity.rs::catalogs_teams_catalog_validation_and_source_policy", "final_status": "unclosed", "initial_status": "missing", "original_behavior": "Teams catalog validation and source policy", "original_evidence": "Teams catalog validation and source policy", "passing_test": "cd backend-rs && cargo test --test import_export_parity", "phase": "phases/10-import-export-catalogs-portability.md", "review_note": "Added after Codex review: plan-design added this V1 area as a phase, so it must own row-level closure rows, not just a filename.", "row_id": "catalogs-teams-catalog-validation-and-source-policy", "verdict": "PLAN_DESIGN_EXTENSION", "waiver_ref": ""}
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.jsonl:203:{"acceptance_id": "ACC-10-IMPORT-EXPORT-CATALOGS-PORTABILITY-0203", "capability": "Company export preview/apply/import round trip", "current_rewrite_evidence": "not yet covered by rewrite parity matrix; implementation evidence required", "domain": "Portability", "evidence_ref": "to be filled when phase implements source-backed behavior", "failing_test": "backend-rs/tests/import_export_parity.rs::portability_company_export_preview_apply_import_round_trip", "final_status": "unclosed", "initial_status": "missing", "original_behavior": "Company export preview/apply/import round trip", "original_evidence": "Company export preview/apply/import round trip", "passing_test": "cd backend-rs && cargo test --test import_export_parity", "phase": "phases/10-import-export-catalogs-portability.md", "review_note": "Added after Codex review: plan-design added this V1 area as a phase, so it must own row-level closure rows, not just a filename.", "row_id": "portability-company-export-preview-apply-import-round-trip", "verdict": "PLAN_DESIGN_EXTENSION", "waiver_ref": ""}
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.jsonl:204:{"acceptance_id": "ACC-10-IMPORT-EXPORT-CATALOGS-PORTABILITY-0204", "capability": "Export bundle secret redaction and no key_hash leakage", "current_rewrite_evidence": "not yet covered by rewrite parity matrix; implementation evidence required", "domain": "Portability", "evidence_ref": "to be filled when phase implements source-backed behavior", "failing_test": "backend-rs/tests/import_export_parity.rs::portability_export_bundle_secret_redaction_and_no_key_hash_leakage", "final_status": "unclosed", "initial_status": "missing", "original_behavior": "Export bundle secret redaction and no key_hash leakage", "original_evidence": "Export bundle secret redaction and no key_hash leakage", "passing_test": "cd backend-rs && cargo test --test import_export_parity", "phase": "phases/10-import-export-catalogs-portability.md", "review_note": "Added after Codex review: plan-design added this V1 area as a phase, so it must own row-level closure rows, not just a filename.", "row_id": "portability-export-bundle-secret-redaction-and-no-key-hash-leakage", "verdict": "PLAN_DESIGN_EXTENSION", "waiver_ref": ""}
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.jsonl:205:{"acceptance_id": "ACC-10-IMPORT-EXPORT-CATALOGS-PORTABILITY-0205", "capability": "Reusable company/team templates and onboarding packages", "current_rewrite_evidence": "not yet covered by rewrite parity matrix; implementation evidence required", "domain": "Templates", "evidence_ref": "to be filled when phase implements source-backed behavior", "failing_test": "backend-rs/tests/import_export_parity.rs::templates_reusable_company_team_templates_and_onboarding_packages", "final_status": "unclosed", "initial_status": "missing", "original_behavior": "Reusable company/team templates and onboarding packages", "original_evidence": "Reusable company/team templates and onboarding packages", "passing_test": "cd backend-rs && cargo test --test import_export_parity", "phase": "phases/10-import-export-catalogs-portability.md", "review_note": "Added after Codex review: plan-design added this V1 area as a phase, so it must own row-level closure rows, not just a filename.", "row_id": "templates-reusable-company-team-templates-and-onboarding-packages", "verdict": "PLAN_DESIGN_EXTENSION", "waiver_ref": ""}
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.jsonl:206:{"acceptance_id": "ACC-11-OBSERVABILITY-FEEDBACK-PIPELINES-0206", "capability": "Dashboard, health, doctor observability", "current_rewrite_evidence": "not yet covered by rewrite parity matrix; implementation evidence required", "domain": "Observability", "evidence_ref": "to be filled when phase implements source-backed behavior", "failing_test": "backend-rs/tests/observability_parity.rs::observability_dashboard_health_doctor_observability", "final_status": "unclosed", "initial_status": "missing", "original_behavior": "Dashboard, health, doctor observability", "original_evidence": "Dashboard, health, doctor observability", "passing_test": "cd backend-rs && cargo test --test observability_parity", "phase": "phases/11-observability-feedback-pipelines.md", "review_note": "Added after Codex review: plan-design added this V1 area as a phase, so it must own row-level closure rows, not just a filename.", "row_id": "observability-dashboard-health-doctor-observability", "verdict": "PLAN_DESIGN_EXTENSION", "waiver_ref": ""}
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.jsonl:207:{"acceptance_id": "ACC-11-OBSERVABILITY-FEEDBACK-PIPELINES-0207", "capability": "Feedback exports and votes", "current_rewrite_evidence": "not yet covered by rewrite parity matrix; implementation evidence required", "domain": "Observability", "evidence_ref": "to be filled when phase implements source-backed behavior", "failing_test": "backend-rs/tests/observability_parity.rs::observability_feedback_exports_and_votes", "final_status": "unclosed", "initial_status": "missing", "original_behavior": "Feedback exports and votes", "original_evidence": "Feedback exports and votes", "passing_test": "cd backend-rs && cargo test --test observability_parity", "phase": "phases/11-observability-feedback-pipelines.md", "review_note": "Added after Codex review: plan-design added this V1 area as a phase, so it must own row-level closure rows, not just a filename.", "row_id": "observability-feedback-exports-and-votes", "verdict": "PLAN_DESIGN_EXTENSION", "waiver_ref": ""}
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.jsonl:208:{"acceptance_id": "ACC-11-OBSERVABILITY-FEEDBACK-PIPELINES-0208", "capability": "Pipeline cases, events, outputs, conversation context", "current_rewrite_evidence": "not yet covered by rewrite parity matrix; implementation evidence required", "domain": "Observability", "evidence_ref": "to be filled when phase implements source-backed behavior", "failing_test": "backend-rs/tests/observability_parity.rs::observability_pipeline_cases_events_outputs_conversation_context", "final_status": "unclosed", "initial_status": "missing", "original_behavior": "Pipeline cases, events, outputs, conversation context", "original_evidence": "Pipeline cases, events, outputs, conversation context", "passing_test": "cd backend-rs && cargo test --test observability_parity", "phase": "phases/11-observability-feedback-pipelines.md", "review_note": "Added after Codex review: plan-design added this V1 area as a phase, so it must own row-level closure rows, not just a filename.", "row_id": "observability-pipeline-cases-events-outputs-conversation-context", "verdict": "PLAN_DESIGN_EXTENSION", "waiver_ref": ""}
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.jsonl:209:{"acceptance_id": "ACC-11-OBSERVABILITY-FEEDBACK-PIPELINES-0209", "capability": "Productivity review states", "current_rewrite_evidence": "not yet covered by rewrite parity matrix; implementation evidence required", "domain": "Observability", "evidence_ref": "to be filled when phase implements source-backed behavior", "failing_test": "backend-rs/tests/observability_parity.rs::observability_productivity_review_states", "final_status": "unclosed", "initial_status": "missing", "original_behavior": "Productivity review states", "original_evidence": "Productivity review states", "passing_test": "cd backend-rs && cargo test --test observability_parity", "phase": "phases/11-observability-feedback-pipelines.md", "review_note": "Added after Codex review: plan-design added this V1 area as a phase, so it must own row-level closure rows, not just a filename.", "row_id": "observability-productivity-review-states", "verdict": "PLAN_DESIGN_EXTENSION", "waiver_ref": ""}
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.jsonl:210:{"acceptance_id": "ACC-11-OBSERVABILITY-FEEDBACK-PIPELINES-0210", "capability": "Release and e2e smoke result ingestion", "current_rewrite_evidence": "not yet covered by rewrite parity matrix; implementation evidence required", "domain": "Observability", "evidence_ref": "to be filled when phase implements source-backed behavior", "failing_test": "backend-rs/tests/observability_parity.rs::observability_release_and_e2e_smoke_result_ingestion", "final_status": "unclosed", "initial_status": "missing", "original_behavior": "Release and e2e smoke result ingestion", "original_evidence": "Release and e2e smoke result ingestion", "passing_test": "cd backend-rs && cargo test --test observability_parity", "phase": "phases/11-observability-feedback-pipelines.md", "review_note": "Added after Codex review: plan-design added this V1 area as a phase, so it must own row-level closure rows, not just a filename.", "row_id": "observability-release-and-e2e-smoke-result-ingestion", "verdict": "PLAN_DESIGN_EXTENSION", "waiver_ref": ""}
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.jsonl:211:{"acceptance_id": "ACC-12-SHARED-CONTRACT-DOCS-PARITY-0211", "capability": "OpenAPI/schema export and generated clients", "current_rewrite_evidence": "not yet covered by rewrite parity matrix; implementation evidence required", "domain": "Contracts", "evidence_ref": "to be filled when phase implements source-backed behavior", "failing_test": "backend-rs/tests/shared_contract_parity.rs::contracts_openapi_schema_export_and_generated_clients", "final_status": "unclosed", "initial_status": "missing", "original_behavior": "OpenAPI/schema export and generated clients", "original_evidence": "OpenAPI/schema export and generated clients", "passing_test": "cd backend-rs && cargo test --test shared_contract_parity", "phase": "phases/12-shared-contract-docs-parity.md", "review_note": "Added after Codex review: plan-design added this V1 area as a phase, so it must own row-level closure rows, not just a filename.", "row_id": "contracts-openapi-schema-export-and-generated-clients", "verdict": "PLAN_DESIGN_EXTENSION", "waiver_ref": ""}
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.jsonl:212:{"acceptance_id": "ACC-12-SHARED-CONTRACT-DOCS-PARITY-0212", "capability": "packages/shared constants, validators, API path parity surface", "current_rewrite_evidence": "not yet covered by rewrite parity matrix; implementation evidence required", "domain": "Contracts", "evidence_ref": "to be filled when phase implements source-backed behavior", "failing_test": "backend-rs/tests/shared_contract_parity.rs::contracts_packages_shared_constants_validators_api_path_parity_surface", "final_status": "unclosed", "initial_status": "missing", "original_behavior": "packages/shared constants, validators, API path parity surface", "original_evidence": "packages/shared constants, validators, API path parity surface", "passing_test": "cd backend-rs && cargo test --test shared_contract_parity", "phase": "phases/12-shared-contract-docs-parity.md", "review_note": "Added after Codex review: plan-design added this V1 area as a phase, so it must own row-level closure rows, not just a filename.", "row_id": "contracts-packages-shared-constants-validators-api-path-parity-surface", "verdict": "PLAN_DESIGN_EXTENSION", "waiver_ref": ""}
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.jsonl:213:{"acceptance_id": "ACC-12-SHARED-CONTRACT-DOCS-PARITY-0213", "capability": "Internal doc architecture records kept current", "current_rewrite_evidence": "not yet covered by rewrite parity matrix; implementation evidence required", "domain": "Docs", "evidence_ref": "to be filled when phase implements source-backed behavior", "failing_test": "backend-rs/tests/shared_contract_parity.rs::docs_internal_doc_architecture_records_kept_current", "final_status": "unclosed", "initial_status": "missing", "original_behavior": "Internal doc architecture records kept current", "original_evidence": "Internal doc architecture records kept current", "passing_test": "cd backend-rs && cargo test --test shared_contract_parity", "phase": "phases/12-shared-contract-docs-parity.md", "review_note": "Added after Codex review: plan-design added this V1 area as a phase, so it must own row-level closure rows, not just a filename.", "row_id": "docs-internal-doc-architecture-records-kept-current", "verdict": "PLAN_DESIGN_EXTENSION", "waiver_ref": ""}
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.jsonl:214:{"acceptance_id": "ACC-12-SHARED-CONTRACT-DOCS-PARITY-0214", "capability": "No internal IDs, secrets, tailnet, or agent URLs in public docs", "current_rewrite_evidence": "not yet covered by rewrite parity matrix; implementation evidence required", "domain": "Docs", "evidence_ref": "to be filled when phase implements source-backed behavior", "failing_test": "backend-rs/tests/shared_contract_parity.rs::docs_no_internal_ids_secrets_tailnet_or_agent_urls_in_public_docs", "final_status": "unclosed", "initial_status": "missing", "original_behavior": "No internal IDs, secrets, tailnet, or agent URLs in public docs", "original_evidence": "No internal IDs, secrets, tailnet, or agent URLs in public docs", "passing_test": "cd backend-rs && cargo test --test shared_contract_parity", "phase": "phases/12-shared-contract-docs-parity.md", "review_note": "Added after Codex review: plan-design added this V1 area as a phase, so it must own row-level closure rows, not just a filename.", "row_id": "docs-no-internal-ids-secrets-tailnet-or-agent-urls-in-public-docs", "verdict": "PLAN_DESIGN_EXTENSION", "waiver_ref": ""}
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.jsonl:215:{"acceptance_id": "ACC-12-SHARED-CONTRACT-DOCS-PARITY-0215", "capability": "Public docs navigation docs/docs.json accuracy", "current_rewrite_evidence": "not yet covered by rewrite parity matrix; implementation evidence required", "domain": "Docs", "evidence_ref": "to be filled when phase implements source-backed behavior", "failing_test": "backend-rs/tests/shared_contract_parity.rs::docs_public_docs_navigation_docs_docs_json_accuracy", "final_status": "unclosed", "initial_status": "missing", "original_behavior": "Public docs navigation docs/docs.json accuracy", "original_evidence": "Public docs navigation docs/docs.json accuracy", "passing_test": "cd backend-rs && cargo test --test shared_contract_parity", "phase": "phases/12-shared-contract-docs-parity.md", "review_note": "Added after Codex review: plan-design added this V1 area as a phase, so it must own row-level closure rows, not just a filename.", "row_id": "docs-public-docs-navigation-docs-docs-json-accuracy", "verdict": "PLAN_DESIGN_EXTENSION", "waiver_ref": ""}
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.jsonl:216:{"acceptance_id": "ACC-13-MIGRATION-COMPATIBILITY-CUTOVER-0216", "capability": "Compatibility views and RPC aliases during transition", "current_rewrite_evidence": "not yet covered by rewrite parity matrix; implementation evidence required", "domain": "Cutover", "evidence_ref": "to be filled when phase implements source-backed behavior", "failing_test": "acceptance/run_original_gap_diff.py::cutover_compatibility_views_and_rpc_aliases_during_transition", "final_status": "unclosed", "initial_status": "missing", "original_behavior": "Compatibility views and RPC aliases during transition", "original_evidence": "Compatibility views and RPC aliases during transition", "passing_test": "python3 docs/plans/2026-07-05-full-paperclip-parity/acceptance/run_original_gap_diff.py", "phase": "phases/13-migration-compatibility-cutover.md", "review_note": "Added after Codex review: plan-design added this V1 area as a phase, so it must own row-level closure rows, not just a filename.", "row_id": "cutover-compatibility-views-and-rpc-aliases-during-transition", "verdict": "PLAN_DESIGN_EXTENSION", "waiver_ref": ""}
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.jsonl:217:{"acceptance_id": "ACC-13-MIGRATION-COMPATIBILITY-CUTOVER-0217", "capability": "Feature flags and strangler routing between old and new surfaces", "current_rewrite_evidence": "not yet covered by rewrite parity matrix; implementation evidence required", "domain": "Cutover", "evidence_ref": "to be filled when phase implements source-backed behavior", "failing_test": "acceptance/run_original_gap_diff.py::cutover_feature_flags_and_strangler_routing_between_old_and_new_surfaces", "final_status": "unclosed", "initial_status": "missing", "original_behavior": "Feature flags and strangler routing between old and new surfaces", "original_evidence": "Feature flags and strangler routing between old and new surfaces", "passing_test": "python3 docs/plans/2026-07-05-full-paperclip-parity/acceptance/run_original_gap_diff.py", "phase": "phases/13-migration-compatibility-cutover.md", "review_note": "Added after Codex review: plan-design added this V1 area as a phase, so it must own row-level closure rows, not just a filename.", "row_id": "cutover-feature-flags-and-strangler-routing-between-old-and-new-surfaces", "verdict": "PLAN_DESIGN_EXTENSION", "waiver_ref": ""}
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.jsonl:218:{"acceptance_id": "ACC-13-MIGRATION-COMPATIBILITY-CUTOVER-0218", "capability": "Forward-only data-preserving migrations and backfill", "current_rewrite_evidence": "not yet covered by rewrite parity matrix; implementation evidence required", "domain": "Cutover", "evidence_ref": "to be filled when phase implements source-backed behavior", "failing_test": "acceptance/run_original_gap_diff.py::cutover_forward_only_data_preserving_migrations_and_backfill", "final_status": "unclosed", "initial_status": "missing", "original_behavior": "Forward-only data-preserving migrations and backfill", "original_evidence": "Forward-only data-preserving migrations and backfill", "passing_test": "python3 docs/plans/2026-07-05-full-paperclip-parity/acceptance/run_original_gap_diff.py", "phase": "phases/13-migration-compatibility-cutover.md", "review_note": "Added after Codex review: plan-design added this V1 area as a phase, so it must own row-level closure rows, not just a filename.", "row_id": "cutover-forward-only-data-preserving-migrations-and-backfill", "verdict": "PLAN_DESIGN_EXTENSION", "waiver_ref": ""}
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.jsonl:219:{"acceptance_id": "ACC-13-MIGRATION-COMPATIBILITY-CUTOVER-0219", "capability": "Post-cutover validation queries and Supabase advisors", "current_rewrite_evidence": "not yet covered by rewrite parity matrix; implementation evidence required", "domain": "Cutover", "evidence_ref": "to be filled when phase implements source-backed behavior", "failing_test": "acceptance/run_original_gap_diff.py::cutover_post_cutover_validation_queries_and_supabase_advisors", "final_status": "unclosed", "initial_status": "missing", "original_behavior": "Post-cutover validation queries and Supabase advisors", "original_evidence": "Post-cutover validation queries and Supabase advisors", "passing_test": "python3 docs/plans/2026-07-05-full-paperclip-parity/acceptance/run_original_gap_diff.py", "phase": "phases/13-migration-compatibility-cutover.md", "review_note": "Added after Codex review: plan-design added this V1 area as a phase, so it must own row-level closure rows, not just a filename.", "row_id": "cutover-post-cutover-validation-queries-and-supabase-advisors", "verdict": "PLAN_DESIGN_EXTENSION", "waiver_ref": ""}
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.jsonl:220:{"acceptance_id": "ACC-13-MIGRATION-COMPATIBILITY-CUTOVER-0220", "capability": "Rollback without destructive enum/table drops", "current_rewrite_evidence": "not yet covered by rewrite parity matrix; implementation evidence required", "domain": "Cutover", "evidence_ref": "to be filled when phase implements source-backed behavior", "failing_test": "acceptance/run_original_gap_diff.py::cutover_rollback_without_destructive_enum_table_drops", "final_status": "unclosed", "initial_status": "missing", "original_behavior": "Rollback without destructive enum/table drops", "original_evidence": "Rollback without destructive enum/table drops", "passing_test": "python3 docs/plans/2026-07-05-full-paperclip-parity/acceptance/run_original_gap_diff.py", "phase": "phases/13-migration-compatibility-cutover.md", "review_note": "Added after Codex review: plan-design added this V1 area as a phase, so it must own row-level closure rows, not just a filename.", "row_id": "cutover-rollback-without-destructive-enum-table-drops", "verdict": "PLAN_DESIGN_EXTENSION", "waiver_ref": ""}
docs/plans/2026-07-05-full-paperclip-parity/phases/00-parity-oracle-and-matrix.md:1:# Phase 00 — Parity Oracle and Matrix
docs/plans/2026-07-05-full-paperclip-parity/phases/00-parity-oracle-and-matrix.md:7:## Required artifacts (already scaffolded; Phase 00 must finish them)
docs/plans/2026-07-05-full-paperclip-parity/phases/00-parity-oracle-and-matrix.md:9:- `evidence/rewrite-gap-matrix.md` + `evidence/rewrite-gap-matrix.jsonl`: row-level matrix with original evidence, current rewrite evidence, status, owning phase, red test, green command, acceptance ID, waiver field. Today it carries 220 rows across all 14 phases.
docs/plans/2026-07-05-full-paperclip-parity/phases/00-parity-oracle-and-matrix.md:11:- `acceptance/run_original_gap_diff.py`: the ledger gate. Implemented now; fails on the current rewrite and prints exactly `gaps_missing=0 gaps_partial=0 gaps_divergent=0` only on success.
docs/plans/2026-07-05-full-paperclip-parity/phases/00-parity-oracle-and-matrix.md:12:- `acceptance/run_real_user_real_agent.py`: RED scaffold. Phase 00 must replace the body with the live Supabase + real OpenAI Responses driver.
docs/plans/2026-07-05-full-paperclip-parity/phases/00-parity-oracle-and-matrix.md:13:- `acceptance/run_cli_mcp_surface.py`: RED scaffold. Phase 00 must implement the CLI/MCP surface parity check.
docs/plans/2026-07-05-full-paperclip-parity/phases/00-parity-oracle-and-matrix.md:14:- `acceptance/run_frontend_user_flow.py`: RED scaffold. Phase 00 must implement the Nuxt board-flow parity check.
docs/plans/2026-07-05-full-paperclip-parity/phases/00-parity-oracle-and-matrix.md:15:- No-secret scanner (see `acceptance/no-secret-scanner-spec.md`): Phase 00 must implement it.
docs/plans/2026-07-05-full-paperclip-parity/phases/00-parity-oracle-and-matrix.md:19:1. Confirm matrix rows cover all 14 phases (done: 220 rows). Refine row evidence as later phases cite exact source lines.
docs/plans/2026-07-05-full-paperclip-parity/phases/00-parity-oracle-and-matrix.md:20:2. Classify rows as `v1-core`, `current-addendum`, `post-v1`, or `not-in-v1`.
docs/plans/2026-07-05-full-paperclip-parity/phases/00-parity-oracle-and-matrix.md:21:3. Implement the four acceptance harnesses above so each is a real driver, not a RED scaffold.
docs/plans/2026-07-05-full-paperclip-parity/phases/00-parity-oracle-and-matrix.md:27:- `run_original_gap_diff.py` runs today, fails with non-zero counts, and on eventual success prints exactly `gaps_missing=0 gaps_partial=0 gaps_divergent=0`.
docs/plans/2026-07-05-full-paperclip-parity/phases/00-parity-oracle-and-matrix.md:28:- Phase 00 is not closed until `run_real_user_real_agent.py`, `run_cli_mcp_surface.py`, `run_frontend_user_flow.py`, and the no-secret scanner are real drivers (the current RED scaffolds intentionally return non-zero).
docs/plans/2026-07-05-full-paperclip-parity/phases/00-parity-oracle-and-matrix.md:29:- Final release requires all blocking rows full or explicitly waived by an updated product contract.
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.md:3:Row-level closure ledger. Workers close rows, not categories. Generated from the adversarial parity audit journals plus plan-design coverage rows for every owned phase.
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.md:9:- missing: 145
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.md:12:Machine-readable companion: [rewrite-gap-matrix.jsonl](rewrite-gap-matrix.jsonl).
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.md:18:| `phases/00-parity-oracle-and-matrix.md` | 0 | 0 | 4 | 0 | 4 |
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.md:33:## Row-level matrix
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.md:37:| `parity-oracle-no-secret-scanner-implementation-across-all-surfaces` | `missing` | `phases/00-parity-oracle-and-matrix.md` | `ACC-00-PARITY-ORACLE-AND-MATRIX-0001` | `acceptance/run_original_gap_diff.py::parity_oracle_no_secret_scanner_implementation_across_all_surfaces` | No-secret scanner implementation across all surfaces |
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.md:38:| `parity-oracle-run-cli-mcp-surface-py-and-run-frontend-user-flow-py-harnesses` | `missing` | `phases/00-parity-oracle-and-matrix.md` | `ACC-00-PARITY-ORACLE-AND-MATRIX-0002` | `acceptance/run_original_gap_diff.py::parity_oracle_run_cli_mcp_surface_py_and_run_frontend_user_flow_py_harnesses` | run_cli_mcp_surface.py and run_frontend_user_flow.py harnesses |
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.md:39:| `parity-oracle-run-original-gap-diff-py-exact-gaps-output-contract` | `missing` | `phases/00-parity-oracle-and-matrix.md` | `ACC-00-PARITY-ORACLE-AND-MATRIX-0003` | `acceptance/run_original_gap_diff.py::parity_oracle_run_original_gap_diff_py_exact_gaps_output_contract` | run_original_gap_diff.py exact gaps_* output contract |
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.md:40:| `parity-oracle-run-real-user-real-agent-py-live-harness-implementation` | `missing` | `phases/00-parity-oracle-and-matrix.md` | `ACC-00-PARITY-ORACLE-AND-MATRIX-0004` | `acceptance/run_original_gap_diff.py::parity_oracle_run_real_user_real_agent_py_live_harness_implementation` | run_real_user_real_agent.py live harness implementation |
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.md:42:| `data-model-schema-agent-budget-fields` | `missing` | `phases/01-schema-authz-parity.md` | `ACC-01-SCHEMA-AUTHZ-PARITY-0006` | `backend-rs/tests/authz_parity.rs::data_model_schema_agent_budget_fields` | per-agent monthly budget accounting |
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.md:43:| `data-model-schema-agent-config-revisions-table` | `missing` | `phases/01-schema-authz-parity.md` | `ACC-01-SCHEMA-AUTHZ-PARITY-0007` | `backend-rs/tests/authz_parity.rs::data_model_schema_agent_config_revisions_table` | agent_config_revisions audit table |
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.md:44:| `data-model-schema-agent-icon-names` | `missing` | `phases/01-schema-authz-parity.md` | `ACC-01-SCHEMA-AUTHZ-PARITY-0008` | `backend-rs/tests/authz_parity.rs::data_model_schema_agent_icon_names` | AGENT_ICON_NAMES enum |
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.md:49:| `data-model-schema-agents-orm-extra-columns` | `missing` | `phases/01-schema-authz-parity.md` | `ACC-01-SCHEMA-AUTHZ-PARITY-0013` | `backend-rs/tests/authz_parity.rs::data_model_schema_agents_orm_extra_columns` | agents ORM adds icon/runtime_config/env/pause/error columns |
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.md:52:| `data-model-schema-approval-comments-table` | `missing` | `phases/01-schema-authz-parity.md` | `ACC-01-SCHEMA-AUTHZ-PARITY-0016` | `backend-rs/tests/authz_parity.rs::data_model_schema_approval_comments_table` | approval_comments table |
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.md:59:| `data-model-schema-company-toggle-validators` | `missing` | `phases/01-schema-authz-parity.md` | `ACC-01-SCHEMA-AUTHZ-PARITY-0023` | `backend-rs/tests/authz_parity.rs::data_model_schema_company_toggle_validators` | board-approval toggle in company validators |
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.md:62:| `data-model-schema-issue-approvals-link-table` | `missing` | `phases/01-schema-authz-parity.md` | `ACC-01-SCHEMA-AUTHZ-PARITY-0026` | `backend-rs/tests/authz_parity.rs::data_model_schema_issue_approvals_link_table` | issue_approvals junction table |
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.md:63:| `data-model-schema-pause-reasons-enum` | `missing` | `phases/01-schema-authz-parity.md` | `ACC-01-SCHEMA-AUTHZ-PARITY-0027` | `backend-rs/tests/authz_parity.rs::data_model_schema_pause_reasons_enum` | PAUSE_REASONS enum |
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.md:64:| `data-model-schema-update-agent-permissions-validator` | `missing` | `phases/01-schema-authz-parity.md` | `ACC-01-SCHEMA-AUTHZ-PARITY-0028` | `backend-rs/tests/authz_parity.rs::data_model_schema_update_agent_permissions_validator` | updateAgentPermissionsSchema |
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.md:65:| `data-model-schema-update-agent-validator` | `missing` | `phases/01-schema-authz-parity.md` | `ACC-01-SCHEMA-AUTHZ-PARITY-0029` | `backend-rs/tests/authz_parity.rs::data_model_schema_update_agent_validator` | updateAgentSchema restrictions |
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.md:67:| `hiring-flow-side-effects-default-task-assign-grant` | `missing` | `phases/01-schema-authz-parity.md` | `ACC-01-SCHEMA-AUTHZ-PARITY-0031` | `backend-rs/tests/authz_parity.rs::hiring_flow_side_effects_default_task_assign_grant` | Default tasks:assign grant on every hire |
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.md:68:| `hiring-flow-side-effects-direct-create-blocked-when-approval` | `missing` | `phases/01-schema-authz-parity.md` | `ACC-01-SCHEMA-AUTHZ-PARITY-0032` | `backend-rs/tests/authz_parity.rs::hiring_flow_side_effects_direct_create_blocked_when_approval` | Direct POST /agents blocked when approvals required |
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.md:69:| `hiring-flow-side-effects-hire-activity-log` | `missing` | `phases/01-schema-authz-parity.md` | `ACC-01-SCHEMA-AUTHZ-PARITY-0033` | `backend-rs/tests/authz_parity.rs::hiring_flow_side_effects_hire_activity_log` | agent.hire_created + approval.created activity logs + telemetry |
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.md:74:| `hiring-flow-side-effects-join-request-hire-hook` | `missing` | `phases/01-schema-authz-parity.md` | `ACC-01-SCHEMA-AUTHZ-PARITY-0038` | `backend-rs/tests/authz_parity.rs::hiring_flow_side_effects_join_request_hire_hook` | Join-request approval also triggers hire hook |
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.md:75:| `hiring-flow-side-effects-legacy-prompt-guard` | `missing` | `phases/01-schema-authz-parity.md` | `ACC-01-SCHEMA-AUTHZ-PARITY-0039` | `backend-rs/tests/authz_parity.rs::hiring_flow_side_effects_legacy_prompt_guard` | Reject legacy promptTemplate on new hires |
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.md:76:| `hiring-flow-side-effects-plugin-managed-agent-hire` | `missing` | `phases/01-schema-authz-parity.md` | `ACC-01-SCHEMA-AUTHZ-PARITY-0040` | `backend-rs/tests/authz_parity.rs::hiring_flow_side_effects_plugin_managed_agent_hire` | Plugin-managed agent creation mirrors the hire+approval flow |
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.md:77:| `hiring-flow-side-effects-source-issue-linking` | `missing` | `phases/01-schema-authz-parity.md` | `ACC-01-SCHEMA-AUTHZ-PARITY-0041` | `backend-rs/tests/authz_parity.rs::hiring_flow_side_effects_source_issue_linking` | Source issues linked to approval |
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.md:78:| `permissions-board-membership-access-activity-audit-side-effects-for-pe` | `missing` | `phases/01-schema-authz-parity.md` | `ACC-01-SCHEMA-AUTHZ-PARITY-0042` | `backend-rs/tests/authz_parity.rs::permissions_board_membership_access_activity_audit_side_effects_for_pe` | Activity/audit side effects for permission and hiring actions |
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.md:81:| `permissions-board-membership-access-agent-configuration-read-update-au` | `missing` | `phases/01-schema-authz-parity.md` | `ACC-01-SCHEMA-AUTHZ-PARITY-0045` | `backend-rs/tests/authz_parity.rs::permissions_board_membership_access_agent_configuration_read_update_au` | Agent configuration read/update authorization and redaction |
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.md:85:| `permissions-board-membership-access-company-member-management-and-cust` | `missing` | `phases/01-schema-authz-parity.md` | `ACC-01-SCHEMA-AUTHZ-PARITY-0049` | `backend-rs/tests/authz_parity.rs::permissions_board_membership_access_company_member_management_and_cust` | Company member management and custom permission grants |
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.md:89:| `permissions-board-membership-access-original-permission-key-inventory` | `missing` | `phases/01-schema-authz-parity.md` | `ACC-01-SCHEMA-AUTHZ-PARITY-0053` | `backend-rs/tests/authz_parity.rs::permissions_board_membership_access_original_permission_key_inventory` | Original permission key inventory |
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.md:90:| `permissions-board-membership-access-permission-update-endpoint-for-age` | `missing` | `phases/01-schema-authz-parity.md` | `ACC-01-SCHEMA-AUTHZ-PARITY-0054` | `backend-rs/tests/authz_parity.rs::permissions_board_membership_access_permission_update_endpoint_for_age` | Permission update endpoint for agents (`canCreateAgents`, `canCreateSkills`, `canAssignTasks`) |
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.md:91:| `permissions-board-membership-access-protected-private-agent-authorizat` | `missing` | `phases/01-schema-authz-parity.md` | `ACC-01-SCHEMA-AUTHZ-PARITY-0055` | `backend-rs/tests/authz_parity.rs::permissions_board_membership_access_protected_private_agent_authorizat` | Protected/private agent authorization policies and low-trust boundaries |
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.md:95:| `permissions-board-membership-access-task-assignment-permissions-tasks-` | `missing` | `phases/01-schema-authz-parity.md` | `ACC-01-SCHEMA-AUTHZ-PARITY-0059` | `backend-rs/tests/authz_parity.rs::permissions_board_membership_access_task_assignment_permissions_tasks_` | Task assignment permissions (`tasks:assign`, scoped assignment, simple default) |
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.md:96:| `comments-issue-comments-as-the-built-in-communication-model` | `missing` | `phases/02-goals-projects-issues-work-products.md` | `ACC-02-GOALS-PROJECTS-ISSUES-WORK-PRODUCTS-0060` | `backend-rs/tests/tasks_parity.rs::comments_issue_comments_as_the_built_in_communication_model` | Issue comments as the built-in communication model |
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.md:97:| `documents-issue-documents-and-attachments` | `missing` | `phases/02-goals-projects-issues-work-products.md` | `ACC-02-GOALS-PROJECTS-ISSUES-WORK-PRODUCTS-0061` | `backend-rs/tests/tasks_parity.rs::documents_issue_documents_and_attachments` | Issue documents and attachments |
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.md:98:| `goals-hierarchy-company-mission-and-goal-hierarchy-company-team-agent-task` | `missing` | `phases/02-goals-projects-issues-work-products.md` | `ACC-02-GOALS-PROJECTS-ISSUES-WORK-PRODUCTS-0062` | `backend-rs/tests/tasks_parity.rs::goals_hierarchy_company_mission_and_goal_hierarchy_company_team_agent_task` | Company mission and goal hierarchy company/team/agent/task |
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.md:99:| `goals-hierarchy-goal-crud-parent-linkage-owner-agent-assignment` | `missing` | `phases/02-goals-projects-issues-work-products.md` | `ACC-02-GOALS-PROJECTS-ISSUES-WORK-PRODUCTS-0063` | `backend-rs/tests/tasks_parity.rs::goals_hierarchy_goal_crud_parent_linkage_owner_agent_assignment` | Goal CRUD, parent linkage, owner agent assignment |
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.md:100:| `issues-blockers-recovery-issues-review-approval-handoffs` | `missing` | `phases/02-goals-projects-issues-work-products.md` | `ACC-02-GOALS-PROJECTS-ISSUES-WORK-PRODUCTS-0064` | `backend-rs/tests/tasks_parity.rs::issues_blockers_recovery_issues_review_approval_handoffs` | Blockers, recovery issues, review/approval handoffs |
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.md:101:| `issues-issue-stable-identifiers-and-parent-sub-issue-tree` | `missing` | `phases/02-goals-projects-issues-work-products.md` | `ACC-02-GOALS-PROJECTS-ISSUES-WORK-PRODUCTS-0065` | `backend-rs/tests/tasks_parity.rs::issues_issue_stable_identifiers_and_parent_sub_issue_tree` | Issue stable identifiers and parent/sub-issue tree |
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.md:102:| `issues-single-assignee-and-atomic-checkout-for-in-progress` | `missing` | `phases/02-goals-projects-issues-work-products.md` | `ACC-02-GOALS-PROJECTS-ISSUES-WORK-PRODUCTS-0066` | `backend-rs/tests/tasks_parity.rs::issues_single_assignee_and_atomic_checkout_for_in_progress` | Single-assignee and atomic checkout for in_progress |
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.md:103:| `projects-project-crud-lead-target-date-goal-linkage` | `missing` | `phases/02-goals-projects-issues-work-products.md` | `ACC-02-GOALS-PROJECTS-ISSUES-WORK-PRODUCTS-0067` | `backend-rs/tests/tasks_parity.rs::projects_project_crud_lead_target_date_goal_linkage` | Project CRUD, lead, target date, goal linkage |
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.md:104:| `projects-project-env-overlay-merging-into-run-environment` | `missing` | `phases/02-goals-projects-issues-work-products.md` | `ACC-02-GOALS-PROJECTS-ISSUES-WORK-PRODUCTS-0068` | `backend-rs/tests/tasks_parity.rs::projects_project_env_overlay_merging_into_run_environment` | Project env overlay merging into run environment |
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.md:105:| `work-products-work-product-artifact-metadata-and-inspectable-outputs` | `missing` | `phases/02-goals-projects-issues-work-products.md` | `ACC-02-GOALS-PROJECTS-ISSUES-WORK-PRODUCTS-0069` | `backend-rs/tests/tasks_parity.rs::work_products_work_product_artifact_metadata_and_inspectable_outputs` | Work product/artifact metadata and inspectable outputs |
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.md:106:| `execution-workspaces-project-execution-workspace-policy-and-provision-command` | `missing` | `phases/03-workspaces-routines-plugins-sandbox.md` | `ACC-03-WORKSPACES-ROUTINES-PLUGINS-SANDBOX-0070` | `backend-rs/tests/runtime_extensions_parity.rs::execution_workspaces_project_execution_workspace_policy_and_provision_command` | Project execution workspace policy and provision command |
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.md:107:| `plugins-plugin-manifest-namespace-trust-boundary` | `missing` | `phases/03-workspaces-routines-plugins-sandbox.md` | `ACC-03-WORKSPACES-ROUTINES-PLUGINS-SANDBOX-0071` | `backend-rs/tests/runtime_extensions_parity.rs::plugins_plugin_manifest_namespace_trust_boundary` | Plugin manifest, namespace, trust boundary |
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.md:108:| `routines-routine-schedule-webhook-api-dispatch-env-overlay-snapshot` | `missing` | `phases/03-workspaces-routines-plugins-sandbox.md` | `ACC-03-WORKSPACES-ROUTINES-PLUGINS-SANDBOX-0072` | `backend-rs/tests/runtime_extensions_parity.rs::routines_routine_schedule_webhook_api_dispatch_env_overlay_snapshot` | Routine schedule, webhook/API dispatch, env overlay snapshot |
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.md:109:| `runtime-services-managed-runtime-preview-dev-service-lifecycle` | `missing` | `phases/03-workspaces-routines-plugins-sandbox.md` | `ACC-03-WORKSPACES-ROUTINES-PLUGINS-SANDBOX-0073` | `backend-rs/tests/runtime_extensions_parity.rs::runtime_services_managed_runtime_preview_dev_service_lifecycle` | Managed runtime/preview/dev service lifecycle |
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.md:110:| `sandbox-sandbox-provider-containment-and-low-trust-boundaries` | `missing` | `phases/03-workspaces-routines-plugins-sandbox.md` | `ACC-03-WORKSPACES-ROUTINES-PLUGINS-SANDBOX-0074` | `backend-rs/tests/runtime_extensions_parity.rs::sandbox_sandbox_provider_containment_and_low_trust_boundaries` | Sandbox provider containment and low-trust boundaries |
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.md:111:| `deployment-modes-authenticated-private-and-public-exposure-policy` | `missing` | `phases/04-deployment-auth-onboarding-ops.md` | `ACC-04-DEPLOYMENT-AUTH-ONBOARDING-OPS-0075` | `backend-rs/tests/deployment_auth_parity.rs::deployment_modes_authenticated_private_and_public_exposure_policy` | authenticated private and public exposure policy |
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.md:112:| `deployment-modes-local-trusted-implicit-board-behavior` | `missing` | `phases/04-deployment-auth-onboarding-ops.md` | `ACC-04-DEPLOYMENT-AUTH-ONBOARDING-OPS-0076` | `backend-rs/tests/deployment_auth_parity.rs::deployment_modes_local_trusted_implicit_board_behavior` | local_trusted implicit board behavior |
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.md:113:| `deployment-modes-public-authenticated-deployment-refuses-embedded-db-fallback` | `missing` | `phases/04-deployment-auth-onboarding-ops.md` | `ACC-04-DEPLOYMENT-AUTH-ONBOARDING-OPS-0077` | `backend-rs/tests/deployment_auth_parity.rs::deployment_modes_public_authenticated_deployment_refuses_embedded_db_fallback` | Public authenticated deployment refuses embedded DB fallback |
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.md:114:| `onboarding-first-admin-bootstrap-and-claim-flow-without-supabase-exposure` | `missing` | `phases/04-deployment-auth-onboarding-ops.md` | `ACC-04-DEPLOYMENT-AUTH-ONBOARDING-OPS-0078` | `backend-rs/tests/deployment_auth_parity.rs::onboarding_first_admin_bootstrap_and_claim_flow_without_supabase_exposure` | First-admin bootstrap and claim flow without Supabase exposure |
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.md:115:| `ops-doctor-onboard-backup-storage-config-admin-surfaces` | `missing` | `phases/04-deployment-auth-onboarding-ops.md` | `ACC-04-DEPLOYMENT-AUTH-ONBOARDING-OPS-0079` | `backend-rs/tests/deployment_auth_parity.rs::ops_doctor_onboard_backup_storage_config_admin_surfaces` | Doctor, onboard, backup, storage, config admin surfaces |
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.md:116:| `ops-instance-settings-and-sidebar-preferences` | `missing` | `phases/04-deployment-auth-onboarding-ops.md` | `ACC-04-DEPLOYMENT-AUTH-ONBOARDING-OPS-0080` | `backend-rs/tests/deployment_auth_parity.rs::ops_instance_settings_and_sidebar_preferences` | Instance settings and sidebar preferences |
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.md:117:| `hiring-flow-side-effects-adapter-defaults-injection` | `missing` | `phases/05-adapters-secrets-skills-config.md` | `ACC-05-ADAPTERS-SECRETS-SKILLS-CONFIG-0081` | `backend-rs/tests/adapter_config_parity.rs::hiring_flow_side_effects_adapter_defaults_injection` | Adapter-type default config injection |
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.md:118:| `hiring-flow-side-effects-adapter-type-validation` | `missing` | `phases/05-adapters-secrets-skills-config.md` | `ACC-05-ADAPTERS-SECRETS-SKILLS-CONFIG-0082` | `backend-rs/tests/adapter_config_parity.rs::hiring_flow_side_effects_adapter_type_validation` | Adapter type must be a registered server adapter |
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.md:119:| `hiring-flow-side-effects-agent-actor-config-guard` | `missing` | `phases/05-adapters-secrets-skills-config.md` | `ACC-05-ADAPTERS-SECRETS-SKILLS-CONFIG-0083` | `backend-rs/tests/adapter_config_parity.rs::hiring_flow_side_effects_agent_actor_config_guard` | Agent-authenticated callers cannot set instructions/workspace-command config |
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.md:122:| `hiring-flow-side-effects-codex-key-isolation` | `missing` | `phases/05-adapters-secrets-skills-config.md` | `ACC-05-ADAPTERS-SECRETS-SKILLS-CONFIG-0086` | `backend-rs/tests/adapter_config_parity.rs::hiring_flow_side_effects_codex_key_isolation` | codex_local per-agent CODEX_HOME isolation |
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.md:123:| `hiring-flow-side-effects-desired-skill-assignment` | `missing` | `phases/05-adapters-secrets-skills-config.md` | `ACC-05-ADAPTERS-SECRETS-SKILLS-CONFIG-0087` | `backend-rs/tests/adapter_config_parity.rs::hiring_flow_side_effects_desired_skill_assignment` | Desired skills resolution + skill sync preference persisted |
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.md:124:| `hiring-flow-side-effects-hire-hook-onhireapproved` | `missing` | `phases/05-adapters-secrets-skills-config.md` | `ACC-05-ADAPTERS-SECRETS-SKILLS-CONFIG-0088` | `backend-rs/tests/adapter_config_parity.rs::hiring_flow_side_effects_hire_hook_onhireapproved` | onHireApproved adapter hook (hire-hook.ts) |
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.md:125:| `hiring-flow-side-effects-instructions-bundle-materialization` | `missing` | `phases/05-adapters-secrets-skills-config.md` | `ACC-05-ADAPTERS-SECRETS-SKILLS-CONFIG-0089` | `backend-rs/tests/adapter_config_parity.rs::hiring_flow_side_effects_instructions_bundle_materialization` | Default instructions bundle materialized on hire |
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.md:127:| `hiring-flow-side-effects-runtime-heartbeat-normalization` | `missing` | `phases/05-adapters-secrets-skills-config.md` | `ACC-05-ADAPTERS-SECRETS-SKILLS-CONFIG-0091` | `backend-rs/tests/adapter_config_parity.rs::hiring_flow_side_effects_runtime_heartbeat_normalization` | Runtime config heartbeat defaults |
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.md:128:| `hiring-flow-side-effects-secret-normalization` | `missing` | `phases/05-adapters-secrets-skills-config.md` | `ACC-05-ADAPTERS-SECRETS-SKILLS-CONFIG-0092` | `backend-rs/tests/adapter_config_parity.rs::hiring_flow_side_effects_secret_normalization` | Adapter/runtime config secret normalization + constraints |
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.md:130:| `approvals-system-governance-approval-comments-table` | `missing` | `phases/06-approvals-budgets-costs-activity.md` | `ACC-06-APPROVALS-BUDGETS-COSTS-ACTIVITY-0094` | `backend-rs/tests/approvals_parity.rs::approvals_system_governance_approval_comments_table` | approval_comments table + comment endpoints |
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.md:135:| `approvals-system-governance-approve-effect-requester-wakeup` | `missing` | `phases/06-approvals-budgets-costs-activity.md` | `ACC-06-APPROVALS-BUDGETS-COSTS-ACTIVITY-0099` | `backend-rs/tests/approvals_parity.rs::approvals_system_governance_approve_effect_requester_wakeup` | approve effect: wake requesting agent + activity logs |
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.md:136:| `approvals-system-governance-approve-lifecycle` | `divergent` | `phases/06-approvals-budgets-costs-activity.md` | `ACC-06-APPROVALS-BUDGETS-COSTS-ACTIVITY-0100` | `backend-rs/tests/approvals_parity.rs::approvals_system_governance_approve_lifecycle` | Approve transition + idempotency |
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.md:137:| `approvals-system-governance-board-chat-issue-persistence` | `missing` | `phases/06-approvals-budgets-costs-activity.md` | `ACC-06-APPROVALS-BUDGETS-COSTS-ACTIVITY-0101` | `backend-rs/tests/approvals_parity.rs::approvals_system_governance_board_chat_issue_persistence` | Board chat persisted to standing 'Board Operations' issue |
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.md:138:| `approvals-system-governance-board-chat-relay` | `missing` | `phases/06-approvals-budgets-costs-activity.md` | `ACC-06-APPROVALS-BUDGETS-COSTS-ACTIVITY-0102` | `backend-rs/tests/approvals_parity.rs::approvals_system_governance_board_chat_relay` | Board Concierge Chat SSE relay |
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.md:141:| `approvals-system-governance-find-open-hire-approval` | `missing` | `phases/06-approvals-budgets-costs-activity.md` | `ACC-06-APPROVALS-BUDGETS-COSTS-ACTIVITY-0105` | `backend-rs/tests/approvals_parity.rs::approvals_system_governance_find_open_hire_approval` | findOpenHireApprovalForAgent helper |
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.md:142:| `approvals-system-governance-get-approval` | `missing` | `phases/06-approvals-budgets-costs-activity.md` | `ACC-06-APPROVALS-BUDGETS-COSTS-ACTIVITY-0106` | `backend-rs/tests/approvals_parity.rs::approvals_system_governance_get_approval` | Get single approval / linked issues |
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.md:144:| `approvals-system-governance-payload-redaction` | `missing` | `phases/06-approvals-budgets-costs-activity.md` | `ACC-06-APPROVALS-BUDGETS-COSTS-ACTIVITY-0108` | `backend-rs/tests/approvals_parity.rs::approvals_system_governance_payload_redaction` | Approval payload + comment redaction on read |
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.md:146:| `approvals-system-governance-request-revision` | `missing` | `phases/06-approvals-budgets-costs-activity.md` | `ACC-06-APPROVALS-BUDGETS-COSTS-ACTIVITY-0110` | `backend-rs/tests/approvals_parity.rs::approvals_system_governance_request_revision` | Request revision transition (board only, pending→revision_requested) |
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.md:148:| `approvals-system-governance-resubmit` | `missing` | `phases/06-approvals-budgets-costs-activity.md` | `ACC-06-APPROVALS-BUDGETS-COSTS-ACTIVITY-0112` | `backend-rs/tests/approvals_parity.rs::approvals_system_governance_resubmit` | Resubmit (revision_requested→pending), requester-only |
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.md:149:| `approvals-system-governance-run-context-mutation-guard` | `missing` | `phases/06-approvals-budgets-costs-activity.md` | `ACC-06-APPROVALS-BUDGETS-COSTS-ACTIVITY-0113` | `backend-rs/tests/approvals_parity.rs::approvals_system_governance_run_context_mutation_guard` | Cheap status-only recovery run cannot mutate approvals |
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.md:155:| `agent-lifecycle-status-transitions-error-state-and-clear-error` | `missing` | `phases/07-agent-runtime-lifecycle.md` | `ACC-07-AGENT-RUNTIME-LIFECYCLE-0119` | `backend-rs/tests/agent_lifecycle_parity.rs::agent_lifecycle_status_transitions_error_state_and_clear_error` | Error state and clear-error |
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.md:157:| `agent-lifecycle-status-transitions-heartbeat-run-wakeup-request-log-ev` | `missing` | `phases/07-agent-runtime-lifecycle.md` | `ACC-07-AGENT-RUNTIME-LIFECYCLE-0121` | `backend-rs/tests/agent_lifecycle_parity.rs::agent_lifecycle_status_transitions_heartbeat_run_wakeup_request_log_ev` | Heartbeat run, wakeup request, log/event, live-run, and cancellation model |
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.md:159:| `agent-lifecycle-status-transitions-pause-agent` | `missing` | `phases/07-agent-runtime-lifecycle.md` | `ACC-07-AGENT-RUNTIME-LIFECYCLE-0123` | `backend-rs/tests/agent_lifecycle_parity.rs::agent_lifecycle_status_transitions_pause_agent` | Pause agent |
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.md:161:| `agent-lifecycle-status-transitions-resume-agent` | `missing` | `phases/07-agent-runtime-lifecycle.md` | `ACC-07-AGENT-RUNTIME-LIFECYCLE-0125` | `backend-rs/tests/agent_lifecycle_parity.rs::agent_lifecycle_status_transitions_resume_agent` | Resume agent |
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.md:162:| `agent-lifecycle-status-transitions-runtime-state-task-sessions-and-ses` | `missing` | `phases/07-agent-runtime-lifecycle.md` | `ACC-07-AGENT-RUNTIME-LIFECYCLE-0126` | `backend-rs/tests/agent_lifecycle_parity.rs::agent_lifecycle_status_transitions_runtime_state_task_sessions_and_ses` | Runtime state, task sessions, and session reset |
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.md:163:| `agent-lifecycle-status-transitions-runtime-transition-idle-running-idl` | `missing` | `phases/07-agent-runtime-lifecycle.md` | `ACC-07-AGENT-RUNTIME-LIFECYCLE-0127` | `backend-rs/tests/agent_lifecycle_parity.rs::agent_lifecycle_status_transitions_runtime_transition_idle_running_idl` | Runtime transition idle → running → idle/error |
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.md:164:| `agent-lifecycle-status-transitions-terminate-fire-active-agents` | `missing` | `phases/07-agent-runtime-lifecycle.md` | `ACC-07-AGENT-RUNTIME-LIFECYCLE-0128` | `backend-rs/tests/agent_lifecycle_parity.rs::agent_lifecycle_status_transitions_terminate_fire_active_agents` | Terminate/fire active agents |
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.md:165:| `agent-lifecycle-status-transitions-wakeup-heartbeat-invoke` | `missing` | `phases/07-agent-runtime-lifecycle.md` | `ACC-07-AGENT-RUNTIME-LIFECYCLE-0129` | `backend-rs/tests/agent_lifecycle_parity.rs::agent_lifecycle_status_transitions_wakeup_heartbeat_invoke` | Wakeup / heartbeat invoke |
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.md:168:| `cli-api-surface-agent-users-api-agent-keys` | `missing` | `phases/08-interfaces-rest-cli-mcp.md` | `ACC-08-INTERFACES-REST-CLI-MCP-0132` | `backend-rs/tests/api_surface_parity.rs::cli_api_surface_agent_users_api_agent_keys` | Agent API key endpoints |
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.md:169:| `cli-api-surface-agent-users-api-agent-lifecycle-simple` | `missing` | `phases/08-interfaces-rest-cli-mcp.md` | `ACC-08-INTERFACES-REST-CLI-MCP-0133` | `backend-rs/tests/api_surface_parity.rs::cli_api_surface_agent_users_api_agent_lifecycle_simple` | pause/resume/clear-error/delete |
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.md:170:| `cli-api-surface-agent-users-api-agent-me` | `missing` | `phases/08-interfaces-rest-cli-mcp.md` | `ACC-08-INTERFACES-REST-CLI-MCP-0134` | `backend-rs/tests/api_surface_parity.rs::cli_api_surface_agent_users_api_agent_me` | GET /api/agents/me + inbox |
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.md:171:| `cli-api-surface-agent-users-api-agent-permissions` | `missing` | `phases/08-interfaces-rest-cli-mcp.md` | `ACC-08-INTERFACES-REST-CLI-MCP-0135` | `backend-rs/tests/api_surface_parity.rs::cli_api_surface_agent_users_api_agent_permissions` | PATCH /api/agents/:id/permissions |
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.md:173:| `cli-api-surface-agent-users-api-agent-wakeup` | `missing` | `phases/08-interfaces-rest-cli-mcp.md` | `ACC-08-INTERFACES-REST-CLI-MCP-0137` | `backend-rs/tests/api_surface_parity.rs::cli_api_surface_agent_users_api_agent_wakeup` | POST /api/agents/:id/wakeup + /heartbeat/invoke |
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.md:175:| `cli-api-surface-agent-users-api-approvals-create` | `missing` | `phases/08-interfaces-rest-cli-mcp.md` | `ACC-08-INTERFACES-REST-CLI-MCP-0139` | `backend-rs/tests/api_surface_parity.rs::cli_api_surface_agent_users_api_approvals_create` | POST /api/companies/:companyId/approvals |
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.md:178:| `cli-api-surface-agent-users-api-approvals-resubmit-comment` | `missing` | `phases/08-interfaces-rest-cli-mcp.md` | `ACC-08-INTERFACES-REST-CLI-MCP-0142` | `backend-rs/tests/api_surface_parity.rs::cli_api_surface_agent_users_api_approvals_resubmit_comment` | resubmit + comments |
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.md:180:| `cli-api-surface-agent-users-authz-read-config` | `missing` | `phases/08-interfaces-rest-cli-mcp.md` | `ACC-08-INTERFACES-REST-CLI-MCP-0144` | `backend-rs/tests/api_surface_parity.rs::cli_api_surface_agent_users_authz_read_config` | config-read authorization |
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.md:181:| `cli-api-surface-agent-users-cli-agent-create` | `missing` | `phases/08-interfaces-rest-cli-mcp.md` | `ACC-08-INTERFACES-REST-CLI-MCP-0145` | `backend-rs/tests/api_surface_parity.rs::cli_api_surface_agent_users_cli_agent_create` | CLI `agent create` |
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.md:184:| `cli-api-surface-agent-users-cli-agent-inbox` | `missing` | `phases/08-interfaces-rest-cli-mcp.md` | `ACC-08-INTERFACES-REST-CLI-MCP-0148` | `backend-rs/tests/api_surface_parity.rs::cli_api_surface_agent_users_cli_agent_inbox` | CLI agent inbox commands |
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.md:185:| `cli-api-surface-agent-users-cli-agent-instructions` | `missing` | `phases/08-interfaces-rest-cli-mcp.md` | `ACC-08-INTERFACES-REST-CLI-MCP-0149` | `backend-rs/tests/api_surface_parity.rs::cli_api_surface_agent_users_cli_agent_instructions` | CLI instructions path/bundle/file |
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.md:186:| `cli-api-surface-agent-users-cli-agent-lifecycle` | `missing` | `phases/08-interfaces-rest-cli-mcp.md` | `ACC-08-INTERFACES-REST-CLI-MCP-0150` | `backend-rs/tests/api_surface_parity.rs::cli_api_surface_agent_users_cli_agent_lifecycle` | CLI agent lifecycle verbs |
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.md:187:| `cli-api-surface-agent-users-cli-agent-local-cli` | `missing` | `phases/08-interfaces-rest-cli-mcp.md` | `ACC-08-INTERFACES-REST-CLI-MCP-0151` | `backend-rs/tests/api_surface_parity.rs::cli_api_surface_agent_users_cli_agent_local_cli` | CLI `agent local-cli` |
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.md:188:| `cli-api-surface-agent-users-cli-agent-permissions-config` | `missing` | `phases/08-interfaces-rest-cli-mcp.md` | `ACC-08-INTERFACES-REST-CLI-MCP-0152` | `backend-rs/tests/api_surface_parity.rs::cli_api_surface_agent_users_cli_agent_permissions_config` | CLI permissions/configuration/revisions |
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.md:189:| `cli-api-surface-agent-users-cli-agent-wake` | `missing` | `phases/08-interfaces-rest-cli-mcp.md` | `ACC-08-INTERFACES-REST-CLI-MCP-0153` | `backend-rs/tests/api_surface_parity.rs::cli_api_surface_agent_users_cli_agent_wake` | CLI `agent wake` |
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.md:196:| `cli-api-surface-agent-users-side-effect-task-assign-grant` | `missing` | `phases/08-interfaces-rest-cli-mcp.md` | `ACC-08-INTERFACES-REST-CLI-MCP-0160` | `backend-rs/tests/api_surface_parity.rs::cli_api_surface_agent_users_side_effect_task_assign_grant` | Default tasks:assign grant on hire/create |
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.md:197:| `ui-surface-human-non-agent-users-agent-action-assign-task` | `missing` | `phases/09-nuxt-board-ui-parity.md` | `ACC-09-NUXT-BOARD-UI-PARITY-0161` | `frontend-nuxt/tests/ui-parity.test.ts::ui_surface_human_non_agent_users_agent_action_assign_task` | Assign Task action |
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.md:198:| `ui-surface-human-non-agent-users-agent-action-clear-error` | `missing` | `phases/09-nuxt-board-ui-parity.md` | `ACC-09-NUXT-BOARD-UI-PARITY-0162` | `frontend-nuxt/tests/ui-parity.test.ts::ui_surface_human_non_agent_users_agent_action_clear_error` | Clear error action |
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.md:199:| `ui-surface-human-non-agent-users-agent-action-invoke` | `missing` | `phases/09-nuxt-board-ui-parity.md` | `ACC-09-NUXT-BOARD-UI-PARITY-0163` | `frontend-nuxt/tests/ui-parity.test.ts::ui_surface_human_non_agent_users_agent_action_invoke` | Run heartbeat / invoke action |
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.md:200:| `ui-surface-human-non-agent-users-agent-action-overflow-menu` | `missing` | `phases/09-nuxt-board-ui-parity.md` | `ACC-09-NUXT-BOARD-UI-PARITY-0164` | `frontend-nuxt/tests/ui-parity.test.ts::ui_surface_human_non_agent_users_agent_action_overflow_menu` | Overflow menu: duplicate / copy ID / reset sessions / terminate |
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.md:201:| `ui-surface-human-non-agent-users-agent-action-pause-resume` | `missing` | `phases/09-nuxt-board-ui-parity.md` | `ACC-09-NUXT-BOARD-UI-PARITY-0165` | `frontend-nuxt/tests/ui-parity.test.ts::ui_surface_human_non_agent_users_agent_action_pause_resume` | Pause / Resume action |
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.md:202:| `ui-surface-human-non-agent-users-agent-budget-tab` | `missing` | `phases/09-nuxt-board-ui-parity.md` | `ACC-09-NUXT-BOARD-UI-PARITY-0166` | `frontend-nuxt/tests/ui-parity.test.ts::ui_surface_human_non_agent_users_agent_budget_tab` | Per-agent budget policy |
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.md:203:| `ui-surface-human-non-agent-users-agent-config-secret-redaction` | `missing` | `phases/09-nuxt-board-ui-parity.md` | `ACC-09-NUXT-BOARD-UI-PARITY-0167` | `frontend-nuxt/tests/ui-parity.test.ts::ui_surface_human_non_agent_users_agent_config_secret_redaction` | Run log/env secret + path redaction in UI |
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.md:204:| `ui-surface-human-non-agent-users-agent-dashboard-overview` | `missing` | `phases/09-nuxt-board-ui-parity.md` | `ACC-09-NUXT-BOARD-UI-PARITY-0168` | `frontend-nuxt/tests/ui-parity.test.ts::ui_surface_human_non_agent_users_agent_dashboard_overview` | Agent dashboard overview |
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.md:205:| `ui-surface-human-non-agent-users-agent-detail-tabs` | `missing` | `phases/09-nuxt-board-ui-parity.md` | `ACC-09-NUXT-BOARD-UI-PARITY-0169` | `frontend-nuxt/tests/ui-parity.test.ts::ui_surface_human_non_agent_users_agent_detail_tabs` | Agent detail tabbed page |
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.md:206:| `ui-surface-human-non-agent-users-agent-icon-edit` | `missing` | `phases/09-nuxt-board-ui-parity.md` | `ACC-09-NUXT-BOARD-UI-PARITY-0170` | `frontend-nuxt/tests/ui-parity.test.ts::ui_surface_human_non_agent_users_agent_icon_edit` | Edit agent icon |
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.md:207:| `ui-surface-human-non-agent-users-agent-invalid-org-chain` | `missing` | `phases/09-nuxt-board-ui-parity.md` | `ACC-09-NUXT-BOARD-UI-PARITY-0171` | `frontend-nuxt/tests/ui-parity.test.ts::ui_surface_human_non_agent_users_agent_invalid_org_chain` | Invalid reporting-chain warning + gating |
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.md:208:| `ui-surface-human-non-agent-users-agent-membership-join-leave` | `missing` | `phases/09-nuxt-board-ui-parity.md` | `ACC-09-NUXT-BOARD-UI-PARITY-0172` | `frontend-nuxt/tests/ui-parity.test.ts::ui_surface_human_non_agent_users_agent_membership_join_leave` | Join/leave agent membership |
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.md:210:| `ui-surface-human-non-agent-users-agent-permissions-update` | `missing` | `phases/09-nuxt-board-ui-parity.md` | `ACC-09-NUXT-BOARD-UI-PARITY-0174` | `frontend-nuxt/tests/ui-parity.test.ts::ui_surface_human_non_agent_users_agent_permissions_update` | Update agent permissions/trust |
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.md:211:| `ui-surface-human-non-agent-users-agents-api-surface` | `partial` | `phases/09-nuxt-board-ui-parity.md` | `ACC-09-NUXT-BOARD-UI-PARITY-0175` | `frontend-nuxt/tests/ui-parity.test.ts::ui_surface_human_non_agent_users_agents_api_surface` | Agents REST API client surface |
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.md:213:| `ui-surface-human-non-agent-users-agents-live-run-indicator` | `missing` | `phases/09-nuxt-board-ui-parity.md` | `ACC-09-NUXT-BOARD-UI-PARITY-0177` | `frontend-nuxt/tests/ui-parity.test.ts::ui_surface_human_non_agent_users_agents_live_run_indicator` | Live run indicator |
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.md:214:| `ui-surface-human-non-agent-users-agents-org-chart` | `missing` | `phases/09-nuxt-board-ui-parity.md` | `ACC-09-NUXT-BOARD-UI-PARITY-0178` | `frontend-nuxt/tests/ui-parity.test.ts::ui_surface_human_non_agent_users_agents_org_chart` | Org chart (tree) view |
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.md:215:| `ui-surface-human-non-agent-users-agents-status-filter-tabs` | `missing` | `phases/09-nuxt-board-ui-parity.md` | `ACC-09-NUXT-BOARD-UI-PARITY-0179` | `frontend-nuxt/tests/ui-parity.test.ts::ui_surface_human_non_agent_users_agents_status_filter_tabs` | Agent status filter tabs |
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.md:217:| `ui-surface-human-non-agent-users-approval-budget-special-case` | `missing` | `phases/09-nuxt-board-ui-parity.md` | `ACC-09-NUXT-BOARD-UI-PARITY-0181` | `frontend-nuxt/tests/ui-parity.test.ts::ui_surface_human_non_agent_users_approval_budget_special_case` | Budget override approval redirect |
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.md:218:| `ui-surface-human-non-agent-users-approval-comments` | `missing` | `phases/09-nuxt-board-ui-parity.md` | `ACC-09-NUXT-BOARD-UI-PARITY-0182` | `frontend-nuxt/tests/ui-parity.test.ts::ui_surface_human_non_agent_users_approval_comments` | Approval comments thread |
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.md:220:| `ui-surface-human-non-agent-users-approval-detail-view` | `missing` | `phases/09-nuxt-board-ui-parity.md` | `ACC-09-NUXT-BOARD-UI-PARITY-0184` | `frontend-nuxt/tests/ui-parity.test.ts::ui_surface_human_non_agent_users_approval_detail_view` | Approval detail page |
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.md:221:| `ui-surface-human-non-agent-users-approval-payload-renderers` | `missing` | `phases/09-nuxt-board-ui-parity.md` | `ACC-09-NUXT-BOARD-UI-PARITY-0185` | `frontend-nuxt/tests/ui-parity.test.ts::ui_surface_human_non_agent_users_approval_payload_renderers` | Typed approval payload renderers |
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.md:222:| `ui-surface-human-non-agent-users-approval-revision-resubmit` | `missing` | `phases/09-nuxt-board-ui-parity.md` | `ACC-09-NUXT-BOARD-UI-PARITY-0186` | `frontend-nuxt/tests/ui-parity.test.ts::ui_surface_human_non_agent_users_approval_revision_resubmit` | Request revision / Mark resubmitted |
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.md:223:| `ui-surface-human-non-agent-users-board-chat-conference-room` | `missing` | `phases/09-nuxt-board-ui-parity.md` | `ACC-09-NUXT-BOARD-UI-PARITY-0187` | `frontend-nuxt/tests/ui-parity.test.ts::ui_surface_human_non_agent_users_board_chat_conference_room` | Board Concierge chat (Conference Room) |
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.md:224:| `ui-surface-human-non-agent-users-board-chat-feedback-votes` | `missing` | `phases/09-nuxt-board-ui-parity.md` | `ACC-09-NUXT-BOARD-UI-PARITY-0188` | `frontend-nuxt/tests/ui-parity.test.ts::ui_surface_human_non_agent_users_board_chat_feedback_votes` | Board chat message feedback votes |
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.md:225:| `ui-surface-human-non-agent-users-company-archive` | `missing` | `phases/09-nuxt-board-ui-parity.md` | `ACC-09-NUXT-BOARD-UI-PARITY-0189` | `frontend-nuxt/tests/ui-parity.test.ts::ui_surface_human_non_agent_users_company_archive` | Archive company (danger zone) |
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.md:227:| `ui-surface-human-non-agent-users-company-general-settings` | `missing` | `phases/09-nuxt-board-ui-parity.md` | `ACC-09-NUXT-BOARD-UI-PARITY-0191` | `frontend-nuxt/tests/ui-parity.test.ts::ui_surface_human_non_agent_users_company_general_settings` | Company general/appearance settings |
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.md:228:| `ui-surface-human-non-agent-users-company-packages-links` | `missing` | `phases/09-nuxt-board-ui-parity.md` | `ACC-09-NUXT-BOARD-UI-PARITY-0192` | `frontend-nuxt/tests/ui-parity.test.ts::ui_surface_human_non_agent_users_company_packages_links` | Company import/export/cloud links |
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.md:229:| `ui-surface-human-non-agent-users-first-agent-is-ceo` | `missing` | `phases/09-nuxt-board-ui-parity.md` | `ACC-09-NUXT-BOARD-UI-PARITY-0193` | `frontend-nuxt/tests/ui-parity.test.ts::ui_surface_human_non_agent_users_first_agent_is_ceo` | First agent forced to CEO |
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.md:230:| `ui-surface-human-non-agent-users-hire-adapter-presets` | `missing` | `phases/09-nuxt-board-ui-parity.md` | `ACC-09-NUXT-BOARD-UI-PARITY-0194` | `frontend-nuxt/tests/ui-parity.test.ts::ui_surface_human_non_agent_users_hire_adapter_presets` | Adapter-type presets and validation |
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.md:231:| `ui-surface-human-non-agent-users-hire-company-skills-select` | `missing` | `phases/09-nuxt-board-ui-parity.md` | `ACC-09-NUXT-BOARD-UI-PARITY-0195` | `frontend-nuxt/tests/ui-parity.test.ts::ui_surface_human_non_agent_users_hire_company_skills_select` | Optional company-skill selection on hire |
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.md:232:| `ui-surface-human-non-agent-users-hire-test-agent` | `missing` | `phases/09-nuxt-board-ui-parity.md` | `ACC-09-NUXT-BOARD-UI-PARITY-0196` | `frontend-nuxt/tests/ui-parity.test.ts::ui_surface_human_non_agent_users_hire_test_agent` | Test Agent (adapter environment probe) button |
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.md:233:| `ui-surface-human-non-agent-users-hire-trust-preset` | `missing` | `phases/09-nuxt-board-ui-parity.md` | `ACC-09-NUXT-BOARD-UI-PARITY-0197` | `frontend-nuxt/tests/ui-parity.test.ts::ui_surface_human_non_agent_users_hire_trust_preset` | Trust preset + low-trust boundary picker on hire |
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.md:234:| `ui-surface-human-non-agent-users-invite-external-agent` | `missing` | `phases/09-nuxt-board-ui-parity.md` | `ACC-09-NUXT-BOARD-UI-PARITY-0198` | `frontend-nuxt/tests/ui-parity.test.ts::ui_surface_human_non_agent_users_invite_external_agent` | External agent invite + onboarding prompt |
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.md:235:| `ui-surface-human-non-agent-users-new-agent-dialog-choices` | `missing` | `phases/09-nuxt-board-ui-parity.md` | `ACC-09-NUXT-BOARD-UI-PARITY-0199` | `frontend-nuxt/tests/ui-parity.test.ts::ui_surface_human_non_agent_users_new_agent_dialog_choices` | New Agent dialog with 4 creation paths |
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.md:237:| `catalogs-skills-catalog-validation-and-source-policy` | `missing` | `phases/10-import-export-catalogs-portability.md` | `ACC-10-IMPORT-EXPORT-CATALOGS-PORTABILITY-0201` | `backend-rs/tests/import_export_parity.rs::catalogs_skills_catalog_validation_and_source_policy` | Skills catalog validation and source policy |
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.md:238:| `catalogs-teams-catalog-validation-and-source-policy` | `missing` | `phases/10-import-export-catalogs-portability.md` | `ACC-10-IMPORT-EXPORT-CATALOGS-PORTABILITY-0202` | `backend-rs/tests/import_export_parity.rs::catalogs_teams_catalog_validation_and_source_policy` | Teams catalog validation and source policy |
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.md:239:| `portability-company-export-preview-apply-import-round-trip` | `missing` | `phases/10-import-export-catalogs-portability.md` | `ACC-10-IMPORT-EXPORT-CATALOGS-PORTABILITY-0203` | `backend-rs/tests/import_export_parity.rs::portability_company_export_preview_apply_import_round_trip` | Company export preview/apply/import round trip |
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.md:240:| `portability-export-bundle-secret-redaction-and-no-key-hash-leakage` | `missing` | `phases/10-import-export-catalogs-portability.md` | `ACC-10-IMPORT-EXPORT-CATALOGS-PORTABILITY-0204` | `backend-rs/tests/import_export_parity.rs::portability_export_bundle_secret_redaction_and_no_key_hash_leakage` | Export bundle secret redaction and no key_hash leakage |
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.md:241:| `templates-reusable-company-team-templates-and-onboarding-packages` | `missing` | `phases/10-import-export-catalogs-portability.md` | `ACC-10-IMPORT-EXPORT-CATALOGS-PORTABILITY-0205` | `backend-rs/tests/import_export_parity.rs::templates_reusable_company_team_templates_and_onboarding_packages` | Reusable company/team templates and onboarding packages |
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.md:242:| `observability-dashboard-health-doctor-observability` | `missing` | `phases/11-observability-feedback-pipelines.md` | `ACC-11-OBSERVABILITY-FEEDBACK-PIPELINES-0206` | `backend-rs/tests/observability_parity.rs::observability_dashboard_health_doctor_observability` | Dashboard, health, doctor observability |
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.md:243:| `observability-feedback-exports-and-votes` | `missing` | `phases/11-observability-feedback-pipelines.md` | `ACC-11-OBSERVABILITY-FEEDBACK-PIPELINES-0207` | `backend-rs/tests/observability_parity.rs::observability_feedback_exports_and_votes` | Feedback exports and votes |
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.md:244:| `observability-pipeline-cases-events-outputs-conversation-context` | `missing` | `phases/11-observability-feedback-pipelines.md` | `ACC-11-OBSERVABILITY-FEEDBACK-PIPELINES-0208` | `backend-rs/tests/observability_parity.rs::observability_pipeline_cases_events_outputs_conversation_context` | Pipeline cases, events, outputs, conversation context |
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.md:245:| `observability-productivity-review-states` | `missing` | `phases/11-observability-feedback-pipelines.md` | `ACC-11-OBSERVABILITY-FEEDBACK-PIPELINES-0209` | `backend-rs/tests/observability_parity.rs::observability_productivity_review_states` | Productivity review states |
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.md:246:| `observability-release-and-e2e-smoke-result-ingestion` | `missing` | `phases/11-observability-feedback-pipelines.md` | `ACC-11-OBSERVABILITY-FEEDBACK-PIPELINES-0210` | `backend-rs/tests/observability_parity.rs::observability_release_and_e2e_smoke_result_ingestion` | Release and e2e smoke result ingestion |
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.md:247:| `contracts-openapi-schema-export-and-generated-clients` | `missing` | `phases/12-shared-contract-docs-parity.md` | `ACC-12-SHARED-CONTRACT-DOCS-PARITY-0211` | `backend-rs/tests/shared_contract_parity.rs::contracts_openapi_schema_export_and_generated_clients` | OpenAPI/schema export and generated clients |
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.md:248:| `contracts-packages-shared-constants-validators-api-path-parity-surface` | `missing` | `phases/12-shared-contract-docs-parity.md` | `ACC-12-SHARED-CONTRACT-DOCS-PARITY-0212` | `backend-rs/tests/shared_contract_parity.rs::contracts_packages_shared_constants_validators_api_path_parity_surface` | packages/shared constants, validators, API path parity surface |
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.md:249:| `docs-internal-doc-architecture-records-kept-current` | `missing` | `phases/12-shared-contract-docs-parity.md` | `ACC-12-SHARED-CONTRACT-DOCS-PARITY-0213` | `backend-rs/tests/shared_contract_parity.rs::docs_internal_doc_architecture_records_kept_current` | Internal doc architecture records kept current |
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.md:250:| `docs-no-internal-ids-secrets-tailnet-or-agent-urls-in-public-docs` | `missing` | `phases/12-shared-contract-docs-parity.md` | `ACC-12-SHARED-CONTRACT-DOCS-PARITY-0214` | `backend-rs/tests/shared_contract_parity.rs::docs_no_internal_ids_secrets_tailnet_or_agent_urls_in_public_docs` | No internal IDs, secrets, tailnet, or agent URLs in public docs |
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.md:251:| `docs-public-docs-navigation-docs-docs-json-accuracy` | `missing` | `phases/12-shared-contract-docs-parity.md` | `ACC-12-SHARED-CONTRACT-DOCS-PARITY-0215` | `backend-rs/tests/shared_contract_parity.rs::docs_public_docs_navigation_docs_docs_json_accuracy` | Public docs navigation docs/docs.json accuracy |
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.md:252:| `cutover-compatibility-views-and-rpc-aliases-during-transition` | `missing` | `phases/13-migration-compatibility-cutover.md` | `ACC-13-MIGRATION-COMPATIBILITY-CUTOVER-0216` | `acceptance/run_original_gap_diff.py::cutover_compatibility_views_and_rpc_aliases_during_transition` | Compatibility views and RPC aliases during transition |
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.md:253:| `cutover-feature-flags-and-strangler-routing-between-old-and-new-surfaces` | `missing` | `phases/13-migration-compatibility-cutover.md` | `ACC-13-MIGRATION-COMPATIBILITY-CUTOVER-0217` | `acceptance/run_original_gap_diff.py::cutover_feature_flags_and_strangler_routing_between_old_and_new_surfaces` | Feature flags and strangler routing between old and new surfaces |
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.md:254:| `cutover-forward-only-data-preserving-migrations-and-backfill` | `missing` | `phases/13-migration-compatibility-cutover.md` | `ACC-13-MIGRATION-COMPATIBILITY-CUTOVER-0218` | `acceptance/run_original_gap_diff.py::cutover_forward_only_data_preserving_migrations_and_backfill` | Forward-only data-preserving migrations and backfill |
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.md:255:| `cutover-post-cutover-validation-queries-and-supabase-advisors` | `missing` | `phases/13-migration-compatibility-cutover.md` | `ACC-13-MIGRATION-COMPATIBILITY-CUTOVER-0219` | `acceptance/run_original_gap_diff.py::cutover_post_cutover_validation_queries_and_supabase_advisors` | Post-cutover validation queries and Supabase advisors |
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.md:256:| `cutover-rollback-without-destructive-enum-table-drops` | `missing` | `phases/13-migration-compatibility-cutover.md` | `ACC-13-MIGRATION-COMPATIBILITY-CUTOVER-0220` | `acceptance/run_original_gap_diff.py::cutover_rollback_without_destructive_enum_table_drops` | Rollback without destructive enum/table drops |
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.md:267:| `initial_status` | yes | full/partial/missing/divergent from corrected audit. |
docs/plans/2026-07-05-full-paperclip-parity/acceptance/run_original_gap_diff.py:2:"""Parity gap ledger gate (Phase 00 parity oracle).
docs/plans/2026-07-05-full-paperclip-parity/acceptance/run_original_gap_diff.py:4:Fails until every non-full row in evidence/rewrite-gap-matrix.jsonl is closed
docs/plans/2026-07-05-full-paperclip-parity/acceptance/run_original_gap_diff.py:10:    gaps_missing=0 gaps_partial=0 gaps_divergent=0
docs/plans/2026-07-05-full-paperclip-parity/acceptance/run_original_gap_diff.py:20:MATRIX = ROOT / "evidence" / "rewrite-gap-matrix.jsonl"
docs/plans/2026-07-05-full-paperclip-parity/acceptance/run_original_gap_diff.py:22:rows = [json.loads(line) for line in MATRIX.read_text().splitlines() if line.strip()]
docs/plans/2026-07-05-full-paperclip-parity/acceptance/run_original_gap_diff.py:24:open_rows = []
docs/plans/2026-07-05-full-paperclip-parity/acceptance/run_original_gap_diff.py:25:for row in rows:
docs/plans/2026-07-05-full-paperclip-parity/acceptance/run_original_gap_diff.py:34:    open_rows.append(row)
docs/plans/2026-07-05-full-paperclip-parity/acceptance/run_original_gap_diff.py:36:counts = {"missing": 0, "partial": 0, "divergent": 0}
docs/plans/2026-07-05-full-paperclip-parity/acceptance/run_original_gap_diff.py:37:for row in open_rows:
docs/plans/2026-07-05-full-paperclip-parity/acceptance/run_original_gap_diff.py:43:        "gaps_missing=" + str(counts["missing"])
docs/plans/2026-07-05-full-paperclip-parity/acceptance/run_original_gap_diff.py:44:        + " gaps_partial=" + str(counts["partial"])
docs/plans/2026-07-05-full-paperclip-parity/acceptance/run_original_gap_diff.py:45:        + " gaps_divergent=" + str(counts["divergent"])
docs/plans/2026-07-05-full-paperclip-parity/acceptance/run_original_gap_diff.py:46:        + " rows_total=" + str(len(rows))
docs/plans/2026-07-05-full-paperclip-parity/acceptance/run_original_gap_diff.py:47:        + " rows_open=" + str(len(open_rows))
docs/plans/2026-07-05-full-paperclip-parity/acceptance/run_original_gap_diff.py:50:    for row in open_rows[:20]:
docs/plans/2026-07-05-full-paperclip-parity/acceptance/run_original_gap_diff.py:54:print("gaps_missing=0 gaps_partial=0 gaps_divergent=0")
docs/plans/2026-07-05-full-paperclip-parity/05-test-strategy.md:11:- Authz predicate matrix.
docs/plans/2026-07-05-full-paperclip-parity/05-test-strategy.md:71:python3 docs/plans/2026-07-05-full-paperclip-parity/acceptance/run_original_gap_diff.py
docs/plans/2026-07-05-full-paperclip-parity/05-test-strategy.md:75:The final gap diff must print exactly: `gaps_missing=0 gaps_partial=0 gaps_divergent=0`.
docs/plans/2026-07-05-full-paperclip-parity/05-test-strategy.md:80:Category checks are not enough. Every non-full audit row from the authoritative ledger (`3 full / 50 partial / 145 missing / 22 divergent` across 220 rows in `evidence/rewrite-gap-matrix.jsonl`) must carry:
docs/plans/2026-07-05-full-paperclip-parity/05-test-strategy.md:86:A row may move to `full` only after the failing test has been observed red, the implementation makes it green, source evidence is cited, and the reviewer updates `evidence/rewrite-gap-matrix.md`. A row may move to `waived` only after `doc/SPEC-implementation.md` or another canonical product contract is updated to say the original behavior is no longer required.
docs/plans/2026-07-05-full-paperclip-parity/04-execution-order.md:7:0. [`00-parity-oracle-and-matrix.md`](phases/00-parity-oracle-and-matrix.md) — build the failing parity oracle and row-level matrix.
docs/plans/2026-07-05-full-paperclip-parity/04-execution-order.md:24:- Phase 00 blocks all implementation. No worker can close a gap that is not represented in the matrix.
docs/plans/2026-07-05-full-paperclip-parity/04-execution-order.md:26:- Domain backend phases must land before their UI/CLI/MCP parity rows.
docs/plans/2026-07-05-full-paperclip-parity/04-execution-order.md:32:The plan is complete only when `acceptance/run_original_gap_diff.py` reports:
docs/plans/2026-07-05-full-paperclip-parity/04-execution-order.md:35:gaps_missing=0 gaps_partial=0 gaps_divergent=0
docs/plans/2026-07-05-full-paperclip-parity/04-execution-order.md:43:Each phase owns explicit rows in `evidence/rewrite-gap-matrix.md`. A phase cannot close from a category-level pass. Every row must end with:
docs/plans/2026-07-05-full-paperclip-parity/04-execution-order.md:49:The final cutover phase must fail if any blocking row remains `partial`, `missing`, `divergent`, or lacks a reviewed waiver.

hook: PostToolUse
hook: PostToolUse Completed
codex
The visible summary references now point to the 220-row ledger, but I’m not going to rely on prose matches alone. I’m computing the matrix counts from both machine and markdown files, and then executing the Phase 00 gap oracle to see the exact output and exit status.
hook: PreToolUse
hook: PreToolUse Completed
hook: PreToolUse
hook: PreToolUse
hook: PreToolUse Completed
hook: PreToolUse Completed
hook: PreToolUse
hook: PreToolUse Completed
exec
/bin/bash -lc "python3 - <<'PY'
import json
from collections import Counter
p='docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.jsonl'
rows=[json.loads(line) for line in open(p) if line.strip()]
print('jsonl rows_total', len(rows))
print('initial_status', dict(sorted(Counter(r.get('initial_status') for r in rows).items())))
print('final_status', dict(sorted(Counter(r.get('final_status') for r in rows).items())))
print('phases', len(set(r.get('phase') for r in rows)))
for phase,count in sorted(Counter(r.get('phase') for r in rows).items()): print(count, phase)
PY" in /home/frankji/Projects/paperclip
 succeeded in 0ms:
jsonl rows_total 220
initial_status {'divergent': 22, 'full': 3, 'missing': 145, 'partial': 50}
final_status {'full': 3, 'unclosed': 217}
phases 14
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

hook: PostToolUse
hook: PostToolUse Completed
exec
/bin/bash -lc "python3 docs/plans/2026-07-05-full-paperclip-parity/acceptance/run_original_gap_diff.py; printf 'exit=%s\\n' \""'$?"' in /home/frankji/Projects/paperclip
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
exit=1

exec
/bin/bash -lc "sed -n '1,140p' docs/plans/2026-07-05-full-paperclip-parity/README.md && sed -n '1,120p' docs/plans/2026-07-05-full-paperclip-parity/01-parity-audit-summary.md && sed -n '1,120p' docs/plans/2026-07-05-full-paperclip-parity/05-test-strategy.md" in /home/frankji/Projects/paperclip
hook: PostToolUse
 succeeded in 0ms:
# Full Paperclip Parity Implementation Plan

> **Goal:** eliminate the verified parity gap between the current Rust/Supabase/Nuxt rewrite and original Paperclip V1 for agent hiring, governance, runtime, interfaces, and real work execution.
>
> **Definition of success:** the rewrite reaches **zero known parity gaps** against `doc/SPEC-implementation.md` and the original TypeScript implementation evidence cited here. A working vertical slice is not enough.

## Directory map

- [`00-overview.md`](00-overview.md) — scope, success criteria, and non-negotiable invariants.
- [`01-parity-audit-summary.md`](01-parity-audit-summary.md) — corrected audit baseline: 3 full / 50 partial / 145 missing / 22 divergent across 220 rows (authoritative ledger: `evidence/rewrite-gap-matrix.jsonl`).
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

The first audit synthesis reported `3 full / 48 partial / 98 missing / 24 divergent`. Converting the audit journals into a row-level closure ledger exposed two counter mismatches inside focused retry outputs, yielding `3 full / 50 partial / 100 missing / 22 divergent` across 175 rows for the originally-audited hiring/governance/runtime surface. The plan-design review then added owned rows for the remaining original-V1 control-plane areas (goals/projects/issues, deployment/auth/onboarding, workspaces/routines/plugins, import/export/catalogs, observability/pipelines, shared contracts/docs, cutover, and the parity oracle). The authoritative ledger is `evidence/rewrite-gap-matrix.jsonl`: `3 full / 50 partial / 145 missing / 22 divergent` across 220 rows, with every one of the 14 phases owning rows.
# Parity Audit Summary

## Corrected audit result

Two layers of evidence feed this plan:

1. The adversarial parity audit + focused retries produced an initial synthesized tally of `3 full / 48 partial / 98 missing / 24 divergent`. When converted into a row-level closure ledger, the focused retry item lists were more precise than their declared counters, yielding `3 full / 50 partial / 100 missing / 22 divergent` across 175 rows for the originally-audited hiring/governance/runtime surface.
2. The plan-design review then added the remaining original-V1 control-plane areas (goals/projects/issues, deployment/auth/onboarding, workspaces/routines/plugins, import/export/catalogs, observability/pipelines, shared contracts/docs, cutover, and the parity oracle itself) as additional owned rows. With those coverage rows the authoritative ledger is `3 full / 50 partial / 145 missing / 22 divergent` across 220 rows.

| Status | Count |
|---|---:|
| Full | 3 |
| Partial | 50 |
| Missing | 145 |
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

Category checks are not enough. Every non-full audit row from the authoritative ledger (`3 full / 50 partial / 145 missing / 22 divergent` across 220 rows in `evidence/rewrite-gap-matrix.jsonl`) must carry:

```text
phase=<owner> acceptance_id=<id> failing_test=<path::name> passing_test=<command> evidence_ref=<source+rewrite> final_status=<full|waived>
```

A row may move to `full` only after the failing test has been observed red, the implementation makes it green, source evidence is cited, and the reviewer updates `evidence/rewrite-gap-matrix.md`. A row may move to `waived` only after `doc/SPEC-implementation.md` or another canonical product contract is updated to say the original behavior is no longer required.

hook: PostToolUse
hook: PostToolUse Completed
hook: PostToolUse Completed
exec
/bin/bash -lc "python3 - <<'PY'
from pathlib import Path
p=Path('docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.md')
rows=[]
for line in p.read_text().splitlines():
    if line.startswith('| "'`'"') and 'row_id' not in line:
        parts=[part.strip() for part in line.strip().strip('|').split('|')]
        if len(parts)>=2 and parts[1].startswith('"'`'"'):
            rows.append(parts)
from collections import Counter
print('md table rows_total', len(rows))
print('initial_status', dict(sorted(Counter(r[1].strip('"'`'"') for r in rows).items())))
PY" in /home/frankji/Projects/paperclip
 succeeded in 0ms:
md table rows_total 220
initial_status {'divergent': 22, 'full': 3, 'missing': 145, 'partial': 50}

hook: PostToolUse
hook: PostToolUse Completed
codex
BLOCKERS

None. The prior authoritative-tally contradiction is resolved.

Evidence:
- `README.md` states `3 full / 50 partial / 145 missing / 22 divergent` across `220 rows`.
- `01-parity-audit-summary.md` states the same authoritative ledger.
- `05-test-strategy.md` states the same authoritative ledger.
- `rewrite-gap-matrix.jsonl` computed as:
  - total rows: `220`
  - `full: 3`
  - `partial: 50`
  - `missing: 145`
  - `divergent: 22`
- `rewrite-gap-matrix.md` computed as:
  - total rows: `220`
  - same status counts.
- `run_original_gap_diff.py` output:
  - `gaps_missing=145 gaps_partial=50 gaps_divergent=22 rows_total=220 rows_open=217`
  - exit code `1`

That non-zero exit is acceptable for this PLAN artifact because it is the intended Phase 00 RED gate until implementation closes the rows.

PATCHES

None. Read-only review only.

VERDICT

PASS.
hook: Stop
hook: Stop Completed
tokens used
49,464
BLOCKERS

None. The prior authoritative-tally contradiction is resolved.

Evidence:
- `README.md` states `3 full / 50 partial / 145 missing / 22 divergent` across `220 rows`.
- `01-parity-audit-summary.md` states the same authoritative ledger.
- `05-test-strategy.md` states the same authoritative ledger.
- `rewrite-gap-matrix.jsonl` computed as:
  - total rows: `220`
  - `full: 3`
  - `partial: 50`
  - `missing: 145`
  - `divergent: 22`
- `rewrite-gap-matrix.md` computed as:
  - total rows: `220`
  - same status counts.
- `run_original_gap_diff.py` output:
  - `gaps_missing=145 gaps_partial=50 gaps_divergent=22 rows_total=220 rows_open=217`
  - exit code `1`

That non-zero exit is acceptable for this PLAN artifact because it is the intended Phase 00 RED gate until implementation closes the rows.

PATCHES

None. Read-only review only.

VERDICT

PASS.
