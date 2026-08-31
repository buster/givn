# Tasks: readme-enforcement

Scenario-by-scenario RED→GREEN→REFACTOR→COMMIT per
`specs/readme-enforcement/readme-enforcement.feature` (in file order;
non-@e2e first, @e2e last).

## Setup: runner, strict mode, step skeletons, e2e filter proof

- [x] Runner is the existing cucumber-rs binary (`tests/features_runner.rs`,
      `.fail_on_skipped()`); register `tests/steps/readme_enforcement_steps.rs`
      and `tests/steps/readme_enforcement_e2e_steps.rs` in
      `tests/steps/mod.rs` with `unimplemented!()` skeletons.
- [x] Strictness proof: run one delta scenario with an unimplemented step;
      record non-zero exit output here:

  ```
  $ GIVN_FEATURES=givn/changes/readme-enforcement/specs \
    GIVN_SPEC_EMBEDDER=stub GIVN_SPEC_RERANKER=disabled \
    cargo test --test features_runner -- \
    --name "README check reports every open placeholder and its expectation"
  1 feature
  1 scenario (1 failed)
  2 steps (1 passed, 1 failed)
  thread 'main' panicked ... 1 step failed
  error: test failed, to rerun pass `--test features_runner`
  EXIT=101
  ```

- [x] verify.command (`./run-tests.sh`) and verify.e2e_command
      (`./run-tests.sh --e2e`) are already configured; prove the e2e filter is
      a strict subset — full-suite scenario count vs @e2e-only count:

  ```
  $ ./run-tests.sh            -> FULL_EXIT=0, 520 scenarios (all passed)
  $ ./run-tests.sh --e2e      -> E2E_EXIT=0,  94 scenarios  (all passed)
  94 < 520 -> the e2e filter is a strict subset.
  ```

- [x] No local services to start (CLI capability; design.md Local Runnability).

## Scenario 1: README check reports every open placeholder and its expectation

- [x] RED — remove @wip from this scenario only, run:
      `cargo test --test features_runner -- --name "README check reports every open placeholder and its expectation"`

  ```
  $ GIVN_FEATURES=givn/changes/readme-enforcement/specs GIVN_SPEC_EMBEDDER=stub \
    GIVN_SPEC_RERANKER=disabled cargo test --test features_runner -- \
    --name "README check reports every open placeholder and its expectation"
  EXIT=101 — 1 scenario (1 failed), unimplemented!() stub fired
  ```

- [x] GREEN — minimum code: checker + `givn readme check` dispatch.

  Files:
  - src/cli/readme.rs (checker: ReadmeFindingKind/ReadmeFinding, check_readme, run_check)
  - src/main.rs (Commands::Readme + ReadmeCommands::Check dispatch)
  - tests/steps/readme_enforcement_steps.rs (scaffolded README fixture)

  ```
  EXIT=0 — 1 scenario (1 passed). Coverage: exercised by the scenario; the
  project's coverage instrumentation runs via measure-coverage.sh at archive,
  not per targeted run.
  ```

- [x] REFACTOR

  ```
  EXIT=0 — 1 scenario (1 passed). No cleanup needed beyond the initial shape.
  ```

- [x] COMMIT — `feat(readme-enforcement): README check reports every open placeholder and its expectation`

  ```
  commit hash: c9f0971
  ```

## Scenario 2: README check reports a legacy scaffold README as needing migration

- [x] RED — `cargo test --test features_runner -- --name "README check reports a legacy scaffold README as needing migration"`

  ```
  EXIT=101 — 1 scenario (1 failed), fixture step unimplemented
  ```

- [x] GREEN — legacy matcher list in the checker.

  Files:
  - tests/steps/readme_enforcement_steps.rs (verbatim previous-scaffold fixture)

  ```
  EXIT=0 — 1 scenario (1 passed); stdout mentions "migration"
  ```

- [x] REFACTOR

  ```
  EXIT=0 — 1 scenario (1 passed)
  ```

- [x] COMMIT — `feat(readme-enforcement): README check reports a legacy scaffold README as needing migration`

  ```
  commit hash: 5f670f9
  ```

## Scenario 3: README check reports a damaged managed block

- [x] RED — `cargo test --test features_runner -- --name "README check reports a damaged managed block"`

  ```
  EXIT=101 — 1 scenario (1 failed), fixture step unimplemented
  ```

- [x] GREEN — development managed block damage detection (marker-count
      mismatch in the checker; begin/end pair rule from the design).

  Files:
  - tests/steps/readme_enforcement_steps.rs (broken-block fixture)

  ```
  EXIT=0 — 1 scenario (1 passed); stdout mentions "development managed block"
  ```

- [x] REFACTOR

  ```
  EXIT=0 — 1 scenario (1 passed); no behaviour change
  ```

- [x] COMMIT — `feat(readme-enforcement): README check reports a damaged managed block`

  ```
  commit hash: f900edd
  ```

## Scenario 4: README check reports a missing README

- [x] RED — `cargo test --test features_runner -- --name "README check reports a missing README"`

  ```
  EXIT=101 — 1 scenario (1 failed), fixture step unimplemented
  ```

- [x] GREEN — `MissingReadme` finding kind.

  Files:
  - tests/steps/readme_enforcement_steps.rs (remove-README fixture)

  ```
  EXIT=0 — 1 scenario (1 passed); stdout mentions "README.md is missing"
  ```

- [x] REFACTOR

  ```
  EXIT=0 — 1 scenario (1 passed)
  ```

- [x] COMMIT — `feat(readme-enforcement): README check reports a missing README`

  ```
  commit hash: 84f7849
  ```

## Scenario 5: README check reports a hand-written README without givn markers as unmanaged

- [x] RED — `cargo test --test features_runner -- --name "README check reports a hand-written README without givn markers as unmanaged"`

  ```
  EXIT=101 — 1 scenario (1 failed), fixture step unimplemented
  ```

- [x] GREEN — management-marker rule + `Unmanaged` finding kind (checker
      already emits both from scenario 1's implementation; the fixture pins it).

  Files:
  - tests/steps/readme_enforcement_steps.rs (hand-written fixture)

  ```
  EXIT=0 — 1 scenario (1 passed); stdout mentions "adapt the README or disable the readme addon"
  ```

- [x] REFACTOR

  ```
  EXIT=0 — 1 scenario (1 passed)
  ```

- [x] COMMIT — `feat(readme-enforcement): README check reports a hand-written README without givn markers as unmanaged`

  ```
  commit hash: 070e2a4
  ```

## Scenario 6: Review refuses sign-off without a recorded README impact decision

- [x] RED — `cargo test --test features_runner -- --name "Review refuses sign-off without a recorded README impact decision"`

  ```
  First run: compile error (fixture signature) — fixed.
  Second run: EXIT=101 — fixture step unimplemented.
  Third run: exit-0 against the current binary — the missing-impact
  behaviour genuinely does not exist (correct assertion RED).
  ```

- [x] GREEN — `README-IMPACT` parsing in the integrity hook, readme flag
      threaded through `hooks.rs` and `check.rs`.

  Files:
  - src/integrity_hook.rs (check_readme_impact, new readme_enabled parameter)
  - src/hooks.rs (integrity call site)
  - src/cli/check.rs (integrity call site)
  - tests/steps/readme_enforcement_steps.rs (reviewable-change fixture,
    impact-decision fixture, no-op verify commands fixture)

  ```
  EXIT=0 — 1 scenario (1 passed); stderr contains "README impact decision"
  ```

- [x] REFACTOR

  ```
  EXIT=0 — 1 scenario (1 passed)
  ```

- [x] COMMIT — `feat(readme-enforcement): Review refuses sign-off without a recorded README impact decision`

  ```
  commit hash: 4789df2
  ```

## Scenario 7: Review accepts an updated README impact decision naming the sections

- [x] RED — legitimate immediate GREEN: every step is a reused definition
      (`a project root with givn/ initialised`, `the project's verify commands
      are no-ops`, `a reviewable change`, the impact-decision fixture,
      `the command exits 0`); the acceptance logic shipped with scenario 6.

  ```
  EXIT=0 — 1 scenario (1 passed)
  ```

- [x] GREEN — reuse only (see RED note).

  Files:
  - (no production files — pure step reuse of scenario 6's implementation)

  ```
  EXIT=0 — 1 scenario (1 passed)
  ```

- [x] REFACTOR

  ```
  EXIT=0 — 1 scenario (1 passed)
  ```

- [x] COMMIT — `feat(readme-enforcement): Review accepts an updated README impact decision naming the sections`

  ```
  commit hash: a0d2ffd
  ```

## Scenario 8: A project without the readme addon needs no README impact decision

- [ ] RED — `cargo test --test features_runner -- --name "A project without the readme addon needs no README impact decision"`

  ```
  (paste RED output)
  ```

- [ ] GREEN — addon gating in the integrity check.

  Files:
  - (paste production files)

  ```
  (paste GREEN output)
  ```

- [ ] REFACTOR

  ```
  (paste output)
  ```

- [ ] COMMIT — `feat(readme-enforcement): A project without the readme addon needs no README impact decision`

  ```
  commit hash:
  ```

## Scenario 9: An unknown README impact value is refused

- [ ] RED — `cargo test --test features_runner -- --name "An unknown README impact value is refused"`

  ```
  (paste RED output)
  ```

- [ ] GREEN — strict value validation in the integrity check.

  Files:
  - (paste production files)

  ```
  (paste GREEN output)
  ```

- [ ] REFACTOR

  ```
  (paste output)
  ```

- [ ] COMMIT — `feat(readme-enforcement): An unknown README impact value is refused`

  ```
  commit hash:
  ```

## Scenario 10: Archive blocks while the README has open placeholders

- [ ] RED — `cargo test --test features_runner -- --name "Archive blocks while the README has open placeholders"`

  ```
  (paste RED output)
  ```

- [ ] GREEN — completeness gate call in `archive.rs` before backup/merge.

  Files:
  - (paste production files)

  ```
  (paste GREEN output)
  ```

- [ ] REFACTOR

  ```
  (paste output)
  ```

- [ ] COMMIT — `feat(readme-enforcement): Archive blocks while the README has open placeholders`

  ```
  commit hash:
  ```

## Scenario 11: Archive blocks when the README is missing

- [ ] RED — `cargo test --test features_runner -- --name "Archive blocks when the README is missing"`

  ```
  (paste RED output)
  ```

- [ ] GREEN — missing-README branch in the completeness gate.

  Files:
  - (paste production files)

  ```
  (paste GREEN output)
  ```

- [ ] REFACTOR

  ```
  (paste output)
  ```

- [ ] COMMIT — `feat(readme-enforcement): Archive blocks when the README is missing`

  ```
  commit hash:
  ```

## Scenario 12: Archive blocks for a hand-written README without givn markers

- [ ] RED — `cargo test --test features_runner -- --name "Archive blocks for a hand-written README without givn markers"`

  ```
  (paste RED output)
  ```

- [ ] GREEN — unmanaged branch in the completeness gate (reuses checker).

  Files:
  - (paste production files)

  ```
  (paste GREEN output)
  ```

- [ ] REFACTOR

  ```
  (paste output)
  ```

- [ ] COMMIT — `feat(readme-enforcement): Archive blocks for a hand-written README without givn markers`

  ```
  commit hash:
  ```

## Scenario 13: Archive does not re-insert a deleted development managed block while the README stays managed

- [ ] RED — `cargo test --test features_runner -- --name "Archive does not re-insert a deleted development managed block while the README stays managed"`

  ```
  (paste RED output)
  ```

- [ ] GREEN — refresh-if-present semantics in the archive refresh path.

  Files:
  - (paste production files)

  ```
  (paste GREEN output)
  ```

- [ ] REFACTOR

  ```
  (paste output)
  ```

- [ ] COMMIT — `feat(readme-enforcement): Archive does not re-insert a deleted development managed block while the README stays managed`

  ```
  commit hash:
  ```

## Scenario 14: Instructions stay silent about the README when it is complete

- [ ] RED — `cargo test --test features_runner -- --name "Instructions stay silent about the README when it is complete"`

  ```
  (paste RED output)
  ```

- [ ] GREEN — nudge only when findings exist (regression guard; RED proof for
      the nudge surface is scenario 19's paired evidence).

  Files:
  - (paste production files)

  ```
  (paste GREEN output)
  ```

- [ ] REFACTOR

  ```
  (paste output)
  ```

- [ ] COMMIT — `feat(readme-enforcement): Instructions stay silent about the README when it is complete`

  ```
  commit hash:
  ```

## E2E setup: prove the e2e filter and fixture conventions

- [ ] Already proven in the setup task above (counts pasted).
- [ ] Fixture convention for archive scenarios: `verify.command` passes and
      the coverage addon is satisfied or disabled
      (`tests/steps/extract_coverage_gate_steps.rs` convention).

## Scenario 15: Consumer scaffolds a project and the README marks its unfilled sections

- [ ] RED — `cargo test --test features_runner -- --tags '@e2e' --name "Consumer scaffolds a project and the README marks its unfilled sections"`

  ```
  (paste RED output)
  ```

- [ ] GREEN — template rewritten with `givn:todo` markers + development
      managed block rendered from commands.yaml; scaffold renders the block.

  Files:
  - (paste production files)

  ```
  (paste GREEN output)
  ```

- [ ] REFACTOR

  ```
  (paste output)
  ```

- [ ] COMMIT — `test(e2e): Consumer scaffolds a project and the README marks its unfilled sections`

  ```
  commit hash:
  ```

## Scenario 16: Consumer runs the README check on a complete README and it passes

- [ ] RED/GREEN — regression guard for the check happy path (RED evidence is
      scenario 1's paired refusal; this proves exit 0 end-to-end):

  ```
  (paste output)
  ```

- [ ] COMMIT — `test(e2e): Consumer runs the README check on a complete README and it passes`

  ```
  commit hash:
  ```

## Scenario 17: Consumer checks a review with a recorded README impact decision and it passes

- [ ] RED/GREEN — regression guard for the review happy path (RED evidence is
      scenario 6's paired refusal):

  ```
  (paste output)
  ```

- [ ] COMMIT — `test(e2e): Consumer checks a review with a recorded README impact decision and it passes`

  ```
  commit hash:
  ```

## Scenario 18: Consumer archives a change and the development managed block refreshes

- [ ] RED — `cargo test --test features_runner -- --tags '@e2e' --name "Consumer archives a change and the development managed block refreshes"`

  ```
  (paste RED output)
  ```

- [ ] GREEN — archive-time refresh of the development managed block.

  Files:
  - (paste production files)

  ```
  (paste GREEN output)
  ```

- [ ] REFACTOR

  ```
  (paste output)
  ```

- [ ] COMMIT — `test(e2e): Consumer archives a change and the development managed block refreshes`

  ```
  commit hash:
  ```

## Scenario 19: Consumer asks for instructions and the unfinished README is surfaced

- [ ] RED — `cargo test --test features_runner -- --tags '@e2e' --name "Consumer asks for instructions and the unfinished README is surfaced"`

  ```
  (paste RED output)
  ```

- [ ] GREEN — instructions nudge in both report paths.

  Files:
  - (paste production files)

  ```
  (paste GREEN output)
  ```

- [ ] REFACTOR

  ```
  (paste output)
  ```

- [ ] COMMIT — `test(e2e): Consumer asks for instructions and the unfinished README is surfaced`

  ```
  commit hash:
  ```

## Final verification

- [ ] `./run-tests.sh` exits 0 on the full suite:

  ```
  (paste output)
  ```

- [ ] `./run-tests.sh --e2e` exits 0:

  ```
  (paste output)
  ```

- [ ] E2E filter proof (strict subset counts) recorded in the setup task.
