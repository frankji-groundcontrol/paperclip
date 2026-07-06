-- Phase 01b/06: companies V1 columns + approvals enrichment + permission grants.
-- Sources: packages/db/src/schema/{companies,approvals,principal_permission_grants}.ts,
--          packages/shared/src/constants.ts (PERMISSION_KEYS, APPROVAL_TYPES),
--          server/src/services/company-member-roles.ts (role -> grant defaults).

-- ===== companies: original V1 columns =====
alter table paperclip.companies add column if not exists description text;
alter table paperclip.companies add column if not exists status text not null default 'active';
alter table paperclip.companies add column if not exists pause_reason text;
alter table paperclip.companies add column if not exists paused_at timestamptz;
alter table paperclip.companies add column if not exists issue_prefix text not null default 'PAP';
alter table paperclip.companies add column if not exists issue_counter integer not null default 0;
alter table paperclip.companies add column if not exists budget_monthly_cents integer not null default 0;
alter table paperclip.companies add column if not exists spent_monthly_cents integer not null default 0;
alter table paperclip.companies add column if not exists attachment_max_bytes integer not null default 10485760;
alter table paperclip.companies add column if not exists feedback_data_sharing_enabled boolean not null default false;
alter table paperclip.companies add column if not exists feedback_data_sharing_consent_at timestamptz;
alter table paperclip.companies add column if not exists feedback_data_sharing_consent_by_user_id text;
alter table paperclip.companies add column if not exists feedback_data_sharing_terms_version text;
alter table paperclip.companies add column if not exists brand_color text;
create index if not exists companies_issue_prefix_idx on paperclip.companies(issue_prefix);
-- NOTE: original makes issue_prefix UNIQUE; the rewrite's existing test companies share the
-- 'PAP' default, so this migration creates a non-unique index. A follow-up backfill + unique
-- index is tracked as a Phase 13 cutover task.

-- ===== approvals: decision note + revision + linked issue =====
alter table paperclip.approvals add column if not exists decision_note text;
alter table paperclip.approvals add column if not exists revision_requested_at timestamptz;
alter table paperclip.approvals add column if not exists revision_requested_by uuid references paperclip.users(id) on delete set null;
alter table paperclip.approvals add column if not exists source_issue_id uuid;
create index if not exists approvals_status_idx on paperclip.approvals(company_id, status);

-- ===== approval_comments =====
create table if not exists paperclip.approval_comments (
  id           uuid primary key default gen_random_uuid(),
  approval_id  uuid not null references paperclip.approvals(id) on delete cascade,
  team_id      uuid not null references paperclip.teams(id) on delete cascade,
  author_id    uuid references paperclip.users(id) on delete set null,
  body         text not null,
  created_at   timestamptz not null default now()
);
create index if not exists approval_comments_approval_idx on paperclip.approval_comments(approval_id);
alter table paperclip.approval_comments enable row level security;
drop policy if exists approval_comments_member_read on paperclip.approval_comments;
create policy approval_comments_member_read on paperclip.approval_comments for select to authenticated
  using ( team_id in (select paperclip_private.fn_user_team_ids(auth.uid())) );
grant select on paperclip.approval_comments to authenticated;

-- ===== principal_permission_grants (original PERMISSION_KEYS model) =====
do $$ begin
  if not exists (select 1 from pg_type t join pg_namespace n on n.oid=t.typnamespace
                 where n.nspname='paperclip' and t.typname='permission_key') then
    create type paperclip.permission_key as enum (
      'agents:create','skills:create','environments:manage','users:invite',
      'users:manage_permissions','tasks:assign','tasks:assign_scope',
      'tasks:manage_active_checkouts','pipelines:write','joins:approve'
    );
  end if;
end $$;

create table if not exists paperclip.principal_permission_grants (
  id              uuid primary key default gen_random_uuid(),
  team_id         uuid not null references paperclip.teams(id) on delete cascade,
  company_id      uuid references paperclip.companies(id) on delete cascade,
  subject_type    text not null,          -- 'user' | 'agent'
  subject_id      uuid not null,
  permission_key  paperclip.permission_key not null,
  granted_by      uuid references paperclip.users(id) on delete set null,
  created_at      timestamptz not null default now(),
  unique (team_id, company_id, subject_type, subject_id, permission_key)
);
create index if not exists principal_grants_subject_idx on paperclip.principal_permission_grants(subject_type, subject_id);
alter table paperclip.principal_permission_grants enable row level security;
drop policy if exists principal_grants_member_read on paperclip.principal_permission_grants;
create policy principal_grants_member_read on paperclip.principal_permission_grants for select to authenticated
  using ( team_id in (select paperclip_private.fn_user_team_ids(auth.uid())) );
grant select on paperclip.principal_permission_grants to authenticated;

-- Helper: does a principal hold a permission key in a team/company? (SECURITY DEFINER, no public grant)
create or replace function paperclip_private.fn_has_permission(
  p_team uuid, p_company uuid, p_subject_type text, p_subject_id uuid, p_key paperclip.permission_key)
returns boolean language sql stable security definer set search_path='' as $$
  select exists (
    select 1 from paperclip.principal_permission_grants
    where team_id = p_team
      and (p_company is null or company_id is null or company_id = p_company)
      and subject_type = p_subject_type and subject_id = p_subject_id
      and permission_key = p_key
  );
$$;
revoke execute on function paperclip_private.fn_has_permission(uuid,uuid,text,uuid,paperclip.permission_key) from public, anon, authenticated;
