import { describe, it, expect, vi } from "vitest";
import { fetchRoutines, createRoutine, deleteRoutine } from "../composables/useRoutines";

// Ports the frontend data layer for a company's routines, preserving company
// scoping in the request path.
describe("fetchRoutines", () => {
  it("requests the company-scoped routines path and returns the list", async () => {
    const fetcher = vi
      .fn()
      .mockResolvedValue([{ id: "1", title: "Daily standup", status: "active" }]);

    const routines = await fetchRoutines(fetcher, "co-1");

    expect(fetcher).toHaveBeenCalledWith("/api/companies/co-1/routines");
    expect(routines).toEqual([{ id: "1", title: "Daily standup", status: "active" }]);
  });
});

describe("createRoutine", () => {
  it("POSTs the payload to the company-scoped path and returns the created routine", async () => {
    const created = { id: "9", title: "Weekly review", status: "active" };
    const fetcher = vi.fn().mockResolvedValue(created);

    const routine = await createRoutine(fetcher, "co-1", { title: "Weekly review" });

    expect(fetcher).toHaveBeenCalledWith("/api/companies/co-1/routines", {
      method: "POST",
      body: { title: "Weekly review" },
    });
    expect(routine).toEqual(created);
  });
});

describe("deleteRoutine", () => {
  it("DELETEs the company-scoped routine path", async () => {
    const fetcher = vi.fn().mockResolvedValue(undefined);

    await deleteRoutine(fetcher, "co-1", "9");

    expect(fetcher).toHaveBeenCalledWith("/api/companies/co-1/routines/9", {
      method: "DELETE",
    });
  });
});
