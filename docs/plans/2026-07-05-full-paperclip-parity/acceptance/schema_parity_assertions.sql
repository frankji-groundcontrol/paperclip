-- Phase 01 schema-parity assertions. Run via operator DB access (supabase-franky MCP / psql).
-- Returns one row per check with ok boolean. All must be ok=true after migration 0014.

with agent_status_vals as (
  select array_agg(e.enumlabel::text order by e.enumsortorder) as v
  from pg_type t join pg_namespace n on n.oid=t.typnamespace
  join pg_enum e on e.enumtypid=t.oid
  where n.nspname='paperclip' and t.typname='agent_status'
),
approval_type_vals as (
  select array_agg(e.enumlabel::text order by e.enumsortorder) as v
  from pg_type t join pg_namespace n on n.oid=t.typnamespace
  join pg_enum e on e.enumtypid=t.oid
  where n.nspname='paperclip' and t.typname='approval_type'
),
approval_status_vals as (
  select array_agg(e.enumlabel::text order by e.enumsortorder) as v
  from pg_type t join pg_namespace n on n.oid=t.typnamespace
  join pg_enum e on e.enumtypid=t.oid
  where n.nspname='paperclip' and t.typname='approval_status'
),
company_default as (
  select column_default from information_schema.columns
  where table_schema='paperclip' and table_name='companies' and column_name='require_board_approval_for_new_agents'
),
agent_status_default as (
  select column_default from information_schema.columns
  where table_schema='paperclip' and table_name='agents' and column_name='status'
)
select
  ('company hire-approval default false') as check_name,
  (select column_default from company_default) as actual,
  ('false'::text) as expected,
  ((select column_default from company_default) = 'false') as ok
union all select
  'agent_status has idle', (select v::text from agent_status_vals), 'contains idle',
  (select v @> '{idle}'::text[] from agent_status_vals)
union all select
  'agent_status has running', (select v::text from agent_status_vals), 'contains running',
  (select v @> '{running}'::text[] from agent_status_vals)
union all select
  'agent_status has error', (select v::text from agent_status_vals), 'contains error',
  (select v @> '{error}'::text[] from agent_status_vals)
union all select
  'agent_status has terminated', (select v::text from agent_status_vals), 'contains terminated',
  (select v @> '{terminated}'::text[] from agent_status_vals)
union all select
  'agent_status no archived', (select v::text from agent_status_vals), 'no archived',
  (coalesce((select v @> '{archived}'::text[] from agent_status_vals), false) = false)
union all select
  'approval_type has budget_override_required', (select v::text from approval_type_vals), 'present',
  (select v @> '{budget_override_required}'::text[] from approval_type_vals)
union all select
  'approval_type has request_board_approval', (select v::text from approval_type_vals), 'present',
  (select v @> '{request_board_approval}'::text[] from approval_type_vals)
union all select
  'approval_status has revision_requested', (select v::text from approval_status_vals), 'present',
  (select v @> '{revision_requested}'::text[] from approval_status_vals)
union all select
  'agents.status default idle', (select column_default from agent_status_default), 'idle',
  ((select column_default from agent_status_default) like '''idle''%');
