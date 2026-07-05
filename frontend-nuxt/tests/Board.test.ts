import { describe, it, expect } from "vitest";
import { mount } from "@vue/test-utils";
import Board from "../components/Board.vue";

// The board shell composes the tested pieces (health, actor, domain lists) into
// a single page.
describe("Board", () => {
  it("renders health, actor, and the domain sections with their data", () => {
    const wrapper = mount(Board, {
      props: {
        health: { status: "ok" },
        actor: { actor: "board" },
        companies: [{ id: "1", name: "Acme" }],
        issues: [{ id: "1", title: "Fix bug", status: "backlog" }],
        agents: [{ id: "1", name: "Ada", role: "general", status: "idle" }],
        runs: [{ id: "1", agentId: "a-1", status: "queued" }],
      },
    });

    // Header: health + actor.
    expect(wrapper.get(".health-status").text()).toContain("Healthy");
    expect(wrapper.get(".actor-badge").text()).toContain("Board");

    // Domain sections delegate to the tested list components.
    expect(wrapper.get('[data-section="companies"]').text()).toContain("Companies");
    expect(wrapper.get('[data-section="companies"]').text()).toContain("Acme");
    expect(wrapper.get('[data-section="issues"]').text()).toContain("Fix bug");
    expect(wrapper.get('[data-section="agents"]').text()).toContain("Ada");
    expect(wrapper.get('[data-section="runs"]').text()).toContain("a-1");
  });
});
