import { fetchWhoami, type Whoami } from "./useWhoami";
import { fetchCompanies, type Company } from "./useCompanies";
import { fetchIssues, type Issue } from "./useIssues";
import { fetchAgents, type Agent } from "./useAgents";
import { fetchRuns, type Run } from "./useRuns";

export type Health = { status: string; version?: string };

export type BoardData = {
  health: Health;
  actor: Whoami;
  companies: Company[];
  issues: Issue[];
  agents: Agent[];
  runs: Run[];
};

/** HTTP boundary — in the Nuxt app this is `$fetch` (ofetch). */
export type Fetcher = (url: string) => Promise<unknown>;

/**
 * Loads everything the Board shell needs for a company in parallel, returning
 * exactly the props <Board> expects. This is what `app.vue` calls (via
 * `useApi()`), turning the tested composables into a live-data page.
 */
export async function loadBoard(fetcher: Fetcher, companyId: string): Promise<BoardData> {
  const [health, actor, companies, issues, agents, runs] = await Promise.all([
    fetcher("/api/health") as Promise<Health>,
    fetchWhoami(fetcher),
    fetchCompanies(fetcher),
    fetchIssues(fetcher, companyId),
    fetchAgents(fetcher, companyId),
    fetchRuns(fetcher, companyId),
  ]);
  return { health, actor, companies, issues, agents, runs };
}
