export type SecretRef = { id: string; name: string; provider: string };

/** ofetch-style request options for mutating calls. */
export type FetchOptions = { method?: string; body?: unknown };

/**
 * HTTP boundary — in the Nuxt app this is `$fetch` (ofetch). Reads pass just a
 * URL; mutations pass options (method, body).
 */
export type Fetcher = (url: string, options?: FetchOptions) => Promise<unknown>;

/** Loads a company's secret references (`GET /api/companies/:companyId/secrets`). */
export async function fetchSecrets(fetcher: Fetcher, companyId: string): Promise<SecretRef[]> {
  const data = await fetcher(`/api/companies/${companyId}/secrets`);
  return data as SecretRef[];
}

/**
 * Creates a secret (`POST /api/companies/:companyId/secrets`). The value is sent
 * but never returned — the backend responds with a reference only.
 */
export async function createSecret(
  fetcher: Fetcher,
  companyId: string,
  input: { name: string; value: string; provider?: string },
): Promise<SecretRef> {
  const data = await fetcher(`/api/companies/${companyId}/secrets`, {
    method: "POST",
    body: input,
  });
  return data as SecretRef;
}

/** Deletes a secret (`DELETE /api/companies/:companyId/secrets/:secretId`). */
export async function deleteSecret(
  fetcher: Fetcher,
  companyId: string,
  secretId: string,
): Promise<void> {
  await fetcher(`/api/companies/${companyId}/secrets/${secretId}`, {
    method: "DELETE",
  });
}
