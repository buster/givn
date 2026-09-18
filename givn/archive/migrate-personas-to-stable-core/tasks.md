# Tasks: migrate-personas-to-stable-core

Documentation repair; `specs` is skipped.

## Setup

- [x] Change scaffolded; `.givn-skip` declares `specs`.

## Implementation

- [x] Convert the three persona files to stable core + running record + topic
      stances; state the shared identity criterion in each Role line; fold
      interests, goals, and pain points into the core fields.
- [x] COMMIT: `b4f4fc89` -- `docs(persona-identity-reuse): migrate the permanent personas to the stable core`

## Verification

- [x] Each file has the three sections and the five stable-core fields.
- [x] Every dated log entry is in the running record.
- [x] `givn lint` reports no `[PERS ]` finding (0 findings).
- [x] `./run-tests.sh` and `./run-tests.sh --e2e` pass. Evidence:
      ```
      Summary [30.040s] 539 tests run: 539 passed, 0 skipped
      Summary [5.4s] 88 tests run: 88 passed, 451 skipped
      ```
- [x] COMMIT: `d58f4dc4` -- `docs(migrate-personas-to-stable-core): complete the review`
- [x] Archive the change.
