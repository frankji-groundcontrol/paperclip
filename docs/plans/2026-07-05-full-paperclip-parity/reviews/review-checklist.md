# Review Checklist

Before any phase closes:

- Source evidence cited.
- Matrix rows updated.
- Every non-full baseline row has all closure fields: phase, acceptance_id, failing_test, passing_test, evidence_ref, final_status.
- Red test shown failing before implementation.
- Targeted green command shown passing.
- No unresolved owned gaps.
- No Supabase service-role dependency.
- No Paperclip application objects in `public` schema.
- No company-boundary leak.
- No raw secret exposure.
- Agent status labels are canonical.
- Migration number is unique and recorded in `evidence/migration-ledger.md`.
- Docs/contracts updated when behavior/API/commands changed.
- UI/CLI/MCP rows updated if affected.
- Acceptance IDs linked.
- Category-level acceptance cannot close a row-level gap; row-level matrix must be green or waived.
- Waiver present for any deliberate divergence, with product spec update.

Final review requires:

```text
gaps_missing=0 gaps_partial=0 gaps_divergent=0
```
