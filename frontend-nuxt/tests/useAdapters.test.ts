import { describe, it, expect, vi } from "vitest";
import { fetchAdapters, updateAdapter, deleteAdapter } from "../composables/useAdapters";

// Ports the frontend data layer for a company's configured adapters.
describe("fetchAdapters", () => {
  it("requests the company-scoped adapters path and returns the list", async () => {
    const fetcher = vi
      .fn()
      .mockResolvedValue([{ id: "1", adapterType: "claude_local", enabled: true }]);

    const adapters = await fetchAdapters(fetcher, "co-1");

    expect(fetcher).toHaveBeenCalledWith("/api/companies/co-1/adapters");
    expect(adapters).toEqual([{ id: "1", adapterType: "claude_local", enabled: true }]);
  });
});

describe("updateAdapter", () => {
  it("PATCHes the enabled toggle on the company-scoped adapter path", async () => {
    const updated = { id: "9", adapterType: "claude_local", enabled: false };
    const fetcher = vi.fn().mockResolvedValue(updated);

    const adapter = await updateAdapter(fetcher, "co-1", "9", { enabled: false });

    expect(fetcher).toHaveBeenCalledWith("/api/companies/co-1/adapters/9", {
      method: "PATCH",
      body: { enabled: false },
    });
    expect(adapter).toEqual(updated);
  });
});

describe("deleteAdapter", () => {
  it("DELETEs the company-scoped adapter path", async () => {
    const fetcher = vi.fn().mockResolvedValue(undefined);

    await deleteAdapter(fetcher, "co-1", "9");

    expect(fetcher).toHaveBeenCalledWith("/api/companies/co-1/adapters/9", {
      method: "DELETE",
    });
  });
});
