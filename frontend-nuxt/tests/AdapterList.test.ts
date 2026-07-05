import { describe, it, expect } from "vitest";
import { mount } from "@vue/test-utils";
import AdapterList from "../components/AdapterList.vue";

// Ports the company-scoped adapter registry (GET /api/companies/:companyId/adapters).
describe("AdapterList", () => {
  it("renders each configured adapter with its enabled state", () => {
    const wrapper = mount(AdapterList, {
      props: {
        adapters: [
          { id: "1", adapterType: "claude_local", enabled: true },
          { id: "2", adapterType: "codex", enabled: false },
        ],
      },
    });

    const items = wrapper.findAll(".adapter");
    expect(items).toHaveLength(2);
    expect(wrapper.text()).toContain("claude_local");
    expect(items[1].attributes("data-enabled")).toBe("false");
  });

  it("shows an empty state when there are no adapters", () => {
    const wrapper = mount(AdapterList, { props: { adapters: [] } });

    expect(wrapper.find(".adapter").exists()).toBe(false);
    expect(wrapper.get(".empty").text()).toContain("No adapters yet");
  });
});
