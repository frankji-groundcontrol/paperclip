-- Phase 06: approval revision/resubmit + comments RPCs (canonical approval lifecycle).
-- Schema prerequisites from 0014 (revision_requested status) and 0017 (approval_comments,
-- decision_note, revision_requested_at/by). These RPCs close the approval governance loop.

-- ===== request_revision: board only, pending -> revision_requested =====
create or replace function paperclip.request_revision(p_approval_id uuid, p_note text default null)
returns paperclip.approval_status language plpgsql security definer set search_path='' as $$
declare v_uid uuid := auth.uid(); v_ap paperclip.approvals%rowtype;
begin
  if v_uid is null then raise exception 'not authenticated' using errcode='28000'; end if;
  select * into v_ap from paperclip.approvals where id=p_approval_id for update;
  if v_ap.id is null then raise exception 'approval not found' using errcode='P0002'; end if;
  if not paperclip_private.fn_has_team_role(v_ap.team_id, v_uid, array['owner','admin']::paperclip.team_role[]) then
    raise exception 'board only may request revision' using errcode='42501'; end if;
  if v_ap.status <> 'pending' then raise exception 'approval is not pending' using errcode='22023'; end if;
  update paperclip.approvals
    set status='revision_requested', decision_note=p_note,
        revision_requested_at=now(), revision_requested_by=v_uid
    where id=p_approval_id;
  return 'revision_requested';
end $$;

-- ===== resubmit: requester only, revision_requested -> pending =====
create or replace function paperclip.resubmit_approval(p_approval_id uuid, p_note text default null)
returns paperclip.approval_status language plpgsql security definer set search_path='' as $$
declare v_uid uuid := auth.uid(); v_ap paperclip.approvals%rowtype;
begin
  if v_uid is null then raise exception 'not authenticated' using errcode='28000'; end if;
  select * into v_ap from paperclip.approvals where id=p_approval_id for update;
  if v_ap.id is null then raise exception 'approval not found' using errcode='P0002'; end if;
  if v_ap.requested_by is distinct from v_uid then
    raise exception 'only the requester may resubmit' using errcode='42501'; end if;
  if v_ap.status <> 'revision_requested' then raise exception 'approval is not in revision' using errcode='22023'; end if;
  update paperclip.approvals
    set status='pending', decision_note=p_note
    where id=p_approval_id;
  return 'pending';
end $$;

-- ===== add_approval_comment: any team member =====
create or replace function paperclip.add_approval_comment(p_approval_id uuid, p_body text)
returns uuid language plpgsql security definer set search_path='' as $$
declare v_uid uuid := auth.uid(); v_team uuid; v_id uuid;
begin
  if v_uid is null then raise exception 'not authenticated' using errcode='28000'; end if;
  if p_body is null or btrim(p_body) = '' then raise exception 'comment body required' using errcode='22023'; end if;
  select team_id into v_team from paperclip.approvals where id=p_approval_id;
  if v_team is null then raise exception 'approval not found' using errcode='P0002'; end if;
  if not (v_team in (select paperclip_private.fn_user_team_ids(auth.uid()))) then
    raise exception 'not a team member' using errcode='42501'; end if;
  insert into paperclip.approval_comments(approval_id, team_id, author_id, body)
    values (p_approval_id, v_team, v_uid, p_body)
    returning id into v_id;
  return v_id;
end $$;

-- ===== list_approval_comments: team member read =====
create or replace function paperclip.list_approval_comments(p_approval_id uuid)
returns table(id uuid, author_id uuid, body text, created_at timestamptz)
language sql stable security definer set search_path='' as $$
  select c.id, c.author_id, c.body, c.created_at
  from paperclip.approval_comments c
  join paperclip.approvals a on a.id = c.approval_id
  where c.approval_id = p_approval_id
    and a.team_id in (select paperclip_private.fn_user_team_ids(auth.uid()))
  order by c.created_at asc;
$$;

revoke execute on function
  paperclip.request_revision(uuid,text),
  paperclip.resubmit_approval(uuid,text),
  paperclip.add_approval_comment(uuid,text),
  paperclip.list_approval_comments(uuid) from public, anon;
grant execute on function
  paperclip.request_revision(uuid,text),
  paperclip.resubmit_approval(uuid,text),
  paperclip.add_approval_comment(uuid,text),
  paperclip.list_approval_comments(uuid) to authenticated;
