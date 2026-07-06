# Phase 13 — Import/Export, Catalogs, Templates, and Portability

## Goal

Preserve current catalog/template portability surfaces that affect real company setup and reuse.

## Source evidence

- `packages/teams-catalog/*`
- `packages/skills-catalog/*`
- company export/import routes/services where current implementation exposes them.
- onboarding assets and generated catalogs.

## Tasks

1. RED: catalog validation and company export/import round-trip tests.
2. Implement supported export preview/apply/import surfaces with secret redaction.
3. Implement team/skill catalog validation paths and source policy.
4. Add public docs/internal docs updates for any changed commands.

## Acceptance

Export/import round trip preserves companies, agents, goals, projects, issues, docs, skills, routines/plugins where supported, with no secrets or key hashes in bundles.
