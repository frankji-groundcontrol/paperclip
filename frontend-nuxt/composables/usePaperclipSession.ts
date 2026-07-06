// Paperclip (real product) composable for NON-AGENT users: log in with
// email/password → opaque `pcs_` session, then form a company and run jobs.
// Pure functions over a `Fetcher` (unit-testable); the browser only ever holds
// the opaque `pcs_` token — no Supabase/OpenAI detail reaches the client.
import type { Fetcher } from "./useApi";

export type Session = { token: string; user: unknown };
export type Company = { id: string; name: string };
export type Job = {
  id: string;
  status: string;
  prompt?: string;
  result_text?: string | null;
  usage?: unknown;
};
export type JobResult = {
  jobId: string;
  status: string;
  result?: string;
  error?: string;
  usage?: { total_tokens?: number };
};

function auth(session: string): Record<string, string> {
  return { Authorization: `Bearer ${session}` };
}

/** Log in via the broker (`POST /api/auth/login`) → opaque `pcs_` session. */
export async function login(fetcher: Fetcher, email: string, password: string): Promise<Session> {
  const data = (await fetcher("/api/auth/login", {
    method: "POST",
    body: { email, password },
  })) as { session: string; whoami: unknown };
  return { token: data.session, user: data.whoami };
}

/** Form a company (`POST /api/paperclip/companies`) → companyId. */
export async function createCompany(fetcher: Fetcher, session: string, name: string): Promise<string> {
  const data = (await fetcher("/api/paperclip/companies", {
    method: "POST",
    body: { name },
    headers: auth(session),
  })) as { companyId: string };
  return data.companyId;
}

/** List companies visible to the session (`GET /api/paperclip/companies`). */
export async function listCompanies(fetcher: Fetcher, session: string): Promise<Company[]> {
  const data = await fetcher("/api/paperclip/companies", { headers: auth(session) });
  return (Array.isArray(data) ? data : (data as { companies?: Company[] }).companies ?? []) as Company[];
}

/** Run a job — an LLM task — on a company (`POST …/jobs`). */
export async function runJob(
  fetcher: Fetcher,
  session: string,
  companyId: string,
  prompt: string,
  model?: string,
): Promise<JobResult> {
  const body: Record<string, unknown> = { prompt };
  if (model) body.model = model;
  return (await fetcher(`/api/paperclip/companies/${companyId}/jobs`, {
    method: "POST",
    body,
    headers: auth(session),
  })) as JobResult;
}

/** List a company's jobs (`GET …/jobs`). */
export async function listJobs(fetcher: Fetcher, session: string, companyId: string): Promise<Job[]> {
  const data = await fetcher(`/api/paperclip/companies/${companyId}/jobs`, { headers: auth(session) });
  return (Array.isArray(data) ? data : (data as { jobs?: Job[] }).jobs ?? []) as Job[];
}

// --- Hiring: agents (employees) hired via approval-gated governance ---
export type Agent = { id: string; name: string; role: string; status: string };
export type Approval = { id: string; type: string; status: string; subject_agent_id?: string | null };
export type HireResult = { agentId: string; status: string; approvalId?: string | null };

/** Hire an agent into a company (`POST …/agents`). Starts in `pending_approval` when the
 *  company requires board approval. */
export async function hireAgent(
  fetcher: Fetcher,
  session: string,
  companyId: string,
  name: string,
  role?: string,
  model?: string,
): Promise<HireResult> {
  const body: Record<string, unknown> = { name };
  if (role) body.role = role;
  if (model) body.model = model;
  return (await fetcher(`/api/paperclip/companies/${companyId}/agents`, {
    method: "POST",
    body,
    headers: auth(session),
  })) as HireResult;
}

/** List a company's agents (`GET …/agents`). */
export async function listAgents(fetcher: Fetcher, session: string, companyId: string): Promise<Agent[]> {
  const data = await fetcher(`/api/paperclip/companies/${companyId}/agents`, { headers: auth(session) });
  return (Array.isArray(data) ? data : (data as { agents?: Agent[] }).agents ?? []) as Agent[];
}

/** List a company's approvals (`GET …/approvals`). */
export async function listApprovals(fetcher: Fetcher, session: string, companyId: string): Promise<Approval[]> {
  const data = await fetcher(`/api/paperclip/companies/${companyId}/approvals`, { headers: auth(session) });
  return (Array.isArray(data) ? data : (data as { approvals?: Approval[] }).approvals ?? []) as Approval[];
}

/** Board decision on an approval (`POST /api/paperclip/approvals/:id/decide`). */
export async function decideApproval(
  fetcher: Fetcher,
  session: string,
  approvalId: string,
  approve: boolean,
): Promise<{ status: string }> {
  return (await fetcher(`/api/paperclip/approvals/${approvalId}/decide`, {
    method: "POST",
    body: { approve },
    headers: auth(session),
  })) as { status: string };
}

// ===== Phase 02/06/07: work backbone + costs + dashboard + agent lifecycle =====

export type Issue = { id: string; title: string; status: string; priority?: string; assignee_agent_id?: string | null };
export type Goal = { id: string; title: string; level: string; status: string };
export type Project = { id: string; name: string; status: string };
export type Comment = { id: string; body: string; author_user_id?: string | null; author_agent_id?: string | null; created_at: string };
export type Dashboard = Record<string, unknown>;
export type Activity = { id: string; action: string; actor_type: string; created_at: string };
export type CostEvent = { id: string; cost_cents: number; input_tokens: number; output_tokens: number; occurred_at: string };
export type Run = { id: string; agent_id: string; status: string; started_at?: string; finished_at?: string };

export async function createIssue(
  fetcher: Fetcher, session: string, companyId: string, title: string,
): Promise<{ issueId: string }> {
  return (await fetcher(`/api/paperclip/companies/${companyId}/issues`, {
    method: "POST", body: { title }, headers: auth(session),
  })) as { issueId: string };
}

export async function listIssues(fetcher: Fetcher, session: string, companyId: string): Promise<Issue[]> {
  const data = await fetcher(`/api/paperclip/companies/${companyId}/issues`, { headers: auth(session) });
  return (Array.isArray(data) ? data : (data as { issues?: Issue[] }).issues ?? []) as Issue[];
}

export async function addIssueComment(
  fetcher: Fetcher, session: string, issueId: string, body: string,
): Promise<{ commentId?: string }> {
  return (await fetcher(`/api/paperclip/issues/${issueId}/comments`, {
    method: "POST", body: { body }, headers: auth(session),
  })) as { commentId?: string };
}

export async function listIssueComments(fetcher: Fetcher, session: string, issueId: string): Promise<Comment[]> {
  const data = await fetcher(`/api/paperclip/issues/${issueId}/comments`, { headers: auth(session) });
  return (Array.isArray(data) ? data : (data as { comments?: Comment[] }).comments ?? []) as Comment[];
}

export async function createGoal(
  fetcher: Fetcher, session: string, companyId: string, title: string,
): Promise<{ goalId: string }> {
  return (await fetcher(`/api/paperclip/companies/${companyId}/goals`, {
    method: "POST", body: { title }, headers: auth(session),
  })) as { goalId: string };
}

export async function listGoals(fetcher: Fetcher, session: string, companyId: string): Promise<Goal[]> {
  const data = await fetcher(`/api/paperclip/companies/${companyId}/goals`, { headers: auth(session) });
  return (Array.isArray(data) ? data : (data as { goals?: Goal[] }).goals ?? []) as Goal[];
}

export async function createProject(
  fetcher: Fetcher, session: string, companyId: string, name: string,
): Promise<{ projectId: string }> {
  return (await fetcher(`/api/paperclip/companies/${companyId}/projects`, {
    method: "POST", body: { name }, headers: auth(session),
  })) as { projectId: string };
}

export async function listProjects(fetcher: Fetcher, session: string, companyId: string): Promise<Project[]> {
  const data = await fetcher(`/api/paperclip/companies/${companyId}/projects`, { headers: auth(session) });
  return (Array.isArray(data) ? data : (data as { projects?: Project[] }).projects ?? []) as Project[];
}

export async function pauseAgent(fetcher: Fetcher, session: string, agentId: string): Promise<{ status: string }> {
  return (await fetcher(`/api/paperclip/agents/${agentId}/pause`, { method: "POST", headers: auth(session) })) as { status: string };
}

export async function resumeAgent(fetcher: Fetcher, session: string, agentId: string): Promise<{ status: string }> {
  return (await fetcher(`/api/paperclip/agents/${agentId}/resume`, { method: "POST", headers: auth(session) })) as { status: string };
}

export async function terminateAgent(fetcher: Fetcher, session: string, agentId: string): Promise<{ status: string }> {
  return (await fetcher(`/api/paperclip/agents/${agentId}/terminate`, { method: "POST", headers: auth(session) })) as { status: string };
}

export async function getDashboard(fetcher: Fetcher, session: string, companyId: string): Promise<Dashboard> {
  return (await fetcher(`/api/paperclip/companies/${companyId}/dashboard`, { headers: auth(session) })) as Dashboard;
}

export async function listActivity(fetcher: Fetcher, session: string, companyId: string): Promise<Activity[]> {
  const data = await fetcher(`/api/paperclip/companies/${companyId}/activity`, { headers: auth(session) });
  return (Array.isArray(data) ? data : (data as { activity?: Activity[] }).activity ?? []) as Activity[];
}

export async function listCostEvents(fetcher: Fetcher, session: string, companyId: string): Promise<CostEvent[]> {
  const data = await fetcher(`/api/paperclip/companies/${companyId}/cost-events`, { headers: auth(session) });
  return (Array.isArray(data) ? data : (data as { costEvents?: CostEvent[] }).costEvents ?? []) as CostEvent[];
}

export async function listRuns(fetcher: Fetcher, session: string, companyId: string): Promise<Run[]> {
  const data = await fetcher(`/api/paperclip/companies/${companyId}/runs`, { headers: auth(session) });
  return (Array.isArray(data) ? data : (data as { runs?: Run[] }).runs ?? []) as Run[];
}
