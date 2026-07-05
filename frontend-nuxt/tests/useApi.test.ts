import { describe, it, expect, vi } from "vitest";
import { makeApiFetcher } from "../composables/useApi";

// The app's composables (fetchIssues, createIssue, …) take a `Fetcher`. In the
// running Nuxt app that fetcher is ofetch's `$fetch` bound to the API base URL.
// makeApiFetcher builds that binding and is the seam we can unit-test without a
// live server.
describe("makeApiFetcher", () => {
  it("prefixes the base URL for reads and returns the parsed body", async () => {
    const raw = vi.fn().mockResolvedValue([{ id: "1", name: "Acme" }]);
    const fetcher = makeApiFetcher(raw, "http://localhost:3100");

    const result = await fetcher("/api/companies");

    expect(raw).toHaveBeenCalledWith("http://localhost:3100/api/companies");
    expect(result).toEqual([{ id: "1", name: "Acme" }]);
  });

  it("forwards method and body for mutations, still prefixing the base URL", async () => {
    const raw = vi.fn().mockResolvedValue({ id: "9", name: "Globex" });
    const fetcher = makeApiFetcher(raw, "http://localhost:3100");

    await fetcher("/api/companies", { method: "POST", body: { name: "Globex" } });

    expect(raw).toHaveBeenCalledWith("http://localhost:3100/api/companies", {
      method: "POST",
      body: { name: "Globex" },
    });
  });

  it("defaults to a same-origin base (empty prefix) so relative paths pass through", async () => {
    const raw = vi.fn().mockResolvedValue([]);
    const fetcher = makeApiFetcher(raw);

    await fetcher("/api/companies/co-1/issues");

    expect(raw).toHaveBeenCalledWith("/api/companies/co-1/issues");
  });

  it("injects default headers on reads when provided (e.g. board X-Actor-User)", async () => {
    const raw = vi.fn().mockResolvedValue([]);
    const fetcher = makeApiFetcher(raw, "", { "X-Actor-User": "u-1" });

    await fetcher("/api/companies/co-1/inbox-dismissals");

    expect(raw).toHaveBeenCalledWith("/api/companies/co-1/inbox-dismissals", {
      headers: { "X-Actor-User": "u-1" },
    });
  });

  it("merges default headers with mutation options", async () => {
    const raw = vi.fn().mockResolvedValue({});
    const fetcher = makeApiFetcher(raw, "", { "X-Actor-User": "u-1" });

    await fetcher("/api/companies/co-1/inbox-dismissals", {
      method: "POST",
      body: { itemKey: "run:r-1" },
    });

    expect(raw).toHaveBeenCalledWith("/api/companies/co-1/inbox-dismissals", {
      method: "POST",
      body: { itemKey: "run:r-1" },
      headers: { "X-Actor-User": "u-1" },
    });
  });
});
