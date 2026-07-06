#!/usr/bin/env python3
"""Parity gap ledger gate (Phase 00 parity oracle).

Fails until every non-full row in evidence/rewrite-gap-matrix.jsonl is closed
as final_status=full, or final_status=waived with a waiver_ref.

On failure it prints diagnostic detail. On success it prints EXACTLY the
contract line required by 05-test-strategy.md and acceptance/full-parity-harness-spec.md:

    gaps_missing=0 gaps_partial=0 gaps_divergent=0

No other fields are appended on the success line.
"""
from __future__ import annotations
import json
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
MATRIX = ROOT / "evidence" / "rewrite-gap-matrix.jsonl"

rows = [json.loads(line) for line in MATRIX.read_text().splitlines() if line.strip()]

open_rows = []
for row in rows:
    status = row.get("initial_status")
    final = row.get("final_status")
    if status == "full":
        continue
    if final == "full":
        continue
    if final == "waived" and row.get("waiver_ref"):
        continue
    open_rows.append(row)

counts = {"missing": 0, "partial": 0, "divergent": 0}
for row in open_rows:
    if row.get("initial_status") in counts:
        counts[row["initial_status"]] += 1

if any(counts.values()):
    print(
        "gaps_missing=" + str(counts["missing"])
        + " gaps_partial=" + str(counts["partial"])
        + " gaps_divergent=" + str(counts["divergent"])
        + " rows_total=" + str(len(rows))
        + " rows_open=" + str(len(open_rows))
    )
    print("OPEN_ROWS_SAMPLE:")
    for row in open_rows[:20]:
        print("- " + row["row_id"] + " [" + row["initial_status"] + "] " + row["phase"] + " :: " + row["capability"])
    sys.exit(1)

print("gaps_missing=0 gaps_partial=0 gaps_divergent=0")
sys.exit(0)
