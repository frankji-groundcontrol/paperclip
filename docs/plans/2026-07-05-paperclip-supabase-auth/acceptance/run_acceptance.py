#!/usr/bin/env python3
"""Paperclip custom-schema auth — reproducible real-user acceptance matrix.

Runs entirely over the real wire: GoTrue password login (genuine JWTs) + PostgREST
against the `paperclip` schema. No mocks, no service-role key. Proves A1, A3-A13.

Usage:
    export SUPABASE_ANON_KEY=<public anon key>     # not committed
    python3 run_acceptance.py                      # reads ../../../.. /test_users.json

Requires test_users.json (gitignored) at the repo root with project_url, password,
and the four accounts. Exits non-zero if any scenario fails.
"""
import json, os, sys, hashlib, secrets, uuid, urllib.request, urllib.error, pathlib

ANON = os.environ.get("SUPABASE_ANON_KEY")
if not ANON:
    sys.exit("set SUPABASE_ANON_KEY (public anon/publishable key) in the environment")

# locate test_users.json at repo root (four levels up from this file)
root = pathlib.Path(__file__).resolve().parents[4]
cfg = json.load(open(root / "test_users.json"))
BASE = cfg["project_url"].rstrip("/")
PW = cfg["password"]
EMAIL = {u["email"].split("_")[0]: u["email"] for u in cfg["users"]}   # testuser1 -> full email
UID = {u["email"].split("_")[0]: u["auth_uid"] for u in cfg["users"]}


def http(method, url, headers, body=None):
    data = json.dumps(body).encode() if body is not None else None
    req = urllib.request.Request(url, data=data, method=method)
    for k, v in headers.items():
        req.add_header(k, v)
    try:
        with urllib.request.urlopen(req) as r:
            raw = r.read().decode()
            return r.status, (json.loads(raw) if raw else None)
    except urllib.error.HTTPError as e:
        raw = e.read().decode()
        try:
            return e.code, json.loads(raw)
        except Exception:
            return e.code, raw


def login(user):
    st, d = http("POST", f"{BASE}/auth/v1/token?grant_type=password",
                 {"apikey": ANON, "Content-Type": "application/json"},
                 {"email": EMAIL[user], "password": PW})
    if st != 200 or "access_token" not in (d or {}):
        sys.exit(f"login failed for {user}: {st} {d}")
    return d["access_token"]


JWT = {}


def rpc(fn, body, who=None):
    tok = JWT[who] if who else ANON
    return http("POST", f"{BASE}/rest/v1/rpc/{fn}",
                {"apikey": ANON, "Authorization": f"Bearer {tok}", "Content-Type": "application/json",
                 "Content-Profile": "paperclip", "Accept-Profile": "paperclip"}, body)


def get(path, who):
    return http("GET", f"{BASE}/rest/v1/{path}",
                {"apikey": ANON, "Authorization": f"Bearer {JWT[who]}", "Accept-Profile": "paperclip"}, None)


def sha(s):
    return hashlib.sha256(s.encode()).hexdigest()


def mint():
    pref = "pc_" + secrets.token_hex(4)
    full = f"paperclip_{pref}_{secrets.token_urlsafe(32)}"
    return pref, full, sha(full)


results = []
def check(cid, desc, ok, detail=""):
    results.append((cid, ok))
    print(f"[{'PASS' if ok else 'FAIL'}] {cid}: {desc}" + (f"  -- {detail}" if detail else ""))


def main():
    for u in ("testuser1", "testuser2", "testadmin1", "testuser3"):
        JWT[u] = login(u)

    st, data = rpc("create_team", {"p_name": "Paperclip Test Team"}, "testadmin1")
    team = data if isinstance(data, str) else (data[0] if isinstance(data, list) and data else None)
    check("A3", "admin creates team", st == 200 and bool(team), f"status={st}")

    st, d = rpc("add_team_member", {"p_team": team, "p_user": UID["testuser2"], "p_role": "viewer"}, "testadmin1")
    check("A13a", "admin adds testuser2 as viewer", st == 200 and d is True)

    code = secrets.token_urlsafe(24)
    st, d = rpc("create_invitation", {"p_team_id": team, "p_role": "admin", "p_code_hash": sha(code),
                                      "p_invited_email": EMAIL["testuser3"], "p_allowed_join_types": "both"}, "testadmin1")
    check("A4", "invite testuser3 as admin", st == 200 and bool(d))

    st, d = rpc("accept_invitation", {"p_code_hash": sha(code)}, "testuser3")
    row = d[0] if isinstance(d, list) and d else {}
    check("A5", "testuser3 accepts -> admin", st == 200 and row.get("team_id") == team and row.get("role") == "admin", f"{st} {row}")
    st, d = get(f"my_team_members?team_id=eq.{team}&user_id=eq.{UID['testuser3']}&select=role", "testuser3")
    check("A5b", "testuser3 sees own admin membership", st == 200 and d and d[0].get("role") == "admin")

    st, d = get("my_teams?select=id", "testuser1")
    check("A6a", "testuser1 non-member can't see team", st == 200 and not any(r.get("id") == team for r in (d or [])))
    st, d = get(f"teams?id=eq.{team}&select=id", "testuser1")
    check("A6b", "RLS: direct teams read 0 rows", st == 200 and d == [])
    st, d = get(f"my_team_members?team_id=eq.{team}&select=user_id", "testuser1")
    check("A6c", "non-member can't enumerate members", st == 200 and d == [])

    pref, full, kh = mint()
    st, d = rpc("create_api_key", {"p_team_id": team, "p_name": "CI user key", "p_prefix": pref, "p_key_hash": kh}, "testadmin1")
    key_id = d if isinstance(d, str) else None
    check("A7", "admin mints user API key", st == 200 and bool(key_id))
    st, rows = get(f"my_api_keys?id=eq.{key_id}&select=*", "testadmin1")
    ok7b = isinstance(rows, list) and len(rows) == 1 and "key_hash" not in rows[0]
    check("A7b", "my_api_keys excludes key_hash", ok7b, f"status={st} rows={str(rows)[:80]}")

    st, d = rpc("resolve_api_key", {"p_prefix": pref, "p_key_hash": kh}, None)
    r = d[0] if isinstance(d, list) and d else {}
    check("A8", "resolve_api_key -> team/subject", r.get("api_key_id") == key_id and r.get("team_id") == team and r.get("subject_type") == "user")
    st, d = rpc("resolve_api_key", {"p_prefix": pref, "p_key_hash": sha("wrong")}, None)
    check("A8b", "resolve wrong hash -> empty", d == [])

    agent_id = str(uuid.UUID(bytes=secrets.token_bytes(16)))
    pa, fa, ha = mint()
    cfgj = {"pipelines": ["read", "write"], "secrets": ["read"]}
    st, d = rpc("create_api_key", {"p_team_id": team, "p_name": "agent key", "p_prefix": pa, "p_key_hash": ha,
                                   "p_subject_type": "agent", "p_agent_id": agent_id, "p_scope_config": cfgj}, "testadmin1")
    ak = d if isinstance(d, str) else None
    st2, d2 = rpc("resolve_api_key", {"p_prefix": pa, "p_key_hash": ha}, None)
    ra = d2[0] if isinstance(d2, list) and d2 else {}
    check("A9", "agent key resolves w/ agent_id+scope_config",
          bool(ak) and ra.get("subject_type") == "agent" and ra.get("agent_id") == agent_id and ra.get("scope_config") == cfgj)

    ds = secrets.token_urlsafe(24); uc = secrets.token_urlsafe(6); cp, cf, ch = mint()
    st, d = rpc("cli_start_device_login", {"p_secret_hash": sha(ds), "p_user_code_hash": sha(uc),
                "p_pending_key_prefix": cp, "p_pending_key_hash": ch, "p_pending_key_name": "cli key"}, None)
    started = st == 200 and bool(d)
    st, p1 = rpc("cli_poll_device_login", {"p_secret_hash": sha(ds)}, None)
    st, ap = rpc("cli_approve_device_login", {"p_user_code_hash": sha(uc)}, "testuser1")
    st, p2 = rpc("cli_poll_device_login", {"p_secret_hash": sha(ds)}, None)
    st, rk = rpc("resolve_api_key", {"p_prefix": cp, "p_key_hash": ch}, None)
    p1s = p1[0].get("status") if isinstance(p1, list) and p1 else None
    p2r = p2[0] if isinstance(p2, list) and p2 else {}
    check("A10", "CLI device-login end-to-end",
          started and p1s == "pending" and ap is True and p2r.get("status") == "approved"
          and p2r.get("prefix") == cp and p2r.get("user_id") == UID["testuser1"] and bool(rk))

    st, d = get("api_keys?select=key_hash&limit=1", "testadmin1")
    check("A11a", "authenticated cannot select key_hash", st != 200)
    st, d = rpc("create_team", {"p_name": "anon fail"}, None)
    check("A11b", "anon cannot create_team", st != 200)

    st, d = rpc("revoke_api_key", {"p_id": key_id}, "testadmin1")
    st2, d2 = rpc("resolve_api_key", {"p_prefix": pref, "p_key_hash": kh}, None)
    check("A12", "revoke -> resolve empty", d is True and d2 == [])

    st, d = rpc("create_invitation", {"p_team_id": team, "p_role": "member", "p_code_hash": sha(secrets.token_urlsafe(16))}, "testuser3")
    gm = st == 200
    st, d = rpc("create_invitation", {"p_team_id": team, "p_role": "owner", "p_code_hash": sha(secrets.token_urlsafe(16))}, "testuser3")
    check("A13b", "admin invites member not owner", gm and st != 200)
    pv, fv, hv = mint()
    st, d = rpc("create_api_key", {"p_team_id": team, "p_name": "viewer key", "p_prefix": pv, "p_key_hash": hv}, "testuser2")
    check("A13c", "viewer cannot mint key", st != 200)

    rpc("bootstrap_current_user", {}, "testuser2"); rpc("bootstrap_current_user", {}, "testuser2")
    st, d = get("my_teams?is_personal=eq.true&select=id", "testuser2")
    check("A1", "bootstrap idempotent (1 personal team)", st == 200 and isinstance(d, list) and len(d) == 1)

    # --- Authorization attack matrix (must all be BLOCKED). team members here:
    #     testadmin1=owner, testuser3=admin, testuser2=viewer, testuser1=non-member.
    # ATK1 (critical): a non-member mints a key bound to the victim team via device-login
    ds = secrets.token_urlsafe(16); uc = secrets.token_urlsafe(6); ap, af, ah = mint()
    rpc("cli_start_device_login", {"p_secret_hash": sha(ds), "p_user_code_hash": sha(uc),
        "p_pending_key_prefix": ap, "p_pending_key_hash": ah, "p_pending_key_name": "evil", "p_team_id": team}, None)
    st, d = rpc("cli_approve_device_login", {"p_user_code_hash": sha(uc)}, "testuser1")
    st2, rk = rpc("resolve_api_key", {"p_prefix": ap, "p_key_hash": ah}, None)
    check("ATK1", "non-member cannot mint cross-team device key", st != 200 and rk == [], f"approve={st}")
    # ATK2/3/4: an admin cannot demote/modify/remove the owner
    st, d = rpc("add_team_member", {"p_team": team, "p_user": UID["testadmin1"], "p_role": "member"}, "testuser3")
    check("ATK2", "admin cannot demote owner via add_team_member", st != 200, f"status={st}")
    st, d = rpc("update_team_member_role", {"p_team": team, "p_user": UID["testadmin1"], "p_role": "member"}, "testuser3")
    check("ATK3", "admin cannot change owner role", st != 200, f"status={st}")
    st, d = rpc("remove_team_member", {"p_team": team, "p_user": UID["testadmin1"]}, "testuser3")
    check("ATK4", "admin cannot remove owner", st != 200, f"status={st}")
    # ATK5: an admin cannot grant 'owner' by approving a join request
    st, jr = rpc("create_join_request", {"p_team_id": team, "p_request_type": "human", "p_requester_name": "u1"}, "testuser1")
    jrid = jr if isinstance(jr, str) else None
    st, d = rpc("decide_join_request", {"p_id": jrid, "p_approve": True, "p_role": "owner"}, "testuser3")
    check("ATK5", "admin cannot grant owner via join-request", st != 200, f"status={st}")
    rpc("decide_join_request", {"p_id": jrid, "p_approve": False}, "testuser3")  # cleanup

    npass = sum(1 for _, ok in results if ok)
    fails = [c for c, ok in results if not ok]
    print(f"\n=== {npass}/{len(results)} passed ===")
    print("FAILURES:", fails if fails else "none")
    # best-effort cleanup of the freshly-created team so re-runs stay clean is left to the
    # operator (delete via SQL); leaving artifacts is harmless.
    sys.exit(0 if not fails else 1)


if __name__ == "__main__":
    main()
