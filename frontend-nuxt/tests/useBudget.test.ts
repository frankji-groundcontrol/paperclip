import { describe, it, expect, vi } from "vitest";
import { fetchBudget, setBudgetLimit, recordBudgetSpend } from "../composables/useBudget";

// Ports the frontend data layer for a company's budget.
describe("fetchBudget", () => {
  it("requests the company-scoped budget path and returns it", async () => {
    const budget = {
      companyId: "co-1",
      monthlyLimitCents: 1000,
      spentCents: 400,
      exceeded: false,
    };
    const fetcher = vi.fn().mockResolvedValue(budget);

    const result = await fetchBudget(fetcher, "co-1");

    expect(fetcher).toHaveBeenCalledWith("/api/companies/co-1/budget");
    expect(result).toEqual(budget);
  });
});

describe("setBudgetLimit", () => {
  it("POSTs the monthly limit to the budget/limit path", async () => {
    const budget = {
      companyId: "co-1",
      monthlyLimitCents: 5000,
      spentCents: 0,
      exceeded: false,
    };
    const fetcher = vi.fn().mockResolvedValue(budget);

    const result = await setBudgetLimit(fetcher, "co-1", 5000);

    expect(fetcher).toHaveBeenCalledWith("/api/companies/co-1/budget/limit", {
      method: "POST",
      body: { monthlyLimitCents: 5000 },
    });
    expect(result).toEqual(budget);
  });
});

describe("recordBudgetSpend", () => {
  it("POSTs the amount to the budget/spend path and reflects the hard-stop", async () => {
    const budget = {
      companyId: "co-1",
      monthlyLimitCents: 5000,
      spentCents: 5000,
      exceeded: true,
    };
    const fetcher = vi.fn().mockResolvedValue(budget);

    const result = await recordBudgetSpend(fetcher, "co-1", 5000);

    expect(fetcher).toHaveBeenCalledWith("/api/companies/co-1/budget/spend", {
      method: "POST",
      body: { amountCents: 5000 },
    });
    expect(result).toEqual(budget);
  });
});
