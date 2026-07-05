import { describe, it, expect, vi } from "vitest";
import { fetchActivity, createActivity } from "../composables/useActivity";

// Ports the frontend data layer for a company's activity feed
// (backend-rs/src/activity.rs): company-scoped list with optional filters, plus
// a board-only create.
describe("fetchActivity", () => {
  it("requests the company-scoped activity path with no filters", async () => {
    const fetcher = vi
      .fn()
      .mockResolvedValue([{ id: "1", action: "issue.created", entityType: "issue" }]);

    const events = await fetchActivity(fetcher, "co-1");

    expect(fetcher).toHaveBeenCalledWith("/api/companies/co-1/activity");
    expect(events).toEqual([{ id: "1", action: "issue.created", entityType: "issue" }]);
  });

  it("appends provided filters as query params in a stable order", async () => {
    const fetcher = vi.fn().mockResolvedValue([]);

    await fetchActivity(fetcher, "co-1", { entityType: "goal", limit: 10 });

    expect(fetcher).toHaveBeenCalledWith(
      "/api/companies/co-1/activity?entityType=goal&limit=10",
    );
  });
});

describe("createActivity", () => {
  it("POSTs the event to the company-scoped path and returns it", async () => {
    const created = {
      id: "9",
      companyId: "co-1",
      actorId: "u-1",
      action: "issue.created",
      entityType: "issue",
      entityId: "i-1",
      actorType: "system",
    };
    const fetcher = vi.fn().mockResolvedValue(created);

    const event = await createActivity(fetcher, "co-1", {
      actorId: "u-1",
      action: "issue.created",
      entityType: "issue",
      entityId: "i-1",
    });

    expect(fetcher).toHaveBeenCalledWith("/api/companies/co-1/activity", {
      method: "POST",
      body: { actorId: "u-1", action: "issue.created", entityType: "issue", entityId: "i-1" },
    });
    expect(event).toEqual(created);
  });
});
