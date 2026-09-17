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

- [ ] Confirm strict mode and the runner: `tests/features_runner.rs` uses
      `.fail_on_skipped()`; `verify.command` is `./run-tests.sh` and
      `verify.e2e_command` is `./run-tests.sh --e2e`. No new runner is needed.
- [ ] Proof-of-strictness: write a temporary feature file with one undefined
      step, run it, confirm non-zero exit, then delete the probe. Paste the
      command and output here:
      ```
      ```
- [ ] Create the step skeletons in `tests/steps/instruction_integrity_steps.rs`
      (new steps for the retrieval, record-shape, no-change, and error
      scenarios) and the `grill-<name>` command step in
      `tests/steps/prose_consolidation_steps.rs`, all with `unimplemented!()`
      bodies; `cargo check --locked --tests` succeeds. Paste the output:
      ```
      ```
- [ ] COMMIT: `<hash>` -- `test(dialog-question-quality): strict-mode proof and step skeletons`

## Scenarios

### instruction-integrity

- [ ] Scenario: `The canonical dialog policy is retrievable through the Givn CLI`
  - [ ] RED: removed `@wip` from this scenario only; ran the single-scenario
        command → non-zero. Evidence:
        ```
        ```
  - [ ] GREEN: wrote `assets/instructions/dialog.md` and reordered the
        non-artifact fallback in `src/cli/instructions.rs` ahead of change
        resolution. Files: `assets/instructions/dialog.md`,
        `src/cli/instructions.rs`. Run → zero. Evidence:
        ```
        ```
  - [ ] REFACTOR: re-ran → zero. Evidence:
        ```
        ```
  - [ ] COMMIT: `<hash>` -- `feat(instruction-integrity): The canonical dialog policy is retrievable through the Givn CLI`

- [ ] Scenario: `The canonical dialog policy demonstrates the full question record`
  - [ ] RED: removed `@wip` from this scenario only; run → non-zero. Evidence:
        ```
        ```
  - [ ] GREEN: completed the policy's worked example (bad one-liner vs. full
        record with context, per-option meaning, example, gain, cost, and
        rejected alternatives) and the "one-line gloss on first use" rule.
        Files: `assets/instructions/dialog.md`. Run → zero. Evidence:
        ```
        ```
  - [ ] REFACTOR: re-ran → zero. Evidence:
        ```
        ```
  - [ ] COMMIT: `<hash>` -- `feat(instruction-integrity): The canonical dialog policy demonstrates the full question record`

- [ ] Scenario: `The dialog policy is retrievable without an active change`
  - [ ] RED: removed `@wip` from this scenario only; run → non-zero. Evidence:
        ```
        ```
  - [ ] GREEN: confirmed the fallback resolves before change resolution (the
        previous scenario's production code); adjusted the branch order if the
        no-change path still failed. Files: `src/cli/instructions.rs`. Run →
        zero. Evidence:
        ```
        ```
  - [ ] REFACTOR: re-ran → zero. Evidence:
        ```
        ```
  - [ ] COMMIT: `<hash>` -- `feat(instruction-integrity): The dialog policy is retrievable without an active change`

- [ ] Scenario: `An unresolvable instruction id keeps the known-artifact error`
  - [ ] RED: removed `@wip` from this scenario only; run → non-zero. Evidence:
        ```
        ```
  - [ ] GREEN: kept the `unknown artifact '<id>'` error for an unresolvable
        id. Files: `src/cli/instructions.rs`. Run → zero. Evidence:
        ```
        ```
  - [ ] REFACTOR: re-ran → zero. Evidence:
        ```
        ```
  - [ ] COMMIT: `<hash>` -- `feat(instruction-integrity): An unresolvable instruction id keeps the known-artifact error`

- [ ] Scenario: `The design-review template carries the per-question record shape`
  - [ ] RED: removed `@wip` from this scenario only; run → non-zero. Evidence:
        ```
        ```
  - [ ] GREEN: reshaped `assets/templates/design-review.md` — index table
        `# | Branch | Question | Outcome` plus the per-question blocks with
        `Context:`, `Options:`, `In practice:`, `Gain:`, `Cost:`,
        `Recommended:`, and `Resolution:` with rejected alternatives. Files:
        `assets/templates/design-review.md`. Run → zero. Evidence:
        ```
        ```
  - [ ] REFACTOR: re-ran → zero. Evidence:
        ```
        ```
  - [ ] COMMIT: `<hash>` -- `feat(instruction-integrity): The design-review template carries the per-question record shape`

### ideation-skill

- [ ] Scenario: `The ideation guidance carries the phase ladder and the one-question rule`
  - [ ] RED: removed `@wip` from this modified scenario only; run → non-zero.
        Evidence:
        ```
        ```
  - [ ] GREEN: added the policy reference and the `questions.md` entry schema
        to `assets/skills/givn-ideation/SKILL.md.tmpl`, keeping "one question
        at a time". Files: `assets/skills/givn-ideation/SKILL.md.tmpl`. Run →
        zero. Evidence:
        ```
        ```
  - [ ] REFACTOR: re-ran → zero. Evidence:
        ```
        ```
  - [ ] COMMIT: `<hash>` -- `feat(ideation-skill): The ideation guidance carries the phase ladder and the one-question rule`

### prose-consolidation

- [ ] Scenario: `Generated interactive guidance points to the canonical dialog policy`
  - [ ] RED: removed `@wip` from this scenario only; run → non-zero. Evidence:
        ```
        ```
  - [ ] GREEN: added the `givn instructions dialog` pointer to
        `assets/skills/givn-ideation/SKILL.md.tmpl`,
        `assets/skills/givn-explore/SKILL.md.tmpl`,
        `assets/skills/givn-design-review/SKILL.md.tmpl`,
        `assets/commands/givn-ideation.md`, `assets/commands/givn-explore.md`,
        `assets/commands/givn-propose.md`, `assets/commands/grill-me.md`,
        `assets/commands/grill-with-docs.md`, and
        `assets/commands/givn-design-review.md`; added the `grill-<name>`
        command step. Files: the eight assets plus
        `tests/steps/prose_consolidation_steps.rs`. Run → zero. Evidence:
        ```
        ```
  - [ ] REFACTOR: re-ran → zero. Evidence:
        ```
        ```
  - [ ] COMMIT: `<hash>` -- `feat(prose-consolidation): Generated interactive guidance points to the canonical dialog policy`

### install-extras

- [ ] Scenario: `The absolute-mode policy block carries the dialog context rule`
  - [ ] RED: removed `@wip` from this scenario only; run → non-zero. Evidence:
        ```
        ```
  - [ ] GREEN: added the "Context required to answer a question is not
        filler; option codes without meanings are." line to
        `assets/addons/absolute-mode/policy.md`, then ran
        `givn addons enable absolute_mode` in this repository to refresh the
        local `AGENTS.md` block. Files: `assets/addons/absolute-mode/policy.md`,
        `AGENTS.md`. Run → zero. Evidence:
        ```
        ```
  - [ ] REFACTOR: re-ran → zero. Evidence:
        ```
        ```
  - [ ] COMMIT: `<hash>` -- `feat(install-extras): The absolute-mode policy block carries the dialog context rule`

## Final checks

- [ ] Run the full suite (`./run-tests.sh`) → zero exit. Evidence:
      ```
      ```
- [ ] Run the e2e suite (`./run-tests.sh --e2e`) → zero exit. Evidence:
      ```
      ```
- [ ] Grep every touched step-definition file for empty/no-op bodies → 0.
      Evidence:
      ```
      ```
- [ ] Confirm commit count matches scenario task count and every task has a
      recorded hash.
- [ ] Confirm `givn lint --change dialog-question-quality` has no parse errors
      (exit 0 or 2).
