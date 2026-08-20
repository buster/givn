# Tasks: semantic-e5-token-cap

## Setup

- [x] Prove strict Cucumber behavior with an undefined/pending token-cap step;
      confirm non-zero exit and record the output.
- [x] Create and register one regular and one E2E step-definition module for
      `semantic-e5-token-cap`; no empty step bodies.
  - Strict proof command: `GIVN_FEATURES=givn/changes/semantic-e5-token-cap/specs GIVN_SPEC_EMBEDDER=stub cargo test --features spec-retrieval --test features_runner -- --name "Indexing keeps fitting scenarios when another passage exceeds E5 capacity"`
  - Strict proof output: `1 scenario (1 failed)`, pending step panicked with `not implemented: E5 token-cap E2E setup step`, process exited non-zero.
- [x] Confirm the full and E2E runner commands and record their scenario counts.
  - Full command/output: `./run-tests.sh` -> 8 Cucumber invocations, 388 scenarios total (328 + 36 + 1 + 1 + 9 + 8 + 4 + 1), all passed.
  - E2E command/output: `./run-tests.sh --e2e` -> 4 Cucumber invocations, 55 scenarios total (42 + 7 + 5 + 1), all passed; strict subset of full.
  - Commit:

## Scenario: Indexing keeps fitting scenarios when another passage exceeds E5 capacity

- [x] RED: remove `@wip`, add non-empty pending step definitions, target only
      this scenario, and record the non-zero result.
   - Command: `GIVN_FEATURES=givn/changes/semantic-e5-token-cap/specs GIVN_SPEC_EMBEDDER=stub cargo test --features spec-retrieval --test features_runner -- --name "Indexing keeps fitting scenarios when another passage exceeds E5 capacity"`
   - Output: non-zero; `1 scenario (1 failed)`, the pending E2E setup step panicked with `not implemented: E5 token-cap E2E setup step`.
- [x] GREEN: partition fitting/capped E5 passages, persist cap metadata, and
      assert fitting vectors plus no truncated vector for the capped endpoint.
   - Command: `GIVN_FEATURES=givn/changes/semantic-e5-token-cap/specs GIVN_SPEC_EMBEDDER=stub cargo test --features spec-retrieval --test features_runner -- --name "Indexing keeps fitting scenarios when another passage exceeds E5 capacity"`
   - Output: `1 feature`, `1 scenario (1 passed)`, `9 steps (9 passed)`.
   - Production files: `src/spec/retrieval.rs` (partial E5 embedding batch, fixture/real tokenizer preflight, index cap metadata and refresh); test files `tests/steps/mod.rs`, `tests/steps/semantic_e5_token_cap_e2e_steps.rs`, `tests/steps/semantic_e5_token_cap_steps.rs`; feature/task evidence files.
   - Coverage: targeted runner output has no coverage instrumentation; production coverage is unmeasured for this phase.
- [x] REFACTOR: rerun the targeted scenario after cleanup.
   - Command: `GIVN_FEATURES=givn/changes/semantic-e5-token-cap/specs GIVN_SPEC_EMBEDDER=stub cargo test --features spec-retrieval --test features_runner -- --name "Indexing keeps fitting scenarios when another passage exceeds E5 capacity"`
   - Output: `1 feature`, `1 scenario (1 passed)`, `9 steps (9 passed)` after `cargo fmt`.
- [ ] COMMIT: record the scenario commit.
  - Commit:

## Scenario: Duplicate audit preserves deterministic findings and reports an E5 token-cap candidate

- [ ] RED: target this scenario and record the non-zero result.
  - Command:
  - Output:
- [ ] GREEN: add the separate `e5_token_cap_candidates` duplicate JSON/text
      contract and deterministic endpoint assertions.
  - Command:
  - Output:
  - Production files:
- [ ] REFACTOR: rerun the targeted scenario after cleanup.
  - Command:
  - Output:
- [ ] COMMIT: record the scenario commit.
  - Commit:

## Scenario: Explore keeps an E5 token-cap finding visible while fitting results remain usable

- [ ] RED: target this scenario and record the non-zero result.
  - Command:
  - Output:
- [ ] GREEN: expose additive Explore cap findings while preserving fitting
      ranked results.
  - Command:
  - Output:
  - Production files:
- [ ] REFACTOR: rerun the targeted scenario after cleanup.
  - Command:
  - Output:
- [ ] COMMIT: record the scenario commit.
  - Commit:

## Scenario: Advisory review exposes an E5 token-cap finding without blocking

- [ ] RED: target this scenario and record the non-zero result.
  - Command:
  - Output:
- [ ] GREEN: expose cap findings in advisory review without changing its exit
      behavior.
  - Command:
  - Output:
  - Production files:
- [ ] REFACTOR: rerun the targeted scenario after cleanup.
  - Command:
  - Output:
- [ ] COMMIT: record the scenario commit.
  - Commit:

## Scenario: Blocking review keeps an over-cap active scenario unresolved

- [ ] RED: target this scenario and record the non-zero result.
  - Command:
  - Output:
- [ ] GREEN: write the additive blocked semantic report section with null
      classification/rationale and refactoring guidance.
  - Command:
  - Output:
  - Production files:
- [ ] REFACTOR: rerun the targeted scenario after cleanup.
  - Command:
  - Output:
- [ ] COMMIT: record the scenario commit.
  - Commit:

## Scenario: Exactly 512 E5 passage tokens retain normal embedding behaviour

- [ ] RED: target this scenario and record the non-zero result.
  - Command:
  - Output:
- [ ] GREEN: prove the inclusive boundary does not create cap metadata.
  - Command:
  - Output:
  - Production files:
- [ ] REFACTOR: rerun the targeted scenario after cleanup.
  - Command:
  - Output:
- [ ] COMMIT: record the scenario commit.
  - Commit:

## Final Verification

- [ ] `givn lint --change semantic-e5-token-cap` has no unexpected findings.
- [ ] `./run-tests.sh` passes.
- [ ] `./run-tests.sh --e2e` passes.
- [ ] `cargo fmt --check` passes.
- [ ] `cargo clippy --workspace --all-targets -- -D warnings` passes.
- [ ] `givn check review --change semantic-e5-token-cap` passes.
- [ ] Archive gate passes and the change is archived.
