import { describe, it, expect, vi } from "vitest";
import {
  fetchCatalog,
  fetchInstalledTeams,
  installTeam,
} from "../composables/useTeamsCatalog";

// Ports the frontend data layer for the teams catalog
// (backend-rs/src/teams_catalog.rs): list/filter catalog + install + list installed.
describe("fetchCatalog", () => {
  it("requests the catalog and returns the teams", async () => {
    const fetcher = vi.fn().mockResolvedValue([{ id: "cat-engineering", key: "engineering", name: "Engineering Team" }]);

    const teams = await fetchCatalog(fetcher);

    expect(fetcher).toHaveBeenCalledWith("/api/teams/catalog");
    expect(teams).toEqual([{ id: "cat-engineering", key: "engineering", name: "Engineering Team" }]);
  });

  it("appends category/q filters when provided", async () => {
    const fetcher = vi.fn().mockResolvedValue([]);

    await fetchCatalog(fetcher, { category: "engineering", q: "build" });

    expect(fetcher).toHaveBeenCalledWith("/api/teams/catalog?category=engineering&q=build");
  });
});

describe("fetchInstalledTeams", () => {
  it("requests the company-scoped installed path", async () => {
    const fetcher = vi.fn().mockResolvedValue([{ catalogId: "cat-engineering", name: "Engineering Team" }]);

    const installed = await fetchInstalledTeams(fetcher, "co-1");

    expect(fetcher).toHaveBeenCalledWith("/api/companies/co-1/teams/catalog/installed");
    expect(installed).toEqual([{ catalogId: "cat-engineering", name: "Engineering Team" }]);
  });
});

describe("installTeam", () => {
  it("POSTs to the install path and returns the installed team", async () => {
    const installed = { catalogId: "cat-engineering", name: "Engineering Team" };
    const fetcher = vi.fn().mockResolvedValue(installed);

    const out = await installTeam(fetcher, "co-1", "cat-engineering");

    expect(fetcher).toHaveBeenCalledWith(
      "/api/companies/co-1/teams/catalog/cat-engineering/install",
      { method: "POST", body: {} },
    );
    expect(out).toEqual(installed);
  });
});
