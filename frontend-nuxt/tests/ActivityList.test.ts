import { describe, it, expect } from "vitest";
import { mount } from "@vue/test-utils";
import ActivityList from "../components/ActivityList.vue";

// Ports the company activity feed (GET /api/companies/:companyId/activity).
describe("ActivityList", () => {
  it("renders each event's action and entity type", () => {
    const wrapper = mount(ActivityList, {
      props: {
        events: [
          { id: "1", action: "issue.created", entityType: "issue" },
          { id: "2", action: "goal.updated", entityType: "goal" },
        ],
      },
    });

    const items = wrapper.findAll(".activity");
    expect(items).toHaveLength(2);
    expect(wrapper.text()).toContain("issue.created");
    expect(items[1].attributes("data-entity")).toBe("goal");
  });

  it("shows an empty state when there is no activity", () => {
    const wrapper = mount(ActivityList, { props: { events: [] } });
    expect(wrapper.find(".activity").exists()).toBe(false);
    expect(wrapper.get(".empty").text()).toContain("No activity yet");
  });
});
