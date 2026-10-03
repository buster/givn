# Tasks: dialog-question-quality

Runner: `./run-tests.sh` (regular) / `./run-tests.sh --e2e` (e2e). Strict mode:
`tests/features_runner.rs` uses `.fail_on_skipped()` (cucumber-rs) — undefined
steps fail. Not-implemented stub: `unimplemented!()`.

Single-scenario run command (all scenarios):

```
GIVN_FEATURES=givn/changes/dialog-question-quality/specs cargo test --test features_runner -- --name "<scenario title>"
```

Use-case constraints carried into every scenario task:

- `author-change`: generated guidance is synchronized from canonical source
  assets; the change scaffold and its guidance must stay consistent.
- `author-change` rule: generated guidance may name only commands that work in
  the active project; the new policy command must be served by the binary.
- Support-fragment capabilities (`instruction-integrity`, `ideation-skill`)
  are reusable behavior consumed by the use cases; they carry no independent
  user goal and no interaction inventory.
- Confirmed Personas: `agent--technical-role` (generated guidance and CLI
  output) and `givn-maintainer--owner` (rule corpus). Actors in Gherkin stay
  Developer and Authoring agent; Persona biographies are review context only.
- A mandatory check is never silently deferred: an operator question and its
  recorded decision precede any deferral.

Capabilities and step files:

- `instruction-integrity` — new steps in `tests/steps/instruction_integrity_steps.rs`
- `ideation-skill` — reused generic steps in `tests/steps/prose_consolidation_steps.rs`
- `prose-consolidation` — `tests/steps/prose_consolidation_steps.rs` plus one new `grill-<name>` command step
- `install-extras` — reused generic steps in `tests/steps/mod.rs` and `tests/steps/run_execution_e2e_steps.rs`

No scenario in this change is tagged `@e2e`: every new scenario is a variant
of an action already proven end-to-end (request artifact instructions, run
`givn skills sync`, run `givn addons enable`). The existing `@e2e` scenarios
for those actions remain the real-interface evidence.

## Setup

- [x] Confirm strict mode and the runner: `tests/features_runner.rs` uses
      `.fail_on_skipped()`; `verify.command` is `./run-tests.sh` and
      `verify.e2e_command` is `./run-tests.sh --e2e`. No new runner is needed.
- [x] Proof-of-strictness: wrote a temporary `tests/strict-mode-probe.feature`
      with one undefined step, ran it, confirmed non-zero exit, then deleted
      the probe. Evidence:
      ```
      GIVN_FEATURES=tests/strict-mode-probe.feature cargo test --locked --test features_runner
      1 feature
      1 scenario (1 failed)
      1 step (1 failed)
      Step doesn't match any function
      exit status: 1
      ```
- [x] Created the step skeletons in `tests/steps/instruction_integrity_steps.rs`
      (14 new steps for the retrieval, record-shape, no-change, and error
      scenarios) and the `grill-<name>` command step in
      `tests/steps/prose_consolidation_steps.rs`, all with `unimplemented!()`
      bodies. Evidence:
      ```
      cargo check --locked --tests
      Finished `dev` profile [unoptimized + debuginfo] target(s) in 11.14s
      ```
- [x] COMMIT: `cdfae0fe` -- `test(dialog-question-quality): strict-mode proof and step skeletons`

## Scenarios

### instruction-integrity

- [x] Scenario: `The canonical dialog policy is retrievable through the Givn CLI`
  - [x] RED: removed `@wip` from this scenario only; ran the single-scenario
        command → non-zero. Evidence:
        ```
        GIVN_FEATURES=givn/changes/dialog-question-quality/specs cargo test --locked --test features_runner -- --name "The canonical dialog policy is retrievable through the Givn CLI"
        Step panicked. Captured output: not implemented
        1 feature
        1 scenario (1 failed)
        1 step (1 failed)
        exit status: 1
        ```
  - [x] GREEN: wrote `assets/instructions/dialog.md` and moved the
        non-artifact policy fallback in `src/cli/instructions.rs` ahead of
        change resolution. Files: `assets/instructions/dialog.md` (new),
        `src/cli/instructions.rs`, `tests/steps/instruction_integrity_steps.rs`.
        `cargo clean -p givn` first (rust_embed does not track `assets/`), then
        the targeted run → zero. Evidence:
        ```
        ✔ Given a Givn project with an active change for the dialog policy
        ✔ When the author requests the dialog policy through the Givn CLI
        ✔ Then the CLI output contains the named-artifact rule
        ✔ And the CLI output contains the option treatment rule
        ✔ And the CLI output contains the standalone-answerability rule
        ✔ And the CLI output contains the rejected-alternative rule
        1 feature
        1 scenario (1 passed)
        6 steps (6 passed)
        ```
  - [x] REFACTOR: re-ran → zero. Evidence:
        ```
        1 feature
        1 scenario (1 passed)
        6 steps (6 passed)
        ```
  - [x] COMMIT: `bf4d45e2` -- `feat(instruction-integrity): The canonical dialog policy is retrievable through the Givn CLI`

- [x] Scenario: `The canonical dialog policy demonstrates the full question record`
  - [x] RED: removed `@wip` from this scenario only; run → non-zero. Evidence:
        ```
        GIVN_FEATURES=givn/changes/dialog-question-quality/specs cargo test --locked --test features_runner -- --name "The canonical dialog policy demonstrates the full question record"
        Step panicked. Captured output: not implemented
        1 feature
        1 scenario (1 failed)
        1 step (1 failed)
        exit status: 1
        ```
  - [x] GREEN: completed the policy's worked example (bad one-liner vs. full
        record with context, per-option Meaning/In practice/Gain/Cost, and the
        `Rejected:` line) and the "one-line gloss on first use" rule. Files:
        `assets/instructions/dialog.md`,
        `tests/steps/instruction_integrity_steps.rs`. `cargo clean -p givn`,
        then the targeted run → zero. Evidence:
        ```
        ✔ Given the canonical dialog policy is requested
        ✔ When an author reads the option treatment and the record shape
        ✔ Then the policy requires every option's meaning gain cost and concrete example
        ✔ And the policy requires the rejected alternatives with their reasons
        ✔ And the policy rejects an outcome-only resolution
        ✔ And the policy requires a gloss for every cross-reference
        1 feature
        1 scenario (1 passed)
        6 steps (6 passed)
        ```
  - [x] REFACTOR: re-ran → zero. Evidence:
        ```
        1 feature
        1 scenario (1 passed)
        6 steps (6 passed)
        ```
  - [x] COMMIT: `a72ff6a9` -- `feat(instruction-integrity): The canonical dialog policy demonstrates the full question record`

- [x] Scenario: `The dialog policy is retrievable without an active change`
  - [x] RED: removed `@wip` from this scenario only; run → non-zero. Evidence:
        ```
        GIVN_FEATURES=givn/changes/dialog-question-quality/specs cargo test --locked --test features_runner -- --name "The dialog policy is retrievable without an active change"
        1 feature
        1 scenario (1 failed)
        2 steps (1 passed, 1 failed)
        exit status: 1
        ```
  - [x] GREEN: the fallback from the first scenario already resolves before
        change resolution; implemented the no-change When step. Files:
        `tests/steps/instruction_integrity_steps.rs`. Run → zero. Evidence:
        ```
        GIVN_FEATURES=givn/changes/dialog-question-quality/specs cargo test --locked --test features_runner -- --name "The dialog policy is retrievable without an active change"
        ✔ Given a project root with givn/ initialised
        ✔ When the author requests the dialog policy through the Givn CLI without a change
        ✔ Then the command exits 0
        ✔ And the CLI output contains the named-artifact rule
        1 feature
        1 scenario (1 passed)
        4 steps (4 passed)
        ```
  - [x] REFACTOR: re-ran → zero. Evidence:
        ```
        1 feature
        1 scenario (1 passed)
        4 steps (4 passed)
        ```
  - [x] COMMIT: `8f7e9b2f` -- `feat(instruction-integrity): The dialog policy is retrievable without an active change`

- [x] Scenario: `An unresolvable instruction id keeps the known-artifact error`
  - [x] RED: removed `@wip` from this scenario only; run → non-zero. Evidence:
        ```
        GIVN_FEATURES=givn/changes/dialog-question-quality/specs cargo test --locked --test features_runner -- --name "An unresolvable instruction id keeps the known-artifact error"
        1 feature
        1 scenario (1 failed)
        2 steps (1 passed, 1 failed)
        exit status: 1
        ```
  - [x] GREEN: the fallback's error mapping keeps the `unknown artifact
        '<id>'` message; implemented the unknown-instruction When step. Files:
        `tests/steps/instruction_integrity_steps.rs`. Run → zero. Evidence:
        ```
        ✔ Given a project root with givn/ initialised
        ✔ When the author requests an unknown instruction through the Givn CLI
        ✔ Then the command fails
        ✔ And stderr contains "unknown artifact"
        1 feature
        1 scenario (1 passed)
        4 steps (4 passed)
        ```
  - [x] REFACTOR: re-ran → zero. Evidence:
        ```
        1 feature
        1 scenario (1 passed)
        4 steps (4 passed)
        ```
  - [x] COMMIT: `d2a22a23` -- `feat(instruction-integrity): An unresolvable instruction id keeps the known-artifact error`

- [x] Scenario: `The design-review template carries the per-question record shape`
  - [x] RED: removed `@wip` from this scenario only; run → non-zero. Evidence:
        ```
        GIVN_FEATURES=givn/changes/dialog-question-quality/specs cargo test --locked --test features_runner -- --name "The design-review template carries the per-question record shape"
        1 feature
        1 scenario (1 failed)
        5 steps (4 passed, 1 failed)
        exit status: 1
        ```
  - [x] GREEN: reshaped `assets/templates/design-review.md` — index table
        `# | Branch | Question | Outcome` plus the per-question block with
        `Context:`, `Options:`, `In practice:`, `Gain:`, `Cost:`,
        `Recommended:`, and `Resolution:` with rejected alternatives. Files:
        `assets/templates/design-review.md`. `cargo clean -p givn`, then the
        targeted run → zero. Evidence:
        ```
        ✔ When the user runs `givn eject design-review:template`
        ✔ Then the ejected design-review template contains "Branch"
        ✔ And the ejected design-review template contains "Outcome"
        ✔ And the ejected design-review template contains "In practice:"
        ✔ And the ejected design-review template contains "Recommended:"
        ✔ And the ejected design-review template contains "Rejected:"
        1 feature
        1 scenario (1 passed)
        7 steps (7 passed)
        ```
  - [x] REFACTOR: re-ran → zero. Evidence:
        ```
        1 feature
        1 scenario (1 passed)
        7 steps (7 passed)
        ```
  - [x] COMMIT: `785c2f51` -- `feat(instruction-integrity): The design-review template carries the per-question record shape`

### ideation-skill

- [x] Scenario: `The ideation guidance carries the phase ladder and the one-question rule`
  - [x] RED: removed `@wip` from this modified scenario only; run → non-zero.
        Evidence:
        ```
        GIVN_FEATURES=givn/changes/dialog-question-quality/specs cargo test --locked --test features_runner -- --name "The ideation guidance carries the phase ladder and the one-question rule"
        1 feature
        1 scenario (1 failed)
        9 steps (8 passed, 1 failed)
        exit status: 1
        ```
  - [x] GREEN: added the policy reference and the `questions.md` entry schema
        to `assets/skills/givn-ideation/SKILL.md.tmpl`, keeping "one question
        at a time". Files: `assets/skills/givn-ideation/SKILL.md.tmpl`.
        `cargo clean -p givn`, then the targeted run → zero. Evidence:
        ```
        ✔ When the user runs `givn skills sync`
        ✔ Then the generated givn-ideation guidance contains "Event Storming"
        ✔ And ... "Use Case Definition"
        ✔ And ... "Use Case Elaboration"
        ✔ And ... "Handoff"
        ✔ And ... "one question at a time"
        ✔ And ... "quality review"
        ✔ And ... "givn instructions dialog"
        ✔ And ... "In practice:"
        ✔ And ... "Rejected:"
        1 feature
        1 scenario (1 passed)
        11 steps (11 passed)
        ```
  - [x] REFACTOR: re-ran → zero. Evidence:
        ```
        1 feature
        1 scenario (1 passed)
        11 steps (11 passed)
        ```
  - [x] COMMIT: `520f85f2` -- `feat(ideation-skill): The ideation guidance carries the phase ladder and the one-question rule`

### prose-consolidation

- [x] Scenario: `Generated interactive guidance points to the canonical dialog policy`
  - [x] RED: removed `@wip` from this scenario only; run → non-zero. Evidence:
        ```
        GIVN_FEATURES=givn/changes/dialog-question-quality/specs cargo test --locked --test features_runner -- --name "Generated interactive guidance points to the canonical dialog policy"
        1 feature
        1 scenario (1 failed)
        4 steps (3 passed, 1 failed)
        exit status: 1
        ```
  - [x] GREEN: added the `givn instructions dialog` pointer to the explore
        skill and command, the ideation command, the propose command's
        ambiguity handoff, both grill commands, and both design-review
        surfaces (replacing the duplicated dialog-discipline rule text with
        the reference while keeping the bad/good example); added the
        `grill-<name>` command step. Files:
        `assets/skills/givn-explore/SKILL.md.tmpl`,
        `assets/skills/givn-design-review/SKILL.md.tmpl`,
        `assets/commands/givn-explore.md`, `assets/commands/givn-ideation.md`,
        `assets/commands/givn-propose.md`, `assets/commands/grill-me.md`,
        `assets/commands/grill-with-docs.md`,
        `assets/commands/givn-design-review.md`,
        `tests/steps/prose_consolidation_steps.rs`. `cargo clean -p givn`,
        then the targeted run → zero. Evidence:
        ```
        ✔ When the user runs `givn skills sync`
        ✔ Then the generated givn-ideation guidance contains "givn instructions dialog"
        ✔ And ... givn-explore guidance
        ✔ And ... givn-design-review guidance
        ✔ And ... givn-design-review command
        ✔ And ... givn-ideation command
        ✔ And ... givn-explore command
        ✔ And ... givn-propose command
        ✔ And ... grill-me command
        ✔ And ... grill-with-docs command
        1 feature
        1 scenario (1 passed)
        11 steps (11 passed)
        ```
  - [x] REFACTOR: re-ran → zero. Evidence:
        ```
        1 feature
        1 scenario (1 passed)
        11 steps (11 passed)
        ```
  - [x] COMMIT: `b84ddf67` -- `feat(prose-consolidation): Generated interactive guidance points to the canonical dialog policy`

### install-extras

- [x] Scenario: `The absolute-mode policy block carries the dialog context rule`
  - [x] RED: removed `@wip` from this scenario only; run → non-zero. Evidence:
        ```
        GIVN_FEATURES=givn/changes/dialog-question-quality/specs cargo test --locked --test features_runner -- --name "The absolute-mode policy block carries the dialog context rule"
        1 feature
        1 scenario (1 failed)
        5 steps (4 passed, 1 failed)
        exit status: 1
        ```
  - [x] GREEN: added the "Context required to answer a question is not
        filler; option codes without meanings are." line to
        `assets/addons/absolute-mode/policy.md`, reinstalled the binary, and
        ran `givn addons enable absolute_mode` in this repository to refresh
        the local `AGENTS.md` block. Files:
        `assets/addons/absolute-mode/policy.md`, `AGENTS.md`. `cargo clean -p
        givn`, then the targeted run → zero. Evidence:
        ```
        ✔ Given a project root with givn/ initialised without addons
        ✔ And givn addon "absolute_mode" is "false"
        ✔ When the user runs `givn addons enable absolute_mode`
        ✔ Then the command exits 0
        ✔ And the file "AGENTS.md" contains "Context required to answer a question is not filler"
        1 feature
        1 scenario (1 passed)
        5 steps (5 passed)
        ```
  - [x] REFACTOR: re-ran → zero. Evidence:
        ```
        1 feature
        1 scenario (1 passed)
        5 steps (5 passed)
        ```
  - [x] COMMIT: `876561ab` -- `feat(install-extras): The absolute-mode policy block carries the dialog context rule`

## Final checks

- [x] Run the full suite (`./run-tests.sh`) → zero exit. Evidence:
      ```
      Summary [26.559s] 491 tests run: 491 passed, 0 skipped
      ```
- [x] Run the e2e suite (`./run-tests.sh --e2e`) → zero exit. Evidence:
      ```
      Summary [5.179s] 88 tests run: 88 passed, 403 skipped
      ```
- [x] Run the change delta suite
      (`GIVN_FEATURES=givn/changes/dialog-question-quality/specs cargo test --locked --test features_runner`)
      → zero exit. Evidence:
      ```
      4 features
      8 scenarios (8 passed)
      54 steps (54 passed)
      ```
- [x] Grep every touched step-definition file for empty/no-op bodies → 0.
      Evidence:
      ```
      grep -n "unimplemented!\|todo!()" tests/steps/instruction_integrity_steps.rs tests/steps/prose_consolidation_steps.rs
      (no matches)
      ```
- [x] Confirm commit count matches scenario task count and every task has a
      recorded hash: one setup commit (`cdfae0fe`) plus eight scenario
      commits (`bf4d45e2`, `a72ff6a9`, `8f7e9b2f`, `d2a22a23`, `785c2f51`,
      `520f85f2`, `b84ddf67`, `876561ab`).
- [x] Confirm `givn lint --change dialog-question-quality` has no parse errors
      (exit 0). Evidence:
      ```
      givn lint: 4 file(s) checked — clean
      exit 0
      ```
      (53 overlap warnings remain; review records their dispositions.)
