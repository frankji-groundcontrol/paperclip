import { describe, it, expect } from "vitest";
import { mount } from "@vue/test-utils";
import HealthStatus from "../components/HealthStatus.vue";

// Ports the frontend health surface: the board shows whether the control plane
// is healthy. Mirrors the `status: "ok"` contract of GET /api/health.
describe("HealthStatus", () => {
  it("renders a healthy label when status is ok", () => {
    const wrapper = mount(HealthStatus, { props: { status: "ok" } });

    expect(wrapper.text()).toContain("Healthy");
    expect(wrapper.get(".health-status").attributes("data-status")).toBe("ok");
  });
});
