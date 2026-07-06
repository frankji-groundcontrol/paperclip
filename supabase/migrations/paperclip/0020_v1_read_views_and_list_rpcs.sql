-- Phase 02/06/07 read path: security_invoker views + list/get RPCs over the 0019 control-plane tables.
-- Gives the live backend (JobService session/api-key dispatch) a uniform read path.

-- ===== security_invoker views (team-scoped read for authenticated users) =====
create or replace view paperclip.my_goals with (security_invoker=true) as
  select * from paperclip.goals where team_id in (select paperclip_private.fn_user_team_ids(auth.uid()));
create or replace view paperclip.my_projects with (security_invoker=true) as
  select * from paperclip.projects where team_id in (select paperclip_private.fn_user_team_ids(auth.uid()));
create or replace view paperclip.my_issues with (security_invoker=true) as
  select * from paperclip.issues where team_id in (select paperclip_private.fn_user_team_ids(auth.uid()));
create or replace view paperclip.my_issue_comments with (security_invoker=true) as
  select * from paperclip.issue_comments where team_id in (select paperclip_private.fn_user_team_ids(auth.uid()));
create or replace view paperclip.my_activity with (security_invoker=true) as
  select * from paperclip.activity_log where team_id in (select paperclip_private.fn_user_team_ids(auth.uid()));
create or replace view paperclip.my_cost_events with (security_invoker=true) as
  select * from paperclip.cost_events where team_id in (select paperclip_private.fn_user_team_ids(auth.uid()));
create or replace view paperclip.my_heartbeat_runs with (security_invoker=true) as
  select * from paperclip.heartbeat_runs where team_id in (select paperclip_private.fn_user_team_ids(auth.uid()));
create or replace view paperclip.my_environments with (security_invoker=true) as
  select * from paperclip.environments where team_id in (select paperclip_private.fn_user_team_ids(auth.uid()));
create or replace view paperclip.my_routines with (security_invoker=true) as
  select * from paperclip.routines where team_id in (select paperclip_private.fn_user_team_ids(auth.uid()));
create or replace view paperclip.my_workspaces with (security_invoker=true) as
  select * from paperclip.execution_workspaces where team_id in (select paperclip_private.fn_user_team_ids(auth.uid()));
grant select on paperclip.my_goals, paperclip.my_projects, paperclip.my_issues, paperclip.my_issue_comments,
  paperclip.my_activity, paperclip.my_cost_events, paperclip.my_heartbeat_runs, paperclip.my_environments,
  paperclip.my_routines, paperclip.my_workspaces to authenticated;

-- ===== list_*_with_key RPCs (api-key path, forge-proof) + session list via views =====
-- Generic pattern: re-resolve principal, scope by team, return rows for a company.

create or replace function paperclip.list_goals_with_key(p_prefix text, p_key_hash text, p_company_id uuid)
returns setof paperclip.goals language sql stable security definer set search_path='' as $$
  select g.* from paperclip.goals g, paperclip_private.fn_key_principal(p_prefix, p_key_hash) k
  where k.team_id is not null and g.team_id = k.team_id and g.company_id = p_company_id
  order by g.created_at desc;
$$;
create or replace function paperclip.list_projects_with_key(p_prefix text, p_key_hash text, p_company_id uuid)
returns setof paperclip.projects language sql stable security definer set search_path='' as $$
  select p.* from paperclip.projects p, paperclip_private.fn_key_principal(p_prefix, p_key_hash) k
  where k.team_id is not null and p.team_id = k.team_id and p.company_id = p_company_id
  order by p.created_at desc;
$$;
create or replace function paperclip.list_issues_with_key(p_prefix text, p_key_hash text, p_company_id uuid, p_status text default null)
returns setof paperclip.issues language sql stable security definer set search_path='' as $$
  select i.* from paperclip.issues i, paperclip_private.fn_key_principal(p_prefix, p_key_hash) k
  where k.team_id is not null and i.team_id = k.team_id and i.company_id = p_company_id
    and (p_status is null or i.status = p_status)
  order by i.created_at desc;
$$;
create or replace function paperclip.list_issue_comments_with_key(p_prefix text, p_key_hash text, p_issue_id uuid)
returns setof paperclip.issue_comments language sql stable security definer set search_path='' as $$
  select c.* from paperclip.issue_comments c, paperclip_private.fn_key_principal(p_prefix, p_key_hash) k
  where k.team_id is not null and c.team_id = k.team_id and c.issue_id = p_issue_id
  order by c.created_at asc;
$$;
create or replace function paperclip.list_activity_with_key(p_prefix text, p_key_hash text, p_company_id uuid, p_limit int default 100)
returns setof paperclip.activity_log language sql stable security definer set search_path='' as $$
  select a.* from paperclip.activity_log a, paperclip_private.fn_key_principal(p_prefix, p_key_hash) k
  where k.team_id is not null and a.team_id = k.team_id and a.company_id = p_company_id
  order by a.created_at desc limit p_limit;
$$;
create or replace function paperclip.list_cost_events_with_key(p_prefix text, p_key_hash text, p_company_id uuid, p_limit int default 100)
returns setof paperclip.cost_events language sql stable security definer set search_path='' as $$
  select c.* from paperclip.cost_events c, paperclip_private.fn_key_principal(p_prefix, p_key_hash) k
  where k.team_id is not null and c.team_id = k.team_id and c.company_id = p_company_id
  order by c.occurred_at desc limit p_limit;
$$;
create or replace function paperclip.list_heartbeat_runs_with_key(p_prefix text, p_key_hash text, p_company_id uuid, p_agent_id uuid default null, p_limit int default 50)
returns setof paperclip.heartbeat_runs language sql stable security definer set search_path='' as $$
  select r.* from paperclip.heartbeat_runs r, paperclip_private.fn_key_principal(p_prefix, p_key_hash) k
  where k.team_id is not null and r.team_id = k.team_id and r.company_id = p_company_id
    and (p_agent_id is null or r.agent_id = p_agent_id)
  order by r.created_at desc limit p_limit;
$$;

revoke execute on function
  paperclip.list_goals_with_key(text,text,uuid),
  paperclip.list_projects_with_key(text,text,uuid),
  paperclip.list_issues_with_key(text,text,uuid,text),
  paperclip.list_issue_comments_with_key(text,text,uuid),
  paperclip.list_activity_with_key(text,text,uuid,int),
  paperclip.list_cost_events_with_key(text,text,uuid,int),
  paperclip.list_heartbeat_runs_with_key(text,text,uuid,uuid,int) from public;
grant execute on function
  paperclip.list_goals_with_key(text,text,uuid),
  paperclip.list_projects_with_key(text,text,uuid),
  paperclip.list_issues_with_key(text,text,uuid,text),
  paperclip.list_issue_comments_with_key(text,text,uuid),
  paperclip.list_activity_with_key(text,text,uuid,int),
  paperclip.list_cost_events_with_key(text,text,uuid,int),
  paperclip.list_heartbeat_runs_with_key(text,text,uuid,uuid,int) to anon, authenticated;
