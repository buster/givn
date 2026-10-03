# Tasks: reshape-author-change

Specification reorganization; `specs` is skipped.

## Setup

- [x] Change scaffolded; the operator approved the split and the names.

## Implementation

- [x] Write the `+` summary narrative and the two child narratives
      (`initialize-project`, `create-change`) with step-referenced
      extensions and mechanism-free Goal, Trigger, Main flow, and
      Guarantees.
- [x] `git mv` the five feature files into their new homes; rename the three
      capabilities in the Feature lines.
- [x] COMMIT: `6af09a22` -- `feat(use-case-model-scoping): split author-change into a summary and two leaves`

## Verification

- [x] `givn spec tree` shows the summary with two children and the five
      capabilities in their new homes.
- [x] `givn lint` exits 0; no reference to the old capability names remains.
- [x] `./run-tests.sh` and `./run-tests.sh --e2e` pass. Evidence:
      ```
      Summary [31.099s] 539 tests run: 539 passed, 0 skipped
      Summary [5.796s] 88 tests run: 88 passed, 451 skipped
      ```
- [x] COMMIT: `861c20ce` -- `docs(reshape-author-change): complete the review`
- [x] Archive the change.
