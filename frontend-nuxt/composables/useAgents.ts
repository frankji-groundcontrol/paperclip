export type Agent = {
  id: string;
  name: string;
  role: string;
  status: string;
  adapterType: string;
};

/** ofetch-style request options for mutating calls. */
export type FetchOptions = { method?: string; body?: unknown };

/**
 * HTTP boundary — in the Nuxt app this is `$fetch` (ofetch). Reads pass just a
 * URL; mutations pass options (method, body).
 */
export type Fetcher = (url: string, options?: FetchOptions) => Promise<unknown>;

/** Loads a company's agents (`GET /api/companies/:companyId/agents`). */
export async function fetchAgents(fetcher: Fetcher, companyId: string): Promise<Agent[]> {
  const data = await fetcher(`/api/companies/${companyId}/agents`);
  return data as Agent[];
}

/** Creates an agent (`POST /api/companies/:companyId/agents`). */
export async function createAgent(
  fetcher: Fetcher,
  companyId: string,
  input: { name: string; role?: string; adapterType?: string },
): Promise<Agent> {
  const data = await fetcher(`/api/companies/${companyId}/agents`, {
    method: "POST",
    body: input,
  });
  return data as Agent;
}

/** Deletes an agent (`DELETE /api/companies/:companyId/agents/:agentId`). */
export async function deleteAgent(
  fetcher: Fetcher,
  companyId: string,
  agentId: string,
): Promise<void> {
  await fetcher(`/api/companies/${companyId}/agents/${agentId}`, {
    method: "DELETE",
  });
}
