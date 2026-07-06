# Migration Ledger

Existing Supabase Paperclip migrations:

- `0001_schema.sql` through `0013_agents_hiring.sql` already exist.

Applied on supabase-franky (Phase 01a, codex TDD cycle):

- `0014_parity_enums_defaults.sql` — company hire-approval default false; agent_status rename archived→terminated + add idle/running/error; approval_type add budget_override_required/request_board_approval; approval_status add revision_requested. (Applied as MCP migration `paperclip_parity_enums_defaults`.)
- `0015_parity_transitions.sql` — agents.status default idle; active→idle data migration; fn_agent_active checks (idle,running); hire/decide RPCs use idle (approve) and terminated (reject). (Applied as MCP migration `paperclip_parity_transitions`.)

Applied on supabase-franky (Phase 01b / 06, hardening cycle):

- `0016_agent_schema_columns.sql` — original V1 agent columns (icon, adapter_config, runtime_config, default_environment_id, budget_monthly_cents, spent_monthly_cents, pause_reason, paused_at, error_reason, last_heartbeat_at, capabilities_text) + parity indexes. (Applied as MCP migration `paperclip_agent_schema_columns`.)
- `0017_company_approval_permission_columns.sql` — companies V1 columns (description/status/issue_prefix/budgets/branding/feedback); approvals enrichment (decision_note, revision_requested_at/by, source_issue_id); approval_comments table; principal_permission_grants + permission_key enum + fn_has_permission helper. (Applied as MCP migration `paperclip_company_approval_permission_columns`; issue_prefix index is non-unique until a backfill follow-up.)
- `0018_approval_revision_comments.sql` — request_revision / resubmit_approval / add_approval_comment / list_approval_comments RPCs. (Applied as MCP migration `paperclip_approval_revision_comments`; lifecycle verified live.)

Planned forward-only parity migrations continue after `0020`:

- Phase 02/07 write RPCs (create/update issue, comment, goal, project) + live-backend route wiring.
- Phase 07 heartbeat execution (scheduler, run lifecycle, wakeup fulfillment).

Applied on supabase-franky (Supabase unification — port everything to Supabase):

- `0019_v1_control_plane_tables.sql` — 33 V1 control-plane tables ported to `paperclip` schema (goals, projects, issues, issue_comments/documents/work_products/attachments/relations, cost_events, budget_policies/incidents, activity_log, agent_wakeup_requests, heartbeat_runs/events, agent_runtime_state/task_sessions/config_revisions, environments, routines, execution_workspaces, workspace_runtime_services, company_secrets/versions/bindings/provider_configs, secret_access_events, company_skills, pipelines/cases/case_events, feedback_exports/votes), each with team-scoped RLS + member-read policy. Adds agents.default_environment_id FK now that environments exists. (Applied as MCP migration `paperclip_v1_control_plane_tables`.)
- `0020_v1_read_views_and_list_rpcs.sql` — `my_*` security_invoker read views + `list_*_with_key` forge-proof RPCs for goals/projects/issues/comments/activity/costs/heartbeat_runs. (Applied as MCP migration `paperclip_v1_read_views_and_list_rpcs`.)
- `0021a_work_backbone_write_rpcs.sql` — write-path RPCs for the V1 work backbone: create_goal/create_project/create_issue/update_issue/add_issue_comment (+ `_with_key` variants), plus paperclip_private.log_activity helper. All mutate with team-scoped authz + same-company FK validation + activity logging. Verified live on supabase-franky.
- `0021b_costs_budgets_heartbeat_write_rpcs.sql` — write-path RPCs for costs/budgets/runtime: ingest_cost_event (with automatic company-over-budget incident + agent auto-pause when company budget exceeded), upsert_budget_policy (board-only), create_heartbeat_run/complete_heartbeat_run/fail_heartbeat_run (agent idle→running→idle|error transitions, only-if-no-other-run-active guard), create_wakeup_request/fulfill_wakeup_request. All verified live.

Rule: do not create a migration number without updating this file in the same change.
