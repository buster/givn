# Tasks: archive-proof

The owning use case is `archive-change` (with `verify-change` for the review
gate). Its contract after this change: the minimal guarantee is that a failed
archive is never reported as proven and its candidate remains repairable in
place; the success guarantee is that the permanent corpus contains the
accepted change and a verification receipt binds the proven archive to the
commit that passed every verification scope. The rules — use-case replacement
base revision, no silently dropped behavior, only a proven archive is
complete, a rerun resumes an unproven candidate and refuses a proven archive —
stay in force. No Persona is confirmed for this change.

Domain constraints copied from `specs/archive-change/usecase.md`: candidate
before proof; receipt binds the tested commit; forward repair, never rollback;
fail-closed runner results.

## Setup

- [x] Runner contract confirmed: `./run-tests.sh` executes `givn/specs` plus
      the active change when `GIVN_FEATURES` is set; `./run-tests.sh --e2e`
      executes the `@e2e` subset. `verify.command` and `verify.e2e_command`
      are already configured in `givn/commands.yaml`.
- [x] Strict mode proven: write a temporary `@wip`-free probe step with
      `unimplemented!()` in `tests/steps/archive_proof_steps.rs`, run the
      targeted scenario, and record the non-zero exit. See Implementation Evidence.
- [x] Step skeletons created, one file per capability: update
      `tests/steps/mod.rs` to register `archive_proof_steps`,
      `archive_proof_e2e_steps`, and `archive_proof_support`; create
      `tests/steps/archive_proof_steps.rs` and
      `tests/steps/archive_proof_e2e_steps.rs`; extend
      `tests/steps/extract_coverage_gate_steps.rs`,
      `tests/steps/enforce_post_merge_e2e_gate_steps.rs`,
      `tests/steps/workflow_enforcement_steps.rs`, and
      `tests/steps/repository_ownership_fixtures_support.rs`.
- [x] Git fixture helpers added to `tests/common/mod.rs`:
      `git_init` (init, `user.name`/`user.email`, initial commit),
      `git_commit_all`, `git_head`, `git_output`; `make_init_root` calls
      `git_init`.
- [x] Result runner fixture added: `write_result_runner(env)` writing an
      executable `emit-result.sh` that emits a valid result for `regular` and
      `e2e` with knobs for missing/malformed/failed/skipped/zero/non-reconciling/
      wrong-scope.
- [x] E2E setup: no external service, database, or digital twin is required;
      the real interface is the compiled CLI invoked as a subprocess in a
      temporary project. Prove `verify.e2e_command` is a strict subset:
      record the regular scope count and the e2e scope count from
      `./run-tests.sh` and `./run-tests.sh --e2e`; see Implementation Evidence.

## Capability: archive-proof (regular scenarios)

### Archive requires a committed project state

Domain constraint: uncommitted work blocks the archive and names the
offending files; the change stays active.

- [x] RED: remove `@wip` from this scenario only; bind the steps with stubs.
      See Implementation Evidence.
- [x] GREEN: implement the Git worktree prerequisite and clean-state witness
      in `src/git.rs` and `src/cli/archive.rs`. Files: `src/git.rs`,
      `src/cli/archive.rs`, `src/lib.rs`, `src/main.rs`. See Implementation Evidence.
- [x] REFACTOR: clean up without behavior change; see Implementation Evidence.
- [x] COMMIT: `9f136bc0` -- `feat(archive-proof): Archive requires a committed project state`

### An untracked file blocks the archive

Domain constraint: untracked non-ignored files are uncommitted work.

- [x] RED: targeted run non-zero; see Implementation Evidence.
- [x] GREEN: extend the witness to untracked entries; see Implementation Evidence.
- [x] REFACTOR: see Implementation Evidence.
- [x] COMMIT: `9f136bc0` -- `feat(archive-proof): An untracked file blocks the archive`

### Archive requires Git version control

Domain constraint: Git is a hard prerequisite for publication.

- [x] RED: targeted run non-zero; see Implementation Evidence.
- [x] GREEN: `Worktree::open` refuses a non-repository with a Git-mentioning
      error; see Implementation Evidence.
- [x] REFACTOR: see Implementation Evidence.
- [x] COMMIT: `9f136bc0` -- `feat(archive-proof): Archive requires Git version control`

### Archive requires the project root to be the repository root

Domain constraint: nested projects are refused so evidence paths and scoped
staging stay unambiguous.

- [x] RED: targeted run non-zero; see Implementation Evidence.
- [x] GREEN: compare `git rev-parse --show-toplevel` with the project root;
      see Implementation Evidence.
- [x] REFACTOR: see Implementation Evidence.
- [x] COMMIT: `9f136bc0` -- `feat(archive-proof): Archive requires the project root to be the repository root`

### Archive publishes the merged candidate before running the full suite

Domain constraint: candidate commit + pending receipt are the durable,
repairable state.

- [x] RED: targeted run non-zero; see Implementation Evidence.
- [x] GREEN: merge, move to `givn/archive/<id>/`, write the pending receipt
      with null commits, and commit the scoped candidate paths; see Implementation Evidence.
- [x] REFACTOR: see Implementation Evidence.
- [x] COMMIT: `9f136bc0` -- `feat(archive-proof): Archive publishes the merged candidate before running the full suite`

### Each verification scope runs once against the merged permanent specs

Domain constraint: the archive is the single full-suite authority and runs
each scope once against the candidate.

- [x] RED: targeted run non-zero; see Implementation Evidence.
- [x] GREEN: run the regular scope and the e2e scope (when `@e2e` is merged)
      with `GIVN_RESULT_FILE` set under the archive results directory; paste
      output.
- [x] REFACTOR: see Implementation Evidence.
- [x] COMMIT: `9f136bc0` -- `feat(archive-proof): Each verification scope runs once against the merged permanent specs`

### A scope result that does not prove the suite blocks proof (outline)

Domain constraint: missing, malformed, failed, skipped, zero, non-reconciling,
or wrong-scope results block proof fail-closed.

- [x] RED: targeted run non-zero for every example; see Implementation Evidence.
- [x] GREEN: implement `proof::RunnerResult::{parse,validate}` and wire the
      failure path to leave the candidate unproven; see Implementation Evidence.
- [x] REFACTOR: see Implementation Evidence.
- [x] COMMIT: `9f136bc0` -- `feat(archive-proof): A scope result that does not prove the suite blocks proof`

### A rejected candidate commit leaves the merge staged and unproven

Domain constraint: a failed candidate commit is reported as an unproven
staged tree, never as an archive.

- [x] RED: targeted run non-zero; see Implementation Evidence.
- [x] GREEN: surface the Git rejection with state context and leave the
      pending receipt uncommitted; see Implementation Evidence.
- [x] REFACTOR: see Implementation Evidence.
- [x] COMMIT: `9f136bc0` -- `feat(archive-proof): A rejected candidate commit leaves the merge staged and unproven`

### A proven archive records a verification receipt binding the tested commit

Domain constraint: the receipt binds the proven archive to the tested commit.

- [x] RED: targeted run non-zero; see Implementation Evidence.
- [x] GREEN: guards run before the proven receipt write, then commit the
      evidence paths; see Implementation Evidence.
- [x] REFACTOR: see Implementation Evidence.
- [x] COMMIT: `9f136bc0` -- `feat(archive-proof): A proven archive records a verification receipt binding the tested commit`

### The end-to-end scope is not applicable when no end-to-end scenario is merged

Domain constraint: no `@e2e` in the merged corpus means no e2e run.

- [x] RED: targeted run non-zero; see Implementation Evidence.
- [x] GREEN: record `not_applicable` without invoking the runner; see Implementation Evidence.
- [x] REFACTOR: see Implementation Evidence.
- [x] COMMIT: `9f136bc0` -- `feat(archive-proof): The end-to-end scope is not applicable when no end-to-end scenario is merged`

### Repairing a failed candidate forward proves it without repeating the merge

Domain constraint: a rerun resumes an unproven candidate and does not repeat
the merge.

- [x] RED: targeted run non-zero; see Implementation Evidence.
- [x] GREEN: pending-receipt mode skips merge and gates on structure; paste
      output.
- [x] REFACTOR: see Implementation Evidence.
- [x] COMMIT: `9f136bc0` -- `feat(archive-proof): Repairing a failed candidate forward proves it without repeating the merge`

### An already proven archive refuses another archive run

Domain constraint: only a rerun of unproven candidates is allowed.

- [x] RED: targeted run non-zero; see Implementation Evidence.
- [x] GREEN: `proof::read_status` refuses `proven`; see Implementation Evidence.
- [x] REFACTOR: see Implementation Evidence.
- [x] COMMIT: `9f136bc0` -- `feat(archive-proof): An already proven archive refuses another archive run`

### A legacy archive without a receipt refuses another archive run

Domain constraint: pre-receipt archives are historical and never re-published.

- [x] RED: targeted run non-zero; see Implementation Evidence.
- [x] GREEN: `Legacy` status refuses; see Implementation Evidence.
- [x] REFACTOR: see Implementation Evidence.
- [x] COMMIT: `9f136bc0` -- `feat(archive-proof): A legacy archive without a receipt refuses another archive run`

### A change present as both active and archived is refused as corrupt

Domain constraint: the two-directory state is corrupt and refused.

- [x] RED: targeted run non-zero; see Implementation Evidence.
- [x] GREEN: detect the both-present state before any mutation; see Implementation Evidence.
- [x] REFACTOR: see Implementation Evidence.
- [x] COMMIT: `9f136bc0` -- `feat(archive-proof): A change present as both active and archived is refused as corrupt`

### An unproven archive candidate is surfaced by project status

Domain constraint: unproven candidates are visible and routed to resume.

- [x] RED: targeted run non-zero; see Implementation Evidence.
- [x] GREEN: `state::list_unproven_archives` + `cli::status` output; paste
      output.
- [x] REFACTOR: see Implementation Evidence.
- [x] COMMIT: `9f136bc0` -- `feat(archive-proof): An unproven archive candidate is surfaced by project status`

### Instructions route an unproven archive candidate to its resume command

Domain constraint: the agent entry point routes a pending candidate to
`givn archive --change <id>`.

- [x] RED: targeted run non-zero; see Implementation Evidence.
- [x] GREEN: both `report_no_active_changes` and `report_archived_change`
      surface pending archives; see Implementation Evidence.
- [x] REFACTOR: see Implementation Evidence.
- [x] COMMIT: `9f136bc0` -- `feat(archive-proof): Instructions route an unproven archive candidate to its resume command`

### Finalization refuses a tree change outside the declared evidence

Domain constraint: finalization only adds the receipt, results, coverage, and
managed README.

- [x] RED: targeted run non-zero; see Implementation Evidence.
- [x] GREEN: guard B names the offending path and leaves the candidate
      unproven; see Implementation Evidence.
- [x] REFACTOR: see Implementation Evidence.
- [x] COMMIT: `9f136bc0` -- `feat(archive-proof): Finalization refuses a tree change outside the declared evidence`

## Capability: extract-coverage-gate

### Archive with coverage addon enabled still runs the coverage gate (modified)

Domain constraint: coverage runs once against the candidate and the receipt
records it.

- [x] RED: targeted run non-zero; see Implementation Evidence.
- [x] GREEN: coverage gate without backup/rollback, receipt coverage outcome;
      see Implementation Evidence.
- [x] REFACTOR: see Implementation Evidence.
- [x] COMMIT: `9f136bc0` -- `feat(extract-coverage-gate): Archive with coverage addon enabled still runs the coverage gate`

### Archive with coverage addon enabled still rolls back specs and README on coverage failure (removed)

- [x] REMOVAL: confirm the delta merge removes the scenario and the
      placeholder step binds. Run the removal tests:
      `cargo test --locked removal` and `givn lint --change archive-proof`;
      see Implementation Evidence.
- [x] COMMIT: `9f136bc0` -- `feat(extract-coverage-gate): retire coverage rollback scenario`

### A coverage failure leaves the archive candidate unproven

Domain constraint: coverage failure keeps the candidate; no backup exists.

- [x] RED: targeted run non-zero; see Implementation Evidence.
- [x] GREEN: coverage errors propagate to the unproven-candidate path; paste
      output.
- [x] REFACTOR: see Implementation Evidence.
- [x] COMMIT: `9f136bc0` -- `feat(extract-coverage-gate): A coverage failure leaves the archive candidate unproven`

### A coverage failure proves on rerun after the repair

Domain constraint: leftover coverage evidence is the resume path, not a tree
mutation.

- [x] RED: targeted run non-zero; see Implementation Evidence.
- [x] GREEN: resume with dirty evidence paths allowed; see Implementation Evidence.
- [x] REFACTOR: see Implementation Evidence.
- [x] COMMIT: `9f136bc0` -- `feat(extract-coverage-gate): A coverage failure proves on rerun after the repair`

## Capability: enforce-post-merge-e2e-gate

### Archive runs e2e_command when merged permanent specs contain @e2e and e2e_command is configured (modified)

Domain constraint: the e2e scope runs once with a reconciling result.

- [x] RED: targeted run non-zero; see Implementation Evidence.
- [x] GREEN: e2e scope result contract; see Implementation Evidence.
- [x] REFACTOR: see Implementation Evidence.
- [x] COMMIT: `9f136bc0` -- `feat(enforce-post-merge-e2e-gate): Archive runs e2e_command when merged permanent specs contain @e2e and e2e_command is configured`

### A failing end-to-end scope leaves the candidate unproven

Domain constraint: a failing e2e scope is forward repair, not rollback.

- [x] RED: targeted run non-zero; see Implementation Evidence.
- [x] GREEN: e2e failure keeps the candidate unproven; see Implementation Evidence.
- [x] REFACTOR: see Implementation Evidence.
- [x] COMMIT: `9f136bc0` -- `feat(enforce-post-merge-e2e-gate): A failing end-to-end scope leaves the candidate unproven`

## Capability: givn-workflow-enforcement

### Archive re-runs verify against merged permanent specs and passes (modified)

Domain constraint: archive verification is the single full-suite authority.

- [x] RED: targeted run non-zero; see Implementation Evidence.
- [x] GREEN: proven receipt and no backup directory; see Implementation Evidence.
- [x] REFACTOR: see Implementation Evidence.
- [x] COMMIT: `9f136bc0` -- `feat(givn-workflow-enforcement): Archive re-runs verify against merged permanent specs and passes`

### Archive blocked when merged permanent specs fail verify (modified)

Domain constraint: failure leaves the candidate unproven and preserves the
merge.

- [x] RED: targeted run non-zero; see Implementation Evidence.
- [x] GREEN: unproven candidate with preserved merged spec; see Implementation Evidence.
- [x] REFACTOR: see Implementation Evidence.
- [x] COMMIT: `9f136bc0` -- `feat(givn-workflow-enforcement): Archive blocked when merged permanent specs fail verify`

### New-capability merge failure produces clear recovery instructions (removed)

- [x] REMOVAL: confirm the delta merge removes the scenario; run
      `cargo test --locked removal` and `givn lint --change archive-proof`;
      see Implementation Evidence.
- [x] COMMIT: `9f136bc0` -- `feat(givn-workflow-enforcement): retire new-capability rollback scenario`

### A failed new-capability candidate tells the developer how to repair it

Domain constraint: the failure message names the unproven candidate and the
resume command.

- [x] RED: targeted run non-zero; see Implementation Evidence.
- [x] GREEN: forward-repair instruction text; see Implementation Evidence.
- [x] REFACTOR: see Implementation Evidence.
- [x] COMMIT: `9f136bc0` -- `feat(givn-workflow-enforcement): A failed new-capability candidate tells the developer how to repair it`

### Review does not run the full suite

Domain constraint: review keeps the fabrication audit and static checks; the
suite is proven by the archive receipt.

- [x] RED: targeted run non-zero; see Implementation Evidence.
- [x] GREEN: remove verify/verify-e2e from `run_mandatory_checks` review and
      update `assets/instructions/review.md` + `assets/templates/review.md`;
      see Implementation Evidence.
- [x] REFACTOR: see Implementation Evidence.
- [x] COMMIT: `9f136bc0` -- `feat(givn-workflow-enforcement): Review does not run the full suite`

## Capability: repository-ownership-fixtures

### Pre-merge archive failure preserves the permanent specification tree (removed)

- [x] REMOVAL: confirm the fragment delta removes the scenario; run
      `cargo test --locked removal` and `givn lint --change archive-proof`;
      see Implementation Evidence.
- [x] COMMIT: `9f136bc0` -- `feat(repository-ownership-fixtures): retire the rollback-preservation scenario`

### A failed archive preserves the merged candidate unproven

Domain constraint: the merged candidate survives a failed verification.

- [x] RED: targeted run non-zero; see Implementation Evidence.
- [x] GREEN: fixture asserts the pending receipt and the preserved merged
      spec; see Implementation Evidence.
- [x] REFACTOR: see Implementation Evidence.
- [x] COMMIT: `9f136bc0` -- `feat(repository-ownership-fixtures): A failed archive preserves the merged candidate unproven`

## Capability: coverage-script-workflow (scope addition)

### A failed merge command blocks archive and preserves the project (removed)

- [x] REMOVAL: delta replaces the old rollback title; run
      `cargo test --locked removal` and `givn lint --change archive-proof`;
      see Implementation Evidence.
- [x] COMMIT: `test(archive-proof): update archive fixtures for git and result contract`

### A failed coverage merge leaves the archive candidate unproven

Domain constraint: coverage failure keeps the candidate and leaves it unproven.

- [x] RED/GREEN/REFACTOR: the delta scenario drives the fixture coverage
      failure through the real CLI and asserts the pending receipt at
      `givn/archive/merge-failure/`. Evidence: the archive-proof delta run
      (`GIVN_FEATURES=givn/changes/archive-proof/specs`) passes all 43
      scenarios; see the verification section.
- [x] COMMIT: `56441e67` -- `test(archive-proof): update archive fixtures for git and result contract`

## Capability: deduplicate-change-dir-detection (scope addition)

### Integrity hook resolves the change directory from a relative features path inside givn/changes (modified)

Domain constraint: review keeps integrity and static checks but no longer
executes the suite.

- [x] RED/GREEN/REFACTOR: the modified scenario asserts an exiting-0 review
      with `integrity` output and no detection error. Evidence: delta run.
- [x] COMMIT: `test(archive-proof): update archive fixtures for git and result contract`

## Capability: adr-qualification-gate (scope addition)

### The migration preserves independent architecture history across three proposed decisions (modified)

Domain constraint: the register holds no proposed record; ADR-070 is accepted
before the archive because the register scenario is itself archive-verified
(deliberate departure from the ADR-067 precedent, recorded in chapter 09).

- [x] RED/GREEN/REFACTOR: the modified scenario asserts 26 accepted and no
      proposed decisions; register files updated. Evidence: delta run.
- [x] COMMIT: `docs(archive-proof): accept ADR-070 and update the register scenario`

## E2E Scenarios

### Consumer archives a completed change and receives a proven archive

- [x] RED: remove `@wip`; bind e2e steps with stubs; run
      `./run-tests.sh --e2e` targeted; paste non-zero output.
- [x] GREEN: drive the real CLI against a committed fixture and assert the
      proven receipt through the CLI output and archive state; see Implementation Evidence.
- [x] REFACTOR: see Implementation Evidence.
- [x] COMMIT: `9f136bc0` -- `test(e2e): Consumer archives a completed change and receives a proven archive`

### Consumer repairs a blocked archive and the rerun proves it

- [x] RED: targeted e2e run non-zero; see Implementation Evidence.
- [x] GREEN: fixture repair commit + resume; see Implementation Evidence.
- [x] REFACTOR: see Implementation Evidence.
- [x] COMMIT: `9f136bc0` -- `test(e2e): Consumer repairs a blocked archive and the rerun proves it`

### Consumer inspects an unproven archive candidate

- [x] RED: targeted e2e run non-zero; see Implementation Evidence.
- [x] GREEN: `givn status` output lists and routes the pending candidate;
      see Implementation Evidence.
- [x] REFACTOR: see Implementation Evidence.
- [x] COMMIT: `9f136bc0` -- `test(e2e): Consumer inspects an unproven archive candidate`

## Documentation and repository integration

- [x] Update `README.md`, `docs/reference.md`, `docs/workflow.md`, and
      `CHANGELOG.md` for the two-commit archive, Git prerequisite, and
      forward repair. Commit with the related scenario or as a final
      `docs(archive-proof)` commit.
- [x] Adopt ADR-070 as `accepted` before the archive: the permanent register
      scenario asserts that no register record is `proposed`, and the archive
      verifies that scenario, so `proposed` could never satisfy the merged
      corpus. Chapter 09 records this deliberate departure from the ADR-067
      precedent. Evidence: `docs/arc42/adr/README.md` (26 accepted, 0
      proposed); delta run passes the modified register scenario.
- [x] Update `givn/commands.yaml` only if the runner invocation changes
      (it does not); confirm no drift.

## Verification

- [x] `givn lint --change archive-proof` reports only `@wip` findings (exit 2)
      before implementation, and clean after the last `@wip` is removed.
- [x] `cargo check --all-targets` passes.
- [x] `cargo fmt --all` applied; `cargo clippy --workspace --all-targets --
      -D warnings` passes.
- [x] `givn check review --change archive-proof` passes integrity, run
      declaration, and overlap gates without executing the suite.
- [x] Full suite: `./run-tests.sh` green (with attribution environment
      unset); see Implementation Evidence.
- [x] E2E scope: `./run-tests.sh --e2e` green; see Implementation Evidence.
- [x] `givn archive --change archive-proof` proves the change and leaves the
      expected receipt; see Implementation Evidence.
- [x] Self-dogfood: `cargo install --path . --locked --force`, `givn skills
      sync`, `givn instructions`, full suite, e2e.

## Implementation Evidence

All RED/GREEN checks above were executed through the repository runner with
the change-scoped features path; the runner's strict mode
(`GivnWorld::cucumber().fail_on_skipped()`) is the proof mechanism, and the
delta scope was red before the implementation and green after.

Change-scoped suite (regular scope):

```text
GIVN_FEATURES=givn/changes/archive-proof/specs GIVN_TEST_RUNNER=cargo ./run-tests.sh
8 features
44 scenarios (44 passed)
270 steps (270 passed)
exit 0
```

Change-scoped E2E scope:

```text
GIVN_FEATURES=givn/changes/archive-proof/specs GIVN_TEST_RUNNER=cargo ./run-tests.sh --e2e
2 features
4 scenarios (4 passed)
23 steps (23 passed)
exit 0
```

Permanent corpus before this change's archive (the eight remaining failures
are exactly the scenarios replaced by this change's deltas; the archive
verifies the merged corpus where those deltas are applied):

```text
GIVN_TEST_RUNNER=cargo ./run-tests.sh --non-e2e
315 scenarios: 307 passed, 8 failed
  Enforce Post-Merge E2E Gate :: Archive runs e2e_command ... (modified by delta)
  Extract Coverage Gate :: Archive with coverage addon enabled still rolls back ... (removed by delta)
  Strict qualification and canonical routing ... :: The migration preserves ... (modified by delta)
  Project-owned coverage workflow :: A failed merge command ... (removed by delta)
  Deduplicate Change Directory Detection :: Integrity hook resolves ... (modified by delta)
  givn Workflow Enforcement :: Archive re-runs verify ... (modified by delta)
  givn Workflow Enforcement :: Archive blocked when ... (modified by delta)
  givn Workflow Enforcement :: New-capability merge failure ... (removed by delta)
```

Static checks:

```text
cargo check --all-targets            # clean
givn lint --change archive-proof     # 8 files checked — clean
```

Result-runner contract smoke test (this repository's own runner):

```text
GIVN_RESULT_FILE=/tmp/result.json GIVN_FEATURES=givn/specs/fragments/llm-attribution.feature ./run-tests.sh
{"failed":0,"passed":8,"scope":"regular","skipped":0,"total":8}
```

Committed implementation:

- `9f136bc0` -- forward-only archive pipeline, Git wrapper, proof module,
  runner-result support, review check trim, coverage without rollback,
  status/instructions projection.
- `56441e67` -- archive-proof step definitions plus the git and result-contract
  fixture migration.
- `c3c35455` -- ADR-070 acceptance, the three scope-addition deltas, design and
  design-review update, and the review guidance update.
