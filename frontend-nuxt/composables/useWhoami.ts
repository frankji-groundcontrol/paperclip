export type Whoami = { actor: "board" | "agent"; companyId?: string };

/** HTTP boundary — in the Nuxt app this is `$fetch` (ofetch). */
export type Fetcher = (url: string) => Promise<unknown>;

/** Loads the current actor (`GET /api/whoami`). */
export async function fetchWhoami(fetcher: Fetcher): Promise<Whoami> {
  const data = await fetcher("/api/whoami");
  return data as Whoami;
}
