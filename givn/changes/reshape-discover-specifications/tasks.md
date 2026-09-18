# Tasks: reshape-discover-specifications

Specification reorganization; `specs` is skipped.

## Setup

- [x] Change scaffolded; the name is pre-approved.

## Implementation

- [x] Rewrite the Trigger; anchor the extension; rename the capability.
- [x] `git mv` the feature file and update its title.
- [ ] COMMIT: `<hash>` -- `feat(use-case-model-scoping): reshape discover-specifications to the 0.8.0 model`

## Verification

- [x] `givn lint` exits 0; no reference to the old capability name remains.
- [x] `./run-tests.sh` and `./run-tests.sh --e2e` pass. Evidence:
      ```
      Summary [33.739s] 539 tests run: 539 passed, 0 skipped
      Summary [6.7s] 88 tests run: 88 passed, 451 skipped
      ```
- [ ] COMMIT: `<hash>` -- `docs(reshape-discover-specifications): complete the review`
- [ ] Archive the change.
