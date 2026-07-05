<template>
  <ul v-if="events.length" class="activity-list">
    <li
      v-for="event in events"
      :key="event.id"
      class="activity"
      :data-entity="event.entityType"
    >
      <span class="activity__action">{{ event.action }}</span>
      <span class="activity__entity">{{ event.entityType }}</span>
    </li>
  </ul>
  <p v-else class="empty">No activity yet</p>
</template>

<script setup lang="ts">
// Renders a company's activity feed (GET /api/companies/:companyId/activity).
interface ActivityEvent {
  id: string;
  action: string;
  entityType: string;
}
defineProps<{ events: ActivityEvent[] }>();
</script>
