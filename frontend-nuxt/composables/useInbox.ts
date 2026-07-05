export type InboxDismissal = {
  companyId?: string;
  userId?: string;
  itemKey: string;
  dismissedAt: string;
};

/** ofetch-style request options for mutating calls. */
export type FetchOptions = { method?: string; body?: unknown };

/**
 * HTTP boundary — in the Nuxt app this is `$fetch` (ofetch). Reads pass just a
 * URL; mutations pass options (method, body). The board user identity is carried
 * by the `X-Actor-User` header at the fetcher level.
 */
export type Fetcher = (url: string, options?: FetchOptions) => Promise<unknown>;

/** Loads the board user's inbox dismissals (`GET /api/companies/:companyId/inbox-dismissals`). */
export async function fetchInboxDismissals(
  fetcher: Fetcher,
  companyId: string,
): Promise<InboxDismissal[]> {
  const data = await fetcher(`/api/companies/${companyId}/inbox-dismissals`);
  return data as InboxDismissal[];
}

/** Dismisses an inbox item (`POST /api/companies/:companyId/inbox-dismissals`). */
export async function dismissInboxItem(
  fetcher: Fetcher,
  companyId: string,
  itemKey: string,
): Promise<InboxDismissal> {
  const data = await fetcher(`/api/companies/${companyId}/inbox-dismissals`, {
    method: "POST",
    body: { itemKey },
  });
  return data as InboxDismissal;
}
