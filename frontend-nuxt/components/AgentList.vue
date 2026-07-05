<template>
  <ul v-if="agents.length" class="agent-list">
    <li
      v-for="agent in agents"
      :key="agent.id"
      class="agent"
      :data-status="agent.status"
    >
      {{ agent.name }} — {{ agent.role }}
    </li>
  </ul>
  <p v-else class="empty">No agents yet</p>
</template>

<script setup lang="ts">
// Renders a company's agents (GET /api/companies/:companyId/agents), with an
// empty state.
interface Agent {
  id: string;
  name: string;
  role: string;
  status: string;
}
defineProps<{ agents: Agent[] }>();
</script>
