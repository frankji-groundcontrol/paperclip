export type JoinRequest = {
  id: string;
  companyId?: string;
  requestType: string;
  requesterName?: string | null;
  status: string;
};

/** ofetch-style request options for mutating calls. */
export type FetchOptions = { method?: string; body?: unknown };

/**
 * HTTP boundary — in the Nuxt app this is `$fetch` (ofetch). Reads pass just a
 * URL; mutations pass options (method, body).
 */
export type Fetcher = (url: string, options?: FetchOptions) => Promise<unknown>;

/** Optional list filters for join-requests. */
export type JoinRequestFilters = { status?: string; requestType?: string };

/**
 * Loads a company's join-requests (`GET /api/companies/:companyId/join-requests`),
 * with optional filters appended in a stable order (status, requestType).
 */
export async function fetchJoinRequests(
  fetcher: Fetcher,
  companyId: string,
  filters: JoinRequestFilters = {},
): Promise<JoinRequest[]> {
  const params = new URLSearchParams();
  if (filters.status !== undefined) params.set("status", filters.status);
  if (filters.requestType !== undefined) params.set("requestType", filters.requestType);
  const query = params.toString();
  const url = `/api/companies/${companyId}/join-requests${query ? `?${query}` : ""}`;
  const data = await fetcher(url);
  return data as JoinRequest[];
}

/** Creates a join-request (`POST /api/companies/:companyId/join-requests`). */
export async function createJoinRequest(
  fetcher: Fetcher,
  companyId: string,
  input: { requestType: string; requesterName?: string },
): Promise<JoinRequest> {
  const data = await fetcher(`/api/companies/${companyId}/join-requests`, {
    method: "POST",
    body: input,
  });
  return data as JoinRequest;
}

/** Approves a join-request (`POST /…/join-requests/:requestId/approve`). */
export async function approveJoinRequest(
  fetcher: Fetcher,
  companyId: string,
  requestId: string,
): Promise<JoinRequest> {
  const data = await fetcher(
    `/api/companies/${companyId}/join-requests/${requestId}/approve`,
    { method: "POST" },
  );
  return data as JoinRequest;
}

/** Rejects a join-request (`POST /…/join-requests/:requestId/reject`). */
export async function rejectJoinRequest(
  fetcher: Fetcher,
  companyId: string,
  requestId: string,
): Promise<JoinRequest> {
  const data = await fetcher(
    `/api/companies/${companyId}/join-requests/${requestId}/reject`,
    { method: "POST" },
  );
  return data as JoinRequest;
}
