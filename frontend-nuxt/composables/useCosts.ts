export type CostEvent = {
  id: string;
  companyId: string;
  agentId: string;
  provider: string;
  biller: string;
  billingType: string;
  model: string;
  costCents: number;
};

export type CostSummary = {
  companyId: string;
  spendCents: number;
  budgetCents: number;
  utilizationPercent: number;
};

/** ofetch-style request options for mutating calls. */
export type FetchOptions = { method?: string; body?: unknown };

/**
 * HTTP boundary — in the Nuxt app this is `$fetch` (ofetch). Reads pass just a
 * URL; mutations pass options (method, body).
 */
export type Fetcher = (url: string, options?: FetchOptions) => Promise<unknown>;

/** Reports a cost event (`POST /api/companies/:companyId/cost-events`). */
export async function reportCostEvent(
  fetcher: Fetcher,
  companyId: string,
  input: {
    agentId: string;
    provider: string;
    model: string;
    costCents: number;
    occurredAt: string;
    biller?: string;
    billingType?: string;
    issueId?: string;
    inputTokens?: number;
    outputTokens?: number;
  },
): Promise<CostEvent> {
  const data = await fetcher(`/api/companies/${companyId}/cost-events`, {
    method: "POST",
    body: input,
  });
  return data as CostEvent;
}

/** Loads the spend-vs-budget summary (`GET /api/companies/:companyId/costs/summary`). */
export async function fetchCostSummary(
  fetcher: Fetcher,
  companyId: string,
): Promise<CostSummary> {
  const data = await fetcher(`/api/companies/${companyId}/costs/summary`);
  return data as CostSummary;
}
