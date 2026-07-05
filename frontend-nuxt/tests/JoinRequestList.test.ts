import { describe, it, expect } from "vitest";
import { mount } from "@vue/test-utils";
import JoinRequestList from "../components/JoinRequestList.vue";

// Ports the company-scoped join-request list (GET /api/companies/:companyId/join-requests).
describe("JoinRequestList", () => {
  it("renders each request with its type and status", () => {
    const wrapper = mount(JoinRequestList, {
      props: {
        requests: [
          { id: "1", requestType: "human", requesterName: "Alice", status: "pending_approval" },
          { id: "2", requestType: "agent", requesterName: null, status: "approved" },
        ],
      },
    });

    const items = wrapper.findAll(".join-request");
    expect(items).toHaveLength(2);
    expect(items[0].attributes("data-status")).toBe("pending_approval");
    expect(items[1].attributes("data-type")).toBe("agent");
    expect(wrapper.text()).toContain("Alice");
  });

  it("shows an empty state when there are no requests", () => {
    const wrapper = mount(JoinRequestList, { props: { requests: [] } });
    expect(wrapper.find(".join-request").exists()).toBe(false);
    expect(wrapper.get(".empty").text()).toContain("No join requests");
  });
});
