import { describe, it, expect, vi } from "vitest";
import { fetchUserProfile } from "../composables/useUserProfile";

// Ports the frontend data layer for a user's profile rollup
// (backend-rs/src/user_profiles.rs): a per-user activity aggregation.
describe("fetchUserProfile", () => {
  it("requests the company/user-scoped profile path and returns the rollup", async () => {
    const profile = {
      userId: "u-1",
      companyId: "co-1",
      activityCount: 2,
      recentActivity: [],
      actionCounts: { "issue.created": 1, "goal.updated": 1 },
    };
    const fetcher = vi.fn().mockResolvedValue(profile);

    const result = await fetchUserProfile(fetcher, "co-1", "u-1");

    expect(fetcher).toHaveBeenCalledWith("/api/companies/co-1/users/u-1/profile");
    expect(result).toEqual(profile);
  });
});
