-- Private SECURITY DEFINER helpers (owner=postgres, bypassrls) — see 03-rpcs.md
create or replace function paperclip_private.fn_touch_updated_at()
returns trigger language plpgsql security definer set search_path = '' as $$
begin new.updated_at := now(); return new; end $$;

create or replace function paperclip_private.fn_user_team_ids(p_uid uuid)
returns setof uuid language sql stable security definer set search_path = '' as $$
  select tm.team_id from paperclip.team_members tm where tm.user_id = p_uid
$$;

create or replace function paperclip_private.fn_team_role(p_team uuid, p_uid uuid)
returns paperclip.team_role language sql stable security definer set search_path = '' as $$
  select tm.role from paperclip.team_members tm where tm.team_id = p_team and tm.user_id = p_uid
$$;

create or replace function paperclip_private.fn_has_team_role(p_team uuid, p_uid uuid, p_roles paperclip.team_role[])
returns boolean language sql stable security definer set search_path = '' as $$
  select exists (select 1 from paperclip.team_members tm
     where tm.team_id = p_team and tm.user_id = p_uid and tm.role = any(p_roles))
$$;

create or replace function paperclip_private.fn_bootstrap_auth_user(p_uid uuid, p_email text)
returns uuid language plpgsql security definer set search_path = '' as $$
declare v_team uuid; v_slug text;
begin
  insert into paperclip.users (id, email, display_name)
  values (p_uid, p_email, split_part(coalesce(p_email,'user'),'@',1))
  on conflict (id) do update set email = excluded.email;

  select default_team_id into v_team from paperclip.users where id = p_uid;
  if v_team is not null then return v_team; end if;

  v_slug := 'u-' || left(replace(p_uid::text,'-',''), 12);
  insert into paperclip.teams (name, slug, is_personal, created_by)
  values (coalesce(split_part(p_email,'@',1),'Personal') || '''s Team', v_slug, true, p_uid)
  on conflict (slug) do nothing
  returning id into v_team;
  if v_team is null then
    select id into v_team from paperclip.teams where slug = v_slug;
  end if;

  insert into paperclip.team_members (team_id, user_id, role, added_by)
  values (v_team, p_uid, 'owner', p_uid)
  on conflict (team_id, user_id) do nothing;

  update paperclip.users set default_team_id = v_team where id = p_uid;
  return v_team;
end $$;

create or replace function paperclip_private.fn_handle_new_auth_user()
returns trigger language plpgsql security definer set search_path = '' as $$
begin
  perform paperclip_private.fn_bootstrap_auth_user(new.id, new.email);
  return new;
end $$;

-- updated_at touch triggers (fn now exists)
drop trigger if exists trg_users_touch on paperclip.users;
create trigger trg_users_touch    before update on paperclip.users    for each row execute function paperclip_private.fn_touch_updated_at();
drop trigger if exists trg_teams_touch on paperclip.teams;
create trigger trg_teams_touch    before update on paperclip.teams    for each row execute function paperclip_private.fn_touch_updated_at();
drop trigger if exists trg_cli_auth_touch on paperclip.cli_auth;
create trigger trg_cli_auth_touch before update on paperclip.cli_auth for each row execute function paperclip_private.fn_touch_updated_at();
