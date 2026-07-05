import { describe, it, expect, vi } from "vitest";
import { loadBoard } from "../composables/useBoard";

// loadBoard is the orchestration the running Nuxt app uses to populate the Board
// shell: it fetches health, actor, companies, and the selected company's issues/
// agents/runs in parallel, returning exactly the props <Board> expects.
describe("loadBoard", () => {
  it("fetches all board data and shapes it into the Board props", async () => {
    const responses: Record<string, unknown> = {
      "/api/health": { status: "ok", version: "0.0.0" },
      "/api/whoami": { actor: "board" },
      "/api/companies": [{ id: "co-1", name: "Acme" }],
      "/api/companies/co-1/issues": [{ id: "i-1", title: "X", status: "backlog" }],
      "/api/companies/co-1/agents": [
        { id: "a-1", name: "Ada", role: "general", status: "idle", adapterType: "process" },
      ],
      "/api/companies/co-1/runs": [{ id: "r-1", agentId: "a-1", status: "queued" }],
    };
    const fetcher = vi.fn((url: string) => Promise.resolve(responses[url]));

    const board = await loadBoard(fetcher, "co-1");

    expect(board).toEqual({
      health: { status: "ok", version: "0.0.0" },
      actor: { actor: "board" },
      companies: [{ id: "co-1", name: "Acme" }],
      issues: [{ id: "i-1", title: "X", status: "backlog" }],
      agents: [
        { id: "a-1", name: "Ada", role: "general", status: "idle", adapterType: "process" },
      ],
      runs: [{ id: "r-1", agentId: "a-1", status: "queued" }],
    });

    // Company-scoped resources use the selected company id.
    expect(fetcher).toHaveBeenCalledWith("/api/companies/co-1/issues");
    expect(fetcher).toHaveBeenCalledWith("/api/companies/co-1/agents");
    expect(fetcher).toHaveBeenCalledWith("/api/companies/co-1/runs");
  });
});
