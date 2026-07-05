export type Environment = {
  id: string;
  companyId: string;
  name: string;
  description?: string | null;
  driver: string;
  status: string;
  config?: Record<string, unknown>;
  envVars?: Record<string, unknown>;
  metadata?: Record<string, unknown> | null;
};

/** ofetch-style request options for mutating calls. */
export type FetchOptions = { method?: string; body?: unknown };

/**
 * HTTP boundary — in the Nuxt app this is `$fetch` (ofetch). Reads pass just a
 * URL; mutations pass options (method, body).
 */
export type Fetcher = (url: string, options?: FetchOptions) => Promise<unknown>;

/** Loads a company's environments (`GET /api/companies/:companyId/environments`). */
export async function fetchEnvironments(
  fetcher: Fetcher,
  companyId: string,
): Promise<Environment[]> {
  const data = await fetcher(`/api/companies/${companyId}/environments`);
  return data as Environment[];
}

/** Creates an environment (`POST /api/companies/:companyId/environments`). */
export async function createEnvironment(
  fetcher: Fetcher,
  companyId: string,
  input: {
    name: string;
    driver: string;
    status?: string;
    description?: string;
    config?: Record<string, unknown>;
    envVars?: Record<string, unknown>;
  },
): Promise<Environment> {
  const data = await fetcher(`/api/companies/${companyId}/environments`, {
    method: "POST",
    body: input,
  });
  return data as Environment;
}

/** Updates an environment (`PATCH /api/companies/:companyId/environments/:environmentId`). */
export async function updateEnvironment(
  fetcher: Fetcher,
  companyId: string,
  environmentId: string,
  patch: {
    name?: string;
    driver?: string;
    status?: string;
    description?: string;
    config?: Record<string, unknown>;
    envVars?: Record<string, unknown>;
  },
): Promise<Environment> {
  const data = await fetcher(`/api/companies/${companyId}/environments/${environmentId}`, {
    method: "PATCH",
    body: patch,
  });
  return data as Environment;
}

/** Deletes an environment (`DELETE /api/companies/:companyId/environments/:environmentId`). */
export async function deleteEnvironment(
  fetcher: Fetcher,
  companyId: string,
  environmentId: string,
): Promise<void> {
  await fetcher(`/api/companies/${companyId}/environments/${environmentId}`, {
    method: "DELETE",
  });
}
