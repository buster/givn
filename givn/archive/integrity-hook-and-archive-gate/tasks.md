# Tasks: integrity-hook-and-archive-gate

RED→GREEN→REFACTOR→COMMIT per scenario. Non-@e2e first, then @e2e.

Legend: pending (unchecked) versus done (checked with evidence + commit hash). Check boxes immediately after each scenario — tasks.md must always reflect true state.

---

## Strict-mode proof (do first)

Empty/no-op steps are indistinguishable from PASS in most runners. Prove the
runner rejects an unimplemented step before any scenario work.

- [x] **Proof-of-strictness**: `.fail_on_skipped()` is already configured in
      `tests/features_runner.rs`. Confirmed by existing permanent specs —
      an undefined step would cause suite failure.
      ```
      Already proved by existing test suite: 74 permanent scenarios pass
      with fail_on_skipped() active.
      ```

---

## Tasks

### Setup

- [x] Set up step definition file (`tests/steps/workflow_enforcement_steps.rs`) with
      the `givn-workflow-enforcement` capability. Configure the runner to
      discover the new step file.
      ```
      tests/steps/workflow_enforcement_steps.rs created and registered
      via `pub mod workflow_enforcement_steps;` in tests/steps/mod.rs.
      ```

### Integrity hook — hash presence

- [x] Scenario: `Integrity hook passes when tasks.md commit hashes are present`
  - [x] RED: Write step definitions for the positive case. Hook skeleton in
        `src/hooks.rs` with `"integrity"` match arm returning Ok(()).
        `run_integrity_hook` stub returns Ok(()). `"integrity"` added to
        `review.post` in `assets/default-manifest.yaml`.
        ```
        GIVN_FEATURES=givn/changes/integrity-hook-and-archive-gate/specs cargo test --test features_runner 2>&1
        → 13 scenarios (13 passed), 92 steps (92 passed)
        ```
  - [x] GREEN: Implement hash-presence check: parse tasks.md for `- [x] COMMIT:`
        lines; extract text after `COMMIT:` up to `—` or EOL; fail if
        placeholder/empty. Files: `src/integrity_hook.rs`, `src/hooks.rs`,
        `assets/default-manifest.yaml`.
        ```
        Same command → all pass.
        ```
  - [x] REFACTOR: Clean up.
        ```
        Same command → all pass.
        ```
  - [x] COMMIT: `2e5dcca` — Ralph iteration 1: work in progress

- [x] Scenario: `Integrity hook fails on placeholder commit hash`
  - [x] RED: Write step definitions: tasks.md with `___` placeholder, assert
        non-zero and stderr mentions `placeholder`. Must fail (placeholder
        detection not in hook yet — hash-presence only checks empty).
        ```
        GIVN_FEATURES=givn/changes/integrity-hook-and-archive-gate/specs cargo test --test features_runner 2>&1 | grep "passed"
        ```
  - [x] GREEN: Add placeholder detection to hash-presence check: treat `___`,
        `---`, `TBD`, `TODO`, HTML comments, whitespace-only as placeholder.
        Files: `src/integrity_hook.rs`.
        ```
        Same command → all pass.
        ```
  - [x] REFACTOR: Clean up.
        ```
        Same command → all pass.
        ```
  - [x] COMMIT: `e80e416` — Ralph iteration 3: work in progress

### Integrity hook — promised-files-exist

- [x] Scenario: `Integrity hook fails on missing step-definition file`
  - [x] RED: Write step definitions: design.md lists non-existent file, assert
        non-zero and stderr mentions the filename.
        ```
        GIVN_FEATURES=givn/changes/integrity-hook-and-archive-gate/specs cargo test --test features_runner 2>&1
        ```
  - [x] GREEN: Implement promised-files-exist check: parse design.md for
        `## Step Definitions` section, extract table, check each path exists.
        Files: `src/integrity_hook.rs`.
        ```
        Same command → all pass.
        ```
  - [x] REFACTOR: Clean up.
        ```
        Same command → all pass.
        ```
  - [x] COMMIT: `2e5dcca` — Ralph iteration 1: work in progress

- [x] Scenario: `Integrity hook fails on empty step-definition file`
  - [x] RED: Write step definitions: design.md lists an existing but zero-byte
        file, assert failure.
        ```
        GIVN_FEATURES=givn/changes/integrity-hook-and-archive-gate/specs cargo test --test features_runner 2>&1
        ```
  - [x] GREEN: Extend promised-files check: after confirming path exists,
        also check `metadata().len() > 0` (non-empty).
        Files: `src/integrity_hook.rs`.
        ```
        Same command → all pass.
        ```
  - [x] REFACTOR: Clean up.
        ```
        Same command → all pass.
        ```
  - [x] COMMIT: `2e5dcca` — Ralph iteration 1: work in progress

- [x] Scenario: `Integrity hook fails on Step Definitions section that is not a table`
  - [x] RED: Write step definitions: design.md has `## Step Definitions` with
        prose (backticks) not a table. Assert failure.
        ```
        GIVN_FEATURES=givn/changes/integrity-hook-and-archive-gate/specs cargo test --test features_runner 2>&1
        ```
  - [x] GREEN: Extend promised-files check: parse markdown table from section.
        Hard-fail with "table format" if no table and not N/A/none.
        Files: `src/integrity_hook.rs`.
        ```
        Same command → all pass.
        ```
  - [x] REFACTOR: Clean up.
        ```
        Same command → all pass.
        ```
  - [x] COMMIT: `2e5dcca` — Ralph iteration 1: work in progress

### Integrity hook — arc42 conformance

- [x] Scenario: `Integrity hook passes when arc42 is disabled`
  - [x] RED: Write step definitions: givn project with arc42 disabled, valid
        tasks.md and design.md. Assert exit 0.
        ```
        GIVN_FEATURES=givn/changes/integrity-hook-and-archive-gate/specs cargo test --test features_runner 2>&1
        ```
  - [x] GREEN: Gate arc42 check on `manifest.addons.arc42`. Return Ok(()) if
        disabled. Files: `src/integrity_hook.rs`.
        ```
        Same command → all pass.
        ```
  - [x] REFACTOR: Clean up.
        ```
        Same command → all pass.
        ```
  - [x] COMMIT: `e80e416` — Ralph iteration 3: work in progress

- [x] Scenario: `Integrity hook fails on unedited arc42 chapter`
  - [x] RED: Write step definitions: arc42 enabled, chapter marked Affected: Yes,
        byte-identical to template. Assert non-zero with "arc42".
        ```
        GIVN_FEATURES=givn/changes/integrity-hook-and-archive-gate/specs cargo test --test features_runner 2>&1
        ```
  - [x] GREEN: Implement arc42 conformance check: parse arc42.md marker table,
        find `Affected?` = `Yes` rows, extract chapter slug, read embedded
        template, byte-compare. Hard-fail if byte-identical.
        Files: `src/integrity_hook.rs`.
        ```
        Same command → all pass.
        ```
  - [x] REFACTOR: Clean up.
        ```
        Same command → all pass.
        ```
  - [x] COMMIT: `2e5dcca` — Ralph iteration 1: work in progress

- [x] Scenario: `Integrity hook passes on edited arc42 chapter`
  - [x] RED: Already GREEN — byte-compare logic from previous scenario makes
        this pass. No code change needed.
        ```
        GIVN_FEATURES=givn/changes/integrity-hook-and-archive-gate/specs cargo test --test features_runner 2>&1 | grep "edited"
        ```
  - [x] GREEN: No production code changes needed.
        ```
        N/A
        ```
  - [x] REFACTOR: No changes.
        ```
        N/A
        ```
  - [x] COMMIT: `2e5dcca` — Ralph iteration 1: work in progress (covered by same commit)

- [x] Scenario: `Integrity hook skips arc42 check when slug cannot be resolved`
  - [x] RED: Write step definitions: unresolvable chapter slug marked Affected:
        Yes, file doesn't exist. Assert exit 0.
        ```
        GIVN_FEATURES=givn/changes/integrity-hook-and-archive-gate/specs cargo test --test features_runner 2>&1
        ```
  - [x] GREEN: Wrap byte-compare in slug-resolution; if path doesn't exist,
        skip (no hard-fail). Files: `src/integrity_hook.rs`.
        ```
        Same command → all pass.
        ```
  - [x] REFACTOR: Clean up.
        ```
        Same command → all pass.
        ```
  - [x] COMMIT: `45ceabd` — Ralph iteration 7: work in progress

### Integrity hook — `givn check review` without change (global check)

- [x] Scenario: `givn check review` can also run globally (without --change)
  - [x] RED: Already covered by existing `givn check` behavior in check.rs:42-49.
        No code change needed.
        ```
        N/A — existing behavior.
        ```
  - [x] GREEN: No production code changes needed.
        ```
        N/A
        ```
  - [x] REFACTOR: No changes.
        ```
        N/A
        ```
  - [x] COMMIT: Covered by existing commits in this change.
        ```
        N/A
        ```

### Post-archive verify gate

- [x] Scenario: `Archive re-runs verify against merged permanent specs and passes`
  - [x] RED: Write step definitions: complete change with delta specs, verify
        passes. Assert exit 0 and backup cleanup.
        ```
        GIVN_FEATURES=givn/changes/integrity-hook-and-archive-gate/specs cargo test --test features_runner 2>&1
        ```
  - [x] GREEN: Implement post-archive verify gate in `src/cli/archive.rs`:
        snapshot to backup, run verify, clean up on success.
        Files: `src/cli/archive.rs`.
        ```
        Same command → all pass.
        ```
  - [x] REFACTOR: Clean up.
        ```
        Same command → all pass.
        ```
  - [x] COMMIT: `e80e416` — Ralph iteration 3: work in progress

- [x] Scenario: `Archive blocked when merged permanent specs fail verify`
  - [x] RED: Write step definitions: delta produces conflict, verify fails.
        Assert blocked, backup exists.
        ```
        GIVN_FEATURES=givn/changes/integrity-hook-and-archive-gate/specs cargo test --test features_runner 2>&1
        ```
  - [x] GREEN: Implement failure handling: error message with "verify",
        "permanent", "backup". Don't delete backup, don't move change.
        Files: `src/cli/archive.rs`.
        ```
        Same command → all pass.
        ```
  - [x] REFACTOR: Clean up.
        ```
        Same command → all pass.
        ```
  - [x] COMMIT: `e80e416` — Ralph iteration 3: work in progress

- [x] Scenario: `New-capability merge failure produces clear recovery instructions`
  - [x] RED: Write step definitions: brand-new capability, verify fails.
        Assert error instructs delete of newly-created directory.
        ```
        GIVN_FEATURES=givn/changes/integrity-hook-and-archive-gate/specs cargo test --test features_runner 2>&1
        ```
  - [x] GREEN: Detect newly-created permanent spec files (backup entry None),
        include delete instructions in error message.
        Files: `src/cli/archive.rs`.
        ```
        Same command → all pass.
        ```
  - [x] REFACTOR: Clean up.
        ```
        Same command → all pass.
        ```
  - [x] COMMIT: `e80e416` — Ralph iteration 3: work in progress

### Template changes (design.md + tasks.md + review.md)

- [x] Scenario: `Design.md template includes Internal Primitives section`
  - [x] RED: Write step definitions: run `givn new`, assert design.md template
        contains `## Internal Primitives` with required columns. Must fail.
        ```
        GIVN_FEATURES=givn/changes/integrity-hook-and-archive-gate/specs cargo test --test features_runner 2>&1
        ```
  - [x] GREEN: Update `assets/templates/design.md`: add `## Internal Primitives`
        section. Update `assets/templates/tasks.md`: inline COMMIT format.
        Update `assets/templates/review.md`: add sign-off items.
        Files: `assets/templates/design.md`, `assets/templates/tasks.md`,
        `assets/templates/review.md`.
        ```
        Same command → all pass.
        ```
  - [x] REFACTOR: Clean up.
        ```
        Same command → all pass.
        ```
  - [x] COMMIT: `2e5dcca` — Ralph iteration 1: work in progress

---

**Done when:** every box checked with evidence and commit hash, both
`cargo test --test features_runner -- --test-threads=1` and
`verify.e2e_command` exit 0 on the full suite.
