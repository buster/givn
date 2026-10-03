# Tasks: reshape-inspect-change-state

Specification reorganization; `specs` is skipped.

## Setup

- [x] Change scaffolded; the four names are pre-approved.

## Implementation

- [x] Rewrite the Trigger; anchor the extension; rename the four
      capabilities.
- [x] `git mv` the four feature files and update their titles.
- [x] COMMIT: `be739527` -- `feat(use-case-model-scoping): reshape inspect-change-state to the 0.8.0 model`

## Verification

- [x] `givn lint` exits 0; no reference to the old capability names remains.
- [x] `./run-tests.sh` and `./run-tests.sh --e2e` pass. Evidence:
      ```
      Summary [38.514s] 539 tests run: 539 passed, 0 skipped
      Summary [6.702s] 88 tests run: 88 passed, 451 skipped
      ```
- [x] COMMIT: `c03e6ec6` -- `docs(reshape-inspect-change-state): complete the review`
- [x] Archive the change.
