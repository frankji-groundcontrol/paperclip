# Real User / Real Agent Scenarios

## Scenario A — default direct hire/create

1. `testadmin1` signs in.
2. Create company.
3. Assert `require_board_approval_for_new_agents=false`.
4. Create/hire CEO with adapter config and budget.
5. Assert status `idle`, not `active`.
6. Mint agent key.
7. Wake/invoke agent for a real OpenAI Responses run.
8. Assert run lifecycle and work output.

## Scenario B — approval-required hire

1. Board enables `require_board_approval_for_new_agents=true`.
2. Operator/authorized agent requests hire with full payload.
3. Approval payload includes redacted config, desired skills, budget, reportsTo, source issue links.
4. Board requests revision, requester resubmits, board approves.
5. Agent becomes `idle`; budget policy/activity logs created.
6. Rejection path creates `terminated` agent and revoked keys.

## Scenario C — low-trust/cross-company denial

1. Low-trust agent key tries to create agents outside scope: denied.
2. Cross-company key tries to list/modify another company: denied.
3. Viewer attempts unsafe mutation: denied.
4. Agent cannot decide approvals.

## Scenario D — hard-stop budget

1. Configure low budget.
2. Run costed OpenAI Responses task.
3. Exceed budget.
4. Assert hard-stop auto-pause and approval path for override.
