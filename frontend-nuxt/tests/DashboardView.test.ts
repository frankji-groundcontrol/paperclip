import { describe, it, expect } from "vitest";
import { mount } from "@vue/test-utils";
import DashboardView from "../components/DashboardView.vue";

// Ports the company dashboard rollup (GET /api/companies/:companyId/dashboard).
describe("DashboardView", () => {
  it("renders the agent, task, and approval counts", () => {
    const wrapper = mount(DashboardView, {
      props: {
        dashboard: {
          agents: { active: 2, running: 1, paused: 0, error: 0 },
          tasks: { open: 5, inProgress: 2, blocked: 1, done: 3 },
          approvals: { pending: 4 },
        },
      },
    });

    expect(wrapper.get('[data-metric="agents.active"]').text()).toBe("2");
    expect(wrapper.get('[data-metric="tasks.open"]').text()).toBe("5");
    expect(wrapper.get('[data-metric="approvals.pending"]').text()).toBe("4");
  });
});
