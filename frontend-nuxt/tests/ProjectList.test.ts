import { describe, it, expect } from "vitest";
import { mount } from "@vue/test-utils";
import ProjectList from "../components/ProjectList.vue";

// Ports the company-scoped project list (GET /api/companies/:companyId/projects).
describe("ProjectList", () => {
  it("renders each project name with its status", () => {
    const wrapper = mount(ProjectList, {
      props: {
        projects: [
          { id: "1", name: "Website", status: "backlog" },
          { id: "2", name: "Mobile", status: "in_progress" },
        ],
      },
    });

    const items = wrapper.findAll(".project");
    expect(items).toHaveLength(2);
    expect(wrapper.text()).toContain("Website");
    expect(items[1].attributes("data-status")).toBe("in_progress");
  });

  it("shows an empty state when there are no projects", () => {
    const wrapper = mount(ProjectList, { props: { projects: [] } });

    expect(wrapper.find(".project").exists()).toBe(false);
    expect(wrapper.get(".empty").text()).toContain("No projects yet");
  });
});
