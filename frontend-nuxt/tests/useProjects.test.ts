import { describe, it, expect, vi } from "vitest";
import { fetchProjects, createProject, deleteProject } from "../composables/useProjects";

// Ports the frontend data layer for a company's projects, preserving company
// scoping in the request path.
describe("fetchProjects", () => {
  it("requests the company-scoped projects path and returns the list", async () => {
    const fetcher = vi
      .fn()
      .mockResolvedValue([{ id: "1", name: "Website", status: "backlog" }]);

    const projects = await fetchProjects(fetcher, "co-1");

    expect(fetcher).toHaveBeenCalledWith("/api/companies/co-1/projects");
    expect(projects).toEqual([{ id: "1", name: "Website", status: "backlog" }]);
  });
});

describe("createProject", () => {
  it("POSTs the payload to the company-scoped path and returns the created project", async () => {
    const created = { id: "9", name: "Mobile app", status: "backlog" };
    const fetcher = vi.fn().mockResolvedValue(created);

    const project = await createProject(fetcher, "co-1", { name: "Mobile app" });

    expect(fetcher).toHaveBeenCalledWith("/api/companies/co-1/projects", {
      method: "POST",
      body: { name: "Mobile app" },
    });
    expect(project).toEqual(created);
  });
});

describe("deleteProject", () => {
  it("DELETEs the company-scoped project path", async () => {
    const fetcher = vi.fn().mockResolvedValue(undefined);

    await deleteProject(fetcher, "co-1", "9");

    expect(fetcher).toHaveBeenCalledWith("/api/companies/co-1/projects/9", {
      method: "DELETE",
    });
  });
});
