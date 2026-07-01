# Architecture — MCP server (packages/mcp-server)

## Purpose / Overview

`packages/mcp-server` is the [Model Context Protocol (MCP)](https://modelcontextprotocol.io) server that lets an MCP client (e.g. Claude, Cursor, Codex, or any MCP-capable agent runtime) drive the Paperclip control plane through structured tools. It is deliberately a **thin wrapper over the existing Paperclip REST API** — it does not touch the database, and it does not reimplement business logic. Every tool call becomes an authenticated HTTP request against the Paperclip `/api` surface, and the response JSON is returned verbatim as MCP text content.

Concretely, the package exposes 41 named tools covering the "actor" surface an agent needs while working: reading its own identity and inbox, listing/reading/creating/updating issues, checking issues out and releasing them, adding comments, driving issue thread interactions (suggest tasks, ask questions, request confirmations), managing issue documents and their revisions, inspecting and controlling issue execution-workspace runtime services, and creating/deciding board approvals. A generic escape-hatch tool (`paperclipApiRequest`) covers any `/api` endpoint that does not yet have a dedicated tool.

The published package is `@paperclipai/mcp-server` ([package.json](../../packages/mcp-server/package.json)) and ships a stdio binary `paperclip-mcp-server`.

## Entry points

- **Binary / process entry:** [src/stdio.ts](../../packages/mcp-server/src/stdio.ts) is the `#!/usr/bin/env node` shebang script wired to the `paperclip-mcp-server` bin in `package.json`. It calls `runServer()` and exits non-zero on startup failure. This is what runs under `npx -y @paperclipai/mcp-server` or `node packages/mcp-server/dist/stdio.js`.
- **Server factory:** [src/index.ts](../../packages/mcp-server/src/index.ts) exports:
  - `createPaperclipMcpServer(config?)` — constructs an `McpServer` (`name: "paperclip"`, `version: "0.1.0"`), builds a `PaperclipApiClient`, generates the tool definitions, and registers each one with `server.tool(name, description, schema.shape, execute)`. Returns `{ server, tools, client }`.
  - `runServer(config?)` — builds the server and connects it to a `StdioServerTransport`. This is the only transport (see below).
- **Library export:** `package.json` `exports["."]` points at `src/index.ts` (source) / `dist/index.js` (published), so the factory can be embedded in-process rather than only spawned as a subprocess.

## Key modules & responsibilities

| File | Responsibility |
| --- | --- |
| [src/index.ts](../../packages/mcp-server/src/index.ts) | Wires MCP SDK server + transport + client + tool registration. |
| [src/stdio.ts](../../packages/mcp-server/src/stdio.ts) | Executable entry; boots `runServer()` over stdio. |
| [src/config.ts](../../packages/mcp-server/src/config.ts) | `PaperclipMcpConfig` type and `readConfigFromEnv()`; normalizes the API URL and reads auth/context env vars. |
| [src/client.ts](../../packages/mcp-server/src/client.ts) | `PaperclipApiClient` — the single HTTP surface. Adds auth headers, forwards the run id, resolves default company/agent ids, and throws typed `PaperclipApiError`. |
| [src/tools.ts](../../packages/mcp-server/src/tools.ts) | All tool definitions: Zod input schemas + `execute` handlers that map inputs to REST calls. Also the `makeTool` factory and the workspace-runtime helpers. |
| [src/format.ts](../../packages/mcp-server/src/format.ts) | Converts handler results / errors into the MCP `{ content: [{ type: "text", text }] }` shape. |
| [src/tools.test.ts](../../packages/mcp-server/src/tools.test.ts) | Vitest coverage of header injection, default id resolution, interaction payloads, and escape-hatch path validation. |

### Configuration ([src/config.ts](../../packages/mcp-server/src/config.ts))

`readConfigFromEnv()` reads five environment variables and returns a `PaperclipMcpConfig`:

- `PAPERCLIP_API_URL` (**required**) — base URL of the Paperclip control plane. `normalizeApiUrl()` strips trailing slashes and appends `/api` if not already present, so both `https://<host>` and `https://<host>/api` work.
- `PAPERCLIP_API_KEY` (**required**) — bearer token sent on every request.
- `PAPERCLIP_COMPANY_ID` (optional) — default company for company-scoped tools.
- `PAPERCLIP_AGENT_ID` (optional) — default agent for checkout helpers.
- `PAPERCLIP_RUN_ID` (optional) — run id forwarded on mutating requests.

Missing `PAPERCLIP_API_URL` or `PAPERCLIP_API_KEY` throws at startup; the other three are nullable and only enforced lazily when a tool actually needs them.

### API client ([src/client.ts](../../packages/mcp-server/src/client.ts))

`PaperclipApiClient.requestJson(method, path, options)` is the choke point for all outbound traffic:

- Requires `path` to start with `/`; builds the final URL against `config.apiUrl` (already `/api`-suffixed).
- Always sends `Authorization: Bearer <apiKey>` and `Accept: application/json`. Adds `Content-Type: application/json` only when a body is present.
- Adds `X-Paperclip-Run-Id: <runId>` when `options.includeRunId` is set, or by default whenever the method is a write (anything other than `GET`/`HEAD` per `isWriteMethod`) and a run id is configured.
- Parses the response body as JSON (falling back to raw text) and, on non-2xx, throws a `PaperclipApiError` carrying `status`, `method`, `path`, and the parsed error `body`.
- `resolveCompanyId()` / `resolveAgentId()` fall back to the configured defaults and throw a clear message ("... required because `PAPERCLIP_COMPANY_ID`/`PAPERCLIP_AGENT_ID` is not set") when neither an explicit argument nor a default is available.

## MCP tools exposed

Tools are declared in `createToolDefinitions(client)` in [src/tools.ts](../../packages/mcp-server/src/tools.ts). Each is built with `makeTool(name, description, zodSchema, execute)`, which parses the input against the Zod schema, invokes the handler, and formats the result. Names are camelCase and all prefixed `paperclip*`.

**Actor / identity**
- `paperclipMe` → `GET /agents/me`
- `paperclipInboxLite` → `GET /agents/me/inbox-lite`
- `paperclipListAgents` → `GET /companies/{companyId}/agents`
- `paperclipGetAgent` → `GET /agents/{agentId}`

**Issues (read)**
- `paperclipListIssues` → `GET /companies/{companyId}/issues` with a large optional filter set (status, projectId, assignee/participant agent, assignee/touched/unread/inbox-archived user, label, execution workspace, origin kind/id, `includeRoutineExecutions`, free-text `q`).
- `paperclipGetIssue` → `GET /issues/{issueId}` (UUID or short identifier such as `PAP-XXXX`).
- `paperclipGetHeartbeatContext` → `GET /issues/{issueId}/heartbeat-context` (optional `wakeCommentId`).
- `paperclipListComments` / `paperclipGetComment` → `GET /issues/{issueId}/comments[/{commentId}]` (incremental `after`/`order`/`limit`).
- `paperclipListIssueApprovals` → `GET /issues/{issueId}/approvals`.
- `paperclipListDocuments` / `paperclipGetDocument` / `paperclipListDocumentRevisions` → issue-document reads under `/issues/{issueId}/documents`.

**Issues (write)**
- `paperclipCreateIssue` → `POST /companies/{companyId}/issues` (merges `@paperclipai/shared`'s `createIssueInputSchema`).
- `paperclipUpdateIssue` → `PATCH /issues/{issueId}` (merges `updateIssueSchema`; supports `resume=true` for resumable closed work).
- `paperclipCheckoutIssue` → `POST /issues/{issueId}/checkout` (defaults `expectedStatuses` to `["todo","backlog","blocked"]`, resolves agent id).
- `paperclipReleaseIssue` → `POST /issues/{issueId}/release`.
- `paperclipAddComment` → `POST /issues/{issueId}/comments`.
- `paperclipUpsertIssueDocument` → `PUT /issues/{issueId}/documents/{key}` (format defaults to `markdown`; body up to 512 KiB; optional `baseRevisionId` for optimistic concurrency).
- `paperclipRestoreIssueDocumentRevision` → `POST /issues/{issueId}/documents/{key}/revisions/{revisionId}/restore`.

**Issue thread interactions** (all `POST /issues/{issueId}/interactions` with a `kind` discriminator and a shared-schema `payload`; each carries `idempotencyKey`, optional `sourceCommentId`/`sourceRunId`, `title`, `summary`, and a `continuationPolicy`)
- `paperclipSuggestTasks` — `kind: suggest_tasks` (default policy `wake_assignee`).
- `paperclipAskUserQuestions` — `kind: ask_user_questions` (default `wake_assignee`).
- `paperclipRequestConfirmation` — `kind: request_confirmation` (default `none`).
- `paperclipRequestCheckboxConfirmation` — `kind: request_checkbox_confirmation` (default `wake_assignee`).

**Execution workspace runtime**
- `paperclipGetIssueWorkspaceRuntime` — resolves the current execution workspace + runtime services (with service URLs) by reading the issue's heartbeat context.
- `paperclipControlIssueWorkspaceServices` → `POST /execution-workspaces/{workspaceId}/runtime-services/{start|stop|restart}` after locating the workspace via heartbeat context.
- `paperclipWaitForIssueWorkspaceService` — polls the heartbeat context (1s interval, default 60s, max 300s) until the selected service is `running` and not `unhealthy`, returning the workspace + service (or a `timedOut` payload).

**Projects & goals**
- `paperclipListProjects` / `paperclipGetProject` → `/companies/{companyId}/projects`, `/projects/{projectId}`.
- `paperclipListGoals` / `paperclipGetGoal` → `/companies/{companyId}/goals`, `/goals/{goalId}`.

**Approvals**
- `paperclipListApprovals` / `paperclipGetApproval` / `paperclipGetApprovalIssues` / `paperclipListApprovalComments` (reads).
- `paperclipCreateApproval` → `POST /companies/{companyId}/approvals` (merges `createApprovalSchema`).
- `paperclipLinkIssueApproval` / `paperclipUnlinkIssueApproval` → link/unlink an approval to an issue.
- `paperclipApprovalDecision` → one of `/approvals/{id}/{approve|reject|request-revision|resubmit}`; `resubmit` sends `{ payload }` (parsed from `payloadJson`), the others send `{ decisionNote }`.
- `paperclipAddApprovalComment` → `POST /approvals/{approvalId}/comments`.

**Escape hatch**
- `paperclipApiRequest` — arbitrary `{ method, path, jsonBody }` against `/api`. Guarded: `path` must start with `/` and must not contain `..` (path-traversal defense; enforced in the tool *and* re-checked by the client).

> Note: this server exposes **tools only** — it does not register MCP *resources* or *prompts*. `server.tool(...)` is the sole registration call in [src/index.ts](../../packages/mcp-server/src/index.ts).

## Transport

The only transport is **stdio** (`StdioServerTransport` from `@modelcontextprotocol/sdk`), wired in `runServer()` in [src/index.ts](../../packages/mcp-server/src/index.ts). There is no HTTP/SSE MCP listener; the server is designed to be spawned as a child process by an MCP client that speaks the stdio JSON-RPC framing. Because startup errors are logged to `stderr` and cause `process.exit(1)` ([src/stdio.ts](../../packages/mcp-server/src/stdio.ts)), a misconfigured environment fails fast and visibly to the launching client.

## Authentication

Authentication is entirely **bearer-token based and delegated to the control plane**. The MCP server holds no sessions and performs no local authorization — it forwards `Authorization: Bearer <PAPERCLIP_API_KEY>` and lets the Paperclip server's auth middleware ([server/src/middleware/auth.ts](../../server/src/middleware/auth.ts)) resolve the actor. That middleware accepts the same bearer token in several shapes:

- **Agent API keys** — the token is SHA-256 hashed and matched against the `agentApiKeys` table (non-revoked), yielding an agent actor scoped to a company.
- **Local agent JWTs** — verified via `verifyLocalAgentJwt`, carrying `company_id` and optionally a `run_id`.
- **Board API keys** — company/board-scoped keys.

The `X-Paperclip-Run-Id` header sent by the client is read by the same middleware (`x-paperclip-run-id`) and used to stamp `req.actor.runId`, which the control plane uses to attribute mutations to a specific agent run. This is why the MCP client forwards the run id on writes by default (see [src/client.ts](../../packages/mcp-server/src/client.ts)).

Practically, the environment the MCP server reads (`PAPERCLIP_API_URL`, `PAPERCLIP_API_KEY`, plus the optional company/agent/run ids) is provisioned by the agent runtime / CLI connect flow rather than by a human. The CLI `paperclipai connect` command (registered by `registerConnectCommand` in [cli/src/commands/client/connect.ts](../../cli/src/commands/client/connect.ts); note the command is top-level `connect`, even though the source lives under `commands/client/`) creates an agent or board API key and its `buildExports()` helper emits the matching `export PAPERCLIP_API_URL=...` / `export PAPERCLIP_API_KEY=...` (plus optional `PAPERCLIP_COMPANY_ID` / `PAPERCLIP_AGENT_ID`) profile shell. The token env-var name defaults to `PAPERCLIP_API_KEY` (overridable via `--api-key-env-var-name`).

## Data flow / execution lifecycle

```
MCP client (agent runtime)
  │  spawn `paperclip-mcp-server` (stdio)
  ▼
stdio.ts → runServer() → createPaperclipMcpServer(readConfigFromEnv())
  │  register N tools on McpServer
  ▼
MCP client calls tool "paperclipX" with args
  ▼
makeTool.execute(input)
  │  1. schema.parse(input)                 (Zod validation)
  │  2. resolve companyId/agentId defaults   (client.resolve*)
  │  3. client.requestJson(method, path, {body})
  │        → Authorization: Bearer <key>
  │        → X-Paperclip-Run-Id (on writes)
  ▼
Paperclip control plane (/api/...) → auth middleware resolves actor → route handler → DB
  ▼
JSON response (or PaperclipApiError on non-2xx)
  ▼
format.ts → { content: [{ type: "text", text: JSON.stringify(...) }] }
  ▼
back to MCP client
```

For a single tool invocation the lifecycle is: **validate → resolve defaults → HTTP request → format**. The two "smart" tools (`paperclipControlIssueWorkspaceServices`, `paperclipWaitForIssueWorkspaceService`) issue *multiple* requests — they first `GET /issues/{id}/heartbeat-context`, read `currentExecutionWorkspace` and its `runtimeServices`, then act on the resolved workspace id (control) or poll until healthy (wait). Everything else is one request per tool call.

Errors never throw out of a tool: `makeTool` wraps `execute` in try/catch and routes failures through `formatErrorResponse`, so the MCP client always receives a text payload. `PaperclipApiError` is rendered with `{ error, status, method, path, body }`; other errors as `{ error: message }` ([src/format.ts](../../packages/mcp-server/src/format.ts)).

## Contracts & cross-layer coupling

- **REST contract with the control plane.** The tool set is tightly coupled to Paperclip's `/api` route shapes (`/issues/{id}`, `/companies/{id}/issues`, `/issues/{id}/interactions`, `/execution-workspaces/{id}/runtime-services/{action}`, `/approvals/{id}/{decision}`, etc.). These paths must stay in sync with the server routes under [server/src/routes/](../../server/src/routes) (e.g. `issues.ts`, `approvals.ts`, `execution-workspaces.ts`). The MCP server has no independent knowledge of these — a route rename is a breaking change here.
- **Shared validation schemas.** Input schemas are not duplicated: `paperclipCreateIssue`, `paperclipUpdateIssue`, `paperclipCreateApproval`, the interaction payloads, and the continuation-policy enum are imported from `@paperclipai/shared` (`createIssueInputSchema`, `updateIssueSchema`, `createApprovalSchema`, `suggestTasksPayloadSchema`, `askUserQuestionsPayloadSchema`, `requestConfirmationPayloadSchema`, `requestCheckboxConfirmationPayloadSchema`, `issueThreadInteractionContinuationPolicySchema`, `linkIssueApprovalSchema`, `upsertIssueDocumentSchema`). This guarantees the MCP tool surface accepts exactly what the server accepts. See [packages/shared/src/validators/issue.ts](../../packages/shared/src/validators/issue.ts) and [packages/shared/src/validators/approval.ts](../../packages/shared/src/validators/approval.ts).
- **Actor & run attribution.** The `PAPERCLIP_AGENT_ID` / `PAPERCLIP_RUN_ID` env vars couple this process to a specific agent run; the run id header is the mechanism the control plane uses to attribute comments, interactions, and mutations. The heartbeat-context shape (`currentExecutionWorkspace.runtimeServices[]` with `id`, `serviceName`, `status`, `healthStatus`, `url`) is an implicit contract consumed by the workspace-runtime tools.
- **No DB access.** By design the package has no `@paperclipai/db` dependency (only `@modelcontextprotocol/sdk`, `@paperclipai/shared`, `zod`), reinforcing that it is a client of the control plane, not a peer of the server.

## Extension points

- **Add a tool:** append a `makeTool(...)` entry inside `createToolDefinitions` in [src/tools.ts](../../packages/mcp-server/src/tools.ts) with a Zod schema and an `execute` that calls `client.requestJson`. Registration is automatic (the loop in [src/index.ts](../../packages/mcp-server/src/index.ts) registers every returned definition via `server.tool`), and it should be added to the [README](../../packages/mcp-server/README.md) tool-surface list to keep it in sync.
- **Reuse server contracts:** prefer importing the relevant validator from `@paperclipai/shared` and merging it (e.g. `z.object({ issueId }).merge(updateIssueSchema)`) rather than hand-rolling a schema, so the tool tracks the server.
- **Interim coverage:** endpoints without a dedicated tool are reachable today via `paperclipApiRequest` — a good signal for which tool to add next.
- **Config surface:** new env-driven behavior belongs in `PaperclipMcpConfig` + `readConfigFromEnv()` ([src/config.ts](../../packages/mcp-server/src/config.ts)); new header/transport behavior belongs in `requestJson` ([src/client.ts](../../packages/mcp-server/src/client.ts)).

## Testing

- **Runner:** Vitest (`node` environment) via [vitest.config.ts](../../packages/mcp-server/vitest.config.ts); `pnpm --filter @paperclipai/mcp-server test` (`vitest run`).
- **Coverage:** [src/tools.test.ts](../../packages/mcp-server/src/tools.test.ts) constructs a real `PaperclipApiClient` with placeholder ids and stubs global `fetch` to assert:
  - Auth header + `X-Paperclip-Run-Id` are attached to mutating requests.
  - Company-scoped list tools fall back to `PAPERCLIP_COMPANY_ID`; checkout falls back to `PAPERCLIP_AGENT_ID`.
  - `paperclipCreateIssue` may omit `status` (the server derives it from the assignee) while the merged `createIssueInputSchema` fills `workMode: "standard"`, `priority: "medium"`, and `requestDepth: 0` at `parse()` time, so those appear in the outgoing body.
  - `paperclipUpsertIssueDocument` defaults `format` to `markdown`.
  - `paperclipControlIssueWorkspaceServices` performs the two-step heartbeat-context → runtime-service control flow, and `paperclipWaitForIssueWorkspaceService` returns once a service URL is healthy.
  - Interaction tools (`suggest_tasks`, `request_confirmation`, `request_checkbox_confirmation`) serialize the expected issue-scoped payloads with the right `kind` and `continuationPolicy` defaults.
  - `paperclipApiRequest` rejects non-rooted paths and `..` traversal.
- **Static checks:** `pnpm --filter @paperclipai/mcp-server typecheck` (`tsc --noEmit`) and `build` (`tsc` to `dist`).

## Gotchas / invariants

- **Stdio-only.** There is no network MCP listener; the process must be spawned by an MCP client over stdio. `stdout` is the MCP channel — do not `console.log` to it (only `stderr` is used, for fatal startup errors).
- **Fail-fast config.** Startup throws if `PAPERCLIP_API_URL` or `PAPERCLIP_API_KEY` is missing. Company/agent id errors surface *per tool call*, not at boot.
- **URL normalization.** `normalizeApiUrl` appends `/api` only if absent, so passing a URL that already ends in `/api` is safe, but passing one that ends in `/api/` (trailing slash) still normalizes correctly. All `requestJson` paths must be root-relative to `/api`.
- **Run id only on writes by default.** `X-Paperclip-Run-Id` is sent when `includeRunId` is truthy or the method is a write. Read-only tools intentionally omit it unless forced.
- **No client-side authorization.** The MCP server trusts the bearer token entirely; all permission checks happen server-side in the control plane. A leaked `PAPERCLIP_API_KEY` is a full agent-actor credential.
- **Errors are values, not exceptions.** Tool handlers never surface exceptions to the MCP client; failures come back as text content. Callers must inspect the returned JSON for an `error` field rather than relying on protocol-level failures.
- **Escape hatch is guarded but powerful.** `paperclipApiRequest` can reach any `/api` route the token is authorized for; the `..`/leading-slash guards prevent path escape but not privilege escalation beyond the token's grants.

## Key files

- [packages/mcp-server/src/index.ts](../../packages/mcp-server/src/index.ts) — server factory + transport wiring.
- [packages/mcp-server/src/stdio.ts](../../packages/mcp-server/src/stdio.ts) — executable stdio entry point.
- [packages/mcp-server/src/tools.ts](../../packages/mcp-server/src/tools.ts) — all tool definitions and REST mapping.
- [packages/mcp-server/src/client.ts](../../packages/mcp-server/src/client.ts) — `PaperclipApiClient`, headers, and error type.
- [packages/mcp-server/src/config.ts](../../packages/mcp-server/src/config.ts) — env config and URL normalization.
- [packages/mcp-server/src/format.ts](../../packages/mcp-server/src/format.ts) — MCP text-response formatting.
- [packages/mcp-server/src/tools.test.ts](../../packages/mcp-server/src/tools.test.ts) — tool behavior tests.
- [packages/mcp-server/README.md](../../packages/mcp-server/README.md) — usage and tool-surface list (note: the README table currently omits `paperclipRequestCheckboxConfirmation`; [src/tools.ts](../../packages/mcp-server/src/tools.ts) is authoritative).
- [server/src/middleware/auth.ts](../../server/src/middleware/auth.ts) — control-plane bearer/run-id resolution this server relies on.

## Related docs

- [Architecture map](./index.md) — index of all subsystem architecture docs.
- [Server](./server.md) — the control-plane HTTP API, middleware, and auth wiring this server calls.
- [Shared contracts](./shared-contracts.md) — the `@paperclipai/shared` validators the tool schemas merge.
- [Data model](./data-model.md) — the tables (e.g. `agentApiKeys`) the auth path resolves against.
- [Adapters](./adapters.md) — the agent runtimes that spawn this server over stdio.
- [Documentation set](../index.md) — the wider `doc/` map (deployment modes, database, execution semantics, CLI).
