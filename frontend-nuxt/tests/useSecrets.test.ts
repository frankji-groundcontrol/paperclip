import { describe, it, expect, vi } from "vitest";
import { fetchSecrets, createSecret, deleteSecret } from "../composables/useSecrets";

// Ports the frontend data layer for a company's secret references.
describe("fetchSecrets", () => {
  it("requests the company-scoped secrets path and returns the references", async () => {
    const fetcher = vi
      .fn()
      .mockResolvedValue([{ id: "1", name: "API_KEY", provider: "local" }]);

    const secrets = await fetchSecrets(fetcher, "co-1");

    expect(fetcher).toHaveBeenCalledWith("/api/companies/co-1/secrets");
    expect(secrets).toEqual([{ id: "1", name: "API_KEY", provider: "local" }]);
  });
});

describe("createSecret", () => {
  it("POSTs name+value to the company-scoped path and returns only the reference", async () => {
    // The backend never echoes the value; the composable returns whatever
    // reference it gets back.
    const created = { id: "9", name: "API_KEY", provider: "local" };
    const fetcher = vi.fn().mockResolvedValue(created);

    const secret = await createSecret(fetcher, "co-1", { name: "API_KEY", value: "s3cr3t" });

    expect(fetcher).toHaveBeenCalledWith("/api/companies/co-1/secrets", {
      method: "POST",
      body: { name: "API_KEY", value: "s3cr3t" },
    });
    expect(secret).toEqual(created);
  });
});

describe("deleteSecret", () => {
  it("DELETEs the company-scoped secret path", async () => {
    const fetcher = vi.fn().mockResolvedValue(undefined);

    await deleteSecret(fetcher, "co-1", "9");

    expect(fetcher).toHaveBeenCalledWith("/api/companies/co-1/secrets/9", {
      method: "DELETE",
    });
  });
});
