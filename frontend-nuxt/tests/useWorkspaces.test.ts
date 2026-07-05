import { describe, it, expect, vi } from "vitest";
import { fetchWorkspaces, createWorkspace, updateWorkspace } from "../composables/useWorkspaces";

// Ports the frontend data layer for a company's execution workspaces.
describe("fetchWorkspaces", () => {
  it("requests the company-scoped workspaces path and returns the list", async () => {
    const fetcher = vi
      .fn()
      .mockResolvedValue([{ id: "1", issueId: "i-1", status: "starting" }]);

    const workspaces = await fetchWorkspaces(fetcher, "co-1");

    expect(fetcher).toHaveBeenCalledWith("/api/companies/co-1/workspaces");
    expect(workspaces).toEqual([{ id: "1", issueId: "i-1", status: "starting" }]);
  });
});

describe("createWorkspace", () => {
  it("POSTs the payload to the company-scoped path and returns the created workspace", async () => {
    const created = { id: "9", issueId: "i-1", status: "starting" };
    const fetcher = vi.fn().mockResolvedValue(created);

    const workspace = await createWorkspace(fetcher, "co-1", { issueId: "i-1" });

    expect(fetcher).toHaveBeenCalledWith("/api/companies/co-1/workspaces", {
      method: "POST",
      body: { issueId: "i-1" },
    });
    expect(workspace).toEqual(created);
  });
});

describe("updateWorkspace", () => {
  it("PATCHes a workspace's lifecycle status on the company-scoped path", async () => {
    const updated = { id: "9", issueId: "i-1", status: "running" };
    const fetcher = vi.fn().mockResolvedValue(updated);

    const workspace = await updateWorkspace(fetcher, "co-1", "9", { status: "running" });

    expect(fetcher).toHaveBeenCalledWith("/api/companies/co-1/workspaces/9", {
      method: "PATCH",
      body: { status: "running" },
    });
    expect(workspace).toEqual(updated);
  });
});
