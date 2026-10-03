# Tasks: reshape-operate-product

Specification reorganization; `specs` is skipped.

## Setup

- [x] Change scaffolded; the two names are pre-approved.

## Implementation

- [x] Rewrite the Goal, Trigger, Main flow, and Success guarantee; anchor
      the extension; rename the two capabilities.
- [x] `git mv` the two feature files and update their titles.
- [x] COMMIT: `e5b1f7fe` -- `feat(use-case-model-scoping): reshape operate-product to the 0.8.0 model`

## Verification

- [x] `givn lint` exits 0; no reference to the old capability names remains.
- [x] `givn spec tree` shows the renamed capabilities.
- [x] `./run-tests.sh` and `./run-tests.sh --e2e` pass. Evidence:
      ```
      Summary [37.844s] 539 tests run: 539 passed, 0 skipped
      Summary [6.8s] 88 tests run: 88 passed, 451 skipped
      ```
- [x] COMMIT: `58f831fc` -- `docs(reshape-operate-product): complete the review`
- [x] Archive the change.
