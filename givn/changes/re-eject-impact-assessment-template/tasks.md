# Tasks: re-eject-impact-assessment-template

Documentation repair; `specs` is skipped.

## Setup

- [x] Change scaffolded; `.givn-skip` declares `specs`.

## Implementation

- [x] `givn eject arc42-docs:template --force`. Evidence: `ejected givn/artifacts/arc42-docs/impact-assessment-template.md`.
- [ ] COMMIT: `<hash>` -- `fix(adr-qualification-gate): re-eject the impact assessment template from 0.8.0`

## Verification

- [x] `cmp` proves the ejected file is byte-identical to the managed asset.
- [x] The structured verdict block is present. Evidence: one match for `qualification: <blank until recorded>`.
- [x] `./run-tests.sh` and `./run-tests.sh --e2e` pass. Evidence:
      ```
      Summary [30.717s] 539 tests run: 539 passed, 0 skipped
      Summary [5.437s] 88 tests run: 88 passed, 451 skipped
      ```
- [ ] COMMIT: `<hash>` -- `docs(re-eject-impact-assessment-template): complete the review`
- [ ] Archive the change.
