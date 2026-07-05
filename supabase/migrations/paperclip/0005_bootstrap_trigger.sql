-- Bootstrap trigger: new auth.users -> paperclip profile + personal team + owner membership
drop trigger if exists trg_paperclip_new_user on auth.users;
create trigger trg_paperclip_new_user
  after insert on auth.users
  for each row execute function paperclip_private.fn_handle_new_auth_user();
