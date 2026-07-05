import { describe, it, expect } from "vitest";
import { mount } from "@vue/test-utils";
import ActorBadge from "../components/ActorBadge.vue";

// Renders the resolved actor (mirrors the backend /api/whoami contract).
describe("ActorBadge", () => {
  it("labels the board operator", () => {
    const wrapper = mount(ActorBadge, { props: { actor: "board" } });

    expect(wrapper.get(".actor-badge").attributes("data-actor")).toBe("board");
    expect(wrapper.text()).toContain("Board");
  });

  it("labels an agent with its company", () => {
    const wrapper = mount(ActorBadge, { props: { actor: "agent", companyId: "co-1" } });

    expect(wrapper.get(".actor-badge").attributes("data-actor")).toBe("agent");
    expect(wrapper.text()).toContain("Agent");
    expect(wrapper.text()).toContain("co-1");
  });
});
