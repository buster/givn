# Tasks: integrity-hook-and-archive-gate

RED→GREEN→REFACTOR→COMMIT per scenario. Non-@e2e first, then @e2e.

Legend: `[ ]` pending · `[x]` done only when evidence + commit hash are filled in. Check boxes immediately after each scenario — tasks.md must always reflect true state.

---

## Strict-mode proof (do first)

Empty/no-op steps are indistinguishable from PASS in most runners. Prove the
runner rejects an unimplemented step before any scenario work.

- [ ] **Proof-of-strictness**: run the runner against a step using the
      not-implemented stub. Confirm non-zero exit.
      ```
      cargo test --test features_runner "integrity hook" -- --exact --test-threads=1
      ```
      If exit 0, configure the strict-mode flag (`fail_on_skipped()`) before proceeding.

---

## Tasks

### Setup

- [ ] Set up step definition file (`tests/steps/workflow_enforcement_steps.rs`) with
      the `givn-workflow-enforcement` capability. Configure the runner to
      discover the new step file. Proof-of-strictness above must be filled in
      first.

### Integrity hook — hash presence

- [ ] Scenario: `Integrity hook passes when tasks.md commit hashes are present`
  - [ ] RED: Write step definitions for the positive case (valid tasks.md, valid
        design.md, `givn check review` exits 0). Implement the `integrity` hook
        skeleton in `src/hooks.rs` with a new `"integrity"` match arm that
        returns Ok(()). Write `run_integrity_hook` as a stub returning Ok(()).
        Add `"integrity"` to `review.post` in `assets/default-manifest.yaml`.
        Run targeting this scenario only. Must exit non-zero (hash presence
        check not yet implemented).
        ```
        ```
  - [ ] GREEN: Implement hash-presence check: parse tasks.md for `- [x] COMMIT:`
        lines; extract text after `COMMIT:` up to `—` or EOL; fail if
        placeholder/empty. Implement `run_integrity_hook` in
        `src/integrity_hook.rs` with the hash-presence fn. Hook reads
        tasks.md via `features_path` (change dir = parent of specs dir).
        Files created/modified: `src/integrity_hook.rs`, `src/hooks.rs`,
        `assets/default-manifest.yaml`.
        Run targeting this scenario only. Must pass.
        ```
        ```
  - [ ] REFACTOR: Clean up. Re-run, must still pass.
        ```
        ```
  - [ ] COMMIT: one commit for RED+GREEN+REFACTOR. Hash:
        `- [x] COMMIT:`

- [ ] Scenario: `Integrity hook fails on placeholder commit hash`
  - [ ] RED: Write step definitions: create tasks.md with `___` placeholder,
        run `givn check review`, assert non-zero and stderr mentions
        `placeholder` and `COMMIT`. Must fail (placeholder logic not in hook
        yet — wait, actually if the hash-presence check from the previous
        scenario treats `___` as empty, this might already pass. Let me
        reconsider: the hash-presence check from the previous scenario
        extracts text after `COMMIT:` and checks non-empty. `___` is
        non-empty if we just check length. So this scenario specifically
        asserts that the check ALSO detects `___` as a placeholder, not
        just empty strings. So the previous GREEN only handles empty/missing.
        Add placeholder detection (`___`, `---`, `TBD`, `TODO`). Must fail
        because placeholder detection isn't implemented yet.
        ```
        ```
  - [ ] GREEN: Add placeholder detection to the hash-presence check: treat
        `___`, `---`, `TBD`, `TODO`, values containing HTML comments, and
        whitespace-only as placeholder. Files modified: `src/integrity_hook.rs`.
        Run targeting this scenario only. Must pass.
        ```
        ```
  - [ ] REFACTOR: Clean up. Re-run, must still pass.
        ```
        ```
  - [ ] COMMIT: one commit. Hash:
        `- [x] COMMIT:`

### Integrity hook — promised-files-exist

- [ ] Scenario: `Integrity hook fails on missing step-definition file`
  - [ ] RED: Write step definitions: design.md Step Definitions table lists
        a non-existent file, run `givn check review`, assert non-zero and
        stderr mentions the filename. Must fail (promised-files check not
        implemented yet).
        ```
        ```
  - [ ] GREEN: Implement promised-files-exist check: parse design.md for
        `## Step Definitions` section, extract markdown table, get
        `Step definition file` column values, check each path exists and is
        non-empty. Paths resolved relative to project root. Files modified:
        `src/integrity_hook.rs`.
        Run targeting this scenario only. Must pass.
        ```
        ```
  - [ ] REFACTOR: Clean up. Re-run, must still pass.
        ```
        ```
  - [ ] COMMIT: one commit. Hash:
        `- [x] COMMIT:`

- [ ] Scenario: `Integrity hook fails on empty step-definition file`
  - [ ] RED: Write step definitions: design.md lists an existing but zero-byte
        file, assert failure. Must fail (empty-file check not in
        promised-files check yet — currently just checks existence).
        ```
        ```
  - [ ] GREEN: Extend promised-files check: after confirming path exists,
        also check `metadata().len() > 0` (non-empty). Files modified:
        `src/integrity_hook.rs`.
        Run targeting this scenario only. Must pass.
        ```
        ```
  - [ ] REFACTOR: Clean up. Re-run, must still pass.
        ```
        ```
  - [ ] COMMIT: one commit. Hash:
        `- [x] COMMIT:`

- [ ] Scenario: `Integrity hook fails on Step Definitions section that is not a table`
  - [ ] RED: Write step definitions: design.md has `## Step Definitions` but
        the content is prose (backticks) not a table. Assert failure with
        "table format" in stderr. Must fail (table-format check not
        implemented).
        ```
        ```
  - [ ] GREEN: Extend promised-files check: after locating `## Step Definitions`
        section, attempt to parse a markdown table from it. If no table
        found and the section content doesn't say "N/A" or "none", hard-fail
        with "table format" error. Files modified: `src/integrity_hook.rs`.
        Run targeting this scenario only. Must pass.
        ```
        ```
  - [ ] REFACTOR: Clean up. Re-run, must still pass.
        ```
        ```
  - [ ] COMMIT: one commit. Hash:
        `- [x] COMMIT:`

### Integrity hook — arc42 conformance

- [ ] Scenario: `Integrity hook passes when arc42 is disabled`
  - [ ] RED: Write step definitions: givn project with arc42 disabled in
        config. Assert `givn check review` exits 0 (arc42 check skipped).
        Must fail (hook doesn't exist yet — actually the hash-presence and
        promised-files checks from previous scenarios should already make
        this pass IF the step definitions set up a valid tasks.md and
        design.md. Let me ensure the step def for this scenario creates
        valid fixtures. Must fail initially because the step skeleton uses
        not-implemented stubs).
        ```
        ```
  - [ ] GREEN: Ensure the arc42 check is gated on `manifest.addons.arc42`.
        Implement the check to return Ok(()) if arc42 is disabled. Files
        modified: `src/integrity_hook.rs`.
        Run targeting this scenario only. Must pass.
        ```
        ```
  - [ ] REFACTOR: Clean up. Re-run, must still pass.
        ```
        ```
  - [ ] COMMIT: one commit. Hash:
        `- [x] COMMIT:`

- [ ] Scenario: `Integrity hook fails on unedited arc42 chapter`
  - [ ] RED: Write step definitions: givn project with arc42 enabled, a valid
        arc42.md marking chapter "01 introduction-and-goals" as Affected:
        Yes, and the chapter file byte-identical to the embedded template.
        Assert non-zero with "arc42" and "01-introduction-and-goals" in
        stderr. Must fail (arc42 byte-compare not implemented).
        ```
        ```
  - [ ] GREEN: Implement arc42 conformance check: parse arc42.md marker table,
        find rows where `Affected?` is `Yes`, extract chapter slug from
        column 2, map to `docs/arc42/NN-*.md` path, read embedded template
        via `Assets::get_str`, byte-compare. Hard-fail if byte-identical.
        Files modified: `src/integrity_hook.rs`.
        Run targeting this scenario only. Must pass.
        ```
        ```
  - [ ] REFACTOR: Clean up. Re-run, must still pass.
        ```
        ```
  - [ ] COMMIT: one commit. Hash:
        `- [x] COMMIT:`

- [ ] Scenario: `Integrity hook passes on edited arc42 chapter`
  - [ ] RED: Write step definitions: same as above but chapter file differs
        from template. Assert exit 0. Must fail (byte-compare logic from
        previous scenario should make this pass? Actually if the previous
        GREEN implemented byte-compare correctly, this should already pass.
        The RED for THIS scenario is just proving the step skeleton works.
        Then GREEN is a no-op implementation-wise — the feature is already
        built. But the testing paradigm requires GREEN to add code. If the
        feature already exists, the scenario is already GREEN. So we skip
        this? Per the TDD loop: if the feature is already implemented by
        prior GREEN, this scenario was inadvertently made GREEN already.
        Mark it as such. No new production code needed.
        ```
        ```
  - [ ] GREEN: No production code changes needed — already implemented by
        previous scenario.
        ```
        ```
  - [ ] REFACTOR: No changes. Re-run, must still pass.
        ```
        ```
  - [ ] COMMIT: one commit. Hash:
        `- [x] COMMIT:`

- [ ] Scenario: `Integrity hook skips arc42 check when slug cannot be resolved`
  - [ ] RED: Write step definitions: arc42.md marks an unresolvable chapter
        slug as Affected: Yes (file doesn't exist). Assert exit 0.
        Must fail (currently the byte-compare would try to read non-existent
        file and crash).
        ```
        ```
  - [ ] GREEN: Wrap the byte-compare in slug-resolution: attempt to map slug
        to file path; if path doesn't exist, skip (no hard-fail). Files
        modified: `src/integrity_hook.rs`.
        Run targeting this scenario only. Must pass.
        ```
        ```
  - [ ] REFACTOR: Clean up. Re-run, must still pass.
        ```
        ```
  - [ ] COMMIT: one commit. Hash:
        `- [x] COMMIT:`

### Integrity hook — `givn check review` without change (global check)

- [ ] Scenario: `givn check review` can also run globally (without --change)
  - [ ] RED: Write step definitions: no specific scenario needed — covered by
        existing `givn check review` tests. Actually — the existing `givn check`
        command (check.rs) runs hooks for artifacts with existing output. The
        `integrity` hook is attached to `review.post`, so `givn check review`
        runs it. The ability to run without `--change` depends on finding the
        review artifact in the global project. This is existing behaviour from
        check.rs:42-49. No new implementation needed. Mark as covered.
        ```
        ```

### Post-archive verify gate

- [ ] Scenario: `Archive re-runs verify against merged permanent specs and passes`
  - [ ] RED: Write step definitions: create a complete change with delta specs,
        a verify.command that passes, run `givn archive`, assert exit 0 and
        backup dir is cleaned up. Must fail (post-archive verify gate not
        implemented).
        ```
        ```
  - [ ] GREEN: Implement the post-archive verify gate in `src/cli/archive.rs`:
        after merge loop, before file move, snapshot `givn/specs/` to
        `givn/.archive-backup-<id>/`, run `verify` with
        `GIVN_FEATURES=givn/specs`, if success clean up backup. Files
        modified: `src/cli/archive.rs`.
        Run targeting this scenario only. Must pass.
        ```
        ```
  - [ ] REFACTOR: Clean up. Re-run, must still pass.
        ```
        ```
  - [ ] COMMIT: one commit. Hash:
        `- [x] COMMIT:`

- [ ] Scenario: `Archive blocked when merged permanent specs fail verify`
  - [ ] RED: Write step definitions: create a change whose delta produces a
        conflict in the merged permanent spec, verify.command fails, assert
        archive blocked, backup dir exists. Must fail (failure handling not
        implemented).
        ```
        ```
  - [ ] GREEN: Implement failure handling: when post-merge verify fails, print
        error message mentioning "verify", "permanent", "backup" with paths,
        do NOT delete backup dir, do NOT move change. Files modified:
        `src/cli/archive.rs`.
        Run targeting this scenario only. Must pass.
        ```
        ```
  - [ ] REFACTOR: Clean up. Re-run, must still pass.
        ```
        ```
  - [ ] COMMIT: one commit. Hash:
        `- [x] COMMIT:`

- [ ] Scenario: `New-capability merge failure produces clear recovery instructions`
  - [ ] RED: Write step definitions: delta introduces a brand-new capability
        (no existing permanent file), verify fails. Assert error message
        instructs to delete the newly-created directory. Must fail (new-capability
        recovery not in error message yet).
        ```
        ```
  - [ ] GREEN: In the failure handler, detect which permanent spec files were
        newly created (backup entry is None), and include instructions to
        delete them in the error message. Files modified:
        `src/cli/archive.rs`.
        Run targeting this scenario only. Must pass.
        ```
        ```
  - [ ] REFACTOR: Clean up. Re-run, must still pass.
        ```
        ```
  - [ ] COMMIT: one commit. Hash:
        `- [x] COMMIT:`

### Template changes (design.md + tasks.md + review.md)

- [ ] Scenario: `Design.md template includes Internal Primitives section`
  - [ ] RED: Write step definitions: generate a new change, read design.md
        template, assert it contains `## Internal Primitives` section with
        the required columns. Must fail (template not updated yet).
        ```
        ```
  - [ ] GREEN: Update `assets/templates/design.md`: add `## Internal Primitives`
        section after `## Step Definitions` with the table format. Update
        `assets/templates/tasks.md`: change COMMIT box to inline format
        `- [x] COMMIT: <hash> — <message>`. Update `assets/templates/review.md`:
        add sign-off item for internal primitives and arc42 slug-mapping
        instruction. Files modified: `assets/templates/design.md`,
        `assets/templates/tasks.md`, `assets/templates/review.md`.
        Run targeting this scenario only. Must pass.
        ```
        ```
  - [ ] REFACTOR: Clean up. Re-run, must still pass.
        ```
        ```
  - [ ] COMMIT: one commit. Hash:
        `- [x] COMMIT:`

---

**Done when:** every box checked with evidence and commit hash, both
`cargo test --test features_runner -- --test-threads=1` and
`verify.e2e_command` exit 0 on the full suite.
