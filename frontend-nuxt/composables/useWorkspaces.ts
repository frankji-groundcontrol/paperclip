export type Workspace = { id: string; issueId: string; status: string };

/** ofetch-style request options for mutating calls. */
export type FetchOptions = { method?: string; body?: unknown };

/**
 * HTTP boundary — in the Nuxt app this is `$fetch` (ofetch). Reads pass just a
 * URL; mutations pass options (method, body).
 */
export type Fetcher = (url: string, options?: FetchOptions) => Promise<unknown>;

/** Loads a company's execution workspaces (`GET /api/companies/:companyId/workspaces`). */
export async function fetchWorkspaces(fetcher: Fetcher, companyId: string): Promise<Workspace[]> {
  const data = await fetcher(`/api/companies/${companyId}/workspaces`);
  return data as Workspace[];
}

/** Starts a workspace (`POST /api/companies/:companyId/workspaces`). */
export async function createWorkspace(
  fetcher: Fetcher,
  companyId: string,
  input: { issueId: string; status?: string },
): Promise<Workspace> {
  const data = await fetcher(`/api/companies/${companyId}/workspaces`, {
    method: "POST",
    body: input,
  });
  return data as Workspace;
}

/** Transitions a workspace's lifecycle (`PATCH /api/companies/:companyId/workspaces/:workspaceId`). */
export async function updateWorkspace(
  fetcher: Fetcher,
  companyId: string,
  workspaceId: string,
  patch: { status?: string },
): Promise<Workspace> {
  const data = await fetcher(`/api/companies/${companyId}/workspaces/${workspaceId}`, {
    method: "PATCH",
    body: patch,
  });
  return data as Workspace;
}
