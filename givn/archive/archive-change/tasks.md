# Tasks: archive-change

Owning behavior: the existing `archive-change` use case, capability
`archive-proof`, plus a wording modification of `adr-qualification-gate`.

Domain constraints copied from the use-case contract
(`givn/changes/archive-change/specs/archive-change/usecase.md`):

- Clean state is mirrored: a file the backend would not publish blocks the
  archive — Git untracked files and Fossil `EXTRA` files both count as dirty.
- The candidate revision contains exactly the merged corpus paths and the
  pending receipt.
- The tested revision equals the published candidate revision; the receipt
  binds it.
- The finalization revision contains only resolved evidence paths.
- A backend hook that rejects the candidate leaves the archive unproven and
  repairable; the exact intermediate state is backend-specific.
- A failed scope leaves the published candidate in place; a rerun proves it
  without repeating the merge.
- No SCM backend, a nested project root, and a missing backend executable are
  refused by name before any mutation.
- Confirmed Personas: developer--primary-user, agent--technical-role,
  givn-maintainer--owner (review lenses, not Gherkin actors).

`adr-qualification-gate` constraint: the accepted register count follows the
new ADR-072; the permanent scenario and the delta both assert 28.

## Setup

- [x] Runner contract confirmed: `./run-tests.sh` executes the repository
      feature suite plus the active change when `GIVN_FEATURES` is set;
      `./run-tests.sh --e2e` executes the `@e2e` subset.
      `verify.e2e_command` is `./run-tests.sh --e2e`, strictly narrower than
      `verify.command`.
- [x] Step skeletons updated, one file per capability plus its E2E companion:
      `tests/steps/archive_proof_steps.rs`,
      `tests/steps/archive_proof_e2e_steps.rs`,
      `tests/steps/archive_proof_support.rs`, and the
      `adr_qualification_gate_steps.rs` count binding. New step bodies start
      as `unimplemented!()`.
- [x] Strict mode proven with a targeted run of the first scenario: the
      cucumber runner sets `.fail_on_skipped()` in
      `tests/features_runner.rs`; the first RED run exited non-zero with
      `not implemented`:

      ```text
      GIVN_FEATURES=givn/changes/archive-change/specs cargo test --test features_runner -- --name "Archive publishes the merged candidate before running the full suite"
      Scenario Outline: Archive publishes the merged candidate before running the full suite
       ✘  Given a Git project with a completed change "proof-demo" ready to archive
          Step panicked. Captured output: not implemented
      1 feature
      2 scenarios (2 failed)
      2 steps (2 failed)
      error: test failed, to rerun pass `--test features_runner`
      exit 1
      ```

- [x] Fossil test dependency present: `fossil version` resolves to
      2.26 `[611144ddd9]`; `.github/workflows/ci.yml` installs the package
      (`apt-get update && apt-get install -y --no-install-recommends fossil`)
      before the acceptance suites, and `AGENTS.md` records the prerequisite.
- [x] E2E count proof: the change scope runs the full set and the `@e2e`
      scope a strictly smaller set:

      ```text
      GIVN_FEATURES=givn/changes/archive-change/specs ./run-tests.sh
      Summary: 17 tests run: 17 passed, 0 skipped

      GIVN_FEATURES=givn/changes/archive-change/specs ./run-tests.sh --e2e
      Summary: 1 test run: 1 passed, 16 skipped
      ```

## Scenario: Archive publishes the merged candidate before running the full suite

Domain constraint: the candidate revision contains exactly the merged corpus
paths and the pending receipt; on a failed suite it stays published and
unproven.

- [x] RED: removed `@wip` from this scenario only; bound the
      backend-parameterized Given, the publication Thens, and the exact-content
      Then with `unimplemented!()`. Targeted run exited non-zero with
      `not implemented` (output above). Files: `archive_proof_steps.rs`.
- [x] GREEN: `src/scm.rs` returns `Backend::Fossil` for a Fossil checkout with
      the executable available; new `src/fossil.rs` implements `revision`,
      `dirty_entries`, `publish`, `revision_contains`; `src/cli/archive.rs`
      moves onto `scm::select` and `Backend::publish`; `src/git.rs` gains
      `revision_contains`; `src/cli/migration.rs` refuses the Fossil managed
      upgrade by name; `src/lib.rs`/`src/main.rs` register the module; the
      fixtures gain a real Fossil checkout (`tests/common/mod.rs`,
      `archive_proof_support.rs`). Targeted run:

      ```text
      GIVN_FEATURES=givn/changes/archive-change/specs cargo test --test features_runner -- --name "Archive publishes the merged candidate before running the full suite"
      Summary: 1 feature, 2 scenarios (2 passed), 18 steps (18 passed)
      exit 0
      ```

- [x] REFACTOR: neutralized the archive doc comment and formatted the touched
      files; targeted run still exited zero (2 scenarios, 18 steps passed).
- [x] COMMIT: `c31270ed feat(archive-proof): Archive publishes the merged candidate before running the full suite` (planning and architecture artifacts in `ee1a509e`).

## Scenario: Repairing a failed candidate forward proves it without repeating the merge

Domain constraint: the published candidate stays in place; the repair adds a
revision; the rerun proves without re-merging.

- [x] RED: removed `@wip`; bound the backend-parameterized Given and the
      repaired-revision Then (`unimplemented!()`), and the Fossil example
      failed on the Git-only repair step. Targeted run exited non-zero:

      ```text
      2 scenarios (2 failed), 10 steps (8 passed, 2 failed)
      Step panicked. Captured output: git add failed: ... Kein Git-Repository
      Step panicked. Captured output: not implemented
      exit 1
      ```

- [x] GREEN: `the developer commits a repair` uses `commit_fixture` (backend
      aware); `support::parent_revision` reads `git rev-parse HEAD~1` or the
      `fossil info` `parent:` line; `receipt_names_tested_commit` and the new
      repaired-revision Then assert the receipt against it. Targeted run:

      ```text
      GIVN_FEATURES=givn/changes/archive-change/specs cargo test --test features_runner -- --name "Repairing a failed candidate forward proves it without repeating the merge"
      1 feature, 2 scenarios (2 passed), 16 steps (16 passed)
      exit 0
      ```

- [x] REFACTOR: formatted the touched step files; targeted run still exited
      zero (2 scenarios, 16 steps passed).
- [x] COMMIT: `701c3115 feat(archive-proof): Repairing a failed candidate forward proves it without repeating the merge`.

## Scenario: A proven archive records a verification receipt binding the tested revision

Domain constraint: the receipt binds the tested revision; the finalization
revision contains only the resolved evidence paths.

- [x] RED: removed `@wip`; bound the revision/finalization Thens with
      `unimplemented!()`; targeted run exited non-zero:

      ```text
      Step panicked. Captured output: not implemented
      2 scenarios (2 failed), 12 steps (10 passed, 2 failed)
      exit 1
      ```

- [x] GREEN: renamed the receipt fields to `candidate_revision` /
      `tested_revision` (`src/proof.rs`, step readers); `support::current_revision`
      and the new Thens assert the receipt against the parent revision and the
      finalization revision's paths. Targeted run:

      ```text
      GIVN_FEATURES=givn/changes/archive-change/specs cargo test --test features_runner -- --name "A proven archive records a verification receipt binding the tested revision"
      1 feature, 2 scenarios (2 passed), 18 steps (18 passed)
      exit 0
      ```

- [x] REFACTOR: the permanent scenarios' old step texts (`must be the
      repository root`, `candidate commit rejection`) were re-bound to the new
      backend-neutral messages so the pre-archive corpus stays green; the
      permanent archive-change feature passes 44 scenarios. Targeted run still
      exited zero.
- [x] COMMIT: `6f79a523 feat(archive-proof): A proven archive records a verification receipt binding the tested revision`.

## Scenario: A rejected candidate revision leaves the archive unproven and repairable

Domain constraint: a rejected publication leaves the archive unproven and
repairable; the rerun publishes the on-disk candidate again and proves it.

- [x] RED: removed `@wip`; bound the hook Givens/Whens and the tested-revision
      Then with `unimplemented!()`; targeted run exited non-zero:

      ```text
      Step panicked. Captured output: not implemented
      2 scenarios (2 failed), 4 steps (2 passed, 2 failed)
      exit 1
      ```

- [x] GREEN: backend-aware hook fixtures (Git `.git/hooks/commit-msg`, Fossil
      `hook add --command "exit 1" --type before-commit` / `hook delete all`);
      `publish_candidate` extracted; on resume the archive checks
      `backend.revision_contains(pending receipt)`, republishes the on-disk
      merged state when it is absent, and keeps the Project witness only when
      the candidate is published. Targeted run:

      ```text
      GIVN_FEATURES=givn/changes/archive-change/specs cargo test --test features_runner -- --name "A rejected candidate revision leaves the archive unproven and repairable"
      1 feature, 2 scenarios (2 passed), 22 steps (22 passed)
      exit 0
      ```

- [x] REFACTOR: formatted; the permanent archive-change feature passes 44
      scenarios (the witness move does not disturb the fresh path). Targeted
      run still exited zero.
- [x] COMMIT: `c43cea1b feat(archive-proof): A rejected candidate revision leaves the archive unproven and repairable`.

## Scenario: A rejected candidate revision on Git leaves the merge staged

Domain constraint: the Git intermediate state is staged but uncommitted; the
archive is unproven.

- [x] RED/GREEN: removed `@wip`; every step was already bound (the Git hook
      Given/Whens from the repairable scenario and the permanent staged-state
      Then), so this is a legitimate immediate GREEN with no new production
      code. Targeted run:

      ```text
      GIVN_FEATURES=givn/changes/archive-change/specs cargo test --test features_runner -- --name "A rejected candidate revision on Git leaves the merge staged"
      1 feature, 1 scenario (1 passed), 6 steps (6 passed)
      exit 0
      ```

- [x] REFACTOR: no change needed; permanent archive-change feature stays green
      (44 scenarios).
- [x] COMMIT: `05282387` (shared with the Fossil intermediate-state scenario).

## Scenario: A rejected candidate revision on Fossil leaves the merged state uncommitted

Domain constraint: the Fossil intermediate state is present in the checkout but
uncommitted; the archive is unproven.

- [x] RED: removed `@wip`; bound the checkout-state Then with
      `unimplemented!()`; targeted run exited non-zero:

      ```text
      Step panicked. Captured output: not implemented
      1 scenario (1 failed), 5 steps (4 passed, 1 failed)
      exit 1
      ```

- [x] GREEN: `support::dirty_entries` reports Git porcelain and Fossil
      `changes --differ` paths; the Then asserts a merged spec is dirty and the
      pending receipt is not in the current revision. Targeted run:

      ```text
      GIVN_FEATURES=givn/changes/archive-change/specs cargo test --test features_runner -- --name "A rejected candidate revision on Fossil leaves the merged state uncommitted"
      1 feature, 1 scenario (1 passed), 6 steps (6 passed)
      exit 0
      ```

- [x] REFACTOR: formatted; both backend intermediate-state scenarios pass
      together (2 scenarios, 12 steps).
- [x] COMMIT: `05282387 feat(archive-proof): A rejected candidate revision on Git leaves the merge staged and on Fossil leaves the merged state uncommitted`.

## Scenario: A Fossil EXTRA file blocks the archive

Domain constraint: mirrored strictness — Fossil `EXTRA` files block like Git
untracked files, and the refusal mutates nothing.

- [x] RED: removed `@wip`; bound the EXTRA Given with `unimplemented!()`;
      targeted run exited non-zero (`not implemented`, 1 failed).
- [x] GREEN: the Fossil clean-state witness reports `EXTRA` paths; the Given
      writes the EXTRA file, the lazy snapshot runs after all Givens, and the
      refusal names it and changes nothing. Targeted run:

      ```text
      1 feature, 1 scenario (1 passed), 7 steps (7 passed)
      ```

- [x] REFACTOR: harness fix — the no-mutation snapshot now skips backend
      bookkeeping (`.git`, `.fslckout`, `_FOSSIL_`) because Fossil rewrites
      `.fslckout` while merely reporting changes; the assertion reports only
      the differing paths. Targeted run still exited zero.
- [x] COMMIT: `90b96845` (shared with the Fossil edited-file scenario).

## Scenario: A Fossil edited file blocks the archive

Domain constraint: the Fossil witness covers `EDITED` entries, not only
`EXTRA`.

- [x] RED: removed `@wip`; bound the edited-file Given with
      `unimplemented!()`; targeted run exited non-zero.
- [x] GREEN: the Given writes and commits a tracked file, edits it, and the
      witness names it; the refusal changes nothing. Targeted run:

      ```text
      1 feature, 1 scenario (1 passed), 7 steps (7 passed)
      ```

- [x] REFACTOR: shared with the EXTRA scenario's snapshot fix. Targeted run
      still exited zero.
- [x] COMMIT: `90b96845 feat(archive-proof): A Fossil EXTRA file and a Fossil edited file block the archive`.

## Scenario: Archive requires a version-controlled project

Domain constraint: no usable backend is refused by name before any mutation.

- [x] RED/GREEN: removed `@wip`; the steps reuse the seed-1 refusal fixtures
      and Thens, so after aligning the delta's Given texts with the existing
      bindings this is an immediate GREEN with no new production code. The
      seed-1 `given_project_without_backend` now takes the lazy run-time
      snapshot and skips the auto-commit. Targeted run:

      ```text
      4 scenarios (4 passed), 17 steps (17 passed)
      ```

- [x] REFACTOR: bound `Given placeholder` for the delta's `@givn.removed`
      tombstones so the whole change scope runs; formatted.
- [x] COMMIT: `55e164b3` (shared with the checkout-root and missing-fossil
      refusals).

## Scenario: Archive requires the project root to be the checkout root

Domain constraint: a nested project root is refused by name before any
mutation.

- [x] RED/GREEN: removed `@wip`; all steps reuse the existing nested-project
      fixture and the seed-1 worktree-top-level Then (the permanent
      repository-root step text was re-bound to the neutral message in
      scenario 3), so this is an immediate GREEN with no new production code.
      Targeted run: 4 scenarios (4 passed), 17 steps (17 passed).
- [x] REFACTOR: none needed.
- [x] COMMIT: `55e164b3`.

## Scenario: A Fossil checkout without the fossil executable is refused by name

Domain constraint: a marker without its executable is refused by name before
any mutation.

- [x] RED: removed `@wip`; bound the PATH-reduced invocation and the
      missing-executable Then with `unimplemented!()`; targeted run exited
      non-zero.
- [x] GREEN: `scm::select` maps the Fossil probe's `NotFound` to
      ``Fossil checkout detected at '<root>' but the `fossil` executable was
      not found on PATH``; the custom When takes the lazy snapshot. Targeted
      run:

      ```text
      1 feature, 1 scenario (1 passed), 8 steps (8 passed)
      ```

- [x] REFACTOR: formatted; the permanent fragments suite stays green
      (117 scenarios).
- [x] COMMIT: `55e164b3 feat(archive-proof): Archive requires a version-controlled project, the checkout root, and a Fossil executable`.

## Scenario: The migration preserves independent architecture history across three proposed decisions

Domain constraint: the register holds exactly the accepted records, now 28
with ADR-072.

- [x] RED: removed `@wip`; the `28 accepted decisions` binding started as
      `unimplemented!()`; targeted run exited non-zero.
- [x] GREEN: added the 28 binding; aligned the permanent
      `givn/specs/fragments/adr-qualification-gate.feature` scenario to 28 in
      the same commit. Targeted run:

      ```text
      1 feature, 1 scenario (1 passed), 11 steps (11 passed)
      GIVN_FEATURES=givn/specs/fragments: 18 features, 117 scenarios (117 passed)
      ```

- [x] REFACTOR: removed the stale `@wip`; formatted.
- [x] COMMIT: `06808f33 feat(adr-qualification-gate): The migration preserves independent architecture history across three proposed decisions`.

## Scenario: Consumer archives a completed change and receives a proven archive

Domain constraint: the real CLI archives on both backends through the real
interface; the receipt is proven.

- [x] RED: removed `@wip`; bound the backend-parameterized e2e Given with
      `unimplemented!()` and made the e2e count step commit through the
      backend-aware fixture; the targeted `@e2e` run exited non-zero.
- [x] GREEN: the e2e fixture builds a committed Git project and a committed
      Fossil checkout and drives the compiled binary; both examples assert the
      proven receipt. Targeted e2e run:

      ```text
      GIVN_CUCUMBER_TAGS='@e2e' GIVN_FEATURES=givn/changes/archive-change/specs cargo test --test features_runner -- --name "Consumer archives a completed change and receives a proven archive"
      1 feature, 2 scenarios (2 passed), 10 steps (10 passed)
      ```

- [x] REFACTOR: removed the now-unused `Worktree::open` (dead code) and its
      test's use of it; reordered the module declarations so
      `cargo fmt --all -- --check` is clean; re-bound the release-0-3-0
      unclean-upgrade step to the neutral "clean checkout" message. Full suite
      473/473 and e2e 84/84 green.
- [x] COMMIT: `560bcd66 test(e2e): Consumer archives a completed change and receives a proven archive`.

## Finalize

- [x] Full suite green: `./run-tests.sh` 473/473 and `./run-tests.sh --e2e`
      84/84.
- [x] `cargo clippy --workspace --all-targets -- -D warnings` clean;
      `cargo fmt --all -- --check` clean.
- [x] `givn lint --change archive-change` exits 0 (`2 file(s) checked —
      clean`).
- [x] All tasks above checked with evidence and commit hashes.
