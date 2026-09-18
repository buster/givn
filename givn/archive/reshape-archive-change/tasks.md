# Tasks: reshape-archive-change

Specification reorganization; `specs` is skipped.

## Setup

- [x] Change scaffolded; the operator approved the rename and the include
      removal.

## Implementation

- [x] Rewrite the Goal and Success guarantee; replace mechanism vocabulary;
      rewrite the eight extensions; remove the `usecase: verify-change`
      include; rename the capability.
- [x] `git mv` the feature file and update its title.
- [x] COMMIT: `456040ec` -- `feat(use-case-model-scoping): reshape archive-change to the 0.8.0 model`

## Verification

- [x] `givn lint` exits 0; no reference to the old capability name remains.
- [x] `givn spec tree` shows the renamed capability.
- [x] `./run-tests.sh` and `./run-tests.sh --e2e` pass. Evidence:
      ```
      Summary [30.648s] 539 tests run: 539 passed, 0 skipped
      Summary [5.497s] 88 tests run: 88 passed, 451 skipped
      ```
- [x] COMMIT: `8c58846e` -- `docs(reshape-archive-change): complete the review`
- [x] Archive the change.
