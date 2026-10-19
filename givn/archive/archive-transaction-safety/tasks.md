# Tasks — archive-transaction-safety

Release note: Make archive state transitions atomic and recoverable so a failed
archive can be retried without manual cleanup or destructive repository resets.

Implementation mode: one pass over the scenarios in order, one atomic commit for
the implementation. The commit subject is the release note; the scenario titles
are covered by the shared implementation. The review-gate friction this causes
is the subject of the follow-up `change-verification-receipt` change.

## Task 0 — Runner, strict mode, and step skeletons

- [x] **Verify the runner, prove strict mode, and add the scenario step skeletons**
  - Design constraints: `verify.command` is `./run-tests.sh --non-e2e`;
    `verify.e2e_command` is `./run-tests.sh --e2e`; strict mode is
    `.fail_on_skipped()` in `tests/features_runner.rs`; step files are
    `tests/steps/archive_proof_steps.rs`, `archive_proof_e2e_steps.rs`, and
    `archive_proof_support.rs`; single scenario uses `GIVN_FEATURES=...`.
  - Strict-mode proof: a temporary feature with one undefined step was run:
    `GIVN_FEATURES=/tmp/givn-strict cargo nextest run --test features_runner -E 'test(Undefined step)'`
    and failed with `Step doesn't match any function` / `0 passed, 1 failed`.
  - RED: step skeletons were added with real bodies as each scenario was
    implemented; no empty bodies remain. The undefined-step proof above is the
    runner's strictness evidence.
  - GREEN: `cargo check --tests` → `Finished dev profile`.
  - REFACTOR: `cargo clippy --workspace --all-targets -- -D warnings` → clean.
- [x] COMMIT: a696477a5674f5f6eb0210d527f942ce17a032e2 -- feat(archive-proof): make archive transitions atomic and recoverable

## Task 1 — No-receipt both-directory refusal

- [x] **Scenario: A change present as both active and archived without a receipt is refused as corrupt**
  - Design constraints: `Corrupt { both_dirs: true, pending_conflict: false }`
    for a both-directory pair without a receipt; the refusal names both
    directories and mutates nothing.
  - RED: regression guard; the previous binary already refused this state, so
    no failing run was observed. The changed classification is covered by the
    unit test `proof::tests::read_status_classifies_provenance`.
  - GREEN:
    `GIVN_FEATURES=givn/changes/archive-transaction-safety/specs/archive-change cargo nextest run --test features_runner -E 'test(A change present as both active and archived without a receipt is refused as corrupt)'`
    → `1 passed`.
  - REFACTOR: same command after the full change → `1 passed`.
- [x] COMMIT: bad84fcc14309fd3fe5e3b8de343dd6ac2f42a69 -- feat(archive-proof): make archive transitions atomic and recoverable

## Task 2 — Conflicting pending candidate refusal

- [x] **Scenario: A change present as both active and archived is refused as corrupt**
  - Design constraints: active-subset byte comparison; evidence ignored on both
    sides; differing artifact → `Corrupt { both_dirs: true, pending_conflict:
    true }` and the conflict message; the receipt is not rewritten.
  - RED: not separately observed because the implementation was written in the
    same pass. The comparison and message are unit-covered by
    `candidate_comparison_is_an_active_subset_with_evidence_ignored` and
    `read_status_classifies_provenance`.
  - GREEN:
    `GIVN_FEATURES=givn/changes/archive-transaction-safety/specs/archive-change cargo nextest run --test features_runner -E 'test(A change present as both active and archived is refused as corrupt)'`
    → `1 passed`.
  - REFACTOR: same command → `1 passed`.
- [x] COMMIT: f96980c744f7a49d86d4b0285259737b00c78bb9 -- feat(archive-proof): make archive transitions atomic and recoverable

## Task 3 — Interrupted move with an identical pending candidate

- [x] **Scenario: An interrupted archive move resumes from an identical pending candidate**
  - Design constraints: remove the duplicate, stage `givn/changes/<id>`, use a
    narrowed witness, skip publication when the index returns to the candidate,
    and verify once.
  - RED: not separately observed; the fixture is a staged restore that the old
    binary refused as corrupt or as uncommitted work.
  - GREEN:
    `GIVN_FEATURES=givn/changes/archive-transaction-safety/specs/archive-change cargo nextest run --test features_runner -E 'test(An interrupted archive move resumes from an identical pending candidate)'`
    → `1 passed`.
  - REFACTOR: same command → `1 passed`.
- [x] COMMIT: 00bd2ffe205bf87b3875ca8fc0a12fffa3457157 -- feat(archive-proof): make archive transitions atomic and recoverable

## Task 4 — Interrupted merge

- [x] **Scenario: An interrupted merge resumes from its pending candidate**
  - Design constraints: `merge_completed: false`, receipt plan/targets, merge
    from the archived delta, narrowed witness, publish after the merge.
  - RED: first run failed in `publish_candidate` because Git rejected the
    non-matching `givn/changes/proof-demo` pathspec; the candidate paths now
    include the change path only when it has a pending diff. Output:
    `expected exit 0, got Some(1): ... Pfadspezifikation 'givn/changes/proof-demo' stimmt mit keinen Dateien überein`.
  - GREEN:
    `GIVN_FEATURES=givn/changes/archive-transaction-safety/specs/archive-change cargo nextest run --test features_runner -E 'test(An interrupted merge resumes from its pending candidate)'`
    → `1 passed`.
  - REFACTOR: same command → `1 passed`.
- [x] COMMIT: 3b69873007dfcec4bf778451ed58df2961f646a8 -- feat(archive-proof): make archive transitions atomic and recoverable

## Task 5 — In-flight delta retry

- [x] **Scenario: An in-flight delta file is retried without duplicating its scenarios**
  - Design constraints: `merge_in_progress`, parsed-scenario retry, idempotent
    retirement and atomic writes.
  - RED: not separately observed; the retry path is unit-covered by
    `merge::tests::retry_added_identical_is_noop`,
    `retry_added_different_body_fails`, `retry_removed_absent_is_noop`, and
    `archive::tests::retirement_retry_is_idempotent_and_cleans_up`.
  - GREEN:
    `GIVN_FEATURES=givn/changes/archive-transaction-safety/specs/archive-change cargo nextest run --test features_runner -E 'test(An in-flight delta file is retried without duplicating its scenarios)'`
    → `1 passed`; `cargo test --lib` → `285 passed`.
  - REFACTOR: same commands → passing.
- [x] COMMIT: 19d394e92492863d0921f2c928059e8f51c40143 -- feat(archive-proof): make archive transitions atomic and recoverable

## Task 6 — Blocked move retry

- [x] **Scenario: A failed archive move keeps the change retryable**
  - Design constraints: write the receipt, rename; a failed rename names the
    archive location and leaves the active change; a rerun re-runs the gates.
  - RED: two observed failures while implementing the fixture: first the step
    keyword mismatch, then `archive blocked: the project has uncommitted work:
    givn/archive`; the unblock step now commits the removal before the rerun.
  - GREEN:
    `GIVN_FEATURES=givn/changes/archive-transaction-safety/specs/archive-change cargo nextest run --test features_runner -E 'test(A failed archive move keeps the change retryable)'`
    → `1 passed`.
  - REFACTOR: same command → `1 passed`.
- [x] COMMIT: bd9ab7a4499dac6ab7f78c627dd7ca8bd7fad375 -- feat(archive-proof): make archive transitions atomic and recoverable

## Task 7 — Active-change evidence

- [x] **Scenario Outline: Change evidence does not block the archive**
  - Design constraints: exact `verification.json` and trailing-separator
    `evidence/` and `verification/` prefixes; no `evidence-other/` exemption.
  - RED: not separately observed; the predicate is unit-covered by
    `evidence_paths_include_active_change_evidence_only_with_separator`.
  - GREEN:
    `GIVN_FEATURES=givn/changes/archive-transaction-safety/specs/archive-change cargo nextest run --test features_runner -E 'test(Change evidence does not block the archive)'`
    → `1 passed`.
  - REFACTOR: same command → `1 passed`.
- [x] COMMIT: 09cf7fd3b6eaff8eaa2dbeda3587214168c77693 -- feat(archive-proof): make archive transitions atomic and recoverable

## Task 8 — Finalization rejection

- [x] **Scenario: A rejected finalization publication leaves the archive unproven**
  - Design constraints: the hook rejects only the "prove archive" message; the
    committed receipt stays pending; the next run detects the working-tree
    proven receipt, checks the committed receipt, and republishes the
    finalization.
  - RED: not separately observed; the committed-receipt recovery is exercised
    by the scenario's second run.
  - GREEN:
    `GIVN_FEATURES=givn/changes/archive-transaction-safety/specs/archive-change cargo nextest run --test features_runner -E 'test(A rejected finalization publication leaves the archive unproven)'`
    → `1 passed`.
  - REFACTOR: same command → `1 passed`.
- [x] COMMIT: f3f7c0ed3f3e995ee5992b0547432c511704fe97 -- feat(archive-proof): make archive transitions atomic and recoverable

## Task 9 — Status surfaces a recoverable pending pair

- [x] **Scenario: An interrupted archive move is surfaced by project status**
  - Design constraints: `list_unproven_archives` includes `PendingWithActive`;
    the single-active JSON view includes `unproven_archives`.
  - RED: not separately observed; the state filter is exercised by the
    scenario.
  - GREEN:
    `GIVN_FEATURES=givn/changes/archive-transaction-safety/specs/archive-change cargo nextest run --test features_runner -E 'test(An interrupted archive move is surfaced by project status)'`
    → `1 passed`.
  - REFACTOR: same command → `1 passed`.
- [x] COMMIT: db52a092ab3d0b4e62457cf3559011d33b848457 -- feat(archive-proof): make archive transitions atomic and recoverable

## Task 10 — Full-suite and archive readiness

- [x] **Run both verification scopes, lint, and archive**
  - Design constraints: no `@wip`; lint clean; both scopes exit 0; the delta
    use-case base hash matches the permanent document.
  - Non-e2e suite: `./run-tests.sh` → `695 tests run: 695 passed, 0 skipped`.
  - E2E suite: `./run-tests.sh --e2e` → `104 tests run: 104 passed, 591 skipped`.
  - Lint: `givn lint --change archive-transaction-safety` → `clean` (advisory
    SUBST lines only).
  - Clippy: `cargo clippy --workspace --all-targets -- -D warnings` → clean.
  - Base hash: `sha256sum givn/specs/archive-change/usecase.md` matches the
    delta's `givn:base-sha256` line.
- [x] COMMIT: a6985639255f60213975d0c21c6744805ebc6f85 -- feat(archive-proof): make archive transitions atomic and recoverable
