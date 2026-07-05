export type CloudUpstream = {
  id: string;
  companyId: string;
  remoteUrl: string;
  status: string;
};

/** ofetch-style request options for mutating calls. */
export type FetchOptions = { method?: string; body?: unknown };

/**
 * HTTP boundary — in the Nuxt app this is `$fetch` (ofetch). Reads pass just a
 * URL; mutations pass options (method, body).
 */
export type Fetcher = (url: string, options?: FetchOptions) => Promise<unknown>;

/**
 * Loads cloud upstream connections (`GET /api/cloud-upstreams`), optionally
 * filtered by company. Gated server-side by the `enableCloudSync` instance flag.
 */
export async function fetchCloudUpstreams(
  fetcher: Fetcher,
  companyId?: string,
): Promise<CloudUpstream[]> {
  const query = companyId !== undefined ? `?companyId=${companyId}` : "";
  const data = await fetcher(`/api/cloud-upstreams${query}`);
  return data as CloudUpstream[];
}

/** Registers a cloud upstream (`POST /api/cloud-upstreams/register`). */
export async function registerCloudUpstream(
  fetcher: Fetcher,
  input: { companyId: string; remoteUrl: string },
): Promise<CloudUpstream> {
  const data = await fetcher("/api/cloud-upstreams/register", {
    method: "POST",
    body: input,
  });
  return data as CloudUpstream;
}

/** Removes a cloud upstream (`DELETE /api/cloud-upstreams/:connectionId`). */
export async function deleteCloudUpstream(fetcher: Fetcher, connectionId: string): Promise<void> {
  await fetcher(`/api/cloud-upstreams/${connectionId}`, { method: "DELETE" });
}
