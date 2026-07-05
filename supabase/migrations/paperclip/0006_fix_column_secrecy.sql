-- SECURITY FIX: a table-wide GRANT SELECT overrides a per-column REVOKE, so
-- key_hash / code_hash stayed readable on direct base-table reads. Correct
-- pattern: revoke the blanket grant, then grant SELECT only on non-secret
-- columns. (my_* views already exclude the secrets; this closes the base table.)
do $$
declare cols text;
begin
  revoke select on paperclip.api_keys from authenticated;
  select string_agg(quote_ident(column_name), ', ' order by ordinal_position) into cols
    from information_schema.columns
   where table_schema='paperclip' and table_name='api_keys' and column_name <> 'key_hash';
  execute format('grant select (%s) on paperclip.api_keys to authenticated', cols);

  revoke select on paperclip.invitations from authenticated;
  select string_agg(quote_ident(column_name), ', ' order by ordinal_position) into cols
    from information_schema.columns
   where table_schema='paperclip' and table_name='invitations' and column_name <> 'code_hash';
  execute format('grant select (%s) on paperclip.invitations to authenticated', cols);
end $$;
