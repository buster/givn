# Proposal: fix-remediation-delta2delta

## Problem / Opportunity

A change whose scenarios are all brand-new in a repository that has no
committed specification history produces semantic-review candidates that
compare two of the change's own scenarios to each other (a "delta-to-delta"
pair). The review gate can never accept a classified pair of this kind:
no matter how a user classifies each candidate and writes a rationale, the
gate still reports the classification as stale and stays blocked. The only
workaround today is to reword scenarios until their similarity score drops
below the boundary, which silently discards legitimate distinctions between
scenarios that are genuinely different but share vocabulary.

This forces the first change in any fresh repository to either hide real
duplicates or abandon the semantic review gate entirely.

## Proposed Solution

When a classified candidate's two scenarios both come from the change
itself (an empty permanent history), the review gate must be able to
verify the classification against the change's own scenario state:

- If the user marks the pair as redundant and removes one of the scenarios
  from the change, the gate accepts the classification.
- If the user marks the pair as a variant and consolidates the surviving
  scenario into a parameterized example table while removing the other,
  the gate accepts the classification.
- If the user marks the pair as a genuine boundary and rewrites it so the
  pair's similarity drops strictly below the boundary and the two scenarios
  describe observably different outcomes, the gate accepts the
  classification.
- Classifying a pair as redundant without removing either scenario, or as a
  genuine boundary without an observable distinction or a below-boundary
  rewrite, stays blocked exactly as it does today.

The existing behaviour for candidates that compare a change scenario to a
committed specification scenario is unchanged.

## Out of Scope

- Changing the similarity boundary or any model evidence.
- Altering how candidates are generated or how the report is written.
- Anything that weakens the documented contract that remediation is required
  — scores alone never substitute for it.

## Open Questions

- For a redundant pair, may removing *either* of the two scenarios satisfy
  the gate, or must a specific one be removed? (Both delta scenarios are
  generated-side pairs; proposal assumes removing either is acceptable.)
