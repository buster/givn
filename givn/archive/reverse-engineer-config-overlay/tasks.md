# Tasks: reverse-engineer-config-overlay

## Task 0 — Test Infrastructure

- [x] Verify `cargo test --test features_runner` exits 0 on existing specs.
- [x] Confirm `givn lint --change reverse-engineer-config-overlay` exits 2 (@wip).

---

## Scenario 1: Empty overlay returns base unchanged

- [x] Remove `@wip`. Write step def `the overlay "..." is applied to the default manifest`. Calls `overlay::apply(Manifest::load_embedded()?, yaml)`. Write `the effective manifest equals the default manifest` — compares field-by-field. GREEN on first run.

## Scenario 2: Overlay replaces verify command

- [x] Remove `@wip`. Write `the effective manifest has verify command "..."` step. GREEN.

## Scenario 3: Overlay replaces apply requires

- [x] Remove `@wip`. Write `the effective manifest has apply requires exactly "..."` step. GREEN.

## Scenario 4: Overlay adds a new artifact

- [x] Remove `@wip`. Write `the effective manifest has an artifact "..." that requires "..."` step. GREEN.

## Scenario 5: Overlay removes an artifact

- [x] Remove `@wip`. Write `the effective manifest does not have an artifact "..."` step. GREEN. Spec corrected: overlay must also update `apply.requires` when removing an artifact referenced there.

## Scenario 6: Overlay marks artifact as optional

- [x] Remove `@wip`. Write `the effective manifest has an artifact "..." that is optional` step. GREEN.

## Scenario 7: Overlay deep-merges fields

- [x] Remove `@wip`. Write `the effective manifest has an artifact "..." that generates "..."` step. GREEN.

## Scenario 8: Arc42 addon expansion

- [x] Remove `@wip`. Reuse artifact-exists + optional steps. GREEN.

## Scenario 9: Arc42 idempotent

- [x] Remove `@wip`. Write `applied twice` step — calls apply() twice. Write `exactly one artifact "..."` step. GREEN.

## Scenario 10: Extras flag propagation

- [x] Remove `@wip`. Write `the effective manifest has extras enabled` step. GREEN.

## Scenario 11: Cycle rejection

- [x] Remove `@wip`. Write `the overlay application fails with "..."` step — asserts Err contains substring. GREEN.

## Scenario 12: Dangling require rejection

- [x] Remove `@wip`. Reuses `fails with` step. GREEN.

## Scenario 13: apply.requires removed artifact rejection

- [x] Remove `@wip`. Reuses `fails with` step. GREEN.

## Post-implementation

- [x] Run `givn lint --change reverse-engineer-config-overlay` — exit 0.
- [x] Run `cargo test --test features_runner` — all GREEN.
- [x] Run `givn apply --change reverse-engineer-config-overlay`.
