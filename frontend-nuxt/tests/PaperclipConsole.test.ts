import { describe, it, expect } from "vitest";
import { mount, flushPromises } from "@vue/test-utils";
import PaperclipConsole from "../components/PaperclipConsole.vue";
import type { Fetcher } from "../composables/useApi";

// A stateful mock backend: login → session; create/list companies; run job → result.
function fakeBackend(): Fetcher {
  const companies: { id: string; name: string }[] = [];
  return async (url, options) => {
    const method = options?.method ?? "GET";
    if (url === "/api/auth/login") return { session: "pcs_test", whoami: { user: {} } };
    if (url === "/api/paperclip/companies" && method === "POST") {
      const name = (options?.body as { name: string }).name;
      companies.push({ id: "co-1", name });
      return { companyId: "co-1" };
    }
    if (url === "/api/paperclip/companies") return companies;
    if (url.endsWith("/jobs") && method === "POST") {
      return { jobId: "j1", status: "succeeded", result: "7", usage: { total_tokens: 18 } };
    }
    if (url.endsWith("/jobs")) return [];
    return null;
  };
}

describe("PaperclipConsole (non-agent user flow)", () => {
  it("logs in, forms a company, runs a job, and renders the real result", async () => {
    const wrapper = mount(PaperclipConsole, { props: { fetcher: fakeBackend() } });

    // logged out first
    expect(wrapper.find('[data-testid="login-form"]').exists()).toBe(true);

    // log in
    await wrapper.get('[data-testid="email"]').setValue("testuser1@example.com");
    await wrapper.get('[data-testid="password"]').setValue("pw");
    await wrapper.get('[data-testid="login-form"]').trigger("submit");
    await flushPromises();
    expect(wrapper.find('[data-testid="signed-in"]').exists()).toBe(true);

    // form a company
    await wrapper.get('[data-testid="company-name"]').setValue("Acme");
    await wrapper.get('[data-testid="new-company"]').trigger("submit");
    await flushPromises();
    expect(wrapper.text()).toContain("Acme");

    // the new company is auto-selected → run a job
    expect(wrapper.find('[data-testid="job-console"]').exists()).toBe(true);
    await wrapper.get('[data-testid="prompt"]').setValue("Reply with only the number 7");
    await wrapper.get('[data-testid="run-job"]').trigger("click");
    await flushPromises();

    expect(wrapper.get('[data-testid="result-text"]').text()).toBe("7");
    expect(wrapper.get('[data-testid="usage"]').text()).toContain("18");
  });

  it("shows an error when login fails", async () => {
    const failing: Fetcher = async () => {
      throw new Error("bad credentials");
    };
    const wrapper = mount(PaperclipConsole, { props: { fetcher: failing } });
    await wrapper.get('[data-testid="login-form"]').trigger("submit");
    await flushPromises();
    expect(wrapper.get('[data-testid="error"]').text()).toContain("Sign in failed");
  });
});
