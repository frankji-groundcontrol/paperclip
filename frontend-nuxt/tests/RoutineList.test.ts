import { describe, it, expect } from "vitest";
import { mount } from "@vue/test-utils";
import RoutineList from "../components/RoutineList.vue";

// Ports the company-scoped routine list (GET /api/companies/:companyId/routines).
describe("RoutineList", () => {
  it("renders each routine's title with its status", () => {
    const wrapper = mount(RoutineList, {
      props: {
        routines: [
          { id: "1", title: "Daily standup", status: "active" },
          { id: "2", title: "Weekly report", status: "paused" },
        ],
      },
    });

    const items = wrapper.findAll(".routine");
    expect(items).toHaveLength(2);
    expect(wrapper.text()).toContain("Daily standup");
    expect(items[1].attributes("data-status")).toBe("paused");
  });

  it("shows an empty state when there are no routines", () => {
    const wrapper = mount(RoutineList, { props: { routines: [] } });

    expect(wrapper.find(".routine").exists()).toBe(false);
    expect(wrapper.get(".empty").text()).toContain("No routines yet");
  });
});
