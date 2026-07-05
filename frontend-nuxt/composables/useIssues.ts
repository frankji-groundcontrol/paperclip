export type Issue = { id: string; title: string; status: string };

/** ofetch-style request options for mutating calls. */
export type FetchOptions = { method?: string; body?: unknown };

/**
 * HTTP boundary — in the Nuxt app this is `$fetch` (ofetch). Reads pass just a
 * URL; mutations pass options (method, body).
 */
export type Fetcher = (url: string, options?: FetchOptions) => Promise<unknown>;

/** Loads a company's issues (`GET /api/companies/:companyId/issues`). */
export async function fetchIssues(fetcher: Fetcher, companyId: string): Promise<Issue[]> {
  const data = await fetcher(`/api/companies/${companyId}/issues`);
  return data as Issue[];
}

/** Creates an issue (`POST /api/companies/:companyId/issues`). */
export async function createIssue(
  fetcher: Fetcher,
  companyId: string,
  input: { title: string; status?: string },
): Promise<Issue> {
  const data = await fetcher(`/api/companies/${companyId}/issues`, {
    method: "POST",
    body: input,
  });
  return data as Issue;
}

/** Deletes an issue (`DELETE /api/companies/:companyId/issues/:issueId`). */
export async function deleteIssue(
  fetcher: Fetcher,
  companyId: string,
  issueId: string,
): Promise<void> {
  await fetcher(`/api/companies/${companyId}/issues/${issueId}`, {
    method: "DELETE",
  });
}
