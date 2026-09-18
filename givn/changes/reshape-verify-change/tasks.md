# Tasks: reshape-verify-change

Specification reorganization; `specs` is skipped.

## Setup

- [x] Change scaffolded; the three names are pre-approved.

## Implementation

- [x] Rewrite the Trigger, Main flow, and Success guarantee; anchor both
      extensions; rename the three capabilities.
- [x] `git mv` the three feature files and update their titles.
- [x] COMMIT: `de080707` -- `feat(use-case-model-scoping): reshape verify-change to the 0.8.0 model`

## Verification

- [x] `givn lint` exits 0; no reference to the old capability names remains.
- [x] `givn spec tree` shows the renamed capabilities.
- [x] `./run-tests.sh` and `./run-tests.sh --e2e` pass. Evidence:
      ```
      Summary [36.607s] 539 tests run: 539 passed, 0 skipped
      Summary [6.837s] 88 tests run: 88 passed, 451 skipped
      ```
- [x] COMMIT: `5421d2ca` -- `docs(reshape-verify-change): complete the review`
- [x] Archive the change.
