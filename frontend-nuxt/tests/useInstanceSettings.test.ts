import { describe, it, expect, vi } from "vitest";
import {
  fetchInstanceSettings,
  updateInstanceSettings,
  updateInstanceGeneral,
  updateInstanceExperimental,
} from "../composables/useInstanceSettings";

// Ports the frontend data layer for the instance settings singleton
// (backend-rs/src/instance_settings.rs): read + PATCH-merge of general/experimental.
describe("fetchInstanceSettings", () => {
  it("requests the instance settings path and returns the singleton", async () => {
    const settings = {
      id: "instance",
      defaultEnvironmentId: null,
      general: { keyboardShortcuts: false },
      experimental: { enableEnvironments: false },
    };
    const fetcher = vi.fn().mockResolvedValue(settings);

    const result = await fetchInstanceSettings(fetcher);

    expect(fetcher).toHaveBeenCalledWith("/api/instance/settings");
    expect(result).toEqual(settings);
  });
});

describe("updateInstanceSettings", () => {
  it("PATCHes the top-level settings (e.g. defaultEnvironmentId)", async () => {
    const fetcher = vi.fn().mockResolvedValue({ id: "instance", defaultEnvironmentId: "env-1" });

    await updateInstanceSettings(fetcher, { defaultEnvironmentId: "env-1" });

    expect(fetcher).toHaveBeenCalledWith("/api/instance/settings", {
      method: "PATCH",
      body: { defaultEnvironmentId: "env-1" },
    });
  });
});

describe("updateInstanceGeneral", () => {
  it("PATCHes the general block and returns it", async () => {
    const general = { keyboardShortcuts: true, censorUsernameInLogs: false };
    const fetcher = vi.fn().mockResolvedValue(general);

    const result = await updateInstanceGeneral(fetcher, { keyboardShortcuts: true });

    expect(fetcher).toHaveBeenCalledWith("/api/instance/settings/general", {
      method: "PATCH",
      body: { keyboardShortcuts: true },
    });
    expect(result).toEqual(general);
  });
});

describe("updateInstanceExperimental", () => {
  it("PATCHes the experimental block and returns it", async () => {
    const experimental = { enableEnvironments: true };
    const fetcher = vi.fn().mockResolvedValue(experimental);

    const result = await updateInstanceExperimental(fetcher, { enableEnvironments: true });

    expect(fetcher).toHaveBeenCalledWith("/api/instance/settings/experimental", {
      method: "PATCH",
      body: { enableEnvironments: true },
    });
    expect(result).toEqual(experimental);
  });
});
