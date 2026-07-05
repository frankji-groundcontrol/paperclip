<template>
  <ul v-if="goals.length" class="goal-list">
    <li
      v-for="goal in goals"
      :key="goal.id"
      class="goal"
      :data-level="goal.level"
      :data-status="goal.status"
    >
      <span class="goal__title">{{ goal.title }}</span>
      <span class="goal__meta">{{ goal.level }} · {{ goal.status }}</span>
    </li>
  </ul>
  <p v-else class="empty">No goals yet</p>
</template>

<script setup lang="ts">
// Renders a company's goals (GET /api/companies/:companyId/goals), with an
// empty state. Mirrors the backend goals domain (backend-rs/src/goals.rs).
interface Goal {
  id: string;
  title: string;
  level: string;
  status: string;
}
defineProps<{ goals: Goal[] }>();
</script>
