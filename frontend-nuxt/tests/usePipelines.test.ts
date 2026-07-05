import { describe, it, expect, vi } from "vitest";
import {
  fetchPipelines,
  createPipeline,
  fetchPipeline,
  updatePipeline,
  fetchStages,
  createStage,
  updateStage,
  deleteStage,
  fetchCases,
  ingestCase,
  fetchCase,
  updateCase,
  transitionCase,
} from "../composables/usePipelines";

// Ports the frontend data layer for a company's pipelines
// (backend-rs/src/pipelines.rs): list/create/get/patch of the pipeline entity.
describe("fetchPipelines", () => {
  it("requests the company-scoped pipelines path and returns the list", async () => {
    const fetcher = vi
      .fn()
      .mockResolvedValue([{ id: "1", key: "intake", name: "Intake", archived: false }]);

    const pipelines = await fetchPipelines(fetcher, "co-1");

    expect(fetcher).toHaveBeenCalledWith("/api/companies/co-1/pipelines");
    expect(pipelines).toEqual([{ id: "1", key: "intake", name: "Intake", archived: false }]);
  });
});

describe("createPipeline", () => {
  it("POSTs the payload to the company-scoped path and returns the created pipeline", async () => {
    const created = { id: "9", key: "intake", name: "Intake", archived: false };
    const fetcher = vi.fn().mockResolvedValue(created);

    const pipeline = await createPipeline(fetcher, "co-1", { key: "intake", name: "Intake" });

    expect(fetcher).toHaveBeenCalledWith("/api/companies/co-1/pipelines", {
      method: "POST",
      body: { key: "intake", name: "Intake" },
    });
    expect(pipeline).toEqual(created);
  });
});

describe("fetchPipeline", () => {
  it("requests the company-scoped pipeline detail path", async () => {
    const one = { id: "9", key: "intake", name: "Intake", archived: false };
    const fetcher = vi.fn().mockResolvedValue(one);

    const pipeline = await fetchPipeline(fetcher, "co-1", "9");

    expect(fetcher).toHaveBeenCalledWith("/api/companies/co-1/pipelines/9");
    expect(pipeline).toEqual(one);
  });
});

describe("updatePipeline", () => {
  it("PATCHes the pipeline on the company-scoped detail path", async () => {
    const updated = { id: "9", key: "intake", name: "Intake", archived: true };
    const fetcher = vi.fn().mockResolvedValue(updated);

    const pipeline = await updatePipeline(fetcher, "co-1", "9", { archived: true });

    expect(fetcher).toHaveBeenCalledWith("/api/companies/co-1/pipelines/9", {
      method: "PATCH",
      body: { archived: true },
    });
    expect(pipeline).toEqual(updated);
  });
});

describe("fetchStages", () => {
  it("requests the pipeline-scoped stages path and returns the list", async () => {
    const fetcher = vi
      .fn()
      .mockResolvedValue([{ id: "s1", key: "triage", name: "Triage", kind: "working", position: 0 }]);

    const stages = await fetchStages(fetcher, "co-1", "p-1");

    expect(fetcher).toHaveBeenCalledWith("/api/companies/co-1/pipelines/p-1/stages");
    expect(stages).toEqual([
      { id: "s1", key: "triage", name: "Triage", kind: "working", position: 0 },
    ]);
  });
});

describe("createStage", () => {
  it("POSTs the payload to the pipeline-scoped stages path", async () => {
    const created = { id: "s9", key: "triage", name: "Triage", kind: "working", position: 0 };
    const fetcher = vi.fn().mockResolvedValue(created);

    const stage = await createStage(fetcher, "co-1", "p-1", {
      key: "triage",
      name: "Triage",
      kind: "working",
    });

    expect(fetcher).toHaveBeenCalledWith("/api/companies/co-1/pipelines/p-1/stages", {
      method: "POST",
      body: { key: "triage", name: "Triage", kind: "working" },
    });
    expect(stage).toEqual(created);
  });
});

describe("updateStage", () => {
  it("PATCHes the pipeline-scoped stage path", async () => {
    const updated = { id: "s9", key: "triage", name: "Triage", kind: "review", position: 3 };
    const fetcher = vi.fn().mockResolvedValue(updated);

    const stage = await updateStage(fetcher, "co-1", "p-1", "s9", { kind: "review", position: 3 });

    expect(fetcher).toHaveBeenCalledWith("/api/companies/co-1/pipelines/p-1/stages/s9", {
      method: "PATCH",
      body: { kind: "review", position: 3 },
    });
    expect(stage).toEqual(updated);
  });
});

describe("deleteStage", () => {
  it("DELETEs the pipeline-scoped stage path", async () => {
    const fetcher = vi.fn().mockResolvedValue(undefined);

    await deleteStage(fetcher, "co-1", "p-1", "s9");

    expect(fetcher).toHaveBeenCalledWith("/api/companies/co-1/pipelines/p-1/stages/s9", {
      method: "DELETE",
    });
  });
});

describe("fetchCases", () => {
  it("requests the pipeline-scoped cases path and returns the list", async () => {
    const fetcher = vi.fn().mockResolvedValue([{ id: "c1", title: "Refund #12" }]);

    const cases = await fetchCases(fetcher, "co-1", "p-1");

    expect(fetcher).toHaveBeenCalledWith("/api/companies/co-1/pipelines/p-1/cases");
    expect(cases).toEqual([{ id: "c1", title: "Refund #12" }]);
  });
});

describe("ingestCase", () => {
  it("POSTs the payload to the pipeline-scoped cases path", async () => {
    const created = { id: "c9", title: "Refund #12" };
    const fetcher = vi.fn().mockResolvedValue(created);

    const created2 = await ingestCase(fetcher, "co-1", "p-1", { title: "Refund #12" });

    expect(fetcher).toHaveBeenCalledWith("/api/companies/co-1/pipelines/p-1/cases", {
      method: "POST",
      body: { title: "Refund #12" },
    });
    expect(created2).toEqual(created);
  });
});

describe("fetchCase", () => {
  it("requests the pipeline-scoped case detail path", async () => {
    const one = { id: "c9", title: "Refund #12" };
    const fetcher = vi.fn().mockResolvedValue(one);

    const got = await fetchCase(fetcher, "co-1", "p-1", "c9");

    expect(fetcher).toHaveBeenCalledWith("/api/companies/co-1/pipelines/p-1/cases/c9");
    expect(got).toEqual(one);
  });
});

describe("updateCase", () => {
  it("PATCHes the pipeline-scoped case detail path", async () => {
    const updated = { id: "c9", title: "Refund #12 v2" };
    const fetcher = vi.fn().mockResolvedValue(updated);

    const out = await updateCase(fetcher, "co-1", "p-1", "c9", { title: "Refund #12 v2" });

    expect(fetcher).toHaveBeenCalledWith("/api/companies/co-1/pipelines/p-1/cases/c9", {
      method: "PATCH",
      body: { title: "Refund #12 v2" },
    });
    expect(out).toEqual(updated);
  });
});

describe("transitionCase", () => {
  it("POSTs the transition (toStageKey + expectedVersion) to the case path", async () => {
    const moved = { id: "c9", stageKey: "review", version: 2 };
    const fetcher = vi.fn().mockResolvedValue(moved);

    const out = await transitionCase(fetcher, "co-1", "p-1", "c9", "review", 1);

    expect(fetcher).toHaveBeenCalledWith(
      "/api/companies/co-1/pipelines/p-1/cases/c9/transition",
      { method: "POST", body: { toStageKey: "review", expectedVersion: 1 } },
    );
    expect(out).toEqual(moved);
  });
});
