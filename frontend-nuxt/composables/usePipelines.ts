export type Pipeline = {
  id: string;
  companyId?: string;
  key: string;
  name: string;
  description?: string | null;
  projectId?: string | null;
  enforceTransitions?: boolean;
  archived: boolean;
};

/** ofetch-style request options for mutating calls. */
export type FetchOptions = { method?: string; body?: unknown };

/**
 * HTTP boundary — in the Nuxt app this is `$fetch` (ofetch). Reads pass just a
 * URL; mutations pass options (method, body).
 */
export type Fetcher = (url: string, options?: FetchOptions) => Promise<unknown>;

/** Loads a company's pipelines (`GET /api/companies/:companyId/pipelines`). */
export async function fetchPipelines(fetcher: Fetcher, companyId: string): Promise<Pipeline[]> {
  const data = await fetcher(`/api/companies/${companyId}/pipelines`);
  return data as Pipeline[];
}

/** Creates a pipeline (`POST /api/companies/:companyId/pipelines`). */
export async function createPipeline(
  fetcher: Fetcher,
  companyId: string,
  input: {
    key: string;
    name: string;
    description?: string;
    projectId?: string;
    enforceTransitions?: boolean;
  },
): Promise<Pipeline> {
  const data = await fetcher(`/api/companies/${companyId}/pipelines`, {
    method: "POST",
    body: input,
  });
  return data as Pipeline;
}

/** Loads one pipeline (`GET /api/companies/:companyId/pipelines/:pipelineId`). */
export async function fetchPipeline(
  fetcher: Fetcher,
  companyId: string,
  pipelineId: string,
): Promise<Pipeline> {
  const data = await fetcher(`/api/companies/${companyId}/pipelines/${pipelineId}`);
  return data as Pipeline;
}

/** Updates a pipeline (`PATCH /api/companies/:companyId/pipelines/:pipelineId`). */
export async function updatePipeline(
  fetcher: Fetcher,
  companyId: string,
  pipelineId: string,
  patch: { name?: string; description?: string; enforceTransitions?: boolean; archived?: boolean },
): Promise<Pipeline> {
  const data = await fetcher(`/api/companies/${companyId}/pipelines/${pipelineId}`, {
    method: "PATCH",
    body: patch,
  });
  return data as Pipeline;
}

export type Stage = {
  id: string;
  pipelineId?: string;
  key: string;
  name: string;
  kind: string;
  position: number;
  config?: Record<string, unknown>;
};

/** Loads a pipeline's stages (`GET /api/companies/:companyId/pipelines/:pipelineId/stages`). */
export async function fetchStages(
  fetcher: Fetcher,
  companyId: string,
  pipelineId: string,
): Promise<Stage[]> {
  const data = await fetcher(`/api/companies/${companyId}/pipelines/${pipelineId}/stages`);
  return data as Stage[];
}

/** Creates a stage (`POST /api/companies/:companyId/pipelines/:pipelineId/stages`). */
export async function createStage(
  fetcher: Fetcher,
  companyId: string,
  pipelineId: string,
  input: { key: string; name: string; kind: string; position?: number; config?: Record<string, unknown> },
): Promise<Stage> {
  const data = await fetcher(`/api/companies/${companyId}/pipelines/${pipelineId}/stages`, {
    method: "POST",
    body: input,
  });
  return data as Stage;
}

/** Updates a stage (`PATCH /…/pipelines/:pipelineId/stages/:stageId`). */
export async function updateStage(
  fetcher: Fetcher,
  companyId: string,
  pipelineId: string,
  stageId: string,
  patch: { key?: string; name?: string; kind?: string; position?: number; config?: Record<string, unknown> },
): Promise<Stage> {
  const data = await fetcher(
    `/api/companies/${companyId}/pipelines/${pipelineId}/stages/${stageId}`,
    { method: "PATCH", body: patch },
  );
  return data as Stage;
}

/** Deletes a stage (`DELETE /…/pipelines/:pipelineId/stages/:stageId`). */
export async function deleteStage(
  fetcher: Fetcher,
  companyId: string,
  pipelineId: string,
  stageId: string,
): Promise<void> {
  await fetcher(`/api/companies/${companyId}/pipelines/${pipelineId}/stages/${stageId}`, {
    method: "DELETE",
  });
}

export type Case = {
  id: string;
  pipelineId?: string;
  caseKey?: string | null;
  title: string;
  summary?: string | null;
  fields?: Record<string, unknown>;
  stageKey?: string | null;
  parentCaseId?: string | null;
};

/** Loads a pipeline's cases (`GET /…/pipelines/:pipelineId/cases`). */
export async function fetchCases(
  fetcher: Fetcher,
  companyId: string,
  pipelineId: string,
): Promise<Case[]> {
  const data = await fetcher(`/api/companies/${companyId}/pipelines/${pipelineId}/cases`);
  return data as Case[];
}

/** Ingests a case (`POST /…/pipelines/:pipelineId/cases`). */
export async function ingestCase(
  fetcher: Fetcher,
  companyId: string,
  pipelineId: string,
  input: {
    title: string;
    caseKey?: string;
    summary?: string;
    fields?: Record<string, unknown>;
    stageKey?: string;
    parentCaseId?: string;
  },
): Promise<Case> {
  const data = await fetcher(`/api/companies/${companyId}/pipelines/${pipelineId}/cases`, {
    method: "POST",
    body: input,
  });
  return data as Case;
}

/** Loads one case (`GET /…/pipelines/:pipelineId/cases/:caseId`). */
export async function fetchCase(
  fetcher: Fetcher,
  companyId: string,
  pipelineId: string,
  caseId: string,
): Promise<Case> {
  const data = await fetcher(
    `/api/companies/${companyId}/pipelines/${pipelineId}/cases/${caseId}`,
  );
  return data as Case;
}

/** Updates a case (`PATCH /…/pipelines/:pipelineId/cases/:caseId`). */
export async function updateCase(
  fetcher: Fetcher,
  companyId: string,
  pipelineId: string,
  caseId: string,
  patch: { title?: string; summary?: string; fields?: Record<string, unknown>; parentCaseId?: string },
): Promise<Case> {
  const data = await fetcher(
    `/api/companies/${companyId}/pipelines/${pipelineId}/cases/${caseId}`,
    { method: "PATCH", body: patch },
  );
  return data as Case;
}

/**
 * Transitions a case to a stage with optimistic concurrency
 * (`POST /…/cases/:caseId/transition`). `expectedVersion` guards against
 * concurrent moves; a mismatch yields HTTP 409 from the backend.
 */
export async function transitionCase(
  fetcher: Fetcher,
  companyId: string,
  pipelineId: string,
  caseId: string,
  toStageKey: string,
  expectedVersion: number,
): Promise<Case> {
  const data = await fetcher(
    `/api/companies/${companyId}/pipelines/${pipelineId}/cases/${caseId}/transition`,
    { method: "POST", body: { toStageKey, expectedVersion } },
  );
  return data as Case;
}
