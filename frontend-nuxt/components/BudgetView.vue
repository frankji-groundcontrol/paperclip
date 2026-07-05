<template>
  <div class="budget" :data-exceeded="String(exceeded)">
    <span class="budget__amounts">{{ spentCents }} / {{ monthlyLimitCents }}</span>
    <span class="budget__state">{{ exceeded ? "Over budget" : "Within budget" }}</span>
  </div>
</template>

<script setup lang="ts">
// Mirrors the budget hard-stop: spend against the monthly limit, and whether the
// company is over budget (agents auto-pause).
defineProps<{ monthlyLimitCents: number; spentCents: number; exceeded: boolean }>();
</script>
