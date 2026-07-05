export type Company = { id: string; name: string };

/** ofetch-style request options for mutating calls. */
export type FetchOptions = { method?: string; body?: unknown };

/**
 * HTTP boundary — in the Nuxt app this is `$fetch` (ofetch). Reads pass just a
 * URL; mutations pass options (method, body).
 */
export type Fetcher = (url: string, options?: FetchOptions) => Promise<unknown>;

/** Loads companies from the control-plane API (`GET /api/companies`). */
export async function fetchCompanies(fetcher: Fetcher): Promise<Company[]> {
  const data = await fetcher("/api/companies");
  return data as Company[];
}

/** Creates a company (`POST /api/companies`). */
export async function createCompany(
  fetcher: Fetcher,
  input: { name: string },
): Promise<Company> {
  const data = await fetcher("/api/companies", { method: "POST", body: input });
  return data as Company;
}

/** Renames/updates a company (`PATCH /api/companies/:companyId`). */
export async function updateCompany(
  fetcher: Fetcher,
  companyId: string,
  patch: { name?: string },
): Promise<Company> {
  const data = await fetcher(`/api/companies/${companyId}`, {
    method: "PATCH",
    body: patch,
  });
  return data as Company;
}

/** Deletes a company (`DELETE /api/companies/:companyId`). */
export async function deleteCompany(fetcher: Fetcher, companyId: string): Promise<void> {
  await fetcher(`/api/companies/${companyId}`, { method: "DELETE" });
}
