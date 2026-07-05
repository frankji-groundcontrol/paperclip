export type SidebarOrderPreference = {
  orderedIds: string[];
  updatedAt: string | null;
};

/** ofetch-style request options for mutating calls. */
export type FetchOptions = { method?: string; body?: unknown };

/**
 * HTTP boundary — in the Nuxt app this is `$fetch` (ofetch). Reads pass just a
 * URL; mutations pass options (method, body). The board user identity is carried
 * by the `X-Actor-User` header at the fetcher level.
 */
export type Fetcher = (url: string, options?: FetchOptions) => Promise<unknown>;

/** Loads the board user's project order (`GET /api/companies/:companyId/sidebar-preferences/me`). */
export async function fetchProjectOrder(
  fetcher: Fetcher,
  companyId: string,
): Promise<SidebarOrderPreference> {
  const data = await fetcher(`/api/companies/${companyId}/sidebar-preferences/me`);
  return data as SidebarOrderPreference;
}

/** Saves the board user's project order (`PUT /api/companies/:companyId/sidebar-preferences/me`). */
export async function saveProjectOrder(
  fetcher: Fetcher,
  companyId: string,
  orderedIds: string[],
): Promise<SidebarOrderPreference> {
  const data = await fetcher(`/api/companies/${companyId}/sidebar-preferences/me`, {
    method: "PUT",
    body: { orderedIds },
  });
  return data as SidebarOrderPreference;
}
