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
      Hash: `003a633`

## Non-E2E Scenarios

- [x] Scenario: `A new capability without a declared group keeps the flat merge layout`
  - [x] RED: remove `@wip`. Targeted run — must fail on the undefined Given
        (new fixture step).
        ```text
        1 scenario (1 failed)
        2 steps (1 passed, 1 failed)
        (exit 101 — step failed: no matching step definition)
        ```
  - [x] GREEN: implemented the fixture Given in
        `tests/steps/grouped_merge_steps.rs` (completed-change fixture
        without a routing row, delta under `specs/flat-new/`). No
        production change — the invariant already holds.
        ```text
        1 scenario (1 passed)
        7 steps (7 passed)
        (exit 0)
        ```
  - [x] REFACTOR: fixture shares the artifact/delta helpers between both
        Givens; rerun green.
  - [x] COMMIT: `feat(grouped-merge-for-new-capabilities): A new capability without a declared group keeps the flat merge layout`
        Hash: `8f28e44`

## E2E Scenarios

- [x] E2E setup: none beyond the existing CLI-subprocess harness (no server,
      browser, database, or digital twins).

- [x] Scenario: `Consumer archives a new capability and the merged spec lands in its declared group`
  - [x] RED: remove `@wip`. Targeted e2e run — must fail on the missing
        grouped file (merge still lands flat; paste output).
        ```text
        Captured output: expected file "givn/specs/some-group/grouped-new.feature" to exist
        1 scenario (1 failed)
        6 steps (5 passed, 1 failed)
        (exit 101 — the archive succeeded and moved the change, but merged flat)
        ```
  - [x] GREEN: implemented the fixture Given in
        `tests/steps/grouped_merge_steps.rs` (proposal with a parseable
        `NEW in some-group` routing row, delta under `specs/some-group/`).
        Production change in `src/cli/archive.rs` `process_delta_file`: when
        the capability exists nowhere, consult the proposal's routing table;
        `NEW in <group>` targets `givn/specs/<group>/<cap>.feature`; any
        other case keeps the flat fallback. Targeted rerun.
        ```text
        1 scenario (1 passed)
        8 steps (8 passed)
        (exit 0)
        ```
  - [x] REFACTOR: the branch is minimal; helper
        `routing_group_for_new_capability` documented inline. Rerun green.
  - [x] COMMIT: `test(e2e): Consumer archives a new capability and the merged spec lands in its declared group`
        Hash: `6d5522d`

## Final verification

- [x] Full change suite green:
      ```text
      GIVN_FEATURES=givn/changes/grouped-merge-for-new-capabilities/specs ./run-tests.sh       → exit 0
      GIVN_FEATURES=givn/changes/grouped-merge-for-new-capabilities/specs ./run-tests.sh --e2e → 1 scenario (1 passed); exit 0
      ```
- [x] `givn lint --change grouped-merge-for-new-capabilities` → exit 0, clean,
      no `@wip`.
- [x] `cargo fmt` + `cargo clippy --workspace --all-targets -- -D warnings`
      clean.
- [x] Full permanent suite `./run-tests.sh` → 360 scenarios (360 passed);
      exit 0. Unit `cargo test --lib` → 241 passed.
- [x] Self-dogfood: `cargo install --path . --locked --force` (installed
      binary carries the grouped merge); `givn skills sync` clean.