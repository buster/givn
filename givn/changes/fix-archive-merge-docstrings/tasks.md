# Tasks: fix-archive-merge-docstrings

## Setup: configure test infrastructure, strict mode, and verify.command

- [x] Setup
    - `.fail_on_skipped()` already present in `tests/features_runner.rs:20`.
    - `verify.command` already set to `cargo test --test features_runner` in `givn/config.yaml`.
    - Strict mode proof: scenario 1 RED run exited 101 ("Step doesn't match any function")
      — undefined steps fail. `cargo test --test features_runner` full suite: 125 scenarios / 663 steps pass.

---

## Fix: preserve step docstrings and data tables in the archive merge

### Design constraints
- Fix `render_scenario_stripped` in `src/gherkin/merge.rs` only — the
  step-rendering loop currently emits `{keyword} {value}` and drops
  `step.docstring` / `step.table`.
- Render per step, in order: the docstring block (`"""` delimiters at 6-space
  indent, each content line re-indented to 6 spaces, empty content still
  emits the two delimiter lines), then data-table rows (`| a | b |` at
  6-space indent). See design.md for the template.
- Fixture steps (Given) write: permanent spec baseline
  `givn/specs/<cap>/<cap>.feature`, completed change with all archive-gate
  artifacts, the delta feature at `givn/changes/<id>/specs/<cap>/<cap>.feature`
  tagged `@givn.delta @<cap>` and the scenario `@givn.modified`/`@givn.added`,
  and `givn/commands.yaml` per design.md (verify `true`, e2e empty, coverage
  commands `true`, distinct dummy Cobertura XMLs). Fixture deltas carry no
  `@e2e`.
- New step definitions: `tests/steps/preserve_docstrings_on_archive_steps.rs`
  (one file per capability); register in `tests/steps/mod.rs`. All new regex
  steps fully anchored `^...$`.
- Unit tests: in `src/gherkin/merge.rs` tests module — modified keeps
  docstring, added keeps docstring, empty docstring emits delimiters, table
  preserved, docstring+table renders docstring first.
- Single-scenario command:
  `GIVN_FEATURES=givn/changes/fix-archive-merge-docstrings/specs cargo test --test features_runner -- --name "<title regex>"`

- [x] preserve-docstrings-on-archive: A modified scenario keeps its step docstring after archiving
    - RED:      Removed @wip. `... -- --name "A modified scenario keeps its step docstring after archiving"`
                → exit 101, "Step doesn't match any function" (undefined steps fail).
    - GREEN:    Implemented `render_scenario_stripped` docstring rendering in
                `src/gherkin/merge.rs`; added step defs in
                `tests/steps/preserve_docstrings_on_archive_steps.rs`; registered module in
                `tests/steps/mod.rs`. Single-scenario run → exit 0 (6 steps, 1 passed). 13 merge unit tests pass.
    - REFACTOR: `cargo clippy --workspace --all-targets -- -D warnings` clean; `cargo fmt -- --check` clean;
                re-run → still exit 0.
    - [x] COMMIT: `0dd8bbc`

- [x] preserve-docstrings-on-archive: An added scenario keeps its step docstring after archiving
    - RED:      Removed @wip. Single-scenario run → PASSED immediately (step
                definitions and render fix already landed in scenario 1's GREEN;
                genuine reuse). No RED failure needed.
    - GREEN:    No new production code (reuse of `render_scenario_stripped`
                docstring rendering + `completed_change_adds_with_docstring` step).
                Files: none new. Single-scenario run → exit 0 (6 steps, 1 passed).
    - REFACTOR: Re-run → still exit 0.
    - [x] COMMIT: `b7d7b35`

- [x] preserve-docstrings-on-archive: A modified scenario keeps its step data table after archiving
    - RED:      Removed @wip. Single-scenario run → PASSED immediately (data-table
                rendering + step definition shipped with scenario 1's GREEN as the
                same coherent render fix; `modified_scenario_keeps_step_data_table`
                unit test pins the table path).
    - GREEN:    No new production code (reuse). Single-scenario run → exit 0
                (6 steps, 1 passed). Assertion targets the on-disk merged spec
                containing the table cell text — non-vacuous.
    - REFACTOR: Re-run → still exit 0.
    - [x] COMMIT: `9719526`

---

## E2E setup: configure e2e test infrastructure, strict mode, and verify.e2e_command

- [x] Setup: configure e2e test infrastructure
    - `verify.e2e_command` already set in `givn/config.yaml`:
      `cargo test --test features_runner -- --tags @e2e` (tag-filtered subset
      of the same runner — design.md's CLI capability has no browser/server).
    - Scenario-count proof: full suite `cargo test --test features_runner`
      → 125 scenarios; `-- --tags @e2e` → 9 scenarios. 9 < 125 — subset
      proven.
    - Strict mode: `.fail_on_skipped()` already configured (same runner);
      RED runs exit non-zero on undefined steps.

## E2e scenarios

- [x] @e2e preserve-docstrings-on-archive: Consumer archives a modified scenario and the permanent spec keeps the docstring
    - RED:      Removed @wip. Run
      `GIVN_FEATURES=.../specs cargo test --test features_runner -- --tags @e2e`
      → PASSED immediately (e2e given + Then shipped with scenario 1's GREEN;
      reuse).
    - GREEN:    Reuse — `e2e_project_with_completed_change` given + shared
                `the permanent spec file for ... contains the text ...` Then.
                Files: none new (`tests/steps/preserve_docstrings_on_archive_steps.rs`).
                Run → exit 0 (4 steps, 1 passed). Asserts on the on-disk merged
                permanent spec after the real `givn archive` subprocess — the
                CLI's output, not a DB.
    - REFACTOR: Re-run → still exit 0.
    - [x] COMMIT: `8c2c97c`

---

## Post-implementation

- [x] Run full suite `cargo test --test features_runner` → 125 scenarios / 663 steps pass.
- [x] Run e2e suite `cargo test --test features_runner -- --tags @e2e` → 9 scenarios pass.
- [x] Run `cargo clippy --workspace --all-targets -- -D warnings` → pass.
- [x] Run `cargo fmt -- --check` → pass.
- [x] `cargo test --lib gherkin::merge` → 13 tests pass (incl. new docstring/table/empty-docstring/combo tests).
