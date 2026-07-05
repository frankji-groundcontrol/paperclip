import { describe, it, expect } from "vitest";
import { mount } from "@vue/test-utils";
import RunList from "../components/RunList.vue";

// Ports the company-scoped run list (GET /api/companies/:companyId/runs).
describe("RunList", () => {
  it("renders each run's agent with its status", () => {
    const wrapper = mount(RunList, {
      props: {
        runs: [
          { id: "1", agentId: "a-1", status: "queued" },
          { id: "2", agentId: "a-2", status: "running" },
        ],
      },
    });

    const items = wrapper.findAll(".run");
    expect(items).toHaveLength(2);
    expect(wrapper.text()).toContain("a-1");
    expect(items[1].attributes("data-status")).toBe("running");
  });

  it("shows an empty state when there are no runs", () => {
    const wrapper = mount(RunList, { props: { runs: [] } });

    expect(wrapper.find(".run").exists()).toBe(false);
    expect(wrapper.get(".empty").text()).toContain("No runs yet");
  });
});
