<template>
  <ul v-if="routines.length" class="routine-list">
    <li
      v-for="routine in routines"
      :key="routine.id"
      class="routine"
      :data-status="routine.status"
    >
      {{ routine.title }} — {{ routine.status }}
    </li>
  </ul>
  <p v-else class="empty">No routines yet</p>
</template>

<script setup lang="ts">
// Renders a company's routines (GET /api/companies/:companyId/routines), with an
// empty state.
interface Routine {
  id: string;
  title: string;
  status: string;
}
defineProps<{ routines: Routine[] }>();
</script>
