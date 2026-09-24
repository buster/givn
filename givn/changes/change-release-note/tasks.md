# Tasks: change-release-note

RED→GREEN→REFACTOR→COMMIT per scenario. All three scenarios are `@e2e`
(the delta has no non-@e2e scenario), so the loop runs the E2E runner.

Legend: `[ ]` pending · `[x]` done only when evidence + commit hash are filled in. Check boxes immediately after each scenario — tasks.md must always reflect true state.

## Domain Constraints

Owning use case: `create-change`. Confirmed Personas: `developer--primary-user`,
`agent--technical-role`, `givn-maintainer--owner`. Ideation source: none.

| Scenario / capability | Use-case guarantee or rule | Persona impact |
|---|---|---|
| A new change's proposal carries the release-note field | A change's proposal carries one release note; generated guidance is synchronized from canonical source assets. | developer--primary-user authors it; CLI text output. |
| The proposal guidance requires a release note | One canonical policy exists per topic; every revision the change records shares the release note as the message subject, with the scenario title in the message body. | agent--technical-role reads the guidance; developer--primary-user reads the instruction. |
| The generated implementation guidance shares the release note | Generated guidance is synchronized from canonical source assets; one canonical policy per topic, derived guidance points at it. | agent--technical-role follows the generated guidance; givn-maintainer--owner keeps one canonical wording. |

Every task below carries these constraints. The `.feature` file remains the
executable authority.

---

## Setup — runner and strict-mode proof

- [x] **Runner check.** `verify.command` is `./run-tests.sh`; it executes the
      change's `.feature` files through the cucumber-rs runner
      (`tests/features_runner.rs`) against
      `GIVN_FEATURES=givn/changes/change-release-note/specs`. No runner
      configuration change is needed. `givn/commands.yaml`:
      `verify.command: ./run-tests.sh`, `verify.e2e_command: ./run-tests.sh --e2e`.
- [x] **No new step skeleton.** Design's Step Definitions table: all three
      scenarios reuse existing bindings; no file is added (one-file-per-
      capability creates a file only when a capability adds a binding).
- [x] **Proof-of-strictness.** Run `./run-tests.sh` against a temporary
      feature whose only step is undefined; the runner must exit non-zero via
      `.fail_on_skipped()` (`tests/features_runner.rs`).
      ```
      $ printf 'Feature: strict proof\n\n  Scenario: An undefined step must fail\n    Given a step that no definition binds\n' > /tmp/opencode/strict-proof/strict-proof.feature
      $ GIVN_TEST_RUNNER=cargo GIVN_FEATURES=/tmp/opencode/strict-proof ./run-tests.sh --non-e2e
      Feature: strict proof
        Scenario: An undefined step must fail
         ✘  Given a step that no definition binds
            Step failed:
            Defined: tmp/opencode/strict-proof/strict-proof.feature:4:5
            Step doesn't match any function
      [Summary]
      1 feature
      1 scenario (1 failed)
      1 step (1 failed)
      error: test failed, to rerun pass `--test features_runner`
      exit=1
      ```
      Exit non-zero: strict mode rejects an undefined step.
- [x] **Plan commit.** Commit the planning artifacts before implementation so
      each scenario commit stays atomic:
      `givn commit -m "docs(change-release-note): Give every change one release note, shared as the subject of all its revisions"`.
      Hash: `a62ec8b51a01b3bb69076461c54c9da1edf34c69`

---

## E2E setup — runner, filter proof, environment

- [x] **Environment.** None required: the real interface is the `givn` CLI and
      there is no browser, server, database, queue, or digital twin
      (design's Local Runnability section). Confirm clean startup by running
      the suite.
- [x] **E2E command.** `verify.e2e_command` is `./run-tests.sh --e2e`,
      tag-filtered to `@e2e` (`GIVN_CUCUMBER_TAGS=@e2e`), distinct from
      `verify.command`.
- [x] **Filter proof.** Delta scope, both commands, before implementation
      (RED expected — 3 scenarios, 0 passed, 3 failed):
      ```
      $ GIVN_FEATURES=givn/changes/change-release-note/specs ./run-tests.sh
      Summary [0.414s] 3 tests run: 0 passed, 3 failed, 0 skipped
      $ GIVN_FEATURES=givn/changes/change-release-note/specs ./run-tests.sh --e2e
      Summary [0.420s] 3 tests run: 0 passed, 3 failed, 0 skipped
      ```
      All three delta scenarios are tagged `@e2e`, so both scopes run the same
      three in this change. The filter is proven against the permanent scope:
      `all=563 e2e=88 non-e2e=475` (nextest list with and without
      `GIVN_CUCUMBER_TAGS`), so the `@e2e` subset is strictly smaller and
      `verify.e2e_command` is not a re-run of the full suite.
      ```
      $ ALL=$(cargo nextest list --test features_runner | grep -c '\.feature::')
      $ E2E=$(GIVN_CUCUMBER_TAGS='@e2e' cargo nextest list --test features_runner | grep -c '\.feature::')
      all=563 e2e=88
      ```

---

## Task 1 — Scenario: A new change's proposal carries the release-note field

- [x] RED: remove `@wip` from THIS scenario only. Run the single-scenario
      command. Must fail on the missing scaffold wording.
      ```
      GIVN_FEATURES=givn/changes/change-release-note/specs cargo test --test features_runner -- --exact "create-change/change-release-note.feature::A new change's proposal carries the release-note field"
      ```
      ```
        Scenario: A new change's proposal carries the release-note field
         ✔  Given a project root with givn/ initialised
         ✔  When the user runs `givn new shared-note`
         ✔  Then the command exits 0
         ✘  And the file "givn/changes/shared-note/proposal.md" contains "release note"
            Step failed:
          ...
        [Summary]
        1 feature
        1 scenario (1 failed)
        4 steps (3 passed, 1 failed)
        exit=101
      ```
- [x] GREEN: edit `assets/templates/proposal.md` — add the `## Release note`
      section (heading only; guidance quote carries the lowercase phrase).
      Files created/modified:
      `assets/templates/proposal.md`.
      Run the same targeted command. Must pass.
      ```
        ✔  Given a project root with givn/ initialised
        ✔  When the user runs `givn new shared-note`
        ✔  Then the command exits 0
        ✔  And the file "givn/changes/shared-note/proposal.md" contains "release note"
      [Summary]
      1 feature
      1 scenario (1 passed)
      4 steps (4 passed)
      exit=0
      ```
- [x] REFACTOR: no behaviour change; re-run targeted. Must still pass.
      ```
      [Summary]
      1 feature
      1 scenario (1 passed)
      4 steps (4 passed)
      exit=0
      ```
- [x] COMMIT: `7b253d7d7977dc86f048f875652a69d5031cc8fa` —
      ```
      test(e2e): Give every change one release note, shared as the subject of all its revisions

      Scenario: A new change's proposal carries the release-note field
      ```

---

## Task 2 — Scenario: The proposal guidance requires a release note

- [ ] RED: remove `@wip` from THIS scenario only. Run the single-scenario
      command. Must fail on the missing instruction wording.
      ```
      GIVN_FEATURES=givn/changes/change-release-note/specs cargo test --test features_runner -- --exact "create-change/change-release-note.feature::The proposal guidance requires a release note"
      ```
      ```
      <!-- paste RED evidence -->
      ```
- [ ] GREEN: edit `assets/instructions/proposal.md` — required release-note
      rule plus the shared-subject sentence; structure list gains the release
      note. Files created/modified: `assets/instructions/proposal.md`.
      Run the same targeted command. Must pass.
      ```
      <!-- paste GREEN evidence -->
      ```
- [ ] REFACTOR: no behaviour change; re-run targeted. Must still pass.
      ```
      <!-- paste REFACTOR evidence -->
      ```
- [ ] COMMIT: `<!-- hash -->` —
      ```
      test(e2e): Give every change one release note, shared as the subject of all its revisions

      Scenario: The proposal guidance requires a release note
      ```

---

## Task 3 — Scenario: The generated implementation guidance shares the release note

- [ ] RED: remove `@wip` from THIS scenario only. Run the single-scenario
      command. Must fail on the missing generated-guidance wording.
      ```
      GIVN_FEATURES=givn/changes/change-release-note/specs cargo test --test features_runner -- --exact "create-change/change-release-note.feature::The generated implementation guidance shares the release note"
      ```
      ```
      <!-- paste RED evidence -->
      ```
- [ ] GREEN: edit the generated-guidance sources and align the durable docs:
      - Skills: `givn-propose`, `givn-steps`, `givn-implement`, `givn-tasks`.
      - Commands: `givn-propose`, `givn-steps`, `givn-implement`, `givn-tasks`,
        `givn-characterize`, `givn-reverse-engineer`.
      - Instructions: `assets/instructions/tasks.md`,
        `assets/instructions/tasks-reverse.md`.
      - AGENTS.md hint: `assets/agents-project-hint.md`.
      - Durable docs: `README.md`, `docs/workflow.md`.
      Every canonical phrase stays on a single unbroken line. Then self-dogfood:
      `cargo install --path . --locked --force` and `givn skills sync` (updates
      the tracked generated rules and the `AGENTS.md` block; `AGENTS.md` is
      staged by an explicit `git add AGENTS.md`, the developer's act).
      Run the same targeted command. Must pass.
      Files created/modified: `<!-- list them -->`
      ```
      <!-- paste GREEN evidence -->
      ```
- [ ] REFACTOR: grep every changed surface for the superseded phrase
      `message references the scenario title`; confirm zero hits. Re-run
      targeted. Must still pass.
      ```
      <!-- paste REFACTOR evidence -->
      ```
- [ ] COMMIT: `<!-- hash -->` —
      ```
      test(e2e): Give every change one release note, shared as the subject of all its revisions

      Scenario: The generated implementation guidance shares the release note
      ```

---

## Done

- [ ] Full regular suite: `./run-tests.sh` exits 0.
- [ ] Full E2E suite: `./run-tests.sh --e2e` exits 0.
- [ ] Both counts pasted in the E2E setup task; every scenario has a commit.
- [ ] `givn status --change change-release-note` shows implementation complete
      and next required `review`.
