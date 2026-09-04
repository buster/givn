# Proposal: retire-semantic-review-guidance

## Problem / Opportunity

The semantic similarity gate is being removed (see
`retire-semantic-capabilities`, already archived). Two scenarios in the
`instruction-integrity` capability still assert that the review guidance
contains semantic-review classification/remediation sections. Those
sections are being deleted from the review instructions and template
(retirement plan task 1.3), so the two scenarios describe removed
behaviour and will fail the suite once the sections are gone.

## Proposed Solution

- Remove the two semantic-remediation-assertion scenarios from
  `instruction-integrity`.
- Replace them with scenarios that assert the surviving deterministic
  gate guidance: the Overlap dispositions table and split-or-keep
  decisions — one regular scenario over the instruction text, one `@e2e`
  scenario over the CLI output.

## Out of Scope

- The remaining `instruction-integrity` scenarios (E2E guidance, specs
  metadata, overrides, retrieval guidance) are untouched.
- The semantic step definitions in `tests/steps/` still compile until the
  Cargo features are dropped (retirement plan task 1.2).

## Open Questions

None.
