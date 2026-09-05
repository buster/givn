# Tasks: no-flat-fallback-for-new-capabilities

One scenario (blocked archive), replacing the superseded flat-layout
scenario via `@givn.removed` + `@givn.added`. Runner contract and strict
mode unchanged and previously proven.

## Setup

- [x] **Setup**: runner contract unchanged; strict mode previously proven
      (`.fail_on_skipped()`, exit-101 probes in the two predecessor
      changes).
- [x] **Planning-layer commit**: proposal (routing decision `EXTEND
      grouped-merge-for-new-capabilities`), spec delta (removed+added),
      design, arc42 assessment, runtime-view 6.7 update, this tasks.md.
      Commit: `docs(no-flat-fallback-for-new-capabilities): planning layer`
      Hash: `21131bb`

## E2E Scenarios

- [ ] E2E setup: none beyond the existing CLI-subprocess harness.

- [ ] Scenario: `Archive is blocked when a new capability has no declared routing decision`
  - [ ] RED: remove `@wip`. Targeted e2e run — must fail because the
        archive currently succeeds via the flat fallback (paste output).
  - [ ] GREEN: replace the fallback arm in `src/cli/archive.rs`
        `process_delta_file` with the blocking error. Targeted rerun.
  - [ ] REFACTOR + rerun.
  - [ ] COMMIT: `test(e2e): Archive is blocked when a new capability has no declared routing decision`
        Hash: `<fill>`

## Final verification

- [ ] Change suite green (non-e2e + e2e scoped to the change).
- [ ] `givn lint --change no-flat-fallback-for-new-capabilities` exit 0, no
      `@wip` (removed placeholder suppressed by lint).
- [ ] `cargo fmt` + `cargo clippy --workspace --all-targets -- -D warnings`.
- [ ] Full permanent suite exit 0.
- [ ] Self-dogfood: `cargo install --path . --locked --force`; `givn skills
      sync`.