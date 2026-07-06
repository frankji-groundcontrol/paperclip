Reading additional input from stdin...
2026-07-05T17:07:06.434493Z ERROR codex_models_manager::manager: failed to refresh available models: stream disconnected before completion: failed to decode models response: expected value at line 1 column 1; body: <!doctype html>
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
  <script nonce="2pNvwo+c8N+QqSpam6TBKw==">window.__APP_CONFIG__={"registration_enabled":false,"email_verify_enabled":false,"registration_email_suffix_whitelist":[],"promo_code_enabled":true,"password_reset_enabled":false,"invitation_code_enabled":false,"totp_enabled":false,"login_agreement_enabled":false,"login_agreement_mode":"modal","login_agreement_updated_at":"2026-03-31","login_agreement_revision":"0eff189d66e0af4e","login_agreement_documents":[{"id":"terms","title":"服务条款","content_md":""},{"id":"usage-policy","title":"使用政策","content_md":""},{"id":"supported-regions","title":"支持的国家和地区","content_md":""},{"id":"service-specific-terms","title":"服务特定条款","content_md":""}],"turnstile_enabled":false,"turnstile_site_key":"","site_name":"Sub2API","site_logo":"","site_subtitle":"Subscription to API Conversion Platform","api_base_url":"","contact_info":"","doc_url":"","home_content":"","hide_ccs_import_button":false,"purchase_subscription_enabled":false,"purchase_subscription_url":"","table_default_page_size":20,"table_page_size_options":[10,20,50],"custom_menu_items":[],"custom_endpoints":[],"linuxdo_oauth_enabled":false,"dingtalk_oauth_enabled":false,"wechat_oauth_enabled":false,"wechat_oauth_open_enabled":false,"wechat_oauth_mp_enabled":false,"wechat_oauth_mobile_enabled":false,"oidc_oauth_enabled":false,"oidc_oauth_provider_name":"OIDC","github_oauth_enabled":false,"google_oauth_enabled":false,"backend_mode_enabled":false,"payment_enabled":false,"version":"","server_timezone":"Asia/Shanghai","server_utc_offset":"+08:00","balance_low_notify_enabled":false,"account_quota_notify_enabled":false,"balance_low_notify_threshold":0,"balance_low_notify_recharge_url":"","channel_monitor_enabled":true,"channel_monitor_default_interval_seconds":60,"available_channels_enabled":false,"affiliate_enabled":false,"risk_control_enabled":false,"allow_user_view_error_requests":false};</script></head>
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
session id: 019f333f-7c43-7862-a8c3-2619a1230b3d
--------
user
Re-review only these plan files after patches: docs/plans/2026-07-05-full-paperclip-parity/README.md, 04-execution-order.md, 05-test-strategy.md, acceptance/full-parity-harness-spec.md, codex/phase-prompts.md, reviews/review-checklist.md, evidence/rewrite-gap-matrix.md, and phase filenames under phases/. Do not edit. Goal: plan must drive rewrite to zero known Paperclip V1 parity gaps from audit baseline 3 full, 50 partial, 100 missing, 22 divergent. Check whether prior blockers are fixed: phase count, exact gaps_* output, row-level matrix closure, Codex implementer context, anti-false-parity gate. Return BLOCKERS, PATCHES, VERDICT. VERDICT must be PASS or FAIL.
2026-07-05T17:07:07.184282Z ERROR rmcp::transport::worker: worker quit with fatal: Transport channel closed, when Auth(AuthorizationRequired)
hook: SessionStart
hook: SessionStart Completed
hook: UserPromptSubmit
hook: UserPromptSubmit Completed
codex
I’ll re-review the specified plan files only and won’t edit anything. I’m going to read the repo guidance docs first as required, then inspect just the named plan paths and phase filenames for the requested blocker checks.
hook: PreToolUse
hook: PreToolUse Completed
hook: PreToolUse
hook: PreToolUse Completed
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
hook: PreToolUse
hook: PostToolUse Completed
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
exec
/bin/bash -lc "sed -n '1,220p' doc/DEVELOPING.md" in /home/frankji/Projects/paperclip
hook: PreToolUse
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
hook: PreToolUse Completed
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
codex
I’ve got the baseline docs context. Next I’m narrowing to the requested plan subtree and will check counts, harness command specificity, traceability from each audit row to closure, and whether the Codex prompts/gates prevent superficial parity claims.
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
docs/plans/2026-07-05-full-paperclip-parity/codex/README.md
docs/plans/2026-07-05-full-paperclip-parity/codex/phase-prompts.md
docs/plans/2026-07-05-full-paperclip-parity/evidence/migration-ledger.md
docs/plans/2026-07-05-full-paperclip-parity/evidence/original-source-map.md
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
docs/plans/2026-07-05-full-paperclip-parity/reviews/codex-plan-rereview.md
docs/plans/2026-07-05-full-paperclip-parity/reviews/codex-plan-review.md
docs/plans/2026-07-05-full-paperclip-parity/reviews/review-checklist.md

hook: PostToolUse
hook: PostToolUse Completed
hook: PreToolUse
hook: PreToolUse Completed
exec
/bin/bash -lc 'ls -1 docs/plans/2026-07-05-full-paperclip-parity/phases' in /home/frankji/Projects/paperclip
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
hook: PreToolUse Completed
hook: PostToolUse
hook: PostToolUse Completed
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

hook: PostToolUse
hook: PostToolUse Completed
hook: PreToolUse
hook: PreToolUse Completed
hook: PreToolUse
hook: PreToolUse Completed
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
hook: PostToolUse Completed
codex
The top-level files now state 14 phases and the exact `gaps_missing=0 gaps_partial=0 gaps_divergent=0` output, which addresses two of the named blockers at a surface level. I’m reading the harness spec, Codex prompts, review checklist, and matrix now to verify whether those rules are actually enforceable and mapped row-by-row.
hook: PreToolUse
hook: PreToolUse Completed
exec
/bin/bash -lc "sed -n '1,320p' docs/plans/2026-07-05-full-paperclip-parity/acceptance/full-parity-harness-spec.md" in /home/frankji/Projects/paperclip
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

hook: PreToolUse
hook: PostToolUse
hook: PreToolUse Completed
hook: PostToolUse Completed
hook: PreToolUse
hook: PreToolUse
hook: PreToolUse Completed
hook: PreToolUse Completed
exec
/bin/bash -lc "sed -n '1,340p' docs/plans/2026-07-05-full-paperclip-parity/codex/phase-prompts.md" in /home/frankji/Projects/paperclip
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
hook: PostToolUse Completed
exec
/bin/bash -lc "sed -n '1,320p' docs/plans/2026-07-05-full-paperclip-parity/reviews/review-checklist.md" in /home/frankji/Projects/paperclip
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
exec
/bin/bash -lc "sed -n '1,380p' docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.md" in /home/frankji/Projects/paperclip
 succeeded in 0ms:
# Rewrite Gap Matrix

## Corrected audit tally

- Full: 3
- Partial: 48
- Missing: 98
- Divergent: 24

## Matrix by domain

| Domain | Full | Partial | Missing | Divergent | Plan phase |
|---|---:|---:|---:|---:|---|
| Schema/data model | limited | many | many | defaults/enums | Phase 1 |
| Permissions/board/access | 1 | 4 | 6 | 5 | Phase 1-2 |
| Agent lifecycle/runtime | 0 | 3 | 7 | 6 | Phase 2 |
| Hiring/config side effects | limited | many | many | status/defaults | Phase 3-4 |
| Approvals/governance | limited | many | many | enum/status | Phase 3 |
| Adapters/secrets/skills | 0 | limited | many | config model | Phase 4 |
| Tasks/work products | 0 | limited | many | missing backbone | Phase 5 |
| UI/CLI/MCP/API | limited | many | many | narrow surface | Phase 6 |

## Required closeout rule

A gap can move to `full` only when:

1. Original source evidence is cited.
2. Rewrite file/function/route implements equivalent behavior.
3. Tests prove success and at least one negative/boundary case.
4. Acceptance harness includes it or explains why narrower tests fully cover it.
5. Codex adversarial review agrees.


## Row schema

The generated machine-readable matrix must contain one row per audited capability:

| Field | Required | Meaning |
|---|---|---|
| `row_id` | yes | Stable capability identifier. |
| `domain` | yes | Schema/authz/runtime/etc. |
| `original_evidence` | yes | File/symbol/line proving original behavior. |
| `current_rewrite_evidence` | yes | File/symbol/line proving current status. |
| `initial_status` | yes | One of full/partial/missing/divergent from corrected audit. |
| `phase` | yes for non-full | Owning phase file. |
| `acceptance_id` | yes for non-full | Harness assertion that proves closure. |
| `failing_test` | yes for non-full | Red test path/name observed failing before implementation. |
| `passing_test` | yes for non-full | Command observed green after implementation. |
| `evidence_ref` | yes for non-full | Source and rewrite evidence after closure. |
| `final_status` | yes for non-full | `full` or `waived`. |
| `waiver_ref` | only if waived | Product-contract update and reviewer signoff. |

A row cannot be closed by a category-level summary.

hook: PostToolUse Completed
hook: PostToolUse
hook: PostToolUse Completed
codex
The enforcement language is much stronger now, but the matrix itself still looks like a domain summary rather than the actual row-level closure artifact the plan depends on. I’m pulling line-numbered references for the specific places that pass or fail the requested blocker checks.
hook: PreToolUse
hook: PreToolUse Completed
exec
/bin/bash -lc "nl -ba docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.md | sed -n '1,140p'" in /home/frankji/Projects/paperclip
 succeeded in 0ms:
     1	# Rewrite Gap Matrix
     2	
     3	## Corrected audit tally
     4	
     5	- Full: 3
     6	- Partial: 48
     7	- Missing: 98
     8	- Divergent: 24
     9	
    10	## Matrix by domain
    11	
    12	| Domain | Full | Partial | Missing | Divergent | Plan phase |
    13	|---|---:|---:|---:|---:|---|
    14	| Schema/data model | limited | many | many | defaults/enums | Phase 1 |
    15	| Permissions/board/access | 1 | 4 | 6 | 5 | Phase 1-2 |
    16	| Agent lifecycle/runtime | 0 | 3 | 7 | 6 | Phase 2 |
    17	| Hiring/config side effects | limited | many | many | status/defaults | Phase 3-4 |
    18	| Approvals/governance | limited | many | many | enum/status | Phase 3 |
    19	| Adapters/secrets/skills | 0 | limited | many | config model | Phase 4 |
    20	| Tasks/work products | 0 | limited | many | missing backbone | Phase 5 |
    21	| UI/CLI/MCP/API | limited | many | many | narrow surface | Phase 6 |
    22	
    23	## Required closeout rule
    24	
    25	A gap can move to `full` only when:
    26	
    27	1. Original source evidence is cited.
    28	2. Rewrite file/function/route implements equivalent behavior.
    29	3. Tests prove success and at least one negative/boundary case.
    30	4. Acceptance harness includes it or explains why narrower tests fully cover it.
    31	5. Codex adversarial review agrees.
    32	
    33	
    34	## Row schema
    35	
    36	The generated machine-readable matrix must contain one row per audited capability:
    37	
    38	| Field | Required | Meaning |
    39	|---|---|---|
    40	| `row_id` | yes | Stable capability identifier. |
    41	| `domain` | yes | Schema/authz/runtime/etc. |
    42	| `original_evidence` | yes | File/symbol/line proving original behavior. |
    43	| `current_rewrite_evidence` | yes | File/symbol/line proving current status. |
    44	| `initial_status` | yes | One of full/partial/missing/divergent from corrected audit. |
    45	| `phase` | yes for non-full | Owning phase file. |
    46	| `acceptance_id` | yes for non-full | Harness assertion that proves closure. |
    47	| `failing_test` | yes for non-full | Red test path/name observed failing before implementation. |
    48	| `passing_test` | yes for non-full | Command observed green after implementation. |
    49	| `evidence_ref` | yes for non-full | Source and rewrite evidence after closure. |
    50	| `final_status` | yes for non-full | `full` or `waived`. |
    51	| `waiver_ref` | only if waived | Product-contract update and reviewer signoff. |
    52	
    53	A row cannot be closed by a category-level summary.

hook: PostToolUse
hook: PreToolUse
hook: PreToolUse Completed
hook: PostToolUse Completed
hook: PreToolUse
hook: PreToolUse Completed
hook: PreToolUse
hook: PreToolUse Completed
hook: PreToolUse
exec
/bin/bash -lc "nl -ba docs/plans/2026-07-05-full-paperclip-parity/04-execution-order.md | sed -n '1,140p'" in /home/frankji/Projects/paperclip
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
hook: PreToolUse Completed
hook: PostToolUse Completed
exec
/bin/bash -lc "nl -ba docs/plans/2026-07-05-full-paperclip-parity/05-test-strategy.md | sed -n '1,140p'" in /home/frankji/Projects/paperclip
 succeeded in 0ms:
     1	# Test Strategy
     2	
     3	## TDD rule
     4	
     5	Each phase begins with failing tests that encode original parity. Production changes are not accepted until those tests pass.
     6	
     7	## Test layers
     8	
     9	### Unit tests
    10	
    11	- Authz predicate matrix.
    12	- Status transition functions.
    13	- Approval decision side effects.
    14	- Adapter config normalization/redaction.
    15	- Secret ref validation.
    16	- Budget hard-stop calculations.
    17	
    18	### Route/service tests
    19	
    20	- Rust axum route tests for every endpoint.
    21	- Supabase gateway spy tests proving session vs API-key dispatch.
    22	- Error mapping tests for 400/401/403/404/409/422.
    23	
    24	### Migration/RLS tests
    25	
    26	- Live Supabase introspection for schemas/tables/functions/grants.
    27	- RLS tests for board user, normal member, viewer, agent key, low-trust agent key, cross-company attempts.
    28	- SECURITY DEFINER `search_path=''` checks.
    29	
    30	### Frontend tests
    31	
    32	- Composable tests for every API client.
    33	- Component/page tests for hire, approvals, lifecycle, runtime, access, budget, tasks.
    34	- No Supabase/OpenAI details in rendered output or responses.
    35	
    36	### CLI/MCP tests
    37	
    38	- CLI command parsing and server calls.
    39	- MCP tool schemas and JSON-RPC smoke.
    40	- Agent-key vs board-session permissions.
    41	
    42	### Live acceptance
    43	
    44	- Real users: `testadmin1`, `testuser1`, `testuser2`, `testuser3` per `test_users.json` (gitignored).
    45	- Real company.
    46	- Real agent hire/create.
    47	- Real board approval where required.
    48	- Real OpenAI Responses run.
    49	- DB attribution and lifecycle assertion.
    50	
    51	## Required final commands
    52	
    53	Backend:
    54	
    55	```sh
    56	cd backend-rs
    57	cargo test
    58	cargo build --bins
    59	```
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
    86	A row may move to `full` only after the failing test has been observed red, the implementation makes it green, source evidence is cited, and the reviewer updates `evidence/rewrite-gap-matrix.md`. A row may move to `waived` only after `doc/SPEC-implementation.md` or another canonical product contract is updated to say the original behavior is no longer required.

hook: PostToolUse
hook: PostToolUse Completed
exec
/bin/bash -lc "nl -ba docs/plans/2026-07-05-full-paperclip-parity/acceptance/full-parity-harness-spec.md | sed -n '1,140p'" in /home/frankji/Projects/paperclip
 succeeded in 0ms:
     1	# Full Parity Harness Spec
     2	
     3	## Harness inputs
     4	
     5	- Supabase URL and anon key for `supabase-franky` only.
     6	- OpenAI-compatible `OPENAI_BASE_URL` and `OPENAI_API_KEY` from environment.
     7	- `test_users.json` with gitignored real test users.
     8	- Rust backend URL.
     9	- Nuxt frontend URL when frontend smoke runs.
    10	
    11	## Required assertions
    12	
    13	1. Schema introspection: no app objects in `public`; expected tables/functions/enums in `paperclip`; helpers in `paperclip_private`.
    14	2. Defaults/enums match original: company hire approval default false; canonical statuses/types/statuses.
    15	3. Authz matrix: board/session/agent/viewer/low-trust/cross-company behavior.
    16	4. Hire/create: direct create when approval default false; approval flow when enabled.
    17	5. Approval: revision/request/approve/reject/cancel for all types.
    18	6. Runtime: wakeup/run/cancel/logs/status transitions.
    19	7. Budget: cost event, hard-stop auto-pause, budget override approval.
    20	8. Secrets/config: redaction and Codex isolation.
    21	9. Work: task assignment, checkout, comment/work product.
    22	10. Interfaces: HTTP, CLI, MCP, Nuxt.
    23	11. Real OpenAI Responses run: result persisted under run/job/work product and attributed to agent.
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
/bin/bash -lc "nl -ba docs/plans/2026-07-05-full-paperclip-parity/codex/phase-prompts.md | sed -n '1,140p'" in /home/frankji/Projects/paperclip
 succeeded in 0ms:
     1	# Codex Prompts
     2	
     3	## Plan review prompt
     4	
     5	```text
     6	You are reviewing docs/plans/2026-07-05-full-paperclip-parity for completeness against original Paperclip V1. Read doc/SPEC-implementation.md, packages/shared/src/constants.ts, server/src/routes/agents.ts, server/src/routes/approvals.ts, server/src/routes/access.ts, server/src/services/approvals.ts, server/src/services/heartbeat.ts, ui/src/pages/NewAgent.tsx, cli/src/commands/client/agent.ts, and the plan directory. Identify any missing or under-specified parity area. Do not implement. Return must-fix plan gaps with file references.
     7	```
     8	
     9	## Phase implementer prompt template
    10	
    11	```text
    12	Before implementing, read PHASE_FILE plus `01-parity-audit-summary.md`, `04-execution-order.md`, `05-test-strategy.md`, `acceptance/full-parity-harness-spec.md`, `evidence/rewrite-gap-matrix.md`, and `reviews/review-checklist.md`. Implement only the rows owned by PHASE_FILE using strict TDD. First write failing tests that demonstrate the current rewrite does not match original Paperclip behavior. Then implement the minimal code/migrations to pass. Preserve: no public schema, no service-role key, no Supabase/OpenAI details client-side, OpenAI Responses API only. Update each owned matrix row with phase, acceptance_id, failing_test, passing_test, evidence_ref, and final_status. Run the phase checks and report exact output.
    13	```
    14	
    15	## Final parity review prompt
    16	
    17	```text
    18	Adversarially compare the final rewrite against original Paperclip V1 for agent hiring/governance/runtime/interfaces. Use docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.md as the checklist. Confirm gaps_missing=0 gaps_partial=0 gaps_divergent=0 or list blockers. Do not accept a vertical slice as full parity.
    19	```
    20	
    21	
    22	## Anti-false-parity rule for Codex workers
    23	
    24	A phase worker may not report "done" because a category test passed. Done means every owned row in `evidence/rewrite-gap-matrix.md` is `full` or has an approved `waived` product-contract reference. The worker must include the row IDs it closed and the commands that turned each red test green.

hook: PostToolUse
hook: PostToolUse Completed
hook: PreToolUse
hook: PreToolUse Completed
hook: PreToolUse
exec
/bin/bash -lc "nl -ba docs/plans/2026-07-05-full-paperclip-parity/README.md | sed -n '1,110p'" in /home/frankji/Projects/paperclip
hook: PreToolUse
 succeeded in 0ms:
     1	# Full Paperclip Parity Implementation Plan
     2	
     3	> **Goal:** eliminate the verified parity gap between the current Rust/Supabase/Nuxt rewrite and original Paperclip V1 for agent hiring, governance, runtime, interfaces, and real work execution.
     4	>
     5	> **Definition of success:** the rewrite reaches **zero known parity gaps** against `doc/SPEC-implementation.md` and the original TypeScript implementation evidence cited here. A working vertical slice is not enough.
     6	
     7	## Directory map
     8	
     9	- [`00-overview.md`](00-overview.md) — scope, success criteria, and non-negotiable invariants.
    10	- [`01-parity-audit-summary.md`](01-parity-audit-summary.md) — corrected audit baseline: 3 full / 50 partial / 100 missing / 22 divergent.
    11	- [`02-target-architecture.md`](02-target-architecture.md) — target Rust/Supabase/Nuxt architecture that preserves original V1 contracts.
    12	- [`03-file-structure.md`](03-file-structure.md) — planned module/file layout for schema, backend, CLI/MCP, Nuxt, and test harnesses.
    13	- [`04-execution-order.md`](04-execution-order.md) — dependency-ordered phases for TDD/Codex workers.
    14	- [`05-test-strategy.md`](05-test-strategy.md) — red/green/unit/integration/live acceptance strategy.
    15	- [`06-risk-register.md`](06-risk-register.md) — security, data, runtime, migration, and external-service risks.
    16	- [`phases/`](phases/) — implementation workstreams, each modular and reviewable.
    17	- [`acceptance/`](acceptance/) — parity harness specs and real-user/real-agent scenarios.
    18	- [`codex/`](codex/) — Codex execution/review prompts.
    19	- [`reviews/`](reviews/) — plan-eng-review and Codex review artifacts.
    20	- [`evidence/`](evidence/) — original-source map and rewrite gap matrix.
    21	
    22	## How to use this plan
    23	
    24	1. Treat [`01-parity-audit-summary.md`](01-parity-audit-summary.md) as the starting contract; do not erase gaps without source-backed implementation and tests.
    25	2. Implement phases in [`04-execution-order.md`](04-execution-order.md) order. Each phase is designed for Codex TDD workers.
    26	3. After each phase, run the narrow phase checks plus the full parity harness from [`acceptance/full-parity-harness-spec.md`](acceptance/full-parity-harness-spec.md).
    27	4. Completion requires the final Codex parity review to produce no remaining missing/divergent/partial items unless explicitly marked out-of-scope by a revised product contract.
    28	
    29	## Non-negotiables
    30	
    31	- Use only Supabase schemas `paperclip` and `paperclip_private`; **do not use `public`** for application tables/functions.
    32	- Do not use a Supabase service-role key in app, CLI, MCP, frontend, or test acceptance flows.
    33	- No Supabase/OpenAI credentials or implementation details reach the client.
    34	- Preserve company boundaries, agent low-trust boundaries, approval gates, budgets, activity logs, and secret redaction.
    35	- Use OpenAI **Responses API**, not chat completions, for OpenAI-backed adapter/job paths.
    36	
    37	## Plan-design review revision
    38	
    39	The first written plan had 7 phases. Ultracode plan-design review found that still under-covered original V1. This revision expands the plan to 14 owned phases, including goals/projects/issues, deployment/auth/onboarding, workspaces/routines/plugins, import/export/catalogs, observability/pipelines, shared contracts/docs, and cutover.

hook: PreToolUse Completed
hook: PostToolUse
hook: PostToolUse Completed
hook: PreToolUse Completed
exec
/bin/bash -lc "find docs/plans/2026-07-05-full-paperclip-parity/phases -maxdepth 1 -type f -printf '%f\\n' | sort | wc -l" in /home/frankji/Projects/paperclip
 succeeded in 0ms:
14

hook: PostToolUse
exec
/bin/bash -lc "find docs/plans/2026-07-05-full-paperclip-parity/acceptance -maxdepth 1 -type f -printf '%f\\n' | sort" in /home/frankji/Projects/paperclip
 succeeded in 0ms:
README.md
deterministic-local-scenarios.md
full-parity-harness-spec.md
no-secret-scanner-spec.md
real-user-real-agent-scenarios.md
release-gate-commands.md

hook: PostToolUse
hook: PostToolUse Completed
hook: PostToolUse Completed
codex
**BLOCKERS**

1. Row-level matrix closure is still not actually present. [rewrite-gap-matrix.md](/home/frankji/Projects/paperclip/docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.md:10) still contains only a domain summary table, not one row per audited capability. It defines the required row schema at lines 34-53, but the 48 partial, 98 missing, and 24 divergent rows are not enumerated with `row_id`, owner phase, acceptance ID, tests, evidence, and final status. This means workers still cannot close every known gap deterministically.

2. The matrix still has stale phase ownership. [rewrite-gap-matrix.md](/home/frankji/Projects/paperclip/docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.md:14) maps domains to old “Phase 1”, “Phase 1-2”, “Phase 3-4”, etc., while [04-execution-order.md](/home/frankji/Projects/paperclip/docs/plans/2026-07-05-full-paperclip-parity/04-execution-order.md:7) now defines 14 phase files. That undercuts the claimed row ownership model.

3. Required final acceptance commands reference scripts that are not present under `acceptance/`. [05-test-strategy.md](/home/frankji/Projects/paperclip/docs/plans/2026-07-05-full-paperclip-parity/05-test-strategy.md:71) requires `run_original_gap_diff.py` and `run_real_user_real_agent.py`, but the acceptance directory only contains markdown specs. The exact `gaps_missing=0 gaps_partial=0 gaps_divergent=0` output is specified, but there is no executable harness file in the reviewed tree to produce it.

**PATCHES**

- Phase count blocker: fixed. There are 14 phase files, and [README.md](/home/frankji/Projects/paperclip/docs/plans/2026-07-05-full-paperclip-parity/README.md:39) plus [04-execution-order.md](/home/frankji/Projects/paperclip/docs/plans/2026-07-05-full-paperclip-parity/04-execution-order.md:3) explicitly lock the plan to 14 phases.

- Exact gap output blocker: partially fixed. [full-parity-harness-spec.md](/home/frankji/Projects/paperclip/docs/plans/2026-07-05-full-paperclip-parity/acceptance/full-parity-harness-spec.md:47) requires the exact `gaps_missing=0 gaps_partial=0 gaps_divergent=0` fields, and [04-execution-order.md](/home/frankji/Projects/paperclip/docs/plans/2026-07-05-full-paperclip-parity/04-execution-order.md:32) repeats it. Still blocked by missing executable harness scripts.

- Codex implementer context blocker: mostly fixed. [phase-prompts.md](/home/frankji/Projects/paperclip/docs/plans/2026-07-05-full-paperclip-parity/codex/phase-prompts.md:12) now tells implementers to read the phase file, audit summary, execution order, test strategy, harness spec, matrix, and review checklist before implementation.

- Anti-false-parity gate blocker: specified but not enforceable yet. [05-test-strategy.md](/home/frankji/Projects/paperclip/docs/plans/2026-07-05-full-paperclip-parity/05-test-strategy.md:78), [full-parity-harness-spec.md](/home/frankji/Projects/paperclip/docs/plans/2026-07-05-full-paperclip-parity/acceptance/full-parity-harness-spec.md:32), and [phase-prompts.md](/home/frankji/Projects/paperclip/docs/plans/2026-07-05-full-paperclip-parity/codex/phase-prompts.md:22) all state the rule. It remains blocked because the matrix has no actual rows to enforce against.

**VERDICT**

FAIL
hook: Stop
hook: Stop Completed
tokens used
45,920
**BLOCKERS**

1. Row-level matrix closure is still not actually present. [rewrite-gap-matrix.md](/home/frankji/Projects/paperclip/docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.md:10) still contains only a domain summary table, not one row per audited capability. It defines the required row schema at lines 34-53, but the 48 partial, 98 missing, and 24 divergent rows are not enumerated with `row_id`, owner phase, acceptance ID, tests, evidence, and final status. This means workers still cannot close every known gap deterministically.

2. The matrix still has stale phase ownership. [rewrite-gap-matrix.md](/home/frankji/Projects/paperclip/docs/plans/2026-07-05-full-paperclip-parity/evidence/rewrite-gap-matrix.md:14) maps domains to old “Phase 1”, “Phase 1-2”, “Phase 3-4”, etc., while [04-execution-order.md](/home/frankji/Projects/paperclip/docs/plans/2026-07-05-full-paperclip-parity/04-execution-order.md:7) now defines 14 phase files. That undercuts the claimed row ownership model.

3. Required final acceptance commands reference scripts that are not present under `acceptance/`. [05-test-strategy.md](/home/frankji/Projects/paperclip/docs/plans/2026-07-05-full-paperclip-parity/05-test-strategy.md:71) requires `run_original_gap_diff.py` and `run_real_user_real_agent.py`, but the acceptance directory only contains markdown specs. The exact `gaps_missing=0 gaps_partial=0 gaps_divergent=0` output is specified, but there is no executable harness file in the reviewed tree to produce it.

**PATCHES**

- Phase count blocker: fixed. There are 14 phase files, and [README.md](/home/frankji/Projects/paperclip/docs/plans/2026-07-05-full-paperclip-parity/README.md:39) plus [04-execution-order.md](/home/frankji/Projects/paperclip/docs/plans/2026-07-05-full-paperclip-parity/04-execution-order.md:3) explicitly lock the plan to 14 phases.

- Exact gap output blocker: partially fixed. [full-parity-harness-spec.md](/home/frankji/Projects/paperclip/docs/plans/2026-07-05-full-paperclip-parity/acceptance/full-parity-harness-spec.md:47) requires the exact `gaps_missing=0 gaps_partial=0 gaps_divergent=0` fields, and [04-execution-order.md](/home/frankji/Projects/paperclip/docs/plans/2026-07-05-full-paperclip-parity/04-execution-order.md:32) repeats it. Still blocked by missing executable harness scripts.

- Codex implementer context blocker: mostly fixed. [phase-prompts.md](/home/frankji/Projects/paperclip/docs/plans/2026-07-05-full-paperclip-parity/codex/phase-prompts.md:12) now tells implementers to read the phase file, audit summary, execution order, test strategy, harness spec, matrix, and review checklist before implementation.

- Anti-false-parity gate blocker: specified but not enforceable yet. [05-test-strategy.md](/home/frankji/Projects/paperclip/docs/plans/2026-07-05-full-paperclip-parity/05-test-strategy.md:78), [full-parity-harness-spec.md](/home/frankji/Projects/paperclip/docs/plans/2026-07-05-full-paperclip-parity/acceptance/full-parity-harness-spec.md:32), and [phase-prompts.md](/home/frankji/Projects/paperclip/docs/plans/2026-07-05-full-paperclip-parity/codex/phase-prompts.md:22) all state the rule. It remains blocked because the matrix has no actual rows to enforce against.

**VERDICT**

FAIL
