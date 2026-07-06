# Planned File Structure

## Supabase migrations

Add migrations after `0013_agents_hiring.sql` instead of rewriting applied history:

```text
supabase/migrations/paperclip/
  0014_parity_enums_defaults.sql
  0015_permission_grants_authz.sql
  0016_agent_schema_runtime_config.sql
  0017_approval_governance.sql
  0018_activity_cost_budget.sql
  0019_heartbeat_runtime.sql
  0020_tasks_issues_work_products.sql
  0021_secrets_skills_config_revisions.sql
  0022_parity_views_rpcs.sql
```

Each migration must have a matching acceptance note in this plan and a live Supabase verification query.

## Rust backend modules

```text
backend-rs/src/
  authz/
    mod.rs
    principal.rs
    permissions.rs
    low_trust.rs
  companies/
    mod.rs
    service.rs
    routes.rs
  agents/
    mod.rs
    dto.rs
    service.rs
    routes.rs
    lifecycle.rs
    config.rs
    keys.rs
  approvals/
    mod.rs
    service.rs
    routes.rs
  runtime/
    mod.rs
    heartbeat.rs
    wakeups.rs
    runs.rs
    logs.rs
    sessions.rs
  adapters/
    mod.rs
    openai_responses.rs
    process_contract.rs
  tasks/
    mod.rs
    issues.rs
    checkout.rs
    comments.rs
    work_products.rs
  budgets/
    mod.rs
    service.rs
  activity/
    mod.rs
    service.rs
  secrets/
    mod.rs
    redaction.rs
  supabase/
    routes.rs          # composition only; feature routes live in modules
```

Existing files such as `jobs/mod.rs` should shrink into adapter/runtime calls instead of remaining the central control-plane service.

## Frontend modules

```text
frontend-nuxt/
  composables/
    usePaperclipSession.ts
    useCompanies.ts
    useAgents.ts
    useApprovals.ts
    useRuntime.ts
    useTasks.ts
    useBudgets.ts
    useAccess.ts
  components/
    company/
    agents/
    approvals/
    runtime/
    tasks/
    budgets/
    access/
  pages/
    companies/[companyId]/settings.vue
    companies/[companyId]/agents/index.vue
    companies/[companyId]/agents/new.vue
    companies/[companyId]/agents/[agentId].vue
    companies/[companyId]/approvals/index.vue
    companies/[companyId]/approvals/[approvalId].vue
```

## CLI/MCP

```text
backend-rs/src/bin/paperclip.rs       # command router only; move command impl into modules if it grows
backend-rs/src/bin/paperclip-mcp.rs   # tool registry only; helper functions per domain
backend-rs/src/cli/
  agents.rs
  approvals.rs
  runtime.rs
  tasks.rs
  access.rs
```

## Tests and acceptance

```text
backend-rs/tests/
  authz_parity.rs
  agent_lifecycle_parity.rs
  approvals_parity.rs
  runtime_parity.rs
  api_surface_parity.rs
frontend-nuxt/tests/
  agents-parity.test.ts
  approvals-parity.test.ts
  runtime-parity.test.ts
docs/plans/2026-07-05-full-paperclip-parity/acceptance/
  run_full_parity_acceptance.py
  run_original_gap_diff.py
  run_real_user_real_agent.py
```
