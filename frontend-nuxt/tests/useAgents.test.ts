import { describe, it, expect, vi } from "vitest";
import { fetchAgents, createAgent, deleteAgent } from "../composables/useAgents";

// Ports the frontend data layer for a company's agents, preserving company
// scoping in the request path.
describe("fetchAgents", () => {
  it("requests the company-scoped agents path and returns the list", async () => {
    const fetcher = vi.fn().mockResolvedValue([
      { id: "1", name: "Ada", role: "general", status: "idle", adapterType: "process" },
    ]);

    const agents = await fetchAgents(fetcher, "co-1");

    expect(fetcher).toHaveBeenCalledWith("/api/companies/co-1/agents");
    expect(agents).toEqual([
      { id: "1", name: "Ada", role: "general", status: "idle", adapterType: "process" },
    ]);
  });
});

describe("createAgent", () => {
  it("POSTs the payload to the company-scoped path and returns the created agent", async () => {
    const created = {
      id: "9",
      name: "Grace",
      role: "reviewer",
      status: "idle",
      adapterType: "process",
    };
    const fetcher = vi.fn().mockResolvedValue(created);

    const agent = await createAgent(fetcher, "co-1", { name: "Grace", role: "reviewer" });

    expect(fetcher).toHaveBeenCalledWith("/api/companies/co-1/agents", {
      method: "POST",
      body: { name: "Grace", role: "reviewer" },
    });
    expect(agent).toEqual(created);
  });
});

describe("deleteAgent", () => {
  it("DELETEs the company-scoped agent path", async () => {
    const fetcher = vi.fn().mockResolvedValue(undefined);

    await deleteAgent(fetcher, "co-1", "9");

    expect(fetcher).toHaveBeenCalledWith("/api/companies/co-1/agents/9", {
      method: "DELETE",
    });
  });
});
