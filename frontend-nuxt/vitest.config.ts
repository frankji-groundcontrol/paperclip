import { defineConfig } from "vitest/config";
import vue from "@vitejs/plugin-vue";

export default defineConfig({
  plugins: [vue()],
  // Use an inline empty tsconfig so Vite's tsconfig resolver does not walk up
  // into the surrounding monorepo (whose package tsconfigs are unrelated).
  esbuild: { tsconfigRaw: "{}" },
  test: {
    environment: "happy-dom",
  },
});
