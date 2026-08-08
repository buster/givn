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
    - COMMIT:   `0dd8bbc`

- [ ] preserve-docstrings-on-archive: An added scenario keeps its step docstring after archiving
    - RED:      Remove @wip from this scenario only. Single-scenario run → MUST
                exit non-zero (missing step definitions). Paste output.
    - GREEN:    Step definitions for the added-scenario docstring path.
                Files: `tests/steps/preserve_docstrings_on_archive_steps.rs`.
                (render fix already lands in scenario 1.) Run → MUST exit 0. Paste output.
    - REFACTOR: Re-run → still exits 0. Paste output.
    - COMMIT:   `hash`

- [ ] preserve-docstrings-on-archive: A modified scenario keeps its step data table after archiving
    - RED:      Remove @wip from this scenario only. Single-scenario run → MUST
                exit non-zero (missing step definitions). Paste output.
    - GREEN:    Implement data-table step definition + `render_scenario_stripped`
                table rendering. Files: `src/gherkin/merge.rs`,
                `tests/steps/preserve_docstrings_on_archive_steps.rs`. Run → MUST
                exit 0. Paste output.
    - REFACTOR: Re-run → still exits 0. Paste output.
    - COMMIT:   `hash`

---

## E2E setup: configure e2e test infrastructure, strict mode, and verify.e2e_command

- [ ] Setup: configure e2e test infrastructure
    - `verify.e2e_command` already set in `givn/config.yaml`:
      `cargo test --test features_runner -- --tags @e2e` (tag-filtered subset
      of the same runner — design.md's CLI capability has no browser/server).
    - Scenario-count proof: record full-suite scenario count vs `@e2e`
      count; e2e count must be strictly less. Paste both counts here.
    - Strict mode: `.fail_on_skipped()` already configured (same runner).

## E2e scenarios

- [ ] @e2e preserve-docstrings-on-archive: Consumer archives a modified scenario and the permanent spec keeps the docstring
    - RED:      Remove @wip from this scenario only. Run
      `GIVN_FEATURES=givn/changes/fix-archive-merge-docstrings/specs cargo test --test features_runner -- --tags @e2e`
      → MUST exit non-zero (missing step definition). Paste output.
    - GREEN:    Implement the e2e given (`a givn project with a completed
                change ... whose delta ... gives a step the docstring`) —
                sets env = make_init_root, writes the permanent-spec baseline,
                the completed change, and the fixture commands.yaml, then
                asserts via the shared `the permanent spec file for ...`
                Then step. Files: `tests/steps/preserve_docstrings_on_archive_steps.rs`.
                Run → MUST exit 0. Paste output.
    - REFACTOR: Re-run → still exits 0. Paste output.
    - COMMIT:   `hash`

---

## Post-implementation

- [ ] Run full suite `cargo test --test features_runner` → exits 0 (or record
      pre-existing unrelated failures with proof).
- [ ] Run e2e suite `cargo test --test features_runner -- --tags @e2e` → exits 0.
- [ ] Run `cargo clippy --workspace --all-targets -- -D warnings` → pass.
- [ ] Run `cargo fmt -- --check` → pass.
- [ ] Run `cargo test --test features_runner -- --name "docstring"` etc. for the
      new merge.rs unit tests → pass.
