import { describe, it, expect, vi } from "vitest";
import { fetchGoals, createGoal, updateGoal, deleteGoal } from "../composables/useGoals";

// Ports the frontend data layer for a company's goals, mirroring the Rust
// backend (backend-rs/src/goals.rs): company-scoped list + create/update/delete.
describe("fetchGoals", () => {
  it("requests the company-scoped goals path and returns the list", async () => {
    const fetcher = vi
      .fn()
      .mockResolvedValue([{ id: "1", title: "Ship v2", level: "task", status: "planned" }]);

    const goals = await fetchGoals(fetcher, "co-1");

    expect(fetcher).toHaveBeenCalledWith("/api/companies/co-1/goals");
    expect(goals).toEqual([{ id: "1", title: "Ship v2", level: "task", status: "planned" }]);
  });
});

describe("createGoal", () => {
  it("POSTs the payload to the company-scoped path and returns the created goal", async () => {
    const created = { id: "9", title: "Grow revenue", level: "company", status: "active" };
    const fetcher = vi.fn().mockResolvedValue(created);

    const goal = await createGoal(fetcher, "co-1", { title: "Grow revenue", level: "company" });

    expect(fetcher).toHaveBeenCalledWith("/api/companies/co-1/goals", {
      method: "POST",
      body: { title: "Grow revenue", level: "company" },
    });
    expect(goal).toEqual(created);
  });
});

describe("updateGoal", () => {
  it("PATCHes a goal's status on the company-scoped path", async () => {
    const updated = { id: "9", title: "Grow revenue", level: "company", status: "achieved" };
    const fetcher = vi.fn().mockResolvedValue(updated);

    const goal = await updateGoal(fetcher, "co-1", "9", { status: "achieved" });

    expect(fetcher).toHaveBeenCalledWith("/api/companies/co-1/goals/9", {
      method: "PATCH",
      body: { status: "achieved" },
    });
    expect(goal).toEqual(updated);
  });
});

describe("deleteGoal", () => {
  it("DELETEs the company-scoped goal path", async () => {
    const fetcher = vi.fn().mockResolvedValue(undefined);

    await deleteGoal(fetcher, "co-1", "9");

    expect(fetcher).toHaveBeenCalledWith("/api/companies/co-1/goals/9", {
      method: "DELETE",
    });
  });
});
