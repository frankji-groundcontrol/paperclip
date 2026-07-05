-- Companies + jobs (paperclip schema), per plan
-- docs/plans/2026-07-05-real-company-real-job/01-db-companies-jobs.md as amended by 06-eng-review.md.
-- Synchronous jobs (running|succeeded|failed); api-key data path via key-credential RPCs.
-- Proven by acceptance D1-D9 (api-key data path) and the real-company-real-job E-matrix.

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
drop trigger if exists trg_companies_touch on paperclip.companies;
create trigger trg_companies_touch before update on paperclip.companies
  for each row execute function paperclip_private.fn_touch_updated_at();

do $$ begin
  if not exists (select 1 from pg_type t join pg_namespace n on n.oid=t.typnamespace
                 where n.nspname='paperclip' and t.typname='job_status') then
    create type paperclip.job_status as enum ('running','succeeded','failed');
  end if;
end $$;

create table if not exists paperclip.jobs (
  id           uuid primary key default gen_random_uuid(),
  company_id   uuid not null references paperclip.companies(id) on delete cascade,
  team_id      uuid not null references paperclip.teams(id) on delete cascade,
  created_by   uuid references paperclip.users(id) on delete set null,
  subject_type paperclip.api_key_subject not null default 'user',
  agent_id     uuid,
  prompt       text not null,
  model        text not null default 'gpt-5.4-mini',
  status       paperclip.job_status not null default 'running',
  result_text  text,
  usage        jsonb not null default '{}'::jsonb,
  error        text,
  client_token text,
  created_at   timestamptz not null default now(),
  completed_at timestamptz,
  meta         jsonb not null default '{}'::jsonb
);
create index if not exists jobs_company_idx on paperclip.jobs(company_id);
create index if not exists jobs_team_idx on paperclip.jobs(team_id);
create unique index if not exists jobs_idem_uniq on paperclip.jobs(team_id, company_id, client_token)
  where client_token is not null;

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

-- Private key->principal resolver (returns scopes/scope_config/subject_type). Explicit revoke.
create or replace function paperclip_private.fn_key_principal(p_prefix text, p_key_hash text)
returns table(api_key_id uuid, team_id uuid, subject_type paperclip.api_key_subject, agent_id uuid,
              created_by uuid, scopes jsonb, scope_config jsonb)
language sql stable security definer set search_path='' as $$
  select id, team_id, subject_type, agent_id, created_by, scopes, scope_config
  from paperclip.api_keys
  where prefix = p_prefix and key_hash = p_key_hash
    and revoked_at is null and (expires_at is null or expires_at > now())
$$;
revoke execute on function paperclip_private.fn_key_principal(text,text) from public, anon, authenticated;

-- ===== Session RPCs (auth.uid(); authenticated) =====
create or replace function paperclip.create_company(p_team_id uuid, p_name text)
returns uuid language plpgsql security definer set search_path='' as $$
declare v_uid uuid := auth.uid(); v_id uuid; v_slug text;
begin
  if v_uid is null then raise exception 'not authenticated' using errcode='28000'; end if;
  if not paperclip_private.fn_has_team_role(p_team_id, v_uid, array['owner','admin','operator']::paperclip.team_role[]) then
    raise exception 'insufficient role to create company' using errcode='42501'; end if;
  v_slug := lower(regexp_replace(coalesce(p_name,'company'),'[^a-zA-Z0-9]+','-','g'))||'-'||lower(encode(extensions.gen_random_bytes(6),'hex'));
  insert into paperclip.companies(team_id, name, slug, created_by) values (p_team_id, p_name, v_slug, v_uid) returning id into v_id;
  return v_id;
end $$;

create or replace function paperclip.create_job(p_company_id uuid, p_prompt text, p_model text default 'gpt-5.4-mini', p_client_token text default null)
returns uuid language plpgsql security definer set search_path='' as $$
declare v_uid uuid := auth.uid(); v_team uuid; v_id uuid;
begin
  if v_uid is null then raise exception 'not authenticated' using errcode='28000'; end if;
  select team_id into v_team from paperclip.companies where id=p_company_id;
  if v_team is null or not paperclip_private.fn_has_team_role(v_team, v_uid, array['owner','admin','operator','member']::paperclip.team_role[]) then
    raise exception 'not a member of company team' using errcode='42501'; end if;
  if p_client_token is not null then
    select id into v_id from paperclip.jobs where team_id=v_team and company_id=p_company_id and client_token=p_client_token;
    if v_id is not null then return v_id; end if;
  end if;
  begin
    insert into paperclip.jobs(company_id, team_id, created_by, subject_type, prompt, model, status, client_token)
    values (p_company_id, v_team, v_uid, 'user', p_prompt, p_model, 'running', p_client_token) returning id into v_id;
  exception when unique_violation then
    select id into v_id from paperclip.jobs where team_id=v_team and company_id=p_company_id and client_token=p_client_token;
  end;
  return v_id;
end $$;

create or replace function paperclip.complete_job(p_job_id uuid, p_result text, p_usage jsonb default '{}'::jsonb)
returns boolean language plpgsql security definer set search_path='' as $$
declare v_uid uuid := auth.uid(); v_team uuid;
begin
  if v_uid is null then raise exception 'not authenticated' using errcode='28000'; end if;
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
  if v_uid is null then raise exception 'not authenticated' using errcode='28000'; end if;
  select team_id into v_team from paperclip.jobs where id=p_job_id;
  if v_team is null or not paperclip_private.fn_has_team_role(v_team, v_uid, array['owner','admin','operator','member']::paperclip.team_role[]) then
    raise exception 'not a member of job team' using errcode='42501'; end if;
  update paperclip.jobs set status='failed', error=p_error, completed_at=now() where id=p_job_id and status='running'; return found;
end $$;

-- ===== API-key-credential RPCs (anon + authenticated). MVP: valid team key = full company/job write. =====
-- TODO(scopes): enforce k.scopes/scope_config/subject_type here for scoped/agent keys (future layer).
create or replace function paperclip.create_company_with_key(p_prefix text, p_key_hash text, p_name text)
returns uuid language plpgsql security definer set search_path='' as $$
declare k record; v_id uuid; v_slug text;
begin
  select * into k from paperclip_private.fn_key_principal(p_prefix, p_key_hash);
  if k.team_id is null then raise exception 'invalid api key' using errcode='28000'; end if;
  v_slug := lower(regexp_replace(coalesce(p_name,'company'),'[^a-zA-Z0-9]+','-','g'))||'-'||lower(encode(extensions.gen_random_bytes(6),'hex'));
  insert into paperclip.companies(team_id, name, slug, created_by) values (k.team_id, p_name, v_slug, k.created_by) returning id into v_id;
  return v_id;
end $$;

create or replace function paperclip.create_job_with_key(p_prefix text, p_key_hash text, p_company_id uuid, p_prompt text, p_model text default 'gpt-5.4-mini', p_client_token text default null)
returns uuid language plpgsql security definer set search_path='' as $$
declare k record; v_team uuid; v_id uuid;
begin
  select * into k from paperclip_private.fn_key_principal(p_prefix, p_key_hash);
  if k.team_id is null then raise exception 'invalid api key' using errcode='28000'; end if;
  select team_id into v_team from paperclip.companies where id=p_company_id;
  if v_team is null or v_team <> k.team_id then raise exception 'company not in key team' using errcode='42501'; end if;
  if p_client_token is not null then
    select id into v_id from paperclip.jobs where team_id=k.team_id and company_id=p_company_id and client_token=p_client_token;
    if v_id is not null then return v_id; end if;
  end if;
  begin
    insert into paperclip.jobs(company_id, team_id, created_by, subject_type, agent_id, prompt, model, status, client_token)
    values (p_company_id, k.team_id, k.created_by, k.subject_type, k.agent_id, p_prompt, p_model, 'running', p_client_token) returning id into v_id;
  exception when unique_violation then
    select id into v_id from paperclip.jobs where team_id=k.team_id and company_id=p_company_id and client_token=p_client_token;
  end;
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
  update paperclip.jobs set status='failed', error=p_error, completed_at=now() where id=p_job_id and status='running'; return found;
end $$;

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

-- Safety reaper for jobs abandoned mid-request (server maintenance)
create or replace function paperclip.fail_stale_jobs(p_older_than interval default interval '10 minutes')
returns integer language plpgsql security definer set search_path='' as $$
declare n int;
begin
  update paperclip.jobs set status='failed', error='abandoned_timeout', completed_at=now()
    where status='running' and created_at < now() - p_older_than;
  get diagnostics n = row_count; return n;
end $$;

-- ===== Grants (least privilege) =====
revoke execute on function paperclip.create_company(uuid,text), paperclip.create_job(uuid,text,text,text),
  paperclip.complete_job(uuid,text,jsonb), paperclip.fail_job(uuid,text) from public, anon;
grant  execute on function paperclip.create_company(uuid,text), paperclip.create_job(uuid,text,text,text),
  paperclip.complete_job(uuid,text,jsonb), paperclip.fail_job(uuid,text) to authenticated;
revoke execute on function paperclip.create_company_with_key(text,text,text),
  paperclip.create_job_with_key(text,text,uuid,text,text,text), paperclip.complete_job_with_key(text,text,uuid,text,jsonb),
  paperclip.fail_job_with_key(text,text,uuid,text), paperclip.list_companies_with_key(text,text),
  paperclip.list_jobs_with_key(text,text,uuid), paperclip.fail_stale_jobs(interval) from public;
grant  execute on function paperclip.create_company_with_key(text,text,text),
  paperclip.create_job_with_key(text,text,uuid,text,text,text), paperclip.complete_job_with_key(text,text,uuid,text,jsonb),
  paperclip.fail_job_with_key(text,text,uuid,text), paperclip.list_companies_with_key(text,text),
  paperclip.list_jobs_with_key(text,text,uuid), paperclip.fail_stale_jobs(interval) to anon, authenticated;
