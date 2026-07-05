import { describe, it, expect, vi } from "vitest";
import { fetchDashboard } from "../composables/useDashboard";

// Ports the frontend data layer for a company's dashboard rollup
// (backend-rs/src/dashboard.rs): a company-scoped read of cross-domain counts.
describe("fetchDashboard", () => {
  it("requests the company-scoped dashboard path and returns the summary", async () => {
    const summary = {
      agents: { active: 1, running: 1, paused: 0, error: 0 },
      tasks: { open: 2, inProgress: 1, blocked: 0, done: 1 },
      approvals: { pending: 1 },
    };
    const fetcher = vi.fn().mockResolvedValue(summary);

    const result = await fetchDashboard(fetcher, "co-1");

    expect(fetcher).toHaveBeenCalledWith("/api/companies/co-1/dashboard");
    expect(result).toEqual(summary);
  });
});
