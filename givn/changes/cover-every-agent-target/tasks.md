# Tasks: cover-every-agent-target

## Context

- Use case: fragment `support` — capability `instruction-integrity`. The
  shared guarantee: generated guidance stays complete and trustworthy for
  every supported harness.
- Confirmed Personas: `givn-maintainer--owner`, `agent--technical-role`
  (review context only; Gherkin actors stay distinct).
- Verify command: `./run-tests.sh`; change scope:
  `GIVN_FEATURES=givn/changes/cover-every-agent-target/specs`.
- E2E: no delta `@e2e` scenario exists; `verify.e2e_command` is unchanged;
  the e2e setup task is not applicable.
- Design: `givn/changes/cover-every-agent-target/design.md`.
- Interaction coverage cross-reference: no `.feature` file carries a
  `# User Interaction Inventory:` block; the canonical policy keeps the
  inventory in the fragment document. The fragment's two
  `instruction-integrity` rows map to the design's matrix with real CLI
  subprocesses; the delta adds no consumer action.

- [x] Planning artifacts committed. COMMIT: `f1818b73` —
  proposal, delta specs, design, design-review, arc42-docs, and the task
  breakdown committed together after design-review sign-off.

## Setup task — strict mode

- [x] **S1. Prove strict mode on this change's delta.**
  - Runner config: none required — `./run-tests.sh` drives
    `tests/features_runner.rs` with `.fail_on_skipped()`.
  - Proof: the delta's three scenarios carry `@wip` and their new steps are
    undefined; run the change scope:
    `GIVN_FEATURES=givn/changes/cover-every-agent-target/specs ./run-tests.sh`
  - Expected: non-zero exit (undefined steps fail under strict mode).
  - Captured output:
    ```
    exit=100
    Summary [0.458s] 3 tests run: 0 passed, 3 failed, 0 skipped
        FAIL (1/3) fragments/instruction-integrity.feature::A configured target without synchronized guidance fails the check
        FAIL (2/3) fragments/instruction-integrity.feature::The characterization guidance requests the reverse review from Givn
        FAIL (3/3) fragments/instruction-integrity.feature::The generated guidance references resolve with the default configuration
    error: test run failed
    ```

## Task 1 — The characterization guidance requests the reverse review from Givn (`instruction-integrity`, modified)

- [x] RED — remove `@wip` from this scenario only; run the single-scenario
  command; expect non-zero (the new Then step is undefined).
  - Command: `GIVN_FEATURES=givn/changes/cover-every-agent-target/specs cargo test --locked --test features_runner -- --exact "fragments/instruction-integrity.feature::The characterization guidance requests the reverse review from Givn"`
  - Output: `exit=101` — `✘ And every command-owning agent target receives the givn-characterize command` → `Step doesn't match any function`; `1 scenario (1 failed)`.
- [x] GREEN — one atomic step (the fixture edits and the derived expectation
  must land together, or the permanent addon-free scenario fails): derive all
  four "all targets" fixtures from `AgentTarget::ALL`
  (`prose_consolidation_steps.rs` prompt and `--agents`,
  `prose_consolidation_e2e_steps.rs`, `instruction_integrity_steps.rs`
  addon-free), derive `selected_targets_and_addons_unchanged`, derive the two
  "every target" loops (`core_entries_require_dialog_policy`,
  `skill_consumers_receive_replacement`), switch the characterize command
  step to `owns_commands`, and implement the new step "every command-owning
  agent target receives the givn-characterize command". Re-run the scenario;
  also run the full permanent suite to prove the fixture blast radius.
  - Files: `tests/steps/prose_consolidation_steps.rs`,
    `tests/steps/prose_consolidation_e2e_steps.rs`,
    `tests/steps/instruction_integrity_steps.rs`.
  - Output: scenario `exit=0` — `1 scenario (1 passed)`, `5 steps (5 passed)`.
    Full suite `exit=0` — `605 tests run: 605 passed, 0 skipped`. E2E suite
    `exit=0` — `90 tests run: 90 passed, 515 skipped`.
- [x] REFACTOR — re-run; expect zero.
  - Output: `exit=0` — `1 scenario (1 passed)`, `5 steps (5 passed)`.
- [x] COMMIT: `8dfa6373` -- `feat(instruction-integrity): <release note>`
  with the scenario title in the body.

## Task 2 — The generated guidance references resolve with the default configuration (`instruction-integrity`, modified)

- [ ] RED — remove `@wip` from this scenario only; run; expect non-zero (the
  new coverage step is undefined).
  - Command: `GIVN_FEATURES=givn/changes/cover-every-agent-target/specs cargo test --locked --test features_runner -- --exact "fragments/instruction-integrity.feature::The generated guidance references resolve with the default configuration"`
  - Output: _(paste)_
- [ ] GREEN — derive recognition from production
  (`recognizes_target_dir` over `AgentTarget::ALL`, delete `AGENT_TARGETS`),
  add `uncovered_targets`, strengthen the installed positive Then
  (`installed_guidance_references_resolve`) to assert no configured target is
  uncovered, and implement "every agent target Givn supports is covered by
  the synchronized guidance" with the coupling guard. Re-run; expect zero.
  - Files: `tests/common/reference_check.rs`,
    `tests/steps/instruction_integrity_steps.rs`.
  - Output: _(paste)_
- [ ] REFACTOR — re-run; expect zero.
  - Output: _(paste)_
- [ ] COMMIT: `_(filled)_` -- `feat(instruction-integrity): <release note>`
  with the scenario title in the body.

## Task 3 — A configured target without synchronized guidance fails the check (`instruction-integrity`, added)

- [ ] RED — remove `@wip` from this scenario only; run; expect non-zero (both
  new steps are undefined).
  - Command: `GIVN_FEATURES=givn/changes/cover-every-agent-target/specs cargo test --locked --test features_runner -- --exact "fragments/instruction-integrity.feature::A configured target without synchronized guidance fails the check"`
  - Output: _(paste)_
- [ ] GREEN — implement "the skills of the agent target `<target>` are
  removed" (validate via `AgentTarget::from_name`, then remove the skills
  directory) and "the configured-target check fails and names the agent
  target `<target>`" (via `uncovered_targets`). Re-run; expect zero.
  - Files: `tests/steps/instruction_integrity_steps.rs`.
  - Output: _(paste)_
- [ ] REFACTOR — re-run; expect zero.
  - Output: _(paste)_
- [ ] COMMIT: `_(filled)_` -- `feat(instruction-integrity): <release note>`
  with the scenario title in the body.

## Final verification

- [ ] **F1. Change scope green.** Run
  `GIVN_FEATURES=givn/changes/cover-every-agent-target/specs ./run-tests.sh`
  and paste the summary (zero exit).
  - Output: _(paste)_
- [ ] **F2. Full suites green.** Run `./run-tests.sh` and
  `./run-tests.sh --e2e`; paste both summaries (zero exit each).
  - Output: _(paste)_
- [ ] **F3. No empty step bodies.** Grep every touched step file for `{}`,
  bare `return`, and `unimplemented!()`; zero hits.
  - Output: _(paste)_
- [ ] **F4. Commit count.** One commit per scenario (3) plus the planning
  commit; list the hashes.
  - Output: _(paste)_
