-- SECURITY FIX: CREATE FUNCTION grants EXECUTE to PUBLIC by default, so anon
-- could call every RPC (they self-guard on auth.uid(), but least-privilege
-- says revoke). Reset all paperclip function grants, then re-grant precisely:
--   * authenticated  -> the full authenticated surface
--   * anon           -> ONLY the 3 pre-auth server RPCs
do $$
declare
  r record;
  preauth text[] := array['resolve_api_key','cli_start_device_login','cli_poll_device_login'];
begin
  for r in
    select p.oid::regprocedure::text as sig, p.proname as nm
    from pg_proc p
    join pg_namespace n on n.oid = p.pronamespace
    where n.nspname = 'paperclip'
  loop
    execute format('revoke execute on function %s from public', r.sig);
    execute format('revoke execute on function %s from anon', r.sig);
    execute format('revoke execute on function %s from authenticated', r.sig);
    if r.nm = any(preauth) then
      execute format('grant execute on function %s to anon', r.sig);
    else
      execute format('grant execute on function %s to authenticated', r.sig);
    end if;
  end loop;
end $$;
