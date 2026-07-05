# 06 — Registration + Invitations

## Registration (test: create testuser3)
1. Client → `POST /api/auth/register {email,password}`.
2. Server → GoTrue `POST /auth/v1/signup` with **anon** key.
3. `auth.users` insert fires `trg_paperclip_new_user` → `fn_bootstrap_auth_user` → creates `paperclip.users` row + personal team (`is_personal`, owner membership) + sets `default_team_id`.
4. If email confirmation is **on**, the account exists but can't password-login until confirmed. Handling (no service role):
   - Preferred: project has confirmations **off** in this dev project (testuser1/2/admin1 are already confirmed).
   - If on: server surfaces "confirm email" state; for the automated test we verify the row + team were bootstrapped (trigger fires regardless of confirmation) and, if login is blocked, document it. **We do not use the service role to force-confirm.**
5. Server returns an app session (if login succeeds) or a "pending confirmation" status.

## Invitations (test: invite testuser3 to the team as **admin**)
1. Admin (e.g. testadmin1) → `POST /api/teams/:id/invitations {role:'admin', invited_email:'testuser3...'}`.
2. Server mints a random invite `code`, hashes it, calls `create_invitation(team_id, 'admin', code_hash, invited_email, ...)`. Returns `code` (+ link) to the inviter **once**.
3. Invitee (testuser3, logged in) → `POST /api/invitations/accept {code}`.
4. Server hashes `code`, calls `accept_invitation(code_hash)` (with testuser3's JWT). RPC validates (not expired/revoked/used, email matches) → inserts `team_members(team_id, testuser3, 'admin')` → marks accepted.
5. Verify: `my_team_members` for the team now lists testuser3 with role `admin`; RLS lets testuser3 see the team.

## Join-requests (parity, secondary path)
- `create_join_request(team_id, request_type)` → pending; team admin `decide_join_request(id, approve|reject)` → on approve inserts membership. Gated by `joins:approve` (== admin+). Included for parity; primary path is direct invitations.
