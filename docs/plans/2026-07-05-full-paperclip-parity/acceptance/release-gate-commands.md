# Release Gate Commands

## Local deterministic gate

```sh
cargo test --manifest-path backend-rs/Cargo.toml
cd frontend-nuxt && npx vitest run
python3 docs/plans/2026-07-05-full-paperclip-parity/acceptance/run_original_gap_diff.py --mode local
```

## Supabase branch gate

```sh
python3 docs/plans/2026-07-05-full-paperclip-parity/acceptance/run_original_gap_diff.py --mode supabase-branch
```

## Final `supabase-franky` + real OpenAI gate

```sh
python3 docs/plans/2026-07-05-full-paperclip-parity/acceptance/run_real_user_real_agent.py --live-openai
python3 docs/plans/2026-07-05-full-paperclip-parity/acceptance/run_cli_mcp_surface.py
python3 docs/plans/2026-07-05-full-paperclip-parity/acceptance/run_frontend_user_flow.py
```

## Required final line

```text
PASS: gaps_missing=0 gaps_partial=0 gaps_divergent=0 no_secret_leaks=0
```
