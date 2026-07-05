export type CompanyPlugin = { id: string; pluginId: string; enabled: boolean };

/** ofetch-style request options for mutating calls. */
export type FetchOptions = { method?: string; body?: unknown };

/**
 * HTTP boundary — in the Nuxt app this is `$fetch` (ofetch). Reads pass just a
 * URL; mutations pass options (method, body).
 */
export type Fetcher = (url: string, options?: FetchOptions) => Promise<unknown>;

/** Loads a company's installed plugins (`GET /api/companies/:companyId/plugins`). */
export async function fetchPlugins(fetcher: Fetcher, companyId: string): Promise<CompanyPlugin[]> {
  const data = await fetcher(`/api/companies/${companyId}/plugins`);
  return data as CompanyPlugin[];
}

/** Toggles a plugin (`PATCH /api/companies/:companyId/plugins/:pluginId`). */
export async function updatePlugin(
  fetcher: Fetcher,
  companyId: string,
  pluginId: string,
  patch: { enabled?: boolean },
): Promise<CompanyPlugin> {
  const data = await fetcher(`/api/companies/${companyId}/plugins/${pluginId}`, {
    method: "PATCH",
    body: patch,
  });
  return data as CompanyPlugin;
}

/** Uninstalls a plugin (`DELETE /api/companies/:companyId/plugins/:pluginId`). */
export async function deletePlugin(
  fetcher: Fetcher,
  companyId: string,
  pluginId: string,
): Promise<void> {
  await fetcher(`/api/companies/${companyId}/plugins/${pluginId}`, {
    method: "DELETE",
  });
}
