import { describe, it, expect, vi } from "vitest";
import { fetchPlugins, updatePlugin, deletePlugin } from "../composables/usePlugins";

// Ports the frontend data layer for a company's installed plugins.
describe("fetchPlugins", () => {
  it("requests the company-scoped plugins path and returns the list", async () => {
    const fetcher = vi
      .fn()
      .mockResolvedValue([{ id: "1", pluginId: "llm-wiki", enabled: true }]);

    const plugins = await fetchPlugins(fetcher, "co-1");

    expect(fetcher).toHaveBeenCalledWith("/api/companies/co-1/plugins");
    expect(plugins).toEqual([{ id: "1", pluginId: "llm-wiki", enabled: true }]);
  });
});

describe("updatePlugin", () => {
  it("PATCHes the enabled toggle on the company-scoped plugin path", async () => {
    const updated = { id: "9", pluginId: "llm-wiki", enabled: false };
    const fetcher = vi.fn().mockResolvedValue(updated);

    const plugin = await updatePlugin(fetcher, "co-1", "9", { enabled: false });

    expect(fetcher).toHaveBeenCalledWith("/api/companies/co-1/plugins/9", {
      method: "PATCH",
      body: { enabled: false },
    });
    expect(plugin).toEqual(updated);
  });
});

describe("deletePlugin", () => {
  it("DELETEs the company-scoped plugin path", async () => {
    const fetcher = vi.fn().mockResolvedValue(undefined);

    await deletePlugin(fetcher, "co-1", "9");

    expect(fetcher).toHaveBeenCalledWith("/api/companies/co-1/plugins/9", {
      method: "DELETE",
    });
  });
});
