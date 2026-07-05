<template>
  <ul v-if="runs.length" class="run-list">
    <li v-for="run in runs" :key="run.id" class="run" :data-status="run.status">
      {{ run.agentId }} — {{ run.status }}
    </li>
  </ul>
  <p v-else class="empty">No runs yet</p>
</template>

<script setup lang="ts">
// Renders a company's runs (GET /api/companies/:companyId/runs), with an empty
// state.
interface Run {
  id: string;
  agentId: string;
  status: string;
}
defineProps<{ runs: Run[] }>();
</script>
