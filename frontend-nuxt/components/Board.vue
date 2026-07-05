<template>
  <main class="board">
    <header class="board__header">
      <h1 class="board__title">Paperclip</h1>
      <HealthStatus :status="health.status" />
      <ActorBadge :actor="actor.actor" :company-id="actor.companyId" />
    </header>

    <section class="board__section" data-section="companies">
      <h2>Companies</h2>
      <CompanyList :companies="companies" />
    </section>

    <section class="board__section" data-section="issues">
      <h2>Issues</h2>
      <IssueList :issues="issues" />
    </section>

    <section class="board__section" data-section="agents">
      <h2>Agents</h2>
      <AgentList :agents="agents" />
    </section>

    <section class="board__section" data-section="runs">
      <h2>Runs</h2>
      <RunList :runs="runs" />
    </section>
  </main>
</template>

<script setup lang="ts">
import HealthStatus from "./HealthStatus.vue";
import ActorBadge from "./ActorBadge.vue";
import CompanyList from "./CompanyList.vue";
import IssueList from "./IssueList.vue";
import AgentList from "./AgentList.vue";
import RunList from "./RunList.vue";

// The board shell composes the tested pieces into a single page. Data is passed
// in as props; the running app supplies it from the fetch composables.
interface Company {
  id: string;
  name: string;
}
interface Issue {
  id: string;
  title: string;
  status: string;
}
interface Agent {
  id: string;
  name: string;
  role: string;
  status: string;
}
interface Run {
  id: string;
  agentId: string;
  status: string;
}

defineProps<{
  health: { status: string };
  actor: { actor: "board" | "agent"; companyId?: string };
  companies: Company[];
  issues: Issue[];
  agents: Agent[];
  runs: Run[];
}>();
</script>
