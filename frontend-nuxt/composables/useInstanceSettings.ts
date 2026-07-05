export type InstanceSettings = {
  id: string;
  defaultEnvironmentId: string | null;
  general: Record<string, unknown>;
  experimental: Record<string, unknown>;
};

/** ofetch-style request options for mutating calls. */
export type FetchOptions = { method?: string; body?: unknown };

/**
 * HTTP boundary — in the Nuxt app this is `$fetch` (ofetch). Reads pass just a
 * URL; mutations pass options (method, body).
 */
export type Fetcher = (url: string, options?: FetchOptions) => Promise<unknown>;

/** Loads the instance settings singleton (`GET /api/instance/settings`). */
export async function fetchInstanceSettings(fetcher: Fetcher): Promise<InstanceSettings> {
  const data = await fetcher("/api/instance/settings");
  return data as InstanceSettings;
}

/** Patches the top-level instance settings (`PATCH /api/instance/settings`). */
export async function updateInstanceSettings(
  fetcher: Fetcher,
  patch: { defaultEnvironmentId?: string | null },
): Promise<InstanceSettings> {
  const data = await fetcher("/api/instance/settings", { method: "PATCH", body: patch });
  return data as InstanceSettings;
}

/** Patches the general block (`PATCH /api/instance/settings/general`). */
export async function updateInstanceGeneral(
  fetcher: Fetcher,
  patch: Record<string, unknown>,
): Promise<Record<string, unknown>> {
  const data = await fetcher("/api/instance/settings/general", { method: "PATCH", body: patch });
  return data as Record<string, unknown>;
}

/** Patches the experimental block (`PATCH /api/instance/settings/experimental`). */
export async function updateInstanceExperimental(
  fetcher: Fetcher,
  patch: Record<string, unknown>,
): Promise<Record<string, unknown>> {
  const data = await fetcher("/api/instance/settings/experimental", {
    method: "PATCH",
    body: patch,
  });
  return data as Record<string, unknown>;
}
