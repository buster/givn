# Tasks: resync-adr-template

Documentation repair; `specs` is skipped. The design's procedure is the plan.

## Setup

- [x] Change scaffolded; `.givn-skip` declares `specs`.

## Implementation

- [x] Delete the stale `docs/arc42/adr/adr-template.md`.
- [x] Re-scaffold with `givn addons enable arc42`; confirm only the template
      is created. Evidence: `created  docs/arc42/adr/adr-template.md`; all other chapters `exists`.
- [x] COMMIT: `bf3f66ea` -- `fix(adr-qualification-gate): restore the blank ADR template from the 0.8.0 asset`

## Verification

- [x] `cmp` proves the restored file is byte-identical to the managed asset.
- [x] No `ADR-QUALIFICATION: PASS` or `qualification: QUALIFIED` remains.
- [x] `./run-tests.sh` and `./run-tests.sh --e2e` pass. Evidence:
      ```
      Summary [30.580s] 539 tests run: 539 passed, 0 skipped
      Summary [5.410s] 88 tests run: 88 passed, 451 skipped
      ```
- [ ] COMMIT: `<hash>` -- `docs(resync-adr-template): complete the review`
- [ ] Archive the change.
