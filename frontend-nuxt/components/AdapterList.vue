<template>
  <ul v-if="adapters.length" class="adapter-list">
    <li
      v-for="adapter in adapters"
      :key="adapter.id"
      class="adapter"
      :data-enabled="String(adapter.enabled)"
    >
      {{ adapter.adapterType }} — {{ adapter.enabled ? "enabled" : "disabled" }}
    </li>
  </ul>
  <p v-else class="empty">No adapters yet</p>
</template>

<script setup lang="ts">
// Renders a company's configured adapters (GET /api/companies/:companyId/adapters).
interface CompanyAdapter {
  id: string;
  adapterType: string;
  enabled: boolean;
}
defineProps<{ adapters: CompanyAdapter[] }>();
</script>
