export type SidebarBadges = {
  failedRuns: number;
  approvals: number;
  joinRequests: number;
  inbox: number;
};

/** HTTP boundary — in the Nuxt app this is `$fetch` (ofetch). */
export type Fetcher = (url: string) => Promise<unknown>;

/** Loads a company's sidebar badge counts (`GET /api/companies/:companyId/sidebar-badges`). */
export async function fetchSidebarBadges(
  fetcher: Fetcher,
  companyId: string,
): Promise<SidebarBadges> {
  const data = await fetcher(`/api/companies/${companyId}/sidebar-badges`);
  return data as SidebarBadges;
}
