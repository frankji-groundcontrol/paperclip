<template>
  <ul v-if="invites.length" class="invite-list">
    <li
      v-for="invite in invites"
      :key="invite.id"
      class="invite"
      :data-state="invite.state"
    >
      <span class="invite__types">{{ invite.allowedJoinTypes }}</span>
      <span class="invite__state">{{ invite.state }}</span>
    </li>
  </ul>
  <p v-else class="empty">No invites yet</p>
</template>

<script setup lang="ts">
// Renders a company's invites (GET /api/companies/:companyId/invites).
interface Invite {
  id: string;
  token: string;
  allowedJoinTypes: string;
  state: string;
}
defineProps<{ invites: Invite[] }>();
</script>
