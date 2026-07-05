import { describe, it, expect } from "vitest";
import { mount } from "@vue/test-utils";
import GoalList from "../components/GoalList.vue";

// Ports the company-scoped goal list (GET /api/companies/:companyId/goals).
describe("GoalList", () => {
  it("renders each goal title with its level and status", () => {
    const wrapper = mount(GoalList, {
      props: {
        goals: [
          { id: "1", title: "Grow revenue", level: "company", status: "active" },
          { id: "2", title: "Ship v2", level: "task", status: "planned" },
        ],
      },
    });

    const items = wrapper.findAll(".goal");
    expect(items).toHaveLength(2);
    expect(wrapper.text()).toContain("Grow revenue");
    expect(items[0].attributes("data-level")).toBe("company");
    expect(items[1].attributes("data-status")).toBe("planned");
  });

  it("shows an empty state when there are no goals", () => {
    const wrapper = mount(GoalList, { props: { goals: [] } });

    expect(wrapper.find(".goal").exists()).toBe(false);
    expect(wrapper.get(".empty").text()).toContain("No goals yet");
  });
});
