import { describe, it, expect, vi } from "vitest";
import {
  fetchBoardApiKeys,
  createBoardApiKey,
  deleteBoardApiKey,
} from "../composables/useBoardKeys";

// Ports the frontend data layer for the board user's API keys
// (backend-rs/src/board_keys.rs): the token is revealed on create, never listed.
describe("fetchBoardApiKeys", () => {
  it("requests the board-api-keys path and returns the references", async () => {
    const fetcher = vi.fn().mockResolvedValue([{ id: "1", name: "cli", expiresAt: null }]);

    const keys = await fetchBoardApiKeys(fetcher);

    expect(fetcher).toHaveBeenCalledWith("/api/board-api-keys");
    expect(keys).toEqual([{ id: "1", name: "cli", expiresAt: null }]);
  });
});

describe("createBoardApiKey", () => {
  it("POSTs the payload and returns the created key (with one-time token)", async () => {
    const created = { id: "9", name: "cli", token: "secret-token", expiresAt: null };
    const fetcher = vi.fn().mockResolvedValue(created);

    const key = await createBoardApiKey(fetcher, { name: "cli" });

    expect(fetcher).toHaveBeenCalledWith("/api/board-api-keys", {
      method: "POST",
      body: { name: "cli" },
    });
    expect(key).toEqual(created);
  });
});

describe("deleteBoardApiKey", () => {
  it("DELETEs the key by id", async () => {
    const fetcher = vi.fn().mockResolvedValue(undefined);

    await deleteBoardApiKey(fetcher, "9");

    expect(fetcher).toHaveBeenCalledWith("/api/board-api-keys/9", { method: "DELETE" });
  });
});
