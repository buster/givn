# Ralph Tasks

## Integrity hook — arc42 conformance

- [x] Implement arc42 conformance check in src/integrity_hook.rs: parse arc42.md marker table, byte-compare affected chapters against embedded templates

## Post-archive verify gate

- [x] Implement post-archive verify gate in src/cli/archive.rs: backup specs before merge, run verify, clean up / report on result

## Template updates

- [x] Add `## Internal Primitives` section to assets/templates/design.md
- [x] Update assets/templates/tasks.md: change COMMIT box to inline format
- [x] Update assets/templates/review.md: add sign-off item for internal primitives and arc42 slug-mapping instruction

## Test configuration fixes

- [x] Write step definitions to set verify.command properly in test environments so integrity hook and archive scenarios pass

## Verify full suite passes

- [x] Run `cargo test --test features_runner` and confirm all 13 scenarios pass
