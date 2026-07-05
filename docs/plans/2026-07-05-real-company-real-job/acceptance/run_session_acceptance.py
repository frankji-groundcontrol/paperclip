#!/usr/bin/env python3
"""E11: session path (non-agent / browser user). A real user logs in via the broker
(email+password -> opaque pcs_ session), then forms a company and runs a real OpenAI job
using ONLY that session — proving the frontend's path works. Verifies persistence via RLS.
Requires: backend running with real SUPABASE_*/OPENAI_* env; SUPABASE_ANON_KEY in env; test_users.json.
"""
import json, os, sys, secrets, urllib.request, urllib.error, pathlib

ANON = os.environ.get("SUPABASE_ANON_KEY") or sys.exit("set SUPABASE_ANON_KEY")
SERVER = os.environ.get("PAPERCLIP_SERVER", "http://127.0.0.1:8787").rstrip("/")
root = pathlib.Path(__file__).resolve().parents[4]
cfg = json.load(open(root / "test_users.json"))
SB = cfg["project_url"].rstrip("/"); PW = cfg["password"]
UID = {u["email"].split("_")[0]: u["auth_uid"] for u in cfg["users"]}

def req(url, method="GET", headers=None, body=None):
    data = json.dumps(body).encode() if body is not None else None
    r = urllib.request.Request(url, data=data, method=method)
    for k, v in (headers or {}).items():
        r.add_header(k, v)
    try:
        with urllib.request.urlopen(r) as resp:
            raw = resp.read().decode(); return resp.status, (json.loads(raw) if raw else None)
    except urllib.error.HTTPError as e:
        try: return e.code, json.loads(e.read().decode())
        except Exception: return e.code, None

def api(path, method="GET", session=None, body=None):
    h = {"Content-Type": "application/json"}
    if session: h["Authorization"] = f"Bearer {session}"
    return req(f"{SERVER}{path}", method, h, body)

def sb(path, tok=None):  # RLS-scoped verification read as the user
    return req(f"{SB}/rest/v1/{path}", "GET",
               {"apikey": ANON, "Authorization": f"Bearer {tok or ANON}", "Accept-Profile": "paperclip"})

def gotrue_jwt(u):  # separate direct login for DB verification (the broker hides the JWT)
    _, d = req(f"{SB}/auth/v1/token?grant_type=password", "POST",
               {"apikey": ANON, "Content-Type": "application/json"},
               {"email": f"{u}_paperclip@gmail.com", "password": PW})
    return d["access_token"]

res = []
def chk(c, ok, x=""): res.append((c, ok)); print(f"[{'PASS' if ok else 'FAIL'}] {c}" + (f" -- {x}" if x else ""))

def login(u):
    st, d = api("/api/auth/login", "POST", None, {"email": f"{u}_paperclip@gmail.com", "password": PW})
    return st, d

def main():
    nonce = secrets.token_hex(3)
    # S1: real login via broker -> opaque pcs_ session
    st, d = login("testuser1")
    sess = (d or {}).get("session")
    chk("S1.login", st == 200 and isinstance(sess, str) and sess.startswith("pcs_") and "session" in (d or {}), f"status={st}")
    # leak: login payload must not carry a JWT/anon/supabase url
    blob = json.dumps(d)
    chk("S1.no_leak", not any(s in blob for s in [ANON[:20], "supabase.co", "access_token", "refresh_token"]), "")

    # S2: create a company via the SESSION (server uses the stored JWT + default team)
    st, d = api("/api/paperclip/companies", "POST", sess, {"name": f"Session Co {nonce}"})
    cid = (d or {}).get("companyId")
    chk("S2.create_company", st == 200 and isinstance(cid, str), f"status={st} {d}")
    jwt1 = gotrue_jwt("testuser1")
    st, rows = sb(f"my_companies?id=eq.{cid}&select=team_id,created_by", jwt1)
    row = rows[0] if isinstance(rows, list) and rows else {}
    chk("S2.db", row.get("created_by") == UID["testuser1"] and row.get("team_id") == cfg_default_team("testuser1"), f"{row}")

    # S3: run a real OpenAI job via the SESSION
    st, d = api(f"/api/paperclip/companies/{cid}/jobs", "POST", sess,
                {"prompt": "What is 2+2? Reply with only the number.", "clientToken": f"s3-{nonce}"})
    ok = st == 200 and (d or {}).get("status") == "succeeded" and "4" in str((d or {}).get("result"))
    chk("S3.run_job", ok, f"status={st} result={str((d or {}).get('result'))[:30]}")
    jid = (d or {}).get("jobId")
    st, jr = sb(f"my_jobs?id=eq.{jid}&select=status,result_text,subject_type,created_by", jwt1)
    j = jr[0] if isinstance(jr, list) and jr else {}
    chk("S3.db", j.get("status") == "succeeded" and "4" in str(j.get("result_text")) and j.get("subject_type") == "user"
        and j.get("created_by") == UID["testuser1"], f"{j}")

    # S4: list via session
    st, comps = api("/api/paperclip/companies", "GET", sess)
    clist = comps if isinstance(comps, list) else (comps or {}).get("companies", [])
    chk("S4.list_companies", any(c.get("id") == cid for c in clist), f"status={st}")
    st, jobs = api(f"/api/paperclip/companies/{cid}/jobs", "GET", sess)
    jlist = jobs if isinstance(jobs, list) else (jobs or {}).get("jobs", [])
    chk("S4.list_jobs", any(x.get("id") == jid for x in jlist), f"status={st}")

    # S5: RLS — a second user's session cannot see it
    st, d2 = login("testuser2")
    sess2 = (d2 or {}).get("session")
    st, comps2 = api("/api/paperclip/companies", "GET", sess2)
    clist2 = comps2 if isinstance(comps2, list) else (comps2 or {}).get("companies", [])
    chk("S5.rls", not any(c.get("id") == cid for c in clist2), f"status={st}")

    npass = sum(1 for _, ok in res if ok); fails = [c for c, ok in res if not ok]
    print(f"\n=== {npass}/{len(res)} passed ===  FAILURES: {fails if fails else 'none'}")
    print(f"CLEANUP_COMPANY={cid}")
    sys.exit(0 if not fails else 1)

def cfg_default_team(u):
    return next(x.get("default_team_id") for x in cfg["users"] if x["email"].split("_")[0] == u)

if __name__ == "__main__":
    main()
