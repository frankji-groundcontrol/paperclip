# Architecture — Board UI (React + Vite)

Internal architecture doc for the `@paperclipai/ui` subsystem — the Paperclip
"board" single-page application. It is a React 19 + TypeScript app built with
Vite and Tailwind CSS v4, served (in production) as the static `dist/` bundle
published by the workspace package and reverse-proxied under the same origin as
the [server](./server.md) HTTP API. Everything in this document is grounded in
the code under [ui/](../../ui).

Related subsystem docs: [server](./server.md) (the `/api` backend this UI talks
to), [data-model](./data-model.md) (the entities rendered here),
[shared-contracts](./shared-contracts.md) (the `@paperclipai/shared` wire types
imported here), [plugins](./plugins.md) (the plugin system the UI bridge mounts),
[adapters](./adapters.md), [catalogs](./catalogs.md), and
[mcp-server](./mcp-server.md). See the [architecture map](./index.md) for the
full subsystem index.

---

## Purpose / Overview

The board UI is the human control plane for a Paperclip instance: it lets a
"board" (human operators) create companies, hire and configure agents, file and
triage issues, run and monitor agent work, manage routines/pipelines/goals,
review approvals, inspect costs, and administer instance settings. It is:

- **Dense, keyboard-driven, dark-by-default.** A professional control plane, not
  a marketing site (see the design-guide skill, below).
- **Company-scoped.** Almost every route lives under a company prefix
  (`/:companyPrefix/...`, e.g. `/PAP/dashboard`), and almost every query is keyed
  by the selected company id.
- **Live.** A single WebSocket per selected company drives cache invalidation and
  toasts through TanStack Query, so open surfaces refresh as agents work.
- **Extensible.** A plugin bridge lets plugin-supplied UI bundles mount host
  React components, call the host API, and register launchers/route slots.

### Tech stack (from [ui/package.json](../../ui/package.json))

- React 19 + TypeScript 5, bundled by Vite 6, tests via Vitest 4.
- [Tailwind CSS v4](../../ui/src/index.css) via `@tailwindcss/vite`, OKLCH design
  tokens, shadcn/ui "new-york" style ([components.json](../../ui/components.json)).
- Radix UI primitives, `lucide-react` icons, `class-variance-authority` (CVA),
  `clsx` + `tailwind-merge` (via the `cn()` helper in
  [ui/src/lib/utils.ts](../../ui/src/lib/utils.ts)).
- [`@tanstack/react-query`](../../ui/src/lib/queryKeys.ts) for all server state.
- `react-router-dom` v7, wrapped by a company-aware shim ([ui/src/lib/router.tsx](../../ui/src/lib/router.tsx)).
- `react-i18next` + `i18next` for translation ([ui/src/i18n/index.ts](../../ui/src/i18n/index.ts)).
- Rich editing/rendering via `lexical`, `@mdxeditor/editor`, `react-markdown`
  (+ `remark-gfm`), `mermaid`, `@assistant-ui/react`, `@dnd-kit/*`.
- Workspace adapter packages (`@paperclipai/adapter-*`) are direct dependencies,
  consumed by the client-side UI adapter layer under
  [ui/src/adapters/](../../ui/src/adapters) that renders adapter-specific
  configuration forms and parses run transcripts (see "UI adapter layer").

---

## Entry points

### Bootstrap — [ui/src/main.tsx](../../ui/src/main.tsx)

`main.tsx` is the single client entry (referenced from
[ui/index.html](../../ui/index.html)). It:

1. Calls `initPluginBridge(React, ReactDOM)` **before** any plugin UI can load, so
   the host's React instances are registered globally
   ([ui/src/plugins/bridge-init.ts](../../ui/src/plugins/bridge-init.ts)).
2. Registers the service worker (`/sw.js`) on `window.load`.
3. Creates one `QueryClient` with global defaults `staleTime: 30_000` and
   `refetchOnWindowFocus: true`.
4. Renders a deeply nested provider tree inside `<StrictMode>`. Order matters —
   later providers depend on earlier ones:

```
QueryClientProvider → ThemeProvider → BrowserRouter → CompanyProvider
  → EditorAutocompleteProvider → ToastProvider → LiveUpdatesProvider
  → TooltipProvider → CompanyAwareBreadcrumbProvider → SidebarProvider
  → PanelProvider → PluginLauncherProvider → DialogProvider → <App/>
```

`CompanyAwareBreadcrumbProvider` is a small adapter that reads
`useCompany().selectedCompany?.name` and feeds it to `BreadcrumbProvider`, which
is why `BrowserRouter`/`CompanyProvider` sit above it.

### Route tree — [ui/src/App.tsx](../../ui/src/App.tsx)

`App` renders the entire `<Routes>` tree. Two conceptual scopes:

- **Global (prefix-less) routes.** `auth`, `board-claim/:token`, `cli-auth/:id`,
  `invite/:token`, and a set of `ux-lab/*` and `tests/perf/*` dev surfaces render
  outside the company chrome.
- **Company/board routes.** Everything else is wrapped in `<CloudAccessGate>`
  and then in a `:companyPrefix` `<Layout>` element whose children come from the
  `boardRoutes()` helper. `boardRoutes()` enumerates all in-company pages:
  `dashboard`, `agents/*`, `projects/*`, `issues/*`, `routines/*`,
  `pipelines/*` (gated), `goals/*`, `artifacts`, `approvals/*`, `costs`,
  `activity`, `inbox/*`, `company/settings/**`, `skills/*`, `org`,
  `design-guide`, plugin route slots (`:pluginRoutePath/*`), and a board-scoped
  `NotFoundPage`.

Redirect components in `App.tsx` handle the prefix contract:
`CompanyRootRedirect` (root → `/:prefix/dashboard`), `UnprefixedBoardRedirect`
(bare board path → prefixed), and `LegacySettingsRedirect` (old settings paths →
normalized instance settings). When there are no companies these redirect to
`/onboarding` or a `NoCompaniesStartPage` that opens the onboarding wizard.

Feature gates wrap specific routes: `PipelinesExperimentalGate`,
`ConferenceRoomChatGate`, `CloudAccessGate`, and `OnboardingWizardVariant`
(rendered once, outside `<Routes>`).

---

## Key modules & responsibilities

### Routing shim — [ui/src/lib/router.tsx](../../ui/src/lib/router.tsx)

This module **re-exports everything from `react-router-dom`** but overrides
`Link`, `NavLink`, `Navigate`, and `useNavigate` so that absolute paths are
automatically rewritten to include the active company prefix. `resolveTo()`
delegates to `applyCompanyPrefix()` in
[ui/src/lib/company-routes.ts](../../ui/src/lib/company-routes.ts);
`useActiveCompanyPrefix()` resolves the prefix from (in priority order) the
`:companyPrefix` route param, the path, then `selectedCompany.issuePrefix`.

`Link` additionally detects issue-shaped targets (via
[ui/src/lib/issue-reference.ts](../../ui/src/lib/issue-reference.ts)) and wraps
them in `IssueLinkQuicklook` for hover previews. **Invariant:** UI code imports
router primitives from `@/lib/router`, never directly from `react-router-dom`, so
the prefix + quicklook behavior is applied everywhere. `company-routes.ts` also
owns the `BOARD_ROUTE_ROOTS` / `GLOBAL_ROUTE_ROOTS` classification used by the
redirect logic.

### Company selection — [ui/src/context/CompanyContext.tsx](../../ui/src/context/CompanyContext.tsx)

`CompanyProvider` is the source of truth for which company is active. It:

- Loads the company list via `useQuery(companiesListQueryOptions)` (shared query
  options in [ui/src/api/companies-query.ts](../../ui/src/api/companies-query.ts)),
  which normalizes a `401` into `{ companies: [], unauthorized: true }` instead of
  throwing.
- Auto-selects a company on load using `resolveBootstrapCompanySelection()`
  (prefers current selection, then the `paperclip.selectedCompanyId`
  `localStorage` value, then the first non-archived company), and persists manual
  selections back to `localStorage`.
- Exposes `companies`, `selectedCompany(Id)`, `selectionSource`, `loading`,
  `error`, `setSelectedCompanyId`, `reloadCompanies`, and `createCompany`.
- `createCompany` is a `useMutation` that invalidates `queryKeys.companies.all`
  and selects the new company.

`useCompany()` throws outside the provider; `useOptionalCompany()` returns `null`
for provider-less surfaces (e.g. exported standalone markdown). `selectedCompany`
is consumed by the router shim, the live-updates socket, and virtually every
page for query keys.

### Layout & chrome — [ui/src/components/Layout.tsx](../../ui/src/components/Layout.tsx)

`Layout` is the element mounted at `:companyPrefix`. It renders the chrome around
the routed `<Outlet/>`: `Sidebar`, `CompanySettingsSidebar`/`SecondarySidebar`,
`BreadcrumbBar`, `PropertiesPanel`, `CommandPalette`, the family of "New …"
dialogs, `ToastViewport`, `MobileBottomNav`, banners
(`WorktreeBanner`, `DevRestartBanner`), `RouteErrorBoundary`, and plugin sidebar
slots (`PluginSlotMount` / `resolveRouteSidebarSlot`). It wires
`useKeyboardShortcuts()`, `useCompanyPageMemory()` (remembers per-company last
route), navigation scroll memory, and syncs company selection from the route via
`shouldSyncCompanySelectionFromRoute()`.

### Pages — [ui/src/pages/](../../ui/src/pages)

Roughly one file per top-level surface (~65 non-test `*.tsx` page files). Major
ones:

- **Dashboard** ([Dashboard.tsx](../../ui/src/pages/Dashboard.tsx),
  [DashboardLive.tsx](../../ui/src/pages/DashboardLive.tsx)) — company overview.
- **Agents** ([Agents.tsx](../../ui/src/pages/Agents.tsx),
  [AgentDetail.tsx](../../ui/src/pages/AgentDetail.tsx),
  [NewAgent.tsx](../../ui/src/pages/NewAgent.tsx)) — the agent roster, per-agent
  config/runs/skills tabs, and the hire flow.
- **Issues** ([Issues.tsx](../../ui/src/pages/Issues.tsx),
  [IssueDetail.tsx](../../ui/src/pages/IssueDetail.tsx)) — the task board and the
  issue thread (chat, runs, documents, approvals, activity). `Issues` uses
  `useInfiniteQuery` for the paginated list.
- **Projects / Workspaces** ([Projects.tsx](../../ui/src/pages/Projects.tsx),
  [ProjectDetail.tsx](../../ui/src/pages/ProjectDetail.tsx),
  [ExecutionWorkspaceDetail.tsx](../../ui/src/pages/ExecutionWorkspaceDetail.tsx)).
- **Routines / Pipelines / Goals** ([Routines.tsx](../../ui/src/pages/Routines.tsx),
  [Pipelines.tsx](../../ui/src/pages/Pipelines.tsx),
  [Goals.tsx](../../ui/src/pages/Goals.tsx)).
- **Approvals, Costs, Activity, Inbox, Search, Artifacts** — the operational
  surfaces.
- **Company / Instance settings** — a large family under
  `company/settings/**` including `CompanySettings`, `CompanyAccess`,
  `CompanyInvites`, `Secrets`, `CompanyEnvironments`, `PluginManager`,
  `AdapterManager`, and the `Instance*Settings` pages.
- **Design system** ([DesignGuide.tsx](../../ui/src/pages/DesignGuide.tsx)) — see
  "Styling & design tokens".
- **Auth / onboarding** ([Auth.tsx](../../ui/src/pages/Auth.tsx),
  [BoardClaim.tsx](../../ui/src/pages/BoardClaim.tsx),
  [CliAuth.tsx](../../ui/src/pages/CliAuth.tsx),
  [InviteLanding.tsx](../../ui/src/pages/InviteLanding.tsx)).

### Components — [ui/src/components/](../../ui/src/components)

~215 feature-component `*.tsx` files (roughly 175 at the top level, the rest in
subfolders) plus a shadcn/ui primitive layer of ~24 files under
[ui/src/components/ui/](../../ui/src/components/ui) (`button`, `dialog`,
`dropdown-menu`, `select`, `tabs`, `sheet`, `popover`, `command`, `badge`,
`card`, `tooltip`, …). Feature components compose those primitives — e.g.
`AgentCapsule`, `CommandPalette`, `CommentThread`, `FileTree`, `Sidebar`,
`ActivityFeed`, `ApprovalCard`, `DocumentAnnotationLayer`.

### Contexts — [ui/src/context/](../../ui/src/context)

Cross-cutting state providers. The first block below is mounted globally in
`main.tsx` (in the order shown under "Bootstrap"); the last two are mounted lower
in the tree, not in `main.tsx`.

| Provider | Mounted in | Responsibility |
|----------|-----------|----------------|
| `CompanyProvider` | `main.tsx` | Selected company + company list (above). |
| `LiveUpdatesProvider` | `main.tsx` | The per-company WebSocket → cache invalidation + toasts. |
| `ThemeProvider` | `main.tsx` | Light/dark theme (`.dark` class). |
| `ToastProvider` | `main.tsx` | Toast queue consumed by `ToastViewport`. |
| `BreadcrumbProvider` | `main.tsx` (via `CompanyAwareBreadcrumbProvider`) | Breadcrumb trail (fed the company name). |
| `SidebarProvider` | `main.tsx` | Sidebar open/collapsed/peek + mobile state. |
| `PanelProvider` | `main.tsx` | Properties panel visibility. |
| `DialogProvider` | `main.tsx` | Global dialogs (onboarding, "new issue", etc.). |
| `EditorAutocompleteProvider` | `main.tsx` | Mention/autocomplete data for editors. |
| `PluginLauncherProvider` | `main.tsx` ([launchers.tsx](../../ui/src/plugins/launchers.tsx)) | Registry for plugin-declared launcher buttons. |
| `GeneralSettingsProvider` | `Layout.tsx` | Per-render UI settings (e.g. keyboard-shortcuts enabled) from [GeneralSettingsContext.tsx](../../ui/src/context/GeneralSettingsContext.tsx). |
| `FileViewerProvider` | `IssueDetail.tsx` (per issue) | File viewer sheet state, scoped to the open issue ([FileViewerContext.tsx](../../ui/src/context/FileViewerContext.tsx)). |

### API client layer — [ui/src/api/](../../ui/src/api)

- **Transport:** [ui/src/api/client.ts](../../ui/src/api/client.ts) exports a
  tiny `api` object (`get/post/postForm/put/patch/delete`) over `fetch`. All calls
  are same-origin under `BASE = "/api"` with `credentials: "include"` (cookie
  auth). It sets `Content-Type: application/json` unless the body is `FormData`,
  treats `204` as `undefined`, and throws a typed `ApiError` (carrying `status` +
  parsed body) on non-2xx.
- **Resource modules:** one file per domain (`issues.ts`, `agents.ts`,
  `pipelines.ts`, `companies.ts`, `access.ts`, `secrets.ts`, `plugins.ts`, …).
  Each exports a `*Api` object of typed methods that return
  `@paperclipai/shared` types. [ui/src/api/index.ts](../../ui/src/api/index.ts)
  barrel-exports the common ones.
- **Shared query options:** [companies-query.ts](../../ui/src/api/companies-query.ts)
  centralizes the `["companies"]` query so `CompanyProvider` and the invite
  landing page agree on cache shape — mismatched shapes on a shared key silently
  corrupt the cache.

### Data fetching & query keys — [ui/src/lib/queryKeys.ts](../../ui/src/lib/queryKeys.ts)

`queryKeys` is a single nested object that is the **canonical registry of every
React Query cache key** in the app. Keys are grouped by domain (`companies`,
`agents`, `issues`, `routines`, `pipelines`, `projects`, `approvals`, `secrets`,
`access`, `instance`, `plugins`, `adapters`, …) and are almost always
parameterized by `companyId` and/or an entity id, e.g.:

- `queryKeys.issues.list(companyId)`, `queryKeys.issues.detail(id)`,
  `queryKeys.issues.comments(issueId)`
- `queryKeys.agents.list(companyId)`, `queryKeys.agents.detail(id)`
- `queryKeys.dashboard(companyId)`, `queryKeys.liveRuns(companyId)`,
  `queryKeys.sidebarBadges(companyId)`

Pages call `useQuery`/`useInfiniteQuery`/`useMutation` directly with a
`queryKeys.*` key and a `*Api` method as the `queryFn`. Mutations invalidate the
relevant keys on success. **Invariant:** never inline a raw string-array cache key
— always go through `queryKeys` so the live-updates layer and mutations invalidate
the same entries.

### Live updates — [ui/src/context/LiveUpdatesProvider.tsx](../../ui/src/context/LiveUpdatesProvider.tsx)

The realtime engine. It reads the auth session
(`queryKeys.auth.session`) and the selected company, and — only when signed in
and a stable company is selected — opens a single WebSocket to
`/api/companies/:companyId/events/ws` (URL built by
[ui/src/lib/websocket-url.ts](../../ui/src/lib/websocket-url.ts), which upgrades
`http→ws`/`https→wss` and normalizes wildcard hosts to `localhost`). It:

- **Reconnects with exponential backoff** (capped at 15s) driven off `onclose`,
  and delays the initial connect by a `0ms` timeout to survive StrictMode's
  double-mount.
- **Parses each `LiveEvent`** and routes it through `handleLiveEvent`, which
  translates event types (`heartbeat.run.*`, `agent.status`, `activity.logged`,
  and entity-scoped events) into targeted `queryClient.invalidateQueries` /
  `setQueryData` calls against `queryKeys.*` — plus optimistic comment/run updates
  via [optimistic-issue-comments.ts](../../ui/src/lib/optimistic-issue-comments.ts)
  and [optimistic-issue-runs.ts](../../ui/src/lib/optimistic-issue-runs.ts).
- **Emits toasts** for run/agent/activity events through `ToastContext`, with a
  cooldown gate (max 3 per category / 10s) and a post-reconnect suppression window
  to avoid a flood after backfill.
- Uses the current route `pathname` to suppress toasts for the issue you are
  already looking at and to hydrate the visible issue's comments.

This is why an open dashboard/issue refreshes as agents work without manual
reloads: the socket invalidates the exact query keys the visible components
subscribe to.

### i18n — [ui/src/i18n/](../../ui/src/i18n)

`i18next` is initialized synchronously (`initAsync: false`) with bundled
`resources`. Code calls the re-exported `useTranslation()` / `t()` from
`@/i18n`. Locale JSON lives under
[ui/src/i18n/locales/](../../ui/src/i18n/locales), validated by
[locale-validation.ts](../../ui/src/i18n/locale-validation.ts).

### UI adapter layer — [ui/src/adapters/](../../ui/src/adapters)

A client-side registry of **UI adapter modules** — the per-runtime UI code that
renders each agent runtime's config form and parses its run transcript. This is
distinct from the workspace `@paperclipai/adapter-*` packages (those are the
runtime/backend adapters); the code here is the board's rendering/parsing side.

- **Registry** ([registry.ts](../../ui/src/adapters/registry.ts)) — holds a
  `UIAdapterModule` per adapter type (`claude_local`, `codex_local`,
  `cursor`/`cursor_cloud`, `gemini_local`, `grok_local`, `openclaw_gateway`,
  `opencode_local`, `pi_local`, `hermes*`, generic `process`/`http`, …), each
  imported from its sibling folder. Exposes `getUIAdapter` / `listUIAdapters` /
  `findUIAdapter` / `registerUIAdapter` / `syncExternalAdapters` /
  `onAdapterChange` (via [index.ts](../../ui/src/adapters/index.ts)).
- **External/dynamic parsers** ([dynamic-loader.ts](../../ui/src/adapters/dynamic-loader.ts))
  — external adapters can ship their own `ui-parser.js` served by the backend;
  the loader fetches and applies it (a generation counter discards stale loads),
  temporarily overriding a builtin type until the override is deactivated.
- **Config-field renderers** ([schema-config-fields.tsx](../../ui/src/adapters/schema-config-fields.tsx),
  [adapter-display-registry.ts](../../ui/src/adapters/adapter-display-registry.ts),
  and per-adapter folders) drive the adapter configuration forms shown in
  `NewAgent`, `AgentDetail`, and `AdapterManager`.
- **Transcript parsing** ([transcript.ts](../../ui/src/adapters/transcript.ts))
  builds a `TranscriptEntry[]` from raw run-log chunks, with the heavier parsing
  isolated in a [sandboxed-parser-worker.ts](../../ui/src/adapters/sandboxed-parser-worker.ts).

See [adapters](./adapters.md) for the backend/runtime adapter contract this layer
mirrors.

### Hooks — [ui/src/hooks/](../../ui/src/hooks)

Shared React hooks used by `Layout`, pages, and the plugin bridge — e.g.
`useKeyboardShortcuts`, `useCompanyPageMemory`, `useProjectOrder` /
`useCompanyOrder` / `useAgentOrder` (sidebar ordering), `useInboxBadge`,
`useResourceMemberships`, `useRetryNowMutation`, and
`useConferenceRoomChatEnabled`. Data-shaping/business logic lives in
[ui/src/lib/](../../ui/src/lib) (the large set of pure helpers + colocated
`*.test.ts`), which the hooks and pages consume.

### Plugin bridge — [ui/src/plugins/](../../ui/src/plugins)

`initPluginBridge` (in [bridge-init.ts](../../ui/src/plugins/bridge-init.ts))
registers the host React/ReactDOM instances and a set of bridge hooks
(`usePluginData`, `usePluginAction`, `useHostContext`, `useHostLocation`,
`useHostNavigation`, `usePluginStream`, `usePluginToast`, from
[bridge.ts](../../ui/src/plugins/bridge.ts)) plus host SDK components
(`FileTree`, `IssuesList`, `AssigneePicker`, `ProjectPicker`, `DataTable`,
`MarkdownBlock` — which wraps the host `MarkdownBody` — …) onto the global
`globalThis.__paperclipPluginBridge__` so plugin UI bundles can consume them at
load time. `PluginLauncherProvider`
([launchers.tsx](../../ui/src/plugins/launchers.tsx)) renders plugin-declared
launcher buttons; [slots.tsx](../../ui/src/plugins/slots.tsx) mounts plugin route
and sidebar slots inside `Layout`. See the `PLUGIN_SPEC.md` references in
`bridge-init.ts` (and [plugins](./plugins.md)) for the contract details.

---

## Data flow / lifecycle

1. **Load.** `main.tsx` mounts the provider tree. `CompanyProvider` fires the
   `["companies"]` query; `LiveUpdatesProvider` fires the `["auth","session"]`
   query.
2. **Auth gate.** If the company query returns `401`, `unauthorized` is set and
   redirects push to `/auth`. Otherwise a company is auto-selected.
3. **Route resolution.** The URL's `:companyPrefix` selects the company chrome
   (`Layout`); the router shim rewrites internal links to stay prefixed.
4. **Page render.** A page issues `useQuery(queryKeys.<domain>.<...>(companyId,…),
   <api>.<method>)`. `fetch` hits `/api/...` with cookies; `ApiError` surfaces
   typed failures.
5. **Mutation.** User actions call `useMutation` → `<api>.<method>` (POST/PATCH/…)
   → `invalidateQueries(queryKeys.*)` on success, so dependent surfaces refetch.
6. **Live push.** The company WebSocket delivers `LiveEvent`s;
   `handleLiveEvent` invalidates/patches the matching `queryKeys.*` and may raise a
   toast. Open components re-render from fresh cache.
7. **Focus/staleness.** Global defaults (`staleTime` 30s, refetch on window focus)
   backstop the socket for anything not covered by an explicit invalidation.

---

## Contracts & cross-layer coupling

- **Same-origin `/api` + cookies.** The client assumes the backend is reachable at
  `/api` on the same origin (Vite dev proxies `/api` — including WS — to the
  server; see [ui/vite.config.ts](../../ui/vite.config.ts)). Auth is cookie-based
  (`credentials: "include"`); there is no token handling in the client.
- **Shared types.** All request/response shapes come from `@paperclipai/shared`
  (e.g. `Company`, `Issue`, `IssueComment`, `LiveEvent`). The UI does not redefine
  the wire model — it imports it. See [data-model](./data-model.md) and
  [server](./server.md).
- **Company-prefixed URLs.** The `/:companyPrefix/...` scheme is a shared contract
  between `App.tsx` routing, `router.tsx`, and `company-routes.ts`. `issuePrefix`
  comes from the `Company` record.
- **Query-key registry.** `queryKeys.ts` is the coupling point between pages,
  mutations, and `LiveUpdatesProvider`; all three must agree on the key for a
  given resource.
- **WebSocket event schema.** `LiveEvent` types and payload fields
  (`agentId`, `runId`, `identifier`, `key`, …) are consumed structurally in
  `handleLiveEvent`; new server event types need matching handling here to drive
  invalidation.

---

## Styling & design tokens

- **Global stylesheet:** [ui/src/index.css](../../ui/src/index.css) imports
  Tailwind v4 and `@tailwindcss/typography`, defines the `dark` custom variant,
  and declares the token system. `@theme inline` maps Tailwind color utilities to
  CSS variables; `:root` and `.dark` define the actual OKLCH values.
- **Token families:** semantic surface/text tokens
  (`--background`/`--foreground`, `--card`, `--primary`, `--muted`, `--accent`,
  `--destructive`, `--border`, `--ring`, `--sidebar-*`, `--chart-1..5`), radius
  (`--radius*`), search-match chip tints, document-annotation highlights, the
  brand agent-capsule gradients (`--agent-1a` … `--agent-10b`), and the
  status/priority hues (`--status-agent-*`, `--status-task-*`, and AA-tuned
  `--status-task-icon-*`). Status color helpers live in
  [ui/src/lib/status-colors.ts](../../ui/src/lib/status-colors.ts).
- **shadcn/ui config:** [ui/components.json](../../ui/components.json) — new-york
  style, neutral base, CSS variables on, aliases `@/components`, `@/components/ui`,
  `@/lib`, `@/hooks`, lucide icons.
- **Living showcase:** [ui/src/pages/DesignGuide.tsx](../../ui/src/pages/DesignGuide.tsx)
  is the in-app `/design-guide` page — a rendered gallery of the primitives,
  tokens, typography scale, status/priority systems, and composition patterns.

### Relationship to the `design-guide` skill

The repo ships a `design-guide` skill at
[.claude/skills/design-guide/SKILL.md](../../.claude/skills/design-guide/SKILL.md).
It is the written counterpart to the `DesignGuide` page: it documents the design
principles (dense, keyboard-first, dark default), the token catalog (pointing at
`index.css`), the exact typography/spacing/status conventions, and composition
guidance, and it explicitly references the `/design-guide` showcase route. Agents
building or modifying UI are expected to consult that skill (alongside
`frontend-design` and `web-design-guidelines`); this doc and that skill describe
the same system from the code side vs. the authoring-guidance side.

---

## Storybook — [ui/storybook/](../../ui/storybook)

Storybook is configured separately from the app under `ui/storybook/`:

- **Config:** [.storybook/main.ts](../../ui/storybook/.storybook/main.ts) sets
  `@storybook/react-vite`, autodocs, the `addon-docs` and `addon-a11y` addons,
  serves `../../public` as static, and re-applies the Tailwind Vite plugin. It
  re-declares the `@` and `lexical` aliases and adds a `node:crypto` browser shim
  ([node-crypto-browser-shim.ts](../../ui/storybook/.storybook/node-crypto-browser-shim.ts))
  because `@paperclipai/shared` imports `createHash` server-side.
- **Preview:** [.storybook/preview.tsx](../../ui/storybook/.storybook/preview.tsx)
  wraps every story in the real provider stack (`QueryClientProvider`,
  `MemoryRouter`, `ThemeProvider`, company/dialog/sidebar/toast providers) and
  installs a fetch monkeypatch that serves fixtures from
  [fixtures/paperclipData.ts](../../ui/storybook/fixtures/paperclipData.ts) so
  components render against deterministic data with no backend.
- **Stories:** ~43 `*.stories.tsx` files under
  [ui/storybook/stories/](../../ui/storybook/stories) cover surfaces like the
  overview/foundations, issue management, routines, skills store, control-plane
  surfaces, and status language.

Run with `pnpm --filter @paperclipai/ui storybook` (dev) or `build-storybook`
(static output), per [ui/README.md](../../ui/README.md).

---

## Build & test scripts (from [ui/package.json](../../ui/package.json))

| Script | Command | Purpose |
|--------|---------|---------|
| `dev` | `vite` | Local dev server (port 5173, proxies `/api` → `http://localhost:3100`, WS enabled). |
| `build` | `tsc -b && vite build` | Type-check project refs then produce the `dist/` bundle (esbuild minify; `console`/`debugger` dropped in production). |
| `typecheck` | `tsc -b` | Type-check only. |
| `preview` | `vite preview` | Serve the built `dist/`. |
| `storybook` / `build-storybook` | `storybook dev`/`build` (`-c storybook/.storybook`) | Component workshop. |
| `clean` | `rm -rf dist storybook-static tsconfig.tsbuildinfo` | Reset build artifacts. |
| `prepack` / `postpack` | `scripts/generate-ui-package-json.mjs` | Rewrite `package.json` for publishing the prebuilt `dist/` package, then restore it. |

**Tests** use Vitest ([ui/vitest.config.ts](../../ui/vitest.config.ts)) with the
`@`/`lexical` aliases and [ui/vitest.setup.ts](../../ui/vitest.setup.ts) (mocks
`localStorage` and stubs `Element.prototype.scrollIntoView`). Tests live next to
their subjects as `*.test.ts(x)` (e.g.
[CompanyContext.test.tsx](../../ui/src/context/CompanyContext.test.tsx),
[Issues.test.tsx](../../ui/src/pages/Issues.test.tsx),
[LiveUpdatesProvider.test.ts](../../ui/src/context/LiveUpdatesProvider.test.ts)).
The default environment is `node`; component tests set up jsdom themselves.

---

## Extension points

- **New page/route.** Add a page under `ui/src/pages/`, then register it in
  `boardRoutes()` (company-scoped) or the global block in
  [App.tsx](../../ui/src/App.tsx). Board-route roots must also appear in
  `BOARD_ROUTE_ROOTS` in [company-routes.ts](../../ui/src/lib/company-routes.ts)
  so the unprefixed-redirect logic recognizes them.
- **New API + data.** Add a `*Api` module in [ui/src/api/](../../ui/src/api), a
  matching key group in [queryKeys.ts](../../ui/src/lib/queryKeys.ts), and consume
  via `useQuery`/`useMutation`. If the server emits live events for it, add a
  branch to `handleLiveEvent` in
  [LiveUpdatesProvider.tsx](../../ui/src/context/LiveUpdatesProvider.tsx).
- **New primitive/component.** Follow the `design-guide` skill and the
  `/design-guide` page; reuse `@/components/ui` primitives and the `cn()` helper;
  add a Storybook story for stateful/visual surfaces.
- **Plugin UI.** Register launchers/slots via the plugin bridge modules; the host
  contract is the set of bridge hooks/components in
  [ui/src/plugins/](../../ui/src/plugins).

---

## Gotchas / invariants

- **Import router primitives from `@/lib/router`, not `react-router-dom`** — the
  shim provides company-prefixing and issue quicklooks. Bypassing it produces
  links that drop the company prefix.
- **All cache keys go through `queryKeys`.** The live-updates layer and mutations
  invalidate by those exact keys; a hand-rolled key silently won't be refreshed by
  the socket.
- **Shared cache shape.** The `["companies"]` query must return the wrapped
  `CompanyListResult` from every reader (see
  [companies-query.ts](../../ui/src/api/companies-query.ts)); returning a bare
  `Company[]` from one reader corrupts the shared entry.
- **StrictMode + WebSocket.** The socket connect is deferred one tick and cleaned
  up carefully so React's double-invoke in dev does not spam
  "WebSocket closed before connection established". Don't remove the `setTimeout(…,
  0)` / `closeSocketQuietly` handling.
- **Same-origin assumption.** The client hard-codes `/api` and relies on
  cookies; there is no base-URL config. Cross-origin deployment must be handled by
  the reverse proxy, not the client.
- **Company must be stable to go live.** `resolveLiveCompanyId` only returns an id
  when `selectedCompanyId === selectedCompany.id`, so the socket does not connect
  mid-selection with a stale id.
- **`console`/`debugger` are stripped in production** by esbuild
  ([vite.config.ts](../../ui/vite.config.ts)); do not rely on them for runtime
  behavior.
- **Storybook needs the `node:crypto` shim** for `@paperclipai/shared` imports;
  keep the alias when adding stories that pull in shared canonicalization code.

---

## Key files

- [ui/src/main.tsx](../../ui/src/main.tsx) — entry, `QueryClient`, provider tree.
- [ui/src/App.tsx](../../ui/src/App.tsx) — full route tree + prefix redirects.
- [ui/src/lib/router.tsx](../../ui/src/lib/router.tsx) — company-aware router shim.
- [ui/src/lib/company-routes.ts](../../ui/src/lib/company-routes.ts) — prefix rules.
- [ui/src/context/CompanyContext.tsx](../../ui/src/context/CompanyContext.tsx) — company selection state.
- [ui/src/context/LiveUpdatesProvider.tsx](../../ui/src/context/LiveUpdatesProvider.tsx) — WebSocket → cache invalidation + toasts.
- [ui/src/lib/queryKeys.ts](../../ui/src/lib/queryKeys.ts) — the cache-key registry.
- [ui/src/api/client.ts](../../ui/src/api/client.ts) — `fetch` transport + `ApiError`.
- [ui/src/api/index.ts](../../ui/src/api/index.ts) — API module barrel.
- [ui/src/adapters/registry.ts](../../ui/src/adapters/registry.ts) — client-side UI-adapter registry (config forms + transcript parsers).
- [ui/src/plugins/bridge-init.ts](../../ui/src/plugins/bridge-init.ts) — plugin host bridge registration.
- [ui/src/components/Layout.tsx](../../ui/src/components/Layout.tsx) — board chrome + outlet.
- [ui/src/index.css](../../ui/src/index.css) — Tailwind v4 + OKLCH design tokens.
- [ui/src/pages/DesignGuide.tsx](../../ui/src/pages/DesignGuide.tsx) — in-app design showcase.
- [.claude/skills/design-guide/SKILL.md](../../.claude/skills/design-guide/SKILL.md) — design-system authoring skill.
- [ui/storybook/.storybook/main.ts](../../ui/storybook/.storybook/main.ts) / [preview.tsx](../../ui/storybook/.storybook/preview.tsx) — Storybook setup.
- [ui/vite.config.ts](../../ui/vite.config.ts) / [ui/vitest.config.ts](../../ui/vitest.config.ts) — build & test config.
- [ui/package.json](../../ui/package.json) — scripts & dependencies.
