-- Phase 02 write-path RPCs: goals/projects/issues/comments + activity_log helper.
-- Phase 02/06/07 write-path RPCs: goals/projects/issues/comments + cost + budgets + heartbeat runs.
-- Every mutating RPC appends to paperclip.activity_log. Session + api-key paths (forge-proof *_with_key).

-- ===== activity_log insert (internal helper, called by mutating RPCs) =====
create or replace function paperclip_private.log_activity(
  p_team uuid, p_company uuid, p_actor_type text, p_actor_id text, p_action text,
  p_target_type text, p_target_id text, p_details jsonb default '{}'::jsonb)
returns void language plpgsql security definer set search_path='' as $$
begin
  insert into paperclip.activity_log(team_id, company_id, actor_type, actor_id, action, target_type, target_id, details)
  values (p_team, p_company, p_actor_type, p_actor_id, p_action, p_target_type, p_target_id, p_details);
end $$;
revoke execute on function paperclip_private.log_activity(uuid,uuid,text,text,text,text,text,jsonb) from public, anon, authenticated;

-- ===== goals =====
create or replace function paperclip.create_goal(
  p_company_id uuid, p_title text, p_level text default 'task', p_parent_id uuid default null,
  p_owner_agent_id uuid default null, p_description text default null)
returns jsonb language plpgsql security definer set search_path='' as $$
declare v_uid uuid := auth.uid(); v_team uuid; v_id uuid;
begin
  if v_uid is null then raise exception 'not authenticated' using errcode='28000'; end if;
  select team_id into v_team from paperclip.companies where id=p_company_id;
  if v_team is null or not paperclip_private.fn_has_team_role(v_team, v_uid, array['owner','admin','operator']::paperclip.team_role[]) then
    raise exception 'insufficient role' using errcode='42501'; end if;
  if p_parent_id is not null and not exists(select 1 from paperclip.goals where id=p_parent_id and company_id=p_company_id) then
    raise exception 'parent goal not in same company' using errcode='42501'; end if;
  if p_owner_agent_id is not null and not exists(select 1 from paperclip.agents where id=p_owner_agent_id and company_id=p_company_id) then
    raise exception 'owner agent not in same company' using errcode='42501'; end if;
  insert into paperclip.goals(company_id, team_id, title, level, parent_id, owner_agent_id, description)
    values (p_company_id, v_team, p_title, p_level, p_parent_id, p_owner_agent_id, p_description)
    returning id into v_id;
  perform paperclip_private.log_activity(v_team, p_company_id, 'user', v_uid::text, 'goal.created', 'goal', v_id::text);
  return jsonb_build_object('goalId', v_id);
end $$;

create or replace function paperclip.create_goal_with_key(
  p_prefix text, p_key_hash text, p_company_id uuid, p_title text, p_level text default 'task',
  p_parent_id uuid default null, p_owner_agent_id uuid default null, p_description text default null)
returns jsonb language plpgsql security definer set search_path='' as $$
declare k record; v_id uuid;
begin
  select * into k from paperclip_private.fn_key_principal(p_prefix, p_key_hash);
  if k.team_id is null then raise exception 'invalid api key' using errcode='28000'; end if;
  if k.subject_type='agent' then raise exception 'agent keys cannot create goals' using errcode='42501'; end if;
  if not exists(select 1 from paperclip.companies where id=p_company_id and team_id=k.team_id) then
    raise exception 'company not in key team' using errcode='42501'; end if;
  insert into paperclip.goals(company_id, team_id, title, level, parent_id, owner_agent_id, description)
    values (p_company_id, k.team_id, p_title, p_level, p_parent_id, p_owner_agent_id, p_description)
    returning id into v_id;
  perform paperclip_private.log_activity(k.team_id, p_company_id, 'api_key', k.created_by::text, 'goal.created', 'goal', v_id::text);
  return jsonb_build_object('goalId', v_id);
end $$;

-- ===== projects =====
create or replace function paperclip.create_project(
  p_company_id uuid, p_name text, p_goal_id uuid default null, p_lead_agent_id uuid default null,
  p_target_date date default null, p_description text default null, p_env jsonb default '{}'::jsonb)
returns jsonb language plpgsql security definer set search_path='' as $$
declare v_uid uuid := auth.uid(); v_team uuid; v_id uuid;
begin
  if v_uid is null then raise exception 'not authenticated' using errcode='28000'; end if;
  select team_id into v_team from paperclip.companies where id=p_company_id;
  if v_team is null or not paperclip_private.fn_has_team_role(v_team, v_uid, array['owner','admin','operator']::paperclip.team_role[]) then
    raise exception 'insufficient role' using errcode='42501'; end if;
  if p_goal_id is not null and not exists(select 1 from paperclip.goals where id=p_goal_id and company_id=p_company_id) then
    raise exception 'goal not in same company' using errcode='42501'; end if;
  if p_lead_agent_id is not null and not exists(select 1 from paperclip.agents where id=p_lead_agent_id and company_id=p_company_id) then
    raise exception 'lead agent not in same company' using errcode='42501'; end if;
  insert into paperclip.projects(company_id, team_id, name, goal_id, lead_agent_id, target_date, description, env)
    values (p_company_id, v_team, p_name, p_goal_id, p_lead_agent_id, p_target_date, p_description, p_env)
    returning id into v_id;
  perform paperclip_private.log_activity(v_team, p_company_id, 'user', v_uid::text, 'project.created', 'project', v_id::text);
  return jsonb_build_object('projectId', v_id);
end $$;

create or replace function paperclip.create_project_with_key(
  p_prefix text, p_key_hash text, p_company_id uuid, p_name text, p_goal_id uuid default null,
  p_lead_agent_id uuid default null, p_target_date date default null, p_description text default null,
  p_env jsonb default '{}'::jsonb)
returns jsonb language plpgsql security definer set search_path='' as $$
declare k record; v_id uuid;
begin
  select * into k from paperclip_private.fn_key_principal(p_prefix, p_key_hash);
  if k.team_id is null then raise exception 'invalid api key' using errcode='28000'; end if;
  if k.subject_type='agent' then raise exception 'agent keys cannot create projects' using errcode='42501'; end if;
  if not exists(select 1 from paperclip.companies where id=p_company_id and team_id=k.team_id) then
    raise exception 'company not in key team' using errcode='42501'; end if;
  insert into paperclip.projects(company_id, team_id, name, goal_id, lead_agent_id, target_date, description, env)
    values (p_company_id, k.team_id, p_name, p_goal_id, p_lead_agent_id, p_target_date, p_description, p_env)
    returning id into v_id;
  perform paperclip_private.log_activity(k.team_id, p_company_id, 'api_key', k.created_by::text, 'project.created', 'project', v_id::text);
  return jsonb_build_object('projectId', v_id);
end $$;

-- ===== issues =====
create or replace function paperclip.create_issue(
  p_company_id uuid, p_title text, p_parent_id uuid default null, p_project_id uuid default null,
  p_goal_id uuid default null, p_assignee_agent_id uuid default null, p_priority text default 'medium',
  p_description text default null, p_status text default 'backlog')
returns jsonb language plpgsql security definer set search_path='' as $$
declare v_uid uuid := auth.uid(); v_team uuid; v_id uuid;
begin
  if v_uid is null then raise exception 'not authenticated' using errcode='28000'; end if;
  select team_id into v_team from paperclip.companies where id=p_company_id;
  if v_team is null or not paperclip_private.fn_has_team_role(v_team, v_uid, array['owner','admin','operator','member']::paperclip.team_role[]) then
    raise exception 'insufficient role' using errcode='42501'; end if;
  if p_parent_id is not null and not exists(select 1 from paperclip.issues where id=p_parent_id and company_id=p_company_id) then
    raise exception 'parent issue not in same company' using errcode='42501'; end if;
  if p_project_id is not null and not exists(select 1 from paperclip.projects where id=p_project_id and company_id=p_company_id) then
    raise exception 'project not in same company' using errcode='42501'; end if;
  if p_goal_id is not null and not exists(select 1 from paperclip.goals where id=p_goal_id and company_id=p_company_id) then
    raise exception 'goal not in same company' using errcode='42501'; end if;
  if p_assignee_agent_id is not null and not exists(select 1 from paperclip.agents where id=p_assignee_agent_id and company_id=p_company_id) then
    raise exception 'assignee agent not in same company' using errcode='42501'; end if;
  insert into paperclip.issues(company_id, team_id, project_id, goal_id, parent_id, title, description, status, priority, assignee_agent_id, created_by_user_id)
    values (p_company_id, v_team, p_project_id, p_goal_id, p_parent_id, p_title, p_description, p_status, p_priority, p_assignee_agent_id, v_uid)
    returning id into v_id;
  perform paperclip_private.log_activity(v_team, p_company_id, 'user', v_uid::text, 'issue.created', 'issue', v_id::text);
  return jsonb_build_object('issueId', v_id);
end $$;

create or replace function paperclip.create_issue_with_key(
  p_prefix text, p_key_hash text, p_company_id uuid, p_title text, p_parent_id uuid default null,
  p_project_id uuid default null, p_goal_id uuid default null, p_assignee_agent_id uuid default null,
  p_priority text default 'medium', p_description text default null, p_status text default 'backlog')
returns jsonb language plpgsql security definer set search_path='' as $$
declare k record; v_id uuid;
begin
  select * into k from paperclip_private.fn_key_principal(p_prefix, p_key_hash);
  if k.team_id is null then raise exception 'invalid api key' using errcode='28000'; end if;
  if not exists(select 1 from paperclip.companies where id=p_company_id and team_id=k.team_id) then
    raise exception 'company not in key team' using errcode='42501'; end if;
  if k.subject_type='agent' then
    if not paperclip_private.fn_agent_active(k.agent_id, k.team_id) then raise exception 'acting agent not active' using errcode='42501'; end if;
  end if;
  insert into paperclip.issues(company_id, team_id, project_id, goal_id, parent_id, title, description, status, priority, assignee_agent_id, created_by_agent_id, created_by_user_id)
    values (p_company_id, k.team_id, p_project_id, p_goal_id, p_parent_id, p_title, p_description, p_status, p_priority, p_assignee_agent_id,
            (case when k.subject_type='agent' then k.agent_id else null end)::uuid,
            (case when k.subject_type='user' then k.created_by::text else null end)::text)
    returning id into v_id;
  perform paperclip_private.log_activity(k.team_id, p_company_id, k.subject_type, coalesce(k.agent_id, k.created_by)::text, 'issue.created', 'issue', v_id::text);
  return jsonb_build_object('issueId', v_id);
end $$;

create or replace function paperclip.update_issue(p_issue_id uuid, p_status text default null, p_title text default null,
  p_assignee_agent_id uuid default null, p_priority text default null, p_description text default null)
returns jsonb language plpgsql security definer set search_path='' as $$
declare v_uid uuid := auth.uid(); v_iss paperclip.issues%rowtype;
begin
  if v_uid is null then raise exception 'not authenticated' using errcode='28000'; end if;
  select * into v_iss from paperclip.issues where id=p_issue_id for update;
  if v_iss.id is null then raise exception 'issue not found' using errcode='P0002'; end if;
  if not paperclip_private.fn_has_team_role(v_iss.team_id, v_uid, array['owner','admin','operator','member']::paperclip.team_role[]) then
    raise exception 'insufficient role' using errcode='42501'; end if;
  update paperclip.issues set
    status=coalesce(p_status, v_iss.status), title=coalesce(p_title, v_iss.title),
    assignee_agent_id=coalesce(p_assignee_agent_id, v_iss.assignee_agent_id),
    priority=coalesce(p_priority, v_iss.priority), description=coalesce(p_description, v_iss.description),
    updated_at=now()
    where id=p_issue_id;
  perform paperclip_private.log_activity(v_iss.team_id, v_iss.company_id, 'user', v_uid::text, 'issue.updated', 'issue', v_iss.id::text,
    jsonb_build_object('status', coalesce(p_status,v_iss.status)));
  return jsonb_build_object('issueId', v_iss.id);
end $$;

-- ===== issue comments =====
create or replace function paperclip.add_issue_comment(p_issue_id uuid, p_body text)
returns uuid language plpgsql security definer set search_path='' as $$
declare v_uid uuid := auth.uid(); v_iss paperclip.issues%rowtype; v_id uuid;
begin
  if v_uid is null then raise exception 'not authenticated' using errcode='28000'; end if;
  select * into v_iss from paperclip.issues where id=p_issue_id;
  if v_iss.id is null then raise exception 'issue not found' using errcode='P0002'; end if;
  if btrim(coalesce(p_body,''))='' then raise exception 'body required' using errcode='22023'; end if;
  insert into paperclip.issue_comments(issue_id, company_id, team_id, author_user_id, body)
    values (p_issue_id, v_iss.company_id, v_iss.team_id, v_uid, p_body)
    returning id into v_id;
  perform paperclip_private.log_activity(v_iss.team_id, v_iss.company_id, 'user', v_uid::text, 'issue_comment.created', 'issue_comment', v_id::text,
    jsonb_build_object('issueId', p_issue_id::text));
  return v_id;
end $$;

create or replace function paperclip.add_issue_comment_with_key(p_prefix text, p_key_hash text, p_issue_id uuid, p_body text)
returns uuid language plpgsql security definer set search_path='' as $$
declare k record; v_iss paperclip.issues%rowtype; v_id uuid;
begin
  select * into k from paperclip_private.fn_key_principal(p_prefix, p_key_hash);
  if k.team_id is null then raise exception 'invalid api key' using errcode='28000'; end if;
  select * into v_iss from paperclip.issues where id=p_issue_id;
  if v_iss.id is null then raise exception 'issue not found' using errcode='P0002'; end if;
  if v_iss.team_id <> k.team_id then raise exception 'issue not in key team' using errcode='42501'; end if;
  if k.subject_type='agent' and not paperclip_private.fn_agent_active(k.agent_id, k.team_id) then
    raise exception 'acting agent not active' using errcode='42501'; end if;
  if btrim(coalesce(p_body,''))='' then raise exception 'body required' using errcode='22023'; end if;
  insert into paperclip.issue_comments(issue_id, company_id, team_id, author_agent_id, author_user_id, body)
    values (p_issue_id, v_iss.company_id, v_iss.team_id,
            (case when k.subject_type='agent' then k.agent_id else null end)::uuid,
            (case when k.subject_type='user' then k.created_by::text else null end)::text,
            p_body)
    returning id into v_id;
  perform paperclip_private.log_activity(v_iss.team_id, v_iss.company_id, k.subject_type, coalesce(k.agent_id, k.created_by)::text,
    'issue_comment.created', 'issue_comment', v_id::text, jsonb_build_object('issueId', p_issue_id::text));
  return v_id;
end $$;


revoke execute on function
  paperclip.create_goal(uuid,text,text,uuid,uuid,text),
  paperclip.create_goal_with_key(text,text,uuid,text,text,uuid,uuid,text),
  paperclip.create_project(uuid,text,uuid,uuid,date,text,jsonb),
  paperclip.create_project_with_key(text,text,uuid,text,uuid,uuid,date,text,jsonb),
  paperclip.create_issue(uuid,text,uuid,uuid,uuid,uuid,text,text),
  paperclip.create_issue_with_key(text,text,uuid,text,uuid,uuid,uuid,uuid,text,text,text),
  paperclip.update_issue(uuid,text,text,uuid,text,text),
  paperclip.add_issue_comment(uuid,text),
  paperclip.add_issue_comment_with_key(text,text,uuid,text) from public, anon;
grant execute on function
  paperclip.create_goal(uuid,text,text,uuid,uuid,text),
  paperclip.create_goal_with_key(text,text,uuid,text,text,uuid,uuid,text),
  paperclip.create_project(uuid,text,uuid,uuid,date,text,jsonb),
  paperclip.create_project_with_key(text,text,uuid,text,uuid,uuid,date,text,jsonb),
  paperclip.create_issue(uuid,text,uuid,uuid,uuid,uuid,text,text),
  paperclip.create_issue_with_key(text,text,uuid,text,uuid,uuid,uuid,uuid,text,text,text),
  paperclip.update_issue(uuid,text,text,uuid,text,text),
  paperclip.add_issue_comment(uuid,text),
  paperclip.add_issue_comment_with_key(text,text,uuid,text) to anon, authenticated;
