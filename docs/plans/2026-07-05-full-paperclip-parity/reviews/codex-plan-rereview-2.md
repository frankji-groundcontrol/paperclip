Reading additional input from stdin...
2026-07-05T17:16:01.859798Z ERROR codex_models_manager::manager: failed to refresh available models: stream disconnected before completion: failed to decode models response: expected value at line 1 column 1; body: <!doctype html>
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
  <script nonce="U2QtOdTEtHM32ATdx20lEg==">window.__APP_CONFIG__={"registration_enabled":false,"email_verify_enabled":false,"registration_email_suffix_whitelist":[],"promo_code_enabled":true,"password_reset_enabled":false,"invitation_code_enabled":false,"totp_enabled":false,"login_agreement_enabled":false,"login_agreement_mode":"modal","login_agreement_updated_at":"2026-03-31","login_agreement_revision":"0eff189d66e0af4e","login_agreement_documents":[{"id":"terms","title":"服务条款","content_md":""},{"id":"usage-policy","title":"使用政策","content_md":""},{"id":"supported-regions","title":"支持的国家和地区","content_md":""},{"id":"service-specific-terms","title":"服务特定条款","content_md":""}],"turnstile_enabled":false,"turnstile_site_key":"","site_name":"Sub2API","site_logo":"","site_subtitle":"Subscription to API Conversion Platform","api_base_url":"","contact_info":"","doc_url":"","home_content":"","hide_ccs_import_button":false,"purchase_subscription_enabled":false,"purchase_subscription_url":"","table_default_page_size":20,"table_page_size_options":[10,20,50],"custom_menu_items":[],"custom_endpoints":[],"linuxdo_oauth_enabled":false,"dingtalk_oauth_enabled":false,"wechat_oauth_enabled":false,"wechat_oauth_open_enabled":false,"wechat_oauth_mp_enabled":false,"wechat_oauth_mobile_enabled":false,"oidc_oauth_enabled":false,"oidc_oauth_provider_name":"OIDC","github_oauth_enabled":false,"google_oauth_enabled":false,"backend_mode_enabled":false,"payment_enabled":false,"version":"","server_timezone":"Asia/Shanghai","server_utc_offset":"+08:00","balance_low_notify_enabled":false,"account_quota_notify_enabled":false,"balance_low_notify_threshold":0,"balance_low_notify_recharge_url":"","channel_monitor_enabled":true,"channel_monitor_default_interval_seconds":60,"available_channels_enabled":false,"affiliate_enabled":false,"risk_control_enabled":false,"allow_user_view_error_requests":false};</script></head>
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
session id: 019f3347-a7c4-76f1-acc5-b6a483af022f
--------
user
Re-review only these plan files after patches: docs/plans/2026-07-05-full-paperclip-parity/README.md, 01-parity-audit-summary.md, 04-execution-order.md, 05-test-strategy.md, acceptance/full-parity-harness-spec.md, acceptance/run_original_gap_diff.py, acceptance/run_real_user_real_agent.py, codex/phase-prompts.md, reviews/review-checklist.md, evidence/rewrite-gap-matrix.md, evidence/rewrite-gap-matrix.jsonl, and phase filenames under phases/. Do not edit. Goal: plan must drive rewrite to zero known Paperclip V1 parity gaps. Check prior blockers: phase count, exact gaps_* output, row-level matrix closure, executable harness stubs, Codex implementer context, anti-false-parity gate. Return BLOCKERS, PATCHES, VERDICT. VERDICT must be PASS or FAIL.
2026-07-05T17:16:02.661225Z ERROR rmcp::transport::worker: worker quit with fatal: Transport channel closed, when Auth(AuthorizationRequired)
warning: Skill descriptions were shortened to fit the 2% skills context budget. Codex can still see every skill, but some descriptions are shorter. Disable unused skills or plugins to leave more room for the rest.
hook: SessionStart
hook: SessionStart Completed
hook: UserPromptSubmit
hook: UserPromptSubmit Completed
codex
I’ll re-review only the named plan files and the phase filenames, with no edits. I’ll first load the repository guidance docs required for work in this checkout, then inspect the patched plan artifacts against the specific prior blockers.
hook: PreToolUse
hook: PreToolUse Completed
exec
/bin/bash -lc "sed -n '1,220p' /home/frankji/.codex/superpowers/skills/using-superpowers/SKILL.md" in /home/frankji/Projects/paperclip
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

hook: PostToolUse
hook: PostToolUse Completed
hook: PreToolUse
hook: PreToolUse Completed
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
hook: PostToolUse Completed
hook: PreToolUse
hook: PreToolUse Completed
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
hook: PreToolUse
hook: PreToolUse Completed
exec
/bin/bash -lc "sed -n '1,260p' doc/SPEC-implementation.md" in /home/frankji/Projects/paperclip
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
- `project_id` uuid fk `projects.id` null
- `project_workspace_id` uuid fk `project_workspaces.id` null
- `goal_id` uuid fk `goals.id` null
- `parent_id` uuid fk `issues.id` null
- `title` text not null
- `description` text null
- `status` enum: `backlog | todo | in_progress | in_review | done | blocked | cancelled`
- `priority` enum: `critical | high | medium | low`
- `assignee_agent_id` uuid fk `agents.id` null
- `assignee_user_id` text null
- checkout/execution locks: `checkout_run_id`, `execution_run_id`, `execution_agent_name_key`, `execution_locked_at`
- `created_by_agent_id` uuid fk `agents.id` null
- `created_by_user_id` uuid fk `users.id` null
- identifier fields: `issue_number`, `identifier`
- origin fields: `origin_kind`, `origin_id`, `origin_run_id`, `origin_fingerprint`
- `request_depth` int not null default 0
- `work_mode` text not null default `standard`; supported values:
  - `standard`: normal autonomous execution. Agents may investigate, edit files, create artifacts, and complete the task.
  - `ask`: answer-only execution. Agents may use tools for investigation or temporary scratch work, but the deliverable is an issue-thread answer; they must not write implementation code or produce an implementation plan.
  - `planning`: plan-only execution. Agents create or revise the plan without implementation work; accepted-plan continuations remain planning-specific and create child issues from the approved plan.
- `billing_code` text null
- `assignee_adapter_overrides` jsonb null
- `execution_policy` jsonb null
- `execution_state` jsonb null
- execution workspace fields: `execution_workspace_id`, `execution_workspace_preference`, `execution_workspace_settings`
- `started_at` timestamptz null
- `completed_at` timestamptz null
- `cancelled_at` timestamptz null
- `hidden_at` timestamptz null

Invariants:

- single assignee only
- task must trace to company goal chain via `goal_id`, `parent_id`, or project-goal linkage
- `in_progress` requires assignee
- terminal states: `done | cancelled`

## 7.7 `issue_comments`

- `id` uuid pk

hook: PostToolUse
hook: PostToolUse Completed
hook: PreToolUse
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

hook: PostToolUse
hook: PostToolUse Completed
hook: PreToolUse
hook: PreToolUse Completed
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
codex
Using `superpowers:using-superpowers` for the required startup workflow. I’ve read the required repository docs enough to orient on the V1 contract; next I’m checking only the requested plan files and filenames under `phases/`.
hook: PreToolUse
hook: PreToolUse Completed
hook: PreToolUse
hook: PreToolUse Completed
hook: PreToolUse
hook: PreToolUse Completed
exec
/bin/bash -lc 'find docs/plans/2026-07-05-full-paperclip-parity -maxdepth 3 -type f | sort' in /home/frankji/Projects/paperclip
 succeeded in 0ms:
docs/plans/2026-07-05-full-paperclip-parity/00-overview.md
docs/plans/2026-07-05-full-paperclip-parity/01-parity-audit-summary.md
docs/plans/2026-07-05-full-paperclip-parity/02-target-architecture.md
docs/plans/2026-07-05-full-paperclip-parity/03-file-structure.md
docs/plans/2026-07-05-full-paperclip-parity/04-execution-order.md
docs/plans/2026-07-05-full-paperclip-parity/05-test-strategy.md
docs/plans/2026-07-05-full-paperclip-parity/06-risk-register.md
docs/plans/2026-07-05-full-paperclip-parity/README.md
docs/plans/2026-07-05-full-paperclip-parity/acceptance/README.md
docs/plans/2026-07-05-full-paperclip-parity/acceptance/deterministic-local-scenarios.md
docs/plans/2026-07-05-full-paperclip-parity/acceptance/full-parity-harness-spec.md
docs/plans/2026-07-05-full-paperclip-parity/acceptance/no-secret-scanner-spec.md
docs/plans/2026-07-05-full-paperclip-parity/acceptance/real-user-real-agent-scenarios.md
docs/plans/2026-07-05-full-paperclip-parity/acceptance/release-gate-commands.md
docs/plans/2026-07-05-full-paperclip-parity/acceptance/run_cli_mcp_surface.py
docs/plans/2026-07-05-full-paperclip-parity/acceptance/run_frontend_user_flow.py
docs/plans/2026-07-05-full-paperclip-parity/acceptance/run_original_gap_diff.py
docs/plans/2026-07-05-full-paperclip-parity/acceptance/run_real_user_real_agent.py
docs/plans/2026-07-05-full-paperclip-parity/codex/README.md
docs/plans/2026-07-05-full-paperclip-parity/codex/phase-prompts.md
docs/plans/2026-07-05-full-paperclip-parity/evidence/migration-ledger.md
docs/plans/2026-07-05-full-paperclip-parity/evidence/original-source-map.md
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.jsonl
docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.md
docs/plans/2026-07-05-full-paperclip-parity/phases/00-parity-oracle-and-matrix.md
docs/plans/2026-07-05-full-paperclip-parity/phases/01-schema-authz-parity.md
docs/plans/2026-07-05-full-paperclip-parity/phases/02-goals-projects-issues-work-products.md
docs/plans/2026-07-05-full-paperclip-parity/phases/03-workspaces-routines-plugins-sandbox.md
docs/plans/2026-07-05-full-paperclip-parity/phases/04-deployment-auth-onboarding-ops.md
docs/plans/2026-07-05-full-paperclip-parity/phases/05-adapters-secrets-skills-config.md
docs/plans/2026-07-05-full-paperclip-parity/phases/06-approvals-budgets-costs-activity.md
docs/plans/2026-07-05-full-paperclip-parity/phases/07-agent-runtime-lifecycle.md
docs/plans/2026-07-05-full-paperclip-parity/phases/08-interfaces-rest-cli-mcp.md
docs/plans/2026-07-05-full-paperclip-parity/phases/09-nuxt-board-ui-parity.md
docs/plans/2026-07-05-full-paperclip-parity/phases/10-import-export-catalogs-portability.md
docs/plans/2026-07-05-full-paperclip-parity/phases/11-observability-feedback-pipelines.md
docs/plans/2026-07-05-full-paperclip-parity/phases/12-shared-contract-docs-parity.md
docs/plans/2026-07-05-full-paperclip-parity/phases/13-migration-compatibility-cutover.md
docs/plans/2026-07-05-full-paperclip-parity/reviews/README.md
docs/plans/2026-07-05-full-paperclip-parity/reviews/codex-plan-rereview-2.md
docs/plans/2026-07-05-full-paperclip-parity/reviews/codex-plan-rereview.md
docs/plans/2026-07-05-full-paperclip-parity/reviews/codex-plan-review.md
docs/plans/2026-07-05-full-paperclip-parity/reviews/review-checklist.md

hook: PostToolUse
hook: PostToolUse Completed
hook: PreToolUse
hook: PreToolUse
hook: PreToolUse Completed
hook: PreToolUse
hook: PreToolUse Completed
hook: PreToolUse Completed
exec
/bin/bash -lc "find docs/plans/2026-07-05-full-paperclip-parity/phases -maxdepth 1 -type f -printf '%f
' | sort" in /home/frankji/Projects/paperclip
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
exec
/bin/bash -lc "sed -n '1,260p' docs/plans/2026-07-05-full-paperclip-parity/README.md" in /home/frankji/Projects/paperclip
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
/bin/bash -lc "sed -n '1,260p' docs/plans/2026-07-05-full-paperclip-parity/05-test-strategy.md" in /home/frankji/Projects/paperclip
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
exec
/bin/bash -lc "sed -n '1,260p' docs/plans/2026-07-05-full-paperclip-parity/04-execution-order.md" in /home/frankji/Projects/paperclip
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

hook: PostToolUse
hook: PostToolUse Completed
hook: PostToolUse Completed
hook: PreToolUse
hook: PreToolUse Completed
exec
/bin/bash -lc "sed -n '1,260p' docs/plans/2026-07-05-full-paperclip-parity/acceptance/full-parity-harness-spec.md" in /home/frankji/Projects/paperclip
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

hook: PostToolUse
hook: PostToolUse Completed
hook: PreToolUse
hook: PreToolUse Completed
hook: PreToolUse
hook: PreToolUse Completed
hook: PreToolUse
hook: PreToolUse Completed
hook: PreToolUse
exec
/bin/bash -lc "sed -n '1,260p' docs/plans/2026-07-05-full-paperclip-parity/acceptance/run_original_gap_diff.py" in /home/frankji/Projects/paperclip
 succeeded in 0ms:
#!/usr/bin/env python3
"""Parity gap ledger gate.

Fails until every non-full row in evidence/rewrite-gap-matrix.jsonl is closed as
final_status=full or final_status=waived with a waiver_ref.
"""
from __future__ import annotations
import json
from pathlib import Path
import sys

ROOT = Path(__file__).resolve().parents[1]
MATRIX = ROOT / "evidence" / "rewrite-gap-matrix.jsonl"
rows = [json.loads(line) for line in MATRIX.read_text().splitlines() if line.strip()]
open_rows = []
for row in rows:
    status = row.get("initial_status")
    final = row.get("final_status")
    if status == "full" and final == "full":
        continue
    if final == "full":
        continue
    if final == "waived" and row.get("waiver_ref"):
        continue
    open_rows.append(row)
counts = {"missing": 0, "partial": 0, "divergent": 0}
for row in open_rows:
    status = row.get("initial_status")
    if status in counts:
        counts[status] += 1
print(
    f"gaps_missing={counts['missing']} "
    f"gaps_partial={counts['partial']} "
    f"gaps_divergent={counts['divergent']} "
    f"rows_total={len(rows)} rows_open={len(open_rows)}"
)
if any(counts.values()):
    print("OPEN_ROWS_SAMPLE:")
    for row in open_rows[:20]:
        print(f"- {row['row_id']} [{row['initial_status']}] {row['phase']} :: {row['capability']}")
    sys.exit(1)
sys.exit(0)

hook: PostToolUse
hook: PreToolUse Completed
hook: PostToolUse Completed
exec
/bin/bash -lc "sed -n '1,320p' docs/plans/2026-07-05-full-paperclip-parity/acceptance/run_real_user_real_agent.py" in /home/frankji/Projects/paperclip
 succeeded in 0ms:
#!/usr/bin/env python3
"""Live real-user/real-agent acceptance harness placeholder.

Phase 00 owns turning this scaffold into the live Supabase/OpenAI harness. It
intentionally fails until implemented so the plan cannot claim live parity from
missing automation.
"""
import sys
print("NOT_IMPLEMENTED: live real-user/real-agent harness must be implemented in Phase 00/09 before release.")
sys.exit(2)

hook: PostToolUse
hook: PostToolUse Completed
exec
/bin/bash -lc "sed -n '1,320p' docs/plans/2026-07-05-full-paperclip-parity/codex/phase-prompts.md" in /home/frankji/Projects/paperclip
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
/bin/bash -lc "sed -n '1,260p' docs/plans/2026-07-05-full-paperclip-parity/reviews/review-checklist.md" in /home/frankji/Projects/paperclip
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

hook: PostToolUse
hook: PostToolUse Completed
hook: PostToolUse Completed
codex
The patched plan now has 14 phase files and the top-level docs consistently point at the corrected `3 full / 50 partial / 100 missing / 22 divergent` baseline. I’m drilling into the matrix and executable gates now, because those determine whether the plan can actually prevent false parity.
hook: PreToolUse
hook: PreToolUse Completed
hook: PreToolUse
hook: PreToolUse Completed
exec
/bin/bash -lc "sed -n '1,220p' docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.md" in /home/frankji/Projects/paperclip
 succeeded in 0ms:
# Rewrite Gap Matrix

This is the row-level closure ledger generated from the adversarial parity audit journals. It is intentionally long: workers must close rows, not categories.

## Corrected audit tally

- full: 3
- partial: 50
- missing: 100
- divergent: 22

Machine-readable companion: [`rewrite-gap-matrix.jsonl`](rewrite-gap-matrix.jsonl).

## Phase ownership summary

| Phase | Full | Partial | Missing | Divergent | Total |
|---|---:|---:|---:|---:|---:|
| `phases/01-schema-authz-parity.md` | 1 | 20 | 24 | 10 | 55 |
| `phases/05-adapters-secrets-skills-config.md` | 0 | 2 | 9 | 1 | 12 |
| `phases/06-approvals-budgets-costs-activity.md` | 1 | 8 | 10 | 2 | 21 |
| `phases/07-agent-runtime-lifecycle.md` | 0 | 3 | 8 | 5 | 16 |
| `phases/08-interfaces-rest-cli-mcp.md` | 0 | 12 | 16 | 3 | 31 |
| `phases/09-nuxt-board-ui-parity.md` | 1 | 5 | 33 | 1 | 40 |

## Row-level matrix

| Row | Status | Phase | Acceptance | Test | Capability |
|---|---|---|---|---|---|
| `agent-lifecycle-status-transitions-agent-key-permission-to-request-hir` | `partial` | `phases/07-agent-runtime-lifecycle.md` | `ACC-07-AGENT-RUNTIME-LIFECYCLE-0001` | `backend-rs/tests/agent_lifecycle_parity.rs::agent_lifecycle_status_transitions_agent_key_permission_to_request_hir` | Agent-key permission to request hires |
| `agent-lifecycle-status-transitions-approval-gated-hiring-flow` | `partial` | `phases/07-agent-runtime-lifecycle.md` | `ACC-07-AGENT-RUNTIME-LIFECYCLE-0002` | `backend-rs/tests/agent_lifecycle_parity.rs::agent_lifecycle_status_transitions_approval_gated_hiring_flow` | Approval-gated hiring flow |
| `agent-lifecycle-status-transitions-approving-pending-hires` | `divergent` | `phases/07-agent-runtime-lifecycle.md` | `ACC-07-AGENT-RUNTIME-LIFECYCLE-0003` | `backend-rs/tests/agent_lifecycle_parity.rs::agent_lifecycle_status_transitions_approving_pending_hires` | Approving pending hires |
| `agent-lifecycle-status-transitions-canonical-agent-status-vocabulary` | `divergent` | `phases/07-agent-runtime-lifecycle.md` | `ACC-07-AGENT-RUNTIME-LIFECYCLE-0004` | `backend-rs/tests/agent_lifecycle_parity.rs::agent_lifecycle_status_transitions_canonical_agent_status_vocabulary` | Canonical agent status vocabulary |
| `agent-lifecycle-status-transitions-company-auth-boundary-for-lifecycle` | `partial` | `phases/07-agent-runtime-lifecycle.md` | `ACC-07-AGENT-RUNTIME-LIFECYCLE-0005` | `backend-rs/tests/agent_lifecycle_parity.rs::agent_lifecycle_status_transitions_company_auth_boundary_for_lifecycle` | Company/auth boundary for lifecycle mutations |
| `agent-lifecycle-status-transitions-error-state-and-clear-error` | `missing` | `phases/07-agent-runtime-lifecycle.md` | `ACC-07-AGENT-RUNTIME-LIFECYCLE-0006` | `backend-rs/tests/agent_lifecycle_parity.rs::agent_lifecycle_status_transitions_error_state_and_clear_error` | Error state and clear-error |
| `agent-lifecycle-status-transitions-frontend-cli-mcp-lifecycle-controls` | `divergent` | `phases/07-agent-runtime-lifecycle.md` | `ACC-07-AGENT-RUNTIME-LIFECYCLE-0007` | `backend-rs/tests/agent_lifecycle_parity.rs::agent_lifecycle_status_transitions_frontend_cli_mcp_lifecycle_controls` | Frontend/CLI/MCP lifecycle controls |
| `agent-lifecycle-status-transitions-heartbeat-run-wakeup-request-log-ev` | `missing` | `phases/07-agent-runtime-lifecycle.md` | `ACC-07-AGENT-RUNTIME-LIFECYCLE-0008` | `backend-rs/tests/agent_lifecycle_parity.rs::agent_lifecycle_status_transitions_heartbeat_run_wakeup_request_log_ev` | Heartbeat run, wakeup request, log/event, live-run, and cancellation model |
| `agent-lifecycle-status-transitions-new-agent-creation-defaults-and-dir` | `divergent` | `phases/07-agent-runtime-lifecycle.md` | `ACC-07-AGENT-RUNTIME-LIFECYCLE-0009` | `backend-rs/tests/agent_lifecycle_parity.rs::agent_lifecycle_status_transitions_new_agent_creation_defaults_and_dir` | New agent creation defaults and direct hire semantics |
| `agent-lifecycle-status-transitions-pause-agent` | `missing` | `phases/07-agent-runtime-lifecycle.md` | `ACC-07-AGENT-RUNTIME-LIFECYCLE-0010` | `backend-rs/tests/agent_lifecycle_parity.rs::agent_lifecycle_status_transitions_pause_agent` | Pause agent |
| `agent-lifecycle-status-transitions-rejecting-pending-hires-firing-pend` | `divergent` | `phases/07-agent-runtime-lifecycle.md` | `ACC-07-AGENT-RUNTIME-LIFECYCLE-0011` | `backend-rs/tests/agent_lifecycle_parity.rs::agent_lifecycle_status_transitions_rejecting_pending_hires_firing_pend` | Rejecting pending hires / firing pending hires |
| `agent-lifecycle-status-transitions-resume-agent` | `missing` | `phases/07-agent-runtime-lifecycle.md` | `ACC-07-AGENT-RUNTIME-LIFECYCLE-0012` | `backend-rs/tests/agent_lifecycle_parity.rs::agent_lifecycle_status_transitions_resume_agent` | Resume agent |
| `agent-lifecycle-status-transitions-runtime-state-task-sessions-and-ses` | `missing` | `phases/07-agent-runtime-lifecycle.md` | `ACC-07-AGENT-RUNTIME-LIFECYCLE-0013` | `backend-rs/tests/agent_lifecycle_parity.rs::agent_lifecycle_status_transitions_runtime_state_task_sessions_and_ses` | Runtime state, task sessions, and session reset |
| `agent-lifecycle-status-transitions-runtime-transition-idle-running-idl` | `missing` | `phases/07-agent-runtime-lifecycle.md` | `ACC-07-AGENT-RUNTIME-LIFECYCLE-0014` | `backend-rs/tests/agent_lifecycle_parity.rs::agent_lifecycle_status_transitions_runtime_transition_idle_running_idl` | Runtime transition idle → running → idle/error |
| `agent-lifecycle-status-transitions-terminate-fire-active-agents` | `missing` | `phases/07-agent-runtime-lifecycle.md` | `ACC-07-AGENT-RUNTIME-LIFECYCLE-0015` | `backend-rs/tests/agent_lifecycle_parity.rs::agent_lifecycle_status_transitions_terminate_fire_active_agents` | Terminate/fire active agents |
| `agent-lifecycle-status-transitions-wakeup-heartbeat-invoke` | `missing` | `phases/07-agent-runtime-lifecycle.md` | `ACC-07-AGENT-RUNTIME-LIFECYCLE-0016` | `backend-rs/tests/agent_lifecycle_parity.rs::agent_lifecycle_status_transitions_wakeup_heartbeat_invoke` | Wakeup / heartbeat invoke |
| `approvals-system-governance-access-boundary-read` | `partial` | `phases/06-approvals-budgets-costs-activity.md` | `ACC-06-APPROVALS-BUDGETS-COSTS-ACTIVITY-0017` | `backend-rs/tests/approvals_parity.rs::approvals_system_governance_access_boundary_read` | Company-scope read authorization boundary |
| `approvals-system-governance-approval-comments-table` | `missing` | `phases/06-approvals-budgets-costs-activity.md` | `ACC-06-APPROVALS-BUDGETS-COSTS-ACTIVITY-0018` | `backend-rs/tests/approvals_parity.rs::approvals_system_governance_approval_comments_table` | approval_comments table + comment endpoints |
| `approvals-system-governance-approval-status-enum` | `divergent` | `phases/06-approvals-budgets-costs-activity.md` | `ACC-06-APPROVALS-BUDGETS-COSTS-ACTIVITY-0019` | `backend-rs/tests/approvals_parity.rs::approvals_system_governance_approval_status_enum` | Approval status lifecycle enum (5 statuses) |
| `approvals-system-governance-approval-types-enum` | `partial` | `phases/06-approvals-budgets-costs-activity.md` | `ACC-06-APPROVALS-BUDGETS-COSTS-ACTIVITY-0020` | `backend-rs/tests/approvals_parity.rs::approvals_system_governance_approval_types_enum` | Approval type enum (4 governed types) |
| `approvals-system-governance-approvals-table-schema` | `partial` | `phases/06-approvals-budgets-costs-activity.md` | `ACC-06-APPROVALS-BUDGETS-COSTS-ACTIVITY-0021` | `backend-rs/tests/approvals_parity.rs::approvals_system_governance_approvals_table_schema` | approvals table columns |
| `approvals-system-governance-approve-effect-hire-agent` | `partial` | `phases/06-approvals-budgets-costs-activity.md` | `ACC-06-APPROVALS-BUDGETS-COSTS-ACTIVITY-0022` | `backend-rs/tests/approvals_parity.rs::approvals_system_governance_approve_effect_hire_agent` | approve effect: hire_agent activates/creates agent + budget + notify |
| `approvals-system-governance-approve-effect-requester-wakeup` | `missing` | `phases/06-approvals-budgets-costs-activity.md` | `ACC-06-APPROVALS-BUDGETS-COSTS-ACTIVITY-0023` | `backend-rs/tests/approvals_parity.rs::approvals_system_governance_approve_effect_requester_wakeup` | approve effect: wake requesting agent + activity logs |
| `approvals-system-governance-approve-lifecycle` | `divergent` | `phases/06-approvals-budgets-costs-activity.md` | `ACC-06-APPROVALS-BUDGETS-COSTS-ACTIVITY-0024` | `backend-rs/tests/approvals_parity.rs::approvals_system_governance_approve_lifecycle` | Approve transition + idempotency |
| `approvals-system-governance-board-chat-issue-persistence` | `missing` | `phases/06-approvals-budgets-costs-activity.md` | `ACC-06-APPROVALS-BUDGETS-COSTS-ACTIVITY-0025` | `backend-rs/tests/approvals_parity.rs::approvals_system_governance_board_chat_issue_persistence` | Board chat persisted to standing 'Board Operations' issue |
| `approvals-system-governance-board-chat-relay` | `missing` | `phases/06-approvals-budgets-costs-activity.md` | `ACC-06-APPROVALS-BUDGETS-COSTS-ACTIVITY-0026` | `backend-rs/tests/approvals_parity.rs::approvals_system_governance_board_chat_relay` | Board Concierge Chat SSE relay |
| `approvals-system-governance-create-approval` | `partial` | `phases/06-approvals-budgets-costs-activity.md` | `ACC-06-APPROVALS-BUDGETS-COSTS-ACTIVITY-0027` | `backend-rs/tests/approvals_parity.rs::approvals_system_governance_create_approval` | Create approval (pending) |
| `approvals-system-governance-decide-authz-board-only` | `partial` | `phases/06-approvals-budgets-costs-activity.md` | `ACC-06-APPROVALS-BUDGETS-COSTS-ACTIVITY-0028` | `backend-rs/tests/approvals_parity.rs::approvals_system_governance_decide_authz_board_only` | Only board actors may approve/reject/request-revision |
| `approvals-system-governance-find-open-hire-approval` | `missing` | `phases/06-approvals-budgets-costs-activity.md` | `ACC-06-APPROVALS-BUDGETS-COSTS-ACTIVITY-0029` | `backend-rs/tests/approvals_parity.rs::approvals_system_governance_find_open_hire_approval` | findOpenHireApprovalForAgent helper |
| `approvals-system-governance-get-approval` | `missing` | `phases/06-approvals-budgets-costs-activity.md` | `ACC-06-APPROVALS-BUDGETS-COSTS-ACTIVITY-0030` | `backend-rs/tests/approvals_parity.rs::approvals_system_governance_get_approval` | Get single approval / linked issues |
| `approvals-system-governance-list-approvals` | `partial` | `phases/06-approvals-budgets-costs-activity.md` | `ACC-06-APPROVALS-BUDGETS-COSTS-ACTIVITY-0031` | `backend-rs/tests/approvals_parity.rs::approvals_system_governance_list_approvals` | List approvals for a company (status filter) |
| `approvals-system-governance-payload-redaction` | `missing` | `phases/06-approvals-budgets-costs-activity.md` | `ACC-06-APPROVALS-BUDGETS-COSTS-ACTIVITY-0032` | `backend-rs/tests/approvals_parity.rs::approvals_system_governance_payload_redaction` | Approval payload + comment redaction on read |
| `approvals-system-governance-reject-lifecycle-effect` | `partial` | `phases/06-approvals-budgets-costs-activity.md` | `ACC-06-APPROVALS-BUDGETS-COSTS-ACTIVITY-0033` | `backend-rs/tests/approvals_parity.rs::approvals_system_governance_reject_lifecycle_effect` | Reject transition + hire_agent termination |
| `approvals-system-governance-request-revision` | `missing` | `phases/06-approvals-budgets-costs-activity.md` | `ACC-06-APPROVALS-BUDGETS-COSTS-ACTIVITY-0034` | `backend-rs/tests/approvals_parity.rs::approvals_system_governance_request_revision` | Request revision transition (board only, pending→revision_requested) |
| `approvals-system-governance-require-approval-access` | `full` | `phases/06-approvals-budgets-costs-activity.md` | `ACC-06-APPROVALS-BUDGETS-COSTS-ACTIVITY-0035` | `backend-rs/tests/approvals_parity.rs::approvals_system_governance_require_approval_access` | requireApprovalAccess company scoping on decide |
| `approvals-system-governance-resubmit` | `missing` | `phases/06-approvals-budgets-costs-activity.md` | `ACC-06-APPROVALS-BUDGETS-COSTS-ACTIVITY-0036` | `backend-rs/tests/approvals_parity.rs::approvals_system_governance_resubmit` | Resubmit (revision_requested→pending), requester-only |
| `approvals-system-governance-run-context-mutation-guard` | `missing` | `phases/06-approvals-budgets-costs-activity.md` | `ACC-06-APPROVALS-BUDGETS-COSTS-ACTIVITY-0037` | `backend-rs/tests/approvals_parity.rs::approvals_system_governance_run_context_mutation_guard` | Cheap status-only recovery run cannot mutate approvals |
| `cli-api-surface-agent-users-api-agent-approve` | `partial` | `phases/08-interfaces-rest-cli-mcp.md` | `ACC-08-INTERFACES-REST-CLI-MCP-0038` | `backend-rs/tests/api_surface_parity.rs::cli_api_surface_agent_users_api_agent_approve` | POST /api/agents/:id/approve |
| `cli-api-surface-agent-users-api-agent-hires` | `partial` | `phases/08-interfaces-rest-cli-mcp.md` | `ACC-08-INTERFACES-REST-CLI-MCP-0039` | `backend-rs/tests/api_surface_parity.rs::cli_api_surface_agent_users_api_agent_hires` | POST /api/companies/:companyId/agent-hires |
| `cli-api-surface-agent-users-api-agent-keys` | `missing` | `phases/08-interfaces-rest-cli-mcp.md` | `ACC-08-INTERFACES-REST-CLI-MCP-0040` | `backend-rs/tests/api_surface_parity.rs::cli_api_surface_agent_users_api_agent_keys` | Agent API key endpoints |
| `cli-api-surface-agent-users-api-agent-lifecycle-simple` | `missing` | `phases/08-interfaces-rest-cli-mcp.md` | `ACC-08-INTERFACES-REST-CLI-MCP-0041` | `backend-rs/tests/api_surface_parity.rs::cli_api_surface_agent_users_api_agent_lifecycle_simple` | pause/resume/clear-error/delete |
| `cli-api-surface-agent-users-api-agent-me` | `missing` | `phases/08-interfaces-rest-cli-mcp.md` | `ACC-08-INTERFACES-REST-CLI-MCP-0042` | `backend-rs/tests/api_surface_parity.rs::cli_api_surface_agent_users_api_agent_me` | GET /api/agents/me + inbox |
| `cli-api-surface-agent-users-api-agent-permissions` | `missing` | `phases/08-interfaces-rest-cli-mcp.md` | `ACC-08-INTERFACES-REST-CLI-MCP-0043` | `backend-rs/tests/api_surface_parity.rs::cli_api_surface_agent_users_api_agent_permissions` | PATCH /api/agents/:id/permissions |
| `cli-api-surface-agent-users-api-agent-terminate` | `partial` | `phases/08-interfaces-rest-cli-mcp.md` | `ACC-08-INTERFACES-REST-CLI-MCP-0044` | `backend-rs/tests/api_surface_parity.rs::cli_api_surface_agent_users_api_agent_terminate` | POST /api/agents/:id/terminate |
| `cli-api-surface-agent-users-api-agent-wakeup` | `missing` | `phases/08-interfaces-rest-cli-mcp.md` | `ACC-08-INTERFACES-REST-CLI-MCP-0045` | `backend-rs/tests/api_surface_parity.rs::cli_api_surface_agent_users_api_agent_wakeup` | POST /api/agents/:id/wakeup + /heartbeat/invoke |
| `cli-api-surface-agent-users-api-agents-create` | `divergent` | `phases/08-interfaces-rest-cli-mcp.md` | `ACC-08-INTERFACES-REST-CLI-MCP-0046` | `backend-rs/tests/api_surface_parity.rs::cli_api_surface_agent_users_api_agents_create` | POST /api/companies/:companyId/agents |
| `cli-api-surface-agent-users-api-approvals-create` | `missing` | `phases/08-interfaces-rest-cli-mcp.md` | `ACC-08-INTERFACES-REST-CLI-MCP-0047` | `backend-rs/tests/api_surface_parity.rs::cli_api_surface_agent_users_api_approvals_create` | POST /api/companies/:companyId/approvals |
| `cli-api-surface-agent-users-api-approvals-decisions` | `partial` | `phases/08-interfaces-rest-cli-mcp.md` | `ACC-08-INTERFACES-REST-CLI-MCP-0048` | `backend-rs/tests/api_surface_parity.rs::cli_api_surface_agent_users_api_approvals_decisions` | approve/reject/request-revision |
| `cli-api-surface-agent-users-api-approvals-list-get` | `partial` | `phases/08-interfaces-rest-cli-mcp.md` | `ACC-08-INTERFACES-REST-CLI-MCP-0049` | `backend-rs/tests/api_surface_parity.rs::cli_api_surface_agent_users_api_approvals_list_get` | GET approvals list/get |
| `cli-api-surface-agent-users-api-approvals-resubmit-comment` | `missing` | `phases/08-interfaces-rest-cli-mcp.md` | `ACC-08-INTERFACES-REST-CLI-MCP-0050` | `backend-rs/tests/api_surface_parity.rs::cli_api_surface_agent_users_api_approvals_resubmit_comment` | resubmit + comments |
| `cli-api-surface-agent-users-authz-create-gate` | `partial` | `phases/08-interfaces-rest-cli-mcp.md` | `ACC-08-INTERFACES-REST-CLI-MCP-0051` | `backend-rs/tests/api_surface_parity.rs::cli_api_surface_agent_users_authz_create_gate` | agents:create authorization |
| `cli-api-surface-agent-users-authz-read-config` | `missing` | `phases/08-interfaces-rest-cli-mcp.md` | `ACC-08-INTERFACES-REST-CLI-MCP-0052` | `backend-rs/tests/api_surface_parity.rs::cli_api_surface_agent_users_authz_read_config` | config-read authorization |
| `cli-api-surface-agent-users-cli-agent-create` | `missing` | `phases/08-interfaces-rest-cli-mcp.md` | `ACC-08-INTERFACES-REST-CLI-MCP-0053` | `backend-rs/tests/api_surface_parity.rs::cli_api_surface_agent_users_cli_agent_create` | CLI `agent create` |
| `cli-api-surface-agent-users-cli-agent-crud` | `partial` | `phases/08-interfaces-rest-cli-mcp.md` | `ACC-08-INTERFACES-REST-CLI-MCP-0054` | `backend-rs/tests/api_surface_parity.rs::cli_api_surface_agent_users_cli_agent_crud` | CLI agent read/update/delete |
| `cli-api-surface-agent-users-cli-agent-hire` | `partial` | `phases/08-interfaces-rest-cli-mcp.md` | `ACC-08-INTERFACES-REST-CLI-MCP-0055` | `backend-rs/tests/api_surface_parity.rs::cli_api_surface_agent_users_cli_agent_hire` | CLI `agent hire` |
| `cli-api-surface-agent-users-cli-agent-inbox` | `missing` | `phases/08-interfaces-rest-cli-mcp.md` | `ACC-08-INTERFACES-REST-CLI-MCP-0056` | `backend-rs/tests/api_surface_parity.rs::cli_api_surface_agent_users_cli_agent_inbox` | CLI agent inbox commands |
| `cli-api-surface-agent-users-cli-agent-instructions` | `missing` | `phases/08-interfaces-rest-cli-mcp.md` | `ACC-08-INTERFACES-REST-CLI-MCP-0057` | `backend-rs/tests/api_surface_parity.rs::cli_api_surface_agent_users_cli_agent_instructions` | CLI instructions path/bundle/file |
| `cli-api-surface-agent-users-cli-agent-lifecycle` | `missing` | `phases/08-interfaces-rest-cli-mcp.md` | `ACC-08-INTERFACES-REST-CLI-MCP-0058` | `backend-rs/tests/api_surface_parity.rs::cli_api_surface_agent_users_cli_agent_lifecycle` | CLI agent lifecycle verbs |
| `cli-api-surface-agent-users-cli-agent-local-cli` | `missing` | `phases/08-interfaces-rest-cli-mcp.md` | `ACC-08-INTERFACES-REST-CLI-MCP-0059` | `backend-rs/tests/api_surface_parity.rs::cli_api_surface_agent_users_cli_agent_local_cli` | CLI `agent local-cli` |
| `cli-api-surface-agent-users-cli-agent-permissions-config` | `missing` | `phases/08-interfaces-rest-cli-mcp.md` | `ACC-08-INTERFACES-REST-CLI-MCP-0060` | `backend-rs/tests/api_surface_parity.rs::cli_api_surface_agent_users_cli_agent_permissions_config` | CLI permissions/configuration/revisions |
| `cli-api-surface-agent-users-cli-agent-wake` | `missing` | `phases/08-interfaces-rest-cli-mcp.md` | `ACC-08-INTERFACES-REST-CLI-MCP-0061` | `backend-rs/tests/api_surface_parity.rs::cli_api_surface_agent_users_cli_agent_wake` | CLI `agent wake` |
| `cli-api-surface-agent-users-cli-approval-verbs` | `partial` | `phases/08-interfaces-rest-cli-mcp.md` | `ACC-08-INTERFACES-REST-CLI-MCP-0062` | `backend-rs/tests/api_surface_parity.rs::cli_api_surface_agent_users_cli_approval_verbs` | CLI approval commands |
| `cli-api-surface-agent-users-enum-agent-roles-statuses` | `divergent` | `phases/08-interfaces-rest-cli-mcp.md` | `ACC-08-INTERFACES-REST-CLI-MCP-0063` | `backend-rs/tests/api_surface_parity.rs::cli_api_surface_agent_users_enum_agent_roles_statuses` | Agent role/status enums |
| `cli-api-surface-agent-users-enum-approval-types` | `divergent` | `phases/08-interfaces-rest-cli-mcp.md` | `ACC-08-INTERFACES-REST-CLI-MCP-0064` | `backend-rs/tests/api_surface_parity.rs::cli_api_surface_agent_users_enum_approval_types` | Approval type/status enums |
| `cli-api-surface-agent-users-schema-create-agent` | `partial` | `phases/08-interfaces-rest-cli-mcp.md` | `ACC-08-INTERFACES-REST-CLI-MCP-0065` | `backend-rs/tests/api_surface_parity.rs::cli_api_surface_agent_users_schema_create_agent` | createAgent/createAgentHire schema |
| `cli-api-surface-agent-users-schema-permissions` | `partial` | `phases/08-interfaces-rest-cli-mcp.md` | `ACC-08-INTERFACES-REST-CLI-MCP-0066` | `backend-rs/tests/api_surface_parity.rs::cli_api_surface_agent_users_schema_permissions` | agentPermissionsSchema |
| `cli-api-surface-agent-users-side-effect-hire-approval-payload` | `partial` | `phases/08-interfaces-rest-cli-mcp.md` | `ACC-08-INTERFACES-REST-CLI-MCP-0067` | `backend-rs/tests/api_surface_parity.rs::cli_api_surface_agent_users_side_effect_hire_approval_payload` | Hire approval payload snapshot |
| `cli-api-surface-agent-users-side-effect-task-assign-grant` | `missing` | `phases/08-interfaces-rest-cli-mcp.md` | `ACC-08-INTERFACES-REST-CLI-MCP-0068` | `backend-rs/tests/api_surface_parity.rs::cli_api_surface_agent_users_side_effect_task_assign_grant` | Default tasks:assign grant on hire/create |
| `data-model-schema-agent-adapter-types-enum` | `divergent` | `phases/01-schema-authz-parity.md` | `ACC-01-SCHEMA-AUTHZ-PARITY-0069` | `backend-rs/tests/authz_parity.rs::data_model_schema_agent_adapter_types_enum` | AGENT_ADAPTER_TYPES enum |
| `data-model-schema-agent-budget-fields` | `missing` | `phases/01-schema-authz-parity.md` | `ACC-01-SCHEMA-AUTHZ-PARITY-0070` | `backend-rs/tests/authz_parity.rs::data_model_schema_agent_budget_fields` | per-agent monthly budget accounting |
| `data-model-schema-agent-config-revisions-table` | `missing` | `phases/01-schema-authz-parity.md` | `ACC-01-SCHEMA-AUTHZ-PARITY-0071` | `backend-rs/tests/authz_parity.rs::data_model_schema_agent_config_revisions_table` | agent_config_revisions audit table |
| `data-model-schema-agent-icon-names` | `missing` | `phases/01-schema-authz-parity.md` | `ACC-01-SCHEMA-AUTHZ-PARITY-0072` | `backend-rs/tests/authz_parity.rs::data_model_schema_agent_icon_names` | AGENT_ICON_NAMES enum |
| `data-model-schema-agent-roles-enum` | `divergent` | `phases/01-schema-authz-parity.md` | `ACC-01-SCHEMA-AUTHZ-PARITY-0073` | `backend-rs/tests/authz_parity.rs::data_model_schema_agent_roles_enum` | AGENT_ROLES enum + labels |
| `data-model-schema-agent-statuses-enum` | `divergent` | `phases/01-schema-authz-parity.md` | `ACC-01-SCHEMA-AUTHZ-PARITY-0074` | `backend-rs/tests/authz_parity.rs::data_model_schema_agent_statuses_enum` | AGENT_STATUSES enum values |
| `data-model-schema-agents-create-permission-key` | `partial` | `phases/01-schema-authz-parity.md` | `ACC-01-SCHEMA-AUTHZ-PARITY-0075` | `backend-rs/tests/authz_parity.rs::data_model_schema_agents_create_permission_key` | agents:create permission key |
| `data-model-schema-agents-fk-indexes` | `divergent` | `phases/01-schema-authz-parity.md` | `ACC-01-SCHEMA-AUTHZ-PARITY-0076` | `backend-rs/tests/authz_parity.rs::data_model_schema_agents_fk_indexes` | agents FK constraints and indexes |
| `data-model-schema-agents-orm-extra-columns` | `missing` | `phases/01-schema-authz-parity.md` | `ACC-01-SCHEMA-AUTHZ-PARITY-0077` | `backend-rs/tests/authz_parity.rs::data_model_schema_agents_orm_extra_columns` | agents ORM adds icon/runtime_config/env/pause/error columns |
| `data-model-schema-agents-permissions-column` | `partial` | `phases/01-schema-authz-parity.md` | `ACC-01-SCHEMA-AUTHZ-PARITY-0078` | `backend-rs/tests/authz_parity.rs::data_model_schema_agents_permissions_column` | agents.permissions jsonb column |
| `data-model-schema-agents-table-columns` | `partial` | `phases/01-schema-authz-parity.md` | `ACC-01-SCHEMA-AUTHZ-PARITY-0079` | `backend-rs/tests/authz_parity.rs::data_model_schema_agents_table_columns` | agents table column set |
| `data-model-schema-approval-comments-table` | `missing` | `phases/01-schema-authz-parity.md` | `ACC-01-SCHEMA-AUTHZ-PARITY-0080` | `backend-rs/tests/authz_parity.rs::data_model_schema_approval_comments_table` | approval_comments table |
| `data-model-schema-approval-statuses-enum` | `partial` | `phases/01-schema-authz-parity.md` | `ACC-01-SCHEMA-AUTHZ-PARITY-0081` | `backend-rs/tests/authz_parity.rs::data_model_schema_approval_statuses_enum` | APPROVAL_STATUSES enum |
| `data-model-schema-approval-types-enum` | `partial` | `phases/01-schema-authz-parity.md` | `ACC-01-SCHEMA-AUTHZ-PARITY-0082` | `backend-rs/tests/authz_parity.rs::data_model_schema_approval_types_enum` | APPROVAL_TYPES enum |
| `data-model-schema-approval-validators` | `partial` | `phases/01-schema-authz-parity.md` | `ACC-01-SCHEMA-AUTHZ-PARITY-0083` | `backend-rs/tests/authz_parity.rs::data_model_schema_approval_validators` | approval lifecycle validators |
| `data-model-schema-approvals-index` | `partial` | `phases/01-schema-authz-parity.md` | `ACC-01-SCHEMA-AUTHZ-PARITY-0084` | `backend-rs/tests/authz_parity.rs::data_model_schema_approvals_index` | approvals index |
| `data-model-schema-approvals-table-columns` | `partial` | `phases/01-schema-authz-parity.md` | `ACC-01-SCHEMA-AUTHZ-PARITY-0085` | `backend-rs/tests/authz_parity.rs::data_model_schema_approvals_table_columns` | approvals table column set |
| `data-model-schema-company-board-approval-toggle` | `divergent` | `phases/01-schema-authz-parity.md` | `ACC-01-SCHEMA-AUTHZ-PARITY-0086` | `backend-rs/tests/authz_parity.rs::data_model_schema_company_board_approval_toggle` | companies.require_board_approval_for_new_agents toggle |
| `data-model-schema-company-toggle-validators` | `missing` | `phases/01-schema-authz-parity.md` | `ACC-01-SCHEMA-AUTHZ-PARITY-0087` | `backend-rs/tests/authz_parity.rs::data_model_schema_company_toggle_validators` | board-approval toggle in company validators |
| `data-model-schema-create-agent-validator` | `partial` | `phases/01-schema-authz-parity.md` | `ACC-01-SCHEMA-AUTHZ-PARITY-0088` | `backend-rs/tests/authz_parity.rs::data_model_schema_create_agent_validator` | createAgentSchema fields/defaults |
| `data-model-schema-hire-modeled-via-approval` | `partial` | `phases/01-schema-authz-parity.md` | `ACC-01-SCHEMA-AUTHZ-PARITY-0089` | `backend-rs/tests/authz_parity.rs::data_model_schema_hire_modeled_via_approval` | Hiring modeled through approvals, not a hires table |
| `data-model-schema-issue-approvals-link-table` | `missing` | `phases/01-schema-authz-parity.md` | `ACC-01-SCHEMA-AUTHZ-PARITY-0090` | `backend-rs/tests/authz_parity.rs::data_model_schema_issue_approvals_link_table` | issue_approvals junction table |
| `data-model-schema-pause-reasons-enum` | `missing` | `phases/01-schema-authz-parity.md` | `ACC-01-SCHEMA-AUTHZ-PARITY-0091` | `backend-rs/tests/authz_parity.rs::data_model_schema_pause_reasons_enum` | PAUSE_REASONS enum |
| `data-model-schema-update-agent-permissions-validator` | `missing` | `phases/01-schema-authz-parity.md` | `ACC-01-SCHEMA-AUTHZ-PARITY-0092` | `backend-rs/tests/authz_parity.rs::data_model_schema_update_agent_permissions_validator` | updateAgentPermissionsSchema |
| `data-model-schema-update-agent-validator` | `missing` | `phases/01-schema-authz-parity.md` | `ACC-01-SCHEMA-AUTHZ-PARITY-0093` | `backend-rs/tests/authz_parity.rs::data_model_schema_update_agent_validator` | updateAgentSchema restrictions |
| `hiring-flow-side-effects-adapter-defaults-injection` | `missing` | `phases/05-adapters-secrets-skills-config.md` | `ACC-05-ADAPTERS-SECRETS-SKILLS-CONFIG-0094` | `backend-rs/tests/adapter_config_parity.rs::hiring_flow_side_effects_adapter_defaults_injection` | Adapter-type default config injection |
| `hiring-flow-side-effects-adapter-type-validation` | `missing` | `phases/05-adapters-secrets-skills-config.md` | `ACC-05-ADAPTERS-SECRETS-SKILLS-CONFIG-0095` | `backend-rs/tests/adapter_config_parity.rs::hiring_flow_side_effects_adapter_type_validation` | Adapter type must be a registered server adapter |
| `hiring-flow-side-effects-agent-actor-config-guard` | `missing` | `phases/05-adapters-secrets-skills-config.md` | `ACC-05-ADAPTERS-SECRETS-SKILLS-CONFIG-0096` | `backend-rs/tests/adapter_config_parity.rs::hiring_flow_side_effects_agent_actor_config_guard` | Agent-authenticated callers cannot set instructions/workspace-command config |
| `hiring-flow-side-effects-agent-create-side-effects` | `partial` | `phases/05-adapters-secrets-skills-config.md` | `ACC-05-ADAPTERS-SECRETS-SKILLS-CONFIG-0097` | `backend-rs/tests/adapter_config_parity.rs::hiring_flow_side_effects_agent_create_side_effects` | Agent creation side effects (name dedupe, manager validation, secret bindings) |
| `hiring-flow-side-effects-approval-gate-status` | `divergent` | `phases/01-schema-authz-parity.md` | `ACC-01-SCHEMA-AUTHZ-PARITY-0098` | `backend-rs/tests/authz_parity.rs::hiring_flow_side_effects_approval_gate_status` | Approval-required vs not: agent status branch |
| `hiring-flow-side-effects-approval-record-creation` | `partial` | `phases/05-adapters-secrets-skills-config.md` | `ACC-05-ADAPTERS-SECRETS-SKILLS-CONFIG-0099` | `backend-rs/tests/adapter_config_parity.rs::hiring_flow_side_effects_approval_record_creation` | hire_agent approval record created with redacted config snapshot |
| `hiring-flow-side-effects-codex-key-isolation` | `missing` | `phases/05-adapters-secrets-skills-config.md` | `ACC-05-ADAPTERS-SECRETS-SKILLS-CONFIG-0100` | `backend-rs/tests/adapter_config_parity.rs::hiring_flow_side_effects_codex_key_isolation` | codex_local per-agent CODEX_HOME isolation |
| `hiring-flow-side-effects-default-task-assign-grant` | `missing` | `phases/01-schema-authz-parity.md` | `ACC-01-SCHEMA-AUTHZ-PARITY-0101` | `backend-rs/tests/authz_parity.rs::hiring_flow_side_effects_default_task_assign_grant` | Default tasks:assign grant on every hire |
| `hiring-flow-side-effects-desired-skill-assignment` | `missing` | `phases/05-adapters-secrets-skills-config.md` | `ACC-05-ADAPTERS-SECRETS-SKILLS-CONFIG-0102` | `backend-rs/tests/adapter_config_parity.rs::hiring_flow_side_effects_desired_skill_assignment` | Desired skills resolution + skill sync preference persisted |
| `hiring-flow-side-effects-direct-create-blocked-when-approval` | `missing` | `phases/01-schema-authz-parity.md` | `ACC-01-SCHEMA-AUTHZ-PARITY-0103` | `backend-rs/tests/authz_parity.rs::hiring_flow_side_effects_direct_create_blocked_when_approval` | Direct POST /agents blocked when approvals required |
| `hiring-flow-side-effects-hire-activity-log` | `missing` | `phases/01-schema-authz-parity.md` | `ACC-01-SCHEMA-AUTHZ-PARITY-0104` | `backend-rs/tests/authz_parity.rs::hiring_flow_side_effects_hire_activity_log` | agent.hire_created + approval.created activity logs + telemetry |
| `hiring-flow-side-effects-hire-approve-activation` | `partial` | `phases/01-schema-authz-parity.md` | `ACC-01-SCHEMA-AUTHZ-PARITY-0105` | `backend-rs/tests/authz_parity.rs::hiring_flow_side_effects_hire_approve_activation` | Approving a hire_agent approval activates agent + budget + hook |
| `hiring-flow-side-effects-hire-endpoint-authz` | `partial` | `phases/01-schema-authz-parity.md` | `ACC-01-SCHEMA-AUTHZ-PARITY-0106` | `backend-rs/tests/authz_parity.rs::hiring_flow_side_effects_hire_endpoint_authz` | POST /companies/:companyId/agent-hires with agents:create authz |
| `hiring-flow-side-effects-hire-hook-onhireapproved` | `missing` | `phases/05-adapters-secrets-skills-config.md` | `ACC-05-ADAPTERS-SECRETS-SKILLS-CONFIG-0107` | `backend-rs/tests/adapter_config_parity.rs::hiring_flow_side_effects_hire_hook_onhireapproved` | onHireApproved adapter hook (hire-hook.ts) |
| `hiring-flow-side-effects-hire-input-schema` | `partial` | `phases/01-schema-authz-parity.md` | `ACC-01-SCHEMA-AUTHZ-PARITY-0108` | `backend-rs/tests/authz_parity.rs::hiring_flow_side_effects_hire_input_schema` | createAgentHireSchema hire payload fields |
| `hiring-flow-side-effects-hire-reject-terminate` | `partial` | `phases/01-schema-authz-parity.md` | `ACC-01-SCHEMA-AUTHZ-PARITY-0109` | `backend-rs/tests/authz_parity.rs::hiring_flow_side_effects_hire_reject_terminate` | Rejecting a hire_agent approval terminates the pending agent |
| `hiring-flow-side-effects-instructions-bundle-materialization` | `missing` | `phases/05-adapters-secrets-skills-config.md` | `ACC-05-ADAPTERS-SECRETS-SKILLS-CONFIG-0110` | `backend-rs/tests/adapter_config_parity.rs::hiring_flow_side_effects_instructions_bundle_materialization` | Default instructions bundle materialized on hire |
| `hiring-flow-side-effects-join-request-hire-hook` | `missing` | `phases/01-schema-authz-parity.md` | `ACC-01-SCHEMA-AUTHZ-PARITY-0111` | `backend-rs/tests/authz_parity.rs::hiring_flow_side_effects_join_request_hire_hook` | Join-request approval also triggers hire hook |
| `hiring-flow-side-effects-legacy-prompt-guard` | `missing` | `phases/01-schema-authz-parity.md` | `ACC-01-SCHEMA-AUTHZ-PARITY-0112` | `backend-rs/tests/authz_parity.rs::hiring_flow_side_effects_legacy_prompt_guard` | Reject legacy promptTemplate on new hires |
| `hiring-flow-side-effects-plugin-managed-agent-hire` | `missing` | `phases/01-schema-authz-parity.md` | `ACC-01-SCHEMA-AUTHZ-PARITY-0113` | `backend-rs/tests/authz_parity.rs::hiring_flow_side_effects_plugin_managed_agent_hire` | Plugin-managed agent creation mirrors the hire+approval flow |
| `hiring-flow-side-effects-role-status-adapter-enums` | `divergent` | `phases/05-adapters-secrets-skills-config.md` | `ACC-05-ADAPTERS-SECRETS-SKILLS-CONFIG-0114` | `backend-rs/tests/adapter_config_parity.rs::hiring_flow_side_effects_role_status_adapter_enums` | Role / status / adapter-type enum values |
| `hiring-flow-side-effects-runtime-heartbeat-normalization` | `missing` | `phases/05-adapters-secrets-skills-config.md` | `ACC-05-ADAPTERS-SECRETS-SKILLS-CONFIG-0115` | `backend-rs/tests/adapter_config_parity.rs::hiring_flow_side_effects_runtime_heartbeat_normalization` | Runtime config heartbeat defaults |
| `hiring-flow-side-effects-secret-normalization` | `missing` | `phases/05-adapters-secrets-skills-config.md` | `ACC-05-ADAPTERS-SECRETS-SKILLS-CONFIG-0116` | `backend-rs/tests/adapter_config_parity.rs::hiring_flow_side_effects_secret_normalization` | Adapter/runtime config secret normalization + constraints |
| `hiring-flow-side-effects-source-issue-linking` | `missing` | `phases/01-schema-authz-parity.md` | `ACC-01-SCHEMA-AUTHZ-PARITY-0117` | `backend-rs/tests/authz_parity.rs::hiring_flow_side_effects_source_issue_linking` | Source issues linked to approval |
| `permissions-board-membership-access-activity-audit-side-effects-for-pe` | `missing` | `phases/01-schema-authz-parity.md` | `ACC-01-SCHEMA-AUTHZ-PARITY-0118` | `backend-rs/tests/authz_parity.rs::permissions_board_membership_access_activity_audit_side_effects_for_pe` | Activity/audit side effects for permission and hiring actions |
| `permissions-board-membership-access-agent-api-key-scopes-and-key-subje` | `partial` | `phases/01-schema-authz-parity.md` | `ACC-01-SCHEMA-AUTHZ-PARITY-0119` | `backend-rs/tests/authz_parity.rs::permissions_board_membership_access_agent_api_key_scopes_and_key_subje` | Agent API key scopes and key subject constraints |
| `permissions-board-membership-access-agent-cancreateagents-hiring-autho` | `partial` | `phases/01-schema-authz-parity.md` | `ACC-01-SCHEMA-AUTHZ-PARITY-0120` | `backend-rs/tests/authz_parity.rs::permissions_board_membership_access_agent_cancreateagents_hiring_autho` | Agent `canCreateAgents` / hiring authority |
| `permissions-board-membership-access-agent-configuration-read-update-au` | `missing` | `phases/01-schema-authz-parity.md` | `ACC-01-SCHEMA-AUTHZ-PARITY-0121` | `backend-rs/tests/authz_parity.rs::permissions_board_membership_access_agent_configuration_read_update_au` | Agent configuration read/update authorization and redaction |
| `permissions-board-membership-access-approval-decision-authority-for-hi` | `partial` | `phases/01-schema-authz-parity.md` | `ACC-01-SCHEMA-AUTHZ-PARITY-0122` | `backend-rs/tests/authz_parity.rs::permissions_board_membership_access_approval_decision_authority_for_hi` | Approval decision authority for hires |
| `permissions-board-membership-access-board-identification-and-company-a` | `partial` | `phases/01-schema-authz-parity.md` | `ACC-01-SCHEMA-AUTHZ-PARITY-0123` | `backend-rs/tests/authz_parity.rs::permissions_board_membership_access_board_identification_and_company_a` | Board identification and company access guard |
| `permissions-board-membership-access-company-job-access-under-api-key-v` | `divergent` | `phases/01-schema-authz-parity.md` | `ACC-01-SCHEMA-AUTHZ-PARITY-0124` | `backend-rs/tests/authz_parity.rs::permissions_board_membership_access_company_job_access_under_api_key_v` | Company/job access under API key vs session |
| `permissions-board-membership-access-company-member-management-and-cust` | `missing` | `phases/01-schema-authz-parity.md` | `ACC-01-SCHEMA-AUTHZ-PARITY-0125` | `backend-rs/tests/authz_parity.rs::permissions_board_membership_access_company_member_management_and_cust` | Company member management and custom permission grants |
| `permissions-board-membership-access-frontend-user-visible-board-staffi` | `partial` | `phases/01-schema-authz-parity.md` | `ACC-01-SCHEMA-AUTHZ-PARITY-0126` | `backend-rs/tests/authz_parity.rs::permissions_board_membership_access_frontend_user_visible_board_staffi` | Frontend/user-visible board staffing controls |
| `permissions-board-membership-access-human-board-roles-and-default-gran` | `divergent` | `phases/01-schema-authz-parity.md` | `ACC-01-SCHEMA-AUTHZ-PARITY-0127` | `backend-rs/tests/authz_parity.rs::permissions_board_membership_access_human_board_roles_and_default_gran` | Human board roles and default grants |
| `permissions-board-membership-access-invite-join-request-and-approval-p` | `partial` | `phases/01-schema-authz-parity.md` | `ACC-01-SCHEMA-AUTHZ-PARITY-0128` | `backend-rs/tests/authz_parity.rs::permissions_board_membership_access_invite_join_request_and_approval_p` | Invite, join request, and approval permissions |
| `permissions-board-membership-access-original-permission-key-inventory` | `missing` | `phases/01-schema-authz-parity.md` | `ACC-01-SCHEMA-AUTHZ-PARITY-0129` | `backend-rs/tests/authz_parity.rs::permissions_board_membership_access_original_permission_key_inventory` | Original permission key inventory |
| `permissions-board-membership-access-permission-update-endpoint-for-age` | `missing` | `phases/01-schema-authz-parity.md` | `ACC-01-SCHEMA-AUTHZ-PARITY-0130` | `backend-rs/tests/authz_parity.rs::permissions_board_membership_access_permission_update_endpoint_for_age` | Permission update endpoint for agents (`canCreateAgents`, `canCreateSkills`, `canAssignTasks`) |
| `permissions-board-membership-access-protected-private-agent-authorizat` | `missing` | `phases/01-schema-authz-parity.md` | `ACC-01-SCHEMA-AUTHZ-PARITY-0131` | `backend-rs/tests/authz_parity.rs::permissions_board_membership_access_protected_private_agent_authorizat` | Protected/private agent authorization policies and low-trust boundaries |
| `permissions-board-membership-access-reports-to-same-company-hierarchy-` | `full` | `phases/01-schema-authz-parity.md` | `ACC-01-SCHEMA-AUTHZ-PARITY-0132` | `backend-rs/tests/authz_parity.rs::permissions_board_membership_access_reports_to_same_company_hierarchy_` | Reports-to / same-company hierarchy check for hires |
| `permissions-board-membership-access-require-board-approval-for-new-age` | `divergent` | `phases/01-schema-authz-parity.md` | `ACC-01-SCHEMA-AUTHZ-PARITY-0133` | `backend-rs/tests/authz_parity.rs::permissions_board_membership_access_require_board_approval_for_new_age` | `require_board_approval_for_new_agents` default and direct-hire semantics |
| `permissions-board-membership-access-status-model-for-agent-membership-` | `divergent` | `phases/01-schema-authz-parity.md` | `ACC-01-SCHEMA-AUTHZ-PARITY-0134` | `backend-rs/tests/authz_parity.rs::permissions_board_membership_access_status_model_for_agent_membership_` | Status model for agent membership/hiring lifecycle |
| `permissions-board-membership-access-task-assignment-permissions-tasks-` | `missing` | `phases/01-schema-authz-parity.md` | `ACC-01-SCHEMA-AUTHZ-PARITY-0135` | `backend-rs/tests/authz_parity.rs::permissions_board_membership_access_task_assignment_permissions_tasks_` | Task assignment permissions (`tasks:assign`, scoped assignment, simple default) |
| `ui-surface-human-non-agent-users-agent-action-assign-task` | `missing` | `phases/09-nuxt-board-ui-parity.md` | `ACC-09-NUXT-BOARD-UI-PARITY-0136` | `frontend-nuxt/tests/ui-parity.test.ts::ui_surface_human_non_agent_users_agent_action_assign_task` | Assign Task action |
| `ui-surface-human-non-agent-users-agent-action-clear-error` | `missing` | `phases/09-nuxt-board-ui-parity.md` | `ACC-09-NUXT-BOARD-UI-PARITY-0137` | `frontend-nuxt/tests/ui-parity.test.ts::ui_surface_human_non_agent_users_agent_action_clear_error` | Clear error action |
| `ui-surface-human-non-agent-users-agent-action-invoke` | `missing` | `phases/09-nuxt-board-ui-parity.md` | `ACC-09-NUXT-BOARD-UI-PARITY-0138` | `frontend-nuxt/tests/ui-parity.test.ts::ui_surface_human_non_agent_users_agent_action_invoke` | Run heartbeat / invoke action |
| `ui-surface-human-non-agent-users-agent-action-overflow-menu` | `missing` | `phases/09-nuxt-board-ui-parity.md` | `ACC-09-NUXT-BOARD-UI-PARITY-0139` | `frontend-nuxt/tests/ui-parity.test.ts::ui_surface_human_non_agent_users_agent_action_overflow_menu` | Overflow menu: duplicate / copy ID / reset sessions / terminate |
| `ui-surface-human-non-agent-users-agent-action-pause-resume` | `missing` | `phases/09-nuxt-board-ui-parity.md` | `ACC-09-NUXT-BOARD-UI-PARITY-0140` | `frontend-nuxt/tests/ui-parity.test.ts::ui_surface_human_non_agent_users_agent_action_pause_resume` | Pause / Resume action |
| `ui-surface-human-non-agent-users-agent-budget-tab` | `missing` | `phases/09-nuxt-board-ui-parity.md` | `ACC-09-NUXT-BOARD-UI-PARITY-0141` | `frontend-nuxt/tests/ui-parity.test.ts::ui_surface_human_non_agent_users_agent_budget_tab` | Per-agent budget policy |
| `ui-surface-human-non-agent-users-agent-config-secret-redaction` | `missing` | `phases/09-nuxt-board-ui-parity.md` | `ACC-09-NUXT-BOARD-UI-PARITY-0142` | `frontend-nuxt/tests/ui-parity.test.ts::ui_surface_human_non_agent_users_agent_config_secret_redaction` | Run log/env secret + path redaction in UI |
| `ui-surface-human-non-agent-users-agent-dashboard-overview` | `missing` | `phases/09-nuxt-board-ui-parity.md` | `ACC-09-NUXT-BOARD-UI-PARITY-0143` | `frontend-nuxt/tests/ui-parity.test.ts::ui_surface_human_non_agent_users_agent_dashboard_overview` | Agent dashboard overview |
| `ui-surface-human-non-agent-users-agent-detail-tabs` | `missing` | `phases/09-nuxt-board-ui-parity.md` | `ACC-09-NUXT-BOARD-UI-PARITY-0144` | `frontend-nuxt/tests/ui-parity.test.ts::ui_surface_human_non_agent_users_agent_detail_tabs` | Agent detail tabbed page |
| `ui-surface-human-non-agent-users-agent-icon-edit` | `missing` | `phases/09-nuxt-board-ui-parity.md` | `ACC-09-NUXT-BOARD-UI-PARITY-0145` | `frontend-nuxt/tests/ui-parity.test.ts::ui_surface_human_non_agent_users_agent_icon_edit` | Edit agent icon |
| `ui-surface-human-non-agent-users-agent-invalid-org-chain` | `missing` | `phases/09-nuxt-board-ui-parity.md` | `ACC-09-NUXT-BOARD-UI-PARITY-0146` | `frontend-nuxt/tests/ui-parity.test.ts::ui_surface_human_non_agent_users_agent_invalid_org_chain` | Invalid reporting-chain warning + gating |
| `ui-surface-human-non-agent-users-agent-membership-join-leave` | `missing` | `phases/09-nuxt-board-ui-parity.md` | `ACC-09-NUXT-BOARD-UI-PARITY-0147` | `frontend-nuxt/tests/ui-parity.test.ts::ui_surface_human_non_agent_users_agent_membership_join_leave` | Join/leave agent membership |
| `ui-surface-human-non-agent-users-agent-pending-approval-banner` | `partial` | `phases/09-nuxt-board-ui-parity.md` | `ACC-09-NUXT-BOARD-UI-PARITY-0148` | `frontend-nuxt/tests/ui-parity.test.ts::ui_surface_human_non_agent_users_agent_pending_approval_banner` | Pending-approval banner + approve on detail |
| `ui-surface-human-non-agent-users-agent-permissions-update` | `missing` | `phases/09-nuxt-board-ui-parity.md` | `ACC-09-NUXT-BOARD-UI-PARITY-0149` | `frontend-nuxt/tests/ui-parity.test.ts::ui_surface_human_non_agent_users_agent_permissions_update` | Update agent permissions/trust |
| `ui-surface-human-non-agent-users-agents-api-surface` | `partial` | `phases/09-nuxt-board-ui-parity.md` | `ACC-09-NUXT-BOARD-UI-PARITY-0150` | `frontend-nuxt/tests/ui-parity.test.ts::ui_surface_human_non_agent_users_agents_api_surface` | Agents REST API client surface |
| `ui-surface-human-non-agent-users-agents-list-view` | `partial` | `phases/09-nuxt-board-ui-parity.md` | `ACC-09-NUXT-BOARD-UI-PARITY-0151` | `frontend-nuxt/tests/ui-parity.test.ts::ui_surface_human_non_agent_users_agents_list_view` | Agents list (list view) |
| `ui-surface-human-non-agent-users-agents-live-run-indicator` | `missing` | `phases/09-nuxt-board-ui-parity.md` | `ACC-09-NUXT-BOARD-UI-PARITY-0152` | `frontend-nuxt/tests/ui-parity.test.ts::ui_surface_human_non_agent_users_agents_live_run_indicator` | Live run indicator |
| `ui-surface-human-non-agent-users-agents-org-chart` | `missing` | `phases/09-nuxt-board-ui-parity.md` | `ACC-09-NUXT-BOARD-UI-PARITY-0153` | `frontend-nuxt/tests/ui-parity.test.ts::ui_surface_human_non_agent_users_agents_org_chart` | Org chart (tree) view |
| `ui-surface-human-non-agent-users-agents-status-filter-tabs` | `missing` | `phases/09-nuxt-board-ui-parity.md` | `ACC-09-NUXT-BOARD-UI-PARITY-0154` | `frontend-nuxt/tests/ui-parity.test.ts::ui_surface_human_non_agent_users_agents_status_filter_tabs` | Agent status filter tabs |
| `ui-surface-human-non-agent-users-approval-approve-reject` | `full` | `phases/09-nuxt-board-ui-parity.md` | `ACC-09-NUXT-BOARD-UI-PARITY-0155` | `frontend-nuxt/tests/ui-parity.test.ts::ui_surface_human_non_agent_users_approval_approve_reject` | Approve / Reject approval |
| `ui-surface-human-non-agent-users-approval-budget-special-case` | `missing` | `phases/09-nuxt-board-ui-parity.md` | `ACC-09-NUXT-BOARD-UI-PARITY-0156` | `frontend-nuxt/tests/ui-parity.test.ts::ui_surface_human_non_agent_users_approval_budget_special_case` | Budget override approval redirect |
| `ui-surface-human-non-agent-users-approval-comments` | `missing` | `phases/09-nuxt-board-ui-parity.md` | `ACC-09-NUXT-BOARD-UI-PARITY-0157` | `frontend-nuxt/tests/ui-parity.test.ts::ui_surface_human_non_agent_users_approval_comments` | Approval comments thread |
| `ui-surface-human-non-agent-users-approval-delete-disapproved-agent` | `divergent` | `phases/09-nuxt-board-ui-parity.md` | `ACC-09-NUXT-BOARD-UI-PARITY-0158` | `frontend-nuxt/tests/ui-parity.test.ts::ui_surface_human_non_agent_users_approval_delete_disapproved_agent` | Delete disapproved hired agent |
| `ui-surface-human-non-agent-users-approval-detail-view` | `missing` | `phases/09-nuxt-board-ui-parity.md` | `ACC-09-NUXT-BOARD-UI-PARITY-0159` | `frontend-nuxt/tests/ui-parity.test.ts::ui_surface_human_non_agent_users_approval_detail_view` | Approval detail page |
| `ui-surface-human-non-agent-users-approval-payload-renderers` | `missing` | `phases/09-nuxt-board-ui-parity.md` | `ACC-09-NUXT-BOARD-UI-PARITY-0160` | `frontend-nuxt/tests/ui-parity.test.ts::ui_surface_human_non_agent_users_approval_payload_renderers` | Typed approval payload renderers |
| `ui-surface-human-non-agent-users-approval-revision-resubmit` | `missing` | `phases/09-nuxt-board-ui-parity.md` | `ACC-09-NUXT-BOARD-UI-PARITY-0161` | `frontend-nuxt/tests/ui-parity.test.ts::ui_surface_human_non_agent_users_approval_revision_resubmit` | Request revision / Mark resubmitted |
| `ui-surface-human-non-agent-users-board-chat-conference-room` | `missing` | `phases/09-nuxt-board-ui-parity.md` | `ACC-09-NUXT-BOARD-UI-PARITY-0162` | `frontend-nuxt/tests/ui-parity.test.ts::ui_surface_human_non_agent_users_board_chat_conference_room` | Board Concierge chat (Conference Room) |
| `ui-surface-human-non-agent-users-board-chat-feedback-votes` | `missing` | `phases/09-nuxt-board-ui-parity.md` | `ACC-09-NUXT-BOARD-UI-PARITY-0163` | `frontend-nuxt/tests/ui-parity.test.ts::ui_surface_human_non_agent_users_board_chat_feedback_votes` | Board chat message feedback votes |
| `ui-surface-human-non-agent-users-company-archive` | `missing` | `phases/09-nuxt-board-ui-parity.md` | `ACC-09-NUXT-BOARD-UI-PARITY-0164` | `frontend-nuxt/tests/ui-parity.test.ts::ui_surface_human_non_agent_users_company_archive` | Archive company (danger zone) |
| `ui-surface-human-non-agent-users-company-board-approval-toggle` | `partial` | `phases/09-nuxt-board-ui-parity.md` | `ACC-09-NUXT-BOARD-UI-PARITY-0165` | `frontend-nuxt/tests/ui-parity.test.ts::ui_surface_human_non_agent_users_company_board_approval_toggle` | Company setting: require board approval for new hires |
| `ui-surface-human-non-agent-users-company-general-settings` | `missing` | `phases/09-nuxt-board-ui-parity.md` | `ACC-09-NUXT-BOARD-UI-PARITY-0166` | `frontend-nuxt/tests/ui-parity.test.ts::ui_surface_human_non_agent_users_company_general_settings` | Company general/appearance settings |
| `ui-surface-human-non-agent-users-company-packages-links` | `missing` | `phases/09-nuxt-board-ui-parity.md` | `ACC-09-NUXT-BOARD-UI-PARITY-0167` | `frontend-nuxt/tests/ui-parity.test.ts::ui_surface_human_non_agent_users_company_packages_links` | Company import/export/cloud links |
| `ui-surface-human-non-agent-users-first-agent-is-ceo` | `missing` | `phases/09-nuxt-board-ui-parity.md` | `ACC-09-NUXT-BOARD-UI-PARITY-0168` | `frontend-nuxt/tests/ui-parity.test.ts::ui_surface_human_non_agent_users_first_agent_is_ceo` | First agent forced to CEO |
| `ui-surface-human-non-agent-users-hire-adapter-presets` | `missing` | `phases/09-nuxt-board-ui-parity.md` | `ACC-09-NUXT-BOARD-UI-PARITY-0169` | `frontend-nuxt/tests/ui-parity.test.ts::ui_surface_human_non_agent_users_hire_adapter_presets` | Adapter-type presets and validation |
| `ui-surface-human-non-agent-users-hire-company-skills-select` | `missing` | `phases/09-nuxt-board-ui-parity.md` | `ACC-09-NUXT-BOARD-UI-PARITY-0170` | `frontend-nuxt/tests/ui-parity.test.ts::ui_surface_human_non_agent_users_hire_company_skills_select` | Optional company-skill selection on hire |
| `ui-surface-human-non-agent-users-hire-test-agent` | `missing` | `phases/09-nuxt-board-ui-parity.md` | `ACC-09-NUXT-BOARD-UI-PARITY-0171` | `frontend-nuxt/tests/ui-parity.test.ts::ui_surface_human_non_agent_users_hire_test_agent` | Test Agent (adapter environment probe) button |
| `ui-surface-human-non-agent-users-hire-trust-preset` | `missing` | `phases/09-nuxt-board-ui-parity.md` | `ACC-09-NUXT-BOARD-UI-PARITY-0172` | `frontend-nuxt/tests/ui-parity.test.ts::ui_surface_human_non_agent_users_hire_trust_preset` | Trust preset + low-trust boundary picker on hire |
| `ui-surface-human-non-agent-users-invite-external-agent` | `missing` | `phases/09-nuxt-board-ui-parity.md` | `ACC-09-NUXT-BOARD-UI-PARITY-0173` | `frontend-nuxt/tests/ui-parity.test.ts::ui_surface_human_non_agent_users_invite_external_agent` | External agent invite + onboarding prompt |
| `ui-surface-human-non-agent-users-new-agent-dialog-choices` | `missing` | `phases/09-nuxt-board-ui-parity.md` | `ACC-09-NUXT-BOARD-UI-PARITY-0174` | `frontend-nuxt/tests/ui-parity.test.ts::ui_surface_human_non_agent_users_new_agent_dialog_choices` | New Agent dialog with 4 creation paths |
| `ui-surface-human-non-agent-users-new-agent-hire-form` | `partial` | `phases/09-nuxt-board-ui-parity.md` | `ACC-09-NUXT-BOARD-UI-PARITY-0175` | `frontend-nuxt/tests/ui-parity.test.ts::ui_surface_human_non_agent_users_new_agent_hire_form` | Advanced New Agent hire form |

## Row schema

| Field | Required | Meaning |
|---|---|---|
| `row_id` | yes | Stable capability identifier. |
| `domain` | yes | Audit domain. |
| `capability` | yes | Capability name. |
| `original_evidence` | yes | Source proving original behavior. |
| `current_rewrite_evidence` | yes | Source proving current rewrite status. |
| `initial_status` | yes | full/partial/missing/divergent from corrected audit. |
| `phase` | yes for non-full | Owning phase file. |
| `acceptance_id` | yes for non-full | Harness assertion proving closure. |
| `failing_test` | yes for non-full | Red test path/name. |
| `passing_test` | yes for non-full | Green command. |
| `evidence_ref` | yes for non-full | Post-closure original+rewrite evidence. |
| `final_status` | yes | full, waived, or unclosed during implementation. |

hook: PreToolUse
hook: PreToolUse
hook: PreToolUse Completed
hook: PreToolUse Completed
hook: PostToolUse
hook: PostToolUse Completed
exec
/bin/bash -lc 'wc -l docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.jsonl docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.md' in /home/frankji/Projects/paperclip
 succeeded in 0ms:
   175 docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.jsonl
   223 docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.md
   398 total

hook: PostToolUse
hook: PostToolUse Completed
exec
/bin/bash -lc "python3 - <<'PY'
import json, collections
p='docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.jsonl'
rows=[json.loads(l) for l in open(p) if l.strip()]
print('rows', len(rows))
print('initial', collections.Counter(r.get('initial_status') for r in rows))
print('final', collections.Counter(r.get('final_status') for r in rows))
print('missing fields open rows')
req=['phase','acceptance_id','failing_test','passing_test','evidence_ref','final_status']
for r in rows[:10]: print(r)
miss=[]
for r in rows:
    if r.get('initial_status')"'!='"'full':
        absent=[k for k in req if not r.get(k)]
        if absent: miss.append((r.get('row_id'), absent))
print('nonfull missing closure fields', len(miss))
print(miss[:20])
PY" in /home/frankji/Projects/paperclip
 succeeded in 0ms:
rows 175
initial Counter({'missing': 100, 'partial': 50, 'divergent': 22, 'full': 3})
final Counter({'unclosed': 172, 'full': 3})
missing fields open rows
{'acceptance_id': 'ACC-07-AGENT-RUNTIME-LIFECYCLE-0001', 'capability': 'Agent-key permission to request hires', 'current_rewrite_evidence': '/home/frankji/Projects/paperclip/supabase/migrations/paperclip/0013_agents_hiring.sql:122-132 checks agents:write scope, active acting agent, and can_create_agents; /home/frankji/Projects/paperclip/supabase/migrations/paperclip/0013_agents_hiring.sql:141-143 forces newly hired agent permissions to can_create_agents false on API-key path.', 'domain': 'Agent lifecycle & status transitions', 'evidence_ref': 'original_evidence + current_rewrite_evidence', 'failing_test': 'backend-rs/tests/agent_lifecycle_parity.rs::agent_lifecycle_status_transitions_agent_key_permission_to_request_hir', 'final_status': 'unclosed', 'initial_status': 'partial', 'original_behavior': 'Rewrite preserves a narrow slice: scoped API keys can hire only if the acting agent is active and has can_create_agents. However, the original uses centralized authorization (`agents:create` permission grants plus legacy canCreateAgents), company access assertions, and broader route-side control-plane behavior.', 'original_evidence': '/home/frankji/Projects/paperclip/server/src/routes/agents.ts:656-672 gates agent creation through access.decide action agents:create and verifies the actor agent is in the same company; /home/frankji/Projects/paperclip/server/src/routes/agents.ts:640-654 applies default tasks:assign permission grants to created agents.', 'passing_test': 'cd backend-rs && cargo test --test agent_lifecycle_parity', 'phase': 'phases/07-agent-runtime-lifecycle.md', 'review_note': 'Rewrite preserves a narrow slice: scoped API keys can hire only if the acting agent is active and has can_create_agents. However, the original uses centralized authorization (`agents:create` permission grants plus legacy canCreateAgents), company access assertions, and broader route-side control-plane behavior.', 'row_id': 'agent-lifecycle-status-transitions-agent-key-permission-to-request-hir', 'verdict': 'CONFIRMED', 'waiver_ref': ''}
{'acceptance_id': 'ACC-07-AGENT-RUNTIME-LIFECYCLE-0002', 'capability': 'Approval-gated hiring flow', 'current_rewrite_evidence': '/home/frankji/Projects/paperclip/supabase/migrations/paperclip/0013_agents_hiring.sql:84-151 implements hire_agent and hire_agent_with_key with name/role/title/reports_to/model only; /home/frankji/Projects/paperclip/backend-rs/src/jobs/mod.rs:231-271 calls those RPCs; /home/frankji/Projects/paperclip/frontend-nuxt/composables/usePaperclipSession.ts:81-99 exposes only name/role/model.', 'domain': 'Agent lifecycle & status transitions', 'evidence_ref': 'original_evidence + current_rewrite_evidence', 'failing_test': 'backend-rs/tests/agent_lifecycle_parity.rs::agent_lifecycle_status_transitions_approval_gated_hiring_flow', 'final_status': 'unclosed', 'initial_status': 'partial', 'original_behavior': 'The rewrite has an approval-gated hire flow with session and API-key paths, same-company manager validation, and board-only approval decisions. It does not match original side effects and data richness: no adapterConfig/runtimeConfig/capabilities/budget/default task assignment grant/instructions bundle/secret binding parity, and approved status is active rather than idle.', 'original_evidence': '/home/frankji/Projects/paperclip/server/src/routes/agents.ts:2194-2371 creates pending hire rows, redacts config into hire approval payload, links source issues, logs agent.hire_created and approval.created, applies default task assignment grant, and materializes default instructions; /home/frankji/Projects/paperclip/server/src/services/approvals.ts:125-180 activates pending hires, creates budget policy, and notifies hire approval.', 'passing_test': 'cd backend-rs && cargo test --test agent_lifecycle_parity', 'phase': 'phases/07-agent-runtime-lifecycle.md', 'review_note': 'The rewrite has an approval-gated hire flow with session and API-key paths, same-company manager validation, and board-only approval decisions. It does not match original side effects and data richness: no adapterConfig/runtimeConfig/capabilities/budget/default task assignment grant/instructions bundle/secret binding parity, and approved status is active rather than idle.', 'row_id': 'agent-lifecycle-status-transitions-approval-gated-hiring-flow', 'verdict': 'CONFIRMED', 'waiver_ref': ''}
{'acceptance_id': 'ACC-07-AGENT-RUNTIME-LIFECYCLE-0003', 'capability': 'Approving pending hires', 'current_rewrite_evidence': '/home/frankji/Projects/paperclip/supabase/migrations/paperclip/0013_agents_hiring.sql:164-169 sets status active and approval approved; /home/frankji/Projects/paperclip/supabase/migrations/paperclip/0013_agents_hiring.sql:193-198 repeats active transition for key-backed decide_approval.', 'domain': 'Agent lifecycle & status transitions', 'evidence_ref': 'original_evidence + current_rewrite_evidence', 'failing_test': 'backend-rs/tests/agent_lifecycle_parity.rs::agent_lifecycle_status_transitions_approving_pending_hires', 'final_status': 'unclosed', 'initial_status': 'divergent', 'original_behavior': 'Original approval resolves a pending hire into `idle`, sets budget policy if needed, and sends hire-approved notification side effects. Rewrite resolves to `active`, has no budget policy or notification, and does not enter the original idle runtime state.', 'original_evidence': '/home/frankji/Projects/paperclip/server/src/services/agents.ts:678-684 sets pending_approval agents to idle; /home/frankji/Projects/paperclip/server/src/services/approvals.ts:125-180 invokes activation, budget policy upsert, and notifyHireApproved.', 'passing_test': 'cd backend-rs && cargo test --test agent_lifecycle_parity', 'phase': 'phases/07-agent-runtime-lifecycle.md', 'review_note': 'Original approval resolves a pending hire into `idle`, sets budget policy if needed, and sends hire-approved notification side effects. Rewrite resolves to `active`, has no budget policy or notification, and does not enter the original idle runtime state.', 'row_id': 'agent-lifecycle-status-transitions-approving-pending-hires', 'verdict': 'CONFIRMED', 'waiver_ref': ''}
{'acceptance_id': 'ACC-07-AGENT-RUNTIME-LIFECYCLE-0004', 'capability': 'Canonical agent status vocabulary', 'current_rewrite_evidence': '/home/frankji/Projects/paperclip/supabase/migrations/paperclip/0013_agents_hiring.sql:9-12 creates enum paperclip.agent_status as pending_approval, active, paused, archived.', 'domain': 'Agent lifecycle & status transitions', 'evidence_ref': 'original_evidence + current_rewrite_evidence', 'failing_test': 'backend-rs/tests/agent_lifecycle_parity.rs::agent_lifecycle_status_transitions_canonical_agent_status_vocabulary', 'final_status': 'unclosed', 'initial_status': 'divergent', 'original_behavior': 'The rewrite is not just incomplete; it defines a different lifecycle vocabulary. Original V1 includes runtime states (`idle`, `running`, `error`) and irreversible `terminated`; rewrite uses a smaller staffing-state enum with `active` and `archived`.', 'original_evidence': '/home/frankji/Projects/paperclip/packages/shared/src/constants.ts:19-27 defines AGENT_STATUSES as active, paused, idle, running, error, pending_approval, terminated.', 'passing_test': 'cd backend-rs && cargo test --test agent_lifecycle_parity', 'phase': 'phases/07-agent-runtime-lifecycle.md', 'review_note': 'The rewrite is not just incomplete; it defines a different lifecycle vocabulary. Original V1 includes runtime states (`idle`, `running`, `error`) and irreversible `terminated`; rewrite uses a smaller staffing-state enum with `active` and `archived`.', 'row_id': 'agent-lifecycle-status-transitions-canonical-agent-status-vocabulary', 'verdict': 'CONFIRMED', 'waiver_ref': ''}
{'acceptance_id': 'ACC-07-AGENT-RUNTIME-LIFECYCLE-0005', 'capability': 'Company/auth boundary for lifecycle mutations', 'current_rewrite_evidence': '/home/frankji/Projects/paperclip/supabase/migrations/paperclip/0011_companies_jobs.sql:51-68 enables RLS read policies for companies/jobs; /home/frankji/Projects/paperclip/supabase/migrations/paperclip/0012_api_key_scopes.sql:7-16 implements key-scope checks; /home/frankji/Projects/paperclip/supabase/migrations/paperclip/0013_agents_hiring.sql:153-206 implements board-only approval decisions and forbids agent approval decisions; /home/frankji/Projects/paperclip/backend-rs/src/supabase/routes.rs:440-467 maps many DB errors to 401/403/502 only.', 'domain': 'Agent lifecycle & status transitions', 'evidence_ref': 'original_evidence + current_rewrite_evidence', 'failing_test': 'backend-rs/tests/agent_lifecycle_parity.rs::agent_lifecycle_status_transitions_company_auth_boundary_for_lifecycle', 'final_status': 'unclosed', 'initial_status': 'partial', 'original_behavior': 'Rewrite has meaningful team-level RLS and forge-proof key RPCs, but lifecycle parity requires the original board-versus-agent route semantics for each transition. Because most transitions are missing, auth parity exists only for create/list/hire/approval/job slices. Rewrite errors are also coarse compared with original 409/422 state conflicts.', 'original_evidence': '/home/frankji/Projects/paperclip/server/src/routes/agents.ts:674-684 requires board and agents:create for board management; /home/frankji/Projects/paperclip/server/src/routes/agents.ts:3279-3286 allows agent self-invoke but board management otherwise; /home/frankji/Projects/paperclip/server/src/routes/agents.ts:2948, 2974, 3005, 3037, 3091, and 3567 call assertBoard for privileged transitions.', 'passing_test': 'cd backend-rs && cargo test --test agent_lifecycle_parity', 'phase': 'phases/07-agent-runtime-lifecycle.md', 'review_note': 'Rewrite has meaningful team-level RLS and forge-proof key RPCs, but lifecycle parity requires the original board-versus-agent route semantics for each transition. Because most transitions are missing, auth parity exists only for create/list/hire/approval/job slices. Rewrite errors are also coarse compared with original 409/422 state conflicts.', 'row_id': 'agent-lifecycle-status-transitions-company-auth-boundary-for-lifecycle', 'verdict': 'CONFIRMED', 'waiver_ref': ''}
{'acceptance_id': 'ACC-07-AGENT-RUNTIME-LIFECYCLE-0006', 'capability': 'Error state and clear-error', 'current_rewrite_evidence': '/home/frankji/Projects/paperclip/supabase/migrations/paperclip/0013_agents_hiring.sql:10-12 omits error from agent_status; /home/frankji/Projects/paperclip/backend-rs/src/supabase/routes.rs:36-62 registers no clear-error endpoint.', 'domain': 'Agent lifecycle & status transitions', 'evidence_ref': 'original_evidence + current_rewrite_evidence', 'failing_test': 'backend-rs/tests/agent_lifecycle_parity.rs::agent_lifecycle_status_transitions_error_state_and_clear_error', 'final_status': 'unclosed', 'initial_status': 'missing', 'original_behavior': 'Rewrite has no error agent status and no clear-error endpoint. Original failed/timed-out runs put the agent into error with a bounded errorReason, and clear-error is board-only, only valid from error, returns to idle, and logs agent.error_cleared.', 'original_evidence': '/home/frankji/Projects/paperclip/server/src/services/heartbeat.ts:7816-7854 stores errorReason and sets error on failed/timed_out; /home/frankji/Projects/paperclip/server/src/routes/agents.ts:3005-3035 implements POST /agents/:id/clear-error; /home/frankji/Projects/paperclip/server/src/services/agents.ts:592-620 enforces only error agents can be cleared and sets idle.', 'passing_test': 'cd backend-rs && cargo test --test agent_lifecycle_parity', 'phase': 'phases/07-agent-runtime-lifecycle.md', 'review_note': 'Rewrite has no error agent status and no clear-error endpoint. Original failed/timed-out runs put the agent into error with a bounded errorReason, and clear-error is board-only, only valid from error, returns to idle, and logs agent.error_cleared.', 'row_id': 'agent-lifecycle-status-transitions-error-state-and-clear-error', 'verdict': 'CONFIRMED', 'waiver_ref': ''}
{'acceptance_id': 'ACC-07-AGENT-RUNTIME-LIFECYCLE-0007', 'capability': 'Frontend/CLI/MCP lifecycle controls', 'current_rewrite_evidence': '/home/frankji/Projects/paperclip/backend-rs/src/bin/paperclip.rs:272-279 usage lists only login, company, job, agent hire/list, approval list/approve/reject; /home/frankji/Projects/paperclip/backend-rs/src/bin/paperclip-mcp.rs:28-56 lists MCP tools only for company/job/hire/list/approval; /home/frankji/Projects/paperclip/frontend-nuxt/components/PaperclipConsole.vue:53-82 renders hire/list/approval controls only.', 'domain': 'Agent lifecycle & status transitions', 'evidence_ref': 'original_evidence + current_rewrite_evidence', 'failing_test': 'backend-rs/tests/agent_lifecycle_parity.rs::agent_lifecycle_status_transitions_frontend_cli_mcp_lifecycle_controls', 'final_status': 'unclosed', 'initial_status': 'divergent', 'original_behavior': 'Rewrite user-facing surfaces only cover company/job/hire/list/approval. They do not expose original board quick actions for pause/resume/terminate/invoke/clear-error or runtime/run inspection. This is divergent from the original lifecycle operational model, not just a backend omission.', 'original_evidence': '/home/frankji/Projects/paperclip/server/src/routes/agents.ts:684-695 lists agent API contract endpoints including pause, resume, terminate, keys, and heartbeat invoke in the V1 spec route comments via implementation; concrete route implementations are in /home/frankji/Projects/paperclip/server/src/routes/agents.ts:2948-3419 and 3453-3589.', 'passing_test': 'cd backend-rs && cargo test --test agent_lifecycle_parity', 'phase': 'phases/07-agent-runtime-lifecycle.md', 'review_note': 'Rewrite user-facing surfaces only cover company/job/hire/list/approval. They do not expose original board quick actions for pause/resume/terminate/invoke/clear-error or runtime/run inspection. This is divergent from the original lifecycle operational model, not just a backend omission.', 'row_id': 'agent-lifecycle-status-transitions-frontend-cli-mcp-lifecycle-controls', 'verdict': 'CONFIRMED', 'waiver_ref': ''}
{'acceptance_id': 'ACC-07-AGENT-RUNTIME-LIFECYCLE-0008', 'capability': 'Heartbeat run, wakeup request, log/event, live-run, and cancellation model', 'current_rewrite_evidence': '/home/frankji/Projects/paperclip/supabase/migrations/paperclip/0011_companies_jobs.sql:21-45 defines only job_status running/succeeded/failed and jobs table; /home/frankji/Projects/paperclip/backend-rs/src/supabase/routes.rs:47-48 exposes only /jobs for execution.', 'domain': 'Agent lifecycle & status transitions', 'evidence_ref': 'original_evidence + current_rewrite_evidence', 'failing_test': 'backend-rs/tests/agent_lifecycle_parity.rs::agent_lifecycle_status_transitions_heartbeat_run_wakeup_request_log_ev', 'final_status': 'unclosed', 'initial_status': 'missing', 'original_behavior': 'Rewrite job records are not parity for heartbeat runs. Original has queued/running/succeeded/failed/cancelled/timed_out heartbeat runs, wakeup requests, live-run listing, run detail, cancel, events, logs, workspace operations, and issue active-run lookup. Rewrite has only company jobs with running/succeeded/failed and synchronous completion/failure.', 'original_evidence': '/home/frankji/Projects/paperclip/packages/shared/src/constants.ts:681-713 defines heartbeat invocation sources, wakeup request statuses, and heartbeat run statuses; /home/frankji/Projects/paperclip/server/src/routes/agents.ts:3453-3589 exposes heartbeat-runs, live-runs, run detail, and cancel; /home/frankji/Projects/paperclip/server/src/routes/agents.ts:3627-3667 exposes run events and logs.', 'passing_test': 'cd backend-rs && cargo test --test agent_lifecycle_parity', 'phase': 'phases/07-agent-runtime-lifecycle.md', 'review_note': 'Rewrite job records are not parity for heartbeat runs. Original has queued/running/succeeded/failed/cancelled/timed_out heartbeat runs, wakeup requests, live-run listing, run detail, cancel, events, logs, workspace operations, and issue active-run lookup. Rewrite has only company jobs with running/succeeded/failed and synchronous completion/failure.', 'row_id': 'agent-lifecycle-status-transitions-heartbeat-run-wakeup-request-log-ev', 'verdict': 'CONFIRMED', 'waiver_ref': ''}
{'acceptance_id': 'ACC-07-AGENT-RUNTIME-LIFECYCLE-0009', 'capability': 'New agent creation defaults and direct hire semantics', 'current_rewrite_evidence': '/home/frankji/Projects/paperclip/supabase/migrations/paperclip/0013_agents_hiring.sql:6-7 adds require_board_approval_for_new_agents default true; /home/frankji/Projects/paperclip/supabase/migrations/paperclip/0013_agents_hiring.sql:103-112 and 141-150 choose pending_approval or active; /home/frankji/Projects/paperclip/backend-rs/src/supabase/routes.rs:51-52 maps POST /companies/:companyId/agents directly to hire_paperclip_agent.', 'domain': 'Agent lifecycle & status transitions', 'evidence_ref': 'original_evidence + current_rewrite_evidence', 'failing_test': 'backend-rs/tests/agent_lifecycle_parity.rs::agent_lifecycle_status_transitions_new_agent_creation_defaults_and_dir', 'final_status': 'unclosed', 'initial_status': 'divergent', 'original_behavior': 'Original creates runnable direct agents as `idle` and pending hires as `pending_approval`; rewrite creates approved/non-gated agents as `active`, which is not the original runnable runtime baseline. Rewrite also defaults company hire approval to true, whereas original reads a company flag and has separate direct-create versus pending-hire routes.', 'original_evidence': '/home/frankji/Projects/paperclip/server/src/routes/agents.ts:2256-2263 sets agent-hires to pending_approval or idle; /home/frankji/Projects/paperclip/server/src/routes/agents.ts:2374-2391 blocks direct create when approval is required; /home/frankji/Projects/paperclip/server/src/routes/agents.ts:2439-2447 direct creation persists status idle.', 'passing_test': 'cd backend-rs && cargo test --test agent_lifecycle_parity', 'phase': 'phases/07-agent-runtime-lifecycle.md', 'review_note': 'Original creates runnable direct agents as `idle` and pending hires as `pending_approval`; rewrite creates approved/non-gated agents as `active`, which is not the original runnable runtime baseline. Rewrite also defaults company hire approval to true, whereas original reads a company flag and has separate direct-create versus pending-hire routes.', 'row_id': 'agent-lifecycle-status-transitions-new-agent-creation-defaults-and-dir', 'verdict': 'CONFIRMED', 'waiver_ref': ''}
{'acceptance_id': 'ACC-07-AGENT-RUNTIME-LIFECYCLE-0010', 'capability': 'Pause agent', 'current_rewrite_evidence': '/home/frankji/Projects/paperclip/backend-rs/src/supabase/routes.rs:36-62 registers no pause endpoint; /home/frankji/Projects/paperclip/backend-rs/src/bin/paperclip.rs:272-279 CLI usage lists no pause command; /home/frankji/Projects/paperclip/backend-rs/src/bin/paperclip-mcp.rs:28-56 lists no pause tool.', 'domain': 'Agent lifecycle & status transitions', 'evidence_ref': 'original_evidence + current_rewrite_evidence', 'failing_test': 'backend-rs/tests/agent_lifecycle_parity.rs::agent_lifecycle_status_transitions_pause_agent', 'final_status': 'unclosed', 'initial_status': 'missing', 'original_behavior': 'No rewrite route, CLI/MCP tool, frontend action, or RPC pauses an agent. Original pause is board-only, sets pause metadata, clears error reason, cancels active runs, and logs activity.', 'original_evidence': '/home/frankji/Projects/paperclip/server/src/routes/agents.ts:2948-2972 implements POST /agents/:id/pause with assertBoard, heartbeat.cancelActiveForAgent, and agent.paused activity; /home/frankji/Projects/paperclip/server/src/services/agents.ts:549-567 sets status paused, pauseReason, pausedAt, clears errorReason.', 'passing_test': 'cd backend-rs && cargo test --test agent_lifecycle_parity', 'phase': 'phases/07-agent-runtime-lifecycle.md', 'review_note': 'No rewrite route, CLI/MCP tool, frontend action, or RPC pauses an agent. Original pause is board-only, sets pause metadata, clears error reason, cancels active runs, and logs activity.', 'row_id': 'agent-lifecycle-status-transitions-pause-agent', 'verdict': 'CONFIRMED', 'waiver_ref': ''}
nonfull missing closure fields 0
[]

hook: PostToolUse
hook: PostToolUse Completed
hook: PreToolUse
hook: PreToolUse Completed
exec
/bin/bash -lc 'python3 docs/plans/2026-07-05-full-paperclip-parity/acceptance/run_original_gap_diff.py; true' in /home/frankji/Projects/paperclip
 succeeded in 0ms:
gaps_missing=100 gaps_partial=50 gaps_divergent=22 rows_total=175 rows_open=172
OPEN_ROWS_SAMPLE:
- agent-lifecycle-status-transitions-agent-key-permission-to-request-hir [partial] phases/07-agent-runtime-lifecycle.md :: Agent-key permission to request hires
- agent-lifecycle-status-transitions-approval-gated-hiring-flow [partial] phases/07-agent-runtime-lifecycle.md :: Approval-gated hiring flow
- agent-lifecycle-status-transitions-approving-pending-hires [divergent] phases/07-agent-runtime-lifecycle.md :: Approving pending hires
- agent-lifecycle-status-transitions-canonical-agent-status-vocabulary [divergent] phases/07-agent-runtime-lifecycle.md :: Canonical agent status vocabulary
- agent-lifecycle-status-transitions-company-auth-boundary-for-lifecycle [partial] phases/07-agent-runtime-lifecycle.md :: Company/auth boundary for lifecycle mutations
- agent-lifecycle-status-transitions-error-state-and-clear-error [missing] phases/07-agent-runtime-lifecycle.md :: Error state and clear-error
- agent-lifecycle-status-transitions-frontend-cli-mcp-lifecycle-controls [divergent] phases/07-agent-runtime-lifecycle.md :: Frontend/CLI/MCP lifecycle controls
- agent-lifecycle-status-transitions-heartbeat-run-wakeup-request-log-ev [missing] phases/07-agent-runtime-lifecycle.md :: Heartbeat run, wakeup request, log/event, live-run, and cancellation model
- agent-lifecycle-status-transitions-new-agent-creation-defaults-and-dir [divergent] phases/07-agent-runtime-lifecycle.md :: New agent creation defaults and direct hire semantics
- agent-lifecycle-status-transitions-pause-agent [missing] phases/07-agent-runtime-lifecycle.md :: Pause agent
- agent-lifecycle-status-transitions-rejecting-pending-hires-firing-pend [divergent] phases/07-agent-runtime-lifecycle.md :: Rejecting pending hires / firing pending hires
- agent-lifecycle-status-transitions-resume-agent [missing] phases/07-agent-runtime-lifecycle.md :: Resume agent
- agent-lifecycle-status-transitions-runtime-state-task-sessions-and-ses [missing] phases/07-agent-runtime-lifecycle.md :: Runtime state, task sessions, and session reset
- agent-lifecycle-status-transitions-runtime-transition-idle-running-idl [missing] phases/07-agent-runtime-lifecycle.md :: Runtime transition idle → running → idle/error
- agent-lifecycle-status-transitions-terminate-fire-active-agents [missing] phases/07-agent-runtime-lifecycle.md :: Terminate/fire active agents
- agent-lifecycle-status-transitions-wakeup-heartbeat-invoke [missing] phases/07-agent-runtime-lifecycle.md :: Wakeup / heartbeat invoke
- approvals-system-governance-access-boundary-read [partial] phases/06-approvals-budgets-costs-activity.md :: Company-scope read authorization boundary
- approvals-system-governance-approval-comments-table [missing] phases/06-approvals-budgets-costs-activity.md :: approval_comments table + comment endpoints
- approvals-system-governance-approval-status-enum [divergent] phases/06-approvals-budgets-costs-activity.md :: Approval status lifecycle enum (5 statuses)
- approvals-system-governance-approval-types-enum [partial] phases/06-approvals-budgets-costs-activity.md :: Approval type enum (4 governed types)

hook: PostToolUse
hook: PostToolUse Completed
exec
/bin/bash -lc 'python3 -m py_compile docs/plans/2026-07-05-full-paperclip-parity/acceptance/run_original_gap_diff.py docs/plans/2026-07-05-full-paperclip-parity/acceptance/run_real_user_real_agent.py' in /home/frankji/Projects/paperclip
 succeeded in 0ms:
hook: PostToolUse
hook: PostToolUse Completed
codex
The machine-readable matrix has 175 rows with the corrected counts, and every non-full row has owner, acceptance ID, failing test, passing command, evidence ref, and `final_status`. One issue is emerging in the gap-diff executable: it appends extra fields even though the plan now requires an exact `gaps_missing=0 gaps_partial=0 gaps_divergent=0` final output.
hook: PreToolUse
hook: PreToolUse Completed
hook: PreToolUse
hook: PreToolUse
hook: PreToolUse Completed
hook: PreToolUse Completed
exec
/bin/bash -lc "python3 - <<'PY'
import json, re, pathlib, collections
root=pathlib.Path('docs/plans/2026-07-05-full-paperclip-parity')
rows=[json.loads(l) for l in (root/'evidence/rewrite-gap-matrix.jsonl').read_text().splitlines() if l.strip()]
phase_files=sorted(p.name for p in (root/'phases').glob('*.md'))
print('phase_files', len(phase_files), phase_files)
print('row phases', collections.Counter(r['phase'] for r in rows))
ids=[r['row_id'] for r in rows]
print('duplicate row ids', [i for i,c in collections.Counter(ids).items() if c>1][:20])
acc=[r['acceptance_id'] for r in rows if r.get('acceptance_id')]
print('duplicate acceptance ids', [i for i,c in collections.Counter(acc).items() if c>1][:20])
invalid_phase=[r['row_id'] for r in rows if r['phase'].split('/')[-1] not in phase_files]
print('invalid phase refs', len(invalid_phase), invalid_phase[:10])
md=(root/'evidence/rewrite-gap-matrix.md').read_text()
md_rows=re.findall(r'"'^'"\\| "'`([''^`]+)` '"\\| "'`([''^`]+)` '"\\| "'`([''^`]+)` '"\\| "'`([''^`]+)` '"\\| "'`([''^`]+)` '"\\|', md, re.M)
print('md data rows', len(md_rows))
print('json rows absent from md', len(set(ids)-{r[0] for r in md_rows}))
print('md rows absent from json', len({r[0] for r in md_rows}-set(ids)))
PY" in /home/frankji/Projects/paperclip
 succeeded in 0ms:
phase_files 14 ['00-parity-oracle-and-matrix.md', '01-schema-authz-parity.md', '02-goals-projects-issues-work-products.md', '03-workspaces-routines-plugins-sandbox.md', '04-deployment-auth-onboarding-ops.md', '05-adapters-secrets-skills-config.md', '06-approvals-budgets-costs-activity.md', '07-agent-runtime-lifecycle.md', '08-interfaces-rest-cli-mcp.md', '09-nuxt-board-ui-parity.md', '10-import-export-catalogs-portability.md', '11-observability-feedback-pipelines.md', '12-shared-contract-docs-parity.md', '13-migration-compatibility-cutover.md']
row phases Counter({'phases/01-schema-authz-parity.md': 55, 'phases/09-nuxt-board-ui-parity.md': 40, 'phases/08-interfaces-rest-cli-mcp.md': 31, 'phases/06-approvals-budgets-costs-activity.md': 21, 'phases/07-agent-runtime-lifecycle.md': 16, 'phases/05-adapters-secrets-skills-config.md': 12})
duplicate row ids []
duplicate acceptance ids []
invalid phase refs 0 []
md data rows 175
json rows absent from md 0
md rows absent from json 0

hook: PostToolUse
hook: PostToolUse Completed
exec
/bin/bash -lc 'python3 docs/plans/2026-07-05-full-paperclip-parity/acceptance/run_real_user_real_agent.py; true' in /home/frankji/Projects/paperclip
 succeeded in 0ms:
NOT_IMPLEMENTED: live real-user/real-agent harness must be implemented in Phase 00/09 before release.

hook: PostToolUse
hook: PostToolUse Completed
exec
/bin/bash -lc 'rg -n "gaps_missing|gaps_partial|gaps_divergent|rows_total|rows_open|NOT_IMPLEMENTED|placeholder|service-role|Responses API|public" docs/plans/2026-07-05-full-paperclip-parity/README.md docs/plans/2026-07-05-full-paperclip-parity/01-parity-audit-summary.md docs/plans/2026-07-05-full-paperclip-parity/04-execution-order.md docs/plans/2026-07-05-full-paperclip-parity/05-test-strategy.md docs/plans/2026-07-05-full-paperclip-parity/acceptance/full-parity-harness-spec.md docs/plans/2026-07-05-full-paperclip-parity/acceptance/run_original_gap_diff.py docs/plans/2026-07-05-full-paperclip-parity/acceptance/run_real_user_real_agent.py docs/plans/2026-07-05-full-paperclip-parity/codex/phase-prompts.md docs/plans/2026-07-05-full-paperclip-parity/reviews/review-checklist.md docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.md docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.jsonl' in /home/frankji/Projects/paperclip
 succeeded in 0ms:
docs/plans/2026-07-05-full-paperclip-parity/05-test-strategy.md:75:The final gap diff must print exactly: `gaps_missing=0 gaps_partial=0 gaps_divergent=0`.
docs/plans/2026-07-05-full-paperclip-parity/04-execution-order.md:35:gaps_missing=0 gaps_partial=0 gaps_divergent=0
docs/plans/2026-07-05-full-paperclip-parity/README.md:31:- Use only Supabase schemas `paperclip` and `paperclip_private`; **do not use `public`** for application tables/functions.
docs/plans/2026-07-05-full-paperclip-parity/README.md:32:- Do not use a Supabase service-role key in app, CLI, MCP, frontend, or test acceptance flows.
docs/plans/2026-07-05-full-paperclip-parity/README.md:35:- Use OpenAI **Responses API**, not chat completions, for OpenAI-backed adapter/job paths.
docs/plans/2026-07-05-full-paperclip-parity/acceptance/run_real_user_real_agent.py:2:"""Live real-user/real-agent acceptance harness placeholder.
docs/plans/2026-07-05-full-paperclip-parity/acceptance/run_real_user_real_agent.py:9:print("NOT_IMPLEMENTED: live real-user/real-agent harness must be implemented in Phase 00/09 before release.")
docs/plans/2026-07-05-full-paperclip-parity/reviews/review-checklist.md:11:- No Supabase service-role dependency.
docs/plans/2026-07-05-full-paperclip-parity/reviews/review-checklist.md:12:- No Paperclip application objects in `public` schema.
docs/plans/2026-07-05-full-paperclip-parity/reviews/review-checklist.md:26:gaps_missing=0 gaps_partial=0 gaps_divergent=0
docs/plans/2026-07-05-full-paperclip-parity/codex/phase-prompts.md:12:Before implementing, read PHASE_FILE plus `01-parity-audit-summary.md`, `04-execution-order.md`, `05-test-strategy.md`, `acceptance/full-parity-harness-spec.md`, `evidence/rewrite-gap-matrix.md`, and `reviews/review-checklist.md`. Implement only the rows owned by PHASE_FILE using strict TDD. First write failing tests that demonstrate the current rewrite does not match original Paperclip behavior. Then implement the minimal code/migrations to pass. Preserve: no public schema, no service-role key, no Supabase/OpenAI details client-side, OpenAI Responses API only. Update each owned matrix row with phase, acceptance_id, failing_test, passing_test, evidence_ref, and final_status. Run the phase checks and report exact output.
docs/plans/2026-07-05-full-paperclip-parity/codex/phase-prompts.md:18:Adversarially compare the final rewrite against original Paperclip V1 for agent hiring/governance/runtime/interfaces. Use docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.md as the checklist. Confirm gaps_missing=0 gaps_partial=0 gaps_divergent=0 or list blockers. Do not accept a vertical slice as full parity.
docs/plans/2026-07-05-full-paperclip-parity/acceptance/full-parity-harness-spec.md:13:1. Schema introspection: no app objects in `public`; expected tables/functions/enums in `paperclip`; helpers in `paperclip_private`.
docs/plans/2026-07-05-full-paperclip-parity/acceptance/full-parity-harness-spec.md:28:schema=pass authz=pass lifecycle=pass approvals=pass runtime=pass budgets=pass secrets=pass tasks=pass interfaces=pass live_openai=pass gaps_missing=0 gaps_partial=0 gaps_divergent=0
docs/plans/2026-07-05-full-paperclip-parity/acceptance/full-parity-harness-spec.md:50:gaps_missing=0 gaps_partial=0 gaps_divergent=0
docs/plans/2026-07-05-full-paperclip-parity/acceptance/run_original_gap_diff.py:32:    f"gaps_missing={counts['missing']} "
docs/plans/2026-07-05-full-paperclip-parity/acceptance/run_original_gap_diff.py:33:    f"gaps_partial={counts['partial']} "
docs/plans/2026-07-05-full-paperclip-parity/acceptance/run_original_gap_diff.py:34:    f"gaps_divergent={counts['divergent']} "
docs/plans/2026-07-05-full-paperclip-parity/acceptance/run_original_gap_diff.py:35:    f"rows_total={len(rows)} rows_open={len(open_rows)}"

hook: PostToolUse
hook: PostToolUse Completed
codex
I’ve confirmed the phase filenames are 14 and the matrix has no duplicate row IDs or acceptance IDs. The remaining review question is whether the named acceptance scripts are acceptable as plan gates after patches; one script is still a deliberate placeholder, and the gap script’s output contract is internally inconsistent.
hook: PreToolUse
hook: PreToolUse Completed
exec
/bin/bash -lc "nl -ba docs/plans/2026-07-05-full-paperclip-parity/acceptance/run_original_gap_diff.py | sed -n '1,80p'" in /home/frankji/Projects/paperclip
 succeeded in 0ms:
     1	#!/usr/bin/env python3
     2	"""Parity gap ledger gate.
     3	
     4	Fails until every non-full row in evidence/rewrite-gap-matrix.jsonl is closed as
     5	final_status=full or final_status=waived with a waiver_ref.
     6	"""
     7	from __future__ import annotations
     8	import json
     9	from pathlib import Path
    10	import sys
    11	
    12	ROOT = Path(__file__).resolve().parents[1]
    13	MATRIX = ROOT / "evidence" / "rewrite-gap-matrix.jsonl"
    14	rows = [json.loads(line) for line in MATRIX.read_text().splitlines() if line.strip()]
    15	open_rows = []
    16	for row in rows:
    17	    status = row.get("initial_status")
    18	    final = row.get("final_status")
    19	    if status == "full" and final == "full":
    20	        continue
    21	    if final == "full":
    22	        continue
    23	    if final == "waived" and row.get("waiver_ref"):
    24	        continue
    25	    open_rows.append(row)
    26	counts = {"missing": 0, "partial": 0, "divergent": 0}
    27	for row in open_rows:
    28	    status = row.get("initial_status")
    29	    if status in counts:
    30	        counts[status] += 1
    31	print(
    32	    f"gaps_missing={counts['missing']} "
    33	    f"gaps_partial={counts['partial']} "
    34	    f"gaps_divergent={counts['divergent']} "
    35	    f"rows_total={len(rows)} rows_open={len(open_rows)}"
    36	)
    37	if any(counts.values()):
    38	    print("OPEN_ROWS_SAMPLE:")
    39	    for row in open_rows[:20]:
    40	        print(f"- {row['row_id']} [{row['initial_status']}] {row['phase']} :: {row['capability']}")
    41	    sys.exit(1)
    42	sys.exit(0)

hook: PostToolUse
hook: PostToolUse Completed
hook: PreToolUse
hook: PreToolUse Completed
hook: PreToolUse
hook: PreToolUse
hook: PreToolUse Completed
hook: PreToolUse Completed
exec
/bin/bash -lc "nl -ba docs/plans/2026-07-05-full-paperclip-parity/acceptance/run_real_user_real_agent.py | sed -n '1,80p'" in /home/frankji/Projects/paperclip
 succeeded in 0ms:
     1	#!/usr/bin/env python3
     2	"""Live real-user/real-agent acceptance harness placeholder.
     3	
     4	Phase 00 owns turning this scaffold into the live Supabase/OpenAI harness. It
     5	intentionally fails until implemented so the plan cannot claim live parity from
     6	missing automation.
     7	"""
     8	import sys
     9	print("NOT_IMPLEMENTED: live real-user/real-agent harness must be implemented in Phase 00/09 before release.")
    10	sys.exit(2)

hook: PreToolUse
hook: PostToolUse
hook: PreToolUse Completed
hook: PostToolUse Completed
exec
/bin/bash -lc "nl -ba docs/plans/2026-07-05-full-paperclip-parity/acceptance/full-parity-harness-spec.md | sed -n '24,55p'" in /home/frankji/Projects/paperclip
 succeeded in 0ms:
    24	
    25	## Final pass condition
    26	
    27	```text
    28	schema=pass authz=pass lifecycle=pass approvals=pass runtime=pass budgets=pass secrets=pass tasks=pass interfaces=pass live_openai=pass gaps_missing=0 gaps_partial=0 gaps_divergent=0
    29	```
    30	
    31	
    32	## Row-level matrix proof
    33	
    34	The harness must read `evidence/rewrite-gap-matrix.md` (or its generated machine-readable companion) and assert every non-full audit row has:
    35	
    36	- `phase`
    37	- `acceptance_id`
    38	- `failing_test`
    39	- `passing_test`
    40	- `evidence_ref`
    41	- `final_status`
    42	
    43	Allowed final statuses are only `full` or `waived`. Waived rows require a canonical product-contract reference and reviewer signoff. Category-level checks cannot override a row-level non-full status.
    44	
    45	## Exact final output
    46	
    47	The final line must include the exact fields:
    48	
    49	```text
    50	gaps_missing=0 gaps_partial=0 gaps_divergent=0
    51	```
    52	
    53	Any spelling variant, omitted prefix, or category-only summary fails the harness.

hook: PostToolUse
hook: PostToolUse Completed
exec
/bin/bash -lc "nl -ba docs/plans/2026-07-05-full-paperclip-parity/05-test-strategy.md | sed -n '60,85p'" in /home/frankji/Projects/paperclip
 succeeded in 0ms:
    60	
    61	Frontend:
    62	
    63	```sh
    64	cd frontend-nuxt
    65	npx vitest run
    66	```
    67	
    68	Acceptance:
    69	
    70	```sh
    71	python3 docs/plans/2026-07-05-full-paperclip-parity/acceptance/run_original_gap_diff.py
    72	python3 docs/plans/2026-07-05-full-paperclip-parity/acceptance/run_real_user_real_agent.py
    73	```
    74	
    75	The final gap diff must print exactly: `gaps_missing=0 gaps_partial=0 gaps_divergent=0`.
    76	
    77	
    78	## Row-level anti-false-parity gate
    79	
    80	Category checks are not enough. Every non-full audit row from the corrected baseline (`3 full / 50 partial / 100 missing / 22 divergent`) must carry:
    81	
    82	```text
    83	phase=<owner> acceptance_id=<id> failing_test=<path::name> passing_test=<command> evidence_ref=<source+rewrite> final_status=<full|waived>
    84	```
    85	

hook: PostToolUse
hook: PostToolUse Completed
exec
/bin/bash -lc "nl -ba docs/plans/2026-07-05-full-paperclip-parity/04-execution-order.md | sed -n '25,55p'" in /home/frankji/Projects/paperclip
 succeeded in 0ms:
    25	- Phase 01 blocks all domain phases. The wrong status/default/authz contracts make later work invalid.
    26	- Domain backend phases must land before their UI/CLI/MCP parity rows.
    27	- Live Supabase/OpenAI acceptance is final proof, not a substitute for deterministic local red/green tests.
    28	- A waiver for any V1-core row requires a product-spec update and reviewer signoff.
    29	
    30	## Stop condition
    31	
    32	The plan is complete only when `acceptance/run_original_gap_diff.py` reports:
    33	
    34	```text
    35	gaps_missing=0 gaps_partial=0 gaps_divergent=0
    36	```
    37	
    38	or every non-full row is explicitly waived by an updated product contract.
    39	
    40	
    41	## Row-level closure rule
    42	
    43	Each phase owns explicit rows in `evidence/rewrite-gap-matrix.md`. A phase cannot close from a category-level pass. Every row must end with:
    44	
    45	```text
    46	phase=<owner> acceptance_id=<id> failing_test=<path::name> passing_test=<command> evidence_ref=<source+rewrite> final_status=<full|waived>
    47	```
    48	
    49	The final cutover phase must fail if any blocking row remains `partial`, `missing`, `divergent`, or lacks a reviewed waiver.

hook: PostToolUse
hook: PostToolUse Completed
codex
I found a second structural issue: the plan now has 14 phase files, but the authoritative row-level matrix assigns all 175 rows to only six phase owners. Several newly added V1 parity areas therefore exist as phase filenames/order only, without row-level closure entries in the matrix.
hook: PreToolUse
hook: PreToolUse Completed
exec
/bin/bash -lc "nl -ba docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.md | sed -n '1,35p'" in /home/frankji/Projects/paperclip
 succeeded in 0ms:
     1	# Rewrite Gap Matrix
     2	
     3	This is the row-level closure ledger generated from the adversarial parity audit journals. It is intentionally long: workers must close rows, not categories.
     4	
     5	## Corrected audit tally
     6	
     7	- full: 3
     8	- partial: 50
     9	- missing: 100
    10	- divergent: 22
    11	
    12	Machine-readable companion: [`rewrite-gap-matrix.jsonl`](rewrite-gap-matrix.jsonl).
    13	
    14	## Phase ownership summary
    15	
    16	| Phase | Full | Partial | Missing | Divergent | Total |
    17	|---|---:|---:|---:|---:|---:|
    18	| `phases/01-schema-authz-parity.md` | 1 | 20 | 24 | 10 | 55 |
    19	| `phases/05-adapters-secrets-skills-config.md` | 0 | 2 | 9 | 1 | 12 |
    20	| `phases/06-approvals-budgets-costs-activity.md` | 1 | 8 | 10 | 2 | 21 |
    21	| `phases/07-agent-runtime-lifecycle.md` | 0 | 3 | 8 | 5 | 16 |
    22	| `phases/08-interfaces-rest-cli-mcp.md` | 0 | 12 | 16 | 3 | 31 |
    23	| `phases/09-nuxt-board-ui-parity.md` | 1 | 5 | 33 | 1 | 40 |
    24	
    25	## Row-level matrix
    26	
    27	| Row | Status | Phase | Acceptance | Test | Capability |
    28	|---|---|---|---|---|---|
    29	| `agent-lifecycle-status-transitions-agent-key-permission-to-request-hir` | `partial` | `phases/07-agent-runtime-lifecycle.md` | `ACC-07-AGENT-RUNTIME-LIFECYCLE-0001` | `backend-rs/tests/agent_lifecycle_parity.rs::agent_lifecycle_status_transitions_agent_key_permission_to_request_hir` | Agent-key permission to request hires |
    30	| `agent-lifecycle-status-transitions-approval-gated-hiring-flow` | `partial` | `phases/07-agent-runtime-lifecycle.md` | `ACC-07-AGENT-RUNTIME-LIFECYCLE-0002` | `backend-rs/tests/agent_lifecycle_parity.rs::agent_lifecycle_status_transitions_approval_gated_hiring_flow` | Approval-gated hiring flow |
    31	| `agent-lifecycle-status-transitions-approving-pending-hires` | `divergent` | `phases/07-agent-runtime-lifecycle.md` | `ACC-07-AGENT-RUNTIME-LIFECYCLE-0003` | `backend-rs/tests/agent_lifecycle_parity.rs::agent_lifecycle_status_transitions_approving_pending_hires` | Approving pending hires |
    32	| `agent-lifecycle-status-transitions-canonical-agent-status-vocabulary` | `divergent` | `phases/07-agent-runtime-lifecycle.md` | `ACC-07-AGENT-RUNTIME-LIFECYCLE-0004` | `backend-rs/tests/agent_lifecycle_parity.rs::agent_lifecycle_status_transitions_canonical_agent_status_vocabulary` | Canonical agent status vocabulary |
    33	| `agent-lifecycle-status-transitions-company-auth-boundary-for-lifecycle` | `partial` | `phases/07-agent-runtime-lifecycle.md` | `ACC-07-AGENT-RUNTIME-LIFECYCLE-0005` | `backend-rs/tests/agent_lifecycle_parity.rs::agent_lifecycle_status_transitions_company_auth_boundary_for_lifecycle` | Company/auth boundary for lifecycle mutations |
    34	| `agent-lifecycle-status-transitions-error-state-and-clear-error` | `missing` | `phases/07-agent-runtime-lifecycle.md` | `ACC-07-AGENT-RUNTIME-LIFECYCLE-0006` | `backend-rs/tests/agent_lifecycle_parity.rs::agent_lifecycle_status_transitions_error_state_and_clear_error` | Error state and clear-error |
    35	| `agent-lifecycle-status-transitions-frontend-cli-mcp-lifecycle-controls` | `divergent` | `phases/07-agent-runtime-lifecycle.md` | `ACC-07-AGENT-RUNTIME-LIFECYCLE-0007` | `backend-rs/tests/agent_lifecycle_parity.rs::agent_lifecycle_status_transitions_frontend_cli_mcp_lifecycle_controls` | Frontend/CLI/MCP lifecycle controls |

hook: PostToolUse
hook: PostToolUse Completed
hook: PreToolUse
hook: PreToolUse Completed
exec
/bin/bash -lc "nl -ba docs/plans/2026-07-05-full-paperclip-parity/04-execution-order.md | sed -n '1,24p'" in /home/frankji/Projects/paperclip
 succeeded in 0ms:
     1	# Execution Order
     2	
     3	The first draft had seven phases. Plan-design review found that was still too narrow for full parity. The locked plan uses fourteen phases so every original Paperclip control-plane area has an owner.
     4	
     5	## Phase order
     6	
     7	0. [`00-parity-oracle-and-matrix.md`](phases/00-parity-oracle-and-matrix.md) — build the failing parity oracle and row-level matrix.
     8	1. [`01-schema-authz-parity.md`](phases/01-schema-authz-parity.md) — schema/default/enums/authorization/permission grants.
     9	2. [`02-goals-projects-issues-work-products.md`](phases/02-goals-projects-issues-work-products.md) — company mission, goals, projects, issues, comments, documents, artifacts.
    10	3. [`04-deployment-auth-onboarding-ops.md`](phases/04-deployment-auth-onboarding-ops.md) — local/authenticated modes, onboarding, bootstrap, ops safety.
    11	4. [`05-adapters-secrets-skills-config.md`](phases/05-adapters-secrets-skills-config.md) — adapter config, secrets, skills, config revisions.
    12	5. [`06-approvals-budgets-costs-activity.md`](phases/06-approvals-budgets-costs-activity.md) — approval lifecycle, budgets, cost rollups, activity logs.
    13	6. [`07-agent-runtime-lifecycle.md`](phases/07-agent-runtime-lifecycle.md) — canonical statuses, heartbeat runtime, wakeups, cancellation, logs/events.
    14	7. [`03-workspaces-routines-plugins-sandbox.md`](phases/03-workspaces-routines-plugins-sandbox.md) — runtime workspaces, routines, plugin sandbox/current-addendum surfaces.
    15	8. [`08-interfaces-rest-cli-mcp.md`](phases/08-interfaces-rest-cli-mcp.md) — REST, CLI, MCP parity after domain APIs exist.
    16	9. [`09-nuxt-board-ui-parity.md`](phases/09-nuxt-board-ui-parity.md) — human board UI parity.
    17	10. [`10-import-export-catalogs-portability.md`](phases/10-import-export-catalogs-portability.md) — catalogs, templates, export/import.
    18	11. [`11-observability-feedback-pipelines.md`](phases/11-observability-feedback-pipelines.md) — observability/productivity/pipeline current-addendum classification and implementation.
    19	12. [`12-shared-contract-docs-parity.md`](phases/12-shared-contract-docs-parity.md) — shared constants/validators/docs/OpenAPI contract sync.
    20	13. [`13-migration-compatibility-cutover.md`](phases/13-migration-compatibility-cutover.md) — compatibility, cutover, rollback.
    21	
    22	## Dependency rules
    23	
    24	- Phase 00 blocks all implementation. No worker can close a gap that is not represented in the matrix.

hook: PostToolUse
hook: PostToolUse Completed
hook: PreToolUse
hook: PreToolUse Completed
exec
/bin/bash -lc "nl -ba docs/plans/2026-07-05-full-paperclip-parity/README.md | sed -n '45,62p'" in /home/frankji/Projects/paperclip
 succeeded in 0ms:
hook: PostToolUse
hook: PostToolUse Completed
hook: PreToolUse
hook: PreToolUse Completed
exec
/bin/bash -lc "nl -ba docs/plans/2026-07-05-full-paperclip-parity/README.md | sed -n '45,70p'" in /home/frankji/Projects/paperclip
 succeeded in 0ms:
hook: PostToolUse
hook: PostToolUse Completed
codex
**BLOCKERS**

1. [run_original_gap_diff.py](/home/frankji/Projects/paperclip/docs/plans/2026-07-05-full-paperclip-parity/acceptance/run_original_gap_diff.py:31) violates the exact final output contract. The plan requires exactly `gaps_missing=0 gaps_partial=0 gaps_divergent=0` in [05-test-strategy.md](/home/frankji/Projects/paperclip/docs/plans/2026-07-05-full-paperclip-parity/05-test-strategy.md:75), but the script appends `rows_total` and `rows_open`.

2. [run_real_user_real_agent.py](/home/frankji/Projects/paperclip/docs/plans/2026-07-05-full-paperclip-parity/acceptance/run_real_user_real_agent.py:2) is still only a failing placeholder. That is not an executable real-user/real-agent/OpenAI acceptance harness yet.

3. The matrix does not actually assign row-level closure to all 14 phases. [04-execution-order.md](/home/frankji/Projects/paperclip/docs/plans/2026-07-05-full-paperclip-parity/04-execution-order.md:3) says every control-plane area has an owner, but [rewrite-gap-matrix.md](/home/frankji/Projects/paperclip/docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.md:18) assigns all 175 rows to only phases 01, 05, 06, 07, 08, and 09. Phases like goals/projects/issues, deployment/onboarding, workspaces/routines/plugins, import/export, observability, docs/contracts, and cutover have no row-level gap ownership.

**PATCHES**

No edits made. Required patches:

- Make `run_original_gap_diff.py` print the exact required final line when passing.
- Replace `run_real_user_real_agent.py` placeholder with the real live harness or make Phase 00 explicitly block until it exists.
- Add row-level matrix entries or explicit waiver rows for the V1 areas represented by the currently unowned phases.

**VERDICT**

FAIL
hook: Stop
hook: Stop Completed
tokens used
64,540
**BLOCKERS**

1. [run_original_gap_diff.py](/home/frankji/Projects/paperclip/docs/plans/2026-07-05-full-paperclip-parity/acceptance/run_original_gap_diff.py:31) violates the exact final output contract. The plan requires exactly `gaps_missing=0 gaps_partial=0 gaps_divergent=0` in [05-test-strategy.md](/home/frankji/Projects/paperclip/docs/plans/2026-07-05-full-paperclip-parity/05-test-strategy.md:75), but the script appends `rows_total` and `rows_open`.

2. [run_real_user_real_agent.py](/home/frankji/Projects/paperclip/docs/plans/2026-07-05-full-paperclip-parity/acceptance/run_real_user_real_agent.py:2) is still only a failing placeholder. That is not an executable real-user/real-agent/OpenAI acceptance harness yet.

3. The matrix does not actually assign row-level closure to all 14 phases. [04-execution-order.md](/home/frankji/Projects/paperclip/docs/plans/2026-07-05-full-paperclip-parity/04-execution-order.md:3) says every control-plane area has an owner, but [rewrite-gap-matrix.md](/home/frankji/Projects/paperclip/docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.md:18) assigns all 175 rows to only phases 01, 05, 06, 07, 08, and 09. Phases like goals/projects/issues, deployment/onboarding, workspaces/routines/plugins, import/export, observability, docs/contracts, and cutover have no row-level gap ownership.

**PATCHES**

No edits made. Required patches:

- Make `run_original_gap_diff.py` print the exact required final line when passing.
- Replace `run_real_user_real_agent.py` placeholder with the real live harness or make Phase 00 explicitly block until it exists.
- Add row-level matrix entries or explicit waiver rows for the V1 areas represented by the currently unowned phases.

**VERDICT**

FAIL
