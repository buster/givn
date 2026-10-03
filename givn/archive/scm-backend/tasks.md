# Tasks: scm-backend

Owning behavior: the reusable `support` fragment, capability
`scm-backend-selection`, plus a wording modification of `release-0-3-0`.

Domain constraints copied from the fragment contract
(`givn/ideation/multi-scm-support/use-cases/scm-backend.md`):

- Selection order is fixed: Git worktree, then Fossil checkout, then a named
  refusal.
- Selection never mutates the project; every refusal returns before any write.
- A refusal names the reason, the project root, and the missing prerequisite.
- The project boundary is the Git worktree top level; a nested root is refused
  by name.
- Capability facts per backend include the clean-state witness, revision
  identifier, subset publication, hook support, attribution support, and the
  boundary rule.
- Confirmed Personas: developer--primary-user, agent--technical-role,
  givn-maintainer--owner (review lenses, not Gherkin actors).

`release-0-3-0` constraint: a project without a usable backend and a project
with uncommitted work are both protected before any file is written.

## Setup

- [x] Runner contract confirmed: `./run-tests.sh` executes the repository
      feature suite plus the active change when `GIVN_FEATURES` is set;
      `./run-tests.sh --e2e` executes the `@e2e` subset. `verify.e2e_command`
      is `./run-tests.sh --e2e`, strictly narrower than `verify.command`.
- [x] Step skeletons created, one file per capability plus its E2E companion:
      `tests/steps/scm_backend_selection_steps.rs` and
      `tests/steps/scm_backend_selection_e2e_steps.rs`, registered in
      `tests/steps/mod.rs`.
- [x] Strict mode proven: `GivnWorld::cucumber().fail_on_skipped()` in
      `tests/features_runner.rs`; the first targeted run of the first scenario
      panicked in the unimplemented Given and exited non-zero:

      ```text
      GIVN_FEATURES=givn/changes/scm-backend/specs cargo test --test features_runner -- --name "A project without version control is refused by name"
      1 feature
      1 scenario (1 failed)
      1 step (1 failed)
      Step panicked. Captured output: not implemented
      error: test failed, to rerun pass `--test features_runner`
      exit 1
      ```

- [x] No local service, isolated network, database, or digital twin is
      required; the only external executables are `git` (test dependency) and
      the compiled `givn` binary. The Fossil fixture is marker-only.
- [x] E2E count proof: the change scope contains 8 scenarios; the full scope
      runs 8 and the `@e2e` scope runs 1 (strictly smaller).

      ```text
      GIVN_FEATURES=givn/changes/scm-backend/specs ./run-tests.sh
      8 tests run: 8 passed, 0 skipped

      GIVN_FEATURES=givn/changes/scm-backend/specs ./run-tests.sh --e2e
      1 test run: 1 passed, 7 skipped
      ```

## Scenario: A project without version control is refused by name

Domain constraint: the managed upgrade refuses a project with no usable
backend before writing, naming the project root and the missing
version-controlled project.

- [x] RED: removed `@wip` from this scenario only; the steps were bound with
      `unimplemented!()` bodies. Targeted run exited non-zero.

      ```text
      GIVN_FEATURES=givn/changes/scm-backend/specs cargo test --test features_runner -- --name "A project without version control is refused by name"
        ✘  Given a project root without a Git worktree or a Fossil checkout
           Step panicked. Captured output: not implemented
      1 scenario (1 failed), 1 step (1 failed)
      exit 1
      ```

- [x] GREEN: added `src/scm.rs` (`Backend`, `select`, Git probe, no-backend
      refusal) and moved the managed-upgrade preflight onto it in
      `src/cli/migration.rs`; registered the module in `src/lib.rs` and
      `src/main.rs`. Production files: `src/scm.rs` (new), `src/lib.rs`,
      `src/main.rs`, `src/cli/migration.rs`. Targeted run exited 0.

      ```text
      GIVN_FEATURES=givn/changes/scm-backend/specs cargo test --test features_runner -- --name "A project without version control is refused by name"
      1 scenario (1 passed)
      5 steps (5 passed)
      exit 0
      ```

- [x] REFACTOR: removed the duplicated `ensure_clean_git` Git probing from
      `src/cli/migration.rs`; the seam owns selection and the dirty check
      stays on the backend. `rustfmt` applied to the new files,
      `cargo clippy --workspace --all-targets -- -D warnings` passes, and the
      targeted run still exits 0.

      ```text
      GIVN_FEATURES=givn/changes/scm-backend/specs cargo test --test features_runner -- --name "A project without version control is refused by name"
      1 scenario (1 passed)
      5 steps (5 passed)
      exit 0
      ```

- [x] COMMIT: `b61e58f5` -- `feat(scm-backend-selection): A project without version control is refused by name`

## Scenario: A Fossil checkout is refused by name until the Fossil backend is available

Domain constraint: a Fossil checkout is recognized as a version-controlled
project and refused by name; nothing is changed.

- [x] RED: removed `@wip`; the Given was still `unimplemented!()`. Targeted run
      exited non-zero.

      ```text
      GIVN_FEATURES=givn/changes/scm-backend/specs cargo test --test features_runner -- --name "A Fossil checkout is refused by name until the Fossil backend is available"
        ✘  Given a Fossil checkout whose project root is the checkout root
           Step panicked. Captured output: not implemented
      1 scenario (1 failed), 1 step (1 failed)
      exit 1
      ```

- [x] GREEN: added the Fossil marker probe (`.fslckout`/`_FOSSIL_`) and the
      named refusal in `src/scm.rs`; the Given writes a `.fslckout` marker in
      an initialized project without Git. Targeted run exited 0.

      ```text
      GIVN_FEATURES=givn/changes/scm-backend/specs cargo test --test features_runner -- --name "A Fossil checkout is refused by name until the Fossil backend is available"
      1 scenario (1 passed)
      5 steps (5 passed)
      exit 0
      ```

- [x] REFACTOR: the marker lookup is one helper reused by the ambiguity path;
      `rustfmt` and `cargo clippy --workspace --all-targets -- -D warnings`
      pass; targeted run still exits 0.

      ```text
      GIVN_FEATURES=givn/changes/scm-backend/specs cargo test --test features_runner -- --name "A Fossil checkout is refused by name until the Fossil backend is available"
      1 scenario (1 passed)
      5 steps (5 passed)
      exit 0
      ```

- [x] COMMIT: `823f9dbf` -- `feat(scm-backend-selection): A Fossil checkout is refused by name until the Fossil backend is available`

## Scenario: A project root with both markers is refused as ambiguous

Domain constraint: Git is never silently preferred; the refusal names both
markers and nothing is changed.

- [x] RED: removed `@wip`; the Given was still `unimplemented!()`. Targeted run
      exited non-zero.

      ```text
      GIVN_FEATURES=givn/changes/scm-backend/specs cargo test --test features_runner -- --name "A project root with both markers is refused as ambiguous"
      1 scenario (1 failed), 1 step (1 failed)
      Step panicked. Captured output: not implemented
      exit 1
      ```

- [x] GREEN: the ambiguity branch in `src/scm.rs` names the marker found; the
      Given adds a `.fslckout` marker to a real Git worktree. Targeted run
      exited 0.

      ```text
      GIVN_FEATURES=givn/changes/scm-backend/specs cargo test --test features_runner -- --name "A project root with both markers is refused as ambiguous"
      1 scenario (1 passed)
      5 steps (5 passed)
      exit 0
      ```

- [x] REFACTOR: the decision stays one match over both probe results;
      `rustfmt` and clippy pass; targeted run still exits 0.

      ```text
      GIVN_FEATURES=givn/changes/scm-backend/specs cargo test --test features_runner -- --name "A project root with both markers is refused as ambiguous"
      1 scenario (1 passed)
      5 steps (5 passed)
      exit 0
      ```

- [x] COMMIT: `070612d0` -- `feat(scm-backend-selection): A project root with both markers is refused as ambiguous`

## Scenario: A project root nested inside a larger Git worktree is refused by name

Domain constraint: the project boundary is the Git worktree top level; the
refusal names the project root and the worktree top level.

- [x] RED: removed `@wip`; the Given was still `unimplemented!()`. Targeted run
      exited non-zero.

      ```text
      GIVN_FEATURES=givn/changes/scm-backend/specs cargo test --test features_runner -- --name "A project root nested inside a larger Git worktree is refused by name"
      1 scenario (1 failed), 1 step (1 failed)
      Step panicked. Captured output: not implemented
      exit 1
      ```

- [x] GREEN: added the boundary check (`rev-parse --show-toplevel`,
      canonicalized equality) with the neutral refusal in `src/scm.rs` and
      `Worktree::at` in `src/git.rs`; the Given nests an initialized project
      inside a Git worktree and sets `cwd_override`. Targeted run exited 0.

      ```text
      GIVN_FEATURES=givn/changes/scm-backend/specs cargo test --test features_runner -- --name "A project root nested inside a larger Git worktree is refused by name"
      1 scenario (1 passed)
      5 steps (5 passed)
      exit 0
      ```

- [x] REFACTOR: shared `canonical()` for the seam's comparison; the first run
      exposed that the no-mutation step compared the outer root instead of the
      `cwd_override`, fixed to snapshot the invocation root; `rustfmt` and
      clippy pass; targeted run still exits 0.

      ```text
      GIVN_FEATURES=givn/changes/scm-backend/specs cargo test --test features_runner -- --name "A project root nested inside a larger Git worktree is refused by name"
      1 scenario (1 passed)
      5 steps (5 passed)
      exit 0
      ```

- [x] COMMIT: `e72119d4` -- `feat(scm-backend-selection): A project root nested inside a larger Git worktree is refused by name`

## Scenario: A Git worktree without the Git executable is refused by name

Domain constraint: a selected backend whose executable is missing is refused
by name before anything is changed.

- [x] RED: removed `@wip`; the Given was still `unimplemented!()`. Targeted run
      exited non-zero.

      ```text
      GIVN_FEATURES=givn/changes/scm-backend/specs cargo test --test features_runner -- --name "A Git worktree without the Git executable is refused by name"
      1 scenario (1 failed), 1 step (1 failed)
      Step panicked. Captured output: not implemented
      exit 1
      ```

- [x] GREEN: `io::ErrorKind::NotFound` in `git_worktree` now refuses by name
      when a `.git` entry is present; the When invokes the compiled binary
      with `PATH` set to an empty directory. Targeted run exited 0.

      ```text
      GIVN_FEATURES=givn/changes/scm-backend/specs cargo test --test features_runner -- --name "A Git worktree without the Git executable is refused by name"
      1 scenario (1 passed)
      5 steps (5 passed)
      exit 0
      ```

- [x] REFACTOR: one executable-availability branch inside the existing probe;
      `rustfmt` and clippy pass; targeted run still exits 0.

      ```text
      GIVN_FEATURES=givn/changes/scm-backend/specs cargo test --test features_runner -- --name "A Git worktree without the Git executable is refused by name"
      1 scenario (1 passed)
      5 steps (5 passed)
      exit 0
      ```

- [x] COMMIT: `b1d1a857` -- `feat(scm-backend-selection): A Git worktree without the Git executable is refused by name`

## Scenario: A project outside Git is protected from a partial automatic migration

Owning capability: `release-0-3-0` (modified). Domain constraint: the
automatic route fails before changing project files and explains that a
version-controlled project is required.

- [x] RED: the modified Then text had no binding while the permanent scenario
      kept its old text; the targeted run failed on the shared
      `command_fails_before_file_changes` assertion against the new neutral
      message. Targeted run exited non-zero.

      ```text
      GIVN_FEATURES=givn/changes/scm-backend/specs cargo test --test features_runner -- --name "A project outside Git is protected from a partial automatic migration"
        ✘  Then the command fails before changing project files
           Step panicked. Captured output: error: no Git worktree or Fossil checkout found at '/tmp/.tmpDbTEG6'; a version-controlled project is required
      1 scenario (1 failed)
      exit 1
      ```

- [x] GREEN: added the new Then binding and accepted the neutral message in
      `command_fails_before_file_changes` while keeping the old bindings for
      the permanent scenario. Targeted run exited 0.

      ```text
      GIVN_FEATURES=givn/changes/scm-backend/specs cargo test --test features_runner -- --name "A project outside Git is protected from a partial automatic migration"
      1 scenario (1 passed)
      4 steps (4 passed)
      exit 0
      ```

- [x] REFACTOR: both Then texts stay bound to their assertions until the
      archive replaces the permanent scenario; `rustfmt` and clippy pass;
      targeted run still exits 0.

      ```text
      GIVN_FEATURES=givn/changes/scm-backend/specs cargo test --test features_runner -- --name "A project outside Git is protected from a partial automatic migration"
      1 scenario (1 passed)
      4 steps (4 passed)
      exit 0
      ```

- [x] COMMIT: `037fdaf5` -- `feat(release-0-3-0): A project outside Git is protected from a partial automatic migration`

## Scenario: The migration preserves independent architecture history across three proposed decisions

Owning capability: `adr-qualification-gate` (modified). Domain constraint: the
consolidated ADR register counts only the records the canonical sources hold;
adding ADR-071 moves the accepted count from 26 to 27.

- [x] RED: the delta's new Then text had no binding. Targeted run exited
      non-zero.

      ```text
      GIVN_FEATURES=givn/changes/scm-backend/specs cargo test --test features_runner -- --name "The migration preserves independent architecture history across three proposed decisions"
        ✘  Then the final register has 27 accepted decisions and no proposed decisions
           Step doesn't match any function
      1 scenario (1 failed)
      exit 1
      ```

- [x] GREEN: added the 27/0 binding next to the existing 26/0 binding. Targeted
      run exited 0.

      ```text
      GIVN_FEATURES=givn/changes/scm-backend/specs cargo test --test features_runner -- --name "The migration preserves independent architecture history across three proposed decisions"
      1 scenario (1 passed)
      11 steps (11 passed)
      exit 0
      ```

- [x] REFACTOR: the count assertion stays one step per register state; the
      permanent 26/0 binding remains until the archive replaces the scenario.
      `rustfmt` and clippy pass; targeted run still exits 0.

      ```text
      GIVN_FEATURES=givn/changes/scm-backend/specs cargo test --test features_runner -- --name "The migration preserves independent architecture history across three proposed decisions"
      1 scenario (1 passed)
      11 steps (11 passed)
      exit 0
      ```

- [x] COMMIT: `9d3770f2` -- `feat(adr-qualification-gate): The migration preserves independent architecture history across three proposed decisions`

## E2E Scenario: A Git worktree at the project root selects the Git backend

Domain constraint: a Git project keeps its behavior; the managed upgrade
publishes a revision through the selection seam.

- [x] RED: removed `@wip`; the fixture Givens and the e2e Then were
      `unimplemented!()`. The targeted E2E run exited non-zero.

      ```text
      GIVN_FEATURES=givn/changes/scm-backend/specs ./run-tests.sh --e2e
      1 test run: 0 passed, 1 failed, 6 skipped
      FAIL ... fragments/scm-backend-selection.feature::A Git worktree at the project root selects the Git backend
      exit 1
      ```

- [x] GREEN: finished the `Backend` operations (`revision`, `stage_paths`,
      `has_staged_changes`, `commit_staged`) in `src/git.rs`/`src/scm.rs` and
      routed the managed upgrade through them; the e2e fixture commits a
      missing generated integration, then the real binary restores it in a new
      revision. Targeted E2E run exited 0.

      ```text
      GIVN_FEATURES=givn/changes/scm-backend/specs ./run-tests.sh --e2e
      1 test run: 1 passed, 6 skipped
      exit 0
      ```

- [x] REFACTOR: removed the local `git`, `stage_paths`,
      `has_staged_changes`, and `git_commit_hash` helpers from
      `src/cli/migration.rs`; the unit test uses `scm::select` +
      `Backend::revision`. `rustfmt`, `cargo check --all-targets`, and clippy
      pass; the change-scoped full run passes 7/7 and the E2E run 1/1.

      ```text
      GIVN_FEATURES=givn/changes/scm-backend/specs ./run-tests.sh
      7 tests run: 7 passed, 0 skipped
      ```

- [x] COMMIT: `d96ffff4` -- `test(e2e): A Git worktree at the project root selects the Git backend`

## Verification

- [x] `givn lint --change scm-backend` reports only expected findings:
      `3 file(s) checked — clean` (exit 0).
- [x] `./run-tests.sh` (full permanent suite) exits 0: `467 tests run:
      467 passed, 0 skipped`.
- [x] `./run-tests.sh --e2e` exits 0: `83 tests run: 83 passed, 384 skipped`.
- [x] `cargo check --all-targets` passes.
- [x] `cargo clippy --workspace --all-targets -- -D warnings` passes.
- [x] `cargo fmt --all -- --check` reports only pre-existing drift
      (confirmed at HEAD with a clean stash); all files touched by this change
      are rustfmt-clean.
- [x] `givn check review --change scm-backend` passes: integrity, run
      declaration, and overlap dispositions; net delta 6 added, 2 modified.
- [x] Discovered during the full-suite run: adding ADR-071 changes the
      consolidated-register count, so the `adr-qualification-gate` scenario was
      modified (26 → 27) and the permanent spec aligned in
      `chore(scm-backend): align permanent specs with the delta` (`c3cfa1de`).
