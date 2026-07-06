# Phase 14 — Observability, Feedback, Productivity, and Pipelines

## Goal

Classify and implement current-addendum observability/productivity/pipeline surfaces or waive them explicitly.

## Source evidence

- `server/src/routes/*feedback*`, productivity review routes/services.
- pipeline case migrations/routes/services.
- dashboard/health/doctor/release smoke outputs.

## Tasks

1. Classify every observability/feedback/pipeline row as v1-core/current-addendum/post-v1/not-in-v1.
2. Implement blocking current-addendum rows with API/UI tests.
3. Add release/e2e smoke result ingestion if required by current contract.
4. Add waivers for explicit post-v1 decisions only after product doc update.

## Acceptance

No unclassified current implementation surface remains in the parity matrix.
