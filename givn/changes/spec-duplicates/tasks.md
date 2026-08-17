# Tasks: spec-duplicates

Strict RED -> GREEN -> REFACTOR -> COMMIT execution order. There are 25
scenarios in the delta: 24 non-@e2e scenarios followed by one @e2e scenario.
All boxes below are intentionally unchecked. A scenario box may be checked only
after its runner evidence and commit hash have been recorded.

Every RED, GREEN, and REFACTOR phase below has a targeted command, a required
exit status, and a following blank code block for the exact runner output
evidence. Do not replace those evidence fields with predicted or invented
output.

## Interaction inventory verification

The interaction inventory was read from the top of
`givn/changes/spec-duplicates/specs/spec-duplicates/spec-duplicates.feature`:

`invoke givn spec duplicates and consume the complete advisory audit, including its JSON output variant`

Status: VERIFIED. The design's Interaction Coverage Matrix has exactly one
matching row. It maps the entry to the `@e2e` scenario
`Maintainer inspects the complete permanent scenario audit`, identifies the real
interface as the CLI/terminal, and names the real compiled-subprocess mechanism:
Cucumber `features_runner` launches the compiled `givn` subprocess with `spec
duplicates`, captures stdout and exit status, and validates the audit report.
No inventory entry is missing, and no extra @e2e scenario is to be created.

## Setup task

- [x] Setup: configure and prove the strict Gherkin runner before implementing a scenario.
  - Use the runner and infrastructure selected by `design.md`: the existing
    `cucumber-rs` `features_runner` binary, not a bare unit-test runner. It must
    execute `.feature` files under `givn/specs/` and the active change feature
    path under `givn/changes/spec-duplicates/specs/`.
  - Set `verify.command` to the exact `./run-tests.sh` invocation in the
    project's runner configuration. In this checkout the concrete owned key is
    `verify.command` in `givn/commands.yaml`; do not replace it with a unit-test
    command. Configure `verify.e2e_command` as the distinct exact
    `./run-tests.sh --e2e` command.
  - Confirm `tests/features_runner.rs` calls
    `GivnWorld::cucumber().fail_on_skipped()` before `run_and_exit`. Undefined
    or skipped steps must fail, never silently pass.
  - Create or extend the regular capability step module at
    `tests/steps/spec_duplicates_steps.rs`. Create or extend the separate E2E
    capability step module at `tests/steps/spec_duplicates_e2e_steps.rs`.
    Register both with `pub mod spec_duplicates_steps;` and
    `pub mod spec_duplicates_e2e_steps;` in `tests/steps/mod.rs`. Do not combine
    the two modules and do not add a parallel hand-maintained unit-test file.
  - Reuse the existing `GivnWorld` output, retrieval-environment, and snapshot
    state. Add only the state required by the design's fixture assertions.
  - Every new Rust step body must be non-empty. During RED use the explicit
    `unimplemented!()` stub. Never use `{}`, a bare `return`, or an empty setup
    function. Reused steps are allowed only when the reused body already does
    real setup or has a real assertion.
  - Prove strict mode before proceeding to the first scenario. Bind one step
    for the first scenario to `unimplemented!()` or leave it undefined, without
    removing any scenario @wip tag during this setup proof, then run the exact
    targeted command below.
    - Proof command:
      `GIVN_FEATURES=givn/changes/spec-duplicates/specs/spec-duplicates/spec-duplicates.feature GIVN_SPEC_EMBEDDER=stub cargo test --features spec-retrieval --test features_runner -- --name '^Exact duplicates in one feature are reported together$'`
    - Required proof exit status: non-zero. If it exits 0, stop, fix
      `.fail_on_skipped()`, and rerun the proof.
    - Strict-mode proof output evidence, paste the exact command output here:
     ```text
   Compiling givn v0.2.0 ($HOME/projects/givn)
       Finished `test` profile [unoptimized + debuginfo] target(s) in 3.73s
        Running tests/features_runner.rs (target/debug/deps/features_runner-c64f1ee623f3f628)
   Feature: Audit duplicate and related permanent scenarios
     Scenario: Exact duplicates in one feature are reported together
      ✘  Given a permanent feature contains two scenarios with identical behaviour
         Step failed:
         Defined: givn/changes/spec-duplicates/specs/spec-duplicates/spec-duplicates.feature:22:5
         Step doesn't match any function
   [Summary]
   1 feature
   1 scenario (1 failed)
   1 step (1 failed)

   thread 'main' panicked at cucumber.rs:1243:13:
   1 step failed
   error: test failed, to rerun `cargo test --test features_runner`
      ```
  - Confirm the score-control seam is test-only and matches `design.md`:
    `GIVN_SPEC_DUPLICATES_TEST_SCORES` is keyed by normalized unordered endpoint
    identities, is honored only with `GIVN_SPEC_EMBEDDER=stub`, rejects malformed
    keys or values, and cannot change production local-E5 scoring.

## Scenario 1: `Exact duplicates in one feature are reported together`

- [x] Scenario: `Exact duplicates in one feature are reported together`
  - RED:
    - Remove `@wip` only from this scenario's tag line. Do not remove or edit
      `@wip` on any other scenario.
    - Add the regular step bindings in
      `tests/steps/spec_duplicates_steps.rs`; every new binding is a non-empty
      `unimplemented!()` body in RED.
    - Target only this scenario with the design command adapted to its exact
      title:
      `GIVN_FEATURES=givn/changes/spec-duplicates/specs/spec-duplicates/spec-duplicates.feature GIVN_SPEC_EMBEDDER=stub cargo test --features spec-retrieval --test features_runner -- --name '^Exact duplicates in one feature are reported together$'`
    - Required result: non-zero exit caused by the explicit pending stub or an
      undefined step. Exit 0 is a strict-mode failure and blocks progress.
    - Runner output evidence, paste the exact non-zero output:
      ```text
   Compiling givn v0.2.0 ($HOME/projects/givn)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 3.16s
     Running tests/features_runner.rs (target/debug/deps/features_runner-c64f1ee623f3f628)
Feature: Audit duplicate and related permanent scenarios
  Scenario: Exact duplicates in one feature are reported together
   ✘  Given a permanent feature contains two scenarios with identical behaviour
      Step failed:
      Defined: givn/changes/spec-duplicates/specs/spec-duplicates/spec-duplicates.feature:22:5
      Matched: tests/steps/spec_duplicates_steps.rs:7:1
      Step panicked. Captured output: not implemented
[Summary]
1 feature
1 scenario (1 failed)
1 step (1 failed)

thread 'main' (3128883) panicked at $HOME/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cucumber-0.23.0/src/cucumber.rs:1243:13:
1 step failed
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
error: test failed, to rerun `cargo test --test features_runner`
      ```
  - GREEN:
    - Replace only this scenario's stubs with real fixture setup and assertions
      for one exact duplicate in one permanent feature, including count, rank,
      endpoint identities, `identical_behavior`, and null score.
    - Production files expected in GREEN: `src/main.rs`, `src/cli/spec.rs`,
      and `src/spec/retrieval.rs` for the new subcommand plumbing and minimum
      exact-duplicate audit path.
    - Test file expected in GREEN:
      `tests/steps/spec_duplicates_steps.rs`; no E2E step file changes.
    - Compile first and require exit 0:
      `cargo check --features spec-retrieval`
    - Compile output:
      ```text
    Checking givn v0.2.0 ($HOME/projects/givn)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.65s
      ```
    - Rerun the same targeted command above. Required result: exactly this
      scenario passes with exit 0.
    - Runner output evidence, paste the exact GREEN output:
      ```text
   Compiling givn v0.2.0 ($HOME/projects/givn)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 2.93s
     Running tests/features_runner.rs (target/debug/deps/features_runner-c64f1ee623f3f628)
Feature: Audit duplicate and related permanent scenarios
  Scenario: Exact duplicates in one feature are reported together
   ✔  Given a permanent feature contains two scenarios with identical behaviour
   ✔  When the user runs `givn spec duplicates`
   ✔  Then the audit reports candidate count 1
   ✔  And candidate rank 1 is classified as `exact_duplicate`
   ✔  And candidate rank 1 has left identity ("givn/specs/provider/provider.feature", "Provider is rejected")
   ✔  And candidate rank 1 has right identity ("givn/specs/provider/provider.feature", "Unknown provider is rejected")
   ✔  And candidate rank 1 has relation `identical_behavior`
   ✔  And candidate rank 1 has score `null`
[Summary]
1 feature
1 scenario (1 passed)
8 steps (8 passed)
      ```
    - Coverage: the targeted runner output contained no coverage summary; production coverage is unmeasured for this scenario.
  - REFACTOR:
    - Remove duplication in fixtures or assertions only where it does not
      change the output contract. Keep the canonical endpoint ordering and
      exact null serialization. Confirm no `unimplemented!()` remains on this
      scenario's path, run `cargo fmt --check`, and rerun the same targeted
      command.
    - Required result: exit 0 with the same scenario and assertions.
    - `cargo fmt -- --check` exited 0 with empty stdout and stderr.
    - Runner output evidence, paste the exact REFACTOR output:
      ```text
   Compiling givn v0.2.0 ($HOME/projects/givn)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 3.19s
     Running tests/features_runner.rs (target/debug/deps/features_runner-c64f1ee623f3f628)
Feature: Audit duplicate and related permanent scenarios
  Scenario: Exact duplicates in one feature are reported together
   ✔  Given a permanent feature contains two scenarios with identical behaviour
   ✔  When the user runs `givn spec duplicates`
   ✔  Then the audit reports candidate count 1
   ✔  And candidate rank 1 is classified as `exact_duplicate`
   ✔  And candidate rank 1 has left identity ("givn/specs/provider/provider.feature", "Provider is rejected")
   ✔  And candidate rank 1 has right identity ("givn/specs/provider/provider.feature", "Unknown provider is rejected")
   ✔  And candidate rank 1 has relation `identical_behavior`
   ✔  And candidate rank 1 has score `null`
[Summary]
1 feature
1 scenario (1 passed)
8 steps (8 passed)
      ```
  - COMMIT:
    - Create one atomic commit containing this scenario's RED, GREEN, and
      REFACTOR work. It must not be spec-only or stub-only.
    - Required commit message: `feat(spec-duplicates): Exact duplicates in one feature are reported together`
    - Files touched:
      - `givn/changes/spec-duplicates/specs/spec-duplicates/spec-duplicates.feature` (Scenario 1 tag only)
      - `src/main.rs`
      - `src/cli/spec.rs`
      - `src/spec/retrieval.rs`
      - `tests/steps/spec_duplicates_steps.rs`
    - Commit hash: `aef0ecac7fcbb1c77bf351bae916c0f5653b1cb5`

## Scenario 2: `Exact duplicates across features are reported together`

- [x] Scenario: `Exact duplicates across features are reported together`
  - RED:
    - Remove `@wip` only from this scenario. Leave every other `@wip` tag
      untouched.
    - Add any new regular bindings to `tests/steps/spec_duplicates_steps.rs`
      with explicit `unimplemented!()` bodies; reuse only real existing setup
      bindings.
    - Target only this scenario:
      `GIVN_FEATURES=givn/changes/spec-duplicates/specs/spec-duplicates/spec-duplicates.feature GIVN_SPEC_EMBEDDER=stub cargo test --features spec-retrieval --test features_runner -- --name '^Exact duplicates across features are reported together$'`
    - Required result: non-zero exit. Paste the exact runner output:
      ```text
   Compiling givn v0.2.0 ($HOME/projects/givn)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 2.96s
     Running tests/features_runner.rs (target/debug/deps/features_runner-c64f1ee623f3f628)
Feature: Audit duplicate and related permanent scenarios
  Scenario: Exact duplicates across features are reported together
   ✘  Given two permanent features contain scenarios with identical behaviour
      Step failed:
      Defined: givn/changes/spec-duplicates/specs/spec-duplicates/spec-duplicates.feature:33:5
      Matched: tests/steps/spec_duplicates_steps.rs:27:1
      Step panicked. Captured output: not implemented
[Summary]
1 feature
1 scenario (1 failed)
1 step (1 failed)

thread 'main' (3134496) panicked at $HOME/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cucumber-0.23.0/src/cucumber.rs:1243:13:
1 step failed
error: test failed, to rerun `cargo test --test features_runner`
      ```
  - GREEN:
    - Implement the minimum cross-feature fixture and assertions for one
      `exact_duplicate`, canonical left/right identities, `identical_behavior`,
      candidate count 1, and null score.
    - Production files expected in GREEN: `src/spec/retrieval.rs`; preserve the
      command plumbing from Scenario 1 in `src/main.rs` and `src/cli/spec.rs`.
    - Test file expected: `tests/steps/spec_duplicates_steps.rs`.
    - Compile first: `cargo check --features spec-retrieval` must exit 0.
    - Compile output:
      ```text
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.13s
      ```
    - Rerun the targeted command above. Required result: exit 0.
    - Runner output evidence, paste the exact GREEN output:
      ```text
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.14s
     Running tests/features_runner.rs (target/debug/deps/features_runner-c64f1ee623f3f628)
Feature: Audit duplicate and related permanent scenarios
  Scenario: Exact duplicates across features are reported together
   ✔  Given two permanent features contain scenarios with identical behaviour
   ✔  When the user runs `givn spec duplicates`
   ✔  Then the audit reports candidate count 1
   ✔  And candidate rank 1 is classified as `exact_duplicate`
   ✔  And candidate rank 1 has left identity ("givn/specs/auth/auth.feature", "Unknown provider is rejected")
   ✔  And candidate rank 1 has right identity ("givn/specs/provider/provider.feature", "Provider is rejected")
   ✔  And candidate rank 1 has relation `identical_behavior`
   ✔  And candidate rank 1 has score `null`
[Summary]
1 feature
1 scenario (1 passed)
8 steps (8 passed)
      ```
  - REFACTOR:
    - Consolidate cross-feature fixture helpers without changing canonical path
      ordering or exact output. Confirm no empty or stub step body, run
      `cargo fmt --check`, and rerun the targeted command.
    - Required result: exit 0; paste the exact runner output:
    - `cargo fmt -- --check` exited 0 with empty stdout and stderr.
      ```text
   Compiling givn v0.2.0 ($HOME/projects/givn)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 2.38s
     Running tests/features_runner.rs (target/debug/deps/features_runner-c64f1ee623f3f628)
Feature: Audit duplicate and related permanent scenarios
  Scenario: Exact duplicates across features are reported together
   ✔  Given two permanent features contain scenarios with identical behaviour
   ✔  When the user runs `givn spec duplicates`
   ✔  Then the audit reports candidate count 1
   ✔  And candidate rank 1 is classified as `exact_duplicate`
   ✔  And candidate rank 1 has left identity ("givn/specs/auth/auth.feature", "Unknown provider is rejected")
   ✔  And candidate rank 1 has right identity ("givn/specs/provider/provider.feature", "Provider is rejected")
   ✔  And candidate rank 1 has relation `identical_behavior`
   ✔  And candidate rank 1 has score `null`
[Summary]
1 feature
1 scenario (1 passed)
8 steps (8 passed)
      ```
  - COMMIT:
    - One atomic RED+GREEN+REFACTOR commit; it must include real production or
      step-glue work and reference the title verbatim.
    - Required commit message: `feat(spec-duplicates): Exact duplicates across features are reported together`
    - Files touched:
      - `givn/changes/spec-duplicates/proposal.md`
      - `givn/changes/spec-duplicates/design.md`
      - `givn/changes/spec-duplicates/design-review.md`
      - `givn/changes/spec-duplicates/arc42.md`
      - `givn/changes/spec-duplicates/tasks.md`
      - `docs/arc42/01-introduction-and-goals.md`
      - `docs/arc42/02-architecture-constraints.md`
      - `docs/arc42/03-context-and-scope.md`
      - `docs/arc42/04-solution-strategy.md`
      - `docs/arc42/05-building-block-view.md`
      - `docs/arc42/06-runtime-view.md`
      - `docs/arc42/08-crosscutting-concepts.md`
      - `docs/arc42/09-architecture-decisions.md`
      - `docs/arc42/10-quality-requirements.md`
      - `docs/arc42/11-risks-and-technical-debt.md`
      - `docs/arc42/12-glossary.md`
      - `docs/arc42/README.md`
      - `tests/steps/mod.rs`
      - `tests/steps/spec_duplicates_e2e_steps.rs`
      - `givn/changes/spec-duplicates/specs/spec-duplicates/spec-duplicates.feature`
      - `tests/steps/spec_duplicates_steps.rs`
    - Commit hash: `952e3c9c52dfe1ad24cd7c528afa86aa4d52f12e`

## Scenario 3: `Structurally overlapping scenarios retain their distinguishing facts`

- [x] Scenario: `Structurally overlapping scenarios retain their distinguishing facts`
  - RED:
    - Remove `@wip` only from this scenario and leave all other tags unchanged.
    - Add regular bindings in `tests/steps/spec_duplicates_steps.rs` with
      `unimplemented!()` for every new step body.
    - Target only this scenario:
      `GIVN_FEATURES=givn/changes/spec-duplicates/specs/spec-duplicates/spec-duplicates.feature GIVN_SPEC_EMBEDDER=stub cargo test --features spec-retrieval --test features_runner -- --name '^Structurally overlapping scenarios retain their distinguishing facts$'`
    - Required result: non-zero exit. Paste exact runner output:
      ```text
   Compiling givn v0.2.0 ($HOME/projects/givn)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 3.01s
     Running tests/features_runner.rs (target/debug/deps/features_runner-c64f1ee623f3f628)
Feature: Audit duplicate and related permanent scenarios
  Scenario: Structurally overlapping scenarios retain their distinguishing facts
   ✘  Given permanent scenarios share the same masked assertion shape but have different fingerprint facts
      Step failed:
      Defined: givn/changes/spec-duplicates/specs/spec-duplicates/spec-duplicates.feature:44:5
      Matched: tests/steps/spec_duplicates_steps.rs:14:1
      Step panicked. Captured output: not implemented
[Summary]
1 feature
1 scenario (1 failed)
1 step (1 failed)

thread 'main' (3140554) panicked at $HOME/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cucumber-0.23.0/src/cucumber.rs:1243:13:
1 step failed
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
error: test failed, to rerun pass `--test features_runner`
      ```
  - GREEN:
    - Implement masked assertion-shape matching with the existing `Fingerprint`
      facts, preserving all five distinguishing facts and contradiction status;
      do not add Given/When action-structure evidence.
    - Production files expected in GREEN: `src/spec/retrieval.rs`.
    - Test file expected: `tests/steps/spec_duplicates_steps.rs`.
    - Compile first: `cargo check --features spec-retrieval` must exit 0.
    - Rerun the targeted command. Required result: exit 0 and the exact facts,
      arrays, relation, and `fact_contradiction` assertion pass.
    - Compile output:
      ```text
    Checking givn v0.2.0 ($HOME/projects/givn)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.63s
      ```
    - Runner output evidence, paste exact GREEN output:
      ```text
   Compiling givn v0.2.0 ($HOME/projects/givn)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 5.53s
     Running tests/features_runner.rs (target/debug/deps/features_runner-c64f1ee623f3f628)
Feature: Audit duplicate and related permanent scenarios
  Scenario: Structurally overlapping scenarios retain their distinguishing facts
   ✔  Given permanent scenarios share the same masked assertion shape but have different fingerprint facts
   ✔  When the user runs `givn spec duplicates`
   ✔  Then candidate rank 1 is classified as `structural_overlap`
   ✔  And candidate rank 1 has relation `same_assertion_shape`
   ✔  And candidate rank 1 has left fingerprint facts `{"exit_code":"0","contains_polarity":{"0":"positive"},"tables":{"provider":"aws"},"done":false,"tty":false}`
   ✔  And candidate rank 1 has right fingerprint facts `{"exit_code":"2","contains_polarity":{"0":"negative"},"tables":{"provider":"gcp"},"done":true,"tty":true}`
   ✔  And candidate rank 1 has distinguishing facts `[{"field":"exit_code","left":"0","right":"2"},{"field":"contains_polarity[0]","left":"positive","right":"negative"},{"field":"tables[provider]","left":"aws","right":"gcp"},{"field":"done","left":false,"right":true},{"field":"tty","left":false,"right":true}]`
   ✔  And candidate rank 1 has fact contradiction `true`
   ✔  And candidate rank 1 has no Given/When action-structure evidence
[Summary]
1 feature
1 scenario (1 passed)
9 steps (9 passed)
      ```
    - Coverage: the targeted runner output contained no coverage summary; production coverage is unmeasured for this scenario.
  - REFACTOR:
    - Keep the structural evidence limited to `Fingerprint.assertions` and
      `Fingerprint.facts`; simplify fixture construction only. Run
      `cargo fmt --check`, confirm no stubs remain, and rerun the same command.
    - Required result: exit 0; paste exact REFACTOR output:
    - `cargo fmt -- --check` exited 0 with empty stdout and stderr.
      ```text

      ```
      ```text
   Compiling givn v0.2.0 ($HOME/projects/givn)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 3.98s
     Running tests/features_runner.rs (target/debug/deps/features_runner-c64f1ee623f3f628)
Feature: Audit duplicate and related permanent scenarios
  Scenario: Structurally overlapping scenarios retain their distinguishing facts
   ✔  Given permanent scenarios share the same masked assertion shape but have different fingerprint facts
   ✔  When the user runs `givn spec duplicates`
   ✔  Then candidate rank 1 is classified as `structural_overlap`
   ✔  And candidate rank 1 has relation `same_assertion_shape`
   ✔  And candidate rank 1 has left fingerprint facts `{"exit_code":"0","contains_polarity":{"0":"positive"},"tables":{"provider":"aws"},"done":false,"tty":false}`
   ✔  And candidate rank 1 has right fingerprint facts `{"exit_code":"2","contains_polarity":{"0":"negative"},"tables":{"provider":"gcp"},"done":true,"tty":true}`
   ✔  And candidate rank 1 has distinguishing facts `[{"field":"exit_code","left":"0","right":"2"},{"field":"contains_polarity[0]","left":"positive","right":"negative"},{"field":"tables[provider]","left":"aws","right":"gcp"},{"field":"done","left":false,"right":true},{"field":"tty","left":false,"right":true}]`
   ✔  And candidate rank 1 has fact contradiction `true`
   ✔  And candidate rank 1 has no Given/When action-structure evidence
[Summary]
1 feature
1 scenario (1 passed)
9 steps (9 passed)
      ```
  - COMMIT:
    - One atomic RED+GREEN+REFACTOR commit with a title-verbatim message.
    - Required commit message: `feat(spec-duplicates): Structurally overlapping scenarios retain their distinguishing facts`
    - Files touched:
      - `givn/changes/spec-duplicates/specs/spec-duplicates/spec-duplicates.feature`
      - `src/spec/retrieval.rs`
      - `tests/steps/spec_duplicates_steps.rs`
      - `givn/changes/spec-duplicates/tasks.md` (the pending Scenario 2 task hash line was preserved)
    - Commit hash: `8b9e11dca472c5c2b0b70ef5758ba0ff1712196d`

## Scenario 4: `A structural subset is reported as an overlap`

- [x] Scenario: `A structural subset is reported as an overlap`
  - RED:
    - Remove `@wip` only from this scenario; do not touch any other scenario's
      tags.
    - Add regular step bindings with explicit `unimplemented!()` bodies in
      `tests/steps/spec_duplicates_steps.rs`.
    - Target only this scenario:
      `GIVN_FEATURES=givn/changes/spec-duplicates/specs/spec-duplicates/spec-duplicates.feature GIVN_SPEC_EMBEDDER=stub cargo test --features spec-retrieval --test features_runner -- --name '^A structural subset is reported as an overlap$'`
    - Required result: non-zero exit. Paste exact runner output:
      ```text
   Compiling givn v0.2.0 ($HOME/projects/givn)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 3.05s
     Running tests/features_runner.rs (target/debug/deps/features_runner-c64f1ee623f3f628)
Feature: Audit duplicate and related permanent scenarios
  Scenario: A structural subset is reported as an overlap
   ✘  Given the permanent scenario "Short provider assertion" at "givn/specs/a/a.feature" has the assertion sequence of "Long provider assertion" at "givn/specs/b/b.feature"
      Step failed:
      Defined: givn/changes/spec-duplicates/specs/spec-duplicates/spec-duplicates.feature:56:5
      Matched: tests/steps/spec_duplicates_steps.rs:31:1
      Step panicked. Captured output: not implemented
[Summary]
1 feature
1 scenario (1 failed)
1 step (1 failed)

thread 'main' (3147398) panicked at $HOME/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cucumber-0.23.0/src/cucumber.rs:1243:13:
1 step failed
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
error: test failed, to rerun pass `--test features_runner`
      ```
  - GREEN:
    - Implement strict assertion-subsequence detection with the direction
      `left_assertions_subset_of_right`, masked assertion arrays, canonical
      identities, and null semantic score.
    - Production files expected in GREEN: `src/spec/retrieval.rs`.
    - Test file expected: `tests/steps/spec_duplicates_steps.rs`.
    - Compile first: `cargo check --features spec-retrieval` must exit 0.
    - Compile output:
      ```text
    Checking givn v0.2.0 ($HOME/projects/givn)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.94s
      ```
    - Rerun the targeted command. Required result: exit 0.
    - Runner output evidence, paste exact GREEN output:
      ```text
   Compiling givn v0.2.0 ($HOME/projects/givn)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.87s
     Running tests/features_runner.rs (target/debug/deps/features_runner-c64f1ee623f3f628)
Feature: Audit duplicate and related permanent scenarios
  Scenario: A structural subset is reported as an overlap
   ✔  Given the permanent scenario "Short provider assertion" at "givn/specs/a/a.feature" has the assertion sequence of "Long provider assertion" at "givn/specs/b/b.feature"
   ✔  When the user runs `givn spec duplicates`
   ✔  Then candidate rank 1 is classified as `structural_overlap`
   ✔  And candidate rank 1 has left identity ("givn/specs/a/a.feature", "Short provider assertion")
   ✔  And candidate rank 1 has right identity ("givn/specs/b/b.feature", "Long provider assertion")
   ✔  And candidate rank 1 has relation `left_assertions_subset_of_right`
   ✔  And candidate rank 1 has assertion shapes `{"left":["the command exits *"],"right":["the command exits *","the output contains *"]}`
   ✔  And candidate rank 1 has score `null`
[Summary]
1 feature
1 scenario (1 passed)
8 steps (8 passed)
      ```
    - Coverage: the targeted runner output contained no coverage summary; production coverage is unmeasured for this scenario.
  - REFACTOR:
    - Check both subset directions and precedence without changing the
      direction-specific relation. Run `cargo fmt --check`, inspect the diff
      for unchanged observable behavior, and rerun the targeted command.
    - Required result: exit 0; paste exact REFACTOR output:
      ```text
   Compiling givn v0.2.0 ($HOME/projects/givn)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 3.24s
     Running tests/features_runner.rs (target/debug/deps/features_runner-c64f1ee623f3f628)
Feature: Audit duplicate and related permanent scenarios
  Scenario: A structural subset is reported as an overlap
   ✔  Given the permanent scenario "Short provider assertion" at "givn/specs/a/a.feature" has the assertion sequence of "Long provider assertion" at "givn/specs/b/b.feature"
   ✔  When the user runs `givn spec duplicates`
   ✔  Then candidate rank 1 is classified as `structural_overlap`
   ✔  And candidate rank 1 has left identity ("givn/specs/a/a.feature", "Short provider assertion")
   ✔  And candidate rank 1 has right identity ("givn/specs/b/b.feature", "Long provider assertion")
   ✔  And candidate rank 1 has relation `left_assertions_subset_of_right`
   ✔  And candidate rank 1 has assertion shapes `{"left":["the command exits *"],"right":["the command exits *","the output contains *"]}`
   ✔  And candidate rank 1 has score `null`
[Summary]
1 feature
1 scenario (1 passed)
8 steps (8 passed)
      ```
    - `cargo fmt -- --check` exited 0 with empty stdout and stderr.
    - Grep of `tests/steps/spec_duplicates_steps.rs` found no `unimplemented!()`, `todo!()`, or empty step bodies.
  - COMMIT:
    - One atomic RED+GREEN+REFACTOR commit.
    - Required commit message: `feat(spec-duplicates): A structural subset is reported as an overlap`
    - Files touched:
      - `givn/changes/spec-duplicates/specs/spec-duplicates/spec-duplicates.feature` (Scenario 4 tag only)
      - `src/spec/retrieval.rs`
      - `tests/steps/spec_duplicates_steps.rs`
      - `givn/changes/spec-duplicates/tasks.md` (includes the prior Scenario 3 post-commit evidence and hash)
    - Commit hash: ``

## Scenario 5: `Semantically related scenarios are reported as candidates`

- [ ] Scenario: `Semantically related scenarios are reported as candidates`
  - RED:
    - Remove `@wip` only from this scenario. Leave all other @wip tags in
      place.
    - Add regular bindings in `tests/steps/spec_duplicates_steps.rs`; every new
      body uses `unimplemented!()` until GREEN.
    - Target only this scenario:
      `GIVN_FEATURES=givn/changes/spec-duplicates/specs/spec-duplicates/spec-duplicates.feature GIVN_SPEC_EMBEDDER=stub cargo test --features spec-retrieval --test features_runner -- --name '^Semantically related scenarios are reported as candidates$'`
    - Required result: non-zero exit. Paste exact runner output:
      ```text

      ```
  - GREEN:
    - Use the local E5 retrieval path for production and the documented
      stub-only score seam for the fixture. Report `semantic_candidate` at the
      configured score, with null structural fields, empty distinguishing facts,
      false contradiction, and advisory-only semantics.
    - Production files expected in GREEN: `src/spec/retrieval.rs`.
    - Test file expected: `tests/steps/spec_duplicates_steps.rs`.
    - Compile first: `cargo check --features spec-retrieval` must exit 0.
    - Rerun the targeted command. Required result: exit 0 and score `0.9600`.
    - Runner output evidence, paste exact GREEN output:
      ```text

      ```
  - REFACTOR:
    - Confirm the score seam cannot affect production local-E5 execution and
      that semantic evidence never suppresses deterministic evidence. Run
      `cargo fmt --check`, rerun the targeted command, and verify no stub remains.
    - Required result: exit 0; paste exact REFACTOR output:
      ```text

      ```
  - COMMIT:
    - One atomic RED+GREEN+REFACTOR commit.
    - Required commit message: `feat(spec-duplicates): Semantically related scenarios are reported as candidates`
    - Commit hash: ``

## Scenario 6: `The audit reports candidates beyond the interactive top five`

- [ ] Scenario: `The audit reports candidates beyond the interactive top five`
  - RED:
    - Remove `@wip` only from this scenario and leave all other tags unchanged.
    - Add any new regular bindings with non-empty `unimplemented!()` bodies in
      `tests/steps/spec_duplicates_steps.rs`.
    - Target only this scenario:
      `GIVN_FEATURES=givn/changes/spec-duplicates/specs/spec-duplicates/spec-duplicates.feature GIVN_SPEC_EMBEDDER=stub cargo test --features spec-retrieval --test features_runner -- --name '^The audit reports candidates beyond the interactive top five$'`
    - Required result: non-zero exit. Paste exact runner output:
      ```text

      ```
  - GREEN:
    - Remove any top-five truncation from the complete report and assert count
      6, rank 6, contiguous ranks, and highest rank equal to candidate count.
    - Production files expected in GREEN: `src/spec/retrieval.rs`.
    - Test file expected: `tests/steps/spec_duplicates_steps.rs`.
    - Compile first: `cargo check --features spec-retrieval` must exit 0.
    - Rerun the targeted command. Required result: exit 0.
    - Runner output evidence, paste exact GREEN output:
      ```text

      ```
  - REFACTOR:
    - Check complete candidate collection and rank assignment remain separate
      from rendering and do not reintroduce a view limit. Run `cargo fmt --check`
      and rerun the targeted command.
    - Required result: exit 0; paste exact REFACTOR output:
      ```text

      ```
  - COMMIT:
    - One atomic RED+GREEN+REFACTOR commit.
    - Required commit message: `feat(spec-duplicates): The audit reports candidates beyond the interactive top five`
    - Commit hash: ``

## Scenario 7: `Repeating an unchanged audit produces the same report`

- [ ] Scenario: `Repeating an unchanged audit produces the same report`
  - RED:
    - Remove `@wip` only from this scenario. Do not remove @wip elsewhere.
    - Add non-empty regular step stubs using `unimplemented!()` in
      `tests/steps/spec_duplicates_steps.rs`.
    - Target only this scenario:
      `GIVN_FEATURES=givn/changes/spec-duplicates/specs/spec-duplicates/spec-duplicates.feature GIVN_SPEC_EMBEDDER=stub cargo test --features spec-retrieval --test features_runner -- --name '^Repeating an unchanged audit produces the same report$'`
    - Required result: non-zero exit. Paste exact runner output:
      ```text

      ```
  - GREEN:
    - Implement deterministic collection, classification, sorting, rank
      assignment, and text/JSON serialization so two unchanged invocations are
      byte-for-byte identical with identical counts and evidence.
    - Production files expected in GREEN: `src/spec/retrieval.rs`.
    - Test file expected: `tests/steps/spec_duplicates_steps.rs`.
    - Compile first: `cargo check --features spec-retrieval` must exit 0.
    - Rerun the targeted command. Required result: exit 0.
    - Runner output evidence, paste exact GREEN output:
      ```text

      ```
  - REFACTOR:
    - Review all ordering keys, total floating-point ordering, endpoint tie
      breaks, LF endings, and final newline. Run `cargo fmt --check` and rerun
      the same command.
    - Required result: exit 0; paste exact REFACTOR output:
      ```text

      ```
  - COMMIT:
    - One atomic RED+GREEN+REFACTOR commit.
    - Required commit message: `feat(spec-duplicates): Repeating an unchanged audit produces the same report`
    - Commit hash: ``

## Scenario 8: `JSON output identifies every audit candidate`

- [ ] Scenario: `JSON output identifies every audit candidate`
  - RED:
    - Remove `@wip` only from this scenario; preserve every other scenario tag.
    - Add regular bindings with explicit `unimplemented!()` stubs in
      `tests/steps/spec_duplicates_steps.rs`.
    - Target only this scenario:
      `GIVN_FEATURES=givn/changes/spec-duplicates/specs/spec-duplicates/spec-duplicates.feature GIVN_SPEC_EMBEDDER=stub cargo test --features spec-retrieval --test features_runner -- --name '^JSON output identifies every audit candidate$'`
    - Required result: non-zero exit. Paste exact runner output:
      ```text

      ```
  - GREEN:
    - Implement the exact JSON schema, field order, endpoint key order,
      evidence key order, classifications, relations, deterministic ordering,
      contiguous ranks, and eligible non-removed scanned count.
    - Production files expected in GREEN: `src/spec/retrieval.rs` and, only if
      required by the existing format dispatch, `src/cli/spec.rs`.
    - Test file expected: `tests/steps/spec_duplicates_steps.rs`.
    - Compile first: `cargo check --features spec-retrieval` must exit 0.
    - Rerun the targeted command. Required result: exit 0.
    - Runner output evidence, paste exact GREEN output:
      ```text

      ```
  - REFACTOR:
    - Verify text and JSON share one sorted candidate model and that no map or
      serializer change makes key order unstable. Run `cargo fmt --check` and
      rerun the targeted command.
    - Required result: exit 0; paste exact REFACTOR output:
      ```text

      ```
  - COMMIT:
    - One atomic RED+GREEN+REFACTOR commit.
    - Required commit message: `feat(spec-duplicates): JSON output identifies every audit candidate`
    - Commit hash: ``

## Scenario 9: `Auditing leaves permanent specifications unchanged`

- [ ] Scenario: `Auditing leaves permanent specifications unchanged`
  - RED:
    - Remove `@wip` only from this scenario and leave all other @wip tags.
    - Add regular snapshot/assertion bindings with `unimplemented!()` bodies in
      `tests/steps/spec_duplicates_steps.rs`.
    - Target only this scenario:
      `GIVN_FEATURES=givn/changes/spec-duplicates/specs/spec-duplicates/spec-duplicates.feature GIVN_SPEC_EMBEDDER=stub cargo test --features spec-retrieval --test features_runner -- --name '^Auditing leaves permanent specifications unchanged$'`
    - Required result: non-zero exit. Paste exact runner output:
      ```text

      ```
  - GREEN:
    - Read permanent feature bytes for the fixture, run the real subprocess,
      and assert every permanent specification is byte-for-byte unchanged.
      The audit must not write or rewrite `givn/specs/`.
    - Production files expected in GREEN: `src/spec/retrieval.rs`.
    - Test file expected: `tests/steps/spec_duplicates_steps.rs`.
    - Compile first: `cargo check --features spec-retrieval` must exit 0.
    - Rerun the targeted command. Required result: exit 0.
    - Runner output evidence, paste exact GREEN output:
      ```text

      ```
  - REFACTOR:
    - Keep the read-only boundary explicit and keep filesystem snapshots as
      secondary evidence to the CLI result. Run `cargo fmt --check` and rerun
      the targeted command.
    - Required result: exit 0; paste exact REFACTOR output:
      ```text

      ```
  - COMMIT:
    - One atomic RED+GREEN+REFACTOR commit.
    - Required commit message: `feat(spec-duplicates): Auditing leaves permanent specifications unchanged`
    - Commit hash: ``

## Scenario 10: `Auditing leaves the persistent retrieval index unchanged`

- [ ] Scenario: `Auditing leaves the persistent retrieval index unchanged`
  - RED:
    - Remove `@wip` only from this scenario. Leave every other tag untouched.
    - Add non-empty regular step definitions with `unimplemented!()` in
      `tests/steps/spec_duplicates_steps.rs`.
    - Target only this scenario:
      `GIVN_FEATURES=givn/changes/spec-duplicates/specs/spec-duplicates/spec-duplicates.feature GIVN_SPEC_EMBEDDER=stub cargo test --features spec-retrieval --test features_runner -- --name '^Auditing leaves the persistent retrieval index unchanged$'`
    - Required result: non-zero exit. Paste exact runner output:
      ```text

      ```
  - GREEN:
    - Build and snapshot the persistent retrieval index, run the audit without
      `refresh_index`, and assert the index bytes are unchanged.
    - Production files expected in GREEN: `src/spec/retrieval.rs`.
    - Test file expected: `tests/steps/spec_duplicates_steps.rs`.
    - Compile first: `cargo check --features spec-retrieval` must exit 0.
    - Rerun the targeted command. Required result: exit 0.
    - Runner output evidence, paste exact GREEN output:
      ```text

      ```
  - REFACTOR:
    - Confirm the audit never opens the persistent index as evidence and does
      not refresh it. Run `cargo fmt --check` and rerun the targeted command.
    - Required result: exit 0; paste exact REFACTOR output:
      ```text

      ```
  - COMMIT:
    - One atomic RED+GREEN+REFACTOR commit.
    - Required commit message: `feat(spec-duplicates): Auditing leaves the persistent retrieval index unchanged`
    - Commit hash: ``

## Scenario 11: `Active change scenarios are not part of the permanent audit`

- [ ] Scenario: `Active change scenarios are not part of the permanent audit`
  - RED:
    - Remove `@wip` only from this scenario; do not touch any other @wip tag.
    - Add regular fixture and assertion bindings with `unimplemented!()` in
      `tests/steps/spec_duplicates_steps.rs`.
    - Target only this scenario:
      `GIVN_FEATURES=givn/changes/spec-duplicates/specs/spec-duplicates/spec-duplicates.feature GIVN_SPEC_EMBEDDER=stub cargo test --features spec-retrieval --test features_runner -- --name '^Active change scenarios are not part of the permanent audit$'`
    - Required result: non-zero exit. Paste exact runner output:
      ```text

      ```
  - GREEN:
    - Restrict the audit aggregate and endpoint identities to eligible permanent
      non-removed scenarios under `givn/specs/`; assert active identities are
      absent and every endpoint file starts with `givn/specs/`.
    - Production files expected in GREEN: `src/spec/retrieval.rs`.
    - Test file expected: `tests/steps/spec_duplicates_steps.rs`.
    - Compile first: `cargo check --features spec-retrieval` must exit 0.
    - Rerun the targeted command. Required result: exit 0.
    - Runner output evidence, paste exact GREEN output:
      ```text

      ```
  - REFACTOR:
    - Keep active-change and archive path filtering at collection boundaries,
      not as a renderer-only suppression. Run `cargo fmt --check` and rerun the
      targeted command.
    - Required result: exit 0; paste exact REFACTOR output:
      ```text

      ```
  - COMMIT:
    - One atomic RED+GREEN+REFACTOR commit.
    - Required commit message: `feat(spec-duplicates): Active change scenarios are not part of the permanent audit`
    - Commit hash: ``

## Scenario 12: `An audit with no candidates reports an empty result`

- [ ] Scenario: `An audit with no candidates reports an empty result`
  - RED:
    - Remove `@wip` only from this scenario and leave all other tags unchanged.
    - Add regular bindings with explicit `unimplemented!()` stubs in
      `tests/steps/spec_duplicates_steps.rs`.
    - Target only this scenario:
      `GIVN_FEATURES=givn/changes/spec-duplicates/specs/spec-duplicates/spec-duplicates.feature GIVN_SPEC_EMBEDDER=stub cargo test --features spec-retrieval --test features_runner -- --name '^An audit with no candidates reports an empty result$'`
    - Required result: non-zero exit. Paste exact runner output:
      ```text

      ```
  - GREEN:
    - Render the exact zero-candidate text report and matching JSON result for
      two eligible scenarios, with zero candidates and an empty array.
    - Production files expected in GREEN: `src/spec/retrieval.rs`.
    - Test file expected: `tests/steps/spec_duplicates_steps.rs`.
    - Compile first: `cargo check --features spec-retrieval` must exit 0.
    - Rerun the targeted command. Required result: exit 0.
    - Runner output evidence, paste exact GREEN output:
      ```text

      ```
  - REFACTOR:
    - Keep empty and missing-corpus rendering on the same deterministic report
      path; verify exactly one final newline and no diagnostic prefix/suffix.
      Run `cargo fmt --check` and rerun the targeted command.
    - Required result: exit 0; paste exact REFACTOR output:
      ```text

      ```
  - COMMIT:
    - One atomic RED+GREEN+REFACTOR commit.
    - Required commit message: `feat(spec-duplicates): An audit with no candidates reports an empty result`
    - Commit hash: ``

## Scenario 13: `The audit reports retrieval unavailable without the retrieval feature`

- [ ] Scenario: `The audit reports retrieval unavailable without the retrieval feature`
  - RED:
    - Remove `@wip` only from this scenario. Do not edit any other scenario's
      tags.
    - Add regular step bindings with explicit `unimplemented!()` bodies in
      `tests/steps/spec_duplicates_steps.rs`. The binding may invoke a
      no-feature compiled subprocess, but its own RED body is not empty.
    - Target only this scenario with the feature-enabled runner, whose step
      invokes the no-retrieval production build:
      `GIVN_FEATURES=givn/changes/spec-duplicates/specs/spec-duplicates/spec-duplicates.feature GIVN_SPEC_EMBEDDER=stub cargo test --features spec-retrieval --test features_runner -- --name '^The audit reports retrieval unavailable without the retrieval feature$'`
    - Required result: non-zero exit. Paste exact runner output:
      ```text

      ```
  - GREEN:
    - Make the no-`spec-retrieval` production build expose the existing explicit
      retrieval-unavailable error and non-zero exit, with stderr containing
      `spec retrieval is unavailable`.
    - Production files expected in GREEN: `src/main.rs` and `src/cli/spec.rs`
      for feature-gated dispatch/error behavior; do not add retrieval-only
      dependencies to the default build.
    - Test file expected: `tests/steps/spec_duplicates_steps.rs`.
    - Compile first, both must exit 0:
      `cargo check --no-default-features --locked`
      and `cargo check --features spec-retrieval`.
    - Rerun the targeted command. Required result: exit 0 for the feature-runner
      scenario while the scenario's inner no-feature subprocess exits non-zero
      with the required message.
    - Runner output evidence, paste exact GREEN output:
      ```text

      ```
  - REFACTOR:
    - Confirm the no-feature error remains explicit and is not silently masked
      by the feature-enabled test runner. Run `cargo fmt --check`, rerun the
      targeted command, and verify no stub remains.
    - Required result: exit 0; paste exact REFACTOR output:
      ```text

      ```
  - COMMIT:
    - One atomic RED+GREEN+REFACTOR commit.
    - Required commit message: `feat(spec-duplicates): The audit reports retrieval unavailable without the retrieval feature`
    - Commit hash: ``

## Scenario 14: `The audit reports an unavailable retrieval model`

- [ ] Scenario: `The audit reports an unavailable retrieval model`
  - RED:
    - Remove `@wip` only from this scenario and leave all other tags unchanged.
    - Add regular bindings with non-empty `unimplemented!()` bodies in
      `tests/steps/spec_duplicates_steps.rs`.
    - Target only this scenario:
      `GIVN_FEATURES=givn/changes/spec-duplicates/specs/spec-duplicates/spec-duplicates.feature GIVN_SPEC_EMBEDDER=stub cargo test --features spec-retrieval --test features_runner -- --name '^The audit reports an unavailable retrieval model$'`
    - Required result: non-zero exit. Paste exact runner output:
      ```text

      ```
  - GREEN:
    - Exercise a real subprocess with the retrieval model unavailable, assert a
      non-zero exit and stderr containing `retrieval model is unavailable`, and
      keep the deterministic fixture override separate from production model
      availability.
    - Production files expected in GREEN: `src/spec/retrieval.rs`.
    - Test file expected: `tests/steps/spec_duplicates_steps.rs`.
    - Compile first: `cargo check --features spec-retrieval` must exit 0.
    - Rerun the targeted command. Required result: exit 0 for the scenario.
    - Runner output evidence, paste exact GREEN output:
      ```text

      ```
  - REFACTOR:
    - Check error context identifies model availability without swallowing the
      original failure and that the stub path remains deterministic. Run
      `cargo fmt --check` and rerun the targeted command.
    - Required result: exit 0; paste exact REFACTOR output:
      ```text

      ```
  - COMMIT:
    - One atomic RED+GREEN+REFACTOR commit.
    - Required commit message: `feat(spec-duplicates): The audit reports an unavailable retrieval model`
    - Commit hash: ``

## Scenario 15: `A malformed permanent feature reports its path`

- [ ] Scenario: `A malformed permanent feature reports its path`
  - RED:
    - Remove `@wip` only from this scenario. Leave all other @wip tags intact.
    - Add regular bindings with explicit `unimplemented!()` in
      `tests/steps/spec_duplicates_steps.rs`.
    - Target only this scenario:
      `GIVN_FEATURES=givn/changes/spec-duplicates/specs/spec-duplicates/spec-duplicates.feature GIVN_SPEC_EMBEDDER=stub cargo test --features spec-retrieval --test features_runner -- --name '^A malformed permanent feature reports its path$'`
    - Required result: non-zero exit. Paste exact runner output:
      ```text

      ```
  - GREEN:
    - Abort on malformed permanent Gherkin, include the project-relative path
      `givn/specs/broken/broken.feature` in stderr, emit no report on stdout,
      and preserve the no-partial-report rule.
    - Production files expected in GREEN: `src/spec/retrieval.rs`.
    - Test file expected: `tests/steps/spec_duplicates_steps.rs`.
    - Compile first: `cargo check --features spec-retrieval` must exit 0.
    - Rerun the targeted command. Required result: exit 0 for the scenario's
      assertions while the inner `givn` command exits non-zero.
    - Runner output evidence, paste exact GREEN output:
      ```text

      ```
  - REFACTOR:
    - Check path context survives all parse-error branches and stdout remains
      empty. Run `cargo fmt --check` and rerun the targeted command.
    - Required result: exit 0; paste exact REFACTOR output:
      ```text

      ```
  - COMMIT:
    - One atomic RED+GREEN+REFACTOR commit.
    - Required commit message: `feat(spec-duplicates): A malformed permanent feature reports its path`
    - Commit hash: ``

## Scenario 16: `An unreadable permanent feature reports its path`

- [ ] Scenario: `An unreadable permanent feature reports its path`
  - RED:
    - Remove `@wip` only from this scenario; do not alter tags elsewhere.
    - Add regular step definitions with `unimplemented!()` bodies in
      `tests/steps/spec_duplicates_steps.rs`.
    - Target only this scenario:
      `GIVN_FEATURES=givn/changes/spec-duplicates/specs/spec-duplicates/spec-duplicates.feature GIVN_SPEC_EMBEDDER=stub cargo test --features spec-retrieval --test features_runner -- --name '^An unreadable permanent feature reports its path$'`
    - Required result: non-zero exit. Paste exact runner output:
      ```text

      ```
  - GREEN:
    - Abort on an unreadable permanent file, include the project-relative path
      `givn/specs/private/private.feature` in stderr, and emit no stdout report.
      The fixture must produce a real read failure for the subprocess.
    - Production files expected in GREEN: `src/spec/retrieval.rs`.
    - Test file expected: `tests/steps/spec_duplicates_steps.rs`.
    - Compile first: `cargo check --features spec-retrieval` must exit 0.
    - Rerun the targeted command. Required result: exit 0 for the scenario while
      the inner command exits non-zero.
    - Runner output evidence, paste exact GREEN output:
      ```text

      ```
  - REFACTOR:
    - Check permissions/read errors are path-specific and not silently dropped;
      do not weaken the fixture to a parse error. Run `cargo fmt --check` and
      rerun the targeted command.
    - Required result: exit 0; paste exact REFACTOR output:
      ```text

      ```
  - COMMIT:
    - One atomic RED+GREEN+REFACTOR commit.
    - Required commit message: `feat(spec-duplicates): An unreadable permanent feature reports its path`
    - Commit hash: ``

## Scenario 17: `A missing permanent specifications directory is an empty corpus`

- [ ] Scenario: `A missing permanent specifications directory is an empty corpus`
  - RED:
    - Remove `@wip` only from this scenario and leave all other @wip tags.
    - Add regular step bindings with explicit `unimplemented!()` bodies in
      `tests/steps/spec_duplicates_steps.rs`.
    - Target only this scenario:
      `GIVN_FEATURES=givn/changes/spec-duplicates/specs/spec-duplicates/spec-duplicates.feature GIVN_SPEC_EMBEDDER=stub cargo test --features spec-retrieval --test features_runner -- --name '^A missing permanent specifications directory is an empty corpus$'`
    - Required result: non-zero exit. Paste exact runner output:
      ```text

      ```
  - GREEN:
    - Treat a missing `givn/specs/` directory as a successful empty corpus,
      render zero counts and an empty candidates array, and leave the persistent
      index absent.
    - Production files expected in GREEN: `src/spec/retrieval.rs`.
    - Test file expected: `tests/steps/spec_duplicates_steps.rs`.
    - Compile first: `cargo check --features spec-retrieval` must exit 0.
    - Rerun the targeted command. Required result: exit 0.
    - Runner output evidence, paste exact GREEN output:
      ```text

      ```
  - REFACTOR:
    - Keep missing-directory handling distinct from malformed-file errors and
      ensure no index is created as a retrieval side effect. Run
      `cargo fmt --check` and rerun the targeted command.
    - Required result: exit 0; paste exact REFACTOR output:
      ```text

      ```
  - COMMIT:
    - One atomic RED+GREEN+REFACTOR commit.
    - Required commit message: `feat(spec-duplicates): A missing permanent specifications directory is an empty corpus`
    - Commit hash: ``

## Scenario 18: `Archived scenarios are excluded from the audit`

- [ ] Scenario: `Archived scenarios are excluded from the audit`
  - RED:
    - Remove `@wip` only from this scenario; leave all other tags unchanged.
    - Add regular bindings with explicit `unimplemented!()` bodies in
      `tests/steps/spec_duplicates_steps.rs`.
    - Target only this scenario:
      `GIVN_FEATURES=givn/changes/spec-duplicates/specs/spec-duplicates/spec-duplicates.feature GIVN_SPEC_EMBEDDER=stub cargo test --features spec-retrieval --test features_runner -- --name '^Archived scenarios are excluded from the audit$'`
    - Required result: non-zero exit. Paste exact runner output:
      ```text

      ```
  - GREEN:
    - Collect only permanent files under `givn/specs/`; assert archive endpoint
      prefixes, absence of `givn/archive/old-change/`, and unchanged candidate
      count when the matching archived scenario is present.
    - Production files expected in GREEN: `src/spec/retrieval.rs`.
    - Test file expected: `tests/steps/spec_duplicates_steps.rs`.
    - Compile first: `cargo check --features spec-retrieval` must exit 0.
    - Rerun the targeted command. Required result: exit 0.
    - Runner output evidence, paste exact GREEN output:
      ```text

      ```
  - REFACTOR:
    - Check archive exclusion is enforced by input collection and not by
      deleting or mutating archive fixtures. Run `cargo fmt --check` and rerun
      the targeted command.
    - Required result: exit 0; paste exact REFACTOR output:
      ```text

      ```
  - COMMIT:
    - One atomic RED+GREEN+REFACTOR commit.
    - Required commit message: `feat(spec-duplicates): Archived scenarios are excluded from the audit`
    - Commit hash: ``

## Scenario 19: `Auditing leaves active changes and archive contents unchanged`

- [ ] Scenario: `Auditing leaves active changes and archive contents unchanged`
  - RED:
    - Remove `@wip` only from this scenario. Do not modify other scenario tags.
    - Add regular snapshot bindings with non-empty `unimplemented!()` bodies in
      `tests/steps/spec_duplicates_steps.rs`.
    - Target only this scenario:
      `GIVN_FEATURES=givn/changes/spec-duplicates/specs/spec-duplicates/spec-duplicates.feature GIVN_SPEC_EMBEDDER=stub cargo test --features spec-retrieval --test features_runner -- --name '^Auditing leaves active changes and archive contents unchanged$'`
    - Required result: non-zero exit. Paste exact runner output:
      ```text

      ```
  - GREEN:
    - Snapshot active-change files and archive contents before the real
      subprocess, run the permanent-only audit, and assert both trees are
      byte-for-byte unchanged.
    - Production files expected in GREEN: `src/spec/retrieval.rs`.
    - Test file expected: `tests/steps/spec_duplicates_steps.rs`.
    - Compile first: `cargo check --features spec-retrieval` must exit 0.
    - Rerun the targeted command. Required result: exit 0.
    - Runner output evidence, paste exact GREEN output:
      ```text

      ```
  - REFACTOR:
    - Keep non-mutation assertions independent for active and archive trees and
      preserve the permitted model-cache side effect boundary. Run
      `cargo fmt --check` and rerun the targeted command.
    - Required result: exit 0; paste exact REFACTOR output:
      ```text

      ```
  - COMMIT:
    - One atomic RED+GREEN+REFACTOR commit.
    - Required commit message: `feat(spec-duplicates): Auditing leaves active changes and archive contents unchanged`
    - Commit hash: ``

## Scenario 20: `An absent persistent index is ignored and remains absent`

- [ ] Scenario: `An absent persistent index is ignored and remains absent`
  - RED:
    - Remove `@wip` only from this scenario and leave all other @wip tags.
    - Add regular bindings with explicit `unimplemented!()` in
      `tests/steps/spec_duplicates_steps.rs`.
    - Target only this scenario:
      `GIVN_FEATURES=givn/changes/spec-duplicates/specs/spec-duplicates/spec-duplicates.feature GIVN_SPEC_EMBEDDER=stub cargo test --features spec-retrieval --test features_runner -- --name '^An absent persistent index is ignored and remains absent$'`
    - Required result: non-zero exit. Paste exact runner output:
      ```text

      ```
  - GREEN:
    - Run the audit from current permanent files with transient embeddings,
      assert a successful JSON report, and assert `.spec-index.json` is still
      absent.
    - Production files expected in GREEN: `src/spec/retrieval.rs`.
    - Test file expected: `tests/steps/spec_duplicates_steps.rs`.
    - Compile first: `cargo check --features spec-retrieval` must exit 0.
    - Rerun the targeted command. Required result: exit 0.
    - Runner output evidence, paste exact GREEN output:
      ```text

      ```
  - REFACTOR:
    - Verify the audit path does not call the persistent index refresh path and
      does not create an index accidentally. Run `cargo fmt --check` and rerun
      the targeted command.
    - Required result: exit 0; paste exact REFACTOR output:
      ```text

      ```
  - COMMIT:
    - One atomic RED+GREEN+REFACTOR commit.
    - Required commit message: `feat(spec-duplicates): An absent persistent index is ignored and remains absent`
    - Commit hash: ``

## Scenario 21: `A malformed persistent index is ignored and unchanged`

- [ ] Scenario: `A malformed persistent index is ignored and unchanged`
  - RED:
    - Remove `@wip` only from this scenario. Leave all other scenario tags.
    - Add regular step definitions with explicit `unimplemented!()` bodies in
      `tests/steps/spec_duplicates_steps.rs`.
    - Target only this scenario:
      `GIVN_FEATURES=givn/changes/spec-duplicates/specs/spec-duplicates/spec-duplicates.feature GIVN_SPEC_EMBEDDER=stub cargo test --features spec-retrieval --test features_runner -- --name '^A malformed persistent index is ignored and unchanged$'`
    - Required result: non-zero exit. Paste exact runner output:
      ```text

      ```
  - GREEN:
    - Write `not-json` to the persistent index, run the audit using current
      permanent files and transient embeddings, assert valid JSON candidates,
      and assert the index remains exactly `not-json`.
    - Production files expected in GREEN: `src/spec/retrieval.rs`.
    - Test file expected: `tests/steps/spec_duplicates_steps.rs`.
    - Compile first: `cargo check --features spec-retrieval` must exit 0.
    - Rerun the targeted command. Required result: exit 0.
    - Runner output evidence, paste exact GREEN output:
      ```text

      ```
  - REFACTOR:
    - Keep malformed index bytes outside the audit evidence path and avoid
      repairing or rewriting them. Run `cargo fmt --check` and rerun the
      targeted command.
    - Required result: exit 0; paste exact REFACTOR output:
      ```text

      ```
  - COMMIT:
    - One atomic RED+GREEN+REFACTOR commit.
    - Required commit message: `feat(spec-duplicates): A malformed persistent index is ignored and unchanged`
    - Commit hash: ``

## Scenario 22: `A semantic candidate at the inclusive threshold is reported`

- [ ] Scenario: `A semantic candidate at the inclusive threshold is reported`
  - RED:
    - Remove `@wip` only from this scenario and leave all other tags intact.
    - Add regular bindings with explicit `unimplemented!()` bodies in
      `tests/steps/spec_duplicates_steps.rs`.
    - Target only this scenario:
      `GIVN_FEATURES=givn/changes/spec-duplicates/specs/spec-duplicates/spec-duplicates.feature GIVN_SPEC_EMBEDDER=stub cargo test --features spec-retrieval --test features_runner -- --name '^A semantic candidate at the inclusive threshold is reported$'`
    - Required result: non-zero exit. Paste exact runner output:
      ```text

      ```
  - GREEN:
    - Set `GIVN_SPEC_DUPLICATES_TEST_SCORES` for the normalized endpoint pair to
      exactly `0.9500`, use the stub embedder only, and assert the semantic
      candidate is admitted with `semantic_similarity` and serialized score
      `0.9500`.
    - Production files expected in GREEN: `src/spec/retrieval.rs`.
    - Test file expected: `tests/steps/spec_duplicates_steps.rs`.
    - Compile first: `cargo check --features spec-retrieval` must exit 0.
    - Rerun the targeted command. Required result: exit 0.
    - Runner output evidence, paste exact GREEN output:
      ```text

      ```
  - REFACTOR:
    - Confirm thresholding uses the raw score before four-decimal
      serialization, remains inclusive at `0.9500`, and the score seam is not
      available to production local-E5 execution. Run `cargo fmt --check` and
      rerun the targeted command.
    - Required result: exit 0; paste exact REFACTOR output:
      ```text

      ```
  - COMMIT:
    - One atomic RED+GREEN+REFACTOR commit.
    - Required commit message: `feat(spec-duplicates): A semantic candidate at the inclusive threshold is reported`
    - Commit hash: ``

## Scenario 23: `A semantic pair below the threshold is not reported`

- [ ] Scenario: `A semantic pair below the threshold is not reported`
  - RED:
    - Remove `@wip` only from this scenario. Do not touch any other scenario's
      @wip tag.
    - Add regular step bindings with non-empty `unimplemented!()` bodies in
      `tests/steps/spec_duplicates_steps.rs`.
    - Target only this scenario:
      `GIVN_FEATURES=givn/changes/spec-duplicates/specs/spec-duplicates/spec-duplicates.feature GIVN_SPEC_EMBEDDER=stub cargo test --features spec-retrieval --test features_runner -- --name '^A semantic pair below the threshold is not reported$'`
    - Required result: non-zero exit. Paste exact runner output:
      ```text

      ```
  - GREEN:
    - Set the normalized endpoint pair score to exactly `0.9499` through the
      stub-only seam and exclude it before serialization; assert candidate count
      zero and no score `0.9499`.
    - Production files expected in GREEN: `src/spec/retrieval.rs`.
    - Test file expected: `tests/steps/spec_duplicates_steps.rs`.
    - Compile first: `cargo check --features spec-retrieval` must exit 0.
    - Rerun the targeted command. Required result: exit 0.
    - Runner output evidence, paste exact GREEN output:
      ```text

      ```
  - REFACTOR:
    - Check admission happens before four-decimal formatting and that excluded
      pairs do not receive ranks. Run `cargo fmt --check` and rerun the targeted
      command.
    - Required result: exit 0; paste exact REFACTOR output:
      ```text

      ```
  - COMMIT:
    - One atomic RED+GREEN+REFACTOR commit.
    - Required commit message: `feat(spec-duplicates): A semantic pair below the threshold is not reported`
    - Commit hash: ``

## Scenario 24: `Candidate classes take precedence and equal semantic scores use endpoint order`

- [ ] Scenario: `Candidate classes take precedence and equal semantic scores use endpoint order`
  - RED:
    - Remove `@wip` only from this scenario and leave every other tag unchanged.
    - Add regular bindings with explicit `unimplemented!()` bodies in
      `tests/steps/spec_duplicates_steps.rs`.
    - Target only this scenario:
      `GIVN_FEATURES=givn/changes/spec-duplicates/specs/spec-duplicates/spec-duplicates.feature GIVN_SPEC_EMBEDDER=stub cargo test --features spec-retrieval --test features_runner -- --name '^Candidate classes take precedence and equal semantic scores use endpoint order$'`
    - Required result: non-zero exit. Paste exact runner output:
      ```text

      ```
  - GREEN:
    - Implement and assert exact-over-structural-over-semantic precedence, two
      equal `0.9600` semantic scores, endpoint ordering by left file/title then
      right file/title, and contiguous ranks `1,2,3,4`.
    - Production files expected in GREEN: `src/spec/retrieval.rs`.
    - Test file expected: `tests/steps/spec_duplicates_steps.rs`.
    - Compile first: `cargo check --features spec-retrieval` must exit 0.
    - Rerun the targeted command. Required result: exit 0.
    - Runner output evidence, paste exact GREEN output:
      ```text

      ```
  - REFACTOR:
    - Verify sorting keys are explicit and deterministic, semantic candidates do
      not suppress deterministic findings, and all structural evidence fields
      retain the required null/non-null contract. Run `cargo fmt --check` and
      rerun the targeted command.
    - Required result: exit 0; paste exact REFACTOR output:
      ```text

      ```
  - COMMIT:
    - One atomic RED+GREEN+REFACTOR commit.
    - Required commit message: `feat(spec-duplicates): Candidate classes take precedence and equal semantic scores use endpoint order`
    - Commit hash: ``

## E2E setup task

- [ ] E2E setup: prepare and prove the real CLI smoke-test path before the E2E scenario.
  - Read `givn instructions specs --change spec-duplicates` and apply its
    normalized CLI action scope and terminal-output primary assertion policy.
  - This capability is CLI-only. Confirm that no server, database, queue,
    browser, external service, or digital twin is required; use a temporary
    initialized project and the deterministic stub embedder. Confirm the
    compiled subprocess can run a smoke invocation.
  - Use the distinct E2E command required by `design.md`:
    `GIVN_FEATURES=givn/changes/spec-duplicates/specs ./run-tests.sh --e2e`
  - Prove it is a strict subset rather than a second full-suite invocation.
    Run both commands and paste exact output:
    - Full active-change command:
      `GIVN_FEATURES=givn/changes/spec-duplicates/specs ./run-tests.sh`
    - E2E command:
      `GIVN_FEATURES=givn/changes/spec-duplicates/specs ./run-tests.sh --e2e`
    - Full scenario count: `____`; exit `____`.
    - E2E scenario count: `____`; exit `____`.
    - Required relation: E2E count is strictly less than full count. Expected
      final counts are 25 total and 1 E2E scenario.
    ```text

    ```

## Scenario 25: `Maintainer inspects the complete permanent scenario audit` (@e2e, last)

- [ ] Scenario: `Maintainer inspects the complete permanent scenario audit`
  - E2E setup before RED:
    - All 24 non-@e2e scenario boxes must already be complete with their
      targeted RED, GREEN, REFACTOR, and commit evidence. Do not remove the
      `@e2e` tag at any point.
    - Confirm the CLI-only local run needs no server, database, queue, browser,
      external service, or digital twin. Use the design's temporary initialized
      project and `GIVN_SPEC_EMBEDDER=stub` fixture path.
    - Confirm `tests/steps/spec_duplicates_e2e_steps.rs` is separate from
      `tests/steps/spec_duplicates_steps.rs`, is registered in
      `tests/steps/mod.rs`, and has non-empty Rust step bodies.
    - Run the exact standalone E2E command required by the design for the
      strict-subset proof:
      `GIVN_FEATURES=givn/changes/spec-duplicates/specs ./run-tests.sh --e2e`
      Record its scenario count and compare it with the full scoped
      `GIVN_FEATURES=givn/changes/spec-duplicates/specs ./run-tests.sh` count in
      the Setup task. The E2E command must remain distinct from
      `verify.command` and must report strictly fewer scenarios.
  - RED:
    - Remove `@wip` only from this E2E scenario. Preserve `@e2e` permanently;
      never remove or alter any other scenario's tags.
    - Add the E2E bindings only in
      `tests/steps/spec_duplicates_e2e_steps.rs`. Every new Rust binding uses
      explicit `unimplemented!()` in RED, never an empty body. The real driver
      is the compiled `givn` subprocess, not an in-process call, browser, or
      HTTP substitute.
    - Target only this E2E scenario with the design's single-scenario runner
      command adapted to its exact title:
      `GIVN_FEATURES=givn/changes/spec-duplicates/specs GIVN_SPEC_EMBEDDER=stub GIVN_SPEC_RERANKER=stub cargo test --features spec-retrieval --test features_runner -- --name '^Maintainer inspects the complete permanent scenario audit$'`
      The full E2E gate remains exactly
      `GIVN_FEATURES=givn/changes/spec-duplicates/specs ./run-tests.sh --e2e`.
    - Required result: non-zero exit from the targeted E2E scenario because of
      the explicit stub. Paste exact runner output:
      ```text

      ```
  - GREEN:
    - Read the canonical E2E policy and use the reviewed design's CLI interface
      boundary. Assert primarily on the real subprocess exit status and stdout:
      exact header/counts, candidate field order, rank 1 exact duplicate, rank
      2 structural overlap, rank 3 semantic candidate, and contiguous ranks.
      Filesystem read-only checks remain secondary evidence.
    - Production files expected in GREEN: no new production file is expected;
      this scenario must exercise the already implemented `src/main.rs`,
      `src/cli/spec.rs`, and `src/spec/retrieval.rs` through the compiled
      subprocess. Investigate any E2E-only production workaround.
    - Test file expected in GREEN:
      `tests/steps/spec_duplicates_e2e_steps.rs`; do not move these bindings
      into the regular module.
    - Compile first: `cargo check --features spec-retrieval` must exit 0.
    - Run the exact configured E2E command for the scoped feature set:
      `GIVN_FEATURES=givn/changes/spec-duplicates/specs ./run-tests.sh --e2e`
      If a single-scenario run is needed for isolation, use the targeted
      underlying command from RED and record that it selects only this title.
    - Required result: the E2E scenario passes with exit 0, and the full E2E
      command remains a strict subset of the full scoped command.
    - Runner output evidence, paste the exact GREEN output:
      ```text

      ```
  - REFACTOR:
    - Keep the primary assertions on terminal stdout and exit status, retain the
      real compiled-subprocess driver, and keep filesystem assertions secondary.
      Confirm `@e2e` is immutable, no E2E step body is empty or stubbed, run
      `cargo fmt --check`, and rerun the exact E2E command.
    - Required result: exit 0 with the same one-scenario E2E result.
    - Runner output evidence, paste the exact REFACTOR output:
      ```text

      ```
  - COMMIT:
    - Create one atomic E2E RED+GREEN+REFACTOR commit after all non-E2E
      commits. It must contain real E2E step glue and/or production integration
      work, not only a tag or stub change.
    - Required commit message: `test(e2e): Maintainer inspects the complete permanent scenario audit`
    - Commit hash: ``

## Full-suite and review handoff

- [ ] After all 25 scenario boxes are individually complete, run the full
  active-change suite with the exact scoped command:
  `GIVN_FEATURES=givn/changes/spec-duplicates/specs ./run-tests.sh`
  Record the complete runner output and confirm 25 scenarios pass, including
  24 non-@e2e scenarios and one @e2e scenario.
- [ ] Run the regular-only suite with
  `GIVN_FEATURES=givn/changes/spec-duplicates/specs ./run-tests.sh --non-e2e`.
  Record the output and confirm 24 scenarios pass.
- [ ] Run the exact E2E gate with
  `GIVN_FEATURES=givn/changes/spec-duplicates/specs ./run-tests.sh --e2e`.
  Record the output and confirm one scenario passes. Paste the full count and
  E2E count again as strict-subset evidence: full `____`, E2E `____`, and
  `1 < 25` only after the captured counts prove it.
- [ ] Run `cargo fmt --check`, `cargo check --features spec-retrieval`,
  `cargo check --no-default-features --locked`, and
  `cargo clippy --workspace --all-targets -- -D warnings`; record each command's
  actual output and exit status.
- [ ] Run `givn lint --change spec-duplicates` and the configured review gates.
  Confirm all delta @wip tags are removed one scenario at a time, the @e2e tag
  remains present, no scenario box is checked without its evidence and commit
  hash, and the interaction inventory/matrix verification remains unchanged.
- [ ] Review handoff: verify every commit is atomic per scenario, every commit
  message contains the exact scenario title, no commit is spec-only or
  stub-only, and the final task artifact is ready for `givn review` and then
  archive.
