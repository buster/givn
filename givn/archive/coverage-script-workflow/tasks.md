# Tasks: coverage-script-workflow

## Setup: strict runner and command contract

- [x] RED/GREEN: The existing `cucumber-rs` runner uses `.fail_on_skipped()`
  in `tests/features_runner.rs`; `cargo test --test features_runner --no-run`
  compiled it successfully. A temporary undefined-step feature exited non-zero
  in both modes: normal run `2 scenarios (2 failed)` and `-- --tags @e2e` run
  `1 scenario (1 failed)`, each reporting `Step doesn't match any function`.
  The change adds separate e2e and non-e2e step modules and keeps the runner
  command `cargo test --test features_runner`.
- [x] GREEN: `givn/commands.yaml` now uses `./run-tests.sh`,
  `./measure-coverage.sh`, and `./merge-coverages.sh`; the root and harness
  projects contain those scripts and README guidance names the same commands.
- [x] REFACTOR: `cargo fmt` completed; the command schema, templates, durable
  Arc42 docs, and generated README guidance use `measure_command` and
  `merge_command`.
- Evidence: `cargo check` passed after the command schema and archive gate were
  implemented.
- Commit: d256576

## Scenario 1: A failed merge command blocks archive and preserves the project

- [x] RED: Run the exact scenario before the merge boundary is implemented:
  `GIVN_FEATURES=givn/changes/coverage-script-workflow/specs cargo test --test features_runner -- --name "A failed merge command blocks archive and preserves the project"`.
  Evidence: the pre-implementation checkout exited non-zero during compilation
  because the old coverage gate could not consume the new command schema.
- [x] GREEN: Configure deterministic measurement and failing merge fixtures;
  run the scenario. Evidence: `1 scenario (1 passed)`, `5 steps (5 passed)`,
  and archive output identifies `coverage merge command failed`; the active
  change remains under `givn/changes/`.
- [x] REFACTOR: Re-ran the exact scenario after formatting; the same five steps
  passed without changing behavior.
- [x] COMMIT: `8727d02` — `fix(coverage-script-workflow): validate merged coverage rates`.

## Scenario 2: An initialized project documents reusable test and coverage scripts

- [x] RED: Run the exact scenario before the command/readme template changes:
  `GIVN_FEATURES=givn/changes/coverage-script-workflow/specs cargo test --test features_runner -- --name "An initialized project documents reusable test and coverage scripts"`.
  Evidence: the pre-implementation checkout exited non-zero during compilation
  before the new step bindings and command contract were present.
- [x] GREEN: Update the command and README templates; run the scenario.
  Evidence: `1 scenario (1 passed)` and all five steps passed, including exact
  `measure_command` and `merge_command` values.
- [x] REFACTOR: Re-ran the exact scenario after regenerating configured skill
  targets; it still passed.
- [x] COMMIT: `c21ae9c` — `feat(coverage-script-workflow): publish merged README coverage`.

## Scenario 3: Archiving publishes line and branch badges from the merged report

- [x] RED: Run the exact scenario before the external merge/report rendering
  path is implemented:
  `GIVN_FEATURES=givn/changes/coverage-script-workflow/specs cargo test --test features_runner -- --name "Archiving publishes line and branch badges from the merged report"`.
  Evidence: the pre-implementation checkout exited non-zero during compilation
  because the archive gate still expected the removed command fields.
- [x] GREEN: Execute measurement and merge scripts, validate the fresh merged
  report, and render the README blocks. Evidence: `1 scenario (1 passed)`,
  merged counters `95/100` lines and `19/20` branches, and both badges are
  directly below the README title.
- [x] REFACTOR: Re-ran the exact scenario after formatting and full generated
  asset synchronization; confirm all eight steps pass.
- [x] COMMIT: `ab0862a` — `fix(coverage-script-workflow): block failed coverage merges`.

## Final verification

- [x] `cargo test --test features_runner` — 151 scenarios, 819 steps passed.
- [x] `cargo clippy --workspace --all-targets -- -D warnings` — passed.
- [x] `givn lint` on permanent and change specs — passed after removing all
  implementation-complete `@wip` tags.
- [x] `cargo run --quiet -- status --change coverage-script-workflow` — tasks
  complete; review remains the next lifecycle artifact.

Migration compatibility is covered by `legacy_coverage_commands_are_accepted_during_migration`.
