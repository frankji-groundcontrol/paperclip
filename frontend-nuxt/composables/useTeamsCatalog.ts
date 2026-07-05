export type CatalogTeam = {
  id: string;
  key: string;
  name: string;
  description?: string;
  category?: string;
  kind?: string;
  tags?: string[];
};

export type InstalledTeam = {
  companyId?: string;
  catalogId: string;
  name: string;
};

/** ofetch-style request options for mutating calls. */
export type FetchOptions = { method?: string; body?: unknown };

/**
 * HTTP boundary — in the Nuxt app this is `$fetch` (ofetch). Reads pass just a
 * URL; mutations pass options (method, body).
 */
export type Fetcher = (url: string, options?: FetchOptions) => Promise<unknown>;

export type CatalogFilters = { kind?: string; category?: string; q?: string };

/** Loads the bundled catalog (`GET /api/teams/catalog`), with optional filters. */
export async function fetchCatalog(
  fetcher: Fetcher,
  filters: CatalogFilters = {},
): Promise<CatalogTeam[]> {
  const params = new URLSearchParams();
  if (filters.kind !== undefined) params.set("kind", filters.kind);
  if (filters.category !== undefined) params.set("category", filters.category);
  if (filters.q !== undefined) params.set("q", filters.q);
  const query = params.toString();
  const data = await fetcher(`/api/teams/catalog${query ? `?${query}` : ""}`);
  return data as CatalogTeam[];
}

/** Loads a company's installed catalog teams. */
export async function fetchInstalledTeams(
  fetcher: Fetcher,
  companyId: string,
): Promise<InstalledTeam[]> {
  const data = await fetcher(`/api/companies/${companyId}/teams/catalog/installed`);
  return data as InstalledTeam[];
}

/** Installs a catalog team into a company. */
export async function installTeam(
  fetcher: Fetcher,
  companyId: string,
  catalogId: string,
): Promise<InstalledTeam> {
  const data = await fetcher(
    `/api/companies/${companyId}/teams/catalog/${catalogId}/install`,
    { method: "POST", body: {} },
  );
  return data as InstalledTeam;
}
