export type Run = { id: string; agentId: string; status: string };

/** ofetch-style request options for mutating calls. */
export type FetchOptions = { method?: string; body?: unknown };

/**
 * HTTP boundary — in the Nuxt app this is `$fetch` (ofetch). Reads pass just a
 * URL; mutations pass options (method, body).
 */
export type Fetcher = (url: string, options?: FetchOptions) => Promise<unknown>;

/** Loads a company's runs (`GET /api/companies/:companyId/runs`). */
export async function fetchRuns(fetcher: Fetcher, companyId: string): Promise<Run[]> {
  const data = await fetcher(`/api/companies/${companyId}/runs`);
  return data as Run[];
}

/** Starts a run (`POST /api/companies/:companyId/runs`). */
export async function createRun(
  fetcher: Fetcher,
  companyId: string,
  input: { agentId: string; status?: string },
): Promise<Run> {
  const data = await fetcher(`/api/companies/${companyId}/runs`, {
    method: "POST",
    body: input,
  });
  return data as Run;
}

/** Transitions a run's status (`PATCH /api/companies/:companyId/runs/:runId`). */
export async function updateRun(
  fetcher: Fetcher,
  companyId: string,
  runId: string,
  patch: { status?: string },
): Promise<Run> {
  const data = await fetcher(`/api/companies/${companyId}/runs/${runId}`, {
    method: "PATCH",
    body: patch,
  });
  return data as Run;
}
