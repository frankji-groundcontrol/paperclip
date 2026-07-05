import { describe, it, expect, vi } from "vitest";
import { fetchInboxDismissals, dismissInboxItem } from "../composables/useInbox";

// Ports the frontend data layer for a board user's inbox dismissals
// (backend-rs/src/inbox.rs): list + dismiss.
describe("fetchInboxDismissals", () => {
  it("requests the company-scoped inbox-dismissals path and returns the list", async () => {
    const fetcher = vi
      .fn()
      .mockResolvedValue([{ itemKey: "approval:apr-1", dismissedAt: "123" }]);

    const dismissals = await fetchInboxDismissals(fetcher, "co-1");

    expect(fetcher).toHaveBeenCalledWith("/api/companies/co-1/inbox-dismissals");
    expect(dismissals).toEqual([{ itemKey: "approval:apr-1", dismissedAt: "123" }]);
  });
});

describe("dismissInboxItem", () => {
  it("POSTs the itemKey to the company-scoped path and returns the dismissal", async () => {
    const created = { itemKey: "run:r-1", dismissedAt: "456" };
    const fetcher = vi.fn().mockResolvedValue(created);

    const dismissal = await dismissInboxItem(fetcher, "co-1", "run:r-1");

    expect(fetcher).toHaveBeenCalledWith("/api/companies/co-1/inbox-dismissals", {
      method: "POST",
      body: { itemKey: "run:r-1" },
    });
    expect(dismissal).toEqual(created);
  });
});
