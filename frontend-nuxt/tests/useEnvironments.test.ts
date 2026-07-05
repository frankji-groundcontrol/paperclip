import { describe, it, expect, vi } from "vitest";
import {
  fetchEnvironments,
  createEnvironment,
  updateEnvironment,
  deleteEnvironment,
} from "../composables/useEnvironments";

// Ports the frontend data layer for a company's execution environments
// (backend-rs/src/environments.rs): company-scoped list + create/update/delete.
describe("fetchEnvironments", () => {
  it("requests the company-scoped environments path and returns the list", async () => {
    const fetcher = vi
      .fn()
      .mockResolvedValue([{ id: "1", name: "prod", driver: "local", status: "active" }]);

    const envs = await fetchEnvironments(fetcher, "co-1");

    expect(fetcher).toHaveBeenCalledWith("/api/companies/co-1/environments");
    expect(envs).toEqual([{ id: "1", name: "prod", driver: "local", status: "active" }]);
  });
});

describe("createEnvironment", () => {
  it("POSTs the payload to the company-scoped path and returns the created environment", async () => {
    const created = { id: "9", name: "sandbox-1", driver: "sandbox", status: "active" };
    const fetcher = vi.fn().mockResolvedValue(created);

    const env = await createEnvironment(fetcher, "co-1", { name: "sandbox-1", driver: "sandbox" });

    expect(fetcher).toHaveBeenCalledWith("/api/companies/co-1/environments", {
      method: "POST",
      body: { name: "sandbox-1", driver: "sandbox" },
    });
    expect(env).toEqual(created);
  });
});

describe("updateEnvironment", () => {
  it("PATCHes an environment on the company-scoped path", async () => {
    const updated = { id: "9", name: "sandbox-1", driver: "sandbox", status: "archived" };
    const fetcher = vi.fn().mockResolvedValue(updated);

    const env = await updateEnvironment(fetcher, "co-1", "9", { status: "archived" });

    expect(fetcher).toHaveBeenCalledWith("/api/companies/co-1/environments/9", {
      method: "PATCH",
      body: { status: "archived" },
    });
    expect(env).toEqual(updated);
  });
});

describe("deleteEnvironment", () => {
  it("DELETEs the company-scoped environment path", async () => {
    const fetcher = vi.fn().mockResolvedValue(undefined);

    await deleteEnvironment(fetcher, "co-1", "9");

    expect(fetcher).toHaveBeenCalledWith("/api/companies/co-1/environments/9", {
      method: "DELETE",
    });
  });
});
