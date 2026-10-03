# Tasks: stamp-change-seeds

Documentation repair; `specs` is skipped.

## Setup

- [x] Change scaffolded; `.givn-skip` declares `specs`.

## Implementation

- [x] Convert the seven seeds to the schema with `Status: implemented in
      <change-id>` and the five fields.
- [x] Convert the accepted decisions to `###` sections with Context,
      Options, and Chosen/Rejected; carry the open questions with their
      resolutions; keep the scope notes.
- [x] COMMIT: `9a242dee` -- `docs(ideation-skill): stamp the multi-scm-support seeds implemented`

## Verification

- [x] `givn lint` reports no `[SEED ]` finding (was seven). Evidence: `rg -c "SEED"` on the lint output reports 0.
- [x] `./run-tests.sh` and `./run-tests.sh --e2e` pass. Evidence:
      ```
      Summary [30.255s] 539 tests run: 539 passed, 0 skipped
      Summary [5.470s] 88 tests run: 88 passed, 451 skipped
      ```
- [x] COMMIT: `ce17b0e5` -- `docs(stamp-change-seeds): complete the review`
- [x] Archive the change.
