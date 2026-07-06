#!/usr/bin/env python3
"""Live real-user / real-agent acceptance harness (Phase 00 deliverable).

This is a deliberately failing RED scaffold. It is owned by Phase 00 of the
full-parity plan: Phase 00 cannot close until this script is implemented to
prove, against the supabase-franky project and a real OpenAI Responses run:

  - real authenticated users from test_users.json (gitignored) get opaque pcs_ sessions;
  - a real company is formed with require_board_approval_for_new_agents=false;
  - a real agent is created/hired, approved where required, and reaches the
    canonical idle runtime state (not the rewrite's divergent 'active');
  - the hired agent runs a REAL OpenAI Responses job attributed to
    subject_type='agent' and the correct agent_id;
  - cross-company / low-trust / agent-cannot-decide denials hold;
  - no Supabase URL/anon key/service role/JWT/OpenAI key reaches any client surface.

While unimplemented it exits non-zero so the release gate cannot pass. The
Phase 00 worker replaces this body with the live driver and the matrix row
phase-00-parity-oracle-run-real-user-real-agent-py-live is closed to final_status=full.
"""
import sys


def main() -> int:
    # RED state: Phase 00 must implement the live driver described above.
    return 2


if __name__ == "__main__":
    sys.exit(main())
