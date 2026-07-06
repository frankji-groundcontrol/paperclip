# Phase 05 — Goals, Projects, Issues, Comments, Documents, and Work Products

## Goal

Restore the work backbone. Agents do inspectable work tied to company goals, not just one-off prompt jobs.

## Source evidence

- `doc/PRODUCT.md:44-60` task hierarchy.
- `doc/SPEC-implementation.md:186+` goals/projects/issues model.
- `server/src/routes/issues.ts`, `server/src/routes/projects.ts`, `server/src/routes/goals.ts`.
- `doc/AGENT-ARTIFACTS.md` work product expectations.

## Tasks

1. RED: company mission, goal hierarchy, project lead/env, issue tree, comments, documents, work products.
2. Add Supabase schema/RPCs for goals/projects/issues/comments/documents/attachments/work_products.
3. Implement atomic checkout, single assignee, blockers, parent/sub-issue invariants.
4. Wire runtime runs to assigned issues and work products.
5. Add UI/CLI/MCP surfaces for task read/write and artifact/work-product flows.

## Acceptance

A real agent completes a real task that produces a board-inspectable work product and every work item traces to a company goal.
