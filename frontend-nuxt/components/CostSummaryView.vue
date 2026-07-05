<template>
  <div class="cost-summary" :data-over-budget="String(summary.utilizationPercent >= 100)">
    <span data-field="spend">{{ dollars(summary.spendCents) }}</span>
    <span data-field="budget">{{ dollars(summary.budgetCents) }}</span>
    <span data-field="utilization">{{ summary.utilizationPercent }}%</span>
  </div>
</template>

<script setup lang="ts">
// Renders the cost summary (GET /api/companies/:companyId/costs/summary): spend
// vs monthly budget with utilization, flagging over-budget at/over 100%.
interface CostSummary {
  companyId: string;
  spendCents: number;
  budgetCents: number;
  utilizationPercent: number;
}
defineProps<{ summary: CostSummary }>();

function dollars(cents: number): string {
  return `$${(cents / 100).toFixed(2)}`;
}
</script>
