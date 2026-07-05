import { describe, it, expect, vi } from "vitest";
import { fetchProjectOrder, saveProjectOrder } from "../composables/useSidebar";

// Ports the frontend data layer for a board user's sidebar project order
// (backend-rs/src/sidebar.rs): read + upsert (PUT).
describe("fetchProjectOrder", () => {
  it("requests the company-scoped sidebar-preferences path and returns the preference", async () => {
    const pref = { orderedIds: ["p-1", "p-2"], updatedAt: null };
    const fetcher = vi.fn().mockResolvedValue(pref);

    const result = await fetchProjectOrder(fetcher, "co-1");

    expect(fetcher).toHaveBeenCalledWith("/api/companies/co-1/sidebar-preferences/me");
    expect(result).toEqual(pref);
  });
});

describe("saveProjectOrder", () => {
  it("PUTs the ordered ids to the company-scoped path and returns the preference", async () => {
    const pref = { orderedIds: ["p-2", "p-1"], updatedAt: null };
    const fetcher = vi.fn().mockResolvedValue(pref);

    const result = await saveProjectOrder(fetcher, "co-1", ["p-2", "p-1"]);

    expect(fetcher).toHaveBeenCalledWith("/api/companies/co-1/sidebar-preferences/me", {
      method: "PUT",
      body: { orderedIds: ["p-2", "p-1"] },
    });
    expect(result).toEqual(pref);
  });
});
