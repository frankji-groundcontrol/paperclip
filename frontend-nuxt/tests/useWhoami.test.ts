import { describe, it, expect, vi } from "vitest";
import { fetchWhoami } from "../composables/useWhoami";

// Ports the frontend data layer for GET /api/whoami (actor resolution).
describe("fetchWhoami", () => {
  it("requests /api/whoami and returns the actor", async () => {
    const fetcher = vi.fn().mockResolvedValue({ actor: "agent", companyId: "co-1" });

    const who = await fetchWhoami(fetcher);

    expect(fetcher).toHaveBeenCalledWith("/api/whoami");
    expect(who).toEqual({ actor: "agent", companyId: "co-1" });
  });
});
