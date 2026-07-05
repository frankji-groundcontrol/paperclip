import { describe, it, expect, vi } from "vitest";
import { fetchSidebarBadges } from "../composables/useSidebarBadges";

// Ports the frontend data layer for a company's sidebar badge counts
// (backend-rs/src/sidebar_badges.rs): a company-scoped read of pending counts.
describe("fetchSidebarBadges", () => {
  it("requests the company-scoped sidebar-badges path and returns the counts", async () => {
    const badges = { failedRuns: 1, approvals: 2, joinRequests: 1, inbox: 4 };
    const fetcher = vi.fn().mockResolvedValue(badges);

    const result = await fetchSidebarBadges(fetcher, "co-1");

    expect(fetcher).toHaveBeenCalledWith("/api/companies/co-1/sidebar-badges");
    expect(result).toEqual(badges);
  });
});
