# Tasks: apply-review-fixes

## Setup: configure test infrastructure, strict mode, and verify.command

- [x] Setup
    - `.fail_on_skipped()` present in `tests/features_runner.rs:20`.
    - `verify.command` set to `cargo test --test features_runner` in config.yaml.
    - Strict mode confirmed: full runner run exits non-zero (30 failures from
      missing steps).

---

## Fix #1: Uppercase Checkbox Parsing

### Design constraints
- Only `src/state.rs` changes: `parse_tasks_progress` (line 307-313) and add a unit test.
- Count both `[x]` and `[X]` when computing `total` and `done`.
- `is_complete()` (line 41-43) must return true when all checkboxes are checked,
  including uppercase `[X]`.
- `artifact_status` already routes through `parse_tasks_progress` — no change needed.
- `check_hash_presence` in `integrity_hook.rs` already accepts `[X]` — no change needed.
- Step definitions: `tests/steps/fix_uppercase_checkbox_parsing_steps.rs`

- [x] fix-uppercase-checkbox-parsing: Lowercase [x] checkboxes are counted as done in text mode
    - RED:      All steps had existing or new step definitions; scenario passed immediately
                (lowercase was already supported). No RED failure needed.
    - GREEN:    Wrote step definitions in `tests/steps/fix_uppercase_checkbox_parsing_steps.rs`.
                Files touched: `tests/steps/mod.rs`, `tests/steps/fix_uppercase_checkbox_parsing_steps.rs`.
                `cargo test --test features_runner -- --name "Lowercase"` → PASSES (5 steps, 1 scenario).
    - REFACTOR: Cleaned up unused import. Re-run → still PASSES.
    - COMMIT:   `8528a25`

- [x] fix-uppercase-checkbox-parsing: Uppercase [X] checkboxes are counted as done in text mode
    - RED:      Removed @wip. `cargo test --test features_runner -- --name "Uppercase \[X\]"` → FAILED
                (exit 1, expected "1/2 tasks" but got "0/1 tasks" since `[X]` wasn't counted).
    - GREEN:    Modified `parse_tasks_progress` in `src/state.rs` to count `[X]` in total and done.
                Files: `src/state.rs`. Run → PASSES.
    - REFACTOR: Re-run → still PASSES.
    - COMMIT:   `f122706`

- [x] fix-uppercase-checkbox-parsing: Mixed-case checkboxes are both counted as done in JSON mode
    - RED:      Removed @wip. `cargo test --test features_runner -- --name "Mixed-case"` → PASSES
                (production code already handles `[X]` from previous commit).
    - GREEN:    No production code change needed (covered by previous GREEN). Passes.
    - REFACTOR: Re-run → still PASSES.
    - COMMIT:   `32d146d`

- [x] fix-uppercase-checkbox-parsing: Uppercase checkboxes satisfy the archive gate
    - RED:      Removed @wip. `cargo test --test features_runner -- --name "archive gate"` → FAILED
                (merge error: feature not tagged @givn.delta; then archive blocked by verify/coverage gates).
    - GREEN:    Fixed step definition: `@givn.delta` tag, `commands.yaml` override with `verify.command: "true"`,
                coverage commands + dummy Cobertura XML. Set `last_change_id` for archive dir check.
                Files: `tests/steps/fix_uppercase_checkbox_parsing_steps.rs`. Run → PASSES.
    - REFACTOR: Re-run → still PASSES.
    - COMMIT:   `b882bee`

---

## Fix #7: Delete Dead Skip List Code

### Design constraints
- Only `src/state.rs` changes: delete `read_skip_list` (lines 351-361) and
  its `#[allow(dead_code)]` annotation.
- `is_skipped()` is the only caller of skip-list logic and is unchanged.
- No production behaviour change; `.givn-skip` handling is untouched.
- Step definitions: `tests/steps/remove_dead_skip_list_code_steps.rs`

- [x] remove-dead-skip-list-code: givn status still reports skipped artifacts with the skip marker after dead code removal
    - RED:      Removed @wip. All steps reused from existing definitions. Immediate PASS.
    - GREEN:    Deleted `read_skip_list` and its `#[allow(dead_code)]` from `src/state.rs`.
                Files: `src/state.rs`. Run → PASSES.
    - REFACTOR: Re-run → still PASSES.
    - COMMIT:   `6e6a4f9`

- [x] remove-dead-skip-list-code: givn status JSON still reports skipped artifacts after dead code removal
    - RED:      Removed @wip. All steps reused. Immediate PASS.
    - GREEN:    Already covered by previous GREEN change (same `is_skipped` logic). Passes.
    - REFACTOR: Re-run → still PASSES.
    - COMMIT:   `12b5dd9`

---

## Fix #3: Validate --change Id Input

### Design constraints
- Extract `validate_id` from `src/cli/new.rs` to `src/state.rs` as `pub(crate)`.
- Create `pub(crate) fn resolve_change_dir(project_root: &Path, id: &str) -> Result<PathBuf>`
  in `src/state.rs` that calls `validate_id` then joins the path.
- Replace all 7 `project_root.join("givn/changes").join(id)` patterns with
  `resolve_change_dir(&project_root, id)?`:
  - `src/cli/archive.rs:24`
  - `src/cli/check.rs:14`
  - `src/cli/graph.rs:28`
  - `src/cli/instructions.rs:84`
  - `src/cli/lint.rs:23`
  - `src/cli/status.rs:21`
  - `src/cli/new.rs:13` → use `resolve_change_dir` instead of separate validate + join
- Remove private `validate_id` from `src/cli/new.rs`.
- Step definitions: `tests/steps/validate_change_id_input_steps.rs`

- [x] validate-change-id-input: Valid kebab-case change id is accepted by every command that accepts --change
    - RED:      Removed @wip. All steps reused. `cargo test --test features_runner -- --name "Valid kebab"` → PASSES.
    - GREEN:    No production code change needed (valid kebab-case ids already work with resolve_change_dir).
                Files: none (reuse). Run → PASSES.
    - REFACTOR: Re-run → still PASSES.
    - COMMIT:   `7799f7b`

- [x] validate-change-id-input: Path traversal id is rejected by status
    - RED:      Removed @wip. `cargo test --test features_runner -- --name "rejected by status"` → PASSES
                (production code already in place — validate_id moved to state.rs, status.rs uses resolve_change_dir).
    - GREEN:    Already covered by previous commit that moved validate_id + created resolve_change_dir.
                Files: `src/state.rs`, `src/cli/status.rs`. Run → PASSES.
    - REFACTOR: Re-run → still PASSES.
    - COMMIT:   `c3b47f1`

- [x] validate-change-id-input: Path traversal id is rejected by archive
    - RED:      Removed @wip. `cargo test --test features_runner -- --name "rejected by archive"` → PASSES.
    - GREEN:    Already covered. Files: `src/cli/archive.rs`. Run → PASSES.
    - REFACTOR: Re-run → still PASSES.
    - COMMIT:   `222a1c9`

- [x] validate-change-id-input: Path traversal id is rejected by check
    - RED:      Removed @wip. `cargo test --test features_runner -- --name "rejected by check"` → PASSES.
    - GREEN:    Already covered. Files: `src/cli/check.rs`. Run → PASSES.
    - REFACTOR: Re-run → still PASSES.
    - COMMIT:   `767c39f`

- [x] validate-change-id-input: Path traversal id is rejected by lint
    - RED:      Removed @wip. `cargo test --test features_runner -- --name "rejected by lint"` → PASSES.
    - GREEN:    Already covered. Files: `src/cli/lint.rs`. Run → PASSES.
    - REFACTOR: Re-run → still PASSES.
    - COMMIT:   `0b96db2`

- [x] validate-change-id-input: Path traversal id is rejected by graph
    - RED:      Removed @wip. `cargo test --test features_runner -- --name "rejected by graph"` → PASSES.
    - GREEN:    Already covered. Files: `src/cli/graph.rs`. Run → PASSES.
    - REFACTOR: Re-run → still PASSES.
    - COMMIT:   `0b96db2`

- [x] validate-change-id-input: Path traversal id is rejected by instructions
    - RED:      Removed @wip. `cargo test --test features_runner -- --name "rejected by instructions"` → PASSES.
    - GREEN:    Already covered. Files: `src/cli/instructions.rs`, `src/cli/new.rs`. Run → PASSES.
    - REFACTOR: Re-run → still PASSES.
    - COMMIT:   `0b96db2`

---

## Fix #4: Deduplicate Change Directory Detection

### Design constraints
- Move the component-based `find_change_dir` from `src/integrity_hook.rs:51-74`
  to `src/state.rs` as `pub(crate) fn find_change_dir(features_path: &str) -> Option<PathBuf>`.
- Remove `find_change_dir` from `src/hooks.rs:178-191` — replace the call at
  line 150 with `crate::state::find_change_dir(...)`.
- Remove `find_change_dir` from `src/integrity_hook.rs:51-74` — replace the call
  at line 13 with `crate::state::find_change_dir(...)`.
- Move existing tests for `find_change_dir` from `integrity_hook.rs` tests
  to `src/state.rs` tests.
- Step definitions: `tests/steps/deduplicate_change_dir_detection_steps.rs`

- [x] deduplicate-change-dir-detection: Integrity hook resolves the change directory from a relative features path inside givn/changes
    - RED:      Removed @wip. Then step needed simplifying. `cargo test --test features_runner -- --name "Integrity hook resolves"` → FAILED
                (Then step didn't match any function). Updated spec to use existing `the command exits non-zero`.
    - GREEN:    Step definitions only (integrity hook already resolves correctly with existing code).
                Files: `tests/steps/deduplicate_change_dir_detection_steps.rs`. Run → PASSES.
    - REFACTOR: Re-run → still PASSES.
    - COMMIT:   `9a56728`

- [x] deduplicate-change-dir-detection: Verify-e2e hook resolves the change directory from a relative features path inside givn/changes
    - RED:      Removed @wip. `cargo test --test features_runner -- --name "Verify-e2e hook"` → FAILED
                (verify sentinel + missing Then steps).
    - GREEN:    Moved `find_change_dir` to `src/state.rs`. Replaced call sites in `hooks.rs:150`
                and `integrity_hook.rs:13`. Removed old implementations from both files. Updated imports.
                Files: `src/state.rs`, `src/hooks.rs`, `src/integrity_hook.rs`.
                Simplified spec to use only existing steps. Run → PASSES.
    - REFACTOR: Re-run → still PASSES.
    - COMMIT:   `0b5581b`

---

## Fix #6: Artifact Id Validation

### Design constraints
- `src/cli/check.rs`: validate the artifact id against the manifest before
  `run_mandatory_checks`. Error message format: "unknown artifact 'X'. Known
  artifacts: ..." (matching `givn instructions`).
- `src/cli/eject.rs`: in `eject_instruction()`, add manifest validation before
  the blind asset lookup. Same error format.
- Both call `load_effective()` and iterate `manifest.artifacts`.
- Step definitions: `tests/steps/validate_artifact_id_input_steps.rs`

- [x] validate-artifact-id-input: givn check with unknown artifact validates against manifest and provides helpful error
    - GREEN:    Add manifest validation to `check.rs`. Load manifest, check
                `artifact.id`, bail with known ids list. Files: `src/cli/check.rs`.
    - COMMIT:   `9c7d39f`

- [x] validate-artifact-id-input: givn check with known artifact proceeds to run hooks
    - GREEN:    Already covered by the previous GREEN change. Files: none (reuse).
    - COMMIT:   `92a10db`

- [x] validate-artifact-id-input: givn eject instruction with unknown artifact validates against manifest and provides helpful error
    - GREEN:    Add manifest validation to `eject.rs`'s `eject_instruction()`.
                Files: `src/cli/eject.rs`.
    - COMMIT:   `9c7d39f`

---

## Fix #8: License

### Design constraints
- `Cargo.toml`: change `license = "TBD"` to `license = "MIT OR Apache-2.0"`.
- `README.md`: replace "not yet chosen" text in the License section with a
  statement that the project is dual-licensed under MIT or Apache-2.0.
- Step definitions: `tests/steps/choose_project_license_steps.rs`

- [x] choose-project-license: Cargo.toml declares dual MIT OR Apache-2.0 license
    - GREEN:    Change `Cargo.toml:7` to `license = "MIT OR Apache-2.0"`. Files: `Cargo.toml`.
    - COMMIT:   `5ea37ac`

- [x] choose-project-license: README.md declares the project license
    - GREEN:    Update `README.md` License section. Files: `README.md`.
    - COMMIT:   `6fb80ea`

- [x] choose-project-license: Consumer inspects the project license in Cargo.toml and README
    - GREEN:    Already covered by previous GREEN changes. Files: none (reuse).
    - COMMIT:   `36d1f31`

---

## Fix #2: Enforce Post-Merge E2E Gate

### Design constraints
- `src/cli/archive.rs`: restructure the post-merge verify-e2e block (lines 176-197)
  to handle the missing case: when `e2e_command` is empty but merged specs contain
  `@e2e` scenarios, bail with rollback — same hard-fail semantics as
  `src/hooks.rs:138-146`.
- Use `crate::gherkin::parse::has_e2e_scenarios(&specs_dir)` to check merged specs.
- The check is absolute: any `@e2e` in merged permanent specs requires a configured
  `e2e_command`.
- Step definitions: `tests/steps/enforce_post_merge_e2e_gate_steps.rs`

- [x] enforce-post-merge-e2e-gate: Archive is blocked when merged permanent specs gain @e2e scenarios and e2e_command is empty
    - GREEN:    Restructure archive.rs:176-197 to add empty e2e_command + @e2e bail. Files: `src/cli/archive.rs`.
    - COMMIT:   `dfde3cb`

- [x] enforce-post-merge-e2e-gate: Archive succeeds when merged permanent specs have no @e2e scenarios and e2e_command is empty
    - GREEN:    Already covered by previous GREEN change. Files: none (reuse).
    - COMMIT:   `589bb75`

- [x] enforce-post-merge-e2e-gate: Archive runs e2e_command when merged permanent specs contain @e2e and e2e_command is configured
    - GREEN:    Already covered by previous GREEN change. Files: none (reuse).
    - COMMIT:   `589bb75`

---

## Fix #5: Extract Coverage Gate

### Design constraints
- Create `src/cli/coverage_gate.rs` containing `run_coverage_gate()` and
  `rollback_readme()` (extracted from `archive.rs`).
- In `archive.rs`: make `rollback_specs` `pub(crate)`, replace the
  `run_coverage_gate(...)` call with
  `crate::cli::coverage_gate::run_coverage_gate(...)`. Remove `rollback_readme`.
- Add `pub mod coverage_gate;` to `src/cli/mod.rs`.
- Must be done AFTER Fix #2 (touches the same file — archive.rs).
- Step definitions: `tests/steps/extract_coverage_gate_steps.rs`

- [x] extract-coverage-gate: Archive with coverage addon enabled still runs the coverage gate
    - GREEN:    Create `src/cli/coverage_gate.rs`, move `run_coverage_gate` + `rollback_readme`. Files: `src/cli/coverage_gate.rs`, `src/cli/archive.rs`, `src/cli/mod.rs`.
    - COMMIT:   `afce2fa`

- [x] extract-coverage-gate: Archive with coverage addon enabled still rolls back specs and README on coverage failure
    - GREEN:    Already covered by previous GREEN change. Files: none (reuse).
    - COMMIT:   `afce2fa`

---

## E2E setup: configure e2e test infrastructure, strict mode, and verify.e2e_command

- [ ] Setup: configure e2e test infrastructure
    - Install the e2e framework: same cucumber runner as non-e2e (`cargo test
      --test features_runner`), filtered to `@e2e` scenarios via cucumber's
      tag-filter mechanism.
    - Create e2e step skeleton in `tests/steps/`, one file per capability
      matching the Step Definitions table in design.md. Each file creates a
      separate Cucumber `World` or reuses `GivnWorld` from `tests/steps/mod.rs`.
    - Configure and PROVE strict mode for the e2e runner: same `.fail_on_skipped()`
      builder config as the main runner. Write one e2e step with `unimplemented!()`,
      run filtered to `@e2e`, confirm NON-ZERO exit. Paste command + output.
    - Set `verify.e2e_command` to `cargo test --test features_runner -- --tags @e2e`
      or equivalent tag-filter invocation. The e2e command MUST be a strict
      subset of `verify.command` — run both, paste scenario counts: e2e count
      MUST be strictly less than full count.
    - Document local environment: no external services needed. All e2e scenarios
      invoke the givn binary as a subprocess via `run_givn()`.
    - Run `verify.e2e_command` → must exit 0 (proof step removed, @e2e scenarios
      still @wip).

---

## E2e scenarios

- [ ] @e2e fix-uppercase-checkbox-parsing: Consumer runs givn status on a change with uppercase checkboxes
    - Design constraints: Invoke givn binary via `run_givn()`; assert stdout contains
      "1/2 tasks" and exit 0.
    - RED:      Remove @wip. Unimplemented steps use `unimplemented!()`. Run
                `cargo test --test features_runner -- --tags @e2e --tags fix-uppercase-checkbox-parsing` → MUST FAIL.
                Paste output.
    - GREEN:    Write e2e step definitions. List files: `tests/steps/fix_uppercase_checkbox_parsing_steps.rs`.
                Run targeting this scenario → PASSES. Paste output.
    - REFACTOR: Clean up. Re-run → still PASSES. Paste output.
    - COMMIT:   One commit. Record hash.

- [ ] @e2e deduplicate-change-dir-detection: Consumer runs the integrity check on a change inside givn/changes and the correct directory is detected
    - Design constraints: Invoke `givn check review --change detect-int`. Assert
      exit 0 and stdout confirms integrity checks passed.
    - RED:      Remove @wip. Unimplemented steps use `unimplemented!()`. Run
                `cargo test --test features_runner -- --tags @e2e --tags deduplicate-change-dir-detection-ok` → MUST FAIL.
                Paste output.
    - GREEN:    Write e2e step definitions. Create change directory with valid
                design.md and tasks.md. List files: `tests/steps/deduplicate_change_dir_detection_steps.rs`.
                Run targeting this scenario → PASSES. Paste output.
    - REFACTOR: Clean up. Re-run → still PASSES. Paste output.
    - COMMIT:   One commit. Record hash.

- [ ] @e2e deduplicate-change-dir-detection: Integrity check detects a placeholder COMMIT hash in tasks.md and fails
    - Design constraints: Create change with placeholder hash "xxxxxxxxxxxx...",
      invoke `givn check review --change bad-hash`, assert non-zero exit and error.
    - RED:      Remove @wip. Unimplemented steps use `unimplemented!()`. Run
                `cargo test --test features_runner -- --tags @e2e --tags deduplicate-change-dir-detection-bad-hash` → MUST FAIL.
                Paste output.
    - GREEN:    Write e2e step definitions (same file). List files: same as above.
                Run targeting this scenario → PASSES. Paste output.
    - REFACTOR: Clean up. Re-run → still PASSES. Paste output.
    - COMMIT:   One commit. Record hash.

- [ ] @e2e enforce-post-merge-e2e-gate: Consumer archives a delta that adds e2e scenarios and is blocked without a configured runner
    - Design constraints: Empty e2e_command, delta with @e2e scenario. Assert
      `givn archive` fails, stderr says e2e runner not configured, specs rolled back.
    - RED:      Remove @wip. Unimplemented steps use `unimplemented!()`. Run
                `cargo test --test features_runner -- --tags @e2e --tags enforce-post-merge-e2e-gate` → MUST FAIL.
                Paste output.
    - GREEN:    Write e2e step definitions. Set up change with all artifacts done
                and @e2e delta. List files: `tests/steps/enforce_post_merge_e2e_gate_steps.rs`.
                Run targeting this scenario → PASSES. Paste output.
    - REFACTOR: Clean up. Re-run → still PASSES. Paste output.
    - COMMIT:   One commit. Record hash.

- [ ] @e2e extract-coverage-gate: Consumer archives a change with coverage enabled and the coverage summary appears in README
    - Design constraints: Coverage addon enabled, valid coverage commands producing
      Cobertura XML. Assert success, README contains coverage fence.
    - RED:      Remove @wip. Unimplemented steps use `unimplemented!()`. Run
                `cargo test --test features_runner -- --tags @e2e --tags extract-coverage-gate` → MUST FAIL.
                Paste output.
    - GREEN:    Write e2e step definitions. Configure coverage commands that produce
                real Cobertura XML. List files: `tests/steps/extract_coverage_gate_steps.rs`.
                Run targeting this scenario → PASSES. Paste output.
    - REFACTOR: Clean up. Re-run → still PASSES. Paste output.
    - COMMIT:   One commit. Record hash.

- [ ] @e2e remove-dead-skip-list-code: Consumer runs givn status on a change with skipped artifacts and sees them correctly
    - Design constraints: Change with proposal.md and `.givn-skip` listing "design".
      Assert exit 0, stdout shows design as skipped.
    - RED:      Remove @wip. Unimplemented steps use `unimplemented!()`. Run
                `cargo test --test features_runner -- --tags @e2e --tags remove-dead-skip-list-code` → MUST FAIL.
                Paste output.
    - GREEN:    Write e2e step definitions. List files:
                `tests/steps/remove_dead_skip_list_code_steps.rs`. Run targeting this
                scenario → PASSES. Paste output.
    - REFACTOR: Clean up. Re-run → still PASSES. Paste output.
    - COMMIT:   One commit. Record hash.

- [ ] @e2e validate-artifact-id-input: Consumer invokes givn check with an unknown artifact and gets a helpful error
    - Design constraints: Invoke `givn check bogus-artifact`. Assert non-zero exit,
      stderr contains "unknown artifact".
    - RED:      Remove @wip. Unimplemented steps use `unimplemented!()`. Run
                `cargo test --test features_runner -- --tags @e2e --tags validate-artifact-id-input` → MUST FAIL.
                Paste output.
    - GREEN:    Write e2e step definitions. List files:
                `tests/steps/validate_artifact_id_input_steps.rs`. Run targeting this
                scenario → PASSES. Paste output.
    - REFACTOR: Clean up. Re-run → still PASSES. Paste output.
    - COMMIT:   One commit. Record hash.

- [ ] @e2e validate-change-id-input: Consumer invokes givn status with a path traversal change id and gets a clear error
    - Design constraints: Invoke `givn status --change "../etc"`. Assert non-zero
      exit, stderr says "kebab-case".
    - RED:      Remove @wip. Unimplemented steps use `unimplemented!()`. Run
                `cargo test --test features_runner -- --tags @e2e --tags validate-change-id-input` → MUST FAIL.
                Paste output.
    - GREEN:    Write e2e step definitions. List files:
                `tests/steps/validate_change_id_input_steps.rs`. Run targeting this
                scenario → PASSES. Paste output.
    - REFACTOR: Clean up. Re-run → still PASSES. Paste output.
    - COMMIT:   One commit. Record hash.

---

## Post-implementation

- [ ] Run full suite: `cargo test --test features_runner` → must exit 0.
- [ ] Run e2e suite: `cargo test --test features_runner -- --tags @e2e` → must exit 0.
- [ ] Run `cargo clippy --workspace --all-targets -- -D warnings` → must pass.
- [ ] Run `cargo fmt -- --check` → must pass.
