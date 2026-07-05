import { describe, it, expect, vi } from "vitest";
import { fetchAgentIcons } from "../composables/useAgentIcons";

// Ports a UI helper over the llms agent-icons.txt endpoint
// (backend-rs/src/llms.rs): fetch the text list and parse out the icon names.
describe("fetchAgentIcons", () => {
  it("requests the icons text endpoint and parses the `- name` lines", async () => {
    const text = [
      "# Paperclip Agent Icon Names",
      "",
      "Set the `icon` field on hire/create payloads to one of:",
      "- bot",
      "- search",
      "- fingerprint",
      "",
    ].join("\n");
    const fetcher = vi.fn().mockResolvedValue(text);

    const icons = await fetchAgentIcons(fetcher);

    expect(fetcher).toHaveBeenCalledWith("/llms/agent-icons.txt");
    expect(icons).toEqual(["bot", "search", "fingerprint"]);
  });
});
