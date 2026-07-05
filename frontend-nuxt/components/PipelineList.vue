<template>
  <ul v-if="pipelines.length" class="pipeline-list">
    <li
      v-for="pipeline in pipelines"
      :key="pipeline.id"
      class="pipeline"
      :data-archived="String(pipeline.archived)"
    >
      <span class="pipeline__name">{{ pipeline.name }}</span>
      <span v-if="pipeline.archived" class="pipeline__badge">archived</span>
    </li>
  </ul>
  <p v-else class="empty">No pipelines yet</p>
</template>

<script setup lang="ts">
// Renders a company's pipelines (GET /api/companies/:companyId/pipelines), with
// an empty state. Mirrors the backend pipelines domain.
interface Pipeline {
  id: string;
  key: string;
  name: string;
  archived: boolean;
}
defineProps<{ pipelines: Pipeline[] }>();
</script>
