import { describe, it, expect } from "vitest";
import { mount } from "@vue/test-utils";
import CompanySkillList from "../components/CompanySkillList.vue";

// Ports the company-scoped skill list (GET /api/companies/:companyId/skills).
describe("CompanySkillList", () => {
  it("renders each skill name with its star count", () => {
    const wrapper = mount(CompanySkillList, {
      props: {
        skills: [
          { id: "1", key: "pdf", name: "PDF Tools", starCount: 3 },
          { id: "2", key: "sql", name: "SQL Helper", starCount: 0 },
        ],
      },
    });

    const items = wrapper.findAll(".skill");
    expect(items).toHaveLength(2);
    expect(wrapper.text()).toContain("PDF Tools");
    expect(items[0].attributes("data-stars")).toBe("3");
  });

  it("shows an empty state when there are no skills", () => {
    const wrapper = mount(CompanySkillList, { props: { skills: [] } });

    expect(wrapper.find(".skill").exists()).toBe(false);
    expect(wrapper.get(".empty").text()).toContain("No skills yet");
  });
});
