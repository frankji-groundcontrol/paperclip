#!/usr/bin/env python3
"""Agent-key scope enforcement (migration 0012), against the real paperclip schema.
Unscoped key = full access (backward compat); a scoped key is restricted.
Requires SUPABASE_ANON_KEY in env + test_users.json at the repo root.
"""
import json, os, sys, hashlib, secrets, urllib.request, urllib.error, pathlib

ANON = os.environ.get("SUPABASE_ANON_KEY") or sys.exit("set SUPABASE_ANON_KEY")
root = pathlib.Path(__file__).resolve().parents[4]
cfg = json.load(open(root / "test_users.json"))
BASE = cfg["project_url"].rstrip("/"); PW = cfg["password"]

def call(path, method="POST", tok=None, body=None):
    r = urllib.request.Request(BASE + path, data=(json.dumps(body).encode() if body is not None else None), method=method)
    for k, v in {"apikey": ANON, "Authorization": f"Bearer {tok or ANON}", "Content-Type": "application/json",
                 "Content-Profile": "paperclip", "Accept-Profile": "paperclip"}.items():
        r.add_header(k, v)
    try:
        with urllib.request.urlopen(r) as resp:
            return resp.status, json.loads(resp.read().decode() or "null")
    except urllib.error.HTTPError as e:
        try: return e.code, json.loads(e.read().decode())
        except Exception: return e.code, None

def sha(s): return hashlib.sha256(s.encode()).hexdigest()
jwt = call("/auth/v1/token?grant_type=password", body={"email": "testuser1_paperclip@gmail.com", "password": PW})[1]["access_token"]
team = call("/rest/v1/rpc/whoami", tok=jwt, body={})[1]["user"]["default_team_id"]

def mint(scopes):
    p = "pc_" + secrets.token_hex(4); f = f"paperclip_{p}_{secrets.token_urlsafe(20)}"
    _, kid = call("/rest/v1/rpc/create_api_key", tok=jwt,
                  body={"p_team_id": team, "p_name": "scopetest", "p_prefix": p, "p_key_hash": sha(f), "p_scopes": scopes})
    return p, sha(f), kid

res = []
def chk(c, ok, x=""): res.append(ok); print(f"[{'PASS' if ok else 'FAIL'}] {c}" + (f" -- {x}" if x else ""))

keys = []
p0, h0, k0 = mint([]); keys.append(k0)
st, cid = call("/rest/v1/rpc/create_company_with_key", body={"p_prefix": p0, "p_key_hash": h0, "p_name": "Scope Co full"})
chk("SC1 unscoped key creates company (backward compat)", st == 200 and isinstance(cid, str))
st, _ = call("/rest/v1/rpc/create_job_with_key", body={"p_prefix": p0, "p_key_hash": h0, "p_company_id": cid, "p_prompt": "x"})
chk("SC2 unscoped key creates job", st == 200)

pr, hr, kr = mint(["companies:read"]); keys.append(kr)
st, _ = call("/rest/v1/rpc/create_company_with_key", body={"p_prefix": pr, "p_key_hash": hr, "p_name": "nope"})
chk("SC3 companies:read key cannot create company (42501)", st != 200, str(st))
st, rows = call("/rest/v1/rpc/list_companies_with_key", body={"p_prefix": pr, "p_key_hash": hr})
chk("SC4 companies:read key can list companies", st == 200 and isinstance(rows, list))
st, _ = call("/rest/v1/rpc/create_job_with_key", body={"p_prefix": pr, "p_key_hash": hr, "p_company_id": cid, "p_prompt": "x"})
chk("SC5 companies:read key cannot create job (42501)", st != 200, str(st))

pw, hw, kw = mint(["companies:write", "jobs:write"]); keys.append(kw)
st, cid2 = call("/rest/v1/rpc/create_company_with_key", body={"p_prefix": pw, "p_key_hash": hw, "p_name": "Scope Co write"})
st2, _ = call("/rest/v1/rpc/create_job_with_key", body={"p_prefix": pw, "p_key_hash": hw, "p_company_id": cid2, "p_prompt": "x"})
chk("SC6 write-scoped key creates company + job", st == 200 and st2 == 200, f"{st}/{st2}")

for k in keys:
    call("/rest/v1/rpc/revoke_api_key", tok=jwt, body={"p_id": k})
print(f"\n=== {sum(res)}/{len(res)} passed ===   (delete 'Scope Co %' companies to clean up)")
sys.exit(0 if all(res) else 1)
