// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
  compatibilityDate: "2026-07-01",
  devtools: { enabled: false },

  // Same-origin API base by default; the domain composables call `/api/**` and
  // `useApi()` (composables/useApi.ts) binds `$fetch` to this. Override in
  // production with NUXT_PUBLIC_API_BASE to point at the Rust backend directly.
  runtimeConfig: {
    public: {
      apiBase: "",
    },
  },

  // In `nuxt dev`, forward the Rust control-plane API (default port 3100, see
  // backend-rs/src/main.rs) so the tested composables drive a live backend
  // without CORS. Override the target with PAPERCLIP_API_TARGET.
  $development: {
    nitro: {
      devProxy: {
        "/api": {
          target: process.env.PAPERCLIP_API_TARGET ?? "http://localhost:3100/api",
          changeOrigin: true,
        },
      },
    },
  },
});
