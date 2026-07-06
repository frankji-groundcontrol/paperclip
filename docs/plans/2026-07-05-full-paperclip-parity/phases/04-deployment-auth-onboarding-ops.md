# Phase 07 — Deployment, Auth, Onboarding, and Ops Parity

## Goal

Preserve deployment/auth/onboarding behavior while moving to Supabase-backed rewrite.

## Source evidence

- `doc/DEPLOYMENT-MODES.md`
- `doc/DEVELOPING.md`
- `doc/DATABASE.md`
- `server/src/middleware/auth.ts`
- `server/src/routes/auth*`, setup/claim/admin routes.
- Docker and smoke docs/scripts.

## Tasks

1. RED: local_trusted, authenticated/private, authenticated/public mode behavior.
2. Preserve first-admin/bootstrap/claim flows without exposing Supabase implementation.
3. Preserve device/login/session broker behavior with opaque `pcs_` client token.
4. Ensure public authenticated deployments refuse embedded DB fallback.
5. Add doctor/onboard/bootstrap acceptance for CLI and browser.
6. Add backup/storage/config/admin surfaces required by current product docs.

## Acceptance

Fresh local and authenticated instances pass onboarding/doctor/bootstrap flows and no public deployment path uses unsafe embedded DB or service-role shortcuts.
