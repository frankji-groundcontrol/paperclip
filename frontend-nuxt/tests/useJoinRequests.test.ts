import { describe, it, expect, vi } from "vitest";
import {
  fetchJoinRequests,
  createJoinRequest,
  approveJoinRequest,
  rejectJoinRequest,
} from "../composables/useJoinRequests";

// Ports the frontend data layer for a company's join-requests
// (backend-rs/src/join_requests.rs): list (w/ filters) + create + approve/reject.
describe("fetchJoinRequests", () => {
  it("requests the company-scoped join-requests path and returns the list", async () => {
    const fetcher = vi
      .fn()
      .mockResolvedValue([{ id: "1", requestType: "human", status: "pending_approval" }]);

    const requests = await fetchJoinRequests(fetcher, "co-1");

    expect(fetcher).toHaveBeenCalledWith("/api/companies/co-1/join-requests");
    expect(requests).toEqual([{ id: "1", requestType: "human", status: "pending_approval" }]);
  });

  it("appends status/requestType filters when provided", async () => {
    const fetcher = vi.fn().mockResolvedValue([]);

    await fetchJoinRequests(fetcher, "co-1", { requestType: "agent", status: "pending_approval" });

    expect(fetcher).toHaveBeenCalledWith(
      "/api/companies/co-1/join-requests?status=pending_approval&requestType=agent",
    );
  });
});

describe("createJoinRequest", () => {
  it("POSTs the payload to the company-scoped path", async () => {
    const created = { id: "9", requestType: "human", status: "pending_approval" };
    const fetcher = vi.fn().mockResolvedValue(created);

    const jr = await createJoinRequest(fetcher, "co-1", { requestType: "human", requesterName: "Alice" });

    expect(fetcher).toHaveBeenCalledWith("/api/companies/co-1/join-requests", {
      method: "POST",
      body: { requestType: "human", requesterName: "Alice" },
    });
    expect(jr).toEqual(created);
  });
});

describe("approveJoinRequest", () => {
  it("POSTs to the approve path", async () => {
    const approved = { id: "9", requestType: "human", status: "approved" };
    const fetcher = vi.fn().mockResolvedValue(approved);

    const jr = await approveJoinRequest(fetcher, "co-1", "9");

    expect(fetcher).toHaveBeenCalledWith("/api/companies/co-1/join-requests/9/approve", {
      method: "POST",
    });
    expect(jr).toEqual(approved);
  });
});

describe("rejectJoinRequest", () => {
  it("POSTs to the reject path", async () => {
    const rejected = { id: "9", requestType: "human", status: "rejected" };
    const fetcher = vi.fn().mockResolvedValue(rejected);

    const jr = await rejectJoinRequest(fetcher, "co-1", "9");

    expect(fetcher).toHaveBeenCalledWith("/api/companies/co-1/join-requests/9/reject", {
      method: "POST",
    });
    expect(jr).toEqual(rejected);
  });
});
