import { describe, it, expect, vi } from "vitest";
import { fetchIssues, createIssue, deleteIssue } from "../composables/useIssues";

// Ports the frontend data layer for a company's issues, preserving company
// scoping in the request path.
describe("fetchIssues", () => {
  it("requests the company-scoped issues path and returns the list", async () => {
    const fetcher = vi
      .fn()
      .mockResolvedValue([{ id: "1", title: "Fix bug", status: "backlog" }]);

    const issues = await fetchIssues(fetcher, "co-1");

    expect(fetcher).toHaveBeenCalledWith("/api/companies/co-1/issues");
    expect(issues).toEqual([{ id: "1", title: "Fix bug", status: "backlog" }]);
  });
});

// Mutations mirror the Rust backend: POST creates, DELETE removes, both
// company-scoped. The fetcher takes ofetch-style options (method, body).
describe("createIssue", () => {
  it("POSTs the payload to the company-scoped path and returns the created issue", async () => {
    const created = { id: "9", title: "New issue", status: "backlog" };
    const fetcher = vi.fn().mockResolvedValue(created);

    const issue = await createIssue(fetcher, "co-1", { title: "New issue" });

    expect(fetcher).toHaveBeenCalledWith("/api/companies/co-1/issues", {
      method: "POST",
      body: { title: "New issue" },
    });
    expect(issue).toEqual(created);
  });
});

describe("deleteIssue", () => {
  it("DELETEs the company-scoped issue path", async () => {
    const fetcher = vi.fn().mockResolvedValue(undefined);

    await deleteIssue(fetcher, "co-1", "9");

    expect(fetcher).toHaveBeenCalledWith("/api/companies/co-1/issues/9", {
      method: "DELETE",
    });
  });
});
