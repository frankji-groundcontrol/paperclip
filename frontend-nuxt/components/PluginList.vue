<template>
  <ul v-if="plugins.length" class="plugin-list">
    <li
      v-for="plugin in plugins"
      :key="plugin.id"
      class="plugin"
      :data-enabled="String(plugin.enabled)"
    >
      {{ plugin.pluginId }} — {{ plugin.enabled ? "enabled" : "disabled" }}
    </li>
  </ul>
  <p v-else class="empty">No plugins yet</p>
</template>

<script setup lang="ts">
// Renders a company's installed plugins (GET /api/companies/:companyId/plugins).
interface CompanyPlugin {
  id: string;
  pluginId: string;
  enabled: boolean;
}
defineProps<{ plugins: CompanyPlugin[] }>();
</script>
