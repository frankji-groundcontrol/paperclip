<template>
  <span class="actor-badge" :data-actor="actor">{{ label }}</span>
</template>

<script setup lang="ts">
import { computed } from "vue";

const props = defineProps<{ actor: "board" | "agent"; companyId?: string }>();

// Mirrors the /api/whoami contract: the board operator, or an agent scoped to a
// company.
const label = computed(() =>
  props.actor === "agent" ? `Agent · ${props.companyId ?? ""}` : "Board operator",
);
</script>
