import { describe, it, expect } from "vitest";
import { mount } from "@vue/test-utils";
import ApprovalList from "../components/ApprovalList.vue";

// Ports the company-scoped approval list (GET /api/companies/:companyId/approvals).
describe("ApprovalList", () => {
  it("renders each approval's issue with its status", () => {
    const wrapper = mount(ApprovalList, {
      props: {
        approvals: [
          { id: "1", issueId: "i-1", status: "pending" },
          { id: "2", issueId: "i-2", status: "approved" },
        ],
      },
    });

    const items = wrapper.findAll(".approval");
    expect(items).toHaveLength(2);
    expect(wrapper.text()).toContain("i-1");
    expect(items[1].attributes("data-status")).toBe("approved");
  });

  it("shows an empty state when there are no approvals", () => {
    const wrapper = mount(ApprovalList, { props: { approvals: [] } });

    expect(wrapper.find(".approval").exists()).toBe(false);
    expect(wrapper.get(".empty").text()).toContain("No approvals yet");
  });
});
