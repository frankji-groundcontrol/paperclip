<template>
  <ul v-if="environments.length" class="environment-list">
    <li
      v-for="env in environments"
      :key="env.id"
      class="environment"
      :data-driver="env.driver"
      :data-status="env.status"
    >
      <span class="environment__name">{{ env.name }}</span>
      <span class="environment__meta">{{ env.driver }} · {{ env.status }}</span>
    </li>
  </ul>
  <p v-else class="empty">No environments yet</p>
</template>

<script setup lang="ts">
// Renders a company's environments (GET /api/companies/:companyId/environments).
interface Environment {
  id: string;
  name: string;
  driver: string;
  status: string;
}
defineProps<{ environments: Environment[] }>();
</script>
