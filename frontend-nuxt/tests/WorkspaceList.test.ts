import { describe, it, expect } from "vitest";
import { mount } from "@vue/test-utils";
import WorkspaceList from "../components/WorkspaceList.vue";

// Ports the company-scoped workspace list (GET /api/companies/:companyId/workspaces).
describe("WorkspaceList", () => {
  it("renders each workspace's issue with its status", () => {
    const wrapper = mount(WorkspaceList, {
      props: {
        workspaces: [
          { id: "1", issueId: "i-1", status: "starting" },
          { id: "2", issueId: "i-2", status: "running" },
        ],
      },
    });

    const items = wrapper.findAll(".workspace");
    expect(items).toHaveLength(2);
    expect(wrapper.text()).toContain("i-1");
    expect(items[1].attributes("data-status")).toBe("running");
  });

  it("shows an empty state when there are no workspaces", () => {
    const wrapper = mount(WorkspaceList, { props: { workspaces: [] } });

    expect(wrapper.find(".workspace").exists()).toBe(false);
    expect(wrapper.get(".empty").text()).toContain("No workspaces yet");
  });
});
