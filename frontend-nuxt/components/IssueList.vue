<template>
  <ul v-if="issues.length" class="issue-list">
    <li
      v-for="issue in issues"
      :key="issue.id"
      class="issue"
      :data-status="issue.status"
    >
      {{ issue.title }}
    </li>
  </ul>
  <p v-else class="empty">No issues yet</p>
</template>

<script setup lang="ts">
// Renders a company's issues (GET /api/companies/:companyId/issues), with an
// empty state.
interface Issue {
  id: string;
  title: string;
  status: string;
}
defineProps<{ issues: Issue[] }>();
</script>
