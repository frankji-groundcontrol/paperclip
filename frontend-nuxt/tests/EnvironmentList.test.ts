import { describe, it, expect } from "vitest";
import { mount } from "@vue/test-utils";
import EnvironmentList from "../components/EnvironmentList.vue";

// Ports the company-scoped environment list (GET /api/companies/:companyId/environments).
describe("EnvironmentList", () => {
  it("renders each environment name with its driver and status", () => {
    const wrapper = mount(EnvironmentList, {
      props: {
        environments: [
          { id: "1", name: "prod", driver: "local", status: "active" },
          { id: "2", name: "sandbox", driver: "sandbox", status: "archived" },
        ],
      },
    });

    const items = wrapper.findAll(".environment");
    expect(items).toHaveLength(2);
    expect(wrapper.text()).toContain("prod");
    expect(items[0].attributes("data-driver")).toBe("local");
    expect(items[1].attributes("data-status")).toBe("archived");
  });

  it("shows an empty state when there are no environments", () => {
    const wrapper = mount(EnvironmentList, { props: { environments: [] } });
    expect(wrapper.find(".environment").exists()).toBe(false);
    expect(wrapper.get(".empty").text()).toContain("No environments yet");
  });
});
