# Proposal: retire-semantic-retrieval-vocabulary

## Problem / Opportunity

Task 1.4 of the retirement plan deletes the `## Semantic review and
retrieval vocabulary` section from the specs instruction. Two live
scenarios pin text that only existed inside that section:

- `instruction-integrity`: "the CLI output distinguishes available and
  unavailable retrieval behavior" asserts `givn spec index` and
  `retrieval-unavailable` appear in the specs instruction.
- `skill-rewrites`: "Spec instructions use deterministic gates before
  retrieval exists" asserts the instruction contains "deterministic gates"
  (the deleted section's opening phrase).

With the section gone, both scenarios fail against removed behaviour.

## Proposed Solution

- Modify the `instruction-integrity` scenario: the specs instruction must
  no longer advertise retrieval commands (`givn spec index`) — assert
  their absence instead of their presence.
- Modify the `skill-rewrites` scenario: replace the assertion on the
  deleted section's phrase with one on surviving content (the User
  Interaction Inventory policy), keeping the "does not contain
  `Run `givn spec index``" assertion which still holds and is now the
  load-bearing one.

## Out of Scope

- Any new retrieval guidance; deterministic discovery guidance arrives
  with Phase 4 of the retirement plan.

## Open Questions

None.
