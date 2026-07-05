export type Invite = {
  id: string;
  companyId?: string;
  token: string;
  allowedJoinTypes: string;
  humanRole?: string | null;
  agentMessage?: string | null;
  state: string;
};

/** ofetch-style request options for mutating calls. */
export type FetchOptions = { method?: string; body?: unknown };

/**
 * HTTP boundary — in the Nuxt app this is `$fetch` (ofetch). Reads pass just a
 * URL; mutations pass options (method, body).
 */
export type Fetcher = (url: string, options?: FetchOptions) => Promise<unknown>;

/**
 * Loads a company's invites (`GET /api/companies/:companyId/invites`), optionally
 * filtered by state (active/revoked/accepted/expired).
 */
export async function fetchInvites(
  fetcher: Fetcher,
  companyId: string,
  state?: string,
): Promise<Invite[]> {
  const query = state !== undefined ? `?state=${state}` : "";
  const data = await fetcher(`/api/companies/${companyId}/invites${query}`);
  return data as Invite[];
}

/** Creates an invite (`POST /api/companies/:companyId/invites`). */
export async function createInvite(
  fetcher: Fetcher,
  companyId: string,
  input: { allowedJoinTypes?: string; humanRole?: string; agentMessage?: string },
): Promise<Invite> {
  const data = await fetcher(`/api/companies/${companyId}/invites`, {
    method: "POST",
    body: input,
  });
  return data as Invite;
}

/** Revokes an invite (`POST /api/companies/:companyId/invites/:inviteId/revoke`). */
export async function revokeInvite(
  fetcher: Fetcher,
  companyId: string,
  inviteId: string,
): Promise<Invite> {
  const data = await fetcher(`/api/companies/${companyId}/invites/${inviteId}/revoke`, {
    method: "POST",
  });
  return data as Invite;
}
