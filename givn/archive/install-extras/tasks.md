# Tasks: install-extras

## Task 0 — Test Infrastructure Setup

- [x] Verify `verify.command` is `cargo test --test features_runner` in givn/config.yaml
- [x] Confirm `cargo test --test features_runner` exits 0 with current specs
- [x] Add new step definitions skeleton in `tests/steps/mod.rs` for the new step
      patterns used by the extras scenarios (see design for full list)
- [x] Confirm `givn lint --change install-extras` exits 0 or 2 (scenarios @wip)

---

## Scenario 1: givn init defaults to extras enabled and generates extras skills

- [x] **RED**: Remove `@wip` from Scenario 1 only. Write step definitions.
  RED: genuine failure (no extras generated).

- [x] **GREEN**: Implement extras field on Manifest, OverlayDoc, init --no-extras,
  targets.rs sync extension, extras skill/command templates.
  All 8 steps green.

- [x] **REFACTOR**: Extracted resolve_skill_template_for()/resolve_command_template_for()
  dispatchers. validate_frontmatter() covers extras via shared sync loop.

---

## Scenario 2: givn init --no-extras produces no extras skills or commands

- [x] **RED**: Remove `@wip` from Scenario 2. Test already PASSES from Scenario 1
  implementation (side effect). Immediate GREEN — all assertions pass.

- [x] **GREEN**: Guard `set_extras(true)` on `!no_extras`. If `no_extras` is true,
  call `set_extras(false)`. Confirmed green.

- [x] **REFACTOR**: Init flow clear: config write → set_extras(!no_extras) → sync.
  Tests pass.

---

## Scenario 3: givn extras install enables extras and syncs skills

- [x] **RED**: Remove `@wip` from Scenario 3. `givn extras install` does not exist.
  Genuine failure (unexpected argument error).

- [x] **GREEN**: Implemented `run_install()`, `run_uninstall()`, `remove_extras_files()`
  in `src/cli/extras.rs`. Added `Commands::Extras` + `ExtrasCommands` in `src/main.rs`.
  All 7 steps green. Side-effect: Scenarios 4,5,6,10,11 also green.

- [x] **REFACTOR**: Kept `set_extras` and `set_addon` separate — no shared pattern
  to extract (different indent/block semantics). Tests pass.

---

## Scenario 4: givn extras install when already enabled is idempotent

- [x] **RED**: Side-effect green from Scenario 3 implementation. `run_install()`
  always calls `set_extras(true)` + `sync()` — both idempotent by design.

- [x] **GREEN**: Confirmed all steps pass.

- [x] **REFACTOR**: `set_extras` handles "already correct value" case correctly.
  No duplicate lines on re-run.

---

## Scenario 5: givn extras uninstall disables extras and removes generated files

- [x] **RED**: Side-effect green from Scenario 3 implementation.
  `run_uninstall()` + `remove_extras_files()` fully implemented.

- [x] **GREEN**: All 9 steps pass. Files removed from .agents/, .claude/,
  .opencode/ skills and commands dirs.

- [x] **REFACTOR**: Removal list computed from `extras_skill_names()` +
  `extras_command_names()`, not hard-coded paths.

---

## Scenario 6: givn extras uninstall when already disabled is idempotent

- [x] **RED**: Side-effect green. `run_uninstall()` when `extras: false` is
  a no-op (set_extras(false) idempotent, file removal best-effort).

- [x] **GREEN**: All steps pass.

- [x] **REFACTOR**: None needed.

---

## Scenario 7: givn skills sync includes extras skills and commands when enabled

- [x] **RED**: `stdout contains a line` step did exact match — sync output had
  "  synced skill: ..." with indent. Fixed with `l.trim()` in step def.

- [x] **GREEN**: All 6 steps pass. sync output includes extras skill/command lines.

- [x] **REFACTOR**: `l.trim()` fix in `stdout_contains_line` is backward-compatible —
  no change for existing tests without leading whitespace.

---

## Scenario 8: givn skills sync excludes extras skills and commands when disabled

- [x] **RED**: Side-effect green — `manifest.extras` is `false`, sync skips extras.

- [x] **GREEN**: All steps pass. `stdout does not contain` assertions confirm
  no extras in output.

- [x] **REFACTOR**: None.

---

## Scenario 9: extras skills are written to all target directories

- [x] **RED**: Side-effect green — `write_skill_to_targets()` handles all three dirs.

- [x] **GREEN**: All 4 steps pass. .agents/, .claude/, .opencode/ all have skill files.

- [x] **REFACTOR**: None.

---

## Scenario 10: givn extras install errors when givn/ is not initialised

- [x] **RED**: Side-effect green — `run_install()` checks for `givn/config.yaml`.

- [x] **GREEN**: All steps pass. Exit non-zero + stderr "givn/ not found".

- [x] **REFACTOR**: None.

---

## Scenario 11: givn extras uninstall errors when givn/ is not initialised

- [x] **RED**: Side-effect green — `run_uninstall()` checks for `givn/config.yaml`.

- [x] **GREEN**: All steps pass. Exit non-zero + stderr "givn/ not found".

- [x] **REFACTOR**: Both `run_install` and `run_uninstall` have the same guard.
  Kept separate — extraction belongs to this task, but the duplication is modest
  (one `if !project_root.join("givn").exists()` line each).

---

## Post-implementation

- [x] Run `givn lint --change install-extras` — exits 0 (no @wip, no parse errors).
- [x] Run `cargo test --test features_runner` — 42/42 scenarios pass.
- [x] Run `givn apply --change install-extras` — confirm task tracking.