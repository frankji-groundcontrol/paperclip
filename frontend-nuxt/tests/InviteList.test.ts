import { describe, it, expect } from "vitest";
import { mount } from "@vue/test-utils";
import InviteList from "../components/InviteList.vue";

// Ports the company-scoped invite list (GET /api/companies/:companyId/invites).
describe("InviteList", () => {
  it("renders each invite with its join types and state", () => {
    const wrapper = mount(InviteList, {
      props: {
        invites: [
          { id: "1", token: "t1", allowedJoinTypes: "both", state: "active" },
          { id: "2", token: "t2", allowedJoinTypes: "human", state: "revoked" },
        ],
      },
    });

    const items = wrapper.findAll(".invite");
    expect(items).toHaveLength(2);
    expect(items[0].attributes("data-state")).toBe("active");
    expect(wrapper.text()).toContain("both");
  });

  it("shows an empty state when there are no invites", () => {
    const wrapper = mount(InviteList, { props: { invites: [] } });
    expect(wrapper.find(".invite").exists()).toBe(false);
    expect(wrapper.get(".empty").text()).toContain("No invites yet");
  });
});
