<template>
  <div class="dashboard">
    <section class="dashboard__group" data-group="agents">
      <h3>Agents</h3>
      <span data-metric="agents.active">{{ dashboard.agents.active }}</span>
      <span data-metric="agents.running">{{ dashboard.agents.running }}</span>
      <span data-metric="agents.paused">{{ dashboard.agents.paused }}</span>
      <span data-metric="agents.error">{{ dashboard.agents.error }}</span>
    </section>
    <section class="dashboard__group" data-group="tasks">
      <h3>Tasks</h3>
      <span data-metric="tasks.open">{{ dashboard.tasks.open }}</span>
      <span data-metric="tasks.inProgress">{{ dashboard.tasks.inProgress }}</span>
      <span data-metric="tasks.blocked">{{ dashboard.tasks.blocked }}</span>
      <span data-metric="tasks.done">{{ dashboard.tasks.done }}</span>
    </section>
    <section class="dashboard__group" data-group="approvals">
      <h3>Approvals</h3>
      <span data-metric="approvals.pending">{{ dashboard.approvals.pending }}</span>
    </section>
  </div>
</template>

<script setup lang="ts">
// Renders the cross-domain dashboard rollup (GET /api/companies/:companyId/dashboard).
interface Dashboard {
  agents: { active: number; running: number; paused: number; error: number };
  tasks: { open: number; inProgress: number; blocked: number; done: number };
  approvals: { pending: number };
}
defineProps<{ dashboard: Dashboard }>();
</script>
