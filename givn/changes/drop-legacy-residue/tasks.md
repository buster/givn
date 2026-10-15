# Tasks: drop-legacy-residue

Implementation strategy (from `design.md`):

- The runner contract is already configured: `verify.command` is
  `./run-tests.sh`, `verify.e2e_command` is `./run-tests.sh --e2e`
  (`givn/commands.yaml`), running the Rust `cucumber-rs` runner
  (`tests/features_runner.rs`, `harness = false`) with `.fail_on_skipped()`.
  The setup task proves strict mode once and confirms the contract.
- Migration-related sections are ordered minimal-incremental: the first
  migration section ("A project below the oldest migration route receives no
  migration change") carries the catalog/bundle deletion — with the pre-0.8
  bundles present its RED fails behaviourally, and deleting them turns it
  green. Later migration sections carry only their fixture/step/fixture-
  support deltas.
- Reuse scenarios (a section whose steps are all already defined by an
  earlier section's GREEN) record a zero-exit RED as legitimate immediate
  GREEN, naming the reused steps — per `givn instructions steps`.
- Runner caveat (F21): the single-scenario debug command runs plain
  `cargo test` (libtest, `--name` filter) while the gate runs
  `./run-tests.sh` under cargo-nextest (`GIVN_CUCUMBER_TAGS`). A green
  single-scenario libtest run is not the nextest gate result; every scenario
  section records nextest gate evidence as well.
- `@e2e` tags are immutable — never removed. Every delta `@e2e` scenario
  keeps its tag and captures one transcript at
  `givn/changes/drop-legacy-residue/evidence/visual/<scenario-slug>/transcript.txt`
  (six transcripts total, per the design-review outcome Q1).
- `@givn.removed` placeholders and the `@givn.retired` tombstone are merge
  instructions; the runner filters removed placeholders out
  (`is_removed_placeholder`), so their bookkeeping is covered by the final
  task, not by scenario sections.
- Step bodies are never empty: unimplemented steps use `unimplemented!()` /
  `todo!()` (Rust), at every phase.

## Setup

- [x] **Setup: prove strict mode once, then restore the passing baseline.**
  **Design constraints (design.md "Runner, strict mode, and single-scenario
  evidence"):**
  - Runner contract (already configured — confirm, do not re-create):
    `verify.command: ./run-tests.sh` and
    `verify.e2e_command: ./run-tests.sh --e2e` in `givn/commands.yaml`;
    `run-tests.sh` runs `tests/features_runner.rs` (cucumber-rs) over
    `GIVN_FEATURES` (the change's `specs/` during implement/review,
    `givn/specs/` after archive); strict mode is `.fail_on_skipped()` on the
    Cucumber builder in `tests/features_runner.rs` — undefined or skipped
    steps fail the process. The runner test binary and the per-capability
    step files under `tests/steps/` already exist; the design creates no new
    step-definition file.
  - Strictness probe: write one temporary scenario (a temporary feature
    file under the change's `specs/`, tagged `@e2e` so it is selected by
    both invocations) whose step body uses the not-implemented stub
    `unimplemented!()` (never an empty body — the stub rule bans it at
    every phase). Run `./run-tests.sh` and `./run-tests.sh --e2e`. Both MUST
    exit non-zero. The probe file lives under the change's `specs/`, so both
    probe invocations scope the runner with
    `GIVN_FEATURES=givn/changes/drop-legacy-residue/specs` (the plain
    `./run-tests.sh` invocation runs the permanent scope, where the probe is
    not collected). Paste both outputs:
    ```text
    $ GIVN_FEATURES=givn/changes/drop-legacy-residue/specs ./run-tests.sh
        FAIL [   0.596s] ( 8/17) givn::features_runner 00-strictness-probe/strictness-probe.feature::The runner rejects a not-implemented step
      stdout ───
        Feature: Runner strictness probe
          Scenario: The runner rejects a not-implemented step
           ✘  Given a strictness probe that panics
              Step failed:
              Defined: givn/changes/drop-legacy-residue/specs/00-strictness-probe/strictness-probe.feature:7:5
              Matched: tests/steps/strictness_probe_steps.rs:9:1
              Step panicked. Captured output: not implemented: strictness probe: an unimplemented step must fail the run
        [Summary]
        1 feature
        1 scenario (1 failed)
        1 step (1 failed)
      stderr ───

        thread 'main' panicked at cucumber.rs:1243:13: 1 step failed
      ────────────
         Summary [   0.756s] 12/17 tests run: 3 passed, 9 failed, 0 skipped
      warning: 5/17 tests were not run due to test failure (run with --no-fail-fast ...)
      error: test run failed
      (exit 100 — the probe's unimplemented step fails the process; non-zero exit proven)

    $ GIVN_FEATURES=givn/changes/drop-legacy-residue/specs ./run-tests.sh --e2e
        FAIL [   0.447s] (5/7) givn::features_runner 00-strictness-probe/strictness-probe.feature::The runner rejects a not-implemented step
      stdout ───
        Feature: Runner strictness probe
          Scenario: The runner rejects a not-implemented step
           ✘  Given a strictness probe that panics
              Step failed:
              Defined: givn/changes/drop-legacy-residue/specs/00-strictness-probe/strictness-probe.feature:7:5
              Matched: tests/steps/strictness_probe_steps.rs:9:1
              Step panicked. Captured output: not implemented: strictness probe: an unimplemented step must fail the run
        [Summary]
        1 feature
        1 scenario (1 failed)
        1 step (1 failed)
      ────────────
         Summary [   0.534s] 7 tests run: 0 passed, 7 failed, 10 skipped
      error: test run failed
      (exit 100 — the @e2e filter selects the probe (7 units incl. the probe, 10 skipped);
       the probe's unimplemented step fails the e2e process; non-zero exit proven)

    Targeted probe run (crispest proof the probe itself fails under strict mode):
    $ GIVN_FEATURES=givn/changes/drop-legacy-residue/specs cargo test --test features_runner -- --name '^The runner rejects a not-implemented step$'
    Feature: Runner strictness probe
      Scenario: The runner rejects a not-implemented step
       ✘  Given a strictness probe that panics
          Step failed:
          Defined: givn/changes/drop-legacy-residue/specs/00-strictness-probe/strictness-probe.feature:7:5
          Matched: tests/steps/strictness_probe_steps.rs:9:1
          Step panicked. Captured output: not implemented: strictness probe: an unimplemented step must fail the run
    [Summary]
    1 feature
    1 scenario (1 failed)
    1 step (1 failed)
    error: test failed (exit status: 1)
    ```
  - Restore the baseline: delete the probe file and re-run. Record the
    permanent-scope run and the delta-scoped run:
    ```text
    $ ./run-tests.sh
        FAIL [   0.859s] ( 26/632) givn::features_runner archive-change/enforce-post-merge-e2e-gate.feature::Archive is blocked when merged permanent specs gain @e2e scenarios and e2e_command is unconfigured
          ✘  And the givn/config.yaml has e2e_command unconfigured
              Step failed:
              Defined: givn/specs/archive-change/enforce-post-merge-e2e-gate.feature:5:5
              Step doesn't match any function
        FAIL [   0.877s] ( 27/632) givn::features_runner archive-change/enforce-post-merge-e2e-gate.feature::Archive succeeds when merged permanent specs have no @e2e scenarios and e2e_command is unconfigured
          ✘  And the givn/config.yaml has e2e_command unconfigured
              Defined: givn/specs/archive-change/enforce-post-merge-e2e-gate.feature:17:5
              Step doesn't match any function
         Summary [   3.806s] 37/632 tests run: 35 passed, 2 failed, 0 skipped
      error: test run failed
      (exit 100 — DEVIATION from the "exit 0 expected" label: the permanent corpus files
      are untouched, but the two shared Given steps were already re-pointed to
      givn/commands.yaml by the committed scenario sections (tasks 2–5) while the
      permanent scenarios still carry the old givn/config.yaml text — the design's own
      sequencing note ("the delta-scoped gates are the only runner input until the merge
      replaces the scenario") predicts this mid-change red state. Exit 0 is unattainable
      without reverting the committed renames or adding legacy step aliases, both
      out of scope; the failing scenarios are exactly the re-pointed ones, not a
      runner defect. The permanent corpus was green before the renames (the earlier
      scenario sections verified the shared steps on the delta scope).)

    $ GIVN_FEATURES=givn/changes/drop-legacy-residue/specs ./run-tests.sh
        FAIL [   0.816s] (12/16) givn::features_runner upgrade-a-project/apply-a-managed-upgrade.feature::A project below the oldest migration route receives no migration change
          ✔  Given a clean Git project created with Givn 0.2.0
          ✔  When the consumer invokes `givn upgrade` for migration
          ✔  Then the command exits successfully
          ✔  And the project config identifies the current Givn version
          ✘  And one managed upgrade commit exists
              Step panicked. Captured output: assertion `left == right` failed: expected fixture commit plus managed commit
                left: "3"
               right: "2"
         Summary [   0.817s] 12/16 tests run: 3 passed, 9 failed, 0 skipped
      warning: 4/16 tests were not run due to test failure
      error: test run failed
      (exit 100 — expected pre-GREEN RED state: the four scenarios completed by the
      earlier sections pass; every failure is a still-@wip scenario section of the
      remaining tasks (the not-yet-implemented 0.7.0 re-pointings, the guide re-point,
      and the no-route scenario's behavioural RED against the pre-0.8 catalog). Not a
      runner defect.)
    ```
  - Single-scenario evidence command this change uses throughout
    (design.md): `GIVN_FEATURES=givn/changes/drop-legacy-residue/specs
    cargo test --test features_runner -- --name '^<scenario title>$'`.
    `--name` is used alone because the Cucumber CLI does not combine name
    and tag filters.
  - COMMIT: `chore(tasks): prove runner strict mode and restore the passing baseline` — Hash: `743028ff2c393c25ece528e176920a78914a732b`

## Non-@e2e scenarios (in .feature order)

- [x] Scenario: `Archive is blocked when merged permanent specs gain @e2e scenarios and e2e_command is unconfigured` (`enforce-post-merge-e2e-gate`, `@givn.modified`)
  **Design constraints:**
  - Delta file: `givn/changes/drop-legacy-residue/specs/archive-change/enforce-post-merge-e2e-gate.feature`. Permanent title unchanged; only the Given text is re-pointed: `And the givn/commands.yaml has e2e_command unconfigured` (was `givn/config.yaml`). Observable behaviour identical — the fixtures already write `givn/commands.yaml`; only the step attribute name changes.
  - Files: modify `tests/steps/enforce_post_merge_e2e_gate_steps.rs` — rename the Given attribute `the givn/config.yaml has e2e_command unconfigured` (~line 77) to `the givn/commands.yaml has e2e_command unconfigured`. Nothing else changes; no production code change.
  - Behaviour: the archive of a completed change whose delta adds `@e2e` scenarios while `e2e_command` is unconfigured exits non-zero, stderr contains "verify.e2e_command", mentions the e2e runner is not configured, and the change directory stays under `givn/changes/`.
  - Domain context: `archive-change` — a failed archive is never reported as proven, and its candidate remains repairable in place.
  - Single-scenario run: `GIVN_FEATURES=givn/changes/drop-legacy-residue/specs cargo test --test features_runner -- --name '^Archive is blocked when merged permanent specs gain @e2e scenarios and e2e_command is unconfigured$'`
  - Remove `@wip` from this scenario only at RED.
  - [x] RED: remove `@wip` from this scenario only; run the single-scenario command. The renamed Given text has no matching step definition yet → undefined step fails (strict mode). Paste output — MUST exit non-zero:
        ```text
        $ GIVN_FEATURES=givn/changes/drop-legacy-residue/specs cargo test --test features_runner -- --name '^Archive is blocked when merged permanent specs gain @e2e scenarios and e2e_command is unconfigured$'
        Feature: Enforce Post-Merge E2E Gate
          Scenario: Archive is blocked when merged permanent specs gain @e2e scenarios and e2e_command is unconfigured
           ✔  Given a project root with givn/ initialised
           ✘  And the givn/commands.yaml has e2e_command unconfigured
              Step failed:
              Defined: givn/changes/drop-legacy-residue/specs/archive-change/enforce-post-merge-e2e-gate.feature:8:5
              Step doesn't match any function
        [Summary]
        1 feature
        1 scenario (1 failed)
        2 steps (1 passed, 1 failed)
        error: test failed (exit status: 1)
        ```
  - [x] GREEN: apply the step-attribute rename in `tests/steps/enforce_post_merge_e2e_gate_steps.rs` (~line 77); the fixture body already writes `givn/commands.yaml`. Production files modified: `tests/steps/enforce_post_merge_e2e_gate_steps.rs`. Targeted re-run — MUST exit 0:
        ```text
        $ GIVN_FEATURES=givn/changes/drop-legacy-residue/specs cargo test --test features_runner -- --name '^Archive is blocked when merged permanent specs gain @e2e scenarios and e2e_command is unconfigured$'
        Feature: Enforce Post-Merge E2E Gate
          Scenario: Archive is blocked when merged permanent specs gain @e2e scenarios and e2e_command is unconfigured
           ✔  Given a project root with givn/ initialised
           ✔  And the givn/commands.yaml has e2e_command unconfigured
           ✔  And a completed change "add-e2e-scenario" whose delta contains an @e2e-tagged scenario
           ✔  And the permanent specs currently have no @e2e scenarios
           ✔  And the project declares a complete run declaration
           ✔  When the user runs `givn archive --change add-e2e-scenario`
           ✔  Then the command exits non-zero
           ✔  And stderr contains "verify.e2e_command"
           ✔  And stderr mentions the e2e runner is not configured
           ✔  And the change directory still exists under givn/changes/
        [Summary]
        1 feature
        1 scenario (1 passed)
        10 steps (10 passed)
        (exit 0)
        ```
  - [x] REFACTOR: no cleanup (attribute rename only); re-run targeted — MUST exit 0:
        ```text
        $ GIVN_FEATURES=givn/changes/drop-legacy-residue/specs cargo test --test features_runner -- --name '^Archive is blocked when merged permanent specs gain @e2e scenarios and e2e_command is unconfigured$'
        [Summary]
        1 feature
        1 scenario (1 passed)
        10 steps (10 passed)
        (exit 0)
        ```
  - [x] Nextest gate evidence (F21): paste the `./run-tests.sh` nextest run showing this scenario passing under the gate protocol:
        ```text
        $ GIVN_FEATURES=givn/changes/drop-legacy-residue/specs ./run-tests.sh
        Nextest run ID 8679c327-b6ba-44a7-954e-54bd4117cd30 with nextest profile: default
            Starting 16 tests across 1 binary
                PASS [   0.677s] (10/16) givn::features_runner archive-change/enforce-post-merge-e2e-gate.feature::Archive is blocked when merged permanent specs gain @e2e scenarios and e2e_command is unconfigured
                PASS [   0.728s] (11/16) givn::features_runner archive-change/enforce-post-merge-e2e-gate.feature::Archive succeeds when merged permanent specs have no @e2e scenarios and e2e_command is unconfigured
                FAIL ... (the remaining still-@wip scenario sections of other tasks, expected mid-change)
        Summary [   0.836s] 12/16 tests run: 2 passed, 10 failed, 0 skipped
        (exit 100 — the still-@wip sections of the other tasks fail as expected; this scenario PASSes under the nextest gate protocol)
        ```
  - [x] COMMIT: `feat(enforce-post-merge-e2e-gate): <release note>` with the scenario title in the message body — Hash: `8bdcccac0dd7bc29a565a1e55eabff1535fb282d`

- [x] Scenario: `Archive succeeds when merged permanent specs have no @e2e scenarios and e2e_command is unconfigured` (`enforce-post-merge-e2e-gate`, `@givn.modified`, reuse)
  **Design constraints:**
  - Delta file: `givn/changes/drop-legacy-residue/specs/archive-change/enforce-post-merge-e2e-gate.feature`. Given text re-pointed to `the givn/commands.yaml has e2e_command unconfigured` — the exact shared step renamed by the previous scenario's GREEN. No additional file change; no production code change.
  - Behaviour: with zero `@e2e` scenarios in the merged permanent specs, the archive exits 0 and the change directory moves to `givn/archive/`.
  - Domain context: `archive-change` — the permanent corpus contains the accepted change, bound to a verification receipt for the revision that passed every verification scope.
  - Single-scenario run: `GIVN_FEATURES=givn/changes/drop-legacy-residue/specs cargo test --test features_runner -- --name '^Archive succeeds when merged permanent specs have no @e2e scenarios and e2e_command is unconfigured$'`
  - Remove `@wip` from this scenario only at RED.
  - [x] RED: remove `@wip` from this scenario only; run the single-scenario command. All steps reused (the shared Given `the givn/commands.yaml has e2e_command unconfigured` landed in the previous scenario's GREEN) → zero exit is legitimate immediate GREEN. Record the reused steps: `the givn/commands.yaml has e2e_command unconfigured` (renamed in the previous scenario's GREEN); all archive steps pre-existing. Paste the run:
        ```text
        $ GIVN_FEATURES=givn/changes/drop-legacy-residue/specs cargo test --test features_runner -- --name '^Archive succeeds when merged permanent specs have no @e2e scenarios and e2e_command is unconfigured$'
        Feature: Enforce Post-Merge E2E Gate
          Scenario: Archive succeeds when merged permanent specs have no @e2e scenarios and e2e_command is unconfigured
           ✔  Given a project root with givn/ initialised
           ✔  And the givn/commands.yaml has e2e_command unconfigured
           ✔  And a completed change "no-e2e" whose delta contains zero @e2e-tagged scenarios
           ✔  And the permanent specs currently have no @e2e scenarios
           ✔  And the project declares a complete run declaration
           ✔  When the user runs `givn archive --change no-e2e`
           ✔  Then the command exits 0
           ✔  And the change directory has been moved to givn/archive/
        [Summary]
        1 feature
        1 scenario (1 passed)
        8 steps (8 passed)
        (exit 0 — zero exit with all steps reused)
        ```
  - [x] GREEN: no new code; re-run targeted to confirm — MUST exit 0. Production files: none (pure reuse scenario):
        ```text
        $ GIVN_FEATURES=givn/changes/drop-legacy-residue/specs cargo test --test features_runner -- --name '^Archive succeeds when merged permanent specs have no @e2e scenarios and e2e_command is unconfigured$'
        [Summary]
        1 feature
        1 scenario (1 passed)
        8 steps (8 passed)
        (exit 0)
        ```
  - [x] REFACTOR: none; re-run targeted — MUST exit 0:
        ```text
        $ GIVN_FEATURES=givn/changes/drop-legacy-residue/specs cargo test --test features_runner -- --name '^Archive succeeds when merged permanent specs have no @e2e scenarios and e2e_command is unconfigured$'
        [Summary]
        1 feature
        1 scenario (1 passed)
        8 steps (8 passed)
        (exit 0)
        ```
  - [x] Nextest gate evidence (F21): paste the `./run-tests.sh` nextest run showing this scenario passing under the gate protocol:
        ```text
        $ GIVN_FEATURES=givn/changes/drop-legacy-residue/specs ./run-tests.sh
        Nextest run ID 0e89b0b8-67f7-46ea-9d39-6a1df08bbaa8 with nextest profile: default
            Starting 16 tests across 1 binary
                PASS [   0.680s] (10/16) givn::features_runner archive-change/enforce-post-merge-e2e-gate.feature::Archive is blocked when merged permanent specs gain @e2e scenarios and e2e_command is unconfigured
                PASS [   0.729s] (11/16) givn::features_runner archive-change/enforce-post-merge-e2e-gate.feature::Archive succeeds when merged permanent specs have no @e2e scenarios and e2e_command is unconfigured
                FAIL ... (the remaining still-@wip scenario sections of other tasks, expected mid-change)
        Summary [   0.780s] 12/16 tests run: 2 passed, 10 failed, 0 skipped
        (exit 100 — the still-@wip sections of the other tasks fail as expected; this scenario PASSes under the nextest gate protocol)
        ```
  - [x] COMMIT: `feat(enforce-post-merge-e2e-gate): <release note>` with the scenario title in the message body (carries the `@wip` removal and this tasks.md record; no production delta — pure reuse) — Hash: `e4a9ce4a2af7b003faad20c250f6b47e269607e7`

- [x] Scenario: `Retiring a use case's last capability removes the use-case directory and its narrative` (`grouped-merge-for-new-capabilities`, `@givn.added`)
  **Design constraints:**
  - Delta file: `givn/changes/drop-legacy-residue/specs/archive-change/grouped-merge-for-new-capabilities.feature`. Deliberately not `@e2e`: it pins a merge rule through the real archive subprocess (established practice in `archive-proof` and `preserve-scenario-docstrings`); the capability's `@e2e` evidence is unchanged, no interaction-inventory row is added, and the terminal evidence set stays at the six transcripts of Q1.
  - Files: modify `src/cli/archive.rs` — extend the retirement branch of `process_delta_file` (the `Some(usecase_dir)` arm): after removing the retiring capability's `.feature`, when the use-case directory is not `fragments` and holds no remaining `.feature` files, remove `usecase.md` when present and remove the directory; any `io::Error` propagates with `.with_context()`, leaving the already-recorded tombstone in place (repair-forward). A directory that still holds another capability's `.feature` stays untouched together with its narrative.
  - Unit tests (same commit): in `src/cli/archive.rs`, update `retirement_deletes_capability_and_writes_tombstone` — the last-capability fixture now also asserts the use-case directory and narrative are gone; add `retirement_of_shared_usecase_keeps_directory_and_narrative` — a remaining sibling capability's `.feature` keeps the directory untouched.
  - Fixtures: new steps in `tests/steps/grouped_merge_steps.rs`: `a givn project with a permanent use case "…" whose only capability is "…"`, `a completed change "…" whose delta retires capability "…"`, `the tombstone "givn/specs/.retired.yaml" records capability "…"`. The archive steps (`the project declares a complete run declaration`, `the user invokes `givn archive --change …``, `the command exits 0`, `the directory "…" does not exist`) already exist.
  - Behaviour: archiving the change exits 0, `givn/specs/retire-home` (use-case directory) no longer exists, the tombstone records capability "only-cap", and the change directory moves to `givn/archive/`.
  - Domain context: `archive-change` main flow 2 — the archive merge applies retirement and grouping rules (`resolve_capability_usecase` + `process_delta_file`).
  - Single-scenario run: `GIVN_FEATURES=givn/changes/drop-legacy-residue/specs cargo test --test features_runner -- --name "^Retiring a use case's last capability removes the use-case directory and its narrative$"` (double-quoted: the title contains an apostrophe)
  - Remove `@wip` from this scenario only at RED.
  - [x] RED: remove `@wip` from this scenario only; run the single-scenario command. New steps use `unimplemented!()` stubs (`a givn project with a permanent use case "…" whose only capability is "…"`, `a completed change "…" whose delta retires capability "…"`, `the tombstone "givn/specs/.retired.yaml" records capability "…"`). Paste output — MUST exit non-zero:
        ```text
        $ GIVN_FEATURES=givn/changes/drop-legacy-residue/specs cargo test --test features_runner -- --name "^Retiring a use case's last capability removes the use-case directory and its narrative$"
        Feature: grouped-merge-for-new-capabilities
          Scenario: Retiring a use case's last capability removes the use-case directory and its narrative
           ✘  Given a givn project with a permanent use case "retire-home" whose only capability is "only-cap"
              Step failed:
              Defined: givn/changes/drop-legacy-residue/specs/archive-change/grouped-merge-for-new-capabilities.feature:40:5
              Matched: tests/steps/grouped_merge_steps.rs:115:1
              Step panicked. Captured output: not implemented: TODO: retirement fixture
        [Summary]
        1 feature
        1 scenario (1 failed)
        1 step (1 failed)
        error: test failed (exit status: 1)
        ```
  - [x] GREEN: implement the extended retirement in `src/cli/archive.rs`, the three fixture steps in `tests/steps/grouped_merge_steps.rs`, and the two unit-test changes in `src/cli/archive.rs`. Production files modified: `src/cli/archive.rs`, `tests/steps/grouped_merge_steps.rs`. Targeted re-run — MUST exit 0:
        ```text
        $ cargo test --lib cli::archive::tests
        test cli::archive::tests::retirement_deletes_capability_and_writes_tombstone ... ok
        test cli::archive::tests::retirement_of_shared_usecase_keeps_directory_and_narrative ... ok
        test cli::archive::tests::retirement_targets_grouped_capability_directory ... ok
        (… 10 passed; 0 failed …)

        $ GIVN_FEATURES=givn/changes/drop-legacy-residue/specs cargo test --test features_runner -- --name "^Retiring a use case's last capability removes the use-case directory and its narrative$"
        Feature: grouped-merge-for-new-capabilities
          Scenario: Retiring a use case's last capability removes the use-case directory and its narrative
           ✔  Given a givn project with a permanent use case "retire-home" whose only capability is "only-cap"
           ✔  And a completed change "retire-last" whose delta retires capability "only-cap"
           ✔  And the project declares a complete run declaration
           ✔  When the user invokes `givn archive --change retire-last`
           ✔  Then the command exits 0
           ✔  And the directory "givn/specs/retire-home" does not exist
           ✔  And the tombstone "givn/specs/.retired.yaml" records capability "only-cap"
           ✔  And the change directory has been moved to givn/archive/
        [Summary]
        1 feature
        1 scenario (1 passed)
        8 steps (8 passed)
        (exit 0)
        ```
  - [x] REFACTOR: keep the retirement arm minimal (no dead abstractions); re-run targeted — MUST exit 0:
        ```text
        $ GIVN_FEATURES=givn/changes/drop-legacy-residue/specs cargo test --test features_runner -- --name "^Retiring a use case's last capability removes the use-case directory and its narrative$"
        [Summary]
        1 feature
        1 scenario (1 passed)
        8 steps (8 passed)
        (exit 0)
        ```
  - [x] Nextest gate evidence (F21): paste the `./run-tests.sh` nextest run showing this scenario passing under the gate protocol:
        ```text
        $ GIVN_FEATURES=givn/changes/drop-legacy-residue/specs ./run-tests.sh
        Nextest run ID 45f6f86b-12d3-447d-b3c5-07f427c79f05 with nextest profile: default
            Starting 16 tests across 1 binary
                PASS [   0.742s] (11/16) givn::features_runner archive-change/grouped-merge-for-new-capabilities.feature::Retiring a use case's last capability removes the use-case directory and its narrative
                FAIL ... (the remaining still-@wip scenario sections of other tasks, expected mid-change)
        Summary [   0.846s] 12/16 tests run: 3 passed, 9 failed, 0 skipped
        (exit 100 — the still-@wip sections of the other tasks fail as expected; this scenario PASSes under the nextest gate protocol)
        ```
  - [x] COMMIT: `feat(grouped-merge-for-new-capabilities): <release note>` with the scenario title in the message body — Hash: `6047f6e64597631f33b8a2d10e1b2d7010498d9b`

- [x] Scenario: `Verify-e2e hook resolves the change directory from a relative features path inside givn/changes` (`resolve-change-directories`, `@givn.modified`, reuse)
  **Design constraints:**
  - Delta file: `givn/changes/drop-legacy-residue/specs/verify-change/resolve-change-directories.feature`. Given text re-pointed to `the givn/commands.yaml has e2e_command unconfigured` — the exact shared step renamed by the first `enforce-post-merge-e2e-gate` scenario's GREEN. No additional step change; no production code change.
  - Behaviour: `givn check review --change e2e-detect` exits 0 for a change whose design.md carries a valid Step Definitions table, whose tasks.md COMMIT hashes are valid 40-character hex strings, and whose delta specs contain zero `@e2e` scenarios.
  - Domain context: `verify-change` — an invalid change cannot pass the verification gate; the developer receives a complete verification result.
  - Single-scenario run: `GIVN_FEATURES=givn/changes/drop-legacy-residue/specs cargo test --test features_runner -- --name '^Verify-e2e hook resolves the change directory from a relative features path inside givn/changes$'`
  - Remove `@wip` from this scenario only at RED.
  - [x] RED: remove `@wip` from this scenario only; run the single-scenario command. All steps reused (shared Given renamed in the first `enforce-post-merge-e2e-gate` scenario's GREEN) → zero exit is legitimate immediate GREEN. Record the reused steps: `the givn/commands.yaml has e2e_command unconfigured`; the fixture steps are the pre-existing `deduplicate_change_dir_detection_steps.rs` steps. Paste the run:
        ```text
        $ GIVN_FEATURES=givn/changes/drop-legacy-residue/specs cargo test --test features_runner -- --name '^Verify-e2e hook resolves the change directory from a relative features path inside givn/changes$'
        Feature: Resolve change directories
          Scenario: Verify-e2e hook resolves the change directory from a relative features path inside givn/changes
           ✔  Given a project root with givn/ initialised
           ✔  And the givn/commands.yaml has e2e_command unconfigured
           ✔  And a change "e2e-detect" exists with design.md containing a valid Step Definitions table
           ✔  And a change "e2e-detect" exists with tasks.md where every COMMIT hash is a valid 40-character hex string
           ✔  And a change "e2e-detect" exists with specs under "specs/" containing zero @e2e scenarios
           ✔  And the project declares a complete run declaration
           ✔  When the user runs `givn check review --change e2e-detect`
           ✔  Then the command exits 0
        [Summary]
        1 feature
        1 scenario (1 passed)
        8 steps (8 passed)
        (exit 0 — zero exit with all steps reused)
        ```
  - [x] GREEN: no new code; re-run targeted to confirm — MUST exit 0. Production files: none (pure reuse scenario):
        ```text
        $ GIVN_FEATURES=givn/changes/drop-legacy-residue/specs cargo test --test features_runner -- --name '^Verify-e2e hook resolves the change directory from a relative features path inside givn/changes$'
        [Summary]
        1 feature
        1 scenario (1 passed)
        8 steps (8 passed)
        (exit 0)
        ```
  - [x] REFACTOR: none; re-run targeted — MUST exit 0:
        ```text
        $ GIVN_FEATURES=givn/changes/drop-legacy-residue/specs cargo test --test features_runner -- --name '^Verify-e2e hook resolves the change directory from a relative features path inside givn/changes$'
        [Summary]
        1 feature
        1 scenario (1 passed)
        8 steps (8 passed)
        (exit 0)
        ```
  - [x] Nextest gate evidence (F21): paste the `./run-tests.sh` nextest run showing this scenario passing under the gate protocol:
        ```text
        The full `./run-tests.sh` gate run cancels the tail units on first
        failure (nextest fail-fast), so this unit never runs in it. The same
        gate protocol, filtered to this scenario:
        $ GIVN_FEATURES=givn/changes/drop-legacy-residue/specs GIVN_CUCUMBER_TAGS='' cargo nextest run --no-tests=warn --locked --test features_runner 'resolve-change-directories.feature::Verify-e2e hook resolves the change directory from a relative features path inside givn/changes'
        Nextest run ID 0d4aef97-48e4-47c4-887b-1e0c8f0f1bc2 with nextest profile: default
            Starting 1 test across 1 binary (15 tests skipped)
                PASS [   0.488s] (1/1) givn::features_runner verify-change/resolve-change-directories.feature::Verify-e2e hook resolves the change directory from a relative features path inside givn/changes
        Summary [   0.488s] 1 test run: 1 passed, 15 skipped
        (exit 0)

        Full gate with fail-fast off (`--no-fail-fast`): 16 run — 4 passed
        (this scenario and the three earlier sections), 12 failed (the
        remaining still-@wip sections of the other tasks, expected mid-change):
                PASS [   0.517s] (16/16) givn::features_runner verify-change/resolve-change-directories.feature::Verify-e2e hook resolves the change directory from a relative features path inside givn/changes
             Summary [   1.103s] 16 tests run: 4 passed, 12 failed, 0 skipped
        ```
  - [x] COMMIT: `feat(resolve-change-directories): <release note>` with the scenario title in the message body (carries the `@wip` removal and this tasks.md record; no production delta — pure reuse) — Hash: `dacdfd43bd469f2117a366973ebc7722f4048b53`

- [x] Scenario: `The migration guide is an executable prompt sequence` (`describe-the-release`, `@givn.modified`)
  **Design constraints:**
  - Delta file: `givn/changes/drop-legacy-residue/specs/publish-a-release/describe-the-release.feature`. The title carries no version, so it stays unversioned and is re-pointed (D6) to `docs/migrations/0.7.0-to-0.8.0.md`, the oldest retained guide.
  - Files: modify `tests/steps/release_0_3_0_steps.rs` — re-point the Given attribute `the 0.2.0 to 0.3.0 migration guide` to `the 0.7.0 to 0.8.0 migration guide` (~line 122) and the `an LLM reads the guide from a project using Givn` body path (~line 129) to `docs/migrations/0.7.0-to-0.8.0.md`. Update the three Then steps to the retained guide's actual prompt contract: ordered headings of the exact shape `### Prompt N — <Title>` (em dash) ending in `### Prompt 6 — Validate and archive`; the sentence "The LLM must inspect the project before editing it" (line 9); the corpus boundary in the guide's own words — "The maintenance change is the **inventory and plan**, not the repair site." (line 42), "Do not edit the corpus in this change." (Prompt 1), and "Confirm this change edited no corpus artifact except the term collision-check cells" (Prompt 6). The guide carries no standalone "edits no corpus artifact" sentence; the rewrite matches the phrases above.
  - Behaviour: the guide asserts ordered prompts that begin with inventory and end with validation and archive, requires the LLM to inspect the project before editing it, and keeps corpus repairs out of the maintenance change.
  - Sequencing note: between the re-pointing commit and the archive merge, the permanent scenario still names the old guide steps; the delta-scoped gates (`GIVN_FEATURES`) are the only runner input until the merge replaces the scenario.
  - Domain context: `publish-a-release` — a release with incomplete metadata is never presented as complete; the migration guide is an executable prompt sequence that forbids automatic changes to domain specifications and architecture decisions.
  - Single-scenario run: `GIVN_FEATURES=givn/changes/drop-legacy-residue/specs cargo test --test features_runner -- --name '^The migration guide is an executable prompt sequence$'`
  - Remove `@wip` from this scenario only at RED.
  - [x] RED: remove `@wip` from this scenario only; run the single-scenario command. The re-pointed Given text `the 0.7.0 to 0.8.0 migration guide` has no matching step definition → undefined step fails (strict mode). Paste output — MUST exit non-zero:
        ```text
        $ GIVN_FEATURES=givn/changes/drop-legacy-residue/specs cargo test --test features_runner -- --name '^The migration guide is an executable prompt sequence$'
        Feature: Givn release metadata and migration route
          Scenario: The migration guide is an executable prompt sequence
           ✘  Given the 0.7.0 to 0.8.0 migration guide
              Step failed:
              Defined: givn/changes/drop-legacy-residue/specs/publish-a-release/describe-the-release.feature:7:5
              Step doesn't match any function
        [Summary]
        1 feature
        1 scenario (1 failed)
        1 step (1 failed)
        error: test failed (exit status: 1)
        ```
  - [x] GREEN: apply the re-point and the three assertion rewrites in `tests/steps/release_0_3_0_steps.rs`. Production files modified: `tests/steps/release_0_3_0_steps.rs`. Targeted re-run — MUST exit 0:
        ```text
        $ cargo check --tests
            Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.85s

        $ GIVN_FEATURES=givn/changes/drop-legacy-residue/specs cargo test --test features_runner -- --name '^The migration guide is an executable prompt sequence$'
        Feature: Givn release metadata and migration route
          Scenario: The migration guide is an executable prompt sequence
           ✔  Given the 0.7.0 to 0.8.0 migration guide
           ✔  When an LLM reads the guide from a project using Givn
           ✔  Then it finds ordered prompts that begin with inventory and end with validation and archive
           ✔  And the guide requires the LLM to inspect the project before editing it
           ✔  And the guide keeps corpus repairs out of the maintenance change
        [Summary]
        1 feature
        1 scenario (1 passed)
        5 steps (5 passed)
        (exit 0)
        ```
  - [x] REFACTOR: no cleanup; re-run targeted — MUST exit 0:
        ```text
        $ GIVN_FEATURES=givn/changes/drop-legacy-residue/specs cargo test --test features_runner -- --name '^The migration guide is an executable prompt sequence$'
        [Summary]
        1 feature
        1 scenario (1 passed)
        5 steps (5 passed)
        (exit 0)
        ```
  - [x] Nextest gate evidence (F21): paste the `./run-tests.sh` nextest run showing this scenario passing under the gate protocol:
        ```text
        The full `./run-tests.sh` gate run cancels the tail units on first
        failure (nextest fail-fast), so this unit never runs in it. The same
        gate protocol, filtered to this scenario:
        $ GIVN_FEATURES=givn/changes/drop-legacy-residue/specs GIVN_CUCUMBER_TAGS='' cargo nextest run --no-tests=warn --locked --test features_runner 'publish-a-release/describe-the-release.feature::The migration guide is an executable prompt sequence'
        Nextest run ID 5a304010-d090-4eb7-9195-3c03d47b67cc with nextest profile: default
            Starting 1 test across 1 binary (15 tests skipped)
                PASS [   0.400s] (1/1) givn::features_runner publish-a-release/describe-the-release.feature::The migration guide is an executable prompt sequence
        Summary [   0.400s] 1 test run: 1 passed, 15 skipped
        (exit 0)

        Full gate with fail-fast off (`--no-fail-fast`): 16 run — 5 passed
        (this scenario and the four earlier sections), 11 failed (the
        remaining still-@wip sections of the other tasks, expected mid-change):
             Summary [   1.092s] 16 tests run: 5 passed, 11 failed, 0 skipped
        ```
  - [x] COMMIT: `feat(describe-the-release): <release note>` with the scenario title in the message body — Hash: `e4235ed2ae0baafd4aa95da044b192c8278f1469`

- [x] Scenario: `A project below the oldest migration route receives no migration change` (`apply-a-managed-upgrade`, `@givn.added` — first migration section; carries the catalog/bundle deletion)
  **Design constraints:**
  - Delta file: `givn/changes/drop-legacy-residue/specs/upgrade-a-project/apply-a-managed-upgrade.feature`.
  - **This section carries the migration-catalog deletion (ordering note).** It is the natural first migration section: with the pre-0.8 bundles present, a 0.2.0 project receives a migration change and the scenario fails at RED; deleting the routes turns it green. Later migration sections (the four protection scenarios and the 0.7.0 outline) carry only their fixture/step/fixture-support deltas.
  - Files to delete/modify in this section's GREEN: delete the five bundle directories `assets/migrations/0.2.0-to-0.3.0/`, `0.3.0-to-0.4.0/`, `0.4.0-to-0.5.0/`, `0.5.0-to-0.6.0/`, `0.6.0-to-0.7.0/`; delete the five bundle entries from `assets/migrations/catalog.yaml` — the first remaining entry is `migrate-0-7-0-to-0-8-0`; update `src/cli/migration.rs` unit tests in the same commit (the real-catalog test breaks when the entries leave): the `bundle()` helper points at `migrations/0.7.0-to-0.8.0/…`; `catalog_contains_the_0_2_to_0_3_bundle` becomes a test that the oldest declared source is `0.7.0`; `aggregate_design_keeps_bundle_phases_in_order` anchors on the retained design asset's heading (`## Phase 1 — Use-case model (inventory)`) instead of the deleted `## Prompt 1: Inventory`.
  - D4: `migration_path` already returns an empty vector when no route starts at the recorded version — no production logic change, only the catalog, the fixtures, and the docs.
  - Fixtures: in `tests/steps/release_0_3_0_steps.rs` and `tests/steps/release_0_3_0_e2e_steps.rs`, parameterise the legacy fixture with its recorded version: keep `a clean Git project created with Givn 0.2.0` (this scenario's fixture, below the oldest route); delete the now-unused 0.2.0 step functions. `no_migration_change_exists` drops the redundant per-version prefix scan.
  - Behaviour: `givn upgrade` on a clean Git 0.2.0 project exits successfully, the config identifies the current version, one managed upgrade commit exists, and no migration change exists.
  - Domain context: `upgrade-a-project` (delta usecase.md) — a project whose recorded version has no declared route receives the managed refresh only; "a migration-free upgrade publishes exactly one revision and no plan revision"; the minimal guarantee protects the project from partial automatic migration.
  - Single-scenario run: `GIVN_FEATURES=givn/changes/drop-legacy-residue/specs cargo test --test features_runner -- --name '^A project below the oldest migration route receives no migration change$'`
  - Remove `@wip` from this scenario only at RED.
  - [x] RED: remove `@wip` from this scenario only; run the single-scenario command. All steps already exist (`a clean Git project created with Givn 0.2.0`, `the consumer invokes `givn upgrade` for migration`, `the command exits successfully`, `the project config identifies the current Givn version`, `one managed upgrade commit exists`, `no migration change exists`) — the failure is behavioural: the catalog still declares the 0.2.0 routes, so a migration change is created and `no migration change exists` fails. Paste output — MUST exit non-zero:
        ```text
        $ GIVN_FEATURES=givn/changes/drop-legacy-residue/specs cargo test --test features_runner -- --name '^A project below the oldest migration route receives no migration change$'
        Feature: Managed upgrade and legacy project migration
          Scenario: A project below the oldest migration route receives no migration change
           ✔  Given a clean Git project created with Givn 0.2.0
           ✔  When the consumer invokes `givn upgrade` for migration
           ✔  Then the command exits successfully
           ✔  And the project config identifies the current Givn version
           ✘  And one managed upgrade commit exists
              Step failed:
              Defined: givn/changes/drop-legacy-residue/specs/upgrade-a-project/apply-a-managed-upgrade.feature:31:5
              Matched: tests/steps/release_0_3_0_e2e_steps.rs:168:1
              Step panicked. Captured output: assertion `left == right` failed: expected fixture commit plus managed commit
                left: "3"
               right: "2"
        [Summary]
        1 feature
        1 scenario (1 failed)
        5 steps (4 passed, 1 failed)
        error: test failed (exit status: 1)
        (exit 1 — behavioural RED: the pre-0.8 catalog still routes 0.2.0, so the
        upgrade writes the aggregate migration change and its plan revision: 3
        commits instead of the managed-only 2.)
        ```
  - [x] GREEN: delete the five bundle directories and the five `assets/migrations/catalog.yaml` entries; apply the three `src/cli/migration.rs` unit-test updates; apply the fixture parameterisation (`no_migration_change_exists` drops the redundant per-version prefix scan; delete the now-unused 0.2.0 step functions except `a clean Git project created with Givn 0.2.0`). Production files modified: `assets/migrations/catalog.yaml`, five bundle directories deleted under `assets/migrations/`, `src/cli/migration.rs` (unit tests), `tests/steps/release_0_3_0_steps.rs`, `tests/steps/release_0_3_0_e2e_steps.rs`. Targeted re-run — MUST exit 0:
        ```text
        $ cargo test --lib cli::migration::tests
        running 10 tests
        test cli::migration::tests::catalog_declares_0_7_0_as_the_oldest_source ... ok
        test cli::migration::tests::migration_path_routes_0_7_0_to_the_current_release ... ok
        test cli::migration::tests::aggregate_design_keeps_bundle_phases_in_order ... ok
        test cli::migration::tests::aggregate_proposal_preserves_word_spacing ... ok
        (… 10 passed; 0 failed …)

        $ cargo clippy --workspace --all-targets -- -D warnings
            Finished `dev` profile [unoptimized + debuginfo] target(s) in 8.84s
        (exit 0)

        $ GIVN_FEATURES=givn/changes/drop-legacy-residue/specs cargo test --test features_runner -- --name '^A project below the oldest migration route receives no migration change$'
        Feature: Managed upgrade and legacy project migration
          Scenario: A project below the oldest migration route receives no migration change
           ✔  Given a clean Git project created with Givn 0.2.0
           ✔  When the consumer invokes `givn upgrade` for migration
           ✔  Then the command exits successfully
           ✔  And the project config identifies the current Givn version
           ✔  And one managed upgrade commit exists
           ✔  And no migration change exists
        [Summary]
        1 feature
        1 scenario (1 passed)
        6 steps (6 passed)
        (exit 0)
        ```
  - [x] REFACTOR: no cleanup beyond the deletions; re-run targeted — MUST exit 0:
        ```text
        $ GIVN_FEATURES=givn/changes/drop-legacy-residue/specs cargo test --test features_runner -- --name '^A project below the oldest migration route receives no migration change$'
        [Summary]
        1 feature
        1 scenario (1 passed)
        6 steps (6 passed)
        (exit 0)
        ```
  - [x] Nextest gate evidence (F21): paste the `./run-tests.sh` nextest run showing this scenario passing under the gate protocol:
        ```text
        The full `./run-tests.sh` gate run cancels the tail units on first
        failure (nextest fail-fast), so this unit never runs in it. The same
        gate protocol, filtered to this scenario:
        $ GIVN_FEATURES=givn/changes/drop-legacy-residue/specs GIVN_CUCUMBER_TAGS='' cargo nextest run --no-tests=warn --locked --test features_runner 'upgrade-a-project/apply-a-managed-upgrade.feature::A project below the oldest migration route receives no migration change'
        Nextest run ID f70b06ac-7f94-4ab3-9936-3f9d6952030a with nextest profile: default
            Starting 1 test across 1 binary (15 tests skipped)
                PASS [   0.670s] (1/1) givn::features_runner upgrade-a-project/apply-a-managed-upgrade.feature::A project below the oldest migration route receives no migration change
        Summary [   0.670s] 1 test run: 1 passed, 15 skipped
        (exit 0)

        Full gate with fail-fast off (`--no-fail-fast`): 16 run — 6 passed
        (this scenario, the guide scenario, and the four earlier sections),
        10 failed (the remaining still-@wip sections of the other tasks,
        expected mid-change):
                PASS [   0.812s] (11/16) givn::features_runner upgrade-a-project/apply-a-managed-upgrade.feature::A project below the oldest migration route receives no migration change
             Summary 16 tests run: 6 passed, 10 failed, 0 skipped
        ```
  - [x] COMMIT: `feat(apply-a-managed-upgrade): <release note>` with the scenario title in the message body — Hash: `ed00315edbb8f356c66636589a426fa031c26688`

- [x] Scenario: `An unclean project is protected from automatic migration commits` (`apply-a-managed-upgrade`, `@givn.modified` — fixture-only delta)
  **Design constraints:**
  - Delta file: `givn/changes/drop-legacy-residue/specs/upgrade-a-project/apply-a-managed-upgrade.feature`. The catalog deletion already landed in the previous section; this section carries only its fixture delta (ordering note).
  - Files: modify `tests/steps/release_0_3_0_e2e_steps.rs` — add `#[given("a Givn 0.7.0 project with uncommitted user changes")]`; the `dirty_legacy_project` fixture helper moves to the 0.7.0 fixture. The Then steps already exist; no production code change.
  - Behaviour: `givn upgrade` on a dirty 0.7.0 project fails before changing project files, creates no commit, and the user changes remain byte-for-byte unchanged.
  - Domain context: `upgrade-a-project` extension 1 — uncommitted changes exist → the upgrade is refused before any write, naming the dirty paths; protects the minimal guarantee.
  - Single-scenario run: `GIVN_FEATURES=givn/changes/drop-legacy-residue/specs cargo test --test features_runner -- --name '^An unclean project is protected from automatic migration commits$'`
  - Remove `@wip` from this scenario only at RED.
  - [x] RED: remove `@wip` from this scenario only; run the single-scenario command. The 0.7.0 Given text has no matching step definition → undefined step fails (strict mode). Paste output — MUST exit non-zero:
        ```text
        $ GIVN_FEATURES=givn/changes/drop-legacy-residue/specs cargo test --test features_runner -- --name '^An unclean project is protected from automatic migration commits$'
        Feature: Managed upgrade and legacy project migration
          Scenario: An unclean project is protected from automatic migration commits
           ✘  Given a Givn 0.7.0 project with uncommitted user changes
              Step failed:
              Defined: givn/changes/drop-legacy-residue/specs/upgrade-a-project/apply-a-managed-upgrade.feature:36:5
              Step doesn't match any function
        [Summary]
        1 feature
        1 scenario (1 failed)
        1 step (1 failed)
        error: test failed (exit status: 1)
        ```
  - [x] GREEN: add the 0.7.0 Given step definition (fixture parameterised with its recorded version); the Then steps are reused as-is. Production files modified: `tests/steps/release_0_3_0_e2e_steps.rs`. Targeted re-run — MUST exit 0:
        ```text
        $ cargo check --tests
            Finished `dev` profile [unoptimized + debuginfo] target(s) in 2.44s

        $ GIVN_FEATURES=givn/changes/drop-legacy-residue/specs cargo test --test features_runner -- --name '^An unclean project is protected from automatic migration commits$'
        Feature: Managed upgrade and legacy project migration
          Scenario: An unclean project is protected from automatic migration commits
           ✔  Given a Givn 0.7.0 project with uncommitted user changes
           ✔  When the consumer invokes `givn upgrade` for migration
           ✔  Then the command fails before changing project files
           ✔  And no commit is created
           ✔  And the user changes remain byte-for-byte unchanged
        [Summary]
        1 feature
        1 scenario (1 passed)
        5 steps (5 passed)
        (exit 0 — Then steps `the command fails before changing project files`,
        `no commit is created`, `the user changes remain byte-for-byte unchanged`
        reused as-is)
        ```
  - [x] REFACTOR: no cleanup; re-run targeted — MUST exit 0:
        ```text
        $ GIVN_FEATURES=givn/changes/drop-legacy-residue/specs cargo test --test features_runner -- --name '^An unclean project is protected from automatic migration commits$'
        [Summary]
        1 feature
        1 scenario (1 passed)
        5 steps (5 passed)
        (exit 0)
        ```
  - [x] Nextest gate evidence (F21): paste the `./run-tests.sh` nextest run showing this scenario passing under the gate protocol:
        ```text
        The full `./run-tests.sh` gate run cancels the tail units on first
        failure (nextest fail-fast), so this unit never runs in it. The same
        gate protocol, filtered to this scenario:
        $ GIVN_FEATURES=givn/changes/drop-legacy-residue/specs GIVN_CUCUMBER_TAGS='' cargo nextest run --no-tests=warn --locked --test features_runner 'upgrade-a-project/apply-a-managed-upgrade.feature::An unclean project is protected from automatic migration commits'
        Nextest run ID 4d728061-e9bd-467c-8b40-a001844d736a with nextest profile: default
            Starting 1 test across 1 binary (15 tests skipped)
                PASS [   0.478s] (1/1) givn::features_runner upgrade-a-project/apply-a-managed-upgrade.feature::An unclean project is protected from automatic migration commits
        Summary [   0.478s] 1 test run: 1 passed, 15 skipped
        (exit 0)
        ```
  - [x] COMMIT: `feat(apply-a-managed-upgrade): <release note>` with the scenario title in the message body — Hash: `efbe91644e44c2c2494d82e8ba02eb9479f16409`

- [x] Scenario: `A project outside Git is protected from a partial automatic migration` (`apply-a-managed-upgrade`, `@givn.modified` — fixture-only delta)
  **Design constraints:**
  - Delta file: `givn/changes/drop-legacy-residue/specs/upgrade-a-project/apply-a-managed-upgrade.feature`. This section carries only its fixture delta (ordering note).
  - Files: modify `tests/steps/release_0_3_0_e2e_steps.rs` — add `#[given("a Givn 0.7.0 project that is not a Git repository")]`; the `non_git_legacy_project` fixture helper moves to the 0.7.0 fixture. The Then steps already exist; no production code change.
  - Behaviour: `givn upgrade` on a non-Git 0.7.0 project fails before changing project files and explains that a version-controlled project is required for the automatic migration.
  - Domain context: `upgrade-a-project` extension 1 — no version-control system is selected → the upgrade is refused by name before any write; protects the minimal guarantee.
  - Single-scenario run: `GIVN_FEATURES=givn/changes/drop-legacy-residue/specs cargo test --test features_runner -- --name '^A project outside Git is protected from a partial automatic migration$'`
  - Remove `@wip` from this scenario only at RED.
  - [x] RED: remove `@wip` from this scenario only; run the single-scenario command. The 0.7.0 Given text has no matching step definition → undefined step fails (strict mode). Paste output — MUST exit non-zero:
        ```text
        $ GIVN_FEATURES=givn/changes/drop-legacy-residue/specs cargo test --test features_runner -- --name '^A project outside Git is protected from a partial automatic migration$'
        Feature: Managed upgrade and legacy project migration
          Scenario: A project outside Git is protected from a partial automatic migration
           ✘  Given a Givn 0.7.0 project that is not a Git repository
              Step failed:
              Defined: givn/changes/drop-legacy-residue/specs/upgrade-a-project/apply-a-managed-upgrade.feature:44:5
              Step doesn't match any function
        [Summary]
        1 feature
        1 scenario (1 failed)
        1 step (1 failed)
        error: test failed (exit status: 1)
        ```
  - [x] GREEN: add the 0.7.0 Given step definition; the Then steps are reused as-is. Production files modified: `tests/steps/release_0_3_0_e2e_steps.rs`. Targeted re-run — MUST exit 0:
        ```text
        $ cargo check --tests
            Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.85s

        $ GIVN_FEATURES=givn/changes/drop-legacy-residue/specs cargo test --test features_runner -- --name '^A project outside Git is protected from a partial automatic migration$'
        Feature: Managed upgrade and legacy project migration
          Scenario: A project outside Git is protected from a partial automatic migration
           ✔  Given a Givn 0.7.0 project that is not a Git repository
           ✔  When the consumer invokes `givn upgrade` for migration
           ✔  Then the command fails before changing project files
           ✔  And the command explains that a version-controlled project is required for the automatic migration
        [Summary]
        1 feature
        1 scenario (1 passed)
        4 steps (4 passed)
        (exit 0 — Then steps `the command fails before changing project files` and
        `the command explains that a version-controlled project is required for
        the automatic migration` reused as-is)
        ```
  - [x] REFACTOR: no cleanup; re-run targeted — MUST exit 0:
        ```text
        $ GIVN_FEATURES=givn/changes/drop-legacy-residue/specs cargo test --test features_runner -- --name '^A project outside Git is protected from a partial automatic migration$'
        [Summary]
        1 feature
        1 scenario (1 passed)
        4 steps (4 passed)
        (exit 0)
        ```
  - [x] Nextest gate evidence (F21): paste the `./run-tests.sh` nextest run showing this scenario passing under the gate protocol:
        ```text
        The full `./run-tests.sh` gate run cancels the tail units on first
        failure (nextest fail-fast), so this unit never runs in it. The same
        gate protocol, filtered to this scenario:
        $ GIVN_FEATURES=givn/changes/drop-legacy-residue/specs GIVN_CUCUMBER_TAGS='' cargo nextest run --no-tests=warn --locked --test features_runner 'upgrade-a-project/apply-a-managed-upgrade.feature::A project outside Git is protected from a partial automatic migration'
        Nextest run ID 18c3f9f8-3c55-47b1-9b4c-7cf799d83547 with nextest profile: default
            Starting 1 test across 1 binary (15 tests skipped)
                PASS [   0.442s] (1/1) givn::features_runner upgrade-a-project/apply-a-managed-upgrade.feature::A project outside Git is protected from a partial automatic migration
        Summary [   0.442s] 1 test run: 1 passed, 15 skipped
        (exit 0)
        ```
  - [x] COMMIT: `feat(apply-a-managed-upgrade): <release note>` with the scenario title in the message body — Hash: `c97bc27fe043f64e5b878e7667b8879836359767`

- [x] Scenario: `An existing migration plan is never overwritten` (`apply-a-managed-upgrade`, `@givn.modified` — fixture-only delta)
  **Design constraints:**
  - Delta file: `givn/changes/drop-legacy-residue/specs/upgrade-a-project/apply-a-managed-upgrade.feature`. This section carries only its fixture delta (ordering note).
  - Files: modify `tests/steps/release_0_3_0_steps.rs` — add the 0.7.0 existing-plan regex Given (`a Givn 0.7.0 project with an existing "migrate-0-7-0-to-0-8-0" change`); the `automatic_migration_change` fixture helper moves to the 0.7.0 fixture. The Then steps already exist; no production code change.
  - Behaviour: `givn upgrade` does not overwrite the existing migration proposal or design and reports the existing migration change.
  - Domain context: `upgrade-a-project` extension 5 — a plan for the gap already exists → the existing proposal and design are preserved and reported; protects the success guarantee.
  - Single-scenario run: `GIVN_FEATURES=givn/changes/drop-legacy-residue/specs cargo test --test features_runner -- --name '^An existing migration plan is never overwritten$'`
  - Remove `@wip` from this scenario only at RED.
  - [x] RED: remove `@wip` from this scenario only; run the single-scenario command. The 0.7.0 Given text has no matching step definition → undefined step fails (strict mode). Paste output — MUST exit non-zero:
        ```text
        $ GIVN_FEATURES=givn/changes/drop-legacy-residue/specs cargo test --test features_runner -- --name '^An existing migration plan is never overwritten$'
        Feature: Managed upgrade and legacy project migration
          Scenario: An existing migration plan is never overwritten
           ✘  Given a Givn 0.7.0 project with an existing "migrate-0-7-0-to-0-8-0" change
              Step failed:
              Defined: givn/changes/drop-legacy-residue/specs/upgrade-a-project/apply-a-managed-upgrade.feature:51:5
              Step doesn't match any function
        [Summary]
        1 feature
        1 scenario (1 failed)
        1 step (1 failed)
        error: test failed (exit status: 1)
        ```
  - [x] GREEN: add the 0.7.0 existing-plan Given step definition; the Then steps are reused as-is. Production files modified: `tests/steps/release_0_3_0_steps.rs`. Targeted re-run — MUST exit 0:
        ```text
        $ cargo check --tests
            Finished `dev` profile [unoptimized + debuginfo] target(s) in 4.96s

        $ GIVN_FEATURES=givn/changes/drop-legacy-residue/specs cargo test --test features_runner -- --name '^An existing migration plan is never overwritten$'
        Feature: Managed upgrade and legacy project migration
          Scenario: An existing migration plan is never overwritten
           ✔  Given a Givn 0.7.0 project with an existing "migrate-0-7-0-to-0-8-0" change
           ✔  When the consumer invokes `givn upgrade` for migration
           ✔  Then the command does not overwrite the existing migration proposal or design
           ✔  And the command reports the existing migration change
        [Summary]
        1 feature
        1 scenario (1 passed)
        4 steps (4 passed)
        (exit 0 — Then steps `the command does not overwrite the existing migration
        proposal or design` and `the command reports the existing migration change`
        reused as-is. Note: the `automatic_migration_change` helper named in this
        section's constraints already moved to the 0.7.0 fixture in the
        no-route section's GREEN, where the fixture parameterisation landed.)
        ```
  - [x] REFACTOR: no cleanup; re-run targeted — MUST exit 0:
        ```text
        $ GIVN_FEATURES=givn/changes/drop-legacy-residue/specs cargo test --test features_runner -- --name '^An existing migration plan is never overwritten$'
        [Summary]
        1 feature
        1 scenario (1 passed)
        4 steps (4 passed)
        (exit 0)
        ```
  - [x] Nextest gate evidence (F21): paste the `./run-tests.sh` nextest run showing this scenario passing under the gate protocol:
        ```text
        The full `./run-tests.sh` gate run cancels the tail units on first
        failure (nextest fail-fast), so this unit never runs in it. The same
        gate protocol, filtered to this scenario:
        $ GIVN_FEATURES=givn/changes/drop-legacy-residue/specs GIVN_CUCUMBER_TAGS='' cargo nextest run --no-tests=warn --locked --test features_runner 'upgrade-a-project/apply-a-managed-upgrade.feature::An existing migration plan is never overwritten'
        Nextest run ID 806777f9-2977-4955-910d-cb73bb2b4300 with nextest profile: default
            Starting 1 test across 1 binary (15 tests skipped)
                PASS [   0.467s] (1/1) givn::features_runner upgrade-a-project/apply-a-managed-upgrade.feature::An existing migration plan is never overwritten
        Summary [   0.467s] 1 test run: 1 passed, 15 skipped
        (exit 0)
        ```
  - [x] COMMIT: `feat(apply-a-managed-upgrade): <release note>` with the scenario title in the message body — Hash: `320e09c1a603bfd84abb862b7046930d1cd81faf`

- [x] Scenario: `A Fossil project with uncommitted changes is protected from automatic migration commits` (`apply-a-managed-upgrade`, `@givn.modified` — fixture-only delta)
  **Design constraints:**
  - Delta file: `givn/changes/drop-legacy-residue/specs/upgrade-a-project/apply-a-managed-upgrade.feature`. This section carries only its fixture delta (ordering note).
  - Files: modify `tests/steps/release_0_3_0_e2e_steps.rs` — add `#[given("a Givn 0.7.0 Fossil project with uncommitted user changes")]`. The Then steps already exist; no production code change.
  - Behaviour: `givn upgrade` on a dirty 0.7.0 Fossil project fails before changing project files, creates no revision, and the user changes remain byte-for-byte unchanged.
  - Domain context: `upgrade-a-project` extension 1 and the rule "the managed upgrade and migration bundles run on both supported systems with the same guarantees"; protects the minimal guarantee.
  - Single-scenario run: `GIVN_FEATURES=givn/changes/drop-legacy-residue/specs cargo test --test features_runner -- --name '^A Fossil project with uncommitted changes is protected from automatic migration commits$'`
  - Remove `@wip` from this scenario only at RED.
  - [x] RED: remove `@wip` from this scenario only; run the single-scenario command. The 0.7.0 Given text has no matching step definition → undefined step fails (strict mode). Paste output — MUST exit non-zero:
        ```text
        $ GIVN_FEATURES=givn/changes/drop-legacy-residue/specs cargo test --test features_runner -- --name '^A Fossil project with uncommitted changes is protected from automatic migration commits$'
        Feature: Managed upgrade and legacy project migration
          Scenario: A Fossil project with uncommitted changes is protected from automatic migration commits
           ✘  Given a Givn 0.7.0 Fossil project with uncommitted user changes
              Step failed:
              Defined: givn/changes/drop-legacy-residue/specs/upgrade-a-project/apply-a-managed-upgrade.feature:58:5
              Step doesn't match any function
        [Summary]
        1 feature
        1 scenario (1 failed)
        1 step (1 failed)
        error: test failed (exit status: 1)
        ```
  - [x] GREEN: add the 0.7.0 Fossil Given step definition; the Then steps are reused as-is. Production files modified: `tests/steps/release_0_3_0_e2e_steps.rs`. Targeted re-run — MUST exit 0:
        ```text
        $ cargo check --tests
            Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.84s

        $ GIVN_FEATURES=givn/changes/drop-legacy-residue/specs cargo test --test features_runner -- --name '^A Fossil project with uncommitted changes is protected from automatic migration commits$'
        Feature: Managed upgrade and legacy project migration
          Scenario: A Fossil project with uncommitted changes is protected from automatic migration commits
           ✔  Given a Givn 0.7.0 Fossil project with uncommitted user changes
           ✔  When the consumer invokes `givn upgrade` for migration
           ✔  Then the command fails before changing project files
           ✔  And no revision is created
           ✔  And the user changes remain byte-for-byte unchanged
        [Summary]
        1 feature
        1 scenario (1 passed)
        5 steps (5 passed)
        (exit 0 — Then steps `the command fails before changing project files`,
        `no revision is created`, `the user changes remain byte-for-byte unchanged`
        reused as-is; the 0.7.0 Fossil fixture is the parameterised builder from
        the no-route section, now entering service)
        ```
  - [x] REFACTOR: no cleanup; re-run targeted — MUST exit 0:
        ```text
        $ GIVN_FEATURES=givn/changes/drop-legacy-residue/specs cargo test --test features_runner -- --name '^A Fossil project with uncommitted changes is protected from automatic migration commits$'
        [Summary]
        1 feature
        1 scenario (1 passed)
        5 steps (5 passed)
        (exit 0)

        $ cargo clippy --workspace --all-targets -- -D warnings
            Finished `dev` profile [unoptimized + debuginfo] target(s) in 5.41s
        (exit 0 — the 0.7.0 Fossil builder's temporary dead-code allowance was
        removed with this section; no warnings remain)
        ```
  - [x] Nextest gate evidence (F21): paste the `./run-tests.sh` nextest run showing this scenario passing under the gate protocol:
        ```text
        The full `./run-tests.sh` gate run cancels the tail units on first
        failure (nextest fail-fast), so this unit never runs in it. The same
        gate protocol, filtered to this scenario:
        $ GIVN_FEATURES=givn/changes/drop-legacy-residue/specs GIVN_CUCUMBER_TAGS='' cargo nextest run --no-tests=warn --locked --test features_runner 'upgrade-a-project/apply-a-managed-upgrade.feature::A Fossil project with uncommitted changes is protected from automatic migration commits'
        Nextest run ID fd84f548-d798-49d4-99f6-81f20fbd3419 with nextest profile: default
            Starting 1 test across 1 binary (15 tests skipped)
                PASS [   0.586s] (1/1) givn::features_runner upgrade-a-project/apply-a-managed-upgrade.feature::A Fossil project with uncommitted changes is protected from automatic migration commits
        Summary [   0.586s] 1 test run: 1 passed, 15 skipped
        (exit 0)
        ```
  - [x] COMMIT: `feat(apply-a-managed-upgrade): <release note>` with the scenario title in the message body — Hash: `111bac30ce1c763d1c99b38e2fe1fcf2fa3da7b4`

## E2E setup (before the first @e2e scenario)

- [x] **E2E setup and subset proof.**
  **Design constraints (design.md "E2E smoke test infrastructure" and "Local
  runnability & digital twins"):**
  - Local environment: no server, container, or network. The E2E scenarios
    spawn the real compiled `givn` binary (`CARGO_BIN_EXE_givn`) in temp
    project roots and assert its exit code, stdout, and on-disk effects; the
    external tools are `git`, `fossil`, and `script` (util-linux) on `PATH`
    (already acceptance-suite prerequisites). No digital twin is required.
  - E2E framework and step location: `cucumber-rs` through the repository's
    `features_runner`, reusing the shared `GivnWorld` fixtures. E2E step
    definitions live in the owning capability's existing file
    `tests/steps/release_0_3_0_e2e_steps.rs`; regular steps stay in
    `tests/steps/release_0_3_0_steps.rs`. No new step-definition file is
    created — confirm the two files exist, do not create a change-wide file.
  - E2E runner command: `./run-tests.sh --e2e` (already set as
    `verify.e2e_command`; strict subset of `verify.command` via the `@e2e`
    tag expression). Strict mode is the same `.fail_on_skipped()` builder,
    proven by the setup task's probe.
  - **Subset proof (rule: `verify.e2e_command` must be a strict subset,
    never equal to `verify.command`).** Run `verify.command`
    (`GIVN_FEATURES=givn/changes/drop-legacy-residue/specs ./run-tests.sh`)
    and record the scenario count; run `verify.e2e_command`
    (`GIVN_FEATURES=givn/changes/drop-legacy-residue/specs ./run-tests.sh --e2e`)
    and record its scenario count. The e2e count MUST be strictly less than
    the full count. Paste both counts:
    ```text
    Confirmed in givn/commands.yaml (verify.command: "./run-tests.sh";
    verify.e2e_command: "./run-tests.sh --e2e"). The two strings differ by
    the `--e2e` flag, which selects the `@e2e` tag expression inside
    run-tests.sh. The counts below prove the filter is real.

    Note on scope: the counts are taken against the change's delta specs
    (the design's prescribed GIVN_FEATURES scope). The permanent scope
    (`./run-tests.sh` alone) currently fails mid-change on the pre-merge
    permanent scenarios whose shared Given steps were re-pointed by the
    committed task sections (the two enforce-post-merge-e2e-gate scenarios
    plus the other re-pointed permanent texts) — expected until the archive
    merge replaces them; the delta scope is the only runner input until
    then (design.md "Runner, strict mode, and single-scenario evidence").

    Full run (verify.command protocol, delta scope, fail-fast off so the
    count is real):
    $ GIVN_FEATURES=givn/changes/drop-legacy-residue/specs \
        GIVN_CUCUMBER_TAGS='' cargo nextest run --no-fail-fast \
        --no-tests=warn --locked --test features_runner
        Nextest run ID 94e586ee-… with nextest profile: default
            Starting 16 tests across 1 binary
            ...
                 PASS [   0.544s] (16/16) givn::features_runner verify-change/resolve-change-directories.feature::Verify-e2e hook resolves the change directory from a relative features path inside givn/changes
        ────────────
             Summary [   1.131s] 16 tests run: 10 passed, 6 failed, 0 skipped
        error: test run failed
        (exit 100 — the six failing units are exactly the six still-@wip
         @e2e units of the remaining task sections; the ten completed
         sections pass. Full scope count: 16.)

    E2E run (verify.e2e_command protocol, delta scope, fail-fast off so the
    count is real):
    $ GIVN_FEATURES=givn/changes/drop-legacy-residue/specs \
        GIVN_CUCUMBER_TAGS='@e2e' cargo nextest run --no-fail-fast \
        --no-tests=warn --locked --test features_runner
        Nextest run ID 5c3f1d0f-… with nextest profile: default
            Starting 6 tests across 1 binary (10 tests skipped)
            ...
        ────────────
             Summary [   0.423s] 6 tests run: 0 passed, 6 failed, 10 skipped
        error: test run failed
        (exit 100 — the six @e2e units are still @wip with undefined steps,
         expected pre-GREEN. E2E scope count: 6.)

    6 < 16 — proven. The e2e scope is a strict subset of the full scope.
    ```

  - E2E strictness probe (same procedure as the setup task's probe, against
    the `@e2e` invocation): a temporary `@e2e`-tagged probe scenario whose
    step body uses the not-implemented stub
    (`unimplemented!()`), run through the e2e gate protocol, then removed:
    ```text
    Probe added:
    givn/changes/drop-legacy-residue/specs/00-e2e-strictness-probe/e2e-strictness-probe.feature
        @givn.delta @e2e-strictness-probe
        Feature: E2E runner strictness probe
          @e2e
          Scenario: The e2e runner rejects a not-implemented step
            Given an e2e strictness probe that panics
    tests/steps/e2e_strictness_probe_steps.rs — body `unimplemented!(...)`
    (registered in tests/steps/mod.rs; never an empty body).

    $ GIVN_FEATURES=givn/changes/drop-legacy-residue/specs \
        GIVN_CUCUMBER_TAGS='@e2e' cargo nextest run --no-fail-fast \
        --no-tests=warn --locked --test features_runner
        Nextest run ID b5b2b8cb-aefe-457e-9d08-9424d0205689 with nextest profile: default
            Starting 7 tests across 1 binary (10 tests skipped)
                FAIL [   0.430s] (3/7) givn::features_runner 00-e2e-strictness-probe/e2e-strictness-probe.feature::The e2e runner rejects a not-implemented step
          stdout ───
            Feature: E2E runner strictness probe
              Scenario: The e2e runner rejects a not-implemented step
               ✘  Given an e2e strictness probe that panics
                  Step failed:
                  Defined: givn/changes/drop-legacy-residue/specs/00-e2e-strictness-probe/e2e-strictness-probe.feature:7:5
                  Matched: tests/steps/e2e_strictness_probe_steps.rs:5:1
                  Step panicked. Captured output: not implemented: e2e strictness probe: an unimplemented step must fail the run
            [Summary]
            1 feature
            1 scenario (1 failed)
            1 step (1 failed)
          stderr ───

            thread 'main' (405262) panicked at …/cucumber-0.23.0/src/cucumber.rs:1243:13:
            1 step failed
        error: test run failed
        (exit 100 — the e2e runner rejects the not-implemented step; the
        e2e scope selected 7 units with the probe (6 + probe). Non-zero
        exit proven.)

    Probe removed (feature file, step file, and the mod registration line
    deleted); the e2e run returns to the 6-unit pre-probe state:
    $ GIVN_FEATURES=givn/changes/drop-legacy-residue/specs \
        GIVN_CUCUMBER_TAGS='@e2e' cargo nextest run --no-fail-fast \
        --no-tests=warn --locked --test features_runner
            Starting 6 tests across 1 binary (10 tests skipped)
             Summary [   0.454s] 6 tests run: 0 passed, 6 failed, 10 skipped
        error: test run failed
        (the six failures are the still-@wip @e2e units, the expected
        pre-GREEN mid-change state — baseline restored)
    ```
  - Health check: confirm the e2e runner can reach the local environment —
    the compiled binary is built by the suite (the run-tests.sh
    `ready_fixture` build precedes the suite); a trivial smoke invocation of
    the built binary suffices. Paste the smoke output:
    ```text
    $ cargo build --locked
        Finished `dev` profile [unoptimized + debuginfo] target(s) in 2.28s
    $ ./target/debug/givn instructions --json
    {
      "id": "drop-legacy-residue",
      "next_command": "givn instructions tasks --change drop-legacy-residue",
      ...
    (exit 0 — the compiled binary the @e2e scenarios spawn
     (CARGO_BIN_EXE_givn) is built and answers a trivial invocation from the
     local environment; no server, container, or network is involved.)
    ```
  - COMMIT: `chore(tasks): e2e setup and subset proof` — Hash: `3d15bda91151a686b3725efb70595f2f3dc12d93`

## @e2e scenarios (in .feature order)

- [x] Scenario: `Consumer archives a delta that adds e2e scenarios and is blocked without a configured runner` (`enforce-post-merge-e2e-gate`, `@givn.modified`, `@e2e`)
  **Design constraints:**
  - Delta file: `givn/changes/drop-legacy-residue/specs/archive-change/enforce-post-merge-e2e-gate.feature`. Given text re-pointed: `a committed givn project with verify.e2e_command unconfigured in givn/commands.yaml` (was `givn/config.yaml`); `@e2e` tag kept — never removed.
  - Files: modify `tests/steps/enforce_post_merge_e2e_gate_steps.rs` — rename the Given attribute `a committed givn project with verify.e2e_command unconfigured in givn/config.yaml` (~line 367) to `... in givn/commands.yaml`. The fixture body already writes `givn/commands.yaml`; nothing else changes; no production code change.
  - Behaviour (real CLI boundary): `givn archive --change real-e2e-merge` fails; stderr says the e2e runner is not configured for permanent specs with `@e2e` coverage; the change directory still exists under `givn/changes/`; no archive candidate exists.
  - Transcript evidence: capture `givn/changes/drop-legacy-residue/evidence/visual/consumer-archives-a-delta-that-adds-e2e-scenarios-and-is-blocked-without-a-configured-runner/transcript.txt` — the blocked archive run: scenario title, exact argv, cwd disposition, exit code, and raw stderr naming the unconfigured runner.
  - Domain context: `archive-change` — a failed archive is never reported as proven, and its candidate remains repairable in place.
  - Single-scenario run (selected by unique title): `GIVN_FEATURES=givn/changes/drop-legacy-residue/specs cargo test --test features_runner -- --name '^Consumer archives a delta that adds e2e scenarios and is blocked without a configured runner$'`
  - Remove `@wip` from this scenario only at RED.
  - [x] RED: remove `@wip` from this scenario only; run the single-scenario command. The renamed Given text has no matching step definition → undefined step fails (strict mode). Paste output — MUST exit non-zero:
        ```text
        $ GIVN_FEATURES=givn/changes/drop-legacy-residue/specs cargo test --test features_runner -- --name '^Consumer archives a delta that adds e2e scenarios and is blocked without a configured runner$'
        Feature: Enforce Post-Merge E2E Gate
          Scenario: Consumer archives a delta that adds e2e scenarios and is blocked without a configured runner
           ✘  Given a committed givn project with verify.e2e_command unconfigured in givn/commands.yaml
              Step failed:
              Defined: givn/changes/drop-legacy-residue/specs/archive-change/enforce-post-merge-e2e-gate.feature:31:5
              Step doesn't match any function
        [Summary]
        1 feature
        1 scenario (1 failed)
        1 step (1 failed)
        error: test failed, to rerun pass `--test features_runner`
        (exit status: 1 — undefined step fails under strict mode)
        ```
  - [x] GREEN: apply the step-attribute rename in `tests/steps/enforce_post_merge_e2e_gate_steps.rs` (~line 367). Production files modified: `tests/steps/enforce_post_merge_e2e_gate_steps.rs`. Targeted re-run — MUST exit 0:
        ```text
        $ cargo check --tests
            Checking givn v0.9.0 (/home/buster/.paseo/worktrees/008cxm4u/mellow-yak)
            Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.80s

        $ GIVN_FEATURES=givn/changes/drop-legacy-residue/specs cargo test --test features_runner -- --name '^Consumer archives a delta that adds e2e scenarios and is blocked without a configured runner$'
        Feature: Enforce Post-Merge E2E Gate
          Scenario: Consumer archives a delta that adds e2e scenarios and is blocked without a configured runner
           ✔  Given a committed givn project with verify.e2e_command unconfigured in givn/commands.yaml
           ✔  And a change "real-e2e-merge" with all artifacts done, whose delta spec has an @e2e scenario
           ✔  And the permanent spec directory givn/specs/ has no @e2e scenario
           ✔  When the user invokes `givn archive --change real-e2e-merge`
           ✔  Then the command fails
           ✔  And stderr says the e2e runner is not configured for permanent specs with @e2e coverage
           ✔  And the change directory still exists under givn/changes/
           ✔  And no archive candidate exists for "real-e2e-merge"
        [Summary]
        1 feature
        1 scenario (1 passed)
        8 steps (8 passed)
        (exit 0)
        ```
  - [x] REFACTOR: no cleanup; re-run targeted — MUST exit 0:
        ```text
        $ GIVN_FEATURES=givn/changes/drop-legacy-residue/specs cargo test --test features_runner -- --name '^Consumer archives a delta that adds e2e scenarios and is blocked without a configured runner$'
        [Summary]
        1 feature
        1 scenario (1 passed)
        8 steps (8 passed)
        (exit 0)
        ```
  - [x] Nextest gate evidence (F21): paste the `./run-tests.sh --e2e` nextest run showing this scenario passing under the e2e gate protocol:
        ```text
        $ GIVN_FEATURES=givn/changes/drop-legacy-residue/specs GIVN_CUCUMBER_TAGS='@e2e' cargo nextest run --no-fail-fast --no-tests=warn --locked --test features_runner
            Starting 6 tests across 1 binary (10 tests skipped)
                FAIL ... (the five remaining still-@wip @e2e units of the other task sections, expected mid-change)
                PASS [   0.556s] (6/6) givn::features_runner archive-change/enforce-post-merge-e2e-gate.feature::Consumer archives a delta that adds e2e scenarios and is blocked without a configured runner
             Summary [   0.556s] 6 tests run: 1 passed, 5 failed, 10 skipped
        (exit 100 — the still-@wip sections of the other tasks fail as expected; this scenario PASSes under the e2e gate protocol)
        ```
  - [x] COMMIT: capture the transcript before committing. `test(e2e): <release note>` with the scenario title in the message body — Hash: ``

- [ ] Scenario: `Consumer archives a new capability and the merged spec lands in its declared use case` (`grouped-merge-for-new-capabilities`, `@givn.modified`, `@e2e`)
  **Design constraints:**
  - Delta file: `givn/changes/drop-legacy-residue/specs/archive-change/grouped-merge-for-new-capabilities.feature`. Given text re-pointed: `a givn project with verify.e2e_command unconfigured in givn/commands.yaml` (was `givn/config.yaml`); `@e2e` tag kept — never removed.
  - Files: modify `tests/steps/enforce_post_merge_e2e_gate_steps.rs` — rename the Given attribute `a givn project with verify.e2e_command unconfigured in givn/config.yaml` (~line 5) to `... in givn/commands.yaml` (shared Given; the two sibling scenarios below reuse it). The fixture body already writes `givn/commands.yaml`; nothing else changes; no production code change.
  - Behaviour (real CLI boundary): `givn archive --change grouped-new` exits 0; `givn/specs/some-group/grouped-new.feature` exists; the directory `givn/specs/grouped-new` does not exist; the change directory moves to `givn/archive/`.
  - Transcript evidence: capture `givn/changes/drop-legacy-residue/evidence/visual/consumer-archives-a-new-capability-and-the-merged-spec-lands-in-its-declared-use-case/transcript.txt` — the successful grouped-merge archive run and its merged-tree output.
  - Domain context: `archive-change` — the permanent corpus contains the accepted change, bound to a verification receipt for the revision that passed every verification scope.
  - Single-scenario run: `GIVN_FEATURES=givn/changes/drop-legacy-residue/specs cargo test --test features_runner -- --name '^Consumer archives a new capability and the merged spec lands in its declared use case$'`
  - Remove `@wip` from this scenario only at RED.
  - [ ] RED: remove `@wip` from this scenario only; run the single-scenario command. The renamed Given text has no matching step definition → undefined step fails (strict mode). Paste output — MUST exit non-zero:
        ```text
        <paste targeted run output — expected non-zero exit (undefined step)>
        ```
  - [ ] GREEN: apply the step-attribute rename in `tests/steps/enforce_post_merge_e2e_gate_steps.rs` (~line 5). Production files modified: `tests/steps/enforce_post_merge_e2e_gate_steps.rs`. Targeted re-run — MUST exit 0:
        ```text
        <paste targeted run output — expected exit 0>
        ```
  - [ ] REFACTOR: no cleanup; re-run targeted — MUST exit 0:
        ```text
        <paste targeted run output — expected exit 0>
        ```
  - [ ] Nextest gate evidence (F21): paste the `./run-tests.sh --e2e` nextest run showing this scenario passing under the e2e gate protocol:
        ```text
        <paste nextest e2e gate run output>
        ```
  - [ ] COMMIT: capture the transcript before committing. `test(e2e): <release note>` with the scenario title in the message body — Hash: ``

- [ ] Scenario: `Archive is blocked when a new capability has no declared routing decision` (`grouped-merge-for-new-capabilities`, `@givn.modified`, `@e2e`, reuse)
  **Design constraints:**
  - Delta file: `givn/changes/drop-legacy-residue/specs/archive-change/grouped-merge-for-new-capabilities.feature`. Given text re-pointed to `a givn project with verify.e2e_command unconfigured in givn/commands.yaml` — the exact shared step renamed by the previous scenario's GREEN. No additional file change; no production code change.
  - Behaviour (real CLI boundary): `givn archive --change flat-new` exits non-zero; stderr contains "records no 'NEW in"; the directory `givn/specs/flat-new` does not exist; the change directory still exists under `givn/changes/`.
  - Transcript evidence: capture `givn/changes/drop-legacy-residue/evidence/visual/archive-is-blocked-when-a-new-capability-has-no-declared-routing-decision/transcript.txt` — the blocked archive run and its routing refusal.
  - Domain context: `archive-change` — a failed archive is never reported as proven, and its candidate remains repairable in place.
  - Single-scenario run: `GIVN_FEATURES=givn/changes/drop-legacy-residue/specs cargo test --test features_runner -- --name '^Archive is blocked when a new capability has no declared routing decision$'`
  - Remove `@wip` from this scenario only at RED.
  - [ ] RED: remove `@wip` from this scenario only; run the single-scenario command. All steps reused (shared Given renamed in the previous scenario's GREEN; the `unrouted_new_change` fixture step already exists) → zero exit is legitimate immediate GREEN. Record the reused steps: `a givn project with verify.e2e_command unconfigured in givn/commands.yaml` (renamed in the previous scenario's GREEN) and the pre-existing grouped-merge fixture/archive steps. Paste the run:
        ```text
        <paste targeted run output — zero exit with all steps reused>
        ```
  - [ ] GREEN: no new code; re-run targeted to confirm — MUST exit 0. Production files: none (pure reuse scenario):
        ```text
        <paste targeted run output — expected exit 0>
        ```
  - [ ] REFACTOR: none; re-run targeted — MUST exit 0:
        ```text
        <paste targeted run output — expected exit 0>
        ```
  - [ ] Nextest gate evidence (F21): paste the `./run-tests.sh --e2e` nextest run showing this scenario passing under the e2e gate protocol:
        ```text
        <paste nextest e2e gate run output>
        ```
  - [ ] COMMIT: capture the transcript before committing. `test(e2e): <release note>` with the scenario title in the message body (carries the `@wip` removal, the transcript, and this tasks.md record; no production delta — pure reuse) — Hash: ``

- [ ] Scenario: `Nothing is merged when one delta file of a change has no routing decision` (`grouped-merge-for-new-capabilities`, `@givn.modified`, `@e2e`, reuse)
  **Design constraints:**
  - Delta file: `givn/changes/drop-legacy-residue/specs/archive-change/grouped-merge-for-new-capabilities.feature`. Given text re-pointed to `a givn project with verify.e2e_command unconfigured in givn/commands.yaml` — the exact shared step renamed two scenarios earlier. No additional file change; no production code change.
  - Behaviour (real CLI boundary): `givn archive --change mixed-routing` exits non-zero; stderr contains "records no 'NEW in"; the file `givn/specs/mixed-group/mixed-routing.feature` does not exist; the change directory still exists under `givn/changes/`.
  - Transcript evidence: capture `givn/changes/drop-legacy-residue/evidence/visual/nothing-is-merged-when-one-delta-file-of-a-change-has-no-routing-decision/transcript.txt` — the blocked multi-delta run and the untouched-corpus assertion.
  - Domain context: `archive-change` — a failed archive is never reported as proven, and its candidate remains repairable in place.
  - Single-scenario run: `GIVN_FEATURES=givn/changes/drop-legacy-residue/specs cargo test --test features_runner -- --name '^Nothing is merged when one delta file of a change has no routing decision$'`
  - Remove `@wip` from this scenario only at RED.
  - [ ] RED: remove `@wip` from this scenario only; run the single-scenario command. All steps reused (shared Given renamed in the earlier grouped-merge scenario's GREEN; the `mixed_routing_change` fixture step already exists) → zero exit is legitimate immediate GREEN. Record the reused steps: `a givn project with verify.e2e_command unconfigured in givn/commands.yaml` and the pre-existing grouped-merge fixture/archive steps. Paste the run:
        ```text
        <paste targeted run output — zero exit with all steps reused>
        ```
  - [ ] GREEN: no new code; re-run targeted to confirm — MUST exit 0. Production files: none (pure reuse scenario):
        ```text
        <paste targeted run output — expected exit 0>
        ```
  - [ ] REFACTOR: none; re-run targeted — MUST exit 0:
        ```text
        <paste targeted run output — expected exit 0>
        ```
  - [ ] Nextest gate evidence (F21): paste the `./run-tests.sh --e2e` nextest run showing this scenario passing under the e2e gate protocol:
        ```text
        <paste nextest e2e gate run output>
        ```
  - [ ] COMMIT: capture the transcript before committing. `test(e2e): <release note>` with the scenario title in the message body (carries the `@wip` removal, the transcript, and this tasks.md record; no production delta — pure reuse) — Hash: ``

- [ ] Scenario Outline: `A clean 0.7.0 project receives a reviewable aggregate migration plan` (`apply-a-managed-upgrade`, `@givn.added`, `@e2e` — one section for the whole outline; two Examples rows: `Git` and `Fossil`)
  **Design constraints:**
  - Delta file: `givn/changes/drop-legacy-residue/specs/upgrade-a-project/apply-a-managed-upgrade.feature`. The retired 0.2.0 outline cannot be `@givn.modified` (its title carries the version, and modified replaces by exact title), so the delta removes it and adds this 0.7.0 outline; the merge applies both atomically against the projected state. The catalog deletion already landed in the no-route section; this section carries only its fixture/step deltas (ordering note). The catalog keeps 0.7.0 → 0.8.0 → 0.9.0.
  - Files: modify `tests/steps/release_0_3_0_e2e_steps.rs` — add `#[given("a clean Git project created with Givn 0.7.0")]` and the Fossil 0.7.0 variants; the fixture helpers `prepared_project` and `completed_automatic_migration` move to the 0.7.0 fixture. Modify `tests/steps/release_0_3_0_steps.rs` — `current_version_migration_change_exists` asserts the `migrate-0-7-0-to-` prefix; `migration_contains_plan` asserts `id: migrate-0-7-0-to-0-8-0`.
  - Behaviour (real CLI boundary, both Examples rows in one process): `givn upgrade` on a clean 0.7.0 Git or Fossil project exits successfully; the project config identifies the current Givn version; one commit contains only the managed scaffold and generated agent integration changes; a migration change for the current version exists; the migration change records the managed upgrade commit; it contains a proposal and detailed design; it intentionally has no product specification.
  - Transcript evidence: capture `givn/changes/drop-legacy-residue/evidence/visual/a-clean-0-7-0-project-receives-a-reviewable-aggregate-migration-plan/transcript.txt` — the outline as a whole; the transcript records the Git example's `givn upgrade` run (argv, cwd, exit code, raw stdout/stderr) and the resulting migration change and revision shape.
  - Domain context: `upgrade-a-project` goal — cross a release gap with reviewable upgrade and migration revisions; main flow 5 (the plan revision records the upgrade revision); the rule "the managed upgrade and migration bundles run on both supported systems with the same guarantees".
  - Single-scenario run (selects the outline and runs all examples): `GIVN_FEATURES=givn/changes/drop-legacy-residue/specs cargo test --test features_runner -- --name '^A clean 0.7.0 project receives a reviewable aggregate migration plan$'`
  - Remove `@wip` from this scenario only at RED.
  - [ ] RED: remove `@wip` from this scenario only; run the single-scenario command. The 0.7.0 Given text has no matching step definition → undefined step fails (strict mode). Paste output — MUST exit non-zero:
        ```text
        <paste targeted run output — expected non-zero exit (undefined step)>
        ```
  - [ ] GREEN: add the 0.7.0 Git/Fossil fixture steps and move `prepared_project` / `completed_automatic_migration` to the 0.7.0 fixture; apply the `current_version_migration_change_exists` and `migration_contains_plan` assertion updates. Production files modified: `tests/steps/release_0_3_0_e2e_steps.rs`, `tests/steps/release_0_3_0_steps.rs`. Targeted re-run (both Examples rows) — MUST exit 0:
        ```text
        <paste targeted run output — expected exit 0, both examples>
        ```
  - [ ] REFACTOR: no cleanup; re-run targeted — MUST exit 0:
        ```text
        <paste targeted run output — expected exit 0, both examples>
        ```
  - [ ] Nextest gate evidence (F21): paste the `./run-tests.sh --e2e` nextest run showing this outline passing under the e2e gate protocol:
        ```text
        <paste nextest e2e gate run output>
        ```
  - [ ] COMMIT: capture the transcript before committing. `test(e2e): <release note>` with the scenario title in the message body — Hash: ``

- [ ] Scenario: `A consumer invoking the retired history rewrite command is told it does not exist` (`apply-a-managed-upgrade`, `@givn.added`, `@e2e` — carries the rewrite-history production deletions)
  **Design constraints:**
  - Delta file: `givn/changes/drop-legacy-residue/specs/upgrade-a-project/apply-a-managed-upgrade.feature`. Mirrors the retired `givn spec index` precedent in the same capability (D3): each retired command keeps its own refusal evidence so a re-added subcommand cannot hide behind the other's scenario.
  - **This is the design's "final implementation task before archive" — it carries the rewrite-history production deletions (sequencing note).**
  - Files: delete `src/cli/rewrite_history.rs` (the module, 107 lines); delete `pub mod rewrite_history;` in `src/cli/mod.rs`; delete the `RewriteHistory { yes }` clap variant (with its doc comment) and the `Commands::RewriteHistory` dispatch arm in `src/main.rs`. Delete the two Then steps orphaned by the retirement (F14) in `tests/steps/local_only_rules_steps.rs`: `the reachable history contains "…"` (~line 274) and `the reachable history does not contain "…"` (~line 284) — verified against the active corpus: only the retired capability's scenarios use either; the remaining `local_only_rules_steps.rs` steps stay (they serve the report-only `rules-policy` / `publication-cleanliness` scenarios).
  - Fixtures: in `tests/steps/release_0_3_0_steps.rs`, add `the consumer invokes the retired `givn rewrite-history` command` and `the error reports "rewrite-history" is not an available command` beside the existing retired-engine-command steps (~lines 322/333).
  - Behaviour (real CLI boundary — the scenario spawns `CARGO_BIN_EXE_givn`): clap refuses the removed command by name; exit non-zero; stderr reports "rewrite-history" is not an available command.
  - Sequencing note: until the archive merge, the permanent `history-rewrite` feature still exists; the change-scoped gates run only the delta (`GIVN_FEATURES`), and the archive's verification scopes run the merged corpus where the feature is already gone — no interval executes the permanent rewrite-history scenarios against the deleted glue.
  - Transcript evidence: capture `givn/changes/drop-legacy-residue/evidence/visual/a-consumer-invoking-the-retired-history-rewrite-command-is-told-it-does-not-exist/transcript.txt` — the compiled binary's `givn rewrite-history` invocation, exit code, and raw stderr.
  - Domain context: `upgrade-a-project` main flow 6 / extension 6 — the system names any command a release removed instead of leaving a silent gap; a consumer invoking a command the release removed is told it is unavailable; protects the success guarantee.
  - Single-scenario run: `GIVN_FEATURES=givn/changes/drop-legacy-residue/specs cargo test --test features_runner -- --name '^A consumer invoking the retired history rewrite command is told it does not exist$'`
  - Remove `@wip` from this scenario only at RED.
  - [ ] RED: remove `@wip` from this scenario only; run the single-scenario command. The new When/And steps have no matching step definitions → undefined steps fail (strict mode). Paste output — MUST exit non-zero:
        ```text
        <paste targeted run output — expected non-zero exit (undefined step)>
        ```
  - [ ] GREEN: add the two retired-command steps in `tests/steps/release_0_3_0_steps.rs`; delete `src/cli/rewrite_history.rs`, the `pub mod rewrite_history;` line, the clap variant, and the dispatch arm; delete the two orphaned reachable-history steps in `tests/steps/local_only_rules_steps.rs`. Production files modified: `src/main.rs`, `src/cli/mod.rs`, `src/cli/rewrite_history.rs` (deleted), `tests/steps/release_0_3_0_steps.rs`, `tests/steps/local_only_rules_steps.rs`. Targeted re-run — MUST exit 0:
        ```text
        <paste targeted run output — expected exit 0>
        ```
  - [ ] REFACTOR: no cleanup beyond the deletions; re-run targeted — MUST exit 0:
        ```text
        <paste targeted run output — expected exit 0>
        ```
  - [ ] Nextest gate evidence (F21): paste the `./run-tests.sh --e2e` nextest run showing this scenario passing under the e2e gate protocol:
        ```text
        <paste nextest e2e gate run output>
        ```
  - [ ] COMMIT: capture the transcript before committing. `test(e2e): <release note>` with the scenario title in the message body — Hash: ``

## Residue sweeps (non-scenario edit map)

Each sweep is one atomic commit. Design constraints are copied from
`design.md`'s exact-file-changes tables; do not extend the file lists.

- [ ] **Residue sweep: stale documentation corrections and dead guide removal.**
  **Design constraints (design.md "Assets, migration catalog, and docs" table):**
  - `README.md`: remove the semantic-duplicate-review clause from "Blocking gates"; replace the stale "Configurable without forking" build text (one feature-free binary, `[features] default = []`); correct the "Runner-agnostic" skill formats (`agents` neutral plus `claude`, `opencode`, `omp`); fix the install block comments; fix the `docs/workflow.md` bullet ("how semantic search assures deduplication" goes); drop `extras` from the reference bullet; delete the "Enforcement note for agents" section (the semantic engine is gone and its referenced chapter section no longer exists).
  - `docs/reference.md`: drop the retrieval-aware clause from the `givn-spec` row; "Enables all addons and extras by default." → "Enables all addons by default."; fold the four former extras into the addons table (`domain_modeling`, `dev_principles`, `grill_me`, `grill_with_docs`); delete the `## Extras` section and the `--no-extras` / `givn extras install` references; `addons:` / `extras:` → `addons:`; drop `extras` from the document intro; split the Configuration section by owning file — `givn/config.yaml` is the structural scaffold (generated fully-commented by `givn init`; commented lines are the managed documentation scaffold refreshed by `givn upgrade`; uncommented lines are user-owned; carries `givn_config_version`), `givn/commands.yaml` is the project-owned command file (test runner, coverage commands, run declaration, applied as the later overlay; the first value a working project must set is `verify.command` there) — and fix the project-layout comment (`givn/config.yaml` → `# documented + versioned: overlay, addons, rules, extensions`; `givn/commands.yaml` → `# project-owned verify, coverage, and run values`).
  - `docs/workflow.md`: delete the advisory row "missing index notice" (no such hint or command); replace the archive chokepoint's "fresh similarity gate" with the real gates (configured hooks, run-declaration gate, README completeness gate, use-case/delta prevalidation, E2E-runner configuration check, verification scopes).
  - `docs/migrations/README.md`: delete the retrieval-installation paragraph; rewrite the placeholder paragraph (no retained asset uses `{{MIGRATION_GUIDE}}`; the three generated-metadata placeholders stay); state the oldest declared route (`0.7.0`) and the no-route refresh behaviour.
  - `docs/INDEX.md`: remove the deleted 0.2.0 and 0.4.0 guide entries; add the `0.7.0-to-0.8.0` guide entry; drop `extras` from the reference description and `retrieval` from the chapter-08 description.
  - `CHANGELOG.md` — exactly one edit (Q2, option B): in the released `## [0.3.0]` Added section, the bullet "Added the LLM migration playbook at [`docs/migrations/0.2.0-to-0.3.0.md`](docs/migrations/0.2.0-to-0.3.0.md)." keeps the `0.2.0-to-0.3.0` string and drops the dead link target — the text becomes plain `` `docs/migrations/0.2.0-to-0.3.0.md` ``. The scenario "The changelog describes the complete 0.3.0 release" stays green (its step asserts the string, not the link). The later prose code block naming the same path is already plain text and is untouched; no other changelog line changes — the release-record principle stays intact.
  - Delete `docs/migrations/0.2.0-to-0.3.0.md` and `docs/migrations/0.4.0-to-0.5.0.md`.
  - `src/cli/migration.rs` (compile-coupled with the 0.2.0 guide deletion — same commit): delete `const MIGRATION_GUIDE` (the `include_str!("../../docs/migrations/0.2.0-to-0.3.0.md")` at line 11) and the `.replace("{{MIGRATION_GUIDE}}", MIGRATION_GUIDE)` substitution — no retained asset carries that placeholder.
  - Files: `README.md`, `docs/reference.md`, `docs/workflow.md`, `docs/migrations/README.md`, `docs/INDEX.md`, `CHANGELOG.md`, `src/cli/migration.rs` (const + substitution only), deletions `docs/migrations/0.2.0-to-0.3.0.md` + `docs/migrations/0.4.0-to-0.5.0.md`.
  - Verification: no scenario pins this prose; re-run `./run-tests.sh` over the change's specs to confirm the suite still passes (the 0.7.0 guide steps assert the retained guide, untouched here):
    ```text
    <paste run output>
    ```
  - COMMIT: `docs(migration): correct the front-door documentation and remove the dead guides` — Hash: ``

- [ ] **Residue sweep: served-guidance corrections.**
  **Design constraints (design.md "Served guidance assets" table):**
  - Every hit is corrected to name the file that actually owns the value — the runner commands live in `givn/commands.yaml`, applied as the later overlay; addons, skill targets, extensions, and the rules policy live in `givn/config.yaml`. `assets/commands/` and `assets/skills/` carry no hit; `assets/templates/design-review.md`'s `addons.arc42` mention is correct and stays; `assets/commands/grill-with-docs.md` carries only the trigger, so it needs no edit.
  - `assets/instructions/grill-with-docs.md:26` — split the inventory by file: `givn/commands.yaml` (verify.command, verify.e2e_command, features_path, coverage, run) and `givn/config.yaml` (addons, skills.targets, extensions, rules).
  - `assets/instructions/grill-with-docs.md:33-35` — replace the two `set_extras` / `extras` drift examples with the current-vocabulary examples from design.md ("tasks.md says the flow is write `givn/config.yaml` -> `givn skills sync` -> commit, but design.md names `givn/commands.yaml` as the home of the runner values..." and "The spec asserts `rules: versioned`, but the config scaffold ships the key commented and an absent value resolves to `local-only`...").
  - `assets/instructions/review.md:54`, `:70`, `:381` — the three "Read ... from `givn/config.yaml`" attributions become `givn/commands.yaml`.
  - `assets/instructions/specs.md:131` — "`verify.e2e_command` (configured in `givn/config.yaml`)" → "(configured in `givn/commands.yaml`)".
  - `assets/instructions/tasks.md:31` — "Set `verify.command` in `givn/config.yaml`" → "in `givn/commands.yaml`".
  - `assets/templates/design.md:75` — "Test runner command (from `verify.command` in givn/config.yaml)" → "(... in givn/commands.yaml)".
  - `assets/templates/review.md:71` — "`verify.command` and `verify.e2e_command` read literally from `givn/config.yaml`" → "from `givn/commands.yaml`".
  - Files: the eight files above only. The generated copies in this repository's agent targets refresh from these assets through the normal dogfood `givn skills sync` after archive — do not hand-edit generated copies here.
  - Verification: no delta scenario pins the corrected prose (served-guidance spec coverage audit); the reference-resolution scenarios stay green either way. Re-run `./run-tests.sh` over the change's specs:
    ```text
    <paste run output>
    ```
  - COMMIT: `docs(guidance): correct the served-guidance configuration attributions` — Hash: ``

- [ ] **Residue sweep: configuration scaffold corrections.**
  **Design constraints (design.md "Configuration scaffold and reference text" section):**
  - `assets/config.yaml.tmpl`:
    - Addons block: extend the commented example so all eight flags are nested under `addons:` (the exact eight-line block from design.md, ending with `# Toggle with: givn addons enable|disable <name>, or` / `# `givn addons list` to see all known addons.`); then delete the four trailing top-level example lines from the per-addon sections — `# domain_modeling: false`, `# dev_principles: false`, `# grill_me: false`, `# grill_with_docs: false` (each followed by `# set to true to enable`) — and keep each section's description and `Toggle with: givn addons enable|disable <name>` line. An uncommented top-level key is ignored by the lenient overlay, so the old shape taught a setting that does nothing; the main block is now the one canonical example.
    - arc42 block: replace the fake `arc42: output_dir/chapters` snippet with the fixed-path truth (design.md's exact block: durable arc42 chapters live at the fixed path `docs/arc42/` using the embedded twelve-chapter set; there is no output-dir or chapter-list setting; when `addons.arc42` is true the document is produced during the design phase and checked in design-review; an enabled-but-unfinished arc42 doc blocks `givn archive`), keeping the design-phase and archive-gate paragraph.
    - Header URL: `#  givn configuration  —  https://github.com/TBD/givn` → `#  givn configuration  —  https://github.com/buster/givn` (matches `Cargo.toml`'s `repository`).
    - Hooks block: replace the stale post-default claim and relabel the examples (design.md's exact block: a `post:` list is purely additive — it adds user hooks and never replaces the mandatory checks the binary owns: `givn check tasks` -> WIP protection and the test runner (verify); `givn check review` -> integrity, the run-declaration check, and the overlap dispositions; `givn archive` -> both verification scopes and the coverage gate when enabled); rename `# Override examples:` to `# Additive examples:`; change the tasks example to `post: [lint]` (the mandatory check already runs `verify` for tasks); the review example (`verify` with a display name, plus a custom script with `allow_failure: true`) stays.
  - `assets/commands.yaml.tmpl` header: replace the undefined "harness evaluator" reader with `` #  `givn check`, `givn archive`, `givn run`, `givn upgrade`, and `givn status` `` / `` #  read from here. `` (the following "`givn/config.yaml` stores project structure config; test commands live HERE." line is already accurate and stays).
  - Pinning result: no spec scenario pins the template's or the reference's text; the unit tests `config_template_keys_match_manifest_schema` (asserts the key names `verify`, `apply`, `artifacts`, `addons`, `coverage`, `givn_config_version` — all survive) and `run_scaffold_is_embedded_verbatim` (the header edit leaves the embedded `run.yaml.tmpl` copy byte-identical) need no update — re-verify at implementation. This repository's own `givn/config.yaml` is user-owned and is not regenerated by this change; the scaffold edit also repays arc42 debt D-3 (chapter 11).
  - Files: `assets/config.yaml.tmpl`, `assets/commands.yaml.tmpl` only.
  - Verification: `cargo test` for the two named unit tests plus `./run-tests.sh` over the change's specs:
    ```text
    <paste run output>
    ```
  - COMMIT: `docs(config): correct the configuration scaffold text` — Hash: ``

- [ ] **Residue sweep: AGENTS.md and CI prerequisite corrections.**
  **Design constraints (design.md "Assets, migration catalog, and docs" table):**
  - `AGENTS.md`: acceptance prerequisites — drop `git-filter-repo` from the required-tools list and drop the rewrite-history clause from the acceptance-suite sentence (no test needs either after the command is gone).
  - `.github/workflows/ci.yml` and `.github/workflows/release.yml`: drop `git-filter-repo` from both apt install lists (`sudo apt-get install -y --no-install-recommends fossil git-filter-repo` → `fossil` only).
  - Files: `AGENTS.md`, `.github/workflows/ci.yml`, `.github/workflows/release.yml` only.
  - COMMIT: `chore(ci): drop the rewrite-history acceptance prerequisite` — Hash: ``

- [ ] **Residue sweep: remove the dead `extras` flag.**
  **Design constraints (design.md production table, D1):**
  - `src/config/manifest.rs`: delete `Manifest.extras`; `Manifest` keeps `deny_unknown_fields`.
  - `src/config/overlay.rs`: delete the `overlay.extras` merge and the `OverlayDoc.extras` field; `OverlayDoc` stays lenient — which is what keeps a stale `extras:` line harmless (D1: a consumer config that still carries the retired `extras:` key is accepted and ignored; the overlay document is lenient by design, only the `coverage:` block carries `deny_unknown_fields`; no new validation code).
  - `src/cli/graph.rs`: delete `extras: false` from the test manifest constructor.
  - `src/hooks.rs`: delete `extras: false` from the test manifest constructor.
  - `tests/steps/mod.rs`: delete the `the effective manifest has extras enabled` step (`manifest_extras_enabled`, ~line 1291), which reads the removed field; rename the section comment `// --- Steps for install-extras ---` (~line 882) to a current label (for example `// --- Steps for optional addons ---`).
  - Config-refusal contract: `extras: true` in `givn/config.yaml` (or `givn/commands.yaml`) is parsed as an unknown overlay key and ignored; every command behaves exactly as before the field existed; `givn upgrade` preserves the line as a user-owned value.
  - Files: `src/config/manifest.rs`, `src/config/overlay.rs`, `src/cli/graph.rs`, `src/hooks.rs`, `tests/steps/mod.rs`.
  - Verification: the remaining overlay/module-boundary steps and the manifest unit tests stay green; re-run `./run-tests.sh` over the change's specs:
    ```text
    <paste run output>
    ```
  - COMMIT: `feat(config): remove the dead extras manifest flag` — Hash: ``

- [ ] **Residue sweep: remove the legacy coverage schema and the in-tool merge.**
  **Design constraints (design.md production table, D2):**
  - `src/config/manifest.rs`: delete `Coverage.non_e2e_command` / `Coverage.e2e_command` and their `Default` initializers; delete the `legacy_coverage_commands_are_accepted_during_migration` unit test. `Coverage` keeps `#[serde(deny_unknown_fields)]` — which is what makes a stale legacy coverage block a hard load error (D2: the serde error already names the unknown field and the expected keys; no custom message is added; `givn upgrade` cannot rewrite the block because the strict parse refuses the file first, so the repair is a manual edit to declare `measure_command` / `merge_command`).
  - `src/cli/coverage_gate.rs`: delete the `legacy_commands` probe, the legacy command-execution branch, `write_legacy_merged_report`, and `remove_legacy_coverage_badges`. The gate keeps: measure → parse/validate both sources → `assert_distinct` → run `merge_command` → validate the merged report → badges and README summary. The README summary now always names `coverage.measure_command` and `coverage.merge_command` (drop the legacy `unwrap_or` fallbacks).
  - `src/cobertura.rs`: delete `merge_legacy_reports`, the `CoberturaReport.line_hits` field, `CoberturaXml::line_hits()`, `CoberturaLine.number`, and the two tests that exercise the legacy merge. Keep `file_lines`, `parse`, `validate`, `assert_distinct`, `render_*`.
  - `src/integrity_hook.rs`: in `check_coverage_config`, delete the `legacy_configured` acceptance and the "Legacy projects may temporarily keep…" sentence; the refusal keeps naming `coverage.measure_command` / `coverage.merge_command`.
  - `tests/steps/extract_coverage_gate_steps.rs`: delete `legacy_coverage_project`, `fixture_report_xml`, `legacy_coverage_same_lines`, `legacy_coverage_disjoint_lines`, `merged_report_has_line_counters`, and `make_executable` (used only by the legacy fixture).
  - `src/cli/readme.rs` stays **unchanged** — `LEGACY_TEXTS` is current README-enforcement behaviour (the reviewed out-of-scope item).
  - The surviving archive-coverage scenarios keep the working project-owned measurement/merge path pinned; the removed scenarios go with the removed branch (their `@givn.removed` bookkeeping is in the final task).
  - Files: `src/config/manifest.rs`, `src/cli/coverage_gate.rs`, `src/cobertura.rs`, `src/integrity_hook.rs`, `tests/steps/extract_coverage_gate_steps.rs`.
  - Verification: `cargo test` for the surviving coverage/cobertura unit tests plus `./run-tests.sh` over the change's specs (the surviving extract-coverage-gate scenarios stay green):
    ```text
    <paste run output>
    ```
  - COMMIT: `feat(coverage): remove the legacy two-command coverage schema and in-tool merge` — Hash: ``

- [ ] **Residue sweep: remove the legacy attribution-trailer prefixes.**
  **Design constraints (design.md production table):**
  - `src/cli/attribution.rs`: delete `LEGACY_TRAILER_PREFIXES` and the prefix branch in `is_managed_trailer`; `stamp_text` keeps replacing the managed `Co-Authored-By: … with Givn …` line. The legacy `Givn-Version:`, `LLM:`, `LLM-Reasoning:`, `LLM-Harness:`, and `Givn-Attribution-Status:` prefixes are no longer stripped; an existing message that carries them keeps them (they are ordinary body lines now).
  - Update the unit test: drop the legacy trailer lines from the fixture and the assertions that they disappear; keep the human `Co-Authored-By`, the `User-Trailer: keep` line, and the managed-line replacement assertions.
  - Files: `src/cli/attribution.rs` only.
  - Verification: `cargo test` for the attribution unit test:
    ```text
    <paste run output>
    ```
  - COMMIT: `feat(attribution): stamp only the managed co-author line` — Hash: ``

## Final bookkeeping: merge records and the retirement tombstone

- [ ] **Bookkeeping: `@givn.removed` merge records, the `@givn.retired` tombstone, and the narrative delta.** No code work beyond the merge itself.
  **Design constraints (design.md "Delta map and merge semantics", "Retirement mechanics"):**
  - The four `@givn.removed` placeholders are merge instructions (the runner filters them out via `is_removed_placeholder`). Verify each title resolves against the permanent corpus (the merge deletes by exact title):
    - `A line measured by both reports is counted once in the merged report` (`givn/specs/archive-change/extract-coverage-gate.feature:21`)
    - `A line only one report measures counts once and uncovered lines are not dropped` (`givn/specs/archive-change/extract-coverage-gate.feature:29`)
    - `Extras flag is propagated from overlay` (`givn/specs/fragments/customize-the-lifecycle-graph.feature:43`)
    - `A clean 0.2.0 project receives a reviewable aggregate migration plan` (`givn/specs/upgrade-a-project/apply-a-managed-upgrade.feature:4` — the outline; removed rather than modified because its title carries the version; the merge applies the removal and the 0.7.0 outline addition atomically against the projected state)
  - The tombstone `givn/changes/drop-legacy-residue/specs/rewrite-history/history-rewrite.feature` is two lines, zero scenarios, capability tag matching the permanent capability. At archive, `process_delta_file` removes `givn/specs/rewrite-history/history-rewrite.feature` and appends `history-rewrite / retired_by_change: drop-legacy-residue` to `givn/specs/.retired.yaml`; the extended retirement (implemented in the retirement scenario section) then removes the use-case directory and its narrative. `review.md` must contain `RETIRE: history-rewrite — <reason>` — the retirement disclosure gate blocks the archive without it.
  - The narrative delta `givn/changes/drop-legacy-residue/specs/upgrade-a-project/usecase.md` is a full narrative replacement carrying the base SHA-256 line (`givn:base-sha256:52b61b2d64f47251cd0827f3a6df8d19156cae1d0c4f3e878bc1eec2ec9cbd40`); it replaces the 0.2.0 plan row with the 0.7.0 title and adds the retired-history-rewrite interaction row. Verify it is in place; no implementation edit.
  - Baseline-findings note (F3/F4): default-scope `givn lint` exits 3 while the `rewrite-history` directory exists (the dangling `Includes: fragment: rules-policy` reference and the interaction-inventory `@e2e` mismatch) and returns to 0 only when the archive merge removes the directory — expected through implementation and until archive.
  - COMMIT: `chore(bookkeeping): verify the delta merge records resolve` — Hash: ``
