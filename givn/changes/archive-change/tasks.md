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

- [ ] Runner contract confirmed: `./run-tests.sh` executes the repository
      feature suite plus the active change when `GIVN_FEATURES` is set;
      `./run-tests.sh --e2e` executes the `@e2e` subset.
      `verify.e2e_command` is `./run-tests.sh --e2e`, strictly narrower than
      `verify.command`.
- [ ] Step skeletons updated, one file per capability plus its E2E companion:
      `tests/steps/archive_proof_steps.rs`,
      `tests/steps/archive_proof_e2e_steps.rs`,
      `tests/steps/archive_proof_support.rs`, and the
      `adr_qualification_gate_steps.rs` count binding. New step bodies start
      as `unimplemented!()`.
- [ ] Strict mode proven with a targeted run of the first scenario: the
      cucumber runner sets `.fail_on_skipped()` in
      `tests/features_runner.rs`; the first RED run must exit non-zero with
      `not implemented`. Paste command and output.
- [ ] Fossil test dependency present: `fossil version` resolves; the CI
      workflow gains `sudo apt-get update && sudo apt-get install -y
      --no-install-recommends fossil` before the acceptance suites, and
      `AGENTS.md` records the prerequisite.
- [ ] E2E count proof: the change scope runs the full set and the `@e2e`
      scope a strictly smaller set. Paste both outputs.

## Scenario: Archive publishes the merged candidate before running the full suite

Domain constraint: the candidate revision contains exactly the merged corpus
paths and the pending receipt; on a failed suite it stays published and
unproven.

- [ ] RED: removed `@wip` from this scenario only; bound the
      backend-parameterized Given, the publication Thens, and the exact-content
      Then with `unimplemented!()` where not reused. Targeted run exited
      non-zero. Paste output.
- [ ] GREEN: `src/scm.rs` returns `Backend::Fossil` for a Fossil checkout with
      the executable available; new `src/fossil.rs` implements `revision`,
      `dirty_entries`, `publish`, `revision_contains`; `src/cli/archive.rs`
      moves onto `scm::select` and `Backend::publish`; the fixtures gain a real
      Fossil checkout (`tests/common/mod.rs`, `archive_proof_support.rs`).
      List production files touched. Targeted run exited zero. Paste output.
- [ ] REFACTOR: clean up without behavior change; targeted run still exited
      zero. Paste output.
- [ ] COMMIT: one atomic commit referencing the scenario title; hash recorded:
      `<hash>`

## Scenario: Repairing a failed candidate forward proves it without repeating the merge

Domain constraint: the published candidate stays in place; the repair adds a
revision; the rerun proves without re-merging.

- [ ] RED: removed `@wip`; bound the backend-parameterized Given and the
      repaired-revision Then; targeted run exited non-zero. Paste output.
- [ ] GREEN: resume uses the committed revision only
      (`git cat-file -e HEAD:<path>`, `fossil cat <path>`), skips the merge,
      and proves the repaired revision on both backends. Targeted run exited
      zero. Paste output.
- [ ] REFACTOR: clean up; targeted run still exited zero. Paste output.
- [ ] COMMIT: hash recorded: `<hash>`

## Scenario: A proven archive records a verification receipt binding the tested revision

Domain constraint: the receipt binds the tested revision; the finalization
revision contains only the resolved evidence paths.

- [ ] RED: removed `@wip`; bound the revision/finalization Thens; targeted run
      exited non-zero. Paste output.
- [ ] GREEN: renamed the receipt fields to `candidate_revision`/`tested_revision`
      (`src/proof.rs`, `src/cli/archive.rs`, step readers); the finalization
      assertion lists the revision's paths on both backends. Targeted run
      exited zero. Paste output.
- [ ] REFACTOR: clean up; targeted run still exited zero. Paste output.
- [ ] COMMIT: hash recorded: `<hash>`

## Scenario: A rejected candidate revision leaves the archive unproven and repairable

Domain constraint: a rejected publication leaves the archive unproven and
repairable; the rerun publishes the on-disk candidate again and proves it.

- [ ] RED: removed `@wip`; bound the hook Givens/Whens and the tested-revision
      Then; targeted run exited non-zero. Paste output.
- [ ] GREEN: on resume, when the current revision does not contain the pending
      receipt, republish the on-disk candidate (no merge, no gates, no Project
      witness) and rely on `Backend::publish` validation. Both examples pass.
      Targeted run exited zero. Paste output.
- [ ] REFACTOR: clean up; targeted run still exited zero. Paste output.
- [ ] COMMIT: hash recorded: `<hash>`

## Scenario: A rejected candidate revision on Git leaves the merge staged

Domain constraint: the Git intermediate state is staged but uncommitted; the
archive is unproven.

- [ ] RED: removed `@wip`; bound the staged-state Then; targeted run exited
      non-zero. Paste output.
- [ ] GREEN: the Git rejection message names the staged merge and the unproven
      archive. Targeted run exited zero. Paste output.
- [ ] REFACTOR: clean up; targeted run still exited zero. Paste output.
- [ ] COMMIT: hash recorded: `<hash>`

## Scenario: A rejected candidate revision on Fossil leaves the merged state uncommitted

Domain constraint: the Fossil intermediate state is present in the checkout but
uncommitted; the archive is unproven.

- [ ] RED: removed `@wip`; bound the checkout-state Then; targeted run exited
      non-zero. Paste output.
- [ ] GREEN: the Fossil rejection message names the uncommitted checkout state
      and the unproven archive. Targeted run exited zero. Paste output.
- [ ] REFACTOR: clean up; targeted run still exited zero. Paste output.
- [ ] COMMIT: hash recorded: `<hash>`

## Scenario: A Fossil EXTRA file blocks the archive

Domain constraint: mirrored strictness — Fossil `EXTRA` files block like Git
untracked files, and the refusal mutates nothing.

- [ ] RED: removed `@wip`; bound the EXTRA Given and the snapshot Then;
      targeted run exited non-zero. Paste output.
- [ ] GREEN: the Fossil clean-state witness includes `EXTRA` entries and names
      them. Targeted run exited zero. Paste output.
- [ ] REFACTOR: clean up; targeted run still exited zero. Paste output.
- [ ] COMMIT: hash recorded: `<hash>`

## Scenario: A Fossil edited file blocks the archive

Domain constraint: the Fossil witness covers `EDITED` entries, not only
`EXTRA`.

- [ ] RED: removed `@wip`; bound the edited-file Given; targeted run exited
      non-zero. Paste output.
- [ ] GREEN: the witness reports `EDITED` paths and names them. Targeted run
      exited zero. Paste output.
- [ ] REFACTOR: clean up; targeted run still exited zero. Paste output.
- [ ] COMMIT: hash recorded: `<hash>`

## Scenario: Archive requires a version-controlled project

Domain constraint: no usable backend is refused by name before any mutation.

- [ ] RED: removed `@wip`; bound the snapshot Then; targeted run exited
      non-zero. Paste output.
- [ ] GREEN: `scm::select`'s no-backend refusal carries the archive's neutral
      wording and the change directory survives. Targeted run exited zero.
      Paste output.
- [ ] REFACTOR: clean up; targeted run still exited zero. Paste output.
- [ ] COMMIT: hash recorded: `<hash>`

## Scenario: Archive requires the project root to be the checkout root

Domain constraint: a nested project root is refused by name before any
mutation.

- [ ] RED: removed `@wip`; bound the nested-root Then; targeted run exited
      non-zero. Paste output.
- [ ] GREEN: the archive consumes `scm::select`'s boundary refusal and mutates
      nothing. Targeted run exited zero. Paste output.
- [ ] REFACTOR: clean up; targeted run still exited zero. Paste output.
- [ ] COMMIT: hash recorded: `<hash>`

## Scenario: A Fossil checkout without the fossil executable is refused by name

Domain constraint: a marker without its executable is refused by name before
any mutation.

- [ ] RED: removed `@wip`; bound the PATH-reduced invocation and the
      missing-executable Then; targeted run exited non-zero. Paste output.
- [ ] GREEN: `scm::select` maps `NotFound` for the Fossil probe to the named
      refusal. Targeted run exited zero. Paste output.
- [ ] REFACTOR: clean up; targeted run still exited zero. Paste output.
- [ ] COMMIT: hash recorded: `<hash>`

## Scenario: The migration preserves independent architecture history across three proposed decisions

Domain constraint: the register holds exactly the accepted records, now 28
with ADR-072.

- [ ] RED: removed `@wip` from the delta scenario; the `28 accepted decisions`
      binding starts as `unimplemented!()`; targeted run exited non-zero.
      Paste output.
- [ ] GREEN: added the count binding; aligned the permanent
      `givn/specs/fragments/adr-qualification-gate.feature` scenario to 28 in
      the same commit. Targeted run exited zero. Paste output.
- [ ] REFACTOR: clean up; targeted run still exited zero. Paste output.
- [ ] COMMIT: hash recorded: `<hash>`

## Scenario: Consumer archives a completed change and receives a proven archive

Domain constraint: the real CLI archives on both backends through the real
interface; the receipt is proven.

- [ ] RED: removed `@wip` from the `@e2e` outline; the e2e Given/Then bodies
      start as `unimplemented!()`; `verify.e2e_command` targeted at the
      scenario exited non-zero. Paste output.
- [ ] GREEN: the e2e fixture builds a committed Git project and a committed
      Fossil checkout, runs the compiled binary, and asserts the proven
      receipt on both examples. Targeted e2e run exited zero. Paste output.
- [ ] REFACTOR: clean up; targeted e2e run still exited zero. Paste output.
- [ ] COMMIT: hash recorded: `<hash>`

## Finalize

- [ ] Full suite green: `./run-tests.sh` and `./run-tests.sh --e2e`.
- [ ] `cargo clippy --workspace --all-targets -- -D warnings` clean.
- [ ] `givn lint --change archive-change` exits 0.
- [ ] All tasks above checked with evidence and commit hashes.
