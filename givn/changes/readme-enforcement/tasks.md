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

- [x] RED — legitimate immediate GREEN: all steps reused (`givn addon "readme"
      is "false"` is an existing definition; the addon gating shipped with
      scenario 6's integrity-hook threading).

  ```
  EXIT=0 — 1 scenario (1 passed)
  ```

- [x] GREEN — reuse only (addon gating via `manifest.addons.readme`).

  Files:
  - (no production files — pure step reuse)

  ```
  EXIT=0 — 1 scenario (1 passed)
  ```

- [x] REFACTOR

  ```
  EXIT=0 — 1 scenario (1 passed)
  ```

- [x] COMMIT — `feat(readme-enforcement): A project without the readme addon needs no README impact decision`

  ```
  commit hash: 31bdb12
  ```

## Scenario 9: An unknown README impact value is refused

- [x] RED — legitimate immediate GREEN: all steps reused; the strict value
      validation shipped with scenario 6's `check_readme_impact`.

  ```
  EXIT=0 — 1 scenario (1 passed)
  ```

- [x] GREEN — reuse only (strict `README-IMPACT` value validation).

  Files:
  - (no production files — pure step reuse)

  ```
  EXIT=0 — 1 scenario (1 passed)
  ```

- [x] REFACTOR

  ```
  EXIT=0 — 1 scenario (1 passed)
  ```

- [x] COMMIT — `feat(readme-enforcement): An unknown README impact value is refused`

  ```
  commit hash: c8533d5
  ```

## Scenario 10: Archive blocks while the README has open placeholders

- [x] RED — `cargo test --test features_runner -- --name "Archive blocks while the README has open placeholders"`

  ```
  Intermediate iterations: parse-error fix (fixture line placement),
  120 s timeout (subprocess downloaded BGE artifacts — fixed via the
  offline semantic-gate fixture: E5 stub, BGE stub, reranker disabled),
  wrong-reason block (coverage gate; fixed via `givn addon "coverage"
  is "false"`), stdout-vs-stderr report fix.
  Final RED: EXIT=101 — "expected failure, got exit Some(0)":
  the archive succeeded because the completeness gate did not exist.
  ```

- [x] GREEN — completeness gate call in `archive.rs` before backup/merge;
      report printed on stdout before bailing.

  Files:
  - src/cli/archive.rs (gate block + finding_label import)
  - src/cli/readme.rs (finding_label visibility)
  - tests/steps/readme_enforcement_steps.rs (offline semantic-gate fixture)
  - specs/readme-enforcement/readme-enforcement.feature (offline + coverage-off Givens)

  ```
  EXIT=0 — 1 scenario (1 passed)
  ```

- [x] REFACTOR

  ```
  EXIT=0 — 1 scenario (1 passed) (unfiltered run, permanent specs scope)
  ```

- [x] COMMIT — `feat(readme-enforcement): Archive blocks while the README has open placeholders`

  ```
  commit hash: 421ea6f
  ```

## Scenario 11: Archive blocks when the README is missing

- [x] RED — legitimate immediate GREEN: all steps reused; the gate reuses the
      checker whose `MissingReadme` branch shipped with scenario 4 and the
      gate call shipped with scenario 10.

  ```
  EXIT=0 — 1 scenario (1 passed)
  ```

- [x] GREEN — reuse only (checker + gate; missing-README branch pinned
      through the archive path).

  Files:
  - (no production files — pure step reuse)

  ```
  EXIT=0 — 1 scenario (1 passed)
  ```

- [x] REFACTOR

  ```
  EXIT=0 — 1 scenario (1 passed)
  ```

- [x] COMMIT — `feat(readme-enforcement): Archive blocks when the README is missing`

  ```
  commit hash: f4c8bc5
  ```

## Scenario 12: Archive blocks for a hand-written README without givn markers

- [x] RED — legitimate immediate GREEN: all steps reused; the `Unmanaged`
      branch ships in the checker (scenario 5) and the gate call in
      scenario 10.

  ```
  EXIT=0 — 1 scenario (1 passed)
  ```

- [x] GREEN — reuse only (unmanaged branch pinned through the archive path).

  Files:
  - (no production files — pure step reuse)

  ```
  EXIT=0 — 1 scenario (1 passed)
  ```

- [x] REFACTOR

  ```
  EXIT=0 — 1 scenario (1 passed)
  ```

- [x] COMMIT — `feat(readme-enforcement): Archive blocks for a hand-written README without givn markers`

  ```
  commit hash: 89aa21d
  ```

## Scenario 13: Archive does not re-insert a deleted development managed block while the README stays managed

- [x] RED — `cargo test --test features_runner -- --name "Archive does not re-insert a deleted development managed block while the README stays managed"`

  ```
  EXIT=101 — 1 scenario (1 failed): fixture steps unimplemented
  ```

- [x] GREEN — refresh-if-present semantics pinned: the archive's refresh does
      not exist yet, so the archive leaves the README byte-identical (the
      no-reinsert behaviour is trivially true pre-implementation and becomes
      load-bearing once refresh lands in scenario 18).

  Files:
  - tests/steps/readme_enforcement_steps.rs (badge-only fixture, no-dev-block Then)

  ```
  EXIT=0 — 1 scenario (1 passed)
  ```

- [x] REFACTOR

  ```
  EXIT=0 — 1 scenario (1 passed)
  ```

- [x] COMMIT — `feat(readme-enforcement): Archive does not re-insert a deleted development managed block while the README stays managed`

  ```
  commit hash: d4b0fb7
  ```

## Scenario 14: Instructions stay silent about the README when it is complete

- [x] RED — `cargo test --test features_runner -- --name "Instructions stay silent about the README when it is complete"`

  ```
  EXIT=101 — 1 scenario (1 failed): fixture step unimplemented
  ```

- [x] GREEN — nudge only when findings exist (regression guard; RED proof for
      the nudge surface is scenario 19's paired evidence). The current binary
      has no nudge, so silence holds trivially until scenario 19 adds it.

  Files:
  - tests/steps/readme_enforcement_steps.rs (complete-README fixture)

  ```
  EXIT=0 — 1 scenario (1 passed)
  ```

- [x] REFACTOR

  ```
  EXIT=0 — 1 scenario (1 passed)
  ```

- [x] COMMIT — `feat(readme-enforcement): Instructions stay silent about the README when it is complete`

  ```
  commit hash: 5ae1b34
  ```

## E2E setup: prove the e2e filter and fixture conventions

- [ ] Already proven in the setup task above (counts pasted).
- [ ] Fixture convention for archive scenarios: `verify.command` passes and
      the coverage addon is satisfied or disabled
      (`tests/steps/extract_coverage_gate_steps.rs` convention).

## Scenario 15: Consumer scaffolds a project and the README marks its unfilled sections

- [x] RED — targeted e2e run (note: in cucumber-rs 0.23 `--tags` and `--name`
      are mutually exclusive — design.md corrected; `--name` alone scopes the
      delta precisely):

  ```
  $ GIVN_FEATURES=givn/changes/readme-enforcement/specs GIVN_SPEC_EMBEDDER=stub \
    GIVN_SPEC_RERANKER=disabled cargo test --no-default-features \
    --features spec-retrieval --test features_runner -- \
    --name "Consumer scaffolds a project and the README marks its unfilled sections"
  EXIT=101 — 1 scenario (1 failed): assertion steps unimplemented
  ```

- [x] GREEN — template rewritten with `givn:todo` markers + development
      managed block rendered from commands.yaml; scaffold renders the block.

  Files:
  - assets/addons/readme/README.md.tmpl (markers + {{DEVELOPMENT_BLOCK}})
  - src/cli/readme.rs (render_development_block + scaffold token replacement)
  - tests/steps/readme_enforcement_e2e_steps.rs (assertions + scripts fixture)

  ```
  EXIT=0 — 1 scenario (1 passed)
  ```

- [x] REFACTOR

  ```
  EXIT=0 — 1 scenario (1 passed)
  ```

- [x] COMMIT — `test(e2e): Consumer scaffolds a project and the README marks its unfilled sections`

  ```
  commit hash: a5eb61f
  ```

## Scenario 16: Consumer runs the README check on a complete README and it passes

- [x] RED/GREEN — regression guard for the check happy path (RED evidence is
      scenario 1's paired refusal; this proves exit 0 end-to-end):

  ```
  EXIT=0 — 1 scenario (1 passed); all steps reused
  ```

- [x] COMMIT — `test(e2e): Consumer runs the README check on a complete README and it passes`

  ```
  commit hash: 1846936
  ```

## Scenario 17: Consumer checks a review with a recorded README impact decision and it passes

- [x] RED/GREEN — regression guard for the review happy path (RED evidence is
      scenario 6's paired refusal):

  ```
  EXIT=0 — 1 scenario (1 passed); all steps reused
  ```

- [x] COMMIT — `test(e2e): Consumer checks a review with a recorded README impact decision and it passes`

  ```
  commit hash: 92ba8af
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
