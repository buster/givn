# Tasks: semantic-compact-serialization

## Setup

- [x] Configure and prove the strict Gherkin runner before implementation.
  - Runner: `tests/features_runner.rs` with `.fail_on_skipped()`.
  - Verify command: `./run-tests.sh`.
  - E2E command: `./run-tests.sh --e2e`.
  - Create and register `tests/steps/semantic_compact_serialization_steps.rs`.
  - Create and register `tests/steps/semantic_compact_serialization_e2e_steps.rs`.
  - Strictness proof command: `GIVN_FEATURES=givn/changes/semantic-compact-serialization/specs cargo test --features spec-retrieval --test features_runner -- --name "Empty optional sections do not add retrieval content"` while the scenario is still `@wip` or has an unimplemented step.
  - Strictness proof output and non-zero exit: `GIVN_FEATURES=givn/changes/semantic-compact-serialization/specs cargo test --features spec-retrieval --test features_runner -- --name "Empty optional sections do not add retrieval content"` exited non-zero. The runner compiled with `.fail_on_skipped()` and failed on the undefined Given step: `Step doesn't match any function`; summary: `1 feature, 1 scenario (1 failed), 1 step (1 failed)`.
  - Setup files: `tests/steps/semantic_compact_serialization_steps.rs`, `tests/steps/semantic_compact_serialization_e2e_steps.rs`, and `tests/steps/mod.rs`.
  - Setup commit: pending; included in the first implementation commit.

## Scenario: Empty optional sections do not add retrieval content

- [x] RED
  - Remove `@wip` from this scenario only.
  - Bind its steps in `tests/steps/semantic_compact_serialization_steps.rs`; unimplemented steps use `unimplemented!()`.
  - Run: `GIVN_FEATURES=givn/changes/semantic-compact-serialization/specs cargo test --features spec-retrieval --test features_runner -- --name "Empty optional sections do not add retrieval content"`.
  - Expected result: non-zero because the scenario is not implemented.
  - Captured output: command compiled the runner, matched `tests/steps/semantic_compact_serialization_steps.rs:7:1`, panicked with `not implemented`, and exited non-zero; summary: `1 feature, 1 scenario (1 failed), 1 step (1 failed)`.
- [x] GREEN
  - Replace stubs with a temporary-project fixture and exact index-entry hash assertions.
  - Implement the minimum production serialization change in `src/spec/retrieval.rs`.
  - Add the exact empty-field serializer assertions required by this scenario.
  - Compile: `cargo check --features spec-retrieval`.
  - Run the targeted command above; expected result is zero.
  - Production files changed: `src/spec/retrieval.rs`; acceptance files: `tests/steps/semantic_compact_serialization_steps.rs`, `tests/steps/mod.rs`, and the change feature. `cargo check --features spec-retrieval` succeeded. Targeted output: `1 feature, 1 scenario (1 passed), 4 steps (4 passed)`.
  - Coverage: the configured runner emits no coverage artifact; coverage is unmeasured for this targeted run.
- [x] REFACTOR
  - Consolidate conditional emission and normalized optional-value handling without changing output.
  - Re-run the targeted command above; expected result is zero.
  - Captured output: after helper cleanup and exact serializer tests, the targeted run remained green: `1 feature, 1 scenario (1 passed), 4 steps (4 passed)`. Direct source tests: `2 passed; 0 failed`.
- [ ] COMMIT
  - Commit RED, GREEN, and REFACTOR together with a message referencing `Empty optional sections do not add retrieval content`.
  - Commit hash: pending.

## Scenario: Non-empty optional sections remain complete retrieval content

- [ ] RED
  - Remove `@wip` from this scenario only.
  - Bind its rich-content steps in `tests/steps/semantic_compact_serialization_steps.rs`; unimplemented steps use `unimplemented!()`.
  - Run: `GIVN_FEATURES=givn/changes/semantic-compact-serialization/specs cargo test --features spec-retrieval --test features_runner -- --name "Non-empty optional sections remain complete retrieval content"`.
  - Expected result: non-zero because the exact rich-content contract is not implemented.
  - Captured output: pending.
- [ ] GREEN
  - Add a rich Gherkin fixture covering descriptions, Feature/Rule Backgrounds, resolved roles, tables, docstrings, outline templates, and Examples.
  - Assert the exact complete compact content hash and direct serialized text contract.
  - Reuse the production serializer from the previous scenario; do not add a second serializer.
  - Compile: `cargo check --features spec-retrieval`.
  - Run the targeted command above; expected result is zero.
  - Production files changed: pending.
  - Captured output: pending.
- [ ] REFACTOR
  - Review optional nested-field omission, whitespace-only values, JSON escaping, and deterministic ordering.
  - Re-run the targeted command above; expected result is zero.
  - Captured output: pending.
- [ ] COMMIT
  - Commit RED, GREEN, and REFACTOR together with a message referencing `Non-empty optional sections remain complete retrieval content`.
  - Commit hash: pending.

## E2E Setup

- [ ] Prove the configured E2E runner is a strict subset of the full runner before the E2E scenario.
  - Local run command: `./run-tests.sh`.
  - E2E run command: `./run-tests.sh --e2e`.
  - No external services or digital twins are required; the real interface is the compiled CLI subprocess.
  - Full-suite scenario count: pending.
  - E2E-suite scenario count: pending and must be lower than the full-suite count.
  - Strict E2E proof output: pending.

## Scenario: A maintainer can index scenarios with complete compact passages

- [ ] RED
  - Remove `@wip` from this scenario only.
  - Bind the E2E steps in `tests/steps/semantic_compact_serialization_e2e_steps.rs`; unimplemented steps use `unimplemented!()`.
  - Run: `GIVN_FEATURES=givn/changes/semantic-compact-serialization/specs cargo test --features spec-retrieval --test features_runner -- --tags '@e2e' --name "A maintainer can index scenarios with complete compact passages"`.
  - Expected result: non-zero because the E2E steps are not implemented.
  - Captured output: pending.
- [ ] GREEN
  - Drive the real `givn spec index` subprocess in a temporary project.
  - Assert exit status and stdout as primary CLI evidence; assert the indexed scenario identity as secondary evidence.
  - Run `./run-tests.sh --e2e`; expected result is zero for this scenario.
  - Production files changed: none beyond the shared serializer already changed; test files: pending.
  - Captured output: pending.
- [ ] REFACTOR
  - Keep E2E setup isolated and avoid duplicating regular serializer assertions.
  - Re-run `./run-tests.sh --e2e`; expected result is zero.
  - Captured output: pending.
- [ ] COMMIT
  - Commit RED, GREEN, and REFACTOR together with a message referencing `A maintainer can index scenarios with complete compact passages`.
  - Commit hash: pending.

## Final Verification

- [ ] Run `cargo fmt --all -- --check`.
- [ ] Run `cargo clippy --workspace --all-targets -- -D warnings`.
- [ ] Run `./run-tests.sh`.
- [ ] Run `./run-tests.sh --e2e`.
- [ ] Record final status and review evidence before archive.
