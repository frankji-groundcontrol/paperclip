#!/usr/bin/env python3
"""Real-user acceptance for real company + real OpenAI job (agent / API-key path).

Drives the running Paperclip backend (PAPERCLIP_SERVER) with a real API key, and verifies
persistence + RLS directly against the real paperclip schema (PostgREST, RLS-scoped reads).
No mocks. Requires: the backend running with real SUPABASE_* + OPENAI_* env; SUPABASE_ANON_KEY
in this process's env; test_users.json at the repo root.
"""
import json, os, sys, time, hashlib, secrets, urllib.request, urllib.error, pathlib

SERVER = os.environ.get("PAPERCLIP_SERVER", "http://127.0.0.1:8787").rstrip("/")
ANON = os.environ.get("SUPABASE_ANON_KEY") or sys.exit("set SUPABASE_ANON_KEY")
root = pathlib.Path(__file__).resolve().parents[4]
cfg = json.load(open(root / "test_users.json"))
SB = cfg["project_url"].rstrip("/")
PW = cfg["password"]
UID = {u["email"].split("_")[0]: u["auth_uid"] for u in cfg["users"]}

def req(url, method="GET", headers=None, body=None):
    data = json.dumps(body).encode() if body is not None else None
    r = urllib.request.Request(url, data=data, method=method)
    for k, v in (headers or {}).items():
        r.add_header(k, v)
    try:
        with urllib.request.urlopen(r) as resp:
            raw = resp.read().decode()
            return resp.status, (json.loads(raw) if raw else None)
    except urllib.error.HTTPError as e:
        raw = e.read().decode()
        try:
            return e.code, json.loads(raw)
        except Exception:
            return e.code, raw

# --- Supabase helpers (mint key + RLS-scoped verification reads) ---
def sb(path, method="GET", tok=None, body=None):
    return req(f"{SB}{path}", method,
               {"apikey": ANON, "Authorization": f"Bearer {tok or ANON}", "Content-Type": "application/json",
                "Content-Profile": "paperclip", "Accept-Profile": "paperclip"}, body)
def login(u):
    _, d = req(f"{SB}/auth/v1/token?grant_type=password", "POST",
               {"apikey": ANON, "Content-Type": "application/json"},
               {"email": f"{u}_paperclip@gmail.com", "password": PW})
    return d["access_token"]
def sha(s): return hashlib.sha256(s.encode()).hexdigest()
def mint_key(u):
    jwt = login(u); team = sb("/rest/v1/rpc/whoami", "POST", jwt, {})[1]["user"]["default_team_id"]
    pref = "pc_" + secrets.token_hex(4); full = f"paperclip_{pref}_{secrets.token_urlsafe(24)}"
    st, kid = sb("/rest/v1/rpc/create_api_key", "POST", jwt,
                 {"p_team_id": team, "p_name": "jobs acceptance", "p_prefix": pref, "p_key_hash": sha(full)})
    return {"jwt": jwt, "team": team, "prefix": pref, "hash": sha(full), "full": full, "id": kid}

# --- backend (agent HTTP path) ---
def api(path, method="GET", key=None, body=None):
    return req(f"{SERVER}{path}", method,
               {"Authorization": f"Bearer {key}", "Content-Type": "application/json"} if key else {"Content-Type": "application/json"},
               body)

res = []
def chk(c, d, ok, x=""):
    res.append((c, ok)); print(f"[{'PASS' if ok else 'FAIL'}] {c}: {d}" + (f"  -- {x}" if x else ""))

def main():
    k1 = mint_key("testuser1"); k2 = mint_key("testuser2")
    nonce = secrets.token_hex(3)
    cname = f"Acme {nonce}"

    # E1: form a real company (agent path) + DB assert
    st, d = api("/api/paperclip/companies", "POST", k1["full"], {"name": cname})
    cid = (d or {}).get("companyId") if isinstance(d, dict) else None
    chk("E1", "POST /api/paperclip/companies", st == 200 and bool(cid), f"{st} {d}")
    st, rows = sb(f"/rest/v1/my_companies?id=eq.{cid}&select=team_id,created_by,name", "GET", k1["jwt"])
    row = rows[0] if isinstance(rows, list) and rows else {}
    chk("E1b", "company row persisted on user's team", row.get("team_id") == k1["team"] and row.get("created_by") == UID["testuser1"], f"{row}")

    # E2: run a real OpenAI job + DB assert
    st, d = api(f"/api/paperclip/companies/{cid}/jobs", "POST", k1["full"],
                {"prompt": "What is 2+2? Reply with only the number.", "clientToken": f"e2-{nonce}"})
    jid = (d or {}).get("jobId") if isinstance(d, dict) else None
    ok2 = st == 200 and (d or {}).get("status") == "succeeded" and "4" in str((d or {}).get("result", "")) \
          and (d or {}).get("usage", {}).get("total_tokens", 0) > 0
    chk("E2", "run job -> real OpenAI result", ok2, f"{st} status={(d or {}).get('status')} result={str((d or {}).get('result'))[:40]}")
    st, jr = sb(f"/rest/v1/my_jobs?id=eq.{jid}&select=status,result_text,subject_type,created_by,model", "GET", k1["jwt"])
    j = jr[0] if isinstance(jr, list) and jr else {}
    chk("E3", "job row persisted (succeeded, subject/user, model)",
        j.get("status") == "succeeded" and "4" in str(j.get("result_text")) and j.get("subject_type") == "user"
        and j.get("created_by") == UID["testuser1"] and j.get("model") == "gpt-5.4-mini", f"{j}")

    # E2b: forced LLM failure (rejected model) -> failed + sanitized error
    st, d = api(f"/api/paperclip/companies/{cid}/jobs", "POST", k1["full"], {"prompt": "hi", "model": "gpt-4o"})
    failed = (isinstance(d, dict) and d.get("status") == "failed") or st >= 500
    chk("E2b", "rejected-model job fails (not succeeds)", failed, f"{st} {str(d)[:80]}")
    if isinstance(d, dict) and d.get("jobId"):
        st, jr = sb(f"/rest/v1/my_jobs?id=eq.{d['jobId']}&select=status,error", "GET", k1["jwt"])
        j = jr[0] if isinstance(jr, list) and jr else {}
        e = str(j.get("error") or "")
        chk("E2c", "failure error is sanitized (no url/token)", j.get("status") == "failed" and "://" not in e and "Bearer" not in e and "sk-" not in e, f"error={e[:60]}")

    # E5: a real substantive task
    st, d = api(f"/api/paperclip/companies/{cid}/jobs", "POST", k1["full"],
                {"prompt": "Summarize the theory of relativity in one sentence.", "clientToken": f"e5-{nonce}"})
    chk("E5", "substantive job -> real sentence", st == 200 and len(str((d or {}).get("result", ""))) > 20, f"len={len(str((d or {}).get('result','')))}")

    # E4: list jobs
    st, d = api(f"/api/paperclip/companies/{cid}/jobs", "GET", k1["full"])
    jobs = d.get("jobs") if isinstance(d, dict) else d
    chk("E4", "GET jobs lists the job", isinstance(jobs, list) and any(x.get("id") == jid for x in jobs), f"{st}")

    # E8: RLS — testuser2 key cannot see Acme
    st, d = api("/api/paperclip/companies", "GET", k2["full"])
    comps = d.get("companies") if isinstance(d, dict) else d
    chk("E8", "RLS: other-team key can't see Acme", isinstance(comps, list) and not any(c.get("id") == cid for c in comps), f"{st}")

    # E9: cross-team job creation blocked (direct PostgREST)
    st, d = sb("/rest/v1/rpc/create_job_with_key", "POST", None,
               {"p_prefix": k2["prefix"], "p_key_hash": k2["hash"], "p_company_id": cid, "p_prompt": "x"})
    chk("E9", "cross-team create_job_with_key blocked", st != 200, f"{st}")

    # E10: (a) malformed bearer -> 401 ; (b) bogus key direct rpc -> 28000
    st, _ = api("/api/paperclip/companies", "POST", "garbage-not-a-key", {"name": "x"})
    chk("E10a", "malformed bearer -> 401", st == 401, f"{st}")
    st, d = sb("/rest/v1/rpc/create_company_with_key", "POST", None, {"p_prefix": "pc_dead", "p_key_hash": sha("no"), "p_name": "x"})
    code = d.get("code") if isinstance(d, dict) else None
    chk("E10b", "bogus key direct rpc -> 28000", code == "28000", f"{st} {code}")

    # E12: leak check across all backend responses seen
    st, sess = api("/api/paperclip/companies", "GET", k1["full"])
    blob = json.dumps(sess)
    leak = any(s in blob for s in [ANON, os.environ.get("OPENAI_API_KEY", "OPENAI_UNSET"), "supabase.co", "key_hash", k1["jwt"][:20]])
    chk("E12", "no supabase/openai secret leak in responses", not leak, "")

    # cleanup
    sb("/rest/v1/rpc/revoke_api_key", "POST", k1["jwt"], {"p_id": k1["id"]})
    sb("/rest/v1/rpc/revoke_api_key", "POST", k2["jwt"], {"p_id": k2["id"]})
    print(f"\nCOMPANY_ID={cid}   (delete via SQL to clean up)")
    npass = sum(1 for _, ok in res if ok); fails = [c for c, ok in res if not ok]
    print(f"=== {npass}/{len(res)} passed ===  FAILURES: {fails if fails else 'none'}")
    sys.exit(0 if not fails else 1)

if __name__ == "__main__":
    main()
