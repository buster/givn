# Tasks: retire-semantic-adr-residue

Documentation and register repair with one modified scenario.

## Setup

- [x] Change scaffolded; the spec delta adds one `@givn.modified` scenario.

## Implementation

- [x] `git mv` ADR-044, ADR-045, and ADR-054 to `docs/arc42/adr/archive/`
      and set `status: superseded by ADR-069` in each.
- [x] Append the three records to ADR-069's supersession list.
- [x] Update the register README: count 28 → 25, rows moved, relationship
      line extended.
- [x] Delta the register-history scenario to 25 accepted decisions
      (`specs/fragments/adr-qualification-gate.feature`); the existing
      25-count step is reused, so no Rust change is needed.
- [x] COMMIT: `74e90cb1` -- `fix(adr-qualification-gate): retire the semantic-engine ADR residue`

## Verification

- [x] `rg -n "status: accepted" docs/arc42/adr/ADR-*.md | wc -l` reports 25.
- [x] The README's active rows no longer list the three; the archive and
      relationship lines do.
- [x] `./run-tests.sh` and `./run-tests.sh --e2e` pass. Evidence:
      ```
      Summary [30.459s] 539 tests run: 539 passed, 0 skipped
      Summary [5.343s] 88 tests run: 88 passed, 451 skipped
      ```
- [x] COMMIT: `97b2504e` -- `docs(retire-semantic-adr-residue): complete the review`
- [x] Archive the change.
