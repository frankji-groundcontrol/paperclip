-- FIX: 0009's defense-in-depth revoke stripped EXECUTE on the two private
-- helpers that the RLS policies and security_invoker my_* views invoke as the
-- CALLING (authenticated) user (fn_user_team_ids, fn_has_team_role). Restore
-- EXECUTE for those two only. They stay unreachable directly (authenticated has
-- no USAGE on paperclip_private, and the schema is not PostgREST-exposed); only
-- the pre-compiled RLS policies / views reference them.
grant execute on function paperclip_private.fn_user_team_ids(uuid) to authenticated;
grant execute on function paperclip_private.fn_has_team_role(uuid, uuid, paperclip.team_role[]) to authenticated;
