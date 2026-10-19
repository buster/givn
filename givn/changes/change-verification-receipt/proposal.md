# Proposal: change-verification-receipt

## Release note

Make a green change-scoped test run a durable, revision-bound fact that review and archive can reuse without rerunning the change suite.

## Use-Case Context

- Use-case ID: verify-change, archive-change
- Ideation topic: none — derived from a real evaluation run
- Seed path: none
- Confirmed Personas: none — operator confirmed; this is operator/agent tooling without a rendered consumer surface; the operator is a Gherkin actor, not a Persona.

## Problem / Opportunity

`givn check tasks --change <id>` runs the project test command against the change’s specs, but it does not write a durable result. `givn check review` does not run tests at all; its PASS does not prove that the change specs are green. `givn archive` merges first and runs the full merged suite only after publishing a candidate.

A red or unverified change can therefore mutate the permanent corpus before the first integration run. Archive retries caused by state, clean-tree or result-schema failures can also rerun the change suite unnecessarily. The exact runner-result contract is under-specified and was discovered one field at a time in a real trial.

## Proposed Solution

`givn check tasks` remains the only change-scoped test execution. It records a durable verification receipt containing the tested revision, the selected specs, the test command identity and the green scenario counts.

`givn check review` validates that receipt and runs no tests. If the receipt is missing, failed or stale, review refuses and tells the operator to run check tasks.

`givn archive` validates the same receipt before merging. It does not rerun the change suite. After merging and publishing the candidate, it runs the full merged suite once. A full-suite failure leaves the candidate pending; the operator repairs and reruns archive.

The receipt is invalid when the tested code revision, the change specs or the verify command configuration changed.

Every project-specific runner must be able to satisfy one small contract: run the selected features, exit non-zero on failure, and write four integer counts to the supplied result file. Givn supplies a helper and scaffolds a result writer so a from-scratch runner has a discoverable implementation path. Scope is not a required runner field; givn labels the invocation.

## Capability Routing

| Proposed capability | Route's recommendation | Decision | Rationale (if you deviated from route) |
|---|---|---|---|
| givn-workflow-enforcement | `EXTEND givn-workflow-enforcement` (verify-change, 23.50) | `EXTEND givn-workflow-enforcement` | The existing workflow-enforcement capability owns `check tasks` and `check review`; the receipt is a stronger proof inside that capability. |
| archive-proof | not ranked | `EXTEND archive-proof` | Archive-proof owns the pending/proven receipt and validates it before merge. |

## Out of Scope

- Replacing project-specific test runners with a givn runner.
- Requiring a fixed language or test framework.
- Removing the full merged-suite run.
- Making a failed full-suite run roll back the archive candidate.
- Changing the independent evaluation harness.

## Open Questions

### D1 — Must check tasks require a clean committed tree?

- Question: Should `givn check tasks` refuse to run when non-evidence files are uncommitted?
- Options:
  1. **Require clean** — the receipt binds to a revision and cannot be ambiguous. Gain: exact provenance. Cost: the agent must commit before checking tasks. In practice: check tasks runs after the implementation commit.
  2. **Record a dirty-tree digest** — allow uncommitted code and bind to a working-tree hash. Gain: fewer commits during TDD. Cost: more complex staleness checks. In practice: the receipt records dirty paths and archive refuses if they change.
- Disposition: answered — require a clean tree except declared evidence and verification paths. The design defines the exact exemption set.

### D2 — Is the `scope` field required from runners?

- Question: Must every runner write `scope` into the result JSON?
- Options:
  1. **Require scope** — the result is self-describing. Gain: cross-scope mistakes are visible. Cost: every runner must learn and implement an extra field. In practice: a change-scoped run writes `"scope":"change"`.
  2. **Make scope optional** — givn knows which invocation and path it supplied. Gain: the runner contract is four integers. Cost: none for correctness. In practice: givn labels the receipt; the runner writes only counts.
- Disposition: answered — optional. New runners write four counts; existing runners that emit scope remain compatible.

### D3 — Does the full suite duplicate the change run?

- Question: Should archive avoid the full suite because the change specs already passed?
- Options:
  1. **Run the full suite once** — test integration with pre-existing specs and shared steps. Gain: regression detection. Cost: the change scenarios execute again inside the merged suite. In practice: a new step definition conflicts with an old one; only the merged run catches it.
  2. **Trust the change receipt** — skip the merged suite. Gain: faster. Cost: no integration proof. In practice: a change passes alone and breaks an old scenario after merge.
- Disposition: answered — run the full suite once. The change-scoped run and the integration run answer different questions.
