<template>
  <main class="paperclip-page">
    <h1>Paperclip</h1>
    <p class="tagline">Form a company. Run a job. Get real work done.</p>
    <PaperclipConsole :fetcher="fetcher" />
  </main>
</template>

<script setup lang="ts">
// Composition root for non-agent users: binds the real HTTP client and renders
// the console. The browser holds only the opaque `pcs_` session.
import { useApi } from "../composables/useApi";
import PaperclipConsole from "../components/PaperclipConsole.vue";

const fetcher = useApi();
</script>
