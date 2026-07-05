import { describe, it, expect } from "vitest";
import { mount } from "@vue/test-utils";
import BudgetView from "../components/BudgetView.vue";

// Mirrors the budget hard-stop (GET /api/companies/:companyId/budget).
describe("BudgetView", () => {
  it("shows spend against the limit when within budget", () => {
    const wrapper = mount(BudgetView, {
      props: { monthlyLimitCents: 1000, spentCents: 400, exceeded: false },
    });

    expect(wrapper.get(".budget").attributes("data-exceeded")).toBe("false");
    expect(wrapper.text()).toContain("400");
    expect(wrapper.text()).toContain("1000");
    expect(wrapper.text()).toContain("Within budget");
  });

  it("flags over budget when the hard-stop is hit", () => {
    const wrapper = mount(BudgetView, {
      props: { monthlyLimitCents: 1000, spentCents: 1100, exceeded: true },
    });

    expect(wrapper.get(".budget").attributes("data-exceeded")).toBe("true");
    expect(wrapper.text()).toContain("Over budget");
  });
});
