<template>
  <ul v-if="approvals.length" class="approval-list">
    <li
      v-for="approval in approvals"
      :key="approval.id"
      class="approval"
      :data-status="approval.status"
    >
      {{ approval.issueId }} — {{ approval.status }}
    </li>
  </ul>
  <p v-else class="empty">No approvals yet</p>
</template>

<script setup lang="ts">
// Renders a company's approvals (GET /api/companies/:companyId/approvals), with
// an empty state.
interface Approval {
  id: string;
  issueId: string;
  status: string;
}
defineProps<{ approvals: Approval[] }>();
</script>
