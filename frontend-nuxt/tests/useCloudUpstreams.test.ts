import { describe, it, expect, vi } from "vitest";
import {
  fetchCloudUpstreams,
  registerCloudUpstream,
  deleteCloudUpstream,
} from "../composables/useCloudUpstreams";

// Ports the frontend data layer for cloud upstreams
// (backend-rs/src/cloud_upstreams.rs): list/register/delete (gated by enableCloudSync).
describe("fetchCloudUpstreams", () => {
  it("requests the cloud-upstreams path and returns the connections", async () => {
    const fetcher = vi
      .fn()
      .mockResolvedValue([{ id: "1", companyId: "co-1", remoteUrl: "https://r", status: "connected" }]);

    const upstreams = await fetchCloudUpstreams(fetcher);

    expect(fetcher).toHaveBeenCalledWith("/api/cloud-upstreams");
    expect(upstreams).toEqual([
      { id: "1", companyId: "co-1", remoteUrl: "https://r", status: "connected" },
    ]);
  });

  it("appends the companyId filter when provided", async () => {
    const fetcher = vi.fn().mockResolvedValue([]);

    await fetchCloudUpstreams(fetcher, "co-1");

    expect(fetcher).toHaveBeenCalledWith("/api/cloud-upstreams?companyId=co-1");
  });
});

describe("registerCloudUpstream", () => {
  it("POSTs the payload to the register path", async () => {
    const created = { id: "9", companyId: "co-1", remoteUrl: "https://r", status: "connected" };
    const fetcher = vi.fn().mockResolvedValue(created);

    const conn = await registerCloudUpstream(fetcher, { companyId: "co-1", remoteUrl: "https://r" });

    expect(fetcher).toHaveBeenCalledWith("/api/cloud-upstreams/register", {
      method: "POST",
      body: { companyId: "co-1", remoteUrl: "https://r" },
    });
    expect(conn).toEqual(created);
  });
});

describe("deleteCloudUpstream", () => {
  it("DELETEs the connection by id", async () => {
    const fetcher = vi.fn().mockResolvedValue(undefined);

    await deleteCloudUpstream(fetcher, "9");

    expect(fetcher).toHaveBeenCalledWith("/api/cloud-upstreams/9", { method: "DELETE" });
  });
});
