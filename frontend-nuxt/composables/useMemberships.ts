export type ResourceMemberships = {
  projectMemberships: Record<string, "joined" | "left">;
  agentMemberships: Record<string, "joined" | "left">;
  updatedAt: string | null;
};

export type MembershipState = "joined" | "left";

export type MembershipResult = {
  resourceType: string;
  resourceId: string;
  state: MembershipState;
  updatedAt?: string;
};

/** ofetch-style request options for mutating calls. */
export type FetchOptions = { method?: string; body?: unknown };

/**
 * HTTP boundary — in the Nuxt app this is `$fetch` (ofetch). Reads pass just a
 * URL; mutations pass options (method, body). Board user identity is carried by
 * the `X-Actor-User` header at the fetcher level.
 */
export type Fetcher = (url: string, options?: FetchOptions) => Promise<unknown>;

/** Loads the board user's resource memberships (`GET /api/companies/:companyId/resource-memberships/me`). */
export async function fetchMemberships(
  fetcher: Fetcher,
  companyId: string,
): Promise<ResourceMemberships> {
  const data = await fetcher(`/api/companies/${companyId}/resource-memberships/me`);
  return data as ResourceMemberships;
}

/** Joins/leaves a project (`PUT /api/companies/:companyId/resource-memberships/me/projects/:projectId`). */
export async function setProjectMembership(
  fetcher: Fetcher,
  companyId: string,
  projectId: string,
  state: MembershipState,
): Promise<MembershipResult> {
  const data = await fetcher(
    `/api/companies/${companyId}/resource-memberships/me/projects/${projectId}`,
    { method: "PUT", body: { state } },
  );
  return data as MembershipResult;
}

/** Joins/leaves an agent (`PUT /api/companies/:companyId/resource-memberships/me/agents/:agentId`). */
export async function setAgentMembership(
  fetcher: Fetcher,
  companyId: string,
  agentId: string,
  state: MembershipState,
): Promise<MembershipResult> {
  const data = await fetcher(
    `/api/companies/${companyId}/resource-memberships/me/agents/${agentId}`,
    { method: "PUT", body: { state } },
  );
  return data as MembershipResult;
}
