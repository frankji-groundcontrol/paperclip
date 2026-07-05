export type BoardApiKeyRef = {
  id: string;
  name: string;
  expiresAt: string | null;
};

/** The create response includes the one-time plaintext `token`. */
export type CreatedBoardApiKey = BoardApiKeyRef & { token: string };

/** ofetch-style request options for mutating calls. */
export type FetchOptions = { method?: string; body?: unknown };

/**
 * HTTP boundary — in the Nuxt app this is `$fetch` (ofetch). Board user identity
 * is carried by the `X-Actor-User` header at the fetcher level.
 */
export type Fetcher = (url: string, options?: FetchOptions) => Promise<unknown>;

/** Loads the board user's API keys — references only (`GET /api/board-api-keys`). */
export async function fetchBoardApiKeys(fetcher: Fetcher): Promise<BoardApiKeyRef[]> {
  const data = await fetcher("/api/board-api-keys");
  return data as BoardApiKeyRef[];
}

/**
 * Creates a board API key (`POST /api/board-api-keys`). The response carries the
 * plaintext `token` exactly once — it is never returned by list.
 */
export async function createBoardApiKey(
  fetcher: Fetcher,
  input: { name?: string; expiresAt?: string | null },
): Promise<CreatedBoardApiKey> {
  const data = await fetcher("/api/board-api-keys", { method: "POST", body: input });
  return data as CreatedBoardApiKey;
}

/** Revokes a board API key (`DELETE /api/board-api-keys/:keyId`). */
export async function deleteBoardApiKey(fetcher: Fetcher, keyId: string): Promise<void> {
  await fetcher(`/api/board-api-keys/${keyId}`, { method: "DELETE" });
}
