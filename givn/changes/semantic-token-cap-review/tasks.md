# Tasks: semantic-token-cap-review

## Setup

- [x] Configure the strict Cucumber runner and prove undefined/pending steps
      fail non-zero.
  - RED command: `GIVN_FEATURES=givn/changes/semantic-token-cap-review/specs GIVN_SPEC_EMBEDDER=stub cargo test --features spec-retrieval,cross-encoder-reranker,nli-evidence --test features_runner -- --name "Permanent duplicate audit preserves BGE evidence when NLI reaches the cap"`
  - RED output: `1 feature, 1 scenario, 1 step failed; step doesn't match any function; cargo test exited 101.` The failure occurred before any new implementation, proving `.fail_on_skipped()` rejects undefined steps.
  - GREEN command: Existing `tests/features_runner.rs` uses `GivnWorld::cucumber().fail_on_skipped()` and `givn/commands.yaml` already sets `verify.command: ./run-tests.sh`.
  - GREEN output: Strict runner wiring confirmed by the non-zero RED proof; no permissive runner path was added.
  - REFACTOR command: `cargo fmt --check`
  - REFACTOR output: Deferred to final verification; setup changes are formatted by the same check.
  - Production files: None; existing strict runner and command wiring were retained.
  - Commit: `a783bc3`
- [x] Create one regular step-definition file and one E2E step-definition file
      for `semantic-token-cap-review`; register both modules without empty step
      bodies.
  - RED command: `GIVN_FEATURES=givn/changes/semantic-token-cap-review/specs cargo test --features spec-retrieval,cross-encoder-reranker,nli-evidence --test features_runner -- --name "Permanent duplicate audit preserves BGE evidence when NLI reaches the cap"`
  - RED output: Non-zero strict undefined-step failure as above; the new step modules were then registered with `unimplemented!()` RED bodies.
  - GREEN command: `cargo check --test features_runner --features spec-retrieval,cross-encoder-reranker,nli-evidence`
  - GREEN output: `cargo check --test features_runner --features spec-retrieval,cross-encoder-reranker,nli-evidence` exited 0; the compiler accepted both registered modules.
  - REFACTOR command: `cargo fmt --check`
  - REFACTOR output: Deferred to final verification; no empty or no-op step bodies exist.
  - Production files: `tests/steps/semantic_token_cap_review_steps.rs`, `tests/steps/semantic_token_cap_review_e2e_steps.rs`, `tests/steps/mod.rs`, `run-tests.sh`, active feature tags.
  - Commit: `a783bc3`

## Scenario: Permanent duplicate audit preserves BGE evidence when NLI reaches the cap

- [ ] RED: remove `@wip`, implement the pending regular steps with
      `unimplemented!()`, and run only this scenario.
  - Command: `GIVN_FEATURES=givn/changes/semantic-token-cap-review/specs GIVN_SPEC_EMBEDDER=stub GIVN_SPEC_BGE=stub GIVN_SPEC_RERANKER=nli-stub cargo test --features spec-retrieval,cross-encoder-reranker,nli-evidence --test features_runner -- --name "Permanent duplicate audit preserves BGE evidence when NLI reaches the cap"`
  - Output: Non-zero (`cargo test` exit 101): the first regular step matched `semantic_token_cap_review_steps.rs:7` and panicked with `not implemented`; the strict runner reported `1 scenario (1 failed)`.
- [x] GREEN: implement the minimum token-cap fixture and NLI duplicate JSON /
      text assertions; run only this scenario successfully.
  - Command: `GIVN_FEATURES=givn/changes/semantic-token-cap-review/specs GIVN_SPEC_EMBEDDER=stub GIVN_SPEC_BGE=stub GIVN_SPEC_RERANKER=nli-stub cargo test --features spec-retrieval,cross-encoder-reranker,nli-evidence --test features_runner -- --name "Permanent duplicate audit preserves BGE evidence when NLI reaches the cap"`
  - Output: Zero exit; `1 feature`, `1 scenario (1 passed)`, `13 steps (13 passed)`. The JSON assertion proves BGE remains scored for the NLI-capped endpoint, NLI reports `NLI_TOKEN_CAP` with `257/255/513/512` counts, both runs report two E5 candidates, and the cap is excluded from filtered/combined recommendations with refactoring guidance.
  - Production files: `src/spec/retrieval.rs` (complete semantic serializer, pair-budget fixture/preflight, duplicate BGE/NLI cap state, JSON/text evidence); `tests/steps/mod.rs`; `tests/steps/semantic_token_cap_review_steps.rs`.
- [x] REFACTOR: simplify without changing behavior; rerun only this scenario.
  - Command: `cargo fmt --all && GIVN_FEATURES=givn/changes/semantic-token-cap-review/specs GIVN_SPEC_EMBEDDER=stub GIVN_SPEC_BGE=stub GIVN_SPEC_RERANKER=nli-stub cargo test --features spec-retrieval,cross-encoder-reranker,nli-evidence --test features_runner -- --name "Permanent duplicate audit preserves BGE evidence when NLI reaches the cap"`
  - Output: Zero exit after formatting; `1 feature`, `1 scenario (1 passed)`, `13 steps (13 passed)`.
- [ ] COMMIT: record the atomic scenario commit.
  - Commit:

## Scenario: Exactly 512 tokens keeps normal second-stage behavior

- [ ] RED: remove `@wip`, implement the pending regular steps with
      `unimplemented!()`, and run only this scenario.
  - Command:
  - Output:
- [ ] GREEN: implement exact-boundary/normal evidence assertions; run only this
      scenario successfully.
  - Command:
  - Output:
  - Production files:
- [ ] REFACTOR: simplify without changing behavior; rerun only this scenario.
  - Command:
  - Output:
- [ ] COMMIT: record the atomic scenario commit.
  - Commit:

## E2E Setup

- [ ] Configure and prove the strict E2E runner for `@e2e` scenarios; full
      runner count must be greater than the E2E-only count.
  - Full command/output:
  - E2E command/output:
  - Commit:

## Scenario: Permanent duplicate audit keeps partial BGE token-cap evidence per candidate

- [ ] RED: remove `@wip`, implement the E2E steps with `unimplemented!()`, and
      run only this E2E scenario through `./run-tests.sh --e2e`.
  - Command:
  - Output:
- [ ] GREEN: drive the compiled CLI subprocess and assert the duplicate JSON
      and refactoring guidance through the real interface.
  - Command:
  - Output:
  - Production files:
- [ ] REFACTOR: simplify the E2E steps without changing behavior; rerun only
      this scenario.
  - Command:
  - Output:
- [ ] COMMIT: record the atomic E2E scenario commit.
  - Commit:

## Scenario: Blocking change review keeps an all-over-cap candidate unresolved

- [ ] RED: remove `@wip`, implement the E2E steps with `unimplemented!()`, and
      run only this E2E scenario through `./run-tests.sh --e2e`.
  - Command:
  - Output:
- [ ] GREEN: drive blocking change review through the compiled CLI subprocess
      and assert the unresolved semantic report and refactoring guidance.
  - Command:
  - Output:
  - Production files:
- [ ] REFACTOR: simplify the E2E steps without changing behavior; rerun only
      this scenario.
  - Command:
  - Output:
- [ ] COMMIT: record the atomic E2E scenario commit.
  - Commit:

## Final Verification

- [ ] `givn lint --change semantic-token-cap-review` passes with no unexpected
      findings.
- [ ] `./run-tests.sh` passes all regular matrices.
- [ ] `./run-tests.sh --e2e` passes the E2E matrix.
- [ ] `cargo fmt --check` passes.
- [ ] `cargo clippy --workspace --all-targets -- -D warnings` passes.
- [ ] `givn check review --change semantic-token-cap-review` passes.
- [ ] Archive gate and review evidence are complete.
