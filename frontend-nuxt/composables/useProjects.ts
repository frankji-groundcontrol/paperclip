export type Project = { id: string; name: string; status: string };

/** ofetch-style request options for mutating calls. */
export type FetchOptions = { method?: string; body?: unknown };

/**
 * HTTP boundary — in the Nuxt app this is `$fetch` (ofetch). Reads pass just a
 * URL; mutations pass options (method, body).
 */
export type Fetcher = (url: string, options?: FetchOptions) => Promise<unknown>;

/** Loads a company's projects (`GET /api/companies/:companyId/projects`). */
export async function fetchProjects(fetcher: Fetcher, companyId: string): Promise<Project[]> {
  const data = await fetcher(`/api/companies/${companyId}/projects`);
  return data as Project[];
}

/** Creates a project (`POST /api/companies/:companyId/projects`). */
export async function createProject(
  fetcher: Fetcher,
  companyId: string,
  input: { name: string; status?: string },
): Promise<Project> {
  const data = await fetcher(`/api/companies/${companyId}/projects`, {
    method: "POST",
    body: input,
  });
  return data as Project;
}

/** Deletes a project (`DELETE /api/companies/:companyId/projects/:projectId`). */
export async function deleteProject(
  fetcher: Fetcher,
  companyId: string,
  projectId: string,
): Promise<void> {
  await fetcher(`/api/companies/${companyId}/projects/${projectId}`, {
    method: "DELETE",
  });
}
