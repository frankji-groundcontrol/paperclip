#!/usr/bin/env python3
"""Agent hiring parity (migration 0013): a company hires a real agent (approval-gated), the board
approves, and the hired agent does a REAL OpenAI job attributed to it. Also proves the governance:
limbo, board-only approve, agents-cannot-decide, terminal decide, RLS.

Requires: backend running with real SUPABASE_*/OPENAI_* env; SUPABASE_ANON_KEY in env; test_users.json.
"""
import json, os, sys, hashlib, secrets, urllib.request, urllib.error, pathlib

ANON = os.environ.get("SUPABASE_ANON_KEY") or sys.exit("set SUPABASE_ANON_KEY")
SRV = os.environ.get("PAPERCLIP_SERVER", "http://127.0.0.1:8787").rstrip("/")
root = pathlib.Path(__file__).resolve().parents[4]
cfg = json.load(open(root / "test_users.json"))
SB = cfg["project_url"].rstrip("/"); PW = cfg["password"]

def sb(path, method="POST", tok=None, body=None):
    r = urllib.request.Request(SB + path, data=(json.dumps(body).encode() if body is not None else None), method=method)
    for k, v in {"apikey": ANON, "Authorization": f"Bearer {tok or ANON}", "Content-Type": "application/json",
                 "Content-Profile": "paperclip", "Accept-Profile": "paperclip"}.items():
        r.add_header(k, v)
    try:
        with urllib.request.urlopen(r) as resp: return resp.status, json.loads(resp.read().decode() or "null")
    except urllib.error.HTTPError as e:
        try: return e.code, json.loads(e.read().decode())
        except Exception: return e.code, None

def api(path, key=None, body=None):
    r = urllib.request.Request(SRV + path, data=json.dumps(body).encode(), method="POST",
                               headers={"Content-Type": "application/json", "Authorization": f"Bearer {key}"})
    try:
        with urllib.request.urlopen(r) as resp: return resp.status, json.loads(resp.read().decode() or "null")
    except urllib.error.HTTPError as e:
        try: return e.code, json.loads(e.read().decode())
        except Exception: return e.code, None

def sha(s): return hashlib.sha256(s.encode()).hexdigest()
def login(u): return sb("/auth/v1/token?grant_type=password", body={"email": f"{u}_paperclip@gmail.com", "password": PW})[1]["access_token"]

res = []
def chk(c, ok, x=""): res.append(ok); print(f"[{'PASS' if ok else 'FAIL'}] {c}" + (f" -- {x}" if x else ""))

def main():
    admin = login("testadmin1"); u1 = login("testuser1"); u2 = login("testuser2")
    team = sb("/rest/v1/rpc/whoami", tok=admin, body={})[1]["user"]["default_team_id"]
    u2id = sb("/rest/v1/rpc/whoami", tok=u2, body={})[1]["user"]["id"]
    n = secrets.token_hex(3)
    _, cid = sb("/rest/v1/rpc/create_company", tok=admin, body={"p_team_id": team, "p_name": f"HireCo {n}"})

    # H1 hire (approval required by default)
    _, h = sb("/rest/v1/rpc/hire_agent", tok=admin, body={"p_company_id": cid, "p_name": "Data Analyst", "p_role": "analyst"})
    agent, appr = h["agentId"], h["approvalId"]
    chk("H1 hire -> pending_approval + approval", h["status"] == "pending_approval" and agent and appr)
    # H2 limbo: cannot mint a key for a pending agent
    pk = "pc_" + secrets.token_hex(4); full = f"paperclip_{pk}_{secrets.token_urlsafe(24)}"
    st, _ = sb("/rest/v1/rpc/create_api_key", tok=admin, body={"p_team_id": team, "p_name": "ak", "p_prefix": pk, "p_key_hash": sha(full), "p_subject_type": "agent", "p_agent_id": agent})
    chk("H2 limbo: cannot mint key for pending agent", st != 200)
    # H5 non-board cannot approve
    sb("/rest/v1/rpc/add_team_member", tok=admin, body={"p_team": team, "p_user": u2id, "p_role": "operator"})
    st, _ = sb("/rest/v1/rpc/decide_approval", tok=u2, body={"p_approval_id": appr, "p_approve": True})
    chk("H5 operator (non-board) cannot approve", st != 200)
    # H3 board approves
    st, d = sb("/rest/v1/rpc/decide_approval", tok=admin, body={"p_approval_id": appr, "p_approve": True})
    chk("H3 board approves -> active", st == 200 and d == "approved")
    # H-terminal: cannot re-decide
    st, _ = sb("/rest/v1/rpc/decide_approval", tok=admin, body={"p_approval_id": appr, "p_approve": False})
    chk("H-terminal: cannot re-decide an approved approval", st != 200)
    # H4 mint agent key + the HIRED AGENT runs a REAL OpenAI job
    st, _ = sb("/rest/v1/rpc/create_api_key", tok=admin, body={"p_team_id": team, "p_name": "agent key", "p_prefix": pk, "p_key_hash": sha(full), "p_subject_type": "agent", "p_agent_id": agent})
    chk("H4a mint agent key (agent active)", st == 200)
    st, j = api(f"/api/paperclip/companies/{cid}/jobs", key=full, body={"prompt": "What is 2+2? Reply with only the number.", "clientToken": f"agent-{n}"})
    chk("H4b HIRED AGENT runs a REAL OpenAI job -> succeeded", st == 200 and j.get("status") == "succeeded" and "4" in str(j.get("result")), f"result={str(j.get('result'))[:20]}")
    st, row = sb(f"/rest/v1/my_jobs?id=eq.{j.get('jobId')}&select=subject_type,agent_id,result_text", "GET", admin)
    r = row[0] if isinstance(row, list) and row else {}
    chk("H4c job persisted as AGENT work (subject_type=agent, agent_id)", r.get("subject_type") == "agent" and r.get("agent_id") == agent and "4" in str(r.get("result_text")), f"{r}")
    # H9 agent key cannot hire (no can_create_agents) nor decide
    st, _ = sb("/rest/v1/rpc/hire_agent_with_key", body={"p_prefix": pk, "p_key_hash": sha(full), "p_company_id": cid, "p_name": "sub"})
    chk("H9a agent key without can_create_agents cannot hire", st != 200)
    _, d2 = sb("/rest/v1/rpc/hire_agent", tok=admin, body={"p_company_id": cid, "p_name": "Temp"})
    st, _ = sb("/rest/v1/rpc/decide_approval_with_key", body={"p_prefix": pk, "p_key_hash": sha(full), "p_approval_id": d2["approvalId"], "p_approve": True})
    chk("H9b agent key cannot decide approvals", st != 200)
    # H6 reject -> archived
    _, d3 = sb("/rest/v1/rpc/hire_agent", tok=admin, body={"p_company_id": cid, "p_name": "Reject Me"})
    sb("/rest/v1/rpc/decide_approval", tok=admin, body={"p_approval_id": d3["approvalId"], "p_approve": False})
    st, a3 = sb(f"/rest/v1/my_agents?id=eq.{d3['agentId']}&select=status", "GET", admin)
    chk("H6 reject -> agent archived", isinstance(a3, list) and a3 and a3[0]["status"] == "archived")
    # H8 RLS
    st, a8 = sb(f"/rest/v1/my_agents?id=eq.{agent}&select=id", "GET", u1)
    chk("H8 RLS: outside user cannot see the agent", a8 == [])

    sb("/rest/v1/rpc/remove_team_member", tok=admin, body={"p_team": team, "p_user": u2id})
    npass = sum(1 for x in res if x)
    print(f"\n=== {npass}/{len(res)} passed ===   (delete 'HireCo %' companies to clean up)")
    sys.exit(0 if all(res) else 1)

if __name__ == "__main__":
    main()
