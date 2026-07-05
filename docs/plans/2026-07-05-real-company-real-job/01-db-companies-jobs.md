# 01 — DB: companies + jobs (paperclip schema)

Two new tables in `paperclip`, RLS-scoped to team membership, mutated only through
`SECURITY DEFINER` RPCs. **Two data paths**, both service-role-free:
- **Session path** (human/frontend): RPCs use `auth.uid()`; the broker calls them with the user JWT.
- **API-key path** (agent/CLI/MCP): `*_with_key` RPCs take `(p_prefix, p_key_hash, …)` and
  **re-resolve the key + re-authorize internally** — a caller without a valid key can forge nothing.

Migration `paperclip_0011_companies_jobs`.

## Tables
```sql
create table if not exists paperclip.companies (
  id          uuid primary key default gen_random_uuid(),
  team_id     uuid not null references paperclip.teams(id) on delete cascade,
  name        text not null,
  slug        text not null unique,
  created_by  uuid references paperclip.users(id) on delete set null,
  created_at  timestamptz not null default now(),
  updated_at  timestamptz not null default now(),
  meta        jsonb not null default '{}'::jsonb
);
create index if not exists companies_team_idx on paperclip.companies(team_id);

do $$ begin
  if not exists (select 1 from pg_type t join pg_namespace n on n.oid=t.typnamespace
                 where n.nspname='paperclip' and t.typname='job_status') then
    create type paperclip.job_status as enum ('queued','running','succeeded','failed');
  end if;
end $$;

create table if not exists paperclip.jobs (
  id           uuid primary key default gen_random_uuid(),
  company_id   uuid not null references paperclip.companies(id) on delete cascade,
  team_id      uuid not null references paperclip.teams(id) on delete cascade,   -- denormalized for RLS
  created_by   uuid references paperclip.users(id) on delete set null,
  subject_type paperclip.api_key_subject not null default 'user',
  agent_id     uuid,
  prompt       text not null,
  model        text not null default 'gpt-5.4-mini',
  status       paperclip.job_status not null default 'queued',
  result_text  text,
  usage        jsonb not null default '{}'::jsonb,
  error        text,
  created_at   timestamptz not null default now(),
  started_at   timestamptz,
  completed_at timestamptz,
  meta         jsonb not null default '{}'::jsonb
);
create index if not exists jobs_company_idx on paperclip.jobs(company_id);
create index if not exists jobs_team_idx on paperclip.jobs(team_id);
```

## RLS + views + grants (ENABLE, not FORCE — F1)
```sql
alter table paperclip.companies enable row level security;
alter table paperclip.jobs      enable row level security;

drop policy if exists companies_member_read on paperclip.companies;
create policy companies_member_read on paperclip.companies for select to authenticated
  using ( team_id in (select paperclip_private.fn_user_team_ids(auth.uid())) );
drop policy if exists jobs_member_read on paperclip.jobs;
create policy jobs_member_read on paperclip.jobs for select to authenticated
  using ( team_id in (select paperclip_private.fn_user_team_ids(auth.uid())) );

create or replace view paperclip.my_companies with (security_invoker=true) as
  select c.* from paperclip.companies c
  where c.team_id in (select paperclip_private.fn_user_team_ids(auth.uid()));
create or replace view paperclip.my_jobs with (security_invoker=true) as
  select j.* from paperclip.jobs j
  where j.team_id in (select paperclip_private.fn_user_team_ids(auth.uid()));

grant select on paperclip.companies, paperclip.jobs to authenticated;
grant select on paperclip.my_companies, paperclip.my_jobs to authenticated;
```

## Private helper: resolve a key to a principal (no side effects)
```sql
create or replace function paperclip_private.fn_key_principal(p_prefix text, p_key_hash text)
returns table(api_key_id uuid, team_id uuid, subject_type paperclip.api_key_subject, agent_id uuid, created_by uuid)
language sql stable security definer set search_path='' as $$
  select id, team_id, subject_type, agent_id, created_by
  from paperclip.api_keys
  where prefix = p_prefix and key_hash = p_key_hash
    and revoked_at is null and (expires_at is null or expires_at > now())
$$;
-- executable indirectly only; keep it out of anon/authenticated direct reach (0009 pattern).
```

## Session RPCs (auth.uid(); granted to authenticated)
```sql
create or replace function paperclip.create_company(p_team_id uuid, p_name text)
returns uuid language plpgsql security definer set search_path='' as $$
declare v_uid uuid := auth.uid(); v_id uuid; v_slug text;
begin
  if not paperclip_private.fn_has_team_role(p_team_id, v_uid, array['owner','admin','operator']::paperclip.team_role[]) then
    raise exception 'insufficient role to create company' using errcode='42501'; end if;
  v_slug := lower(regexp_replace(coalesce(p_name,'company'),'[^a-zA-Z0-9]+','-','g'))||'-'||lower(encode(extensions.gen_random_bytes(3),'hex'));
  insert into paperclip.companies(team_id, name, slug, created_by) values (p_team_id, p_name, v_slug, v_uid) returning id into v_id;
  return v_id;
end $$;

create or replace function paperclip.create_job(p_company_id uuid, p_prompt text, p_model text default 'gpt-5.4-mini')
returns uuid language plpgsql security definer set search_path='' as $$
declare v_uid uuid := auth.uid(); v_team uuid; v_id uuid;
begin
  select team_id into v_team from paperclip.companies where id=p_company_id;
  if v_team is null or not paperclip_private.fn_has_team_role(v_team, v_uid, array['owner','admin','operator','member']::paperclip.team_role[]) then
    raise exception 'not a member of company team' using errcode='42501'; end if;
  insert into paperclip.jobs(company_id, team_id, created_by, subject_type, prompt, model, status, started_at)
  values (p_company_id, v_team, v_uid, 'user', p_prompt, p_model, 'running', now()) returning id into v_id;
  return v_id;
end $$;

create or replace function paperclip.complete_job(p_job_id uuid, p_result text, p_usage jsonb default '{}'::jsonb)
returns boolean language plpgsql security definer set search_path='' as $$
declare v_uid uuid := auth.uid(); v_team uuid;
begin
  select team_id into v_team from paperclip.jobs where id=p_job_id;
  if v_team is null or not paperclip_private.fn_has_team_role(v_team, v_uid, array['owner','admin','operator','member']::paperclip.team_role[]) then
    raise exception 'not a member of job team' using errcode='42501'; end if;
  update paperclip.jobs set status='succeeded', result_text=p_result, usage=coalesce(p_usage,'{}'::jsonb), completed_at=now()
    where id=p_job_id and status='running'; return found;
end $$;

create or replace function paperclip.fail_job(p_job_id uuid, p_error text)
returns boolean language plpgsql security definer set search_path='' as $$
declare v_uid uuid := auth.uid(); v_team uuid;
begin
  select team_id into v_team from paperclip.jobs where id=p_job_id;
  if v_team is null or not paperclip_private.fn_has_team_role(v_team, v_uid, array['owner','admin','operator','member']::paperclip.team_role[]) then
    raise exception 'not a member of job team' using errcode='42501'; end if;
  update paperclip.jobs set status='failed', error=p_error, completed_at=now() where id=p_job_id and status in ('queued','running'); return found;
end $$;
```

## API-key-credential RPCs (take the key; granted to anon + authenticated)
```sql
create or replace function paperclip.create_company_with_key(p_prefix text, p_key_hash text, p_name text)
returns uuid language plpgsql security definer set search_path='' as $$
declare k record; v_id uuid; v_slug text;
begin
  select * into k from paperclip_private.fn_key_principal(p_prefix, p_key_hash);
  if k.team_id is null then raise exception 'invalid api key' using errcode='28000'; end if;
  v_slug := lower(regexp_replace(coalesce(p_name,'company'),'[^a-zA-Z0-9]+','-','g'))||'-'||lower(encode(extensions.gen_random_bytes(3),'hex'));
  insert into paperclip.companies(team_id, name, slug, created_by) values (k.team_id, p_name, v_slug, k.created_by) returning id into v_id;
  return v_id;
end $$;

create or replace function paperclip.create_job_with_key(p_prefix text, p_key_hash text, p_company_id uuid, p_prompt text, p_model text default 'gpt-5.4-mini')
returns uuid language plpgsql security definer set search_path='' as $$
declare k record; v_team uuid; v_id uuid;
begin
  select * into k from paperclip_private.fn_key_principal(p_prefix, p_key_hash);
  if k.team_id is null then raise exception 'invalid api key' using errcode='28000'; end if;
  select team_id into v_team from paperclip.companies where id=p_company_id;
  if v_team is null or v_team <> k.team_id then raise exception 'company not in key team' using errcode='42501'; end if;
  insert into paperclip.jobs(company_id, team_id, created_by, subject_type, agent_id, prompt, model, status, started_at)
  values (p_company_id, k.team_id, k.created_by, k.subject_type, k.agent_id, p_prompt, p_model, 'running', now()) returning id into v_id;
  return v_id;
end $$;

create or replace function paperclip.complete_job_with_key(p_prefix text, p_key_hash text, p_job_id uuid, p_result text, p_usage jsonb default '{}'::jsonb)
returns boolean language plpgsql security definer set search_path='' as $$
declare k record; v_team uuid;
begin
  select * into k from paperclip_private.fn_key_principal(p_prefix, p_key_hash);
  if k.team_id is null then raise exception 'invalid api key' using errcode='28000'; end if;
  select team_id into v_team from paperclip.jobs where id=p_job_id;
  if v_team is null or v_team <> k.team_id then raise exception 'job not in key team' using errcode='42501'; end if;
  update paperclip.jobs set status='succeeded', result_text=p_result, usage=coalesce(p_usage,'{}'::jsonb), completed_at=now()
    where id=p_job_id and status='running'; return found;
end $$;

create or replace function paperclip.fail_job_with_key(p_prefix text, p_key_hash text, p_job_id uuid, p_error text)
returns boolean language plpgsql security definer set search_path='' as $$
declare k record; v_team uuid;
begin
  select * into k from paperclip_private.fn_key_principal(p_prefix, p_key_hash);
  if k.team_id is null then raise exception 'invalid api key' using errcode='28000'; end if;
  select team_id into v_team from paperclip.jobs where id=p_job_id;
  if v_team is null or v_team <> k.team_id then raise exception 'job not in key team' using errcode='42501'; end if;
  update paperclip.jobs set status='failed', error=p_error, completed_at=now() where id=p_job_id and status in ('queued','running'); return found;
end $$;

-- list on the api-key path (read): jobs/companies for the key's team
create or replace function paperclip.list_companies_with_key(p_prefix text, p_key_hash text)
returns setof paperclip.companies language plpgsql security definer set search_path='' as $$
declare k record; begin
  select * into k from paperclip_private.fn_key_principal(p_prefix, p_key_hash);
  if k.team_id is null then raise exception 'invalid api key' using errcode='28000'; end if;
  return query select * from paperclip.companies where team_id = k.team_id order by created_at desc;
end $$;

create or replace function paperclip.list_jobs_with_key(p_prefix text, p_key_hash text, p_company_id uuid default null)
returns setof paperclip.jobs language plpgsql security definer set search_path='' as $$
declare k record; begin
  select * into k from paperclip_private.fn_key_principal(p_prefix, p_key_hash);
  if k.team_id is null then raise exception 'invalid api key' using errcode='28000'; end if;
  return query select * from paperclip.jobs where team_id = k.team_id and (p_company_id is null or company_id = p_company_id) order by created_at desc;
end $$;
```

## Grants (least privilege; mirror 0007)
```sql
-- session RPCs -> authenticated only
revoke execute on function paperclip.create_company(uuid,text), paperclip.create_job(uuid,text,text),
  paperclip.complete_job(uuid,text,jsonb), paperclip.fail_job(uuid,text) from public, anon;
grant  execute on function paperclip.create_company(uuid,text), paperclip.create_job(uuid,text,text),
  paperclip.complete_job(uuid,text,jsonb), paperclip.fail_job(uuid,text) to authenticated;
-- key-credential RPCs -> anon + authenticated (server calls them with the anon key)
revoke execute on function paperclip.create_company_with_key(text,text,text),
  paperclip.create_job_with_key(text,text,uuid,text,text), paperclip.complete_job_with_key(text,text,uuid,text,jsonb),
  paperclip.fail_job_with_key(text,text,uuid,text), paperclip.list_companies_with_key(text,text),
  paperclip.list_jobs_with_key(text,text,uuid) from public;
grant  execute on function paperclip.create_company_with_key(text,text,text),
  paperclip.create_job_with_key(text,text,uuid,text,text), paperclip.complete_job_with_key(text,text,uuid,text,jsonb),
  paperclip.fail_job_with_key(text,text,uuid,text), paperclip.list_companies_with_key(text,text),
  paperclip.list_jobs_with_key(text,text,uuid) to anon, authenticated;
-- fn_key_principal stays out of direct anon/authenticated reach (no grant; schema USAGE already revoked).
```

## Verification (must pass)
- Non-member cannot `select` a company/job (RLS 0 rows).
- `create_job_with_key` on a company **not in the key's team** → `42501`.
- `complete_job_with_key` with a **wrong/revoked key** → error; with a valid key on its own job → updates.
- Session `create_job` by a non-member → `42501`.
