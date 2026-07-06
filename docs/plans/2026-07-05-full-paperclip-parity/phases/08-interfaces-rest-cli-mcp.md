# Phase 11 — REST, CLI, and MCP Interface Parity

## Goal

Map implemented domain capabilities to external interfaces without shrinking behavior.

## Tasks

1. Generate route inventory from original Express routes and rewrite Rust routes.
2. Generate CLI command inventory from original `cli/src/commands/client/*` and rewrite CLI.
3. Generate MCP tools-list inventory.
4. Add missing HTTP/CLI/MCP surfaces after domain APIs exist.
5. Add consistent error mapping: 400/401/403/404/409/422/500.
6. Run no-secret scanner over HTTP/CLI/MCP output.

## Acceptance

Every original route/command/tool row is full or has an approved waiver. Agent keys cannot perform board-only operations.
