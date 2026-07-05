export type DashboardSummary = {
  agents: { active: number; running: number; paused: number; error: number };
  tasks: { open: number; inProgress: number; blocked: number; done: number };
  approvals: { pending: number };
};

/** HTTP boundary — in the Nuxt app this is `$fetch` (ofetch). */
export type Fetcher = (url: string) => Promise<unknown>;

/** Loads a company's dashboard rollup (`GET /api/companies/:companyId/dashboard`). */
export async function fetchDashboard(
  fetcher: Fetcher,
  companyId: string,
): Promise<DashboardSummary> {
  const data = await fetcher(`/api/companies/${companyId}/dashboard`);
  return data as DashboardSummary;
}
