# Tasks: grouped-merge-for-new-capabilities

Two scenarios; the grouped-merge scenario is foundational (its GREEN carries
the production change), the flat-fallback scenario records the preserved
backward-compatible invariant. Runner contract and strict mode are the
existing, proven ones; the setup proof is re-confirmed with the first RED.

## Setup

- [x] **Setup**: runner contract unchanged (`./run-tests.sh`,
      `./run-tests.sh --e2e`, strict `.fail_on_skipped()` in
      `tests/features_runner.rs`); e2e subset proof from the archived
      predecessor applies unchanged (tag filter is a strict subset).
- [x] **Planning-layer commit**: proposal (with routing decision `NEW in
      archival`), spec delta, design, arc42 assessment, archival inventory
      bullet, runtime-view chapter 6.7 clarification, this tasks.md.
      Commit: `docs(grouped-merge-for-new-capabilities): planning layer`
      Hash: `700ef14`

## Non-E2E Scenarios

- [ ] Scenario: `A new capability without a declared group keeps the flat merge layout`
  - [ ] RED: remove `@wip`. Targeted run — must fail on the undefined Given
        (new fixture step). Paste output.
  - [ ] GREEN: implement the fixture Given in
        `tests/steps/grouped_merge_steps.rs` (completed-change fixture
        without a routing row, delta under `specs/flat-new/`). No
        production change — the invariant already holds. Targeted rerun.
  - [ ] REFACTOR + rerun.
  - [ ] COMMIT: `feat(grouped-merge-for-new-capabilities): A new capability without a declared group keeps the flat merge layout`
        Hash: `<fill>`

## E2E Scenarios

- [ ] E2E setup: none beyond the existing CLI-subprocess harness (no server,
      browser, database, or digital twins).

- [ ] Scenario: `Consumer archives a new capability and the merged spec lands in its declared group`
  - [ ] RED: remove `@wip`. Targeted e2e run — must fail on the missing
        grouped file (merge still lands flat; paste output).
  - [ ] GREEN: implement the fixture Given in
        `tests/steps/grouped_merge_steps.rs` (proposal with a parseable
        `NEW in some-group` routing row, delta under `specs/some-group/`).
        Production change in `src/cli/archive.rs` `process_delta_file`: when
        the capability exists nowhere, consult the proposal's routing table;
        `NEW in <group>` targets `givn/specs/<group>/<cap>.feature`; any
        other case keeps the flat fallback. Targeted rerun.
  - [ ] REFACTOR + rerun.
  - [ ] COMMIT: `test(e2e): Consumer archives a new capability and the merged spec lands in its declared group`
        Hash: `<fill>`

## Final verification

- [ ] Full change suite green (`GIVN_FEATURES=givn/changes/grouped-merge-for-new-capabilities/specs ./run-tests.sh` and `--e2e`).
- [ ] `givn lint --change grouped-merge-for-new-capabilities` exit 0, no
      `@wip`.
- [ ] `cargo fmt` + `cargo clippy --workspace --all-targets -- -D warnings`.
- [ ] Full permanent suite `./run-tests.sh` exit 0.
- [ ] Self-dogfood: `cargo install --path . --locked --force`; `givn skills
      sync`; `givn instructions`; full suite + e2e green.