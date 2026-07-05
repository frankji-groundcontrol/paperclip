-- Paperclip auth foundation — schema (see docs/plans/2026-07-05-paperclip-supabase-auth/01-schema.md)
create schema if not exists paperclip;
create schema if not exists paperclip_private;

revoke all on schema paperclip_private from anon, authenticated, public;
grant usage on schema paperclip to anon, authenticated;

do $$ begin
  if not exists (select 1 from pg_type t join pg_namespace n on n.oid=t.typnamespace where n.nspname='paperclip' and t.typname='team_role') then
    create type paperclip.team_role as enum ('owner','admin','operator','viewer','member');
  end if;
  if not exists (select 1 from pg_type t join pg_namespace n on n.oid=t.typnamespace where n.nspname='paperclip' and t.typname='api_key_subject') then
    create type paperclip.api_key_subject as enum ('user','agent');
  end if;
  if not exists (select 1 from pg_type t join pg_namespace n on n.oid=t.typnamespace where n.nspname='paperclip' and t.typname='join_request_status') then
    create type paperclip.join_request_status as enum ('pending_approval','approved','rejected');
  end if;
end $$;

create table if not exists paperclip.users (
  id             uuid primary key references auth.users(id) on delete cascade,
  email          extensions.citext,
  display_name   text,
  system_role    text not null default 'user' check (system_role in ('user','instance_admin')),
  default_team_id uuid,
  created_at     timestamptz not null default now(),
  updated_at     timestamptz not null default now(),
  meta           jsonb not null default '{}'::jsonb
);

create table if not exists paperclip.teams (
  id           uuid primary key default gen_random_uuid(),
  name         text not null,
  slug         text not null unique,
  is_personal  boolean not null default false,
  created_by   uuid references paperclip.users(id) on delete set null,
  created_at   timestamptz not null default now(),
  updated_at   timestamptz not null default now(),
  meta         jsonb not null default '{}'::jsonb
);

do $$ begin
  if not exists (select 1 from pg_constraint where conname='users_default_team_fk') then
    alter table paperclip.users add constraint users_default_team_fk
      foreign key (default_team_id) references paperclip.teams(id) on delete set null;
  end if;
end $$;

create table if not exists paperclip.team_members (
  team_id    uuid not null references paperclip.teams(id) on delete cascade,
  user_id    uuid not null references paperclip.users(id) on delete cascade,
  role       paperclip.team_role not null default 'member',
  added_by   uuid references paperclip.users(id) on delete set null,
  created_at timestamptz not null default now(),
  meta       jsonb not null default '{}'::jsonb,
  primary key (team_id, user_id)
);
create index if not exists team_members_user_idx on paperclip.team_members(user_id);

create table if not exists paperclip.api_keys (
  id           uuid primary key default gen_random_uuid(),
  team_id      uuid not null references paperclip.teams(id) on delete cascade,
  created_by   uuid references paperclip.users(id) on delete set null,
  subject_type paperclip.api_key_subject not null default 'user',
  agent_id     uuid,
  name         text not null,
  prefix       text not null unique,
  key_hash     text not null,
  scopes       jsonb not null default '[]'::jsonb,
  scope_config jsonb,
  expires_at   timestamptz,
  last_used_at timestamptz,
  revoked_at   timestamptz,
  created_at   timestamptz not null default now(),
  meta         jsonb not null default '{}'::jsonb,
  check (subject_type <> 'agent' or agent_id is not null)
);
create index if not exists api_keys_team_idx on paperclip.api_keys(team_id) where revoked_at is null;
create index if not exists api_keys_hash_idx on paperclip.api_keys(key_hash);

create table if not exists paperclip.invitations (
  id                 uuid primary key default gen_random_uuid(),
  team_id            uuid not null references paperclip.teams(id) on delete cascade,
  role               paperclip.team_role not null default 'member',
  code_hash          text not null,
  invited_email      extensions.citext,
  allowed_join_types text not null default 'both' check (allowed_join_types in ('human','agent','both')),
  defaults_payload   jsonb,
  created_by         uuid references paperclip.users(id) on delete set null,
  expires_at         timestamptz not null,
  accepted_by        uuid references paperclip.users(id) on delete set null,
  accepted_at        timestamptz,
  revoked_at         timestamptz,
  created_at         timestamptz not null default now(),
  meta               jsonb not null default '{}'::jsonb
);
create index if not exists invitations_team_idx on paperclip.invitations(team_id);
create index if not exists invitations_code_idx on paperclip.invitations(code_hash);

create table if not exists paperclip.join_requests (
  id             uuid primary key default gen_random_uuid(),
  team_id        uuid not null references paperclip.teams(id) on delete cascade,
  requested_by   uuid references paperclip.users(id) on delete set null,
  request_type   text not null default 'human' check (request_type in ('human','agent')),
  requester_name text,
  status         paperclip.join_request_status not null default 'pending_approval',
  decided_by     uuid references paperclip.users(id) on delete set null,
  decided_at     timestamptz,
  created_at     timestamptz not null default now(),
  meta           jsonb not null default '{}'::jsonb
);
create index if not exists join_requests_team_status_idx on paperclip.join_requests(team_id, status);

create table if not exists paperclip.cli_auth (
  id                  uuid primary key default gen_random_uuid(),
  secret_hash         text not null,
  user_code_hash      text not null,
  device_name         text not null default 'paperclip CLI',
  command             text,
  requested_access    text not null default 'team',
  team_id             uuid references paperclip.teams(id) on delete set null,
  pending_key_prefix  text not null,
  pending_key_hash    text not null,
  pending_key_name    text not null,
  approved_by_user_id uuid references paperclip.users(id) on delete set null,
  api_key_id          uuid references paperclip.api_keys(id) on delete set null,
  approved_at         timestamptz,
  cancelled_at        timestamptz,
  expires_at          timestamptz not null default (now() + interval '10 minutes'),
  created_at          timestamptz not null default now(),
  updated_at          timestamptz not null default now(),
  meta                jsonb not null default '{}'::jsonb
);
create index if not exists cli_auth_secret_idx on paperclip.cli_auth(secret_hash);
create index if not exists cli_auth_user_code_idx on paperclip.cli_auth(user_code_hash);
