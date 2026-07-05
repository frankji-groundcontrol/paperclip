export type CompanySkill = {
  id: string;
  companyId?: string;
  key: string;
  name: string;
  description?: string | null;
  categories?: string[];
  starCount: number;
};

/** ofetch-style request options for mutating calls. */
export type FetchOptions = { method?: string; body?: unknown };

/**
 * HTTP boundary — in the Nuxt app this is `$fetch` (ofetch). Reads pass just a
 * URL; mutations pass options (method, body).
 */
export type Fetcher = (url: string, options?: FetchOptions) => Promise<unknown>;

export type SkillFilters = { q?: string; category?: string };

/** Loads a company's skills (`GET /api/companies/:companyId/skills`), with filters. */
export async function fetchSkills(
  fetcher: Fetcher,
  companyId: string,
  filters: SkillFilters = {},
): Promise<CompanySkill[]> {
  const params = new URLSearchParams();
  if (filters.q !== undefined) params.set("q", filters.q);
  if (filters.category !== undefined) params.set("category", filters.category);
  const query = params.toString();
  const data = await fetcher(`/api/companies/${companyId}/skills${query ? `?${query}` : ""}`);
  return data as CompanySkill[];
}

/** Creates a skill (`POST /api/companies/:companyId/skills`). */
export async function createSkill(
  fetcher: Fetcher,
  companyId: string,
  input: { key: string; name: string; description?: string; categories?: string[] },
): Promise<CompanySkill> {
  const data = await fetcher(`/api/companies/${companyId}/skills`, {
    method: "POST",
    body: input,
  });
  return data as CompanySkill;
}

/** Loads one skill (`GET /api/companies/:companyId/skills/:skillId`). */
export async function fetchSkill(
  fetcher: Fetcher,
  companyId: string,
  skillId: string,
): Promise<CompanySkill> {
  const data = await fetcher(`/api/companies/${companyId}/skills/${skillId}`);
  return data as CompanySkill;
}

/** Stars a skill (`POST /…/skills/:skillId/star`). */
export async function starSkill(
  fetcher: Fetcher,
  companyId: string,
  skillId: string,
): Promise<CompanySkill> {
  const data = await fetcher(`/api/companies/${companyId}/skills/${skillId}/star`, {
    method: "POST",
  });
  return data as CompanySkill;
}

/** Unstars a skill (`DELETE /…/skills/:skillId/star`). */
export async function unstarSkill(
  fetcher: Fetcher,
  companyId: string,
  skillId: string,
): Promise<CompanySkill> {
  const data = await fetcher(`/api/companies/${companyId}/skills/${skillId}/star`, {
    method: "DELETE",
  });
  return data as CompanySkill;
}
