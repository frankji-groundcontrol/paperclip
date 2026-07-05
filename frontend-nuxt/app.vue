<template>
  <Board
    :health="board.health"
    :actor="board.actor"
    :companies="board.companies"
    :issues="board.issues"
    :agents="board.agents"
    :runs="board.runs"
  />
</template>

<script setup lang="ts">
import Board from "~/components/Board.vue";
import { useApi } from "~/composables/useApi";
import { fetchCompanies } from "~/composables/useCompanies";
import { loadBoard, type BoardData } from "~/composables/useBoard";

// Live-data composition root: bind `$fetch` to the API (dev-proxied to the Rust
// backend), pick the first company, and load the Board's data via the tested
// `loadBoard` orchestrator. Falls back to empty state if the backend is
// unreachable so the page always renders. `useApi`/`useAsyncData` are Nuxt
// runtime globals; the data-loading behaviour itself lives in tested composables.
const EMPTY: BoardData = {
  health: { status: "unknown" },
  actor: { actor: "board" },
  companies: [],
  issues: [],
  agents: [],
  runs: [],
};

const api = useApi();

const { data } = await useAsyncData<BoardData>("board", async () => {
  try {
    const companies = await fetchCompanies(api);
    const companyId = companies[0]?.id ?? "";
    return await loadBoard(api, companyId);
  } catch {
    return EMPTY;
  }
});

const board = computed(() => data.value ?? EMPTY);
</script>
