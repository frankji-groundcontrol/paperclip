<template>
  <ul v-if="workspaces.length" class="workspace-list">
    <li
      v-for="workspace in workspaces"
      :key="workspace.id"
      class="workspace"
      :data-status="workspace.status"
    >
      {{ workspace.issueId }} — {{ workspace.status }}
    </li>
  </ul>
  <p v-else class="empty">No workspaces yet</p>
</template>

<script setup lang="ts">
// Renders a company's execution workspaces
// (GET /api/companies/:companyId/workspaces), with an empty state.
interface Workspace {
  id: string;
  issueId: string;
  status: string;
}
defineProps<{ workspaces: Workspace[] }>();
</script>
