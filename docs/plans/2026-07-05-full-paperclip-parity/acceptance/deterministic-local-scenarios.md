# Deterministic Local Scenarios

Local tests are the daily worker gate. Live Supabase/OpenAI proves integration only after deterministic tests pass.

## Required fixtures

- Fake OpenAI Responses SSE server.
- Fake adapter that can succeed, fail, hang, and cancel.
- Two-company adversarial data set.
- Fixture plugin and fixture workspace repo.
- Secret sentinel values for redaction scanner.

## Required scenario families

1. Schema/authz matrix with board/user/agent/viewer/low-trust/cross-company actors.
2. Agent lifecycle table: direct create, approval required, approve, reject, pause, resume, terminate, clear error, wakeup, cancel.
3. Task/work-product flow: assign, checkout, run, comment, attach artifact, complete.
4. Budget/cost flow: cost event, alert, hard stop, override approval.
5. Secret/config flow: read redaction, activity redaction, export redaction.
6. Interface inventory: REST, CLI, MCP, Nuxt route/component smoke.
