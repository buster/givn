# Tasks: coverage-script-workflow

## Setup: strict runner and command contract

- [x] RED/GREEN: The existing `cucumber-rs` runner uses `.fail_on_skipped()`
  in `tests/features_runner.rs`; `cargo test --test features_runner --no-run`
  compiled it successfully. The change adds separate e2e and non-e2e step
  modules and keeps the runner command `cargo test --test features_runner`.
- [x] GREEN: `givn/commands.yaml` now uses `./run-tests.sh`,
  `./measure-coverage.sh`, and `./merge-coverages.sh`; the root and harness
  projects contain those scripts and README guidance names the same commands.
- [x] REFACTOR: `cargo fmt` completed; the command schema, templates, durable
  Arc42 docs, and generated README guidance use `measure_command` and
  `merge_command`.
- Evidence: `cargo check` passed after the command schema and archive gate were
  implemented.
- Commit: pending

## Scenario 1: A failed merge command blocks archive and preserves the project

- [ ] RED: Run the exact scenario before the merge boundary is implemented:
  `GIVN_FEATURES=givn/changes/coverage-script-workflow/specs cargo test --test features_runner -- --name "A failed merge command blocks archive and preserves the project"`.
  Expected: non-zero because the merge failure step is not implemented.
- [ ] GREEN: Configure deterministic measurement and failing merge fixtures;
  run the scenario. Expected: `5 steps (5 passed)` and archive output identifies
  `coverage merge command failed`; the active change remains under
  `givn/changes/`.
- [ ] REFACTOR: Re-run the exact scenario after formatting and confirm the same
  five steps pass without changing behavior.
- [ ] COMMIT: Record the atomic scenario commit hash here.

## Scenario 2: An initialized project documents reusable test and coverage scripts

- [ ] RED: Run the exact scenario before the command/readme template changes:
  `GIVN_FEATURES=givn/changes/coverage-script-workflow/specs cargo test --test features_runner -- --name "An initialized project documents reusable test and coverage scripts"`.
  Expected: non-zero because the generated files do not name the project-owned
  scripts.
- [ ] GREEN: Update the command and README templates; run the scenario.
  Evidence: `1 scenario (1 passed)` and all five steps pass, including exact
  `measure_command` and `merge_command` values.
- [ ] REFACTOR: Re-run the exact scenario after regenerating configured skill
  targets and confirm it still passes.
- [ ] COMMIT: Record the atomic scenario commit hash here.

## Scenario 3: Archiving publishes line and branch badges from the merged report

- [ ] RED: Run the exact scenario before the external merge/report rendering
  path is implemented:
  `GIVN_FEATURES=givn/changes/coverage-script-workflow/specs cargo test --test features_runner -- --name "Archiving publishes line and branch badges from the merged report"`.
  Expected: non-zero because archive output does not yet use the configured
  merged report for both badges and the README summary.
- [ ] GREEN: Execute measurement and merge scripts, validate the fresh merged
  report, and render the README blocks. Evidence: `1 scenario (1 passed)`,
  merged counters `95/100` lines and `19/20` branches, and both badges are
  directly below the README title.
- [ ] REFACTOR: Re-run the exact scenario after formatting and full generated
  asset synchronization; confirm all eight steps pass.
- [ ] COMMIT: Record the atomic scenario commit hash here.

## Final verification

- [ ] `cargo test --test features_runner`
- [ ] `cargo clippy --workspace --all-targets -- -D warnings`
- [ ] `givn lint` on permanent and change specs
- [ ] `givn status --change coverage-script-workflow`
