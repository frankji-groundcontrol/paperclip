export type Approval = { id: string; issueId: string; status: string };

/** ofetch-style request options for mutating calls. */
export type FetchOptions = { method?: string; body?: unknown };

/**
 * HTTP boundary — in the Nuxt app this is `$fetch` (ofetch). Reads pass just a
 * URL; mutations pass options (method, body).
 */
export type Fetcher = (url: string, options?: FetchOptions) => Promise<unknown>;

/** Loads a company's approvals (`GET /api/companies/:companyId/approvals`). */
export async function fetchApprovals(fetcher: Fetcher, companyId: string): Promise<Approval[]> {
  const data = await fetcher(`/api/companies/${companyId}/approvals`);
  return data as Approval[];
}

/** Requests an approval gate (`POST /api/companies/:companyId/approvals`). */
export async function createApproval(
  fetcher: Fetcher,
  companyId: string,
  input: { issueId: string; status?: string },
): Promise<Approval> {
  const data = await fetcher(`/api/companies/${companyId}/approvals`, {
    method: "POST",
    body: input,
  });
  return data as Approval;
}

/** Records a decision (`PATCH /api/companies/:companyId/approvals/:approvalId`). */
export async function updateApproval(
  fetcher: Fetcher,
  companyId: string,
  approvalId: string,
  patch: { status?: string },
): Promise<Approval> {
  const data = await fetcher(`/api/companies/${companyId}/approvals/${approvalId}`, {
    method: "PATCH",
    body: patch,
  });
  return data as Approval;
}
