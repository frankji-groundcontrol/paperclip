# Phase 16 — Migration Compatibility, Cutover, and Rollback

## Goal

Ship parity safely without losing existing Supabase test data or creating split-brain auth/runtime behavior.

## Tasks

1. Add migration ledger and row-count-before/after checks.
2. Make migrations forward-only and idempotent where possible.
3. Add compatibility views/RPC aliases and feature flags during transition.
4. Define strangler routing policy between old TypeScript and new Rust/Nuxt surfaces.
5. Rehearse rollback without destructive enum/table drops.
6. Run Supabase security/performance advisors after DDL.

## Acceptance

Existing `test_users.json` accounts and real companies/jobs/agents remain readable; migrated rows match canonical parity semantics; rollback plan is tested on branch/project.
