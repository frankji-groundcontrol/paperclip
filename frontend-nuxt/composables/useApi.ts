/** ofetch-style request options for mutating calls. */
export type FetchOptions = {
  method?: string;
  body?: unknown;
  headers?: Record<string, string>;
};

/** The HTTP boundary every domain composable depends on. */
export type Fetcher = (url: string, options?: FetchOptions) => Promise<unknown>;

/** A raw HTTP client (ofetch's `$fetch`) — takes an absolute-or-relative URL. */
export type RawFetch = (url: string, options?: FetchOptions) => Promise<unknown>;

/**
 * Binds a raw HTTP client to an API base URL, producing the `Fetcher` the domain
 * composables (fetchIssues, createIssue, …) consume. Reads call the raw client
 * with a single URL arg; mutations forward `{ method, body }` unchanged. The
 * base URL defaults to empty (same-origin), which is what the Nuxt dev proxy
 * relies on — `/api/**` is forwarded to the Rust backend by nitro.devProxy.
 *
 * `defaultHeaders` (e.g. `{ "X-Actor-User": userId }`) are merged into every
 * request — this is how the board-user-scoped endpoints (inbox, sidebar,
 * resource-memberships) carry the operator identity. When omitted, reads keep
 * their bare single-arg form.
 */
export function makeApiFetcher(
  raw: RawFetch,
  baseUrl = "",
  defaultHeaders?: Record<string, string>,
): Fetcher {
  const hasHeaders = defaultHeaders !== undefined && Object.keys(defaultHeaders).length > 0;
  return (url, options) => {
    const fullUrl = `${baseUrl}${url}`;
    if (!hasHeaders) {
      return options === undefined ? raw(fullUrl) : raw(fullUrl, options);
    }
    const merged: FetchOptions = {
      ...(options ?? {}),
      headers: { ...defaultHeaders, ...(options?.headers ?? {}) },
    };
    return raw(fullUrl, merged);
  };
}

// --- Nuxt glue (composition root; exercised at runtime, not in unit tests) ---
// These reference Nuxt auto-imported globals inside the function body only, so
// importing `makeApiFetcher` in vitest never touches them.
declare const $fetch: RawFetch;
declare function useRuntimeConfig(): { public: { apiBase?: string } };

/**
 * Returns the app-wide fetcher bound to the real `$fetch` and configured API
 * base (see `runtimeConfig.public.apiBase`). Components/pages call this, then
 * hand the result to the tested domain composables.
 */
export function useApi(): Fetcher {
  const base = useRuntimeConfig().public.apiBase ?? "";
  return makeApiFetcher($fetch, base);
}
