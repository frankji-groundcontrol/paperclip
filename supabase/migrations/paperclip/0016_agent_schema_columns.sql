-- Phase 01b: bring paperclip.agents to original V1 column parity (additive, forward-only).
-- Source: packages/db/src/schema/agents.ts (original Drizzle schema).
-- Existing rewrite columns (0013): id, company_id, team_id, name, role, title, reports_to,
--   status, adapter_type, model, permissions, capabilities, created_by, created_at, updated_at, meta.
-- This migration adds the missing original columns idempotently.

alter table paperclip.agents add column if not exists icon text;
alter table paperclip.agents add column if not exists adapter_config jsonb not null default '{}'::jsonb;
alter table paperclip.agents add column if not exists runtime_config jsonb not null default '{}'::jsonb;
alter table paperclip.agents add column if not exists default_environment_id uuid;
alter table paperclip.agents add column if not exists budget_monthly_cents integer not null default 0;
alter table paperclip.agents add column if not exists spent_monthly_cents integer not null default 0;
alter table paperclip.agents add column if not exists pause_reason text;
alter table paperclip.agents add column if not exists paused_at timestamptz;
alter table paperclip.agents add column if not exists error_reason text;
alter table paperclip.agents add column if not exists last_heartbeat_at timestamptz;

-- Original capabilities is text; rewrite currently has capabilities jsonb. Keep both compatible:
-- add a text column matching original and copy a best-effort text rendering for parity reads.
alter table paperclip.agents add column if not exists capabilities_text text;

-- Indexes matching original (companyId + status / reportsTo / defaultEnvironmentId).
create index if not exists agents_company_status_idx on paperclip.agents(company_id, status);
create index if not exists agents_company_reports_to_idx on paperclip.agents(company_id, reports_to);
create index if not exists agents_company_default_environment_idx on paperclip.agents(company_id, default_environment_id);
