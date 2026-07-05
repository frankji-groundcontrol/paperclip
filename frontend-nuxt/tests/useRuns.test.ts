import { describe, it, expect, vi } from "vitest";
import { fetchRuns, createRun, updateRun } from "../composables/useRuns";

// Ports the frontend data layer for a company's runs, preserving company scoping
// in the request path.
describe("fetchRuns", () => {
  it("requests the company-scoped runs path and returns the list", async () => {
    const fetcher = vi
      .fn()
      .mockResolvedValue([{ id: "1", agentId: "a-1", status: "queued" }]);

    const runs = await fetchRuns(fetcher, "co-1");

    expect(fetcher).toHaveBeenCalledWith("/api/companies/co-1/runs");
    expect(runs).toEqual([{ id: "1", agentId: "a-1", status: "queued" }]);
  });
});

describe("createRun", () => {
  it("POSTs the payload to the company-scoped path and returns the created run", async () => {
    const created = { id: "9", agentId: "a-1", status: "queued" };
    const fetcher = vi.fn().mockResolvedValue(created);

    const run = await createRun(fetcher, "co-1", { agentId: "a-1" });

    expect(fetcher).toHaveBeenCalledWith("/api/companies/co-1/runs", {
      method: "POST",
      body: { agentId: "a-1" },
    });
    expect(run).toEqual(created);
  });
});

describe("updateRun", () => {
  it("PATCHes a run's status transition on the company-scoped path", async () => {
    const updated = { id: "9", agentId: "a-1", status: "running" };
    const fetcher = vi.fn().mockResolvedValue(updated);

    const run = await updateRun(fetcher, "co-1", "9", { status: "running" });

    expect(fetcher).toHaveBeenCalledWith("/api/companies/co-1/runs/9", {
      method: "PATCH",
      body: { status: "running" },
    });
    expect(run).toEqual(updated);
  });
});
