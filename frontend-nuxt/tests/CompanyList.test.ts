import { describe, it, expect } from "vitest";
import { mount } from "@vue/test-utils";
import CompanyList from "../components/CompanyList.vue";

// Ports the companies list surface (GET /api/companies) into the board UI.
describe("CompanyList", () => {
  it("renders each company name", () => {
    const wrapper = mount(CompanyList, {
      props: {
        companies: [
          { id: "1", name: "Acme" },
          { id: "2", name: "Beta" },
        ],
      },
    });

    expect(wrapper.findAll(".company")).toHaveLength(2);
    expect(wrapper.text()).toContain("Acme");
    expect(wrapper.text()).toContain("Beta");
  });

  it("shows an empty state when there are no companies", () => {
    const wrapper = mount(CompanyList, { props: { companies: [] } });

    expect(wrapper.find(".company").exists()).toBe(false);
    expect(wrapper.get(".empty").text()).toContain("No companies yet");
  });
});
