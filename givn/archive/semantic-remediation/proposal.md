# Proposal: semantic-remediation

## Problem / Opportunity

The semantic review report identifies similar scenarios and records an agent's
classification, but a classification alone does not prove that the current
change actually resolves the duplication or makes a valid boundary explicit.
An agent could mark a candidate as resolved while leaving the projected
permanent behavior unchanged. That would turn review evidence into an
unverified assertion.

## Proposed Solution

Extend change review so every resolved semantic classification is checked
against the observable change that follows it:

- A `DUPLIKAT` classification requires the redundant scenario to be removed
  from the projected permanent behavior.
- A `VARIANTE` classification requires the overlapping scenarios to be
  consolidated or parameterized so the projected behavior no longer keeps two
  copies of the same contract.
- A `VALID-BOUNDARY` classification requires both scenarios to remain, their
  observable distinction to be explicit, and their similarity to fall below
  the active boundary.
- A classification without the corresponding remediation remains blocking and
  reports what evidence is missing.
- If an endpoint changes before remediation is verified, the previous report
  decision becomes stale and the agent must classify the regenerated candidate
  again.
- Givn verifies the projected result but never edits feature files or chooses a
  classification for the agent.

Successful remediation allows the change review to pass while preserving the
existing non-mutation, rationale, permanent-only index, archive, and advisory
review contracts.

## Out of Scope

- Automatic edits to feature files.
- Permanent-permanent corpus auditing.
- BGE, NLI, or a new semantic classifier.
- Changing the E5 boundary or classification vocabulary.
- Replacing the existing advisory `givn spec review --change` command.

## Open Questions

None. The existing `DUPLIKAT`, `VARIANTE`, and `VALID-BOUNDARY` vocabulary,
agent-owned edits, and below-boundary requirement are the governing decisions
for this slice.
