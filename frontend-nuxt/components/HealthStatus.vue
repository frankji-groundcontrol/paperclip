<template>
  <span class="health-status" :data-status="status">{{ label }}</span>
</template>

<script setup lang="ts">
import { computed } from "vue";

const props = defineProps<{ status: string }>();

// Maps the GET /api/health `status` field to a human label.
const label = computed(() => (props.status === "ok" ? "Healthy" : "Unhealthy"));
</script>
