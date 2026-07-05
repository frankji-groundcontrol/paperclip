export type Routine = { id: string; title: string; status: string };

/** ofetch-style request options for mutating calls. */
export type FetchOptions = { method?: string; body?: unknown };

/**
 * HTTP boundary — in the Nuxt app this is `$fetch` (ofetch). Reads pass just a
 * URL; mutations pass options (method, body).
 */
export type Fetcher = (url: string, options?: FetchOptions) => Promise<unknown>;

/** Loads a company's routines (`GET /api/companies/:companyId/routines`). */
export async function fetchRoutines(fetcher: Fetcher, companyId: string): Promise<Routine[]> {
  const data = await fetcher(`/api/companies/${companyId}/routines`);
  return data as Routine[];
}

/** Creates a routine (`POST /api/companies/:companyId/routines`). */
export async function createRoutine(
  fetcher: Fetcher,
  companyId: string,
  input: { title: string; status?: string },
): Promise<Routine> {
  const data = await fetcher(`/api/companies/${companyId}/routines`, {
    method: "POST",
    body: input,
  });
  return data as Routine;
}

/** Deletes a routine (`DELETE /api/companies/:companyId/routines/:routineId`). */
export async function deleteRoutine(
  fetcher: Fetcher,
  companyId: string,
  routineId: string,
): Promise<void> {
  await fetcher(`/api/companies/${companyId}/routines/${routineId}`, {
    method: "DELETE",
  });
}
