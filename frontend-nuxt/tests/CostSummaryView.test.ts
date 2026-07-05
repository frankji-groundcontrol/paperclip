import { describe, it, expect } from "vitest";
import { mount } from "@vue/test-utils";
import CostSummaryView from "../components/CostSummaryView.vue";

// Ports the company cost summary (GET /api/companies/:companyId/costs/summary).
describe("CostSummaryView", () => {
  it("renders spend, budget (as dollars), and utilization", () => {
    const wrapper = mount(CostSummaryView, {
      props: {
        summary: { companyId: "co-1", spendCents: 500, budgetCents: 1000, utilizationPercent: 50 },
      },
    });

    expect(wrapper.get('[data-field="spend"]').text()).toContain("$5.00");
    expect(wrapper.get('[data-field="budget"]').text()).toContain("$10.00");
    expect(wrapper.get('[data-field="utilization"]').text()).toContain("50%");
  });

  it("marks over-budget when utilization is at/over 100%", () => {
    const wrapper = mount(CostSummaryView, {
      props: {
        summary: { companyId: "co-1", spendCents: 1200, budgetCents: 1000, utilizationPercent: 120 },
      },
    });
    expect(wrapper.get(".cost-summary").attributes("data-over-budget")).toBe("true");
  });
});
