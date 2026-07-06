# Phase 15 — Shared Contracts and Docs Parity

## Goal

Keep cross-layer contracts synchronized while moving from TypeScript original to Rust/Nuxt rewrite.

## Tasks

1. Snapshot constants, validators, API paths, route schemas, CLI help, MCP schemas.
2. Implement generated/shared contract artifacts or documented Rust equivalents.
3. Update internal docs under `doc/**` and public docs under `docs/**` only when behavior changes.
4. Keep `docs/docs.json` navigation current for public docs changes.
5. Add link/no-secret checks for docs.

## Acceptance

No behavior/API/command change lacks a contract/doc row; no public docs expose internal IDs, secrets, local run IDs, tailnet URLs, or agent links.
