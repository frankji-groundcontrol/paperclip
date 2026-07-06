# Acceptance Harnesses

Acceptance scripts in this directory are the final gate. They must run against the Supabase-backed Rust/Nuxt rewrite and prove parity with original Paperclip V1.

Required final scripts:

- `run_original_gap_diff.py` — compares implemented surface against `evidence/rewrite-gap-matrix.md`; final result must be zero missing/partial/divergent.
- `run_real_user_real_agent.py` — real authenticated users, real company, real hired/created agent, real OpenAI Responses run, lifecycle/status/activity/cost assertions.
- `run_cli_mcp_surface.py` — CLI and MCP parity smoke.
- `run_frontend_user_flow.py` — browser or component-driven smoke for human board flow.

Scripts should never require service-role keys and must never expose Supabase/OpenAI internals to client-facing surfaces.
