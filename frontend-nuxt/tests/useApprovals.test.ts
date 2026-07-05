import { describe, it, expect, vi } from "vitest";
import { fetchApprovals, createApproval, updateApproval } from "../composables/useApprovals";

// Ports the frontend data layer for a company's approvals, preserving company
// scoping in the request path.
describe("fetchApprovals", () => {
  it("requests the company-scoped approvals path and returns the list", async () => {
    const fetcher = vi
      .fn()
      .mockResolvedValue([{ id: "1", issueId: "i-1", status: "pending" }]);

    const approvals = await fetchApprovals(fetcher, "co-1");

    expect(fetcher).toHaveBeenCalledWith("/api/companies/co-1/approvals");
    expect(approvals).toEqual([{ id: "1", issueId: "i-1", status: "pending" }]);
  });
});

describe("createApproval", () => {
  it("POSTs the payload to the company-scoped path and returns the created approval", async () => {
    const created = { id: "9", issueId: "i-1", status: "pending" };
    const fetcher = vi.fn().mockResolvedValue(created);

    const approval = await createApproval(fetcher, "co-1", { issueId: "i-1" });

    expect(fetcher).toHaveBeenCalledWith("/api/companies/co-1/approvals", {
      method: "POST",
      body: { issueId: "i-1" },
    });
    expect(approval).toEqual(created);
  });
});

describe("updateApproval", () => {
  it("PATCHes an approval's decision on the company-scoped path", async () => {
    const updated = { id: "9", issueId: "i-1", status: "approved" };
    const fetcher = vi.fn().mockResolvedValue(updated);

    const approval = await updateApproval(fetcher, "co-1", "9", { status: "approved" });

    expect(fetcher).toHaveBeenCalledWith("/api/companies/co-1/approvals/9", {
      method: "PATCH",
      body: { status: "approved" },
    });
    expect(approval).toEqual(updated);
  });
});
