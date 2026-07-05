<template>
  <ul v-if="companies.length" class="company-list">
    <li
      v-for="company in companies"
      :key="company.id"
      class="company"
      :data-id="company.id"
    >
      {{ company.name }}
    </li>
  </ul>
  <p v-else class="empty">No companies yet</p>
</template>

<script setup lang="ts">
// Renders companies from GET /api/companies, with an empty state.
interface Company {
  id: string;
  name: string;
}
defineProps<{ companies: Company[] }>();
</script>
