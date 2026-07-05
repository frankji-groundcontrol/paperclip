import { describe, it, expect } from "vitest";
import { mount } from "@vue/test-utils";
import PipelineList from "../components/PipelineList.vue";

// Ports the company-scoped pipeline list (GET /api/companies/:companyId/pipelines).
describe("PipelineList", () => {
  it("renders each pipeline name and marks archived ones", () => {
    const wrapper = mount(PipelineList, {
      props: {
        pipelines: [
          { id: "1", key: "intake", name: "Intake", archived: false },
          { id: "2", key: "old", name: "Old Flow", archived: true },
        ],
      },
    });

    const items = wrapper.findAll(".pipeline");
    expect(items).toHaveLength(2);
    expect(wrapper.text()).toContain("Intake");
    expect(items[0].attributes("data-archived")).toBe("false");
    expect(items[1].attributes("data-archived")).toBe("true");
  });

  it("shows an empty state when there are no pipelines", () => {
    const wrapper = mount(PipelineList, { props: { pipelines: [] } });

    expect(wrapper.find(".pipeline").exists()).toBe(false);
    expect(wrapper.get(".empty").text()).toContain("No pipelines yet");
  });
});
