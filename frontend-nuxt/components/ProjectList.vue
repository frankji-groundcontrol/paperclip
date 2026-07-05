<template>
  <ul v-if="projects.length" class="project-list">
    <li
      v-for="project in projects"
      :key="project.id"
      class="project"
      :data-status="project.status"
    >
      {{ project.name }}
    </li>
  </ul>
  <p v-else class="empty">No projects yet</p>
</template>

<script setup lang="ts">
// Renders a company's projects (GET /api/companies/:companyId/projects), with an
// empty state.
interface Project {
  id: string;
  name: string;
  status: string;
}
defineProps<{ projects: Project[] }>();
</script>
