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

- [x] E2E setup: fixture helpers that archive brand-new capabilities now
      write a `NEW in <cap>` routing decision into their proposal
      (`fixture_routing_proposal` in tests/steps/mod.rs, applied in
      enforce_post_merge_e2e_gate_steps.rs, extract_coverage_gate_steps.rs,
      fix_uppercase_checkbox_parsing_steps.rs, readme_enforcement_steps.rs,
      workflow_enforcement_steps.rs). Declaring the group as the capability
      name reproduces the delta's own specs layout, so no path assertions
      changed.

- [x] Scenario: `Archive is blocked when a new capability has no declared routing decision`
  - [x] RED: exit 101 — archive succeeded via the flat fallback, so
        `the command exits non-zero` failed.
  - [x] GREEN: fallback arm in `src/cli/archive.rs` `process_delta_file`
        replaced with the blocking error. Targeted rerun: 8/8 steps passed.
  - [x] REFACTOR: none needed.
  - [x] COMMIT: `test(e2e): Archive is blocked when a new capability has no declared routing decision`
        Hash: `f425541`

- [x] Scenario: `Nothing is merged when one delta file of a change has no routing decision`
  - [x] RED: exit 101 — the routed delta had already merged
        (`givn/specs/mixed-group/mixed-routing.feature` existed) when the
        unrouted one blocked; partial merge contradicted the design claim
        "error surfaces before any file is written". Found in design-review
        grilling, spec'd as a second scenario before fixing.
  - [x] GREEN: prevalidation pass in `src/cli/archive.rs` (parse, WIP
        check, resolve every target) before any merge; target resolution
        extracted into `resolve_permanent_dir` + `delta_capability`; two
        stale flat-fallback comments reworded. Rerun passed.
  - [x] COMMIT: `test(e2e): Nothing is merged when one delta file of a change has no routing decision`
        Hash: `89422e9`

## Final verification

- [x] Change suite green: 3 non-e2e + 2 e2e scenarios passed; permanent
      suite 361/362 with the single failure being the superseded
      flat-layout scenario that this change's `@givn.removed` delta
      retires at merge.
- [x] `givn lint --change no-flat-fallback-for-new-capabilities` exit 0.
- [x] `cargo fmt` + `cargo clippy --workspace --all-targets -- -D warnings`.
- [x] `cargo test --lib`: 241 passed.
- [x] Self-dogfood: `cargo install --path . --locked --force`; `givn skills
      sync` (binary replaced from this checkout; integrations regenerated).