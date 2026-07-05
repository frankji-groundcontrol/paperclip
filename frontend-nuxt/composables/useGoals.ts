export type Goal = {
  id: string;
  title: string;
  level: string;
  status: string;
  description?: string | null;
  parentId?: string | null;
  ownerAgentId?: string | null;
};

/** ofetch-style request options for mutating calls. */
export type FetchOptions = { method?: string; body?: unknown };

/**
 * HTTP boundary — in the Nuxt app this is `$fetch` (ofetch). Reads pass just a
 * URL; mutations pass options (method, body).
 */
export type Fetcher = (url: string, options?: FetchOptions) => Promise<unknown>;

/** Loads a company's goals (`GET /api/companies/:companyId/goals`). */
export async function fetchGoals(fetcher: Fetcher, companyId: string): Promise<Goal[]> {
  const data = await fetcher(`/api/companies/${companyId}/goals`);
  return data as Goal[];
}

/** Creates a goal (`POST /api/companies/:companyId/goals`). */
export async function createGoal(
  fetcher: Fetcher,
  companyId: string,
  input: { title: string; level?: string; status?: string; description?: string; parentId?: string },
): Promise<Goal> {
  const data = await fetcher(`/api/companies/${companyId}/goals`, {
    method: "POST",
    body: input,
  });
  return data as Goal;
}

/** Updates a goal (`PATCH /api/companies/:companyId/goals/:goalId`). */
export async function updateGoal(
  fetcher: Fetcher,
  companyId: string,
  goalId: string,
  patch: { title?: string; level?: string; status?: string; description?: string; parentId?: string },
): Promise<Goal> {
  const data = await fetcher(`/api/companies/${companyId}/goals/${goalId}`, {
    method: "PATCH",
    body: patch,
  });
  return data as Goal;
}

/** Deletes a goal (`DELETE /api/companies/:companyId/goals/:goalId`). */
export async function deleteGoal(
  fetcher: Fetcher,
  companyId: string,
  goalId: string,
): Promise<void> {
  await fetcher(`/api/companies/${companyId}/goals/${goalId}`, {
    method: "DELETE",
  });
}
