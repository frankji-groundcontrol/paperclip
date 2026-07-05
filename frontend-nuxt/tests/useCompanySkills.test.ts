import { describe, it, expect, vi } from "vitest";
import {
  fetchSkills,
  createSkill,
  fetchSkill,
  starSkill,
  unstarSkill,
} from "../composables/useCompanySkills";

// Ports the frontend data layer for a company's skills
// (backend-rs/src/company_skills.rs): list/filter/create/get + star/unstar.
describe("fetchSkills", () => {
  it("requests the company-scoped skills path and returns the list", async () => {
    const fetcher = vi
      .fn()
      .mockResolvedValue([{ id: "1", key: "pdf", name: "PDF Tools", starCount: 0 }]);

    const skills = await fetchSkills(fetcher, "co-1");

    expect(fetcher).toHaveBeenCalledWith("/api/companies/co-1/skills");
    expect(skills).toEqual([{ id: "1", key: "pdf", name: "PDF Tools", starCount: 0 }]);
  });

  it("appends q/category filters when provided", async () => {
    const fetcher = vi.fn().mockResolvedValue([]);

    await fetchSkills(fetcher, "co-1", { q: "pdf", category: "docs" });

    expect(fetcher).toHaveBeenCalledWith("/api/companies/co-1/skills?q=pdf&category=docs");
  });
});

describe("createSkill", () => {
  it("POSTs the payload to the company-scoped path", async () => {
    const created = { id: "9", key: "pdf", name: "PDF Tools", starCount: 0 };
    const fetcher = vi.fn().mockResolvedValue(created);

    const skill = await createSkill(fetcher, "co-1", { key: "pdf", name: "PDF Tools" });

    expect(fetcher).toHaveBeenCalledWith("/api/companies/co-1/skills", {
      method: "POST",
      body: { key: "pdf", name: "PDF Tools" },
    });
    expect(skill).toEqual(created);
  });
});

describe("fetchSkill", () => {
  it("requests the company-scoped skill detail path", async () => {
    const one = { id: "9", key: "pdf", name: "PDF Tools", starCount: 0 };
    const fetcher = vi.fn().mockResolvedValue(one);

    const skill = await fetchSkill(fetcher, "co-1", "9");

    expect(fetcher).toHaveBeenCalledWith("/api/companies/co-1/skills/9");
    expect(skill).toEqual(one);
  });
});

describe("starSkill / unstarSkill", () => {
  it("POSTs to star and DELETEs to unstar", async () => {
    const starred = { id: "9", key: "pdf", name: "PDF Tools", starCount: 1 };
    const fetcher = vi.fn().mockResolvedValue(starred);

    await starSkill(fetcher, "co-1", "9");
    expect(fetcher).toHaveBeenCalledWith("/api/companies/co-1/skills/9/star", { method: "POST" });

    await unstarSkill(fetcher, "co-1", "9");
    expect(fetcher).toHaveBeenCalledWith("/api/companies/co-1/skills/9/star", { method: "DELETE" });
  });
});
