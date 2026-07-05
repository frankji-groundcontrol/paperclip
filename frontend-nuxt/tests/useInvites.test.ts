import { describe, it, expect, vi } from "vitest";
import { fetchInvites, createInvite, revokeInvite } from "../composables/useInvites";

// Ports the frontend data layer for a company's invites
// (backend-rs/src/invites.rs): list (optionally by state) + create + revoke.
describe("fetchInvites", () => {
  it("requests the company-scoped invites path and returns the list", async () => {
    const fetcher = vi
      .fn()
      .mockResolvedValue([{ id: "1", token: "t", allowedJoinTypes: "both", state: "active" }]);

    const invites = await fetchInvites(fetcher, "co-1");

    expect(fetcher).toHaveBeenCalledWith("/api/companies/co-1/invites");
    expect(invites).toEqual([
      { id: "1", token: "t", allowedJoinTypes: "both", state: "active" },
    ]);
  });

  it("appends the state filter when provided", async () => {
    const fetcher = vi.fn().mockResolvedValue([]);

    await fetchInvites(fetcher, "co-1", "revoked");

    expect(fetcher).toHaveBeenCalledWith("/api/companies/co-1/invites?state=revoked");
  });
});

describe("createInvite", () => {
  it("POSTs the payload to the company-scoped path and returns the created invite", async () => {
    const created = { id: "9", token: "t", allowedJoinTypes: "human", state: "active" };
    const fetcher = vi.fn().mockResolvedValue(created);

    const invite = await createInvite(fetcher, "co-1", { allowedJoinTypes: "human" });

    expect(fetcher).toHaveBeenCalledWith("/api/companies/co-1/invites", {
      method: "POST",
      body: { allowedJoinTypes: "human" },
    });
    expect(invite).toEqual(created);
  });
});

describe("revokeInvite", () => {
  it("POSTs to the revoke path and returns the revoked invite", async () => {
    const revoked = { id: "9", token: "t", allowedJoinTypes: "both", state: "revoked" };
    const fetcher = vi.fn().mockResolvedValue(revoked);

    const invite = await revokeInvite(fetcher, "co-1", "9");

    expect(fetcher).toHaveBeenCalledWith("/api/companies/co-1/invites/9/revoke", {
      method: "POST",
    });
    expect(invite).toEqual(revoked);
  });
});
