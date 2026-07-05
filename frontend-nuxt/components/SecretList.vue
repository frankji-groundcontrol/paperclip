<template>
  <ul v-if="secrets.length" class="secret-list">
    <li
      v-for="secret in secrets"
      :key="secret.id"
      class="secret"
      :data-provider="secret.provider"
    >
      {{ secret.name }} ({{ secret.provider }})
    </li>
  </ul>
  <p v-else class="empty">No secrets yet</p>
</template>

<script setup lang="ts">
// Renders a company's secret references (GET /api/companies/:companyId/secrets).
// Only name/provider are ever shown — values never leave the server.
interface SecretRef {
  id: string;
  name: string;
  provider: string;
}
defineProps<{ secrets: SecretRef[] }>();
</script>
