# 01 — Schema DDL (`paperclip` + `paperclip_private`)

All objects live in `paperclip` (exposed, RLS) or `paperclip_private` (internal, no grants). **Never `public`.**
Extensions available in this project under `extensions`: `pgcrypto` (`digest`, `gen_random_bytes`), `citext`. Reference them fully-qualified.

Migration `0001_paperclip_schema` (DDL only; RLS in `02`, functions in `03`).

```sql
-- Schemas -------------------------------------------------------------------
create schema if not exists paperclip;
create schema if not exists paperclip_private;

-- Lock down default privileges: no automatic grants to anon/authenticated.
revoke all on schema paperclip_private from anon, authenticated, public;
grant usage on schema paperclip to anon, authenticated;   -- objects still RLS/GRANT gated

-- Enums ---------------------------------------------------------------------
create type paperclip.team_role as enum ('owner','admin','operator','viewer','member');
create type paperclip.api_key_subject as enum ('user','agent');
create type paperclip.join_request_status as enum ('pending_approval','approved','rejected');

-- USERS (1:1 with auth.users) ----------------------------------------------
create table paperclip.users (
  id             uuid primary key references auth.users(id) on delete cascade,
  email          extensions.citext,
  display_name   text,
  system_role    text not null default 'user'      -- 'user' | 'instance_admin'
                   check (system_role in ('user','instance_admin')),
  default_team_id uuid,                             -- FK added after teams exists
  created_at     timestamptz not null default now(),
  updated_at     timestamptz not null default now(),
  meta           jsonb not null default '{}'::jsonb
);

-- TEAMS (access-control group; == original company-membership layer) --------
create table paperclip.teams (
  id           uuid primary key default gen_random_uuid(),
  name         text not null,
  slug         text not null unique,
  is_personal  boolean not null default false,
  created_by   uuid references paperclip.users(id) on delete set null,
  created_at   timestamptz not null default now(),
  updated_at   timestamptz not null default now(),
  meta         jsonb not null default '{}'::jsonb
);
alter table paperclip.users
  add constraint users_default_team_fk
  foreign key (default_team_id) references paperclip.teams(id) on delete set null;

-- TEAM MEMBERS --------------------------------------------------------------
create table paperclip.team_members (
  team_id    uuid not null references paperclip.teams(id) on delete cascade,
  user_id    uuid not null references paperclip.users(id) on delete cascade,
  role       paperclip.team_role not null default 'member',
  added_by   uuid references paperclip.users(id) on delete set null,
  created_at timestamptz not null default now(),
  meta       jsonb not null default '{}'::jsonb,
  primary key (team_id, user_id)
);
create index team_members_user_idx on paperclip.team_members(user_id);

-- API KEYS (unifies board_api_keys + agent_api_keys) ------------------------
create table paperclip.api_keys (
  id           uuid primary key default gen_random_uuid(),
  team_id      uuid not null references paperclip.teams(id) on delete cascade,
  created_by   uuid references paperclip.users(id) on delete set null,
  subject_type paperclip.api_key_subject not null default 'user',
  agent_id     uuid,                              -- set when subject_type='agent'
  name         text not null,
  prefix       text not null unique,              -- 'paperclip_<8hex>' — fast lookup (improvement)
  key_hash     text not null,                     -- sha256(full_key) hex; server computes
  scopes       jsonb not null default '[]'::jsonb,
  scope_config jsonb,                             -- parity: agent_api_keys.scope_config
  expires_at   timestamptz,
  last_used_at timestamptz,
  revoked_at   timestamptz,
  created_at   timestamptz not null default now(),
  meta         jsonb not null default '{}'::jsonb,
  check (subject_type <> 'agent' or agent_id is not null)
);
create index api_keys_team_idx on paperclip.api_keys(team_id) where revoked_at is null;
create index api_keys_hash_idx on paperclip.api_keys(key_hash);

-- INVITATIONS (team+role scoped; hashed code) -------------------------------
create table paperclip.invitations (
  id                uuid primary key default gen_random_uuid(),
  team_id           uuid not null references paperclip.teams(id) on delete cascade,
  role              paperclip.team_role not null default 'member',   -- invite AS a role (improvement)
  code_hash         text not null,                                   -- sha256(invite_code) hex
  invited_email     extensions.citext,
  allowed_join_types text not null default 'both'                    -- parity: human|agent|both
                       check (allowed_join_types in ('human','agent','both')),
  defaults_payload  jsonb,                                           -- parity
  created_by        uuid references paperclip.users(id) on delete set null,
  expires_at        timestamptz not null,
  accepted_by       uuid references paperclip.users(id) on delete set null,
  accepted_at       timestamptz,
  revoked_at        timestamptz,
  created_at        timestamptz not null default now(),
  meta              jsonb not null default '{}'::jsonb
);
create index invitations_team_idx on paperclip.invitations(team_id);

-- JOIN REQUESTS -------------------------------------------------------------
create table paperclip.join_requests (
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
create index join_requests_team_status_idx on paperclip.join_requests(team_id, status);

-- CLI / MCP DEVICE-LOGIN (parity: cli_auth_challenges; no plaintext at rest) -
create table paperclip.cli_auth (
  id                  uuid primary key default gen_random_uuid(),
  secret_hash         text not null,                 -- sha256(challenge_secret); CLI polls with the secret
  user_code_hash      text not null,                 -- sha256(user_code); user types it in browser
  device_name         text not null default 'paperclip CLI',
  command             text,
  requested_access    text not null default 'team',  -- parity: 'board' -> 'team'
  team_id             uuid references paperclip.teams(id) on delete set null,
  pending_key_hash    text not null,                 -- sha256(pending key CLI generated locally)
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
create index cli_auth_secret_idx on paperclip.cli_auth(secret_hash);
create index cli_auth_user_code_idx on paperclip.cli_auth(user_code_hash);

-- updated_at touch trigger (function in 03) ---------------------------------
create trigger trg_users_touch      before update on paperclip.users       for each row execute function paperclip_private.fn_touch_updated_at();
create trigger trg_teams_touch      before update on paperclip.teams       for each row execute function paperclip_private.fn_touch_updated_at();
create trigger trg_cli_auth_touch   before update on paperclip.cli_auth    for each row execute function paperclip_private.fn_touch_updated_at();
```

## Notes / decisions
- **`prefix` is unique + indexed** so api-key resolution is `where prefix=$1 and key_hash=$2` (no full-table hash scan). Original scans by hash; this is an efficiency improvement while staying hash-only.
- **`api_keys.team_id` is required** — every key is team-scoped (parity with agent keys' company scope; board keys become team keys on the user's default team).
- **`join_requests` is included** for parity even though the primary invite path is direct invitations; the reimplementation preserves both.
- **`system_role`** covers instance-admin; a dedicated `paperclip.instance_admins` table is an alternative (see `09`) — decide at review.
- All `citext` columns rely on the `citext` type in `extensions`; if unavailable, fall back to `text` + `lower()` unique indexes.
