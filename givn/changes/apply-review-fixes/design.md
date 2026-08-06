# Design: apply-review-fixes

## Technology Decisions

All changes are pure Rust within the existing codebase. No new dependencies, no
new CLI subcommands, no manifest changes.

## Architecture Impact Summary

| Fix | Files changed | New files | API surface change |
| --- | --- | --- | --- |
| #1 Uppercase checkbox | `src/state.rs` | none | none |
| #2 Post-merge e2e gate | `src/cli/archive.rs` | none | none (archive.rs internal) |
| #3 Validate --change | `src/cli/new.rs`, `src/cli/archive.rs`, `src/cli/check.rs`, `src/cli/graph.rs`, `src/cli/instructions.rs`, `src/cli/lint.rs`, `src/cli/status.rs` | none | new `pub(crate)` functions |
| #4 Deduplicate find_change_dir | `src/hooks.rs`, `src/integrity_hook.rs`, `src/state.rs` | none | none |
| #5 Extract coverage gate | `src/cli/archive.rs`, `src/cli/mod.rs` | `src/cli/coverage_gate.rs` | `rollback_specs` → `pub(crate)` |
| #6 Artifact id validation | `src/cli/check.rs`, `src/cli/eject.rs` | none | none |
| #7 Dead skip list code | `src/state.rs` | none | none |
| #8 License | `Cargo.toml`, `README.md` | none | none |

## Fix #1: Uppercase Checkbox Parsing

**Files:** `src/state.rs`

**Change:** In `parse_tasks_progress` (line 307-313), use case-insensitive
matching for `[x]` and `[X]`. The `artifact_status` function at line 209 already
calls `parse_tasks_progress.is_complete()`, so fixing the parser fixes both
`givn status` and the archive gate.

```rust
// Before (line 310-311):
let total = content.matches("[ ]").count() + content.matches("[x]").count();
let done = content.matches("[x]").count();

// After:
let total = content.matches("[ ]").count()
    + content.matches("[x]").count()
    + content.matches("[X]").count();
let done = content.matches("[x]").count() + content.matches("[X]").count();
```

**Test update:** Add a test case in `src/state.rs` tests for uppercase `[X]` and
mixed-case checkboxes.

## Fix #2: Enforce Post-Merge E2E Gate

**Files:** `src/cli/archive.rs`

**Change:** The post-merge verify-e2e block at lines 176-197 currently only runs
the e2e command if `e2e_command` is non-empty. Add the missing case: when
`e2e_command` is empty but the merged permanent specs (`givn/specs`) now contain
`@e2e` scenarios, block archive with the same hard-fail semantics as
`src/hooks.rs:138-146`.

```rust
// Replace lines 176-197 with:
let e2e_command = &manifest.verify.e2e_command;
if e2e_command.is_empty() {
    let merged_has_e2e = crate::gherkin::parse::has_e2e_scenarios(&specs_dir);
    if merged_has_e2e {
        rollback_specs(&specs_dir, &backup_specs, &backup_dir)?;
        bail!(
            "'{}' contains @e2e-tagged scenario(s) but verify.e2e_command is not \
             configured in givn/config.yaml. An @e2e scenario without a configured \
             e2e runner is an unenforced gap, not an optional one — set verify.e2e_command.",
            specs_dir.display()
        );
    }
    // Skip silently: no e2e_command AND no @e2e in merged specs.
} else {
    let merged_has_e2e = crate::gherkin::parse::has_e2e_scenarios(&specs_dir);
    if merged_has_e2e {
        println!("Running post-merge verify-e2e: {e2e_command}");
        let e2e_status = std::process::Command::new("sh")
            .arg("-c")
            .arg(e2e_command)
            .env("GIVN_FEATURES", "givn/specs")
            .status()?;
        if !e2e_status.success() {
            rollback_specs(&specs_dir, &backup_specs, &backup_dir)?;
            bail!(...);
        }
        println!("✓ Post-merge verify-e2e passed");
    }
}
```

## Fix #3: Validate --change Id Input

**Files:** 7 files (see impact table)

**Step 1 — Extract `validate_id`:** Move `validate_id` from `src/cli/new.rs`
to the crate root or a shared utility module (suggest `src/change_util.rs` or
inline in `src/state.rs`). Make it `pub(crate)`.

**Step 2 — Create `resolve_change_dir`:** A helper that validates the id then
constructs the path:

```rust
pub(crate) fn resolve_change_dir(project_root: &Path, id: &str) -> Result<PathBuf> {
    validate_id(id)?;
    Ok(project_root.join("givn/changes").join(id))
}
```

**Step 3 — Call `resolve_change_dir`** in every command that accepts `--change`,
replacing the direct `project_root.join("givn/changes").join(id)` pattern:

| Command | Current line | Replacement |
| --- | --- | --- |
| `archive.rs:24` | `project_root.join("givn/changes").join(change)` | `resolve_change_dir(&project_root, change)?` |
| `check.rs:14` | `project_root.join("givn/changes").join(id)` | `resolve_change_dir(&project_root, id)?` |
| `graph.rs:28` | `project_root.join("givn/changes").join(id)` | `resolve_change_dir(&project_root, id)?` |
| `instructions.rs:84` | `project_root.join("givn/changes").join(id)` | `resolve_change_dir(&project_root, id)?` |
| `lint.rs:23` | `project_root.join("givn/changes").join(id)` | `resolve_change_dir(&project_root, id)?` |
| `status.rs:21` | `project_root.join("givn/changes").join(id)` | `resolve_change_dir(&project_root, id)?` |

**Step 4 — Replace `validate_id` call in `new.rs`:** `new.rs` calls
`validate_id()` directly and then separately joins paths. Switch to
`resolve_change_dir(project_root, id)?` and drop the separate
`project_root.join("givn/changes").join(id)`.

**Step 5 — Remove private `validate_id` from `new.rs`.**

## Fix #4: Deduplicate Change Directory Detection

**Files:** `src/hooks.rs`, `src/integrity_hook.rs`, `src/state.rs`

**Current implementations:**
- `hooks.rs:178-191` (`find_change_dir`): String-based, searches for
  `"givn/changes/"` substring. Fails for absolute paths.
- `integrity_hook.rs:51-74` (`find_change_dir`): Component-based, walks
  `Path::components()`. Correct for all paths.

**Change:**

1. Move the component-based implementation from `integrity_hook.rs:51-74` to a
   new `pub(crate)` function in `src/state.rs`:
   ```rust
   pub(crate) fn find_change_dir(features_path: &str) -> Option<PathBuf>
   ```

2. Remove `find_change_dir` from `hooks.rs:178-191` — replace the call at line
   150 with `crate::state::find_change_dir(features_path)`.

3. Remove `find_change_dir` from `integrity_hook.rs:51-74` — replace the call at
   line 13 with `crate::state::find_change_dir(features_path)`.

4. Keep the signature `(&str) -> Option<PathBuf>` (the component-based
   implementation's input type is `&str` — it converts to `Path` internally).

**Test update:** The existing tests in `integrity_hook.rs:471-486` test
`find_change_dir` — move them to `src/state.rs` tests.

## Fix #5: Extract Coverage Gate

**Files:** `src/cli/archive.rs`, `src/cli/coverage_gate.rs` (new),
`src/cli/mod.rs`

**Change:**

1. Create `src/cli/coverage_gate.rs` containing `run_coverage_gate()` (lines
   228-384 in current `archive.rs`) and `rollback_readme()` (lines 431-452 in
   current `archive.rs`). Both become `pub(crate)`.

2. In `archive.rs`, make `rollback_specs` `pub(crate)` (line 389) so the
   coverage_gate module can call it.

3. In `archive.rs`, replace the `run_coverage_gate(...)` call at line 202 with
   `crate::cli::coverage_gate::run_coverage_gate(&project_root, &manifest, &backup_dir)?`.

4. In `archive.rs`, remove `rollback_readme` (lines 431-452) — coverage_gate
   owns it.

5. Add `pub mod coverage_gate;` to `src/cli/mod.rs`.

**Invariants preserved:**
- `rollback_readme` is only called by the coverage gate — moving it keeps the
  call tree correct.
- `rollback_specs` stays in `archive.rs` because the archive's own gates (verify,
  verify-e2e) also call it. It just becomes `pub(crate)`.

## Fix #6: Artifact Id Validation

**Files:** `src/cli/check.rs`, `src/cli/eject.rs`

### check.rs

In `run()` (line 20-23), validate the artifact id against the manifest before
`run_mandatory_checks`:

```rust
if let Some(artifact_id) = artifact {
    // Validate against manifest (consistent with `givn instructions`).
    if !manifest.artifacts.iter().any(|a| a.id == artifact_id) {
        let known: Vec<&str> = manifest.artifacts.iter().map(|a| a.id.as_str()).collect();
        anyhow::bail!(
            "unknown artifact '{}'. Known artifacts: {}",
            artifact_id,
            known.join(", ")
        );
    }
    run_mandatory_checks(artifact_id, &manifest, &features_path)?;
    crate::hooks::run_artifact_hooks(artifact_id, &manifest, &features_path)
}
```

The `run_artifact_hooks` function already validates (line 30 in hooks.rs) but
the error message is "artifact 'X' not found in manifest" — functional but
missing the list of known ids. Adding the check in `check.rs` gives the same
helpful error as `givn instructions`.

### eject.rs

In `eject_instruction()` (lines 85-106), add manifest validation before the
blind asset lookup:

```rust
fn eject_instruction(project_root: &Path, artifact_id: &str, force: bool) -> Result<()> {
    let manifest = crate::config::overlay::load_effective()?;
    if !manifest.artifacts.iter().any(|a| a.id == artifact_id) {
        let known: Vec<&str> = manifest.artifacts.iter().map(|a| a.id.as_str()).collect();
        anyhow::bail!(
            "unknown artifact '{}'. Known artifacts: {}",
            artifact_id,
            known.join(", ")
        );
    }
    // ... existing asset lookup ...
}
```

This mirrors the validation already present in `eject_template()`.

## Fix #7: Delete Dead Skip List Code

**Files:** `src/state.rs`

**Change:** Delete `read_skip_list()` (lines 351-361) and its `#[allow(dead_code)]`
annotation.

No other code changes — `is_skipped()` handles the actual use case and is the
only function called from `artifact_status()`.

## Fix #8: License

**Files:** `Cargo.toml`, `README.md`

**Cargo.toml** (line 7): Change `license = "TBD"` to `license = "MIT OR Apache-2.0"`.

**README.md:** Replace the "not yet chosen" text in the license section with a
statement that the project is dual-licensed under MIT or Apache-2.0, at the
user's option.

## Step Definitions

| Capability | Step definition file |
| --- | --- |
| fix-uppercase-checkbox-parsing | tests/steps/fix_uppercase_checkbox_parsing_steps.rs |
| enforce-post-merge-e2e-gate | tests/steps/enforce_post_merge_e2e_gate_steps.rs |
| validate-change-id-input | tests/steps/validate_change_id_input_steps.rs |
| deduplicate-change-dir-detection | tests/steps/deduplicate_change_dir_detection_steps.rs |
| extract-coverage-gate | tests/steps/extract_coverage_gate_steps.rs |
| validate-artifact-id-input | tests/steps/validate_artifact_id_input_steps.rs |
| remove-dead-skip-list-code | tests/steps/remove_dead_skip_list_code_steps.rs |
| choose-project-license | tests/steps/choose_project_license_steps.rs |

The cucumber runner binary (`tests/features_runner.rs`) reads all `.feature`
files under `givn/specs/` (post-archive). During the change lifecycle, the
specs live under `givn/changes/apply-review-fixes/specs/`. Step definitions
live under `tests/steps/` per the existing project convention.

### Strict Mode

The cucumber runner must be configured with `.fail_on_skipped()`. When a
scenario is tagged `@wip` and has no matching step definitions, cucumber
marks it as skipped. `.fail_on_skipped()` converts skipped steps to failures
during `verify.command` execution — this prevents scenarios from being
accidentally green because step definitions don't exist yet.

The runner configuration is in `tests/features_runner.rs`. Verify the strict
mode is already enabled; add it if missing.

### Local Runnability

All step definitions drive the system via `crate::common::run_givn(&env, &args)`
— a real subprocess invocation with a temp directory as the project root. No
external services, no database, no containers. The runner uses `cargo test`.

## Interaction Coverage Matrix

| Inventory entry (capability) | @e2e scenario |
| --- | --- |
| `givn status --change <id>` with uppercase checkboxes | "Consumer runs givn status on a change with uppercase checkboxes" |
| `givn archive` with @e2e delta + empty e2e_command | "Consumer archives a delta that adds e2e scenarios and is blocked without a configured runner" |
| `givn status --change ../` (path traversal) | "Consumer invokes givn status with a path traversal change id and gets a clear error" |
| `givn check review --change <id>` (integrity hook) | "Consumer runs the integrity check on a change inside givn/changes and the correct directory is detected" |
| `givn archive` with coverage addon | "Consumer archives a change with coverage enabled and the coverage summary appears in README" |
| `givn check <unknown-artifact>` | "Consumer invokes givn check with an unknown artifact and gets a helpful error" |
| `givn status --change <id>` with skipped artifacts | "Consumer runs givn status on a change with skipped artifacts and sees them correctly" |
| Inspect Cargo.toml + README.md for license | "Consumer inspects the project license in Cargo.toml and README" |

## Internal Primitives

| Primitive | Type | Purpose |
| --- | --- | --- |
| `validate_id` | `pub(crate) fn validate_id(id: &str) -> Result<()>` | Kebab-case validation extracted from new.rs |
| `resolve_change_dir` | `pub(crate) fn resolve_change_dir(project_root: &Path, id: &str) -> Result<PathBuf>` | Validates id and resolves to `givn/changes/<id>/` |
| `find_change_dir` (unified) | `pub(crate) fn find_change_dir(features_path: &str) -> Option<PathBuf>` | Component-based change dir detection, unified from integrity_hook.rs |
| `run_coverage_gate` | `pub(crate) fn run_coverage_gate(project_root: &Path, manifest: &Manifest, backup_dir: &Path) -> Result<()>` | Archive-level coverage gate, extracted from archive.rs |
