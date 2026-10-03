# Tasks: reshape-maintain-project

Specification reorganization; `specs` is skipped.

## Setup

- [x] Change scaffolded; the operator approved the three-child split.

## Implementation

- [x] Write the `+` summary and the three child narratives with
      step-anchored extensions and mechanism-free Goal, Trigger, Main flow,
      and Guarantees.
- [x] Move `readme-enforcement.feature`; split `release-0-3-0.feature` into
      `apply-a-managed-upgrade.feature` and `describe-the-release.feature`.
- [x] COMMIT: `c2da5b76` -- `feat(use-case-model-scoping): split maintain-project into a summary and three leaves`

## Verification

- [x] `givn spec tree` shows the summary and three children.
- [x] `givn lint` exits 0; no reference to `release-0-3-0` remains.
- [x] `./run-tests.sh` and `./run-tests.sh --e2e` pass. Evidence:
      ```
      Summary [36.676s] 539 tests run: 539 passed, 0 skipped
      Summary [6.638s] 88 tests run: 88 passed, 451 skipped
      ```
- [x] COMMIT: `e4aedccd` -- `docs(reshape-maintain-project): complete the review`
- [x] Archive the change.
