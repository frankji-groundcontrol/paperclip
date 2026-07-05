import { describe, it, expect } from "vitest";
import { mount } from "@vue/test-utils";
import AgentList from "../components/AgentList.vue";

// Ports the company-scoped agent list (GET /api/companies/:companyId/agents).
describe("AgentList", () => {
  it("renders each agent name with its status", () => {
    const wrapper = mount(AgentList, {
      props: {
        agents: [
          { id: "1", name: "Ada", role: "general", status: "idle" },
          { id: "2", name: "Grace", role: "ceo", status: "running" },
        ],
      },
    });

    const items = wrapper.findAll(".agent");
    expect(items).toHaveLength(2);
    expect(wrapper.text()).toContain("Ada");
    expect(items[1].attributes("data-status")).toBe("running");
  });

  it("shows an empty state when there are no agents", () => {
    const wrapper = mount(AgentList, { props: { agents: [] } });

    expect(wrapper.find(".agent").exists()).toBe(false);
    expect(wrapper.get(".empty").text()).toContain("No agents yet");
  });
});
