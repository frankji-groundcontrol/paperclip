import { describe, it, expect } from "vitest";
import { mount } from "@vue/test-utils";
import PluginList from "../components/PluginList.vue";

// Ports the company-scoped plugin registry (GET /api/companies/:companyId/plugins).
describe("PluginList", () => {
  it("renders each installed plugin with its enabled state", () => {
    const wrapper = mount(PluginList, {
      props: {
        plugins: [
          { id: "1", pluginId: "llm-wiki", enabled: true },
          { id: "2", pluginId: "workspace-diff", enabled: false },
        ],
      },
    });

    const items = wrapper.findAll(".plugin");
    expect(items).toHaveLength(2);
    expect(wrapper.text()).toContain("llm-wiki");
    expect(items[1].attributes("data-enabled")).toBe("false");
  });

  it("shows an empty state when there are no plugins", () => {
    const wrapper = mount(PluginList, { props: { plugins: [] } });

    expect(wrapper.find(".plugin").exists()).toBe(false);
    expect(wrapper.get(".empty").text()).toContain("No plugins yet");
  });
});
