import { describe, it, expect, vi } from "vitest";
import {
  fetchMemberships,
  setProjectMembership,
  setAgentMembership,
} from "../composables/useMemberships";

// Ports the frontend data layer for a board user's resource memberships
// (backend-rs/src/memberships.rs): read + per-resource join/leave (PUT).
describe("fetchMemberships", () => {
  it("requests the company-scoped resource-memberships path and returns the maps", async () => {
    const memberships = {
      projectMemberships: { "p-1": "left" },
      agentMemberships: {},
      updatedAt: null,
    };
    const fetcher = vi.fn().mockResolvedValue(memberships);

    const result = await fetchMemberships(fetcher, "co-1");

    expect(fetcher).toHaveBeenCalledWith("/api/companies/co-1/resource-memberships/me");
    expect(result).toEqual(memberships);
  });
});

describe("setProjectMembership", () => {
  it("PUTs the state to the company-scoped project membership path", async () => {
    const result = { resourceType: "project", resourceId: "p-1", state: "left" };
    const fetcher = vi.fn().mockResolvedValue(result);

    const out = await setProjectMembership(fetcher, "co-1", "p-1", "left");

    expect(fetcher).toHaveBeenCalledWith(
      "/api/companies/co-1/resource-memberships/me/projects/p-1",
      { method: "PUT", body: { state: "left" } },
    );
    expect(out).toEqual(result);
  });
});

describe("setAgentMembership", () => {
  it("PUTs the state to the company-scoped agent membership path", async () => {
    const result = { resourceType: "agent", resourceId: "a-1", state: "joined" };
    const fetcher = vi.fn().mockResolvedValue(result);

    const out = await setAgentMembership(fetcher, "co-1", "a-1", "joined");

    expect(fetcher).toHaveBeenCalledWith(
      "/api/companies/co-1/resource-memberships/me/agents/a-1",
      { method: "PUT", body: { state: "joined" } },
    );
    expect(out).toEqual(result);
  });
});
