export type Budget = {
  companyId: string;
  monthlyLimitCents: number;
  spentCents: number;
  exceeded: boolean;
};

/** ofetch-style request options for mutating calls. */
export type FetchOptions = { method?: string; body?: unknown };

/**
 * HTTP boundary — in the Nuxt app this is `$fetch` (ofetch). Reads pass just a
 * URL; mutations pass options (method, body).
 */
export type Fetcher = (url: string, options?: FetchOptions) => Promise<unknown>;

/** Loads a company's budget (`GET /api/companies/:companyId/budget`). */
export async function fetchBudget(fetcher: Fetcher, companyId: string): Promise<Budget> {
  const data = await fetcher(`/api/companies/${companyId}/budget`);
  return data as Budget;
}

/** Sets the monthly limit (`POST /api/companies/:companyId/budget/limit`). */
export async function setBudgetLimit(
  fetcher: Fetcher,
  companyId: string,
  monthlyLimitCents: number,
): Promise<Budget> {
  const data = await fetcher(`/api/companies/${companyId}/budget/limit`, {
    method: "POST",
    body: { monthlyLimitCents },
  });
  return data as Budget;
}

/**
 * Records spend (`POST /api/companies/:companyId/budget/spend`). The returned
 * budget reflects the hard-stop (`exceeded`) invariant.
 */
export async function recordBudgetSpend(
  fetcher: Fetcher,
  companyId: string,
  amountCents: number,
): Promise<Budget> {
  const data = await fetcher(`/api/companies/${companyId}/budget/spend`, {
    method: "POST",
    body: { amountCents },
  });
  return data as Budget;
}
