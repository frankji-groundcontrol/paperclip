# Phase 06 — Workspaces, Routines, Plugins, and Sandbox Boundaries

## Goal

Restore optional but current Paperclip runtime surfaces that affect real agent execution and low-trust containment.

## Source evidence

- `server/src/routes/execution-workspaces.ts`
- `server/src/services/workspace-runtime.ts`
- `server/src/routes/routines.ts`
- `server/src/services/routines.ts`
- `server/src/routes/plugins.ts`
- `packages/plugins/*`
- `doc/plugins/*`

## Tasks

1. RED: workspace policy, runtime service, routine dispatch, plugin manifest/sandbox tests.
2. Implement minimal workspace/runtime tables and broker routes needed by agent execution.
3. Implement routine schedule/webhook/API dispatch parity where current V1 exposes it.
4. Implement plugin namespace/manifest/trust boundaries and sandbox provider hooks where parity matrix marks current-addendum.
5. Add low-trust and no-secret scanner coverage for plugin/runtime logs.

## Acceptance

Fixture plugin and fixture runtime service operate within company boundaries, and copied/live work is quarantined in test worktree scenarios.
