# Tasks: fix-aggregate-plan-wrapping

Product fix with a unit test; `specs` is skipped.

## Setup

- [x] Change scaffolded; `.givn-skip` declares `specs`.

## Implementation

- [x] Add the four missing spaces in `aggregate_migration_proposal`.
- [x] Add `aggregate_proposal_preserves_word_spacing`.
- [x] COMMIT: `7dcac69a` -- `fix(migration): preserve word spacing in the aggregate proposal wrapper`

## Verification

- [x] `cargo test --locked --lib migration` passes (12 tests).
- [x] `cargo fmt` and `cargo clippy --workspace --all-targets --locked -- -D warnings` clean.
- [x] `./run-tests.sh` and `./run-tests.sh --e2e` pass. Evidence:
      ```
      Summary [30.056s] 539 tests run: 539 passed, 0 skipped
      Summary [5.350s] 88 tests run: 88 passed, 451 skipped
      ```
- [x] COMMIT: `c1764946` -- `docs(fix-aggregate-plan-wrapping): complete the review`
- [x] Archive the change.
