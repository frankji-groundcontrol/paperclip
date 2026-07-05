import { describe, it, expect } from "vitest";
import {
  login,
  createCompany,
  listCompanies,
  runJob,
  listJobs,
  hireAgent,
  listAgents,
  listApprovals,
  decideApproval,
} from "../composables/usePaperclipSession";
import type { Fetcher } from "../composables/useApi";

type Call = { url: string; options?: { method?: string; body?: unknown; headers?: Record<string, string> } };

function mock(handler: (url: string, options?: Call["options"]) => unknown): { fetcher: Fetcher; calls: Call[] } {
  const calls: Call[] = [];
  const fetcher: Fetcher = async (url, options) => {
    calls.push({ url, options });
    return handler(url, options);
  };
  return { fetcher, calls };
}

describe("usePaperclipSession", () => {
  it("login posts credentials and returns the opaque session (no JWT surfaced)", async () => {
    const { fetcher, calls } = mock(() => ({ session: "pcs_abc", whoami: { user: { email: "a@b.c" } } }));
    const s = await login(fetcher, "a@b.c", "pw");
    expect(s.token).toBe("pcs_abc");
    expect(calls[0].url).toBe("/api/auth/login");
    expect(calls[0].options?.method).toBe("POST");
    expect(calls[0].options?.body).toEqual({ email: "a@b.c", password: "pw" });
  });

  it("createCompany posts the name with the session bearer", async () => {
    const { fetcher, calls } = mock(() => ({ companyId: "co-1" }));
    const id = await createCompany(fetcher, "pcs_abc", "Acme");
    expect(id).toBe("co-1");
    expect(calls[0].url).toBe("/api/paperclip/companies");
    expect(calls[0].options?.headers?.Authorization).toBe("Bearer pcs_abc");
    expect(calls[0].options?.body).toEqual({ name: "Acme" });
  });

  it("listCompanies normalizes both an array and a { companies } shape", async () => {
    const arr = await listCompanies(mock(() => [{ id: "1", name: "A" }]).fetcher, "pcs_abc");
    expect(arr).toHaveLength(1);
    const wrapped = await listCompanies(mock(() => ({ companies: [{ id: "2", name: "B" }] })).fetcher, "pcs_abc");
    expect(wrapped[0].id).toBe("2");
  });

  it("runJob posts the prompt (and optional model) with the session bearer", async () => {
    const { fetcher, calls } = mock(() => ({ jobId: "j1", status: "succeeded", result: "4", usage: { total_tokens: 18 } }));
    const r = await runJob(fetcher, "pcs_abc", "co-1", "2+2?", "gpt-5.4-mini");
    expect(r.result).toBe("4");
    expect(calls[0].url).toBe("/api/paperclip/companies/co-1/jobs");
    expect(calls[0].options?.body).toEqual({ prompt: "2+2?", model: "gpt-5.4-mini" });
    expect(calls[0].options?.headers?.Authorization).toBe("Bearer pcs_abc");
  });

  it("listJobs reads the company's jobs", async () => {
    const jobs = await listJobs(mock(() => [{ id: "j1", status: "succeeded" }]).fetcher, "pcs_abc", "co-1");
    expect(jobs[0].id).toBe("j1");
  });

  it("hireAgent posts to the company agents endpoint with the session bearer", async () => {
    const { fetcher, calls } = mock(() => ({ agentId: "ag-1", status: "pending_approval", approvalId: "ap-1" }));
    const r = await hireAgent(fetcher, "pcs_abc", "co-1", "Data Analyst", "analyst");
    expect(r.agentId).toBe("ag-1");
    expect(r.status).toBe("pending_approval");
    expect(calls[0].url).toBe("/api/paperclip/companies/co-1/agents");
    expect(calls[0].options?.body).toEqual({ name: "Data Analyst", role: "analyst" });
    expect(calls[0].options?.headers?.Authorization).toBe("Bearer pcs_abc");
  });

  it("listAgents and listApprovals read the right endpoints", async () => {
    const a = await listAgents(mock(() => [{ id: "ag-1", name: "A", role: "r", status: "active" }]).fetcher, "pcs_abc", "co-1");
    expect(a[0].id).toBe("ag-1");
    const ap = await listApprovals(mock(() => ({ approvals: [{ id: "ap-1", type: "hire_agent", status: "pending" }] })).fetcher, "pcs_abc", "co-1");
    expect(ap[0].id).toBe("ap-1");
  });

  it("decideApproval posts the decision to the approval decide endpoint", async () => {
    const { fetcher, calls } = mock(() => ({ status: "approved" }));
    const r = await decideApproval(fetcher, "pcs_abc", "ap-1", true);
    expect(r.status).toBe("approved");
    expect(calls[0].url).toBe("/api/paperclip/approvals/ap-1/decide");
    expect(calls[0].options?.body).toEqual({ approve: true });
    expect(calls[0].options?.headers?.Authorization).toBe("Bearer pcs_abc");
  });
});
