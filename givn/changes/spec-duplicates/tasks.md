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
   Compiling givn v0.2.0 (/home/buster/projects/givn)
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
   Compiling givn v0.2.0 (/home/buster/projects/givn)
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

thread 'main' (3128883) panicked at /home/buster/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cucumber-0.23.0/src/cucumber.rs:1243:13:
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
    Checking givn v0.2.0 (/home/buster/projects/givn)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.65s
      ```
    - Rerun the same targeted command above. Required result: exactly this
      scenario passes with exit 0.
    - Runner output evidence, paste the exact GREEN output:
      ```text
   Compiling givn v0.2.0 (/home/buster/projects/givn)
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
   Compiling givn v0.2.0 (/home/buster/projects/givn)
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
   Compiling givn v0.2.0 (/home/buster/projects/givn)
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

thread 'main' (3134496) panicked at /home/buster/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cucumber-0.23.0/src/cucumber.rs:1243:13:
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
   Compiling givn v0.2.0 (/home/buster/projects/givn)
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
   Compiling givn v0.2.0 (/home/buster/projects/givn)
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

thread 'main' (3140554) panicked at /home/buster/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cucumber-0.23.0/src/cucumber.rs:1243:13:
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
    Checking givn v0.2.0 (/home/buster/projects/givn)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.63s
      ```
     - Runner output evidence, paste exact GREEN output:
      ```text
   Compiling givn v0.2.0 (/home/buster/projects/givn)
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
   Compiling givn v0.2.0 (/home/buster/projects/givn)
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
   Compiling givn v0.2.0 (/home/buster/projects/givn)
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

thread 'main' (3147398) panicked at /home/buster/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cucumber-0.23.0/src/cucumber.rs:1243:13:
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
    Checking givn v0.2.0 (/home/buster/projects/givn)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.94s
      ```
    - Rerun the targeted command. Required result: exit 0.
    - Runner output evidence, paste exact GREEN output:
      ```text
   Compiling givn v0.2.0 (/home/buster/projects/givn)
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
   Compiling givn v0.2.0 (/home/buster/projects/givn)
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
    - Commit hash: `eb126c95fcdfb84d507bbb34d47d3426d683eedd`

## Scenario 5: `Semantically related scenarios are reported as candidates`

- [x] Scenario: `Semantically related scenarios are reported as candidates`
  - RED:
    - Remove `@wip` only from this scenario. Leave all other @wip tags in
      place.
    - Add regular bindings in `tests/steps/spec_duplicates_steps.rs`; every new
      body uses `unimplemented!()` until GREEN.
    - Target only this scenario:
      `GIVN_FEATURES=givn/changes/spec-duplicates/specs/spec-duplicates/spec-duplicates.feature GIVN_SPEC_EMBEDDER=stub cargo test --features spec-retrieval --test features_runner -- --name '^Semantically related scenarios are reported as candidates$'`
    - Required result: non-zero exit. Paste exact runner output:
      ```text
   Compiling givn v0.2.0 (/home/buster/projects/givn)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 3.06s
     Running tests/features_runner.rs (target/debug/deps/features_runner-c64f1ee623f3f628)
Feature: Audit duplicate and related permanent scenarios
  Scenario: Semantically related scenarios are reported as candidates
   ✘  Given the permanent scenario "Token is required" at "givn/specs/auth/auth.feature" and "Authentication needs a token" at "givn/specs/token/token.feature" describe closely related behaviour with different wording
      Step failed:
      Defined: givn/changes/spec-duplicates/specs/spec-duplicates/spec-duplicates.feature:67:5
      Matched: tests/steps/spec_duplicates_steps.rs:139:1
      Step panicked. Captured output: not implemented
[Summary]
1 feature
1 scenario (1 failed)
1 step (1 failed)

thread 'main' (3153811) panicked at /home/buster/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cucumber-0.23.0/src/cucumber.rs:1243:13:
1 step failed
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
error: test failed, to rerun pass `--test features_runner`
      ```
  - GREEN:
    - Use the local E5 retrieval path for production and the documented
      stub-only score seam for the fixture. Report `semantic_candidate` at the
      configured score, with null structural fields, empty distinguishing facts,
      false contradiction, and advisory-only semantics.
    - Production files expected in GREEN: `src/spec/retrieval.rs`.
    - Test files: `tests/steps/spec_duplicates_steps.rs` and
      `tests/steps/mod.rs` for the separate duplicate-audit score environment.
    - Compile first: `cargo check --features spec-retrieval` exited 0.
      Exact output:
      ```text
    Checking givn v0.2.0 (/home/buster/projects/givn)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.15s
      ```
    - Rerun the targeted command. Required result: exit 0 and score `0.9600`.
    - Runner output evidence, paste exact GREEN output:
      ```text
   Compiling givn v0.2.0 (/home/buster/projects/givn)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 8.17s
     Running tests/features_runner.rs (target/debug/deps/features_runner-c64f1ee623f3f628)
Feature: Audit duplicate and related permanent scenarios
  Scenario: Semantically related scenarios are reported as candidates
   ✔  Given the permanent scenario "Token is required" at "givn/specs/auth/auth.feature" and "Authentication needs a token" at "givn/specs/token/token.feature" describe closely related behaviour with different wording
   ✔  When the user runs `givn spec duplicates`
   ✔  Then the audit reports candidate count 1
   ✔  And candidate rank 1 is classified as `semantic_candidate`
   ✔  And candidate rank 1 has left identity ("givn/specs/auth/auth.feature", "Token is required")
   ✔  And candidate rank 1 has right identity ("givn/specs/token/token.feature", "Authentication needs a token")
   ✔  And candidate rank 1 has relation `semantic_similarity`
   ✔  And candidate rank 1 has score `0.9600`
   ✔  And candidate rank 1 has null assertion shape and null fingerprint facts
   ✔  And candidate rank 1 has empty distinguishing facts and fact contradiction `false`
   ✔  And the semantic candidate is advisory and is not an automatic duplicate decision
[Summary]
1 feature
1 scenario (1 passed)
11 steps (11 passed)
      ```
    - Coverage: the targeted runner output contained no coverage summary; production coverage is unmeasured for this scenario.
  - REFACTOR:
    - Confirm the score seam cannot affect production local-E5 execution and
      that semantic evidence never suppresses deterministic evidence. Run
      `cargo fmt --check`, rerun the targeted command, and verify no stub remains.
    - `cargo fmt -- --check` exited 0 with empty stdout and stderr.
    - The malformed stub seam probe exited non-zero with:
      ```text
error: malformed duplicate score entry `malformed`; expected <endpoint>::<endpoint>=<score>
      ```
    - The unset seam stub audit completed through the normal audit path and
      emitted candidates; the output was truncated by the terminal tool and
      saved at `/home/buster/.local/share/opencode/tool-output/tool_0109c962e001d8x2jKxrTfVFcM`.
    - The non-stub boundary probe ignored the duplicate seam and returned the
      existing retrieval-model error:
      ```text
error: retrieval model is unavailable; configure a local model cache or unset GIVN_SPEC_EMBEDDER
      ```
      A local-E5 production scoring run was not feasible because the model is
      unavailable in this environment.
    - Grep of `tests/steps/spec_duplicates_steps.rs` found no
      `unimplemented!()`, `todo!()`, or empty step bodies.
    - Required targeted runner output, exit 0:
      ```text
   Compiling givn v0.2.0 (/home/buster/projects/givn)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 2.62s
     Running tests/features_runner.rs (target/debug/deps/features_runner-c64f1ee623f3f628)
Feature: Audit duplicate and related permanent scenarios
  Scenario: Semantically related scenarios are reported as candidates
   ✔  Given the permanent scenario "Token is required" at "givn/specs/auth/auth.feature" and "Authentication needs a token" at "givn/specs/token/token.feature" describe closely related behaviour with different wording
   ✔  When the user runs `givn spec duplicates`
   ✔  Then the audit reports candidate count 1
   ✔  And candidate rank 1 is classified as `semantic_candidate`
   ✔  And candidate rank 1 has left identity ("givn/specs/auth/auth.feature", "Token is required")
   ✔  And candidate rank 1 has right identity ("givn/specs/token/token.feature", "Authentication needs a token")
   ✔  And candidate rank 1 has relation `semantic_similarity`
   ✔  And candidate rank 1 has score `0.9600`
   ✔  And candidate rank 1 has null assertion shape and null fingerprint facts
   ✔  And candidate rank 1 has empty distinguishing facts and fact contradiction `false`
   ✔  And the semantic candidate is advisory and is not an automatic duplicate decision
[Summary]
1 feature
1 scenario (1 passed)
11 steps (11 passed)
      ```
  - Files touched:
    - `givn/changes/spec-duplicates/specs/spec-duplicates/spec-duplicates.feature` (Scenario 5 tag only)
    - `src/spec/retrieval.rs`
    - `tests/steps/spec_duplicates_steps.rs`
    - `tests/steps/mod.rs`
    - `givn/changes/spec-duplicates/tasks.md` (includes the pending Scenario 4 post-commit hash)
  - COMMIT:
    - One atomic RED+GREEN+REFACTOR commit.
    - Required commit message: `feat(spec-duplicates): Semantically related scenarios are reported as candidates`
    - Commit hash: `02e0429708d19c2403afa4265ca0ce6c938fa540`

## Scenario 6: `The audit reports candidates beyond the interactive top five`

- [x] Scenario: `The audit reports candidates beyond the interactive top five`
  - RED:
    - Remove `@wip` only from this scenario and leave all other tags unchanged.
    - Add any new regular bindings with non-empty `unimplemented!()` bodies in
      `tests/steps/spec_duplicates_steps.rs`.
    - Target only this scenario:
      `GIVN_FEATURES=givn/changes/spec-duplicates/specs/spec-duplicates/spec-duplicates.feature GIVN_SPEC_EMBEDDER=stub cargo test --features spec-retrieval --test features_runner -- --name '^The audit reports candidates beyond the interactive top five$'`
    - Required result: non-zero exit. Paste exact runner output:
      ```text
   Compiling givn v0.2.0 (/home/buster/projects/givn)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 3.08s
     Running tests/features_runner.rs (target/debug/deps/features_runner-c64f1ee623f3f628)
Feature: Audit duplicate and related permanent scenarios
  Scenario: The audit reports candidates beyond the interactive top five
   ✘  Given the permanent corpus contains more than five duplicate and related-scenario candidates
      Step failed:
      Defined: givn/changes/spec-duplicates/specs/spec-duplicates/spec-duplicates.feature:81:5
      Matched: tests/steps/spec_duplicates_steps.rs:238:1
      Step panicked. Captured output: not implemented
[Summary]
1 feature
1 scenario (1 failed)
1 step (1 failed)

thread 'main' (3160079) panicked at /home/buster/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cucumber-0.23.0/src/cucumber.rs:1243:13:
1 step failed
error: test failed, to rerun `cargo test --test features_runner`
      ```
  - GREEN:
    - Assert count 6, rank 6, contiguous ranks, and highest rank equal to
      candidate count against a real 12-scenario permanent corpus containing
      six exact duplicate pairs with distinct endpoint identities and assertion
      shapes. The existing duplicate audit already passes its complete
      candidate collection to rendering, so no production file required a
      change.
    - Production files modified in GREEN: none; `src/spec/retrieval.rs`
      already keeps complete candidate collection separate from rendering and
      has no top-five limit in the duplicate audit path.
    - Test file modified in GREEN: `tests/steps/spec_duplicates_steps.rs`.
    - Compile first: `cargo check --features spec-retrieval` exited 0.
      Exact output:
      ```text
    Checking givn v0.2.0 (/home/buster/projects/givn)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.77s
      ```
    - Rerun the targeted command. Required result: exit 0.
    - Runner output evidence, paste exact GREEN output:
      ```text
   Compiling givn v0.2.0 (/home/buster/projects/givn)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 2.58s
     Running tests/features_runner.rs (target/debug/deps/features_runner-c64f1ee623f3f628)
Feature: Audit duplicate and related permanent scenarios
  Scenario: The audit reports candidates beyond the interactive top five
   ✔  Given the permanent corpus contains more than five duplicate and related-scenario candidates
   ✔  When the user runs `givn spec duplicates`
   ✔  Then the audit reports candidate count 6
   ✔  And the audit includes the candidate at rank 6
   ✔  And candidate ranks are exactly `1,2,3,4,5,6`
   ✔  And the highest rank equals the candidate count
[Summary]
1 feature
1 scenario (1 passed)
6 steps (6 passed)
      ```
    - Coverage: the targeted runner output contained no coverage summary;
      production coverage is unmeasured for this scenario.
  - REFACTOR:
    - Check complete candidate collection and rank assignment remain separate
      from rendering and do not reintroduce a view limit. `duplicate_candidates`
      remains the complete collection/classification path, while
      `duplicates_enabled` assigns ranks during rendering; no `.take(5)` occurs
      in the duplicate audit path. Run `cargo fmt -- --check` and rerun the
      targeted command.
    - `cargo fmt -- --check` exited 0 with empty stdout and stderr.
    - Grep of `tests/steps/spec_duplicates_steps.rs` found no
      `unimplemented!()`, `todo!()`, or empty step bodies.
    - Required result: exit 0; paste exact REFACTOR output:
      ```text
   Compiling givn v0.2.0 (/home/buster/projects/givn)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 2.40s
     Running tests/features_runner.rs (target/debug/deps/features_runner-c64f1ee623f3f628)
Feature: Audit duplicate and related permanent scenarios
  Scenario: The audit reports candidates beyond the interactive top five
   ✔  Given the permanent corpus contains more than five duplicate and related-scenario candidates
   ✔  When the user runs `givn spec duplicates`
   ✔  Then the audit reports candidate count 6
   ✔  And the audit includes the candidate at rank 6
   ✔  And candidate ranks are exactly `1,2,3,4,5,6`
   ✔  And the highest rank equals the candidate count
[Summary]
1 feature
1 scenario (1 passed)
6 steps (6 passed)
      ```
    - Files touched: `givn/changes/spec-duplicates/specs/spec-duplicates/spec-duplicates.feature`
      (Scenario 6 tag only), `tests/steps/spec_duplicates_steps.rs`, and
      `givn/changes/spec-duplicates/tasks.md` (including the pending Scenario 5
      post-commit hash). No production source file was modified because the
      existing duplicate audit already reports the complete candidate vector.
  - COMMIT:
    - One atomic RED+GREEN+REFACTOR commit.
    - Required commit message: `feat(spec-duplicates): The audit reports candidates beyond the interactive top five`
    - Commit hash: `1a741b9ae025d37e93e94300b1f2eddddfaa1678`

## Scenario 7: `Repeating an unchanged audit produces the same report`

- [x] Scenario: `Repeating an unchanged audit produces the same report`
  - RED:
    - Remove `@wip` only from this scenario. Do not remove @wip elsewhere.
    - Add non-empty regular step stubs using `unimplemented!()` in
      `tests/steps/spec_duplicates_steps.rs`.
    - Target only this scenario:
      `GIVN_FEATURES=givn/changes/spec-duplicates/specs/spec-duplicates/spec-duplicates.feature GIVN_SPEC_EMBEDDER=stub cargo test --features spec-retrieval --test features_runner -- --name '^Repeating an unchanged audit produces the same report$'`
    - Required result: non-zero exit. Paste exact runner output:
      ```text
   Compiling givn v0.2.0 (/home/buster/projects/givn)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 2.33s
     Running tests/features_runner.rs (target/debug/deps/features_runner-c64f1ee623f3f628)
Feature: Audit duplicate and related permanent scenarios
  Scenario: Repeating an unchanged audit produces the same report
   ✘  Given a permanent scenario corpus with duplicate and related-scenario candidates
      Step failed:
      Defined: givn/changes/spec-duplicates/specs/spec-duplicates/spec-duplicates.feature:90:5
      Matched: tests/steps/spec_duplicates_steps.rs:435:1
      Step panicked. Captured output: not implemented
[Summary]
1 feature
1 scenario (1 failed)
1 step (1 failed)

thread 'main' (3163491) panicked at /home/buster/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cucumber-0.23.0/src/cucumber.rs:1243:13:
1 step failed
error: test failed, to rerun `cargo test --test features_runner`
      ```
  - GREEN:
    - Implement deterministic collection, classification, sorting, rank
      assignment, and text/JSON serialization so two unchanged invocations are
      byte-for-byte identical with identical counts and evidence.
    - Production files modified in GREEN: none; `src/spec/retrieval.rs` already
      sorts permanent paths and endpoint identities, uses total floating-point
      ordering with endpoint tie-breaks, and renders stable text evidence.
    - Test file modified in GREEN: `tests/steps/spec_duplicates_steps.rs`.
    - Compile first: `cargo check --features spec-retrieval` exited 0.
      Exact output:
      ```text
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.13s
      ```
    - Rerun the targeted command. Required result: exit 0.
    - Runner output evidence, paste exact GREEN output:
      ```text
    Blocking waiting for file lock on package cache
    Blocking waiting for file lock on package cache
    Blocking waiting for file lock on package cache
    Blocking waiting for file lock on build directory
   Compiling givn v0.2.0 (/home/buster/projects/givn)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 2.06s
     Running tests/features_runner.rs (target/debug/deps/features_runner-c64f1ee623f3f628)
Feature: Audit duplicate and related permanent scenarios
  Scenario: Repeating an unchanged audit produces the same report
   ✔  Given a permanent scenario corpus with duplicate and related-scenario candidates
   ✔  When the user runs `givn spec duplicates` twice without changing the corpus
   ✔  Then both audit reports are byte-for-byte identical
   ✔  And both reports have the same scanned scenario count, candidate count, ranks, and evidence
[Summary]
1 feature
1 scenario (1 passed)
4 steps (4 passed)
      ```
    - Coverage: the targeted runner output contained no coverage summary;
      production coverage is unmeasured for this scenario.
  - REFACTOR:
    - Review all ordering keys, total floating-point ordering, endpoint tie
      breaks, LF endings, and final newline. Run `cargo fmt --check` and rerun
      the same command.
    - The assertions explicitly validate exact-before-structural-before-semantic
      ordering, equal-score endpoint ordering (`account` before `auth`), null
      serialization for deterministic candidates, four-decimal `0.9600` semantic
      serialization, LF-only output, and exactly one final newline.
    - `cargo fmt -- --check` exited 0 with empty stdout and stderr.
    - Grep of `tests/steps/spec_duplicates_steps.rs` found no
      `unimplemented!()`, `todo!()`, or empty step bodies.
    - Required result: exit 0; paste exact REFACTOR output:
      ```text
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.13s
     Running tests/features_runner.rs (target/debug/deps/features_runner-c64f1ee623f3f628)
Feature: Audit duplicate and related permanent scenarios
  Scenario: Repeating an unchanged audit produces the same report
   ✔  Given a permanent scenario corpus with duplicate and related-scenario candidates
   ✔  When the user runs `givn spec duplicates` twice without changing the corpus
   ✔  Then both audit reports are byte-for-byte identical
   ✔  And both reports have the same scanned scenario count, candidate count, ranks, and evidence
[Summary]
1 feature
1 scenario (1 passed)
4 steps (4 passed)
      ```
    - Files touched: `givn/changes/spec-duplicates/specs/spec-duplicates/spec-duplicates.feature`
      (Scenario 7 tag only), `tests/steps/spec_duplicates_steps.rs`, and
      `givn/changes/spec-duplicates/tasks.md` (including the pending Scenario 6
      post-commit hash). No production source file was modified because the
      existing duplicate collection, sorting, and renderer already satisfy the
      determinism contract.
  - COMMIT:
    - One atomic RED+GREEN+REFACTOR commit.
    - Required commit message: `feat(spec-duplicates): Repeating an unchanged audit produces the same report`
    - Commit hash: `b6e2a040cf732b67255b673378f33e5466b943cb`

## Scenario 8: `JSON output identifies every audit candidate`

- [x] Scenario: `JSON output identifies every audit candidate`
  - RED:
    - Remove `@wip` only from this scenario; preserve every other scenario tag.
    - Add regular bindings with explicit `unimplemented!()` stubs in
      `tests/steps/spec_duplicates_steps.rs`.
    - Target only this scenario:
      `GIVN_FEATURES=givn/changes/spec-duplicates/specs/spec-duplicates/spec-duplicates.feature GIVN_SPEC_EMBEDDER=stub cargo test --features spec-retrieval --test features_runner -- --name '^JSON output identifies every audit candidate$'`
    - Required result: non-zero exit. Paste exact runner output:
      ```text
   Compiling givn v0.2.0 (/home/buster/projects/givn)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 2.37s
     Running tests/features_runner.rs (target/debug/deps/features_runner-c64f1ee623f3f628)
Feature: Audit duplicate and related permanent scenarios
  Scenario: JSON output identifies every audit candidate
   ✔  Given a permanent scenario corpus with duplicate and related-scenario candidates
   ✔  When the user runs `givn spec duplicates --format json`
   ✘  Then stdout is a valid JSON audit document
      Step failed:
      Defined: givn/changes/spec-duplicates/specs/spec-duplicates/spec-duplicates.feature:99:5
      Matched: tests/steps/spec_duplicates_steps.rs:666:1
      Step panicked. Captured output: not implemented
[Summary]
1 feature
1 scenario (1 failed)
3 steps (2 passed, 1 failed)

thread 'main' (3167838) panicked at /home/buster/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cucumber-0.23.0/src/cucumber.rs:1243:13:
1 step failed
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
error: test failed, to rerun `cargo test --test features_runner`
      ```
  - GREEN:
    - Implement the exact JSON schema, field order, endpoint key order,
      evidence key order, classifications, relations, deterministic ordering,
      contiguous ranks, eligible non-removed scanned count, canonical endpoint
      identities, four-decimal semantic score strings, and null structural
      evidence for exact and semantic candidates.
    - Production files modified in GREEN: `src/main.rs`, `src/cli/spec.rs`, and
      `src/spec/retrieval.rs`.
    - Test file modified in GREEN: `tests/steps/spec_duplicates_steps.rs`.
    - Compile first: `cargo check --features spec-retrieval` exited 0.
      Exact output:
      ```text
    Checking givn v0.2.0 (/home/buster/projects/givn)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.78s
      ```
    - Rerun the targeted command. Required result: exit 0.
    - Runner output evidence, paste exact GREEN output:
      ```text
   Compiling givn v0.2.0 (/home/buster/projects/givn)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 3.82s
     Running tests/features_runner.rs (target/debug/deps/features_runner-c64f1ee623f3f628)
Feature: Audit duplicate and related permanent scenarios
  Scenario: JSON output identifies every audit candidate
   ✔  Given a permanent scenario corpus with duplicate and related-scenario candidates
   ✔  When the user runs `givn spec duplicates --format json`
   ✔  Then stdout is a valid JSON audit document
   ✔  And the JSON schema_version is exactly 1
   ✔  And the JSON top-level keys are exactly `schema_version,scanned_scenarios,candidate_count,candidates` in that order
   ✔  And every JSON candidate has keys exactly `rank,kind,left,right,evidence` in that order
   ✔  And every JSON endpoint has keys exactly `file,title` in that order
   ✔  And every JSON evidence object has keys exactly `relation,assertion_shape,facts,distinguishing_facts,fact_contradiction,score` in that order
   ✔  And every JSON candidate has an exact classification, both scenario identities, a relation, and a rank
   ✔  And the JSON candidates follow the deterministic audit ordering
   ✔  And JSON ranks are contiguous from 1 through candidate_count
   ✔  And JSON scanned_scenarios equals the number of eligible non-removed permanent scenarios
[Summary]
1 feature
1 scenario (1 passed)
12 steps (12 passed)
      ```
    - Coverage: the targeted runner output contained no coverage summary;
      production coverage is unmeasured for this scenario.
  - REFACTOR:
    - Text and JSON consume the same sorted `duplicate_candidates` vector.
      JSON uses ordered structs rather than maps for the report, candidates,
      endpoints, and evidence; fingerprint maps remain `BTreeMap`s. Endpoint
      ordering, contiguous rank assignment, null structural evidence, semantic
      empty facts, false contradiction, and four-decimal score strings are
      asserted by the regular steps.
    - `cargo fmt -- --check` exited 0 with empty stdout and stderr.
    - Grep of `tests/steps/spec_duplicates_steps.rs` found no `unimplemented!()`,
      `todo!()`, or empty step bodies.
    - Required result: exit 0; paste exact REFACTOR output:
      ```text
   Compiling givn v0.2.0 (/home/buster/projects/givn)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 2.52s
     Running tests/features_runner.rs (target/debug/deps/features_runner-c64f1ee623f3f628)
Feature: Audit duplicate and related permanent scenarios
  Scenario: JSON output identifies every audit candidate
   ✔  Given a permanent scenario corpus with duplicate and related-scenario candidates
   ✔  When the user runs `givn spec duplicates --format json`
   ✔  Then stdout is a valid JSON audit document
   ✔  And the JSON schema_version is exactly 1
   ✔  And the JSON top-level keys are exactly `schema_version,scanned_scenarios,candidate_count,candidates` in that order
   ✔  And every JSON candidate has keys exactly `rank,kind,left,right,evidence` in that order
   ✔  And every JSON endpoint has keys exactly `file,title` in that order
   ✔  And every JSON evidence object has keys exactly `relation,assertion_shape,facts,distinguishing_facts,fact_contradiction,score` in that order
   ✔  And every JSON candidate has an exact classification, both scenario identities, a relation, and a rank
   ✔  And the JSON candidates follow the deterministic audit ordering
   ✔  And JSON ranks are contiguous from 1 through candidate_count
   ✔  And JSON scanned_scenarios equals the number of eligible non-removed permanent scenarios
[Summary]
1 feature
1 scenario (1 passed)
12 steps (12 passed)
      ```
    - Files touched: `givn/changes/spec-duplicates/specs/spec-duplicates/spec-duplicates.feature`
      (Scenario 8 tag only), `src/main.rs`, `src/cli/spec.rs`,
      `src/spec/retrieval.rs`, `tests/steps/spec_duplicates_steps.rs`, and
      `givn/changes/spec-duplicates/tasks.md` (including the pending Scenario 7
      post-commit hash update).
  - COMMIT:
    - One atomic RED+GREEN+REFACTOR commit.
    - Required commit message: `feat(spec-duplicates): JSON output identifies every audit candidate`
    - Commit hash: `a98eed298acece8cf2375e3560251accc6ae22b3`

## Scenario 9: `Auditing leaves permanent specifications unchanged`

- [x] Scenario: `Auditing leaves permanent specifications unchanged`
  - RED:
    - Remove `@wip` only from this scenario and leave all other @wip tags.
    - Add regular snapshot/assertion bindings with `unimplemented!()` bodies in
      `tests/steps/spec_duplicates_steps.rs`.
    - Target only this scenario:
      `GIVN_FEATURES=givn/changes/spec-duplicates/specs/spec-duplicates/spec-duplicates.feature GIVN_SPEC_EMBEDDER=stub cargo test --features spec-retrieval --test features_runner -- --name '^Auditing leaves permanent specifications unchanged$'`
    - Required result: non-zero exit. Paste exact runner output:
      ```text
   Compiling givn v0.2.0 (/home/buster/projects/givn)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 2.44s
     Running tests/features_runner.rs (target/debug/deps/features_runner-c64f1ee623f3f628)
Feature: Audit duplicate and related permanent scenarios
  Scenario: Auditing leaves permanent specifications unchanged
   ✔  Given a permanent scenario corpus with duplicate and related-scenario candidates
   ✔  And the permanent specifications are snapshotted
   ✔  When the user runs `givn spec duplicates`
   ✘  Then the permanent specifications are unchanged byte-for-byte
      Step failed:
      Defined: givn/changes/spec-duplicates/specs/spec-duplicates/spec-duplicates.feature:115:5
      Matched: tests/steps/spec_duplicates_steps.rs:493:1
      Step panicked. Captured output: not implemented
[Summary]
1 feature
1 scenario (1 failed)
4 steps (3 passed, 1 failed)

thread 'main' (3174002) panicked at /home/buster/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cucumber-0.23.0/src/cucumber.rs:1243:13:
1 step failed
error: test failed, to rerun `cargo test --test features_runner`
      ```
  - GREEN:
    - Read permanent feature bytes for the fixture, run the real subprocess,
      and assert every permanent specification is byte-for-byte unchanged.
      The audit must not write or rewrite `givn/specs/`.
    - Production files expected in GREEN: `src/spec/retrieval.rs`.
    - Test file expected: `tests/steps/spec_duplicates_steps.rs`.
    - Compile first: `cargo check --features spec-retrieval` must exit 0.
    - Compile output:
      ```text
    Checking givn v0.2.0 (/home/buster/projects/givn)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.11s
      ```
    - Rerun the targeted command. Required result: exit 0.
    - Runner output evidence, paste exact GREEN output:
      ```text
   Compiling givn v0.2.0 (/home/buster/projects/givn)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 2.60s
     Running tests/features_runner.rs (target/debug/deps/features_runner-c64f1ee623f3f628)
Feature: Audit duplicate and related permanent scenarios
  Scenario: Auditing leaves permanent specifications unchanged
   ✔  Given a permanent scenario corpus with duplicate and related-scenario candidates
   ✔  And the permanent specifications are snapshotted
   ✔  When the user runs `givn spec duplicates`
   ✔  Then the permanent specifications are unchanged byte-for-byte
[Summary]
1 feature
1 scenario (1 passed)
4 steps (4 passed)
      ```
    - Coverage: the targeted runner output contained no coverage summary;
      production coverage is unmeasured for this scenario.
  - REFACTOR:
    - Keep the read-only boundary explicit and keep filesystem snapshots as
      secondary evidence to the CLI result. Run `cargo fmt --check` and rerun
      the targeted command.
    - `cargo fmt -- --check` exited 0 with empty stdout and stderr.
    - Grep of `tests/steps/spec_duplicates_steps.rs` found no
      `unimplemented!()` or `todo!()` bodies (`No files found`).
    - The empty-function-body grep found no matches (`No files found`).
    - Required result: exit 0; paste exact REFACTOR output:
      ```text
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.14s
     Running tests/features_runner.rs (target/debug/deps/features_runner-c64f1ee623f3f628)
Feature: Audit duplicate and related permanent scenarios
  Scenario: Auditing leaves permanent specifications unchanged
   ✔  Given a permanent scenario corpus with duplicate and related-scenario candidates
   ✔  And the permanent specifications are snapshotted
   ✔  When the user runs `givn spec duplicates`
   ✔  Then the permanent specifications are unchanged byte-for-byte
[Summary]
1 feature
1 scenario (1 passed)
4 steps (4 passed)
      ```
  - COMMIT:
    - One atomic RED+GREEN+REFACTOR commit.
    - Required commit message: `feat(spec-duplicates): Auditing leaves permanent specifications unchanged`
    - Actual production files modified: none; the existing read-only audit path
      in `src/spec/retrieval.rs` already avoids permanent-spec and index writes.
    - Test and feature files modified:
      - `givn/changes/spec-duplicates/specs/spec-duplicates/spec-duplicates.feature` (Scenario 9 tag only)
      - `tests/steps/spec_duplicates_steps.rs`
      - `givn/changes/spec-duplicates/tasks.md`
    - Commit hash: `c465d8a7d76d61b3ad10167694ecf6d619123fa9`

## Scenario 10: `Auditing leaves the persistent retrieval index unchanged`

- [x] Scenario: `Auditing leaves the persistent retrieval index unchanged`
  - RED:
    - Remove `@wip` only from this scenario. Leave every other tag untouched.
    - Add non-empty regular step definitions with `unimplemented!()` in
      `tests/steps/spec_duplicates_steps.rs`.
    - Target only this scenario:
      `GIVN_FEATURES=givn/changes/spec-duplicates/specs/spec-duplicates/spec-duplicates.feature GIVN_SPEC_EMBEDDER=stub cargo test --features spec-retrieval --test features_runner -- --name '^Auditing leaves the persistent retrieval index unchanged$'`
    - Required result: non-zero exit. Paste exact runner output:
      ```text
   Compiling givn v0.2.0 (/home/buster/projects/givn)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 2.44s
     Running tests/features_runner.rs (target/debug/deps/features_runner-c64f1ee623f3f628)
Feature: Audit duplicate and related permanent scenarios
  Scenario: Auditing leaves the persistent retrieval index unchanged
   ✔  Given a permanent scenario corpus with duplicate and related-scenario candidates
   ✘  And the persistent retrieval index is built and snapshotted
      Step failed:
      Defined: givn/changes/spec-duplicates/specs/spec-duplicates/spec-duplicates.feature:120:5
      Matched: tests/steps/spec_duplicates_steps.rs:493:1
      Step panicked. Captured output: not implemented
[Summary]
1 feature
1 scenario (1 failed)
2 steps (1 passed, 1 failed)

thread 'main' (3176663) panicked at /home/buster/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cucumber-0.23.0/src/cucumber.rs:1243:13:
1 step failed
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
error: test failed, to rerun `cargo test --test features_runner`
      ```
  - GREEN:
    - Build the existing persistent retrieval index through the compiled
      subprocess with `GIVN_SPEC_EMBEDDER=stub`, assert setup success, and reuse
      `GivnWorld.permanent_index_snapshot` for the `.spec-index.json` bytes.
      The duplicate audit reports from current permanent files with transient
      embeddings; `duplicates_enabled` does not call `refresh_index` or read
      persistent index entries as evidence.
    - Production files modified in GREEN: none; the existing read-only audit
      path in `src/spec/retrieval.rs` already enforces this boundary.
    - Test file modified in GREEN: `tests/steps/spec_duplicates_steps.rs`.
    - Compile first: `cargo check --features spec-retrieval` exited 0.
      Exact output:
      ```text
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.13s
      ```
    - Rerun the targeted command. Required result: exit 0.
    - Runner output evidence, paste exact GREEN output:
      ```text
   Compiling givn v0.2.0 (/home/buster/projects/givn)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 2.46s
     Running tests/features_runner.rs (target/debug/deps/features_runner-c64f1ee623f3f628)
Feature: Audit duplicate and related permanent scenarios
  Scenario: Auditing leaves the persistent retrieval index unchanged
   ✔  Given a permanent scenario corpus with duplicate and related-scenario candidates
   ✔  And the persistent retrieval index is built and snapshotted
   ✔  When the user runs `givn spec duplicates`
   ✔  Then the persistent retrieval index is unchanged byte-for-byte
[Summary]
1 feature
1 scenario (1 passed)
4 steps (4 passed)
      ```
    - Coverage: the targeted runner output contained no coverage summary;
      production coverage is unmeasured for this scenario.
  - REFACTOR:
    - Keep setup and snapshot helpers explicit and scope index reads to
      `givn/.spec-index.json`. The CLI status and parsed audit report are
      asserted before the filesystem snapshot comparison. Source inspection
      confirms the duplicate path has no `refresh_index` or persistent-index
      read; persistent entries are not evidence.
    - `cargo fmt -- --check` exited 0 with empty stdout and stderr.
    - Grep of `tests/steps/spec_duplicates_steps.rs` found no `unimplemented!()`
      or `todo!()` bodies (`No files found`).
    - The empty-function-body grep found no matches (`No files found`).
    - Required result: exit 0; paste exact REFACTOR output:
      ```text
   Compiling givn v0.2.0 (/home/buster/projects/givn)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 2.59s
     Running tests/features_runner.rs (target/debug/deps/features_runner-c64f1ee623f3f628)
Feature: Audit duplicate and related permanent scenarios
  Scenario: Auditing leaves the persistent retrieval index unchanged
   ✔  Given a permanent scenario corpus with duplicate and related-scenario candidates
   ✔  And the persistent retrieval index is built and snapshotted
   ✔  When the user runs `givn spec duplicates`
   ✔  Then the persistent retrieval index is unchanged byte-for-byte
[Summary]
1 feature
1 scenario (1 passed)
4 steps (4 passed)
      ```
    - Files touched: `givn/changes/spec-duplicates/specs/spec-duplicates/spec-duplicates.feature`
      (Scenario 10 tag only), `tests/steps/spec_duplicates_steps.rs`, and
      `givn/changes/spec-duplicates/tasks.md` (including the pending Scenario 9
      hash update). No production source file was modified because the existing
      `src/spec/retrieval.rs` duplicate path already uses current permanent
      records and transient embeddings without refreshing or reading the index.
  - COMMIT:
    - One atomic RED+GREEN+REFACTOR commit.
    - Required commit message: `feat(spec-duplicates): Auditing leaves the persistent retrieval index unchanged`
    - Commit hash: `f1e50ac6208b067044b0a2ac1e3e2e82e20ca632`

## Scenario 11: `Active change scenarios are not part of the permanent audit`

- [x] Scenario: `Active change scenarios are not part of the permanent audit`
  - RED:
    - Remove `@wip` only from this scenario; do not touch any other @wip tag.
    - Add regular fixture and assertion bindings with `unimplemented!()` in
      `tests/steps/spec_duplicates_steps.rs`.
    - Target only this scenario:
      `GIVN_FEATURES=givn/changes/spec-duplicates/specs/spec-duplicates/spec-duplicates.feature GIVN_SPEC_EMBEDDER=stub cargo test --features spec-retrieval --test features_runner -- --name '^Active change scenarios are not part of the permanent audit$'`
    - Required result: non-zero exit. Paste exact runner output:
      ```text
   Compiling givn v0.2.0 (/home/buster/projects/givn)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 3.27s
     Running tests/features_runner.rs (target/debug/deps/features_runner-c64f1ee623f3f628)
Feature: Audit duplicate and related permanent scenarios
  Scenario: Active change scenarios are not part of the permanent audit
   ✘  Given a permanent scenario corpus with one related scenario
      Step failed:
      Defined: givn/changes/spec-duplicates/specs/spec-duplicates/spec-duplicates.feature:126:5
      Matched: tests/steps/spec_duplicates_steps.rs:422:1
      Step panicked. Captured output: not implemented
[Summary]
1 feature
1 scenario (1 failed)
1 step (1 failed)

thread 'main' (3179756) panicked at /home/buster/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cucumber-0.23.0/src/cucumber.rs:1243:13:
1 step failed
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
error: test failed, to rerun `cargo test --test features_runner`
      ```
  - GREEN:
    - Restrict the audit aggregate and endpoint identities to eligible permanent
      non-removed scenarios under `givn/specs/`; assert active identities are
      absent and every endpoint file starts with `givn/specs/`.
    - Production files modified in GREEN: none; the existing
      `collect_duplicate_scenarios` path in `src/spec/retrieval.rs` already
      collects only `permanent_specs_path(root)` and does not scan changes or
      archive paths.
    - Test file modified in GREEN: `tests/steps/spec_duplicates_steps.rs`.
    - Compile first: `cargo check --features spec-retrieval` must exit 0.
      Exact output:
      ```text
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.13s
      ```
    - Rerun the targeted command. Required result: exit 0.
    - Runner output evidence, paste exact GREEN output:
      ```text
   Compiling givn v0.2.0 (/home/buster/projects/givn)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 2.68s
     Running tests/features_runner.rs (target/debug/deps/features_runner-c64f1ee623f3f628)
Feature: Audit duplicate and related permanent scenarios
  Scenario: Active change scenarios are not part of the permanent audit
   ✔  Given a permanent scenario corpus with one related scenario
   ✔  And an active change contains another scenario with related behaviour
   ✔  When the user runs `givn spec duplicates`
   ✔  Then the audit reports only relationships between permanent scenarios
   ✔  And the active change scenario identity is absent from every candidate
   ✔  And every candidate endpoint file starts with `givn/specs/`
[Summary]
1 feature
1 scenario (1 passed)
6 steps (6 passed)
      ```
    - Coverage: the targeted runner output contained no coverage summary;
      production coverage is unmeasured for this scenario.
  - REFACTOR:
    - Keep active-change path filtering at the collection boundary, not as a
      renderer-only suppression. Name the permanent and active fixture paths
      explicitly and parse both candidate endpoint files before asserting their
      `givn/specs/` prefix. Run `cargo fmt --check` and rerun the targeted
      command.
    - `cargo fmt -- --check` exited 0 with empty stdout and stderr.
    - Step-file scan for `unimplemented!()` or `todo!()` returned `No files
      found`.
    - Empty-function-body scan returned `No files found`.
    - Bare-return-body scan returned `No files found`.
    - Required result: exit 0; paste exact REFACTOR output:
      ```text
   Compiling givn v0.2.0 (/home/buster/projects/givn)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 2.40s
     Running tests/features_runner.rs (target/debug/deps/features_runner-c64f1ee623f3f628)
Feature: Audit duplicate and related permanent scenarios
  Scenario: Active change scenarios are not part of the permanent audit
   ✔  Given a permanent scenario corpus with one related scenario
   ✔  And an active change contains another scenario with related behaviour
   ✔  When the user runs `givn spec duplicates`
   ✔  Then the audit reports only relationships between permanent scenarios
   ✔  And the active change scenario identity is absent from every candidate
   ✔  And every candidate endpoint file starts with `givn/specs/`
[Summary]
1 feature
1 scenario (1 passed)
6 steps (6 passed)
      ```
    - Files touched: `givn/changes/spec-duplicates/specs/spec-duplicates/spec-duplicates.feature`
      (Scenario 11 tag only), `tests/steps/spec_duplicates_steps.rs`, and
      `givn/changes/spec-duplicates/tasks.md` (including the pending Scenario 10
      hash update). No production source file was modified because the existing
      `src/spec/retrieval.rs` collection boundary is already permanent-only.
  - COMMIT:
    - One atomic RED+GREEN+REFACTOR commit.
    - Required commit message: `feat(spec-duplicates): Active change scenarios are not part of the permanent audit`
    - Commit hash: `0b4f50ece41ec3e192680bc80215120e133a01ff`

## Scenario 12: `An audit with no candidates reports an empty result`

- [x] Scenario: `An audit with no candidates reports an empty result`
  - RED:
    - Remove `@wip` only from this scenario and leave all other tags unchanged.
    - Add regular bindings with explicit `unimplemented!()` stubs in
      `tests/steps/spec_duplicates_steps.rs`.
    - Target only this scenario:
      `GIVN_FEATURES=givn/changes/spec-duplicates/specs/spec-duplicates/spec-duplicates.feature GIVN_SPEC_EMBEDDER=stub cargo test --features spec-retrieval --test features_runner -- --name '^An audit with no candidates reports an empty result$'`
    - Required result: non-zero exit. Paste exact runner output:
      ```text
   Compiling givn v0.2.0 (/home/buster/projects/givn)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 3.31s
     Running tests/features_runner.rs (target/debug/deps/features_runner-c64f1ee623f3f628)
Feature: Audit duplicate and related permanent scenarios
  Scenario: An audit with no candidates reports an empty result
   ✘  Given a permanent scenario corpus with no duplicate or related behaviour
      Step failed:
      Defined: givn/changes/spec-duplicates/specs/spec-duplicates/spec-duplicates.feature:135:5
      Matched: tests/steps/spec_duplicates_steps.rs:345:1
      Step panicked. Captured output: not implemented
[Summary]
1 feature
1 scenario (1 failed)
1 step (1 failed)

thread 'main' (3184054) panicked at /home/buster/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cucumber-0.23.0/src/cucumber.rs:1243:13:
1 step failed
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
error: test failed, to rerun `cargo test --test features_runner`
      ```
  - GREEN:
    - Render the exact zero-candidate text report and matching JSON result for
      exactly two eligible scenarios with no exact, structural, or semantic
      relationship, with zero candidates and an empty array. The fixture is a
      non-empty corpus and does not exercise missing or empty-directory behavior.
    - Production file modified in GREEN: `src/spec/retrieval.rs`.
    - Test file modified in GREEN: `tests/steps/spec_duplicates_steps.rs`.
    - Compile first: `cargo check --features spec-retrieval` must exit 0.
      Exact output:
      ```text
    Checking givn v0.2.0 (/home/buster/projects/givn)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.12s
      ```
    - Rerun the targeted command. Required result: exit 0.
    - Runner output evidence, paste exact GREEN output:
      ```text
   Compiling givn v0.2.0 (/home/buster/projects/givn)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 3.29s
     Running tests/features_runner.rs (target/debug/deps/features_runner-c64f1ee623f3f628)
Feature: Audit duplicate and related permanent scenarios
  Scenario: An audit with no candidates reports an empty result
   ✔  Given a permanent scenario corpus with no duplicate or related behaviour
   ✔  When the user runs `givn spec duplicates`
   ✔  Then the command exits 0
   ✔  And the text report is exactly `Givn spec duplicates\nScanned scenarios: 2\nCandidate count: 0\n`
   ✔  And the JSON report has scanned_scenarios 2, candidate_count 0, and an empty candidates array
[Summary]
1 feature
1 scenario (1 passed)
5 steps (5 passed)
      ```
    - Coverage: the targeted runner output contained no coverage summary;
      production coverage is unmeasured for this scenario.
  - REFACTOR:
    - Keep empty and missing-corpus rendering on the same deterministic report
      path; verify exactly one final newline and no diagnostic prefix/suffix.
      Share the existing text-report parser and JSON document parser between
      the empty-result assertions and the candidate assertions without changing
      exact text or JSON bytes. Run `cargo fmt -- --check` and rerun the
      targeted command.
    - `cargo fmt -- --check` exited 0 with empty stdout and stderr.
    - Step-file scan for `unimplemented!()` or `todo!()` returned `No files
      found`.
    - Empty-function-body scan returned `No files found`.
    - Bare-return-body scan returned `No files found`.
    - Required result: exit 0; paste exact REFACTOR output:
      ```text
   Compiling givn v0.2.0 (/home/buster/projects/givn)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 3.57s
     Running tests/features_runner.rs (target/debug/deps/features_runner-c64f1ee623f3f628)
Feature: Audit duplicate and related permanent scenarios
  Scenario: An audit with no candidates reports an empty result
   ✔  Given a permanent scenario corpus with no duplicate or related behaviour
   ✔  When the user runs `givn spec duplicates`
   ✔  Then the command exits 0
   ✔  And the text report is exactly `Givn spec duplicates\nScanned scenarios: 2\nCandidate count: 0\n`
   ✔  And the JSON report has scanned_scenarios 2, candidate_count 0, and an empty candidates array
[Summary]
1 feature
1 scenario (1 passed)
5 steps (5 passed)
      ```
    - Files touched: `givn/changes/spec-duplicates/specs/spec-duplicates/spec-duplicates.feature`
      (Scenario 12 tag only), `src/spec/retrieval.rs`,
      `tests/steps/spec_duplicates_steps.rs`, and
      `givn/changes/spec-duplicates/tasks.md` (including the pending Scenario 11
      hash update). No design artifact or later `@wip` tag was changed.
  - COMMIT:
    - One atomic RED+GREEN+REFACTOR commit.
    - Required commit message: `feat(spec-duplicates): An audit with no candidates reports an empty result`
    - Commit hash: `5afcf5da58f71cb2c2bbc5e9b1a83cdab65c1f51`

## Scenario 13: `The audit reports retrieval unavailable without the retrieval feature`

- [x] Scenario: `The audit reports retrieval unavailable without the retrieval feature`
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
   Compiling givn v0.2.0 (/home/buster/projects/givn)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 2.46s
     Running tests/features_runner.rs (target/debug/deps/features_runner-c64f1ee623f3f628)
Feature: Audit duplicate and related permanent scenarios
  Scenario: The audit reports retrieval unavailable without the retrieval feature
   ✔  Given a permanent scenario corpus with duplicate and related-scenario candidates
   ✘  When the user runs `givn spec duplicates` through a build without spec retrieval
      Step failed:
      Defined: givn/changes/spec-duplicates/specs/spec-duplicates/spec-duplicates.feature:144:5
      Matched: tests/steps/spec_duplicates_steps.rs:656:1
      Step panicked. Captured output: not implemented
[Summary]
1 feature
1 scenario (1 failed)
2 steps (1 passed, 1 failed)

thread 'main' (3188710) panicked at /home/buster/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cucumber-0.23.0/src/cucumber.rs:1243:13:
1 step failed
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
error: test failed, to rerun `cargo test --test features_runner`
      ```
  - GREEN:
    - Reuse the existing permanent duplicate/related fixture and invoke the real
      no-feature Cargo subprocess from the temporary project root with an
      isolated `CARGO_TARGET_DIR`. The existing feature-gated production path
      returns the explicit retrieval-unavailable error and the scenario asserts
      a non-zero exit plus stderr containing `spec retrieval is unavailable`.
    - Production files modified in GREEN: none. The existing dispatch in
      `src/main.rs`, delegation in `src/cli/spec.rs`, and feature-gated error in
      `src/spec/retrieval.rs` already satisfy this contract; feature gating was
      not changed.
    - Test file modified in GREEN: `tests/steps/spec_duplicates_steps.rs`.
    - Compile first, both must exit 0:
      `cargo check --no-default-features --locked`
      and `cargo check --features spec-retrieval`.
    - `cargo check --no-default-features --locked` output:
      ```text
    Checking givn v0.2.0 (/home/buster/projects/givn)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.61s
      ```
    - `cargo check --features spec-retrieval` output:
      ```text
    Checking givn v0.2.0 (/home/buster/projects/givn)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.11s
      ```
    - Rerun the targeted command. Required result: exit 0 for the feature-runner
      scenario while the scenario's inner no-feature subprocess exits non-zero
      with the required message.
    - Runner output evidence, paste exact GREEN output:
       ```text
   Compiling givn v0.2.0 (/home/buster/projects/givn)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 2.51s
     Running tests/features_runner.rs (target/debug/deps/features_runner-c64f1ee623f3f628)
Feature: Audit duplicate and related permanent scenarios
  Scenario: The audit reports retrieval unavailable without the retrieval feature
   ✔  Given a permanent scenario corpus with duplicate and related-scenario candidates
   ✔  When the user runs `givn spec duplicates` through a build without spec retrieval
   ✔  Then the command exits non-zero
   ✔  And stderr contains "spec retrieval is unavailable"
[Summary]
1 feature
1 scenario (1 passed)
4 steps (4 passed)
      ```
    - Coverage: the targeted runner output contained no coverage summary;
      production coverage is unmeasured for this scenario.
  - REFACTOR:
    - Reuse the existing no-feature subprocess pattern: `cargo run --quiet`
      with the repository manifest, `--no-default-features`, temporary-project
      `current_dir`, isolated `CARGO_TARGET_DIR`, and null stdin. Keep the
      stderr assertion as the primary error-contract assertion.
    - `cargo fmt -- --check` exited 0 with empty stdout and stderr.
    - Step-file scan for `unimplemented!()` or `todo!()`: `No files found`.
    - Empty-function-body scan: `No files found`.
    - Bare-return-body scan: `No files found`.
     - Required result: exit 0; paste exact REFACTOR output:
      ```text
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.13s
     Running tests/features_runner.rs (target/debug/deps/features_runner-c64f1ee623f3f628)
Feature: Audit duplicate and related permanent scenarios
  Scenario: The audit reports retrieval unavailable without the retrieval feature
   ✔  Given a permanent scenario corpus with duplicate and related-scenario candidates
   ✔  When the user runs `givn spec duplicates` through a build without spec retrieval
   ✔  Then the command exits non-zero
   ✔  And stderr contains "spec retrieval is unavailable"
[Summary]
1 feature
1 scenario (1 passed)
4 steps (4 passed)
      ```
    - Files touched: `givn/changes/spec-duplicates/specs/spec-duplicates/spec-duplicates.feature`
      (Scenario 13 tag only), `tests/steps/spec_duplicates_steps.rs`, and
      `givn/changes/spec-duplicates/tasks.md` (including the pending Scenario 12
      hash update). No design artifact or later `@wip` tag was changed.
  - COMMIT:
    - One atomic RED+GREEN+REFACTOR commit.
    - Required commit message: `feat(spec-duplicates): The audit reports retrieval unavailable without the retrieval feature`
    - Commit hash: `cfb16092296a20a298f875ca63a540fa88b596da`

## Scenario 14: `The audit reports an unavailable retrieval model`

- [x] Scenario: `The audit reports an unavailable retrieval model`
  - RED:
    - Remove `@wip` only from this scenario and leave all other tags unchanged.
    - Reuse the existing regular bindings for the duplicate/related corpus,
      unavailable retrieval model, subprocess invocation, non-zero exit, and
      stderr assertion. The reused corpus binding was temporarily replaced by
      an explicit `unimplemented!()` body for the strict RED proof, then
      restored before GREEN.
    - Target only this scenario:
      `GIVN_FEATURES=givn/changes/spec-duplicates/specs/spec-duplicates/spec-duplicates.feature GIVN_SPEC_EMBEDDER=stub cargo test --features spec-retrieval --test features_runner -- --name '^The audit reports an unavailable retrieval model$'`
    - Required result: non-zero exit. Paste exact runner output:
      ```text
   Compiling givn v0.2.0 (/home/buster/projects/givn)
warning: constant `SEMANTIC_ACCOUNT_FEATURE` is never used
  --> tests/steps/spec_duplicates_steps.rs:60:7
   |
60 | const SEMANTIC_ACCOUNT_FEATURE: &str = r#"Feature: account
   |       ^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: `#[warn(dead_code)]` on by default

warning: constant `SEMANTIC_SESSION_FEATURE` is never used
  --> tests/steps/spec_duplicates_steps.rs:67:7
   |
67 | const SEMANTIC_SESSION_FEATURE: &str = r#"Feature: session
   |       ^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: `#[warn(dead_code)]` on by default

warning: `givn` (test "features_runner") generated 2 warnings
    Finished `test` profile [unoptimized + debuginfo] target(s) in 2.47s
     Running tests/features_runner.rs (target/debug/deps/features_runner-c64f1ee623f3f628)
Feature: Audit duplicate and related permanent scenarios
  Scenario: The audit reports an unavailable retrieval model
   ✘  Given a permanent scenario corpus with duplicate and related-scenario candidates
      Step failed:
      Defined: givn/changes/spec-duplicates/specs/spec-duplicates/spec-duplicates.feature:150:5
      Matched: tests/steps/spec_duplicates_steps.rs:626:1
      Step panicked. Captured output: not implemented
[Summary]
1 feature
1 scenario (1 failed)
1 step (1 failed)

thread 'main' (3197068) panicked at /home/buster/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cucumber-0.23.0/src/cucumber.rs:1243:13:
1 step failed
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
error: test failed, to rerun `cargo test --test features_runner`
      ```
  - GREEN:
    - Reuse the real permanent corpus from the existing duplicate/related
      fixture: it contains exact, structural, and unresolved semantic pairs.
      The existing unavailable-model setup sets the subprocess environment to
      `GIVN_SPEC_EMBEDDER=missing`; the generic subprocess binding passes that
      value to the compiled `givn spec duplicates` process. The audit therefore
      requires semantic embedding when deterministic classification leaves an
      unresolved pair, and propagates the model error without converting it to
      success. The primary assertions are the non-zero exit and stderr wording
      `retrieval model is unavailable`; deterministic score overrides remain
      isolated to the stub embedder path.
    - Production file modified in GREEN: `src/spec/retrieval.rs`.
    - Existing test bindings reused; no final step-definition file changed.
    - Compile first: `cargo check --features spec-retrieval` must exit 0.
    - Compile output:
      ```text
    Checking givn v0.2.0 (/home/buster/projects/givn)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.97s
      ```
    - Rerun the targeted command. Required result: exit 0 for the scenario
      while the inner compiled `givn` subprocess exits non-zero with the model
      error.
    - Runner output evidence, paste exact GREEN output:
      ```text
   Compiling givn v0.2.0 (/home/buster/projects/givn)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 3.77s
     Running tests/features_runner.rs (target/debug/deps/features_runner-c64f1ee623f3f628)
Feature: Audit duplicate and related permanent scenarios
  Scenario: The audit reports an unavailable retrieval model
   ✔  Given a permanent scenario corpus with duplicate and related-scenario candidates
   ✔  And the retrieval model is unavailable
   ✔  When the user runs `givn spec duplicates`
   ✔  Then the command exits non-zero
   ✔  And stderr contains "retrieval model is unavailable"
[Summary]
1 feature
1 scenario (1 passed)
5 steps (5 passed)
      ```
    - Coverage: the targeted runner output contained no coverage summary;
      production coverage is unmeasured for this scenario.
  - REFACTOR:
    - Reuse the existing unavailable-model setup and keep the subprocess exit
      and stderr assertions primary. The audit propagates the original
      `embed_texts` error; the stub embedder and duplicate score seam remain
      unchanged.
    - `cargo fmt -- --check` exited 0 with empty stdout and stderr.
    - Stub scan of `tests/steps/spec_duplicates_steps.rs` and
      `tests/steps/spec_retrieval_steps.rs`: `unimplemented!()`/`todo!()` —
      `No files found`.
    - Empty-function-body scan: `No files found`.
    - Bare-return-body scan: `No files found`.
    - Required result: exit 0; paste exact REFACTOR output:
      ```text
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.14s
     Running tests/features_runner.rs (target/debug/deps/features_runner-c64f1ee623f3f628)
Feature: Audit duplicate and related permanent scenarios
  Scenario: The audit reports an unavailable retrieval model
   ✔  Given a permanent scenario corpus with duplicate and related-scenario candidates
   ✔  And the retrieval model is unavailable
   ✔  When the user runs `givn spec duplicates`
   ✔  Then the command exits non-zero
   ✔  And stderr contains "retrieval model is unavailable"
[Summary]
1 feature
1 scenario (1 passed)
5 steps (5 passed)
      ```
    - Files touched: `givn/changes/spec-duplicates/specs/spec-duplicates/spec-duplicates.feature` (Scenario 14 tag only), `src/spec/retrieval.rs`, and `givn/changes/spec-duplicates/tasks.md`. Existing regular bindings in `tests/steps/spec_duplicates_steps.rs`, `tests/steps/spec_retrieval_steps.rs`, and `tests/steps/mod.rs` were reused without a final diff. No design artifact or later `@wip` tag was changed.
  - COMMIT:
    - One atomic RED+GREEN+REFACTOR commit.
    - Required commit message: `feat(spec-duplicates): The audit reports an unavailable retrieval model`
    - Commit hash: `4a35cc10e58dc59826ff471f043f10e14a92d803`

## Scenario 15: `A malformed permanent feature reports its path`

- [x] Scenario: `A malformed permanent feature reports its path`
  - RED:
    - Remove `@wip` only from this scenario. Leave all other @wip tags intact.
    - Add regular bindings with explicit `unimplemented!()` in
      `tests/steps/spec_duplicates_steps.rs`.
    - Target only this scenario:
      `GIVN_FEATURES=givn/changes/spec-duplicates/specs/spec-duplicates/spec-duplicates.feature GIVN_SPEC_EMBEDDER=stub cargo test --features spec-retrieval --test features_runner -- --name '^A malformed permanent feature reports its path$'`
    - Required result: non-zero exit. Paste exact runner output:
      ```text
   Compiling givn v0.2.0 (/home/buster/projects/givn)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 3.42s
     Running tests/features_runner.rs (target/debug/deps/features_runner-c64f1ee623f3f628)
Feature: Audit duplicate and related permanent scenarios
  Scenario: A malformed permanent feature reports its path
   ✘  Given a malformed permanent feature at "givn/specs/broken/broken.feature"
      Step failed:
      Defined: givn/changes/spec-duplicates/specs/spec-duplicates/spec-duplicates.feature:158:5
      Matched: tests/steps/spec_duplicates_steps.rs:99:1
      Step panicked. Captured output: not implemented
[Summary]
1 feature
1 scenario (1 failed)
1 step (1 failed)

thread 'main' (3200447) panicked at /home/buster/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cucumber-0.23.0/src/cucumber.rs:1243:13:
1 step failed
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
error: test failed, to rerun `cargo test --test features_runner`
      ```
  - GREEN:
    - Abort on malformed permanent Gherkin, include the project-relative path
      `givn/specs/broken/broken.feature` in stderr, emit no report on stdout,
      and preserve the no-partial-report rule.
    - Production files expected in GREEN: `src/spec/retrieval.rs`.
    - Test file expected: `tests/steps/spec_duplicates_steps.rs`.
    - Compile first: `cargo check --features spec-retrieval` must exit 0.
    - Compile output:
      ```text
    Checking givn v0.2.0 (/home/buster/projects/givn)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.10s
      ```
    - Rerun the targeted command. Required result: exit 0 for the scenario's
      assertions while the inner `givn` command exits non-zero.
    - Runner output evidence, paste exact GREEN output:
      ```text
   Compiling givn v0.2.0 (/home/buster/projects/givn)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 3.77s
     Running tests/features_runner.rs (target/debug/deps/features_runner-c64f1ee623f3f628)
Feature: Audit duplicate and related permanent scenarios
  Scenario: A malformed permanent feature reports its path
   ✔  Given a malformed permanent feature at "givn/specs/broken/broken.feature"
   ✔  When the user runs `givn spec duplicates`
   ✔  Then the command exits non-zero
   ✔  And stderr contains the project-relative path "givn/specs/broken/broken.feature"
   ✔  And stdout is empty
[Summary]
1 feature
1 scenario (1 passed)
5 steps (5 passed)
      ```
    - Coverage: the targeted runner output contained no coverage summary;
      production coverage is unmeasured for this scenario.
  - REFACTOR:
    - Centralize permanent-feature parsing through explicit project-relative
      path normalization before attaching parse-error context. This keeps all
      permanent parse-error branches deterministic and leaves stdout untouched
      until collection succeeds.
    - `cargo fmt -- --check` exited 0 with empty stdout and stderr.
    - Stub scan of `tests/steps/spec_duplicates_steps.rs` for
      `unimplemented!()`/`todo!()`: `No files found`.
    - Empty-function-body scan of `tests/steps/spec_duplicates_steps.rs`:
      `No files found`.
    - Required result: exit 0; paste exact REFACTOR output:
      ```text
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.14s
     Running tests/features_runner.rs (target/debug/deps/features_runner-c64f1ee623f3f628)
Feature: Audit duplicate and related permanent scenarios
  Scenario: A malformed permanent feature reports its path
   ✔  Given a malformed permanent feature at "givn/specs/broken/broken.feature"
   ✔  When the user runs `givn spec duplicates`
   ✔  Then the command exits non-zero
   ✔  And stderr contains the project-relative path "givn/specs/broken/broken.feature"
   ✔  And stdout is empty
[Summary]
1 feature
1 scenario (1 passed)
5 steps (5 passed)
      ```
    - Files touched: `givn/changes/spec-duplicates/specs/spec-duplicates/spec-duplicates.feature` (Scenario 15 tag only), `src/spec/retrieval.rs`, `tests/steps/spec_duplicates_steps.rs`, and `givn/changes/spec-duplicates/tasks.md` (including the pending Scenario 14 hash update). No design artifact or later `@wip` tag was changed.
  - COMMIT:
    - One atomic RED+GREEN+REFACTOR commit.
    - Required commit message: `feat(spec-duplicates): A malformed permanent feature reports its path`
    - Commit hash: `28730695f430a657930231cc0875ae4cc75d9ab1`

## Scenario 16: `An unreadable permanent feature reports its path`

- [x] Scenario: `An unreadable permanent feature reports its path`
  - RED:
    - Remove `@wip` only from this scenario; do not alter tags elsewhere.
    - Add regular step definitions with `unimplemented!()` bodies in
      `tests/steps/spec_duplicates_steps.rs`.
    - Target only this scenario:
      `GIVN_FEATURES=givn/changes/spec-duplicates/specs/spec-duplicates/spec-duplicates.feature GIVN_SPEC_EMBEDDER=stub cargo test --features spec-retrieval --test features_runner -- --name '^An unreadable permanent feature reports its path$'`
    - Required result: non-zero exit. Paste exact runner output:
      ```text
   Compiling givn v0.2.0 (/home/buster/projects/givn)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 3.39s
     Running tests/features_runner.rs (target/debug/deps/features_runner-c64f1ee623f3f628)
Feature: Audit duplicate and related permanent scenarios
  Scenario: An unreadable permanent feature reports its path
   ✘  Given an unreadable permanent feature at "givn/specs/private/private.feature"
      Step failed:
      Defined: givn/changes/spec-duplicates/specs/spec-duplicates/spec-duplicates.feature:166:5
      Matched: tests/steps/spec_duplicates_steps.rs:107:1
      Step panicked. Captured output: not implemented
[Summary]
1 feature
1 scenario (1 failed)
1 step (1 failed)

thread 'main' (3203367) panicked at /home/buster/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cucumber-0.23.0/src/cucumber.rs:1243:13:
1 step failed
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
error: test failed, to rerun pass `--test features_runner`
      ```
  - GREEN:
    - Write the valid feature fixture at exactly
      `givn/specs/private/private.feature`, apply mode `000`, and probe
      readability as the same user that launches the compiled subprocess. In
      this environment the permission is enforced, so `Feature::parse_path`
      receives a real permission-denied I/O error. If an elevated test user can
      bypass mode `000`, the guard moves the valid file aside and creates a
      directory at the same `.feature` path; `Feature::parse_path` then receives
      a deterministic `EISDIR` I/O error rather than a malformed Gherkin input.
      The RAII guard restores the original file and permissions on scenario
      completion or assertion panic.
    - Abort on the unreadable permanent file, include the project-relative path
      `givn/specs/private/private.feature` in stderr, and emit no stdout report.
    - Production files modified in GREEN: none. The existing
      `src/spec/retrieval.rs` `parse_permanent_feature` boundary centralized by
      Scenario 15 was exercised for the real `Feature::parse_path` I/O error.
    - Test files modified in GREEN: `tests/steps/spec_duplicates_steps.rs` and
      `tests/steps/mod.rs`.
    - Compile first: `cargo check --features spec-retrieval` must exit 0.
    - Compile output:
      ```text
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.15s
      ```
    - Rerun the targeted command. Required result: exit 0 for the scenario while
      the inner command exits non-zero.
    - Runner output evidence, paste exact GREEN output:
      ```text
   Compiling givn v0.2.0 (/home/buster/projects/givn)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 6.25s
     Running tests/features_runner.rs (target/debug/deps/features_runner-c64f1ee623f3f628)
Feature: Audit duplicate and related permanent scenarios
  Scenario: An unreadable permanent feature reports its path
   ✔  Given an unreadable permanent feature at "givn/specs/private/private.feature"
   ✔  When the user runs `givn spec duplicates`
   ✔  Then the command exits non-zero
   ✔  And stderr contains the project-relative path "givn/specs/private/private.feature"
   ✔  And stdout is empty
[Summary]
1 feature
1 scenario (1 passed)
5 steps (5 passed)
      ```
    - Coverage: the targeted runner output contained no coverage summary;
      production coverage is unmeasured for this scenario.
  - REFACTOR:
    - Keep the permission guard path-specific and restore the fixture explicitly
      after the final stdout assertion, while retaining drop-based cleanup if a
      prior assertion panics. The fallback remains a real `Feature::parse_path`
      I/O failure, not a malformed parse case.
    - `cargo fmt -- --check` exited 0 with empty stdout and stderr.
    - Stub scan of `tests/steps/spec_duplicates_steps.rs` for
      `unimplemented!()`/`todo!()`: `No files found`.
    - Empty-function-body scan of `tests/steps/spec_duplicates_steps.rs` and
      `tests/steps/mod.rs`: `No files found`.
    - Required result: exit 0; paste exact REFACTOR output:
      ```text
   Compiling givn v0.2.0 (/home/buster/projects/givn)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 3.24s
     Running tests/features_runner.rs (target/debug/deps/features_runner-c64f1ee623f3f628)
Feature: Audit duplicate and related permanent scenarios
  Scenario: An unreadable permanent feature reports its path
   ✔  Given an unreadable permanent feature at "givn/specs/private/private.feature"
   ✔  When the user runs `givn spec duplicates`
   ✔  Then the command exits non-zero
   ✔  And stderr contains the project-relative path "givn/specs/private/private.feature"
   ✔  And stdout is empty
[Summary]
1 feature
1 scenario (1 passed)
5 steps (5 passed)
      ```
    - Files touched: `givn/changes/spec-duplicates/specs/spec-duplicates/spec-duplicates.feature` (Scenario 16 tag only), `tests/steps/spec_duplicates_steps.rs`, `tests/steps/mod.rs`, and `givn/changes/spec-duplicates/tasks.md` (including the pending Scenario 15 hash update). No production source file or design artifact changed, and all later `@wip` tags remain intact.
  - COMMIT:
    - One atomic RED+GREEN+REFACTOR commit.
    - Required commit message: `feat(spec-duplicates): An unreadable permanent feature reports its path`
    - Commit hash: `673e5c9457a6e443e121744284169e6aca36cd54`

## Scenario 17: `A missing permanent specifications directory is an empty corpus`

- [x] Scenario: `A missing permanent specifications directory is an empty corpus`
  - RED:
    - Remove `@wip` only from this scenario and leave all other @wip tags.
    - Add regular step bindings with explicit `unimplemented!()` bodies in
      `tests/steps/spec_duplicates_steps.rs`.
    - Target only this scenario:
      `GIVN_FEATURES=givn/changes/spec-duplicates/specs/spec-duplicates/spec-duplicates.feature GIVN_SPEC_EMBEDDER=stub cargo test --features spec-retrieval --test features_runner -- --name '^A missing permanent specifications directory is an empty corpus$'`
    - Required result: non-zero exit. Paste exact runner output:
      ```text
   Compiling givn v0.2.0 (/home/buster/projects/givn)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 3.44s
     Running tests/features_runner.rs (target/debug/deps/features_runner-c64f1ee623f3f628)
Feature: Audit duplicate and related permanent scenarios
  Scenario: A missing permanent specifications directory is an empty corpus
   ✘  Given a project without a `givn/specs/` directory
      Step failed:
      Defined: givn/changes/spec-duplicates/specs/spec-duplicates/spec-duplicates.feature:174:5
      Matched: tests/steps/spec_duplicates_steps.rs:190:1
      Step panicked. Captured output: not implemented
[Summary]
1 feature
1 scenario (1 failed)
1 step (1 failed)

thread 'main' (3207109) panicked at /home/buster/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cucumber-0.23.0/src/cucumber.rs:1243:13:
1 step failed
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
error: test failed, to rerun pass `--test features_runner`
      ```
  - GREEN:
    - Treat a missing `givn/specs/` directory as a successful empty corpus,
      render zero counts and an empty candidates array, and leave the persistent
      index absent.
    - Production files expected in GREEN: `src/spec/retrieval.rs`.
    - Test file expected: `tests/steps/spec_duplicates_steps.rs`.
    - Compile first: `cargo check --features spec-retrieval` must exit 0.
    - Compile output:
      ```text
    Checking givn v0.2.0 (/home/buster/projects/givn)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.98s
      ```
    - Rerun the targeted command. Required result: exit 0.
    - Runner output evidence, paste exact GREEN output:
      ```text
   Compiling givn v0.2.0 (/home/buster/projects/givn)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 3.90s
     Running tests/features_runner.rs (target/debug/deps/features_runner-c64f1ee623f3f628)
Feature: Audit duplicate and related permanent scenarios
  Scenario: A missing permanent specifications directory is an empty corpus
   ✔  Given a project without a `givn/specs/` directory
   ✔  When the user runs `givn spec duplicates --format json`
   ✔  Then the command exits 0
   ✔  And the JSON report has scanned_scenarios 0, candidate_count 0, and an empty candidates array
   ✔  And the persistent retrieval index remains absent
[Summary]
1 feature
1 scenario (1 passed)
5 steps (5 passed)
      ```
    - Coverage: the targeted runner output contained no coverage summary;
      production coverage is unmeasured for this scenario.
  - REFACTOR:
    - Keep missing-directory handling distinct from malformed-file errors and
      ensure no index is created as a retrieval side effect. Run
      `cargo fmt --check` and rerun the targeted command.
    - `cargo fmt -- --check` exited 0 with empty stdout and stderr.
    - Stub scan of `tests/steps/spec_duplicates_steps.rs` for
      `unimplemented!()`/`todo!()`: `No files found`.
    - Empty-function-body scan of `tests/steps/spec_duplicates_steps.rs`:
      `No files found`.
    - Required result: exit 0; paste exact REFACTOR output:
      ```text
   Compiling givn v0.2.0 (/home/buster/projects/givn)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 2.70s
     Running tests/features_runner.rs (target/debug/deps/features_runner-c64f1ee623f3f628)
Feature: Audit duplicate and related permanent scenarios
  Scenario: A missing permanent specifications directory is an empty corpus
   ✔  Given a project without a `givn/specs/` directory
   ✔  When the user runs `givn spec duplicates --format json`
   ✔  Then the command exits 0
   ✔  And the JSON report has scanned_scenarios 0, candidate_count 0, and an empty candidates array
   ✔  And the persistent retrieval index remains absent
[Summary]
1 feature
1 scenario (1 passed)
5 steps (5 passed)
      ```
    - Files touched: `givn/changes/spec-duplicates/specs/spec-duplicates/spec-duplicates.feature` (Scenario 17 tag only), `src/spec/retrieval.rs`, `tests/steps/spec_duplicates_steps.rs`, and `givn/changes/spec-duplicates/tasks.md` (including the pending Scenario 16 hash update). No design artifact or later `@wip` tag was changed.
  - COMMIT:
    - One atomic RED+GREEN+REFACTOR commit.
    - Required commit message: `feat(spec-duplicates): A missing permanent specifications directory is an empty corpus`
    - Commit hash: `5df4f7cede8dc86b71b453a82fb5c4b8a8c882ad`

## Scenario 18: `Archived scenarios are excluded from the audit`

- [x] Scenario: `Archived scenarios are excluded from the audit`
  - RED:
    - Remove `@wip` only from this scenario; leave all other tags unchanged.
    - Add regular bindings with explicit `unimplemented!()` bodies in
      `tests/steps/spec_duplicates_steps.rs`.
    - Target only this scenario:
      `GIVN_FEATURES=givn/changes/spec-duplicates/specs/spec-duplicates/spec-duplicates.feature GIVN_SPEC_EMBEDDER=stub cargo test --features spec-retrieval --test features_runner -- --name '^Archived scenarios are excluded from the audit$'`
    - Required result: non-zero exit. Paste exact runner output:
      ```text
   Compiling givn v0.2.0 (/home/buster/projects/givn)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 3.33s
     Running tests/features_runner.rs (target/debug/deps/features_runner-c64f1ee623f3f628)
Feature: Audit duplicate and related permanent scenarios
  Scenario: Archived scenarios are excluded from the audit
   ✘  Given a permanent corpus with one candidate
      Step failed:
      Defined: givn/changes/spec-duplicates/specs/spec-duplicates/spec-duplicates.feature:182:5
      Matched: tests/steps/spec_duplicates_steps.rs:713:1
      Step panicked. Captured output: not implemented
[Summary]
1 feature
1 scenario (1 failed)
1 step (1 failed)

thread 'main' (3211136) panicked at /home/buster/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cucumber-0.23.0/src/cucumber.rs:1243:13:
1 step failed
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
error: test failed, to rerun pass `--test features_runner`

      ```
  - GREEN:
    - Create a permanent corpus with one exact candidate, add the matching
      archive fixture at `givn/archive/old-change/specs/old/old.feature`, and
      compare parsed JSON candidate counts before and after archive creation.
      Assert every parsed endpoint is rooted at `givn/specs/` and no parsed
      endpoint identity contains `givn/archive/old-change/`.
    - Production source changed: none. `src/spec/retrieval.rs` already collects
      duplicate features exclusively from `root/givn/specs`, so the permanent
      boundary did not leak archive data.
    - Test file changed: `tests/steps/spec_duplicates_steps.rs`.
    - Compile first: `cargo check --features spec-retrieval` must exit 0.
    - Compile output:
      ```text
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.13s
      ```
    - Rerun the targeted command. Required result: exit 0.
    - Runner output evidence, paste exact GREEN output:
      ```text
   Compiling givn v0.2.0 (/home/buster/projects/givn)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 2.83s
     Running tests/features_runner.rs (target/debug/deps/features_runner-c64f1ee623f3f628)
Feature: Audit duplicate and related permanent scenarios
  Scenario: Archived scenarios are excluded from the audit
   ✔  Given a permanent corpus with one candidate
   ✔  And the archive contains a matching scenario at "givn/archive/old-change/specs/old/old.feature"
   ✔  When the user runs `givn spec duplicates --format json`
   ✔  Then every JSON candidate endpoint file starts with `givn/specs/`
   ✔  And no JSON candidate identity contains `givn/archive/old-change/`
   ✔  And the archived matching scenario does not change candidate_count
[Summary]
1 feature
1 scenario (1 passed)
6 steps (6 passed)

      ```
    - Coverage: the targeted runner output contained no coverage summary; no
      production source file changed, so production coverage is unmeasured for
      this scenario.
  - REFACTOR:
    - Keep the archive path and matching fixture explicit. The baseline and
      post-archive JSON reports are parsed independently through the real
      subprocess boundary; the archive is not deleted or mutated by the audit.
      Run `cargo fmt -- --check` and rerun the targeted command.
    - `cargo fmt -- --check` exited 0 with empty stdout and stderr.
    - Stub and empty-function-body scan output:
      ```text
No unimplemented!()/todo!() stubs found in tests/steps/spec_duplicates_steps.rs
No empty function bodies found in tests/steps/spec_duplicates_steps.rs
      ```
    - Required result: exit 0; paste exact REFACTOR output:
      ```text
   Compiling givn v0.2.0 (/home/buster/projects/givn)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 2.57s
     Running tests/features_runner.rs (target/debug/deps/features_runner-c64f1ee623f3f628)
Feature: Audit duplicate and related permanent scenarios
  Scenario: Archived scenarios are excluded from the audit
   ✔  Given a permanent corpus with one candidate
   ✔  And the archive contains a matching scenario at "givn/archive/old-change/specs/old/old.feature"
   ✔  When the user runs `givn spec duplicates --format json`
   ✔  Then every JSON candidate endpoint file starts with `givn/specs/`
   ✔  And no JSON candidate identity contains `givn/archive/old-change/`
   ✔  And the archived matching scenario does not change candidate_count
[Summary]
1 feature
1 scenario (1 passed)
6 steps (6 passed)

      ```
    - Files touched: `givn/changes/spec-duplicates/specs/spec-duplicates/spec-duplicates.feature` (Scenario 18 tag only), `tests/steps/spec_duplicates_steps.rs`, and `givn/changes/spec-duplicates/tasks.md` (including the pending Scenario 17 hash update). No `src/spec/retrieval.rs`, `design.md`, or later `@wip` tag changed.
  - COMMIT:
    - One atomic RED+GREEN+REFACTOR commit.
    - Required commit message: `feat(spec-duplicates): Archived scenarios are excluded from the audit`
    - Commit hash: `1703be4cc9ad934843c1830b0b7c5464644556a7`

## Scenario 19: `Auditing leaves active changes and archive contents unchanged`

- [x] Scenario: `Auditing leaves active changes and archive contents unchanged`
  - RED:
    - Remove `@wip` only from this scenario. Do not modify other scenario tags.
    - Add regular fixture and snapshot bindings with non-empty `unimplemented!()`
      bodies in `tests/steps/spec_duplicates_steps.rs`.
    - Target only this scenario:
      `GIVN_FEATURES=givn/changes/spec-duplicates/specs/spec-duplicates/spec-duplicates.feature GIVN_SPEC_EMBEDDER=stub cargo test --features spec-retrieval --test features_runner -- --name '^Auditing leaves active changes and archive contents unchanged$'`
    - Required result: non-zero exit. Paste exact runner output:
      ```text
   Compiling givn v0.2.0 (/home/buster/projects/givn)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 2.53s
     Running tests/features_runner.rs (target/debug/deps/features_runner-c64f1ee623f3f628)
Feature: Audit duplicate and related permanent scenarios
  Scenario: Auditing leaves active changes and archive contents unchanged
   ✘  Given a permanent corpus with duplicate and related-scenario candidates
      Step failed:
      Defined: givn/changes/spec-duplicates/specs/spec-duplicates/spec-duplicates.feature:191:5
      Matched: tests/steps/spec_duplicates_steps.rs:945:1
      Step panicked. Captured output: not implemented
[Summary]
1 feature
1 scenario (1 failed)
1 step (1 failed)

thread 'main' (3214303) panicked at /home/buster/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cucumber-0.23.0/src/cucumber.rs:1243:13:
1 step failed
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
error: test failed, to rerun pass `--test features_runner`
      ```
  - GREEN:
    - Create a permanent duplicate/related candidate corpus, active-change
      files under `givn/changes/audit-boundary/`, and archive files under
      `givn/archive/audit-boundary/`. Snapshot every byte in each fixture tree
      before the real subprocess, run the permanent-only audit, assert its
      successful report and candidate counts, then assert both trees are
      byte-for-byte unchanged.
    - Production files modified in GREEN: none. `src/spec/retrieval.rs` was
      unchanged because the existing audit already reads only `givn/specs/`.
    - Test files modified: `tests/steps/mod.rs` and
      `tests/steps/spec_duplicates_steps.rs`.
    - Compile first: `cargo check --features spec-retrieval` must exit 0.
    - Compile output:
      ```text
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.13s
      ```
    - Rerun the targeted command. Required result: exit 0.
    - Runner output evidence, paste exact GREEN output:
      ```text
   Compiling givn v0.2.0 (/home/buster/projects/givn)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 6.39s
     Running tests/features_runner.rs (target/debug/deps/features_runner-c64f1ee623f3f628)
Feature: Audit duplicate and related permanent scenarios
  Scenario: Auditing leaves active changes and archive contents unchanged
   ✔  Given a permanent corpus with duplicate and related-scenario candidates
   ✔  And active change files and archive contents are snapshotted
   ✔  When the user runs `givn spec duplicates`
   ✔  Then active change files are unchanged byte-for-byte
   ✔  And archive contents are unchanged byte-for-byte
[Summary]
1 feature
1 scenario (1 passed)
5 steps (5 passed)
      ```
    - Coverage: the targeted runner output contained no coverage summary; no
      production source file changed, so production coverage is unmeasured for
      this scenario.
  - REFACTOR:
    - Factor deterministic recursive snapshots for the scoped active and
      archive fixture roots, retain project-relative paths in the snapshot,
      sort traversal and entries deterministically, and keep active/archive
      assertions independent. Preserve the permitted model-cache side effect
      boundary. Run `cargo fmt -- --check` and rerun the targeted command.
    - `cargo fmt -- --check` exited 0 with empty stdout and stderr.
    - Stub scan output:
      ```text
No files found
      ```
    - Empty-function-body scan output:
      ```text
No files found
      ```
    - Required result: exit 0; paste exact REFACTOR output:
      ```text
   Compiling givn v0.2.0 (/home/buster/projects/givn)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 2.63s
     Running tests/features_runner.rs (target/debug/deps/features_runner-c64f1ee623f3f628)
Feature: Audit duplicate and related permanent scenarios
  Scenario: Auditing leaves active changes and archive contents unchanged
   ✔  Given a permanent corpus with duplicate and related-scenario candidates
   ✔  And active change files and archive contents are snapshotted
   ✔  When the user runs `givn spec duplicates`
   ✔  Then active change files are unchanged byte-for-byte
   ✔  And archive contents are unchanged byte-for-byte
[Summary]
1 feature
1 scenario (1 passed)
5 steps (5 passed)
      ```
    - Files touched: `givn/changes/spec-duplicates/specs/spec-duplicates/spec-duplicates.feature` (Scenario 19 tag only), `tests/steps/mod.rs`, `tests/steps/spec_duplicates_steps.rs`, and `givn/changes/spec-duplicates/tasks.md` (including the pending Scenario 18 hash update). No `src/spec/retrieval.rs`, `design.md`, or later `@wip` tag changed.
  - COMMIT:
    - One atomic RED+GREEN+REFACTOR commit.
    - Required commit message: `feat(spec-duplicates): Auditing leaves active changes and archive contents unchanged`
    - Commit hash: `be0a0895c005656d011140d1c6a4a958d2f4c443`

## Scenario 20: `An absent persistent index is ignored and remains absent`

- [x] Scenario: `An absent persistent index is ignored and remains absent`
  - RED:
    - Remove `@wip` only from this scenario and leave all other @wip tags.
    - Add regular bindings with explicit `unimplemented!()` in
      `tests/steps/spec_duplicates_steps.rs`.
    - Target only this scenario:
      `GIVN_FEATURES=givn/changes/spec-duplicates/specs/spec-duplicates/spec-duplicates.feature GIVN_SPEC_EMBEDDER=stub cargo test --features spec-retrieval --test features_runner -- --name '^An absent persistent index is ignored and remains absent$'`
    - Required result: non-zero exit. Paste exact runner output:
      ```text
   Compiling givn v0.2.0 (/home/buster/projects/givn)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 2.53s
     Running tests/features_runner.rs (target/debug/deps/features_runner-c64f1ee623f3f628)
Feature: Audit duplicate and related permanent scenarios
  Scenario: An absent persistent index is ignored and remains absent
   ✔  Given a permanent scenario corpus with duplicate and related-scenario candidates
   ✘  And no persistent retrieval index exists
      Step failed:
      Defined: givn/changes/spec-duplicates/specs/spec-duplicates/spec-duplicates.feature:200:5
      Matched: tests/steps/spec_duplicates_steps.rs:1093:1
      Step panicked. Captured output: not implemented
[Summary]
1 feature
1 scenario (1 failed)
2 steps (1 passed, 1 failed)

thread 'main' (3217155) panicked at /home/buster/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cucumber-0.23.0/src/cucumber.rs:1243:13:
1 step failed
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
error: test failed, to rerun pass `--test features_runner`
      ```
  - GREEN:
    - Create the existing duplicate/related permanent corpus, explicitly remove
      and verify the absence of `givn/.spec-index.json`, run the compiled
      `givn spec duplicates --format json` subprocess with the stub embedder,
      and assert the successful JSON report contains the exact, structural, and
      transient semantic candidates rooted in current `givn/specs/` files.
    - Production files modified in GREEN: none. `src/spec/retrieval.rs` was
      inspected and remains unchanged because `duplicates_enabled` collects
      current permanent files, embeds them in memory, and does not call
      `refresh_index` or read/write `.spec-index.json`.
    - Test file modified: `tests/steps/spec_duplicates_steps.rs`.
    - Compile first: `cargo check --features spec-retrieval` must exit 0.
    - Compile output:
      ```text
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.13s
      ```
    - Rerun the targeted command. Required result: exit 0.
    - Runner output evidence, paste exact GREEN output:
      ```text
   Compiling givn v0.2.0 (/home/buster/projects/givn)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 2.86s
     Running tests/features_runner.rs (target/debug/deps/features_runner-c64f1ee623f3f628)
Feature: Audit duplicate and related permanent scenarios
  Scenario: An absent persistent index is ignored and remains absent
   ✔  Given a permanent scenario corpus with duplicate and related-scenario candidates
   ✔  And no persistent retrieval index exists
   ✔  When the user runs `givn spec duplicates --format json`
   ✔  Then the command exits 0
   ✔  And the audit uses current permanent files and transient embeddings
   ✔  And the persistent retrieval index remains absent
[Summary]
1 feature
1 scenario (1 passed)
6 steps (6 passed)
      ```
    - Coverage: the targeted runner output contained no coverage summary; no
      production source file changed, so production coverage is unmeasured for
      this scenario.
  - REFACTOR:
    - Keep absent-index setup explicit and deterministic by removing any
      pre-existing `givn/.spec-index.json` and asserting it is absent before
      invocation. Assert the JSON candidate classes and permanent endpoint
      roots independently from the post-command absence check. Verify the
      audit path does not call the persistent index refresh path and does not
      create an index accidentally. Run `cargo fmt -- --check` and rerun the
      targeted command.
    - `cargo fmt -- --check` exited 0 with empty stdout and stderr.
    - Stub scan output:
      ```text
No files found
      ```
    - Empty-function-body scan output:
      ```text
No files found
      ```
    - Required result: exit 0; paste exact REFACTOR output:
      ```text
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.13s
     Running tests/features_runner.rs (target/debug/deps/features_runner-c64f1ee623f3f628)
Feature: Audit duplicate and related permanent scenarios
  Scenario: An absent persistent index is ignored and remains absent
   ✔  Given a permanent scenario corpus with duplicate and related-scenario candidates
   ✔  And no persistent retrieval index exists
   ✔  When the user runs `givn spec duplicates --format json`
   ✔  Then the command exits 0
   ✔  And the audit uses current permanent files and transient embeddings
   ✔  And the persistent retrieval index remains absent
[Summary]
1 feature
1 scenario (1 passed)
6 steps (6 passed)
      ```
    - Files touched: `givn/changes/spec-duplicates/specs/spec-duplicates/spec-duplicates.feature` (Scenario 20 tag only), `tests/steps/spec_duplicates_steps.rs`, and `givn/changes/spec-duplicates/tasks.md` (including the pending Scenario 19 hash update). No `src/spec/retrieval.rs`, `design.md`, or later `@wip` tag changed.
  - COMMIT:
    - One atomic RED+GREEN+REFACTOR commit.
    - Required commit message: `feat(spec-duplicates): An absent persistent index is ignored and remains absent`
    - Commit hash: `fbdc70554a0e2f20214da9c2dba9cf81bfdd09e8`

## Scenario 21: `A malformed persistent index is ignored and unchanged`

- [x] Scenario: `A malformed persistent index is ignored and unchanged`
  - RED:
    - Remove `@wip` only from this scenario. Leave all other scenario tags.
    - Add regular step definitions with explicit `unimplemented!()` bodies in
      `tests/steps/spec_duplicates_steps.rs`.
    - Target only this scenario:
      `GIVN_FEATURES=givn/changes/spec-duplicates/specs/spec-duplicates/spec-duplicates.feature GIVN_SPEC_EMBEDDER=stub cargo test --features spec-retrieval --test features_runner -- --name '^A malformed persistent index is ignored and unchanged$'`
    - Required result: non-zero exit. Paste exact runner output:
      ```text
   Compiling givn v0.2.0 (/home/buster/projects/givn)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 2.54s
     Running tests/features_runner.rs (target/debug/deps/features_runner-c64f1ee623f3f628)
Feature: Audit duplicate and related permanent scenarios
  Scenario: A malformed persistent index is ignored and unchanged
   ✔  Given a permanent scenario corpus with duplicate and related-scenario candidates
   ✘  And the persistent retrieval index contains the bytes "not-json"
      Step failed:
      Defined: givn/changes/spec-duplicates/specs/spec-duplicates/spec-duplicates.feature:209:5
      Matched: tests/steps/spec_duplicates_steps.rs:1109:1
      Step panicked. Captured output: not implemented
[Summary]
1 feature
1 scenario (1 failed)
2 steps (1 passed, 1 failed)

thread 'main' (3219600) panicked at /home/buster/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cucumber-0.23.0/src/cucumber.rs:1243:13:
1 step failed
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
error: test failed, to rerun pass `--test features_runner`
      ```
  - GREEN:
    - Write `not-json` to the persistent index, run the audit using current
      permanent files and transient embeddings, assert valid JSON candidates,
      and assert the index remains exactly `not-json`.
    - Production files modified in GREEN: none. `src/spec/retrieval.rs` was
      inspected and remains unchanged because `duplicates_enabled` collects
      current permanent files, embeds them in memory, and does not call
      `read_index` or `refresh_index` or read/write `.spec-index.json`.
    - Test file modified: `tests/steps/spec_duplicates_steps.rs`.
    - Compile first: `cargo check --features spec-retrieval` must exit 0.
    - Compile output:
      ```text
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.14s
      ```
    - Rerun the targeted command. Required result: exit 0.
    - Runner output evidence, paste exact GREEN output:
      ```text
   Compiling givn v0.2.0 (/home/buster/projects/givn)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 3.11s
     Running tests/features_runner.rs (target/debug/deps/features_runner-c64f1ee623f3f628)
Feature: Audit duplicate and related permanent scenarios
  Scenario: A malformed persistent index is ignored and unchanged
   ✔  Given a permanent scenario corpus with duplicate and related-scenario candidates
   ✔  And the persistent retrieval index contains the bytes "not-json"
   ✔  When the user runs `givn spec duplicates --format json`
   ✔  Then the command exits 0
   ✔  And the audit report is valid JSON with the permanent candidates
   ✔  And the persistent retrieval index still contains the bytes "not-json"
[Summary]
1 feature
1 scenario (1 passed)
6 steps (6 passed)
      ```
    - Coverage: the targeted runner output contained no coverage summary; no
      production source file changed, so production coverage is unmeasured for
      this scenario.
  - REFACTOR:
    - Keep the malformed index bytes outside the audit evidence path, snapshot
      exactly `not-json` before invocation, compare the post-command bytes to
      that snapshot and to the required literal, and reuse the existing
      permanent-candidate JSON assertion. The audit path does not call
      `read_index` or `refresh_index` and does not repair or rewrite the file.
      Run `cargo fmt -- --check` and rerun the targeted command.
    - `cargo fmt -- --check` exited 0 with empty stdout and stderr.
    - Stub scan output:
      ```text
No files found
      ```
    - Empty-function-body scan output:
      ```text
No files found
      ```
    - Required result: exit 0; paste exact REFACTOR output:
      ```text
   Compiling givn v0.2.0 (/home/buster/projects/givn)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 2.81s
     Running tests/features_runner.rs (target/debug/deps/features_runner-c64f1ee623f3f628)
Feature: Audit duplicate and related permanent scenarios
  Scenario: A malformed persistent index is ignored and unchanged
   ✔  Given a permanent scenario corpus with duplicate and related-scenario candidates
   ✔  And the persistent retrieval index contains the bytes "not-json"
   ✔  When the user runs `givn spec duplicates --format json`
   ✔  Then the command exits 0
   ✔  And the audit report is valid JSON with the permanent candidates
   ✔  And the persistent retrieval index still contains the bytes "not-json"
[Summary]
1 feature
1 scenario (1 passed)
6 steps (6 passed)
      ```
    - Files touched: `givn/changes/spec-duplicates/specs/spec-duplicates/spec-duplicates.feature` (Scenario 21 tag only), `tests/steps/spec_duplicates_steps.rs`, and `givn/changes/spec-duplicates/tasks.md` (including the pending Scenario 20 hash update). No `src/spec/retrieval.rs`, `design.md`, or later `@wip` tag changed.
  - COMMIT:
    - One atomic RED+GREEN+REFACTOR commit.
    - Required commit message: `feat(spec-duplicates): A malformed persistent index is ignored and unchanged`
    - Commit hash: `65dcc06a81fad6369f292ab81042732fd9c02ae7`

## Scenario 22: `A semantic candidate at the inclusive threshold is reported`

- [x] Scenario: `A semantic candidate at the inclusive threshold is reported`
  - RED:
    - Remove `@wip` only from this scenario and leave all other tags intact.
    - Add regular bindings with explicit `unimplemented!()` bodies in
      `tests/steps/spec_duplicates_steps.rs`.
    - Target only this scenario:
      `GIVN_FEATURES=givn/changes/spec-duplicates/specs/spec-duplicates/spec-duplicates.feature GIVN_SPEC_EMBEDDER=stub cargo test --features spec-retrieval --test features_runner -- --name '^A semantic candidate at the inclusive threshold is reported$'`
    - Required result: non-zero exit. Paste exact runner output:
      ```text
   Compiling givn v0.2.0 (/home/buster/projects/givn)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 3.43s
     Running tests/features_runner.rs (target/debug/deps/features_runner-c64f1ee623f3f628)
Feature: Audit duplicate and related permanent scenarios
  Scenario: A semantic candidate at the inclusive threshold is reported
   ✘  Given permanent scenarios have no exact or structural relationship
      Step failed:
      Defined: givn/changes/spec-duplicates/specs/spec-duplicates/spec-duplicates.feature:217:5
      Matched: tests/steps/spec_duplicates_steps.rs:510:1
      Step panicked. Captured output: not implemented
[Summary]
1 feature
1 scenario (1 failed)
1 step (1 failed)

thread 'main' (3222838) panicked at /home/buster/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cucumber-0.23.0/src/cucumber.rs:1243:13:
1 step failed
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
error: test failed, to rerun `cargo test --test features_runner`
      ```
  - GREEN:
    - Set `GIVN_SPEC_DUPLICATES_TEST_SCORES` for the normalized endpoint pair to
      exactly `0.9500`, use the stub embedder only, and assert the semantic
      candidate is admitted with `semantic_similarity` and serialized score
      `0.9500`.
    - Production files modified in GREEN: none. The existing duplicate-audit
      path in `src/spec/retrieval.rs` already uses the normalized unordered-pair
      seam only with `GIVN_SPEC_EMBEDDER=stub`, compares the raw score
      inclusively, and formats admitted scores only during JSON rendering.
    - Test file modified: `tests/steps/spec_duplicates_steps.rs`.
    - Compile first: `cargo check --features spec-retrieval` exited 0.
    - Compile output:
      ```text
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.13s
      ```
    - Rerun the targeted command. Required result: exit 0.
    - Runner output evidence, paste exact GREEN output:
      ```text
   Compiling givn v0.2.0 (/home/buster/projects/givn)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 2.80s
     Running tests/features_runner.rs (target/debug/deps/features_runner-c64f1ee623f3f628)
Feature: Audit duplicate and related permanent scenarios
  Scenario: A semantic candidate at the inclusive threshold is reported
   ✔  Given permanent scenarios have no exact or structural relationship
   ✔  And the test score for their normalized endpoint pair is `0.9500`
   ✔  When the user runs `givn spec duplicates --format json`
   ✔  Then the command exits 0
   ✔  And candidate rank 1 is classified as `semantic_candidate`
   ✔  And candidate rank 1 has relation `semantic_similarity`
   ✔  And candidate rank 1 has score `0.9500`
[Summary]
1 feature
1 scenario (1 passed)
7 steps (7 passed)
      ```
    - Coverage: the targeted runner output contained no coverage summary;
      production coverage is unmeasured. The production path was already
      implemented and was not modified in this scenario.
  - REFACTOR:
    - Confirm thresholding uses the raw score before four-decimal
      serialization, remains inclusive at `0.9500`, and the score seam is not
      available to production local-E5 execution. Run `cargo fmt --check` and
      rerun the targeted command.
    - `cargo fmt -- --check` exited 0 with empty stdout and stderr.
    - Stub scan output:
      ```text
No files found
      ```
    - Empty-function-body scan output:
      ```text
No files found
      ```
    - The non-stub boundary probe exited non-zero with:
      ```text
error: retrieval model is unavailable; configure a local model cache or unset GIVN_SPEC_EMBEDDER
      ```
    - Required result: exit 0; paste exact REFACTOR output:
      ```text
   Compiling givn v0.2.0 (/home/buster/projects/givn)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 2.72s
     Running tests/features_runner.rs (target/debug/deps/features_runner-c64f1ee623f3f628)
Feature: Audit duplicate and related permanent scenarios
  Scenario: A semantic candidate at the inclusive threshold is reported
   ✔  Given permanent scenarios have no exact or structural relationship
   ✔  And the test score for their normalized endpoint pair is `0.9500`
   ✔  When the user runs `givn spec duplicates --format json`
   ✔  Then the command exits 0
   ✔  And candidate rank 1 is classified as `semantic_candidate`
   ✔  And candidate rank 1 has relation `semantic_similarity`
   ✔  And candidate rank 1 has score `0.9500`
[Summary]
1 feature
1 scenario (1 passed)
7 steps (7 passed)
      ```
    - Files touched: `givn/changes/spec-duplicates/specs/spec-duplicates/spec-duplicates.feature` (Scenario 22 tag only), `tests/steps/spec_duplicates_steps.rs`, and `givn/changes/spec-duplicates/tasks.md` (including the pending Scenario 21 hash update). No `design.md`, later `@wip` tag, or production source file changed.
  - COMMIT:
    - One atomic RED+GREEN+REFACTOR commit.
    - Required commit message: `feat(spec-duplicates): A semantic candidate at the inclusive threshold is reported`
    - Commit hash: `bc4e3c1d7fb6d6af46fb9bf5ac52f8c8c2984213`

## Scenario 23: `A semantic pair below the threshold is not reported`

- [x] Scenario: `A semantic pair below the threshold is not reported`
  - RED:
    - Remove `@wip` only from this scenario. Do not touch any other scenario's
      @wip tag.
    - Add regular step bindings with non-empty `unimplemented!()` bodies in
      `tests/steps/spec_duplicates_steps.rs`.
    - Target only this scenario:
      `GIVN_FEATURES=givn/changes/spec-duplicates/specs/spec-duplicates/spec-duplicates.feature GIVN_SPEC_EMBEDDER=stub cargo test --features spec-retrieval --test features_runner -- --name '^A semantic pair below the threshold is not reported$'`
    - Required result: non-zero exit. Paste exact runner output:
      ```text
   Compiling givn v0.2.0 (/home/buster/projects/givn)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 3.49s
     Running tests/features_runner.rs (target/debug/deps/features_runner-c64f1ee623f3f628)
Feature: Audit duplicate and related permanent scenarios
  Scenario: A semantic pair below the threshold is not reported
   ✔  Given permanent scenarios have no exact or structural relationship
   ✘  And the test score for their normalized endpoint pair is `0.9499`
      Step failed:
      Defined: givn/changes/spec-duplicates/specs/spec-duplicates/spec-duplicates.feature:228:5
      Matched: tests/steps/spec_duplicates_steps.rs:548:1
      Step panicked. Captured output: not implemented
[Summary]
1 feature
1 scenario (1 failed)
2 steps (1 passed, 1 failed)

thread 'main' (3226247) panicked at /home/buster/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cucumber-0.23.0/src/cucumber.rs:1243:13:
1 step failed
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
error: test failed, to rerun `cargo test --test features_runner`
      ```
  - GREEN:
    - Set the normalized endpoint pair score to exactly `0.9499` through the
      stub-only seam and exclude it before serialization; assert candidate count
      zero and no score `0.9499`.
    - Production files modified in GREEN: none. The existing duplicate-audit
      path in `src/spec/retrieval.rs` compares the raw score against the
      inclusive threshold before JSON rendering, so a below-threshold score is
      not rounded up into a candidate.
    - Test file modified: `tests/steps/spec_duplicates_steps.rs`.
    - Compile first: `cargo check --features spec-retrieval` exited 0.
    - Compile output:
      ```text
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.13s
      ```
    - Rerun the targeted command. Required result: exit 0.
    - Runner output evidence, paste exact GREEN output:
      ```text
   Compiling givn v0.2.0 (/home/buster/projects/givn)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 2.72s
     Running tests/features_runner.rs (target/debug/deps/features_runner-c64f1ee623f3f628)
Feature: Audit duplicate and related permanent scenarios
  Scenario: A semantic pair below the threshold is not reported
   ✔  Given permanent scenarios have no exact or structural relationship
   ✔  And the test score for their normalized endpoint pair is `0.9499`
   ✔  When the user runs `givn spec duplicates --format json`
   ✔  Then the command exits 0
   ✔  And the JSON report has candidate_count 0
   ✔  And no candidate has score `0.9499`
[Summary]
1 feature
1 scenario (1 passed)
6 steps (6 passed)
      ```
  - REFACTOR:
    - Check admission happens before four-decimal formatting and that excluded
      pairs do not receive ranks. Run `cargo fmt --check` and rerun the targeted
      command.
    - Shared the normalized threshold score override helper with Scenario 22;
      both `0.9500` and `0.9499` still use the same unordered endpoint pair.
    - `cargo fmt -- --check` exited 0 with empty stdout and stderr.
    - Stub scan output:
      ```text
No files found
      ```
    - Empty-function-body scan output:
      ```text
No files found
      ```
    - Required result: exit 0; paste exact REFACTOR output:
      ```text
   Compiling givn v0.2.0 (/home/buster/projects/givn)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 2.46s
     Running tests/features_runner.rs (target/debug/deps/features_runner-c64f1ee623f3f628)
Feature: Audit duplicate and related permanent scenarios
  Scenario: A semantic pair below the threshold is not reported
   ✔  Given permanent scenarios have no exact or structural relationship
   ✔  And the test score for their normalized endpoint pair is `0.9499`
   ✔  When the user runs `givn spec duplicates --format json`
   ✔  Then the command exits 0
   ✔  And the JSON report has candidate_count 0
   ✔  And no candidate has score `0.9499`
[Summary]
1 feature
1 scenario (1 passed)
6 steps (6 passed)
      ```
    - Files touched: `givn/changes/spec-duplicates/specs/spec-duplicates/spec-duplicates.feature` (Scenario 23 tag only), `tests/steps/spec_duplicates_steps.rs`, and `givn/changes/spec-duplicates/tasks.md` (including the pending Scenario 22 hash update). No `src/spec/retrieval.rs`, `design.md`, or later `@wip` tag changed.
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
