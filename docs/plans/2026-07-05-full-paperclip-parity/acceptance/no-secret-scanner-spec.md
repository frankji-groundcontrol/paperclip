# No-Secret Scanner Spec

Scan these surfaces for forbidden values and sentinel leaks:

- HTTP responses.
- Nuxt rendered HTML/payload/browser storage.
- CLI stdout/stderr.
- MCP JSON-RPC output.
- Backend logs.
- Activity log details.
- Run logs/events.
- Plugin logs.
- Config revisions.
- Approval payloads.
- Export/import bundles.
- Test snapshots.

Forbidden examples:

- `SUPABASE_SERVICE_ROLE_KEY`
- raw JWTs where opaque `pcs_` sessions are expected
- Supabase project refs/URLs in browser surfaces
- `OPENAI_API_KEY`, `sk-`, provider API keys
- `key_hash`, plaintext API keys after creation response
- unredacted env values

The scanner must fail on injected fake sentinel values before it is trusted.
