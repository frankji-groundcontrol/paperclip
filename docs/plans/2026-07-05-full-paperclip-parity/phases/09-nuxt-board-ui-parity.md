# Phase 12 — Nuxt Board UI Parity

## Goal

Replace original React/Vite board surfaces with Nuxt surfaces that expose equivalent user workflows.

## Source evidence

- `ui/src/pages/NewAgent.tsx`
- `ui/src/pages/Agents.tsx`
- `ui/src/pages/AgentDetail.tsx`
- `ui/src/pages/ApprovalDetail.tsx`
- `ui/src/pages/CompanySettings.tsx`
- `ui/src/pages/BoardChat.tsx`
- dashboard, issues, goals, projects, costs, activity, settings pages.

## Tasks

1. RED: UI route inventory diff against original.
2. Implement company selector/dashboard, org chart, agent new/detail/actions, approvals inbox/detail/revision/comments.
3. Implement goals/projects/issues/comments/documents/work products.
4. Implement runtime/live runs/logs, budgets/costs, activity log, access/member/invite/join settings.
5. Add no-leak tests for Supabase URL/anon key/OpenAI secrets/provider keys.

## Acceptance

A non-agent board user can run the full V1 workflow in the browser without knowing Supabase, OpenAI keys, CLI, or MCP.
