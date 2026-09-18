# Tasks: rename-support-fragments

Specification reorganization and one test path; `specs` is skipped.

## Setup

- [x] Change scaffolded; the fourteen names are pre-approved.

## Implementation

- [x] `git mv` the fourteen feature files; update their titles.
- [x] Rewrite `fragment.md`'s capability list, owning `customize-the-lifecycle-graph`.
- [x] Update the register-history step's fixture path.
- [x] COMMIT: `2b8dc4e6` -- `feat(use-case-model-scoping): rename the support-fragment capabilities`

## Verification

- [x] `givn lint` exits 0; no old capability name remains.
- [x] `cargo clippy --workspace --all-targets --locked -- -D warnings` clean.
- [x] `./run-tests.sh` and `./run-tests.sh --e2e` pass. Evidence:
      ```
      Summary [35.901s] 539 tests run: 539 passed, 0 skipped
      Summary [6.2s] 88 tests run: 88 passed, 451 skipped
      ```
- [x] COMMIT: `c4c60298` -- `docs(rename-support-fragments): complete the review`
- [x] Archive the change.
