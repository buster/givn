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

- [x] RED: remove `@wip`, implement the pending regular steps with
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
- [x] COMMIT: record the atomic scenario commit.
  - Commit: `b233079`

## Scenario: Exactly 512 tokens keeps normal second-stage behavior

- [x] RED: remove `@wip`, implement the pending regular steps with
      `unimplemented!()`, and run only this scenario.
  - Command: `GIVN_FEATURES=givn/changes/semantic-token-cap-review/specs GIVN_SPEC_EMBEDDER=stub GIVN_SPEC_BGE=stub GIVN_SPEC_RERANKER=nli-stub cargo test --features spec-retrieval,cross-encoder-reranker,nli-evidence --test features_runner -- --name "Exactly 512 tokens keeps normal second-stage behavior"`
  - Output: Zero exit is legitimate immediate GREEN: all steps were real shared definitions from the prior scenario, so there was no new undefined or pending body. The targeted run reported `1 feature`, `1 scenario (1 passed)`, `8 steps (8 passed)`.
- [x] GREEN: implement exact-boundary/normal evidence assertions; run only this
      scenario successfully.
  - Command: `GIVN_FEATURES=givn/changes/semantic-token-cap-review/specs GIVN_SPEC_EMBEDDER=stub GIVN_SPEC_BGE=stub GIVN_SPEC_RERANKER=nli-stub cargo test --features spec-retrieval,cross-encoder-reranker,nli-evidence --test features_runner -- --name "Exactly 512 tokens keeps normal second-stage behavior"`
  - Output: Zero exit; `1 feature`, `1 scenario (1 passed)`, `8 steps (8 passed)`. The exact fixture produced scored BGE/NLI evidence with no `TOKEN_CAP` or `token-cap` output.
  - Production files: `src/spec/retrieval.rs` (`token_budget_fits` makes the inclusive `pair_tokens <= limit` boundary explicit); shared regular step fixture/assertions were reused.
- [x] REFACTOR: simplify without changing behavior; rerun only this scenario.
  - Command: `cargo fmt --all && GIVN_FEATURES=givn/changes/semantic-token-cap-review/specs GIVN_SPEC_EMBEDDER=stub GIVN_SPEC_BGE=stub GIVN_SPEC_RERANKER=nli-stub cargo test --features spec-retrieval,cross-encoder-reranker,nli-evidence --test features_runner -- --name "Exactly 512 tokens keeps normal second-stage behavior"`
  - Output: Zero exit after formatting; `1 feature`, `1 scenario (1 passed)`, `8 steps (8 passed)`.
- [x] COMMIT: record the atomic scenario commit.
  - Commit: `87baf9f`

## E2E Setup

- [x] Configure and prove the strict E2E runner for `@e2e` scenarios; full
      runner count must be greater than the E2E-only count.
  - Full command/output: `GIVN_FEATURES=givn/changes/semantic-token-cap-review/specs GIVN_SPEC_EMBEDDER=stub GIVN_SPEC_BGE=stub GIVN_SPEC_RERANKER=nli-stub cargo test --features spec-retrieval,cross-encoder-reranker,nli-evidence --test features_runner -- --concurrency 1` produced `1 feature`, `4 scenarios (2 passed, 2 failed)`, `23 steps (21 passed, 2 failed)` while the two E2E step files were intentionally still RED stubs.
  - E2E command/output: `GIVN_FEATURES=givn/changes/semantic-token-cap-review/specs GIVN_SPEC_EMBEDDER=stub GIVN_SPEC_BGE=stub GIVN_SPEC_RERANKER=nli-stub cargo test --features spec-retrieval,cross-encoder-reranker,nli-evidence --test features_runner -- --tags '@e2e' --concurrency 1` selected exactly `2 scenarios`, both failed on `unimplemented!()` with a non-zero exit. The E2E command is the strict subset `./run-tests.sh --e2e`, whose combined matrix now includes `@nli-combined and @e2e`; full active count 4 is strictly greater than E2E-only count 2.
  - Commit: `a783bc3`

## Scenario: Permanent duplicate audit keeps partial BGE token-cap evidence per candidate

- [x] RED: remove `@wip`, implement the E2E steps with `unimplemented!()`, and
      run only this E2E scenario through `./run-tests.sh --e2e`.
  - Command: `GIVN_FEATURES=givn/changes/semantic-token-cap-review/specs GIVN_SPEC_EMBEDDER=stub GIVN_SPEC_BGE=stub cargo test --features spec-retrieval,cross-encoder-reranker --test features_runner -- --name "Permanent duplicate audit keeps partial BGE token-cap evidence per candidate"`
  - Output: Non-zero (`cargo test` exit 101): the first E2E step matched `semantic_token_cap_review_e2e_steps.rs:7` and panicked with `not implemented`; the strict runner reported `1 scenario (1 failed)`. The direct command is the scenario-targeted equivalent of the configured `./run-tests.sh --e2e` BGE matrix.
- [x] GREEN: drive the compiled CLI subprocess and assert the duplicate JSON
      and refactoring guidance through the real interface.
  - Command: `GIVN_FEATURES=givn/changes/semantic-token-cap-review/specs GIVN_SPEC_EMBEDDER=stub GIVN_SPEC_BGE=stub cargo test --features spec-retrieval,cross-encoder-reranker --test features_runner -- --name "Permanent duplicate audit keeps partial BGE token-cap evidence per candidate"`
  - Output: Zero exit; `1 feature`, `1 scenario (1 passed)`, `12 steps (12 passed)`. The compiled `givn spec duplicates --format json` subprocess exposed the 513-token candidate in `candidates`, emitted `BGE_TOKEN_CAP` with `257/255/513/512`, kept `BGE_ENABLED`, omitted it from BGE filtered candidates, and emitted refactoring guidance.
  - Production files: `src/spec/retrieval.rs`; E2E fixture/real-CLI steps in `tests/steps/semantic_token_cap_review_e2e_steps.rs` and shared fixture in `tests/steps/semantic_token_cap_review_steps.rs`.
- [x] REFACTOR: simplify the E2E steps without changing behavior; rerun only
      this scenario.
  - Command: `cargo fmt --all && GIVN_FEATURES=givn/changes/semantic-token-cap-review/specs GIVN_SPEC_EMBEDDER=stub GIVN_SPEC_BGE=stub cargo test --features spec-retrieval,cross-encoder-reranker --test features_runner -- --name "Permanent duplicate audit keeps partial BGE token-cap evidence per candidate"`
  - Output: Zero exit after formatting; `1 feature`, `1 scenario (1 passed)`, `12 steps (12 passed)`.
- [x] COMMIT: record the atomic E2E scenario commit.
  - Commit: `e3a48c5`

## Scenario: Blocking change review keeps an all-over-cap candidate unresolved

- [x] RED: remove `@wip`, implement the E2E steps with `unimplemented!()`, and
      run only this E2E scenario through `./run-tests.sh --e2e`.
  - Command: `GIVN_FEATURES=givn/changes/semantic-token-cap-review/specs GIVN_SPEC_EMBEDDER=stub GIVN_SPEC_BGE=stub GIVN_SPEC_RERANKER=nli-stub cargo test --features spec-retrieval,cross-encoder-reranker,nli-evidence --test features_runner -- --name "Blocking change review keeps an all-over-cap candidate unresolved"`
  - Output: Non-zero (`cargo test` exit 101): the first E2E step matched `semantic_token_cap_review_e2e_steps.rs:134` and panicked with `not implemented`; the strict runner reported `1 scenario (1 failed)`. The direct command is the scenario-targeted equivalent of the configured combined E2E matrix.
- [x] GREEN: drive blocking change review through the compiled CLI subprocess
      and assert the unresolved semantic report and refactoring guidance.
  - Command: `GIVN_FEATURES=givn/changes/semantic-token-cap-review/specs GIVN_SPEC_EMBEDDER=stub GIVN_SPEC_BGE=stub GIVN_SPEC_RERANKER=nli-stub cargo test --features spec-retrieval,cross-encoder-reranker,nli-evidence --test features_runner -- --name "Blocking change review keeps an all-over-cap candidate unresolved"`
  - Output: Zero exit; `1 feature`, `1 scenario (1 passed)`, `11 steps (11 passed)`. The compiled `givn check review --change running-change` subprocess returned non-zero for the unresolved candidate and wrote BGE/NLI `token-cap` unavailable runs, both per-candidate cap admissions, exact `257/255/513/512` counts, blocking `UNCERTAIN`, and shorten/split guidance.
  - Production files: `src/spec/retrieval.rs` (semantic-review BGE/NLI preflight and cap report fields, E5 tokenizer preflight); `tests/steps/semantic_token_cap_review_e2e_steps.rs`; `tests/steps/bge_reranking_evidence_steps.rs` (reused blocking-review action passes the deterministic NLI seam).
- [x] REFACTOR: simplify the E2E steps without changing behavior; rerun only
      this scenario.
  - Command: `cargo fmt --all && GIVN_FEATURES=givn/changes/semantic-token-cap-review/specs GIVN_SPEC_EMBEDDER=stub GIVN_SPEC_BGE=stub GIVN_SPEC_RERANKER=nli-stub cargo test --features spec-retrieval,cross-encoder-reranker,nli-evidence --test features_runner -- --name "Blocking change review keeps an all-over-cap candidate unresolved"`
  - Output: Zero exit after formatting; `1 feature`, `1 scenario (1 passed)`, `11 steps (11 passed)`.
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
