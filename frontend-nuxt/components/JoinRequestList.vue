<template>
  <ul v-if="requests.length" class="join-request-list">
    <li
      v-for="request in requests"
      :key="request.id"
      class="join-request"
      :data-type="request.requestType"
      :data-status="request.status"
    >
      <span class="join-request__who">{{ request.requesterName ?? request.requestType }}</span>
      <span class="join-request__status">{{ request.status }}</span>
    </li>
  </ul>
  <p v-else class="empty">No join requests</p>
</template>

<script setup lang="ts">
// Renders a company's join-requests (GET /api/companies/:companyId/join-requests).
interface JoinRequest {
  id: string;
  requestType: string;
  requesterName?: string | null;
  status: string;
}
defineProps<{ requests: JoinRequest[] }>();
</script>
