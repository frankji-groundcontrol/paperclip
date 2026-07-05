<template>
  <section class="paperclip-console">
    <!-- Logged out: email/password login -->
    <form v-if="!session" class="login" data-testid="login-form" @submit.prevent="onLogin">
      <h2>Sign in to Paperclip</h2>
      <input v-model="email" data-testid="email" type="email" placeholder="you@example.com" />
      <input v-model="password" data-testid="password" type="password" placeholder="password" />
      <button data-testid="login-submit" type="submit">Sign in</button>
      <p v-if="error" class="error" data-testid="error">{{ error }}</p>
    </form>

    <!-- Logged in: companies + job console -->
    <div v-else class="workspace">
      <header class="bar">
        <span data-testid="signed-in">Signed in</span>
        <button data-testid="logout" @click="onLogout">Sign out</button>
      </header>

      <div class="companies">
        <h3>Companies</h3>
        <form class="new-company" data-testid="new-company" @submit.prevent="onCreateCompany">
          <input v-model="newCompanyName" data-testid="company-name" placeholder="New company name" />
          <button data-testid="create-company" type="submit">Create</button>
        </form>
        <ul>
          <li
            v-for="c in companies"
            :key="c.id"
            class="company"
            :data-id="c.id"
            :class="{ selected: c.id === selectedId }"
          >
            <button data-testid="select-company" @click="selectCompany(c.id)">{{ c.name }}</button>
          </li>
        </ul>
      </div>

      <div v-if="selectedId" class="job-console" data-testid="job-console">
        <h3>Run a job</h3>
        <textarea v-model="prompt" data-testid="prompt" placeholder="Describe the task…"></textarea>
        <button data-testid="run-job" :disabled="running" @click="onRunJob">
          {{ running ? "Running…" : "Run" }}
        </button>
        <div v-if="lastResult" class="result" data-testid="result">
          <strong>Result:</strong> <span data-testid="result-text">{{ lastResult.result }}</span>
          <em v-if="lastResult.usage?.total_tokens" data-testid="usage">
            ({{ lastResult.usage.total_tokens }} tokens)
          </em>
        </div>
        <p v-if="error" class="error" data-testid="error">{{ error }}</p>
      </div>

      <!-- Staffing: hire agents (employees) + board approvals -->
      <div v-if="selectedId" class="staffing" data-testid="staffing">
        <h3>Agents</h3>
        <form class="hire" data-testid="hire-form" @submit.prevent="onHireAgent">
          <input v-model="newAgentName" data-testid="agent-name" placeholder="Hire an agent (name)" />
          <button data-testid="hire-agent" type="submit">Hire</button>
        </form>
        <ul>
          <li v-for="a in agents" :key="a.id" class="agent" :data-id="a.id">
            {{ a.name }} — <span class="agent-status" data-testid="agent-status">{{ a.status }}</span>
          </li>
        </ul>

        <h3>Approvals</h3>
        <ul>
          <li
            v-for="ap in approvals.filter((x) => x.status === 'pending')"
            :key="ap.id"
            class="approval"
            :data-id="ap.id"
          >
            {{ ap.type }}
            <button data-testid="approve" @click="onDecide(ap.id, true)">Approve</button>
            <button data-testid="reject" @click="onDecide(ap.id, false)">Reject</button>
          </li>
          <li v-if="!approvals.some((x) => x.status === 'pending')" class="no-approvals" data-testid="no-approvals">
            No pending approvals
          </li>
        </ul>
      </div>
    </div>
  </section>
</template>

<script setup lang="ts">
import { ref } from "vue";
import type { Fetcher } from "../composables/useApi";
import {
  login,
  createCompany,
  listCompanies,
  runJob,
  hireAgent,
  listAgents,
  listApprovals,
  decideApproval,
  type Company,
  type JobResult,
  type Agent,
  type Approval,
} from "../composables/usePaperclipSession";

// The HTTP boundary is injected so the component is testable with a mock fetcher.
const props = defineProps<{ fetcher: Fetcher }>();

const session = ref<string | null>(null);
const email = ref("");
const password = ref("");
const error = ref<string | null>(null);

const companies = ref<Company[]>([]);
const newCompanyName = ref("");
const selectedId = ref<string | null>(null);
const prompt = ref("");
const running = ref(false);
const lastResult = ref<JobResult | null>(null);

const agents = ref<Agent[]>([]);
const approvals = ref<Approval[]>([]);
const newAgentName = ref("");

async function onLogin() {
  error.value = null;
  try {
    const s = await login(props.fetcher, email.value, password.value);
    session.value = s.token;
    await refreshCompanies();
  } catch (e) {
    error.value = "Sign in failed";
  }
}

function onLogout() {
  session.value = null;
  companies.value = [];
  selectedId.value = null;
  lastResult.value = null;
}

async function refreshCompanies() {
  if (!session.value) return;
  companies.value = await listCompanies(props.fetcher, session.value);
}

async function onCreateCompany() {
  if (!session.value || !newCompanyName.value) return;
  error.value = null;
  try {
    const id = await createCompany(props.fetcher, session.value, newCompanyName.value);
    newCompanyName.value = "";
    await refreshCompanies();
    selectCompany(id);
  } catch (e) {
    error.value = "Could not create company";
  }
}

function selectCompany(id: string) {
  selectedId.value = id;
  lastResult.value = null;
  void refreshHiring();
}

async function refreshHiring() {
  if (!session.value || !selectedId.value) return;
  agents.value = await listAgents(props.fetcher, session.value, selectedId.value);
  approvals.value = await listApprovals(props.fetcher, session.value, selectedId.value);
}

async function onHireAgent() {
  if (!session.value || !selectedId.value || !newAgentName.value) return;
  error.value = null;
  try {
    await hireAgent(props.fetcher, session.value, selectedId.value, newAgentName.value);
    newAgentName.value = "";
    await refreshHiring();
  } catch (e) {
    error.value = "Could not hire agent";
  }
}

async function onDecide(approvalId: string, approve: boolean) {
  if (!session.value) return;
  error.value = null;
  try {
    await decideApproval(props.fetcher, session.value, approvalId, approve);
    await refreshHiring();
  } catch (e) {
    error.value = "Decision failed (board approval required)";
  }
}

async function onRunJob() {
  if (!session.value || !selectedId.value || !prompt.value) return;
  error.value = null;
  running.value = true;
  try {
    lastResult.value = await runJob(props.fetcher, session.value, selectedId.value, prompt.value);
    if (lastResult.value.status !== "succeeded") error.value = "Job failed";
  } catch (e) {
    error.value = "Job failed";
  } finally {
    running.value = false;
  }
}
</script>
