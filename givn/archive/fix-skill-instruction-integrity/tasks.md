# Tasks: fix-skill-instruction-integrity

## Setup

- [x] Configure and prove the existing Gherkin runner for this change.
  - Runner: `GIVN_SPEC_EMBEDDER=stub GIVN_SPEC_RERANKER=stub cargo test --features spec-retrieval --test features_runner`
  - E2E runner: `./run-tests.sh --e2e`
  - Strict mode: `GivnWorld::cucumber().fail_on_skipped()`.
  - Stub: Rust `unimplemented!()`.
  - Step files: `tests/steps/instruction_integrity_steps.rs` and
    `tests/steps/instruction_integrity_e2e_steps.rs`.
  - Proof command and non-zero output:
    ```text
    GIVN_FEATURES=givn/changes/fix-skill-instruction-integrity/specs \
    GIVN_SPEC_EMBEDDER=stub GIVN_SPEC_RERANKER=stub \
    cargo test --features spec-retrieval --test features_runner -- \
      --name "Canonical E2E guidance rejects weaker interface evidence"
    1 feature, 1 scenario (1 failed), 1 step (1 failed)
    Step doesn't match any function
    ```
  - Full/e2e scenario-count proof:
    ```text
    Full: 11 scenarios (11 failed)
    E2E: 4 scenarios (4 failed)
    Non-E2E: 7 scenarios (7 failed)
    E2E count is strictly smaller than full count.
    ```

## Non-E2E Scenarios

### Canonical E2E guidance rejects weaker interface evidence

- [x] RED: target `Canonical E2E guidance rejects weaker interface evidence`;
  paste non-zero runner output.
  ```text
  1 feature, 1 scenario (1 failed), 1 step (1 failed)
  Step doesn't match any function
  ```
- [x] GREEN: update the canonical policy and source assertions; list files
  touched; paste zero-exit targeted output.
  ```text
  1 feature, 1 scenario (1 passed), 6 steps (6 passed)
  Files: assets/instructions/specs.md, tests/steps/mod.rs,
  tests/steps/instruction_integrity_steps.rs,
  tests/steps/instruction_integrity_e2e_steps.rs,
  givn/changes/fix-skill-instruction-integrity/specs/instruction-integrity/instruction-integrity.feature
  ```
- [x] REFACTOR: simplify policy wording without behavior change; paste output.
  ```text
  1 feature, 1 scenario (1 passed), 6 steps (6 passed)
  ```
- [x] COMMIT: `d20b164` - `feat(instruction-integrity): enforce canonical E2E policy`

### Design guidance requires the complete E2E setup record

- [x] RED: target `Design guidance requires the complete E2E setup record`;
  paste non-zero runner output.
  ```text
  1 feature, 1 scenario (1 failed), 1 step (1 failed)
  Step doesn't match any function
  ```
- [x] GREEN: restore design-owned E2E fields and interface taxonomy; list files
  touched; paste zero-exit targeted output.
  ```text
  1 feature, 1 scenario (1 passed), 6 steps (6 passed)
  Files: assets/instructions/design.md,
  tests/steps/instruction_integrity_steps.rs,
  givn/changes/fix-skill-instruction-integrity/specs/instruction-integrity/instruction-integrity.feature
  ```
- [x] REFACTOR: remove redundant wording while preserving the design contract;
  paste output.
  ```text
  1 feature, 1 scenario (1 passed), 6 steps (6 passed)
  ```
- [x] COMMIT: `bef6724` - `feat(instruction-integrity): restore design E2E requirements`

### Review guidance verifies semantic remediation

- [x] RED: target `Review guidance verifies semantic remediation`; paste
  non-zero runner output.
  ```text
  1 feature, 1 scenario (1 failed), 1 step (1 failed)
  Step doesn't match any function
  ```
- [x] GREEN: add class-specific remediation evidence to review guidance and
  template; list files touched; paste zero-exit targeted output.
  ```text
  1 feature, 1 scenario (1 passed), 5 steps (5 passed)
  Files: assets/instructions/review.md, assets/templates/review.md,
  tests/steps/instruction_integrity_steps.rs,
  givn/changes/fix-skill-instruction-integrity/specs/instruction-integrity/instruction-integrity.feature
  ```
- [x] REFACTOR: align receipt and failure wording with the existing verifier;
  paste output.
  ```text
  1 feature, 1 scenario (1 passed), 5 steps (5 passed)
  ```
- [x] COMMIT: `68df916` - `feat(instruction-integrity): document semantic remediation review`

### Machine-readable instructions describe the effective artifact

- [x] RED: target `Machine-readable instructions describe the effective artifact`;
  paste non-zero runner output.
  ```text
  1 feature, 1 scenario (1 failed), 1 step (1 failed)
  Step doesn't match any function
  ```
- [x] GREEN: emit `{id, generates, requires, instruction}` and update the
  consumer guidance; list files touched; paste zero-exit targeted output.
  ```text
  1 feature, 1 scenario (1 passed), 5 steps (5 passed)
  Files: src/cli/instructions.rs, assets/commands/givn-spec.md,
  assets/commands/givn-design.md, assets/commands/givn-tasks.md,
  assets/commands/givn-review.md, assets/commands/givn-design-review.md,
  tests/steps/instruction_integrity_steps.rs,
  givn/changes/fix-skill-instruction-integrity/specs/instruction-integrity/instruction-integrity.feature
  ```
- [x] REFACTOR: preserve one-document JSON output and human text output; paste
  output.
  ```text
  1 feature, 1 scenario (1 passed), 5 steps (5 passed)
  ```
- [x] COMMIT: `d4d2a16` - `feat(instruction-integrity): expose instruction metadata`

### Artifact instructions reject an unsafe change context

- [x] RED: target `Artifact instructions reject an unsafe change context`;
  paste non-zero runner output.
  ```text
  1 feature, 1 scenario (1 failed), 1 step (1 failed)
  Step panicked: not implemented
  ```
- [x] GREEN: validate the explicit change id before emitting
  artifact guidance; list files touched; paste zero-exit targeted output.
  ```text
  1 feature, 1 scenario (1 passed), 3 steps (3 passed)
  Files: src/cli/instructions.rs, tests/steps/instruction_integrity_steps.rs,
  givn/changes/fix-skill-instruction-integrity/specs/instruction-integrity/instruction-integrity.feature,
  givn/changes/fix-skill-instruction-integrity/design.md
  ```
- [x] REFACTOR: preserve the single-change auto-selection behavior; paste output.
  ```text
  1 feature, 1 scenario (1 passed), 3 steps (3 passed)
  ```
- [x] COMMIT: `fefa780` - `fix(instruction-integrity): preserve global instruction contexts`

### Project instruction overrides survive skill synchronization

- [x] RED: target `Project instruction overrides survive skill synchronization`;
  paste non-zero runner output.
  ```text
  1 feature, 1 scenario (1 failed), 1 step (1 failed)
  Step panicked: not implemented
  ```
- [x] GREEN: exercise the project instruction override, run `givn skills sync`,
  and verify the override remains effective; list files touched; paste zero-exit
  targeted output.
  ```text
  1 feature, 1 scenario (1 passed), 4 steps (4 passed)
  Files: tests/steps/instruction_integrity_steps.rs,
  givn/changes/fix-skill-instruction-integrity/specs/instruction-integrity/instruction-integrity.feature
  ```
- [x] REFACTOR: keep override and snapshot assertions independent; paste output.
  ```text
  1 feature, 1 scenario (1 passed), 4 steps (4 passed)
  ```
- [x] COMMIT: `ac8f487` - `test(instruction-integrity): preserve policy overrides`

### Retrieval guidance distinguishes build capabilities

- [x] RED: target `Retrieval guidance distinguishes build capabilities`; paste
  non-zero runner output.
  ```text
  1 feature, 1 scenario (1 failed), 1 step (1 failed)
  Step doesn't match any function
  ```
- [x] GREEN: replace stale pre-retrieval wording with feature-aware guidance;
  list files touched; paste zero-exit targeted output.
  ```text
  1 feature, 1 scenario (1 passed), 5 steps (5 passed)
  Files: assets/instructions/specs.md, assets/commands/givn-spec.md,
  assets/skills/givn-spec/SKILL.md.tmpl,
  tests/steps/instruction_integrity_steps.rs,
  givn/changes/fix-skill-instruction-integrity/specs/instruction-integrity/instruction-integrity.feature
  ```
- [x] REFACTOR: align advisory, blocking, and unavailable paths; paste output.
  ```text
  1 feature, 1 scenario (1 passed), 5 steps (5 passed)
  ```
- [x] COMMIT: `0e89485` - `fix(instruction-integrity): align retrieval guidance`

## E2E Scenarios

### An author requests the canonical interaction and E2E guidance

- [x] RED: target `An author requests the canonical interaction and E2E guidance`
  with `verify.e2e_command`; paste non-zero output.
  ```text
  1 feature, 1 scenario (1 failed), 1 step (1 failed)
  Step doesn't match any function
  ```
- [x] GREEN: invoke the compiled CLI and assert complete canonical guidance and
  retrieval-mode wording on stdout; list files touched; paste zero-exit output.
  ```text
  1 feature, 1 scenario (1 passed), 4 steps (4 passed)
  Files: tests/steps/instruction_integrity_e2e_steps.rs,
  givn/changes/fix-skill-instruction-integrity/specs/instruction-integrity/instruction-integrity.feature
  ```
- [x] REFACTOR: clean subprocess setup and assertions; paste output.
  ```text
  1 feature, 1 scenario (1 passed), 4 steps (4 passed)
  ```
- [x] COMMIT: `dae070d` - `test(e2e): verify canonical instruction guidance`

### An author requests the technical E2E design guidance

- [x] RED: target `An author requests the technical E2E design guidance` with
  `verify.e2e_command`; paste non-zero output.
  ```text
  1 feature, 1 scenario (1 failed), 1 step (1 failed)
  Step doesn't match any function
  ```
- [x] GREEN: invoke the compiled CLI and assert all design setup requirements;
  list files touched; paste zero-exit output.
  ```text
  1 feature, 1 scenario (1 passed), 3 steps (3 passed)
  Files: tests/steps/instruction_integrity_e2e_steps.rs,
  givn/changes/fix-skill-instruction-integrity/specs/instruction-integrity/instruction-integrity.feature
  ```
- [x] REFACTOR: clean subprocess setup and assertions; paste output.
  ```text
  1 feature, 1 scenario (1 passed), 3 steps (3 passed)
  ```
- [x] COMMIT: `23703e7` - `test(e2e): verify design instruction guidance`

### A reviewer requests remediation-aware review guidance

- [x] RED: target `A reviewer requests remediation-aware review guidance` with
  `verify.e2e_command`; paste non-zero output.
  ```text
  1 feature, 1 scenario (1 failed), 1 step (1 failed)
  Step doesn't match any function
  ```
- [x] GREEN: invoke the compiled CLI and assert semantic classification and
  remediation checks; list files touched; paste zero-exit output.
  ```text
  1 feature, 1 scenario (1 passed), 3 steps (3 passed)
  Files: tests/steps/instruction_integrity_e2e_steps.rs,
  givn/changes/fix-skill-instruction-integrity/specs/instruction-integrity/instruction-integrity.feature
  ```
- [x] REFACTOR: clean subprocess setup and assertions; paste output.
  ```text
  1 feature, 1 scenario (1 passed), 3 steps (3 passed)
  ```
- [x] COMMIT: `c0fd86d` - `test(e2e): verify remediation review guidance`

### An agent requests machine-readable artifact guidance

- [x] RED: target `An agent requests machine-readable artifact guidance` with
  `verify.e2e_command`; paste non-zero output.
  ```text
  1 feature, 1 scenario (1 failed), 1 step (1 failed)
  Step doesn't match any function
  ```
- [x] GREEN: invoke the compiled CLI, parse one JSON document, and assert the
  four contract fields; list files touched; paste zero-exit output.
  ```text
  1 feature, 1 scenario (1 passed), 3 steps (3 passed)
  Files: tests/steps/instruction_integrity_e2e_steps.rs,
  givn/changes/fix-skill-instruction-integrity/specs/instruction-integrity/instruction-integrity.feature
  ```
- [x] REFACTOR: keep JSON stdout free of diagnostics; paste output.
  ```text
  1 feature, 1 scenario (1 passed), 3 steps (3 passed)
  ```
- [x] COMMIT: `582eff5` - `test(e2e): verify instruction JSON contract`

## Completion

- [x] Run the full non-E2E suite and the full E2E suite.
  - Non-E2E change suite: `7 scenarios`, `34 steps`, all passed.
  - E2E change suite: `4 scenarios`, `13 steps`, all passed.
  - Repository full suite: `282 scenarios`, `1754 steps`, all passed.
  - Repository E2E suite: `37 scenarios`, `224 steps`, all passed.
- [x] Run `givn skills sync` and verify `.opencode`, `.agents`, `.claude`, and
  harness mirrors.
  - Root targets regenerated with the retrieval-enabled binary.
  - Harness `.opencode` commands and skills regenerated.
  - Generated targets contain no stale pre-retrieval prohibition or obsolete
    instruction metadata keys.
- [x] Run `cargo fmt --all -- --check` and
  `cargo clippy --workspace --all-targets -- -D warnings`.
  - Both commands passed.
- [x] Record all scenario commit hashes and check each box immediately after
  its evidence is complete. All 11 scenario commits are recorded above.
