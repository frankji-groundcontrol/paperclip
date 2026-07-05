import { describe, it, expect } from "vitest";
import { mount } from "@vue/test-utils";
import IssueList from "../components/IssueList.vue";

// Ports the company-scoped issue list (GET /api/companies/:companyId/issues).
describe("IssueList", () => {
  it("renders each issue title with its status", () => {
    const wrapper = mount(IssueList, {
      props: {
        issues: [
          { id: "1", title: "Fix bug", status: "backlog" },
          { id: "2", title: "Ship it", status: "in_progress" },
        ],
      },
    });

    const items = wrapper.findAll(".issue");
    expect(items).toHaveLength(2);
    expect(wrapper.text()).toContain("Fix bug");
    expect(items[1].attributes("data-status")).toBe("in_progress");
  });

  it("shows an empty state when there are no issues", () => {
    const wrapper = mount(IssueList, { props: { issues: [] } });

    expect(wrapper.find(".issue").exists()).toBe(false);
    expect(wrapper.get(".empty").text()).toContain("No issues yet");
  });
});
