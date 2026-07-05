-- Agent-key scope enforcement on the api-key data path (eng-review TODO(scopes)).
-- Backward compatible: an empty/absent scopes array = full access (all existing keys keep working);
-- a key minted with explicit scopes is restricted to them. Capabilities:
--   companies:write, companies:read, jobs:write, jobs:read ; '*' = all. write implies read.
-- Proven by scopes acceptance (unscoped=full; companies:read can list not write; write key can both).

create or replace function paperclip_private.fn_key_allows(p_scopes jsonb, p_cap text)
returns boolean language sql immutable set search_path='' as $$
  select
    p_scopes is null
    or jsonb_typeof(p_scopes) <> 'array'
    or jsonb_array_length(p_scopes) = 0        -- unscoped key -> full access (MVP default)
    or p_scopes ? '*'
    or p_scopes ? p_cap
    or (p_cap like '%:read' and p_scopes ? (split_part(p_cap, ':', 1) || ':write'))  -- write implies read
$$;
revoke execute on function paperclip_private.fn_key_allows(jsonb,text) from public, anon, authenticated;

create or replace function paperclip.create_company_with_key(p_prefix text, p_key_hash text, p_name text)
returns uuid language plpgsql security definer set search_path='' as $$
declare k record; v_id uuid; v_slug text;
begin
  select * into k from paperclip_private.fn_key_principal(p_prefix, p_key_hash);
  if k.team_id is null then raise exception 'invalid api key' using errcode='28000'; end if;
  if not paperclip_private.fn_key_allows(k.scopes, 'companies:write') then
    raise exception 'api key lacks scope companies:write' using errcode='42501'; end if;
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
  if not paperclip_private.fn_key_allows(k.scopes, 'jobs:write') then
    raise exception 'api key lacks scope jobs:write' using errcode='42501'; end if;
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
  if not paperclip_private.fn_key_allows(k.scopes, 'jobs:write') then
    raise exception 'api key lacks scope jobs:write' using errcode='42501'; end if;
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
  if not paperclip_private.fn_key_allows(k.scopes, 'jobs:write') then
    raise exception 'api key lacks scope jobs:write' using errcode='42501'; end if;
  select team_id into v_team from paperclip.jobs where id=p_job_id;
  if v_team is null or v_team <> k.team_id then raise exception 'job not in key team' using errcode='42501'; end if;
  update paperclip.jobs set status='failed', error=p_error, completed_at=now() where id=p_job_id and status='running'; return found;
end $$;

create or replace function paperclip.list_companies_with_key(p_prefix text, p_key_hash text)
returns setof paperclip.companies language plpgsql security definer set search_path='' as $$
declare k record; begin
  select * into k from paperclip_private.fn_key_principal(p_prefix, p_key_hash);
  if k.team_id is null then raise exception 'invalid api key' using errcode='28000'; end if;
  if not paperclip_private.fn_key_allows(k.scopes, 'companies:read') then
    raise exception 'api key lacks scope companies:read' using errcode='42501'; end if;
  return query select * from paperclip.companies where team_id = k.team_id order by created_at desc;
end $$;

create or replace function paperclip.list_jobs_with_key(p_prefix text, p_key_hash text, p_company_id uuid default null)
returns setof paperclip.jobs language plpgsql security definer set search_path='' as $$
declare k record; begin
  select * into k from paperclip_private.fn_key_principal(p_prefix, p_key_hash);
  if k.team_id is null then raise exception 'invalid api key' using errcode='28000'; end if;
  if not paperclip_private.fn_key_allows(k.scopes, 'jobs:read') then
    raise exception 'api key lacks scope jobs:read' using errcode='42501'; end if;
  return query select * from paperclip.jobs where team_id = k.team_id and (p_company_id is null or company_id = p_company_id) order by created_at desc;
end $$;
