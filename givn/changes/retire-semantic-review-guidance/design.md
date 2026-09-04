# Design: retire-semantic-review-guidance

## Overview

No new architecture. This change replaces two `instruction-integrity`
scenarios that assert semantic-review guidance (deleted with the semantic
gate) with scenarios asserting the surviving deterministic gate guidance.

## Technology decisions

None. The delta modifies one capability's spec: two `@givn.removed`
scenarios, two `@givn.added` replacements (one regular, one `@e2e`).
Step definitions for the new scenarios read the review instruction and
CLI output and assert the `## Deterministic gate dispositions` content.

## Step Definitions

| Cap | File |
|---|---|
| instruction-integrity | `tests/steps/instruction_integrity_steps.rs` |
| instruction-integrity | `tests/steps/instruction_integrity_e2e_steps.rs` |

## Architecture Impact

None. Guidance content only. The semantic step definitions elsewhere in
`tests/steps/` still compile until the Cargo features are dropped (task
1.2 of the retirement plan).
