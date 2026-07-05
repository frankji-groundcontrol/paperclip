import { describe, it, expect, vi } from "vitest";
import { reportCostEvent, fetchCostSummary } from "../composables/useCosts";

// Ports the frontend data layer for a company's costs (backend-rs/src/costs.rs):
// report a cost event + read the spend-vs-budget summary.
describe("reportCostEvent", () => {
  it("POSTs the event to the company-scoped cost-events path and returns it", async () => {
    const created = { id: "9", companyId: "co-1", agentId: "a-1", costCents: 150 };
    const fetcher = vi.fn().mockResolvedValue(created);

    const event = await reportCostEvent(fetcher, "co-1", {
      agentId: "a-1",
      provider: "anthropic",
      model: "claude",
      costCents: 150,
      occurredAt: "2026-07-01T00:00:00Z",
    });

    expect(fetcher).toHaveBeenCalledWith("/api/companies/co-1/cost-events", {
      method: "POST",
      body: {
        agentId: "a-1",
        provider: "anthropic",
        model: "claude",
        costCents: 150,
        occurredAt: "2026-07-01T00:00:00Z",
      },
    });
    expect(event).toEqual(created);
  });
});

describe("fetchCostSummary", () => {
  it("requests the company-scoped costs summary path and returns it", async () => {
    const summary = {
      companyId: "co-1",
      spendCents: 500,
      budgetCents: 1000,
      utilizationPercent: 50,
    };
    const fetcher = vi.fn().mockResolvedValue(summary);

    const result = await fetchCostSummary(fetcher, "co-1");

    expect(fetcher).toHaveBeenCalledWith("/api/companies/co-1/costs/summary");
    expect(result).toEqual(summary);
  });
});
