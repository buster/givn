# Design: fix-aggregate-plan-wrapping

## Overview

Restore the four missing spaces in the proposal wrapper and pin them with a
unit test.

## Exact file changes

| Path | Change |
|---|---|
| `src/cli/migration.rs` | Four trailing spaces in `aggregate_migration_proposal`; new test `aggregate_proposal_preserves_word_spacing` |

## Procedure

1. `to the\` → `to the \`, `recorded in\` → `recorded in \`,
   `work\` → `work \`, `It must\` → `It must \`.
2. Add the test: generate a one-bundle proposal and assert the spaced
   phrases are present and the joined forms are absent.

## Verification

- `cargo test --locked --lib migration` passes with the new test.
- `cargo fmt`, `cargo clippy --workspace --all-targets --locked -- -D
  warnings`, `./run-tests.sh`, and `./run-tests.sh --e2e` pass.

## Step Definitions

N/A — `specs` is skipped; the unit test is the regression proof.

## Strict Mode (mandatory)

Inherited unchanged from `tests/features_runner.rs`
(`.fail_on_skipped()`); no step is added.

## Local Runnability & Digital Twins (mandatory)

N/A — the change edits a generated string and its unit test.

## Internal Primitives

None.

## Use-case traceability

- Use-case ID: none (project-maintenance repair; `specs` skipped)
- Affected capability: the aggregate migration plan's generated proposal
- Confirmed Personas: none (no user-visible surface)

## Open Technical Questions

None.
