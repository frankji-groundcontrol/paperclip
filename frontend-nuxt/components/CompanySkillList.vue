<template>
  <ul v-if="skills.length" class="skill-list">
    <li
      v-for="skill in skills"
      :key="skill.id"
      class="skill"
      :data-stars="String(skill.starCount)"
    >
      <span class="skill__name">{{ skill.name }}</span>
      <span class="skill__stars">★ {{ skill.starCount }}</span>
    </li>
  </ul>
  <p v-else class="empty">No skills yet</p>
</template>

<script setup lang="ts">
// Renders a company's skills (GET /api/companies/:companyId/skills), with an
// empty state. Mirrors the backend company-skills domain.
interface CompanySkill {
  id: string;
  key: string;
  name: string;
  starCount: number;
}
defineProps<{ skills: CompanySkill[] }>();
</script>
