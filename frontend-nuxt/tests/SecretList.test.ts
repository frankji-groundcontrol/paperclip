import { describe, it, expect } from "vitest";
import { mount } from "@vue/test-utils";
import SecretList from "../components/SecretList.vue";

// Ports the company-scoped secret list (GET /api/companies/:companyId/secrets):
// references only — the API never returns values, and neither does the UI.
describe("SecretList", () => {
  it("renders each secret's name and provider", () => {
    const wrapper = mount(SecretList, {
      props: {
        secrets: [
          { id: "1", name: "API_KEY", provider: "local" },
          { id: "2", name: "DB_URL", provider: "aws" },
        ],
      },
    });

    const items = wrapper.findAll(".secret");
    expect(items).toHaveLength(2);
    expect(wrapper.text()).toContain("API_KEY");
    expect(items[1].attributes("data-provider")).toBe("aws");
  });

  it("shows an empty state when there are no secrets", () => {
    const wrapper = mount(SecretList, { props: { secrets: [] } });

    expect(wrapper.find(".secret").exists()).toBe(false);
    expect(wrapper.get(".empty").text()).toContain("No secrets yet");
  });
});
