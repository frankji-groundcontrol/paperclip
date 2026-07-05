import { describe, it, expect, vi } from "vitest";
import {
  fetchCompanies,
  createCompany,
  updateCompany,
  deleteCompany,
} from "../composables/useCompanies";

// Ports the frontend data layer for GET /api/companies. The HTTP boundary is
// stubbed (the only mock); the composable's own behaviour is exercised for real.
describe("fetchCompanies", () => {
  it("requests /api/companies and returns the parsed list", async () => {
    const fetcher = vi.fn().mockResolvedValue([{ id: "1", name: "Acme" }]);

    const companies = await fetchCompanies(fetcher);

    expect(fetcher).toHaveBeenCalledWith("/api/companies");
    expect(companies).toEqual([{ id: "1", name: "Acme" }]);
  });
});

describe("createCompany", () => {
  it("POSTs the payload to /api/companies and returns the created company", async () => {
    const created = { id: "9", name: "Globex" };
    const fetcher = vi.fn().mockResolvedValue(created);

    const company = await createCompany(fetcher, { name: "Globex" });

    expect(fetcher).toHaveBeenCalledWith("/api/companies", {
      method: "POST",
      body: { name: "Globex" },
    });
    expect(company).toEqual(created);
  });
});

describe("updateCompany", () => {
  it("PATCHes the company on its detail path", async () => {
    const updated = { id: "9", name: "Globex Corp" };
    const fetcher = vi.fn().mockResolvedValue(updated);

    const company = await updateCompany(fetcher, "9", { name: "Globex Corp" });

    expect(fetcher).toHaveBeenCalledWith("/api/companies/9", {
      method: "PATCH",
      body: { name: "Globex Corp" },
    });
    expect(company).toEqual(updated);
  });
});

describe("deleteCompany", () => {
  it("DELETEs the company detail path", async () => {
    const fetcher = vi.fn().mockResolvedValue(undefined);

    await deleteCompany(fetcher, "9");

    expect(fetcher).toHaveBeenCalledWith("/api/companies/9", {
      method: "DELETE",
    });
  });
});
