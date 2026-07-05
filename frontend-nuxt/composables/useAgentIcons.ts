/** HTTP boundary — in the Nuxt app this is `$fetch` (ofetch). Returns text. */
export type Fetcher = (url: string) => Promise<unknown>;

/**
 * Loads the agent icon names from the llms text endpoint
 * (`GET /llms/agent-icons.txt`) and parses the `- name` bullet lines into an
 * array — handy for an icon picker on the hire/create form.
 */
export async function fetchAgentIcons(fetcher: Fetcher): Promise<string[]> {
  const text = (await fetcher("/llms/agent-icons.txt")) as string;
  return text
    .split("\n")
    .filter((line) => line.startsWith("- "))
    .map((line) => line.slice(2).trim());
}
