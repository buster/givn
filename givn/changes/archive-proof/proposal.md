# Proposal: archive-proof

## Use-Case Context

- Use-case ID: archive-change (includes verify-change)
- Ideation topic: none
- Confirmed Personas: none

## Problem / Opportunity

Publishing a completed change is expensive and non-recoverable.

Today the project test suite runs during review and runs again after the
archive merge. Coverage is measured separately at archive. When the post-merge
run fails, the archive restores the permanent corpus to its pre-archive state
and leaves a backup directory behind. The developer must then recreate the same
merge and rediscover the same failure before fixing anything. The repeated
executions add cycle time, and the rollback destroys the exact state that
produced the failure.

A developer cannot look at an archived change and know, from the change itself,
which committed state was verified, what evidence was produced, or whether the
result is safe to push.

## Proposed Solution

Archive becomes a forward-only publication with a durable proof receipt.

- Archiving consumes a completed change: it moves to its permanent archive
  location and no longer exists among active changes.
- The archive starts from a committed project state; uncommitted work blocks
  the archive with a message naming the offending files.
- The archive merges use-case documents, features, and other accepted material
  and records that merged state as the archive candidate commit. The change is
  now at its permanent location but is not yet proven.
- The full test suite runs once against the merged candidate, covering the
  regular and end-to-end scopes. Review no longer runs the full suite.
- Each scope reports its scenario counts in a machine-readable result. A
  missing, malformed, non-reconciling, or failing result blocks proof.
- Coverage is measured once against the merged candidate and published.
- A successful run records a verification receipt naming the tested commit and
  its evidence. The archive is then proven and safe to push.
- A failed run leaves the unproven candidate in place. The developer repairs
  the current committed state and runs the archive again. Nothing is rolled
  back and the merge is not repeated.
- Rerunning the archive resumes an unproven candidate and refuses an already
  proven archive.
- Finalization of a successful archive may add only the receipt, coverage
  output, and managed documentation. A change to anything else requires
  another verification run.

The existing promise that a failed archive restores the pre-archive corpus is
replaced by forward repair: a failed candidate is never reported as proven and
is never silently discarded.

## Capability Routing

`givn spec route` returned a flat ranking (all candidates scored 3.00); its
top entry was `EXTEND deduplicate-change-dir-detection`. The ranking carried no
distinct signal, so the decisions below are made on capability boundaries.

Use-case contract change: `archive-change`'s minimal guarantee changes from
"the pre-archive corpus remains intact when publication fails" to "a failed
archive is never reported as proven and its candidate remains repairable in
place."

| Proposed capability | Route's recommendation | Decision | Rationale (if you deviated from route) |
|---|---|---|---|
| archive-proof | flat ranking; top was EXTEND deduplicate-change-dir-detection | `NEW in archive-change` | The forward-only candidate, proof receipt, resume, and proven status are a new behavior unit. No existing capability owns the archive candidate lifecycle. |
| extract-coverage-gate | flat ranking | `EXTEND extract-coverage-gate` | Coverage failure changes from corpus rollback to an unproven candidate; coverage still runs once at archive. |
| enforce-post-merge-e2e-gate | flat ranking | `EXTEND enforce-post-merge-e2e-gate` | The post-merge E2E scope now runs against the archive candidate; rollback is replaced by forward repair. |
| givn-workflow-enforcement | flat ranking | `EXTEND givn-workflow-enforcement` | Archive verification moves to the candidate model and `givn check review` no longer runs the full suite. |

## Out of Scope

- Task classification changes (reuse / characterization) and the commit hash
  self-reference in task tracking.
- Scenario feasibility checklist in design review.
- Machine-readable interaction inventory and its cross-check.
- Batched verification interface (one runner process per scope remains).
- Scenario-set membership validation (scope results declare counts; Givn
  reconciles the arithmetic, not the scenario identities).
- Compile and test duration breakdown inside runner commands.
- Changes to the coverage command contract.

## Open Questions

- None.
