export type ActivityEvent = {
  id: string;
  companyId: string;
  actorType: string;
  actorId: string;
  action: string;
  entityType: string;
  entityId: string;
  agentId?: string | null;
  runId?: string | null;
  details?: Record<string, unknown> | null;
};

/** ofetch-style request options for mutating calls. */
export type FetchOptions = { method?: string; body?: unknown };

/**
 * HTTP boundary — in the Nuxt app this is `$fetch` (ofetch). Reads pass just a
 * URL; mutations pass options (method, body).
 */
export type Fetcher = (url: string, options?: FetchOptions) => Promise<unknown>;

/** Optional server-side filters for the activity feed. */
export type ActivityFilters = {
  agentId?: string;
  entityType?: string;
  entityId?: string;
  limit?: number;
};

/**
 * Loads a company's activity feed (`GET /api/companies/:companyId/activity`).
 * Filters are appended as query params in a stable order (agentId, entityType,
 * entityId, limit) so callers get deterministic URLs.
 */
export async function fetchActivity(
  fetcher: Fetcher,
  companyId: string,
  filters: ActivityFilters = {},
): Promise<ActivityEvent[]> {
  const params = new URLSearchParams();
  if (filters.agentId !== undefined) params.set("agentId", filters.agentId);
  if (filters.entityType !== undefined) params.set("entityType", filters.entityType);
  if (filters.entityId !== undefined) params.set("entityId", filters.entityId);
  if (filters.limit !== undefined) params.set("limit", String(filters.limit));

  const query = params.toString();
  const url = `/api/companies/${companyId}/activity${query ? `?${query}` : ""}`;
  const data = await fetcher(url);
  return data as ActivityEvent[];
}

/**
 * Records an activity event (`POST /api/companies/:companyId/activity`).
 * Board-only on the backend; `actorType` defaults to "system" server-side.
 */
export async function createActivity(
  fetcher: Fetcher,
  companyId: string,
  input: {
    actorId: string;
    action: string;
    entityType: string;
    entityId: string;
    actorType?: string;
    agentId?: string;
    details?: Record<string, unknown>;
  },
): Promise<ActivityEvent> {
  const data = await fetcher(`/api/companies/${companyId}/activity`, {
    method: "POST",
    body: input,
  });
  return data as ActivityEvent;
}
