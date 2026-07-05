export type CompanyAdapter = { id: string; adapterType: string; enabled: boolean };

/** ofetch-style request options for mutating calls. */
export type FetchOptions = { method?: string; body?: unknown };

/**
 * HTTP boundary — in the Nuxt app this is `$fetch` (ofetch). Reads pass just a
 * URL; mutations pass options (method, body).
 */
export type Fetcher = (url: string, options?: FetchOptions) => Promise<unknown>;

/** Loads a company's configured adapters (`GET /api/companies/:companyId/adapters`). */
export async function fetchAdapters(fetcher: Fetcher, companyId: string): Promise<CompanyAdapter[]> {
  const data = await fetcher(`/api/companies/${companyId}/adapters`);
  return data as CompanyAdapter[];
}

/** Toggles an adapter (`PATCH /api/companies/:companyId/adapters/:adapterId`). */
export async function updateAdapter(
  fetcher: Fetcher,
  companyId: string,
  adapterId: string,
  patch: { enabled?: boolean },
): Promise<CompanyAdapter> {
  const data = await fetcher(`/api/companies/${companyId}/adapters/${adapterId}`, {
    method: "PATCH",
    body: patch,
  });
  return data as CompanyAdapter;
}

/** Removes an adapter (`DELETE /api/companies/:companyId/adapters/:adapterId`). */
export async function deleteAdapter(
  fetcher: Fetcher,
  companyId: string,
  adapterId: string,
): Promise<void> {
  await fetcher(`/api/companies/${companyId}/adapters/${adapterId}`, {
    method: "DELETE",
  });
}
