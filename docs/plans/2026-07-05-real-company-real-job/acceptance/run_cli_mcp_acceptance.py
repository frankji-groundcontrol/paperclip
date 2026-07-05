#!/usr/bin/env python3
"""E6/E7: prove the agent CLI + MCP form a real company and run a real OpenAI job.

Drives the built `paperclip` and `paperclip-mcp` binaries against a running backend
(PAPERCLIP_SERVER), with a real minted API key, and verifies persistence in the real DB.
Requires: backend running with real SUPABASE_*/OPENAI_* env; SUPABASE_ANON_KEY in env;
the binaries built (`cargo build --bins`); test_users.json at repo root.
"""
import json, os, sys, subprocess, secrets, hashlib, urllib.request, urllib.error, pathlib

ANON = os.environ.get("SUPABASE_ANON_KEY") or sys.exit("set SUPABASE_ANON_KEY")
SERVER = os.environ.get("PAPERCLIP_SERVER", "http://127.0.0.1:8787").rstrip("/")
root = pathlib.Path(__file__).resolve().parents[4]
cfg = json.load(open(root / "test_users.json"))
SB = cfg["project_url"].rstrip("/"); PW = cfg["password"]
BIN = str(root / "backend-rs" / "target" / "debug")

def sb(path, method="GET", tok=None, body=None):
    r = urllib.request.Request(SB + path, data=(json.dumps(body).encode() if body is not None else None), method=method)
    for k, v in {"apikey": ANON, "Authorization": f"Bearer {tok or ANON}", "Content-Type": "application/json",
                 "Content-Profile": "paperclip", "Accept-Profile": "paperclip"}.items():
        r.add_header(k, v)
    try:
        with urllib.request.urlopen(r) as resp:
            return resp.status, json.loads(resp.read().decode() or "null")
    except urllib.error.HTTPError as e:
        try: return e.code, json.loads(e.read().decode())
        except Exception: return e.code, None

res = []
def chk(c, ok, x=""): res.append((c, ok)); print(f"[{'PASS' if ok else 'FAIL'}] {c}" + (f" -- {x}" if x else ""))

jwt = sb("/auth/v1/token?grant_type=password", "POST", None, {"email": "testuser1_paperclip@gmail.com", "password": PW})[1]["access_token"]
team = sb("/rest/v1/rpc/whoami", "POST", jwt, {})[1]["user"]["default_team_id"]
pref = "pc_" + secrets.token_hex(4); full = f"paperclip_{pref}_{secrets.token_urlsafe(24)}"
_, kid = sb("/rest/v1/rpc/create_api_key", "POST", jwt, {"p_team_id": team, "p_name": "cli/mcp acceptance", "p_prefix": pref, "p_key_hash": hashlib.sha256(full.encode()).hexdigest()})
env = dict(os.environ, PAPERCLIP_API_KEY=full, PAPERCLIP_SERVER=SERVER)
nonce = secrets.token_hex(3)

# E6: CLI
cid = subprocess.run([f"{BIN}/paperclip", "company", "create", f"CLI Co {nonce}"], env=env, capture_output=True, text=True).stdout.strip()
chk("E6.cli_create_company", len(cid) == 36, cid)
out = subprocess.run([f"{BIN}/paperclip", "job", "run", "--company", cid, "Reply with only the number 7"], env=env, capture_output=True, text=True)
chk("E6.cli_run_job", "7" in out.stdout, f"stdout={out.stdout.strip()!r}")
_, rows = sb(f"/rest/v1/my_jobs?company_id=eq.{cid}&select=status,result_text", "GET", jwt)
chk("E6.cli_db", isinstance(rows, list) and rows and rows[0]["status"] == "succeeded" and "7" in str(rows[0]["result_text"]))

# E7: MCP (stdio JSON-RPC)
p = subprocess.Popen([f"{BIN}/paperclip-mcp"], env=env, stdin=subprocess.PIPE, stdout=subprocess.PIPE, text=True, bufsize=1)
def rpc(o):
    p.stdin.write(json.dumps(o) + "\n"); p.stdin.flush()
    return json.loads(p.stdout.readline()) if "id" in o else None
init = rpc({"jsonrpc": "2.0", "id": 1, "method": "initialize", "params": {}})
chk("E7.initialize", init["result"]["serverInfo"]["name"] == "paperclip-mcp")
rpc({"jsonrpc": "2.0", "method": "notifications/initialized"})
tl = rpc({"jsonrpc": "2.0", "id": 2, "method": "tools/list"})
names = {t["name"] for t in tl["result"]["tools"]}
chk("E7.tools_list", {"paperclip_create_company", "paperclip_run_job", "paperclip_list_companies", "paperclip_list_jobs"} <= names, str(sorted(names)))
cc = rpc({"jsonrpc": "2.0", "id": 3, "method": "tools/call", "params": {"name": "paperclip_create_company", "arguments": {"name": f"MCP Co {nonce}"}}})
mcid = json.loads(cc["result"]["content"][0]["text"]).get("companyId")
chk("E7.create_company", isinstance(mcid, str) and len(mcid) == 36, mcid)
rj = rpc({"jsonrpc": "2.0", "id": 4, "method": "tools/call", "params": {"name": "paperclip_run_job", "arguments": {"companyId": mcid, "prompt": "Reply with only the number 7"}}})
jr = json.loads(rj["result"]["content"][0]["text"])
chk("E7.run_job", jr.get("status") == "succeeded" and "7" in str(jr.get("result")), f"result={jr.get('result')!r}")
p.stdin.close(); p.terminate()
_, rows = sb(f"/rest/v1/my_jobs?company_id=eq.{mcid}&select=status,result_text", "GET", jwt)
chk("E7.db", isinstance(rows, list) and rows and rows[0]["status"] == "succeeded")

sb("/rest/v1/rpc/revoke_api_key", "POST", jwt, {"p_id": kid})
np = sum(1 for _, ok in res if ok); fails = [c for c, ok in res if not ok]
print(f"\n=== {np}/{len(res)} passed ===  FAILURES: {fails if fails else 'none'}")
print(f"CLEANUP_COMPANIES={cid},{mcid}")
sys.exit(0 if not fails else 1)
