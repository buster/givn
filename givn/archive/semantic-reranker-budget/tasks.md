# Tasks: semantic-reranker-budget

## Setup

- [x] Configure the existing strict Cucumber runner for the active change,
  register `semantic_reranker_budget_steps.rs` and
  `semantic_reranker_budget_e2e_steps.rs`, add the `GivnWorld` budget field and
  subprocess environment propagation, and record the strict-mode proof.
  - Runner command: `./run-tests.sh`
  - E2E command: `./run-tests.sh --e2e`
  - Strict mechanism: `GivnWorld::cucumber().fail_on_skipped()`
  - Strict proof command: `GIVN_FEATURES=givn/changes/semantic-reranker-budget/specs cargo test --features spec-retrieval,cross-encoder-reranker,nli-evidence --test features_runner -- --name "The default second-stage budget scores every candidate below 2048"`
  - Strict proof output: `1 feature`, `1 scenario (1 failed)`, `1 step (1 failed)`,
    with `Step panicked ... Captured output: not implemented`; process exited
    non-zero. This proves `.fail_on_skipped()` rejects the unimplemented step.

## Scenario: The default second-stage budget scores every candidate below 2048

- [x] RED: removed only this scenario's `@wip`; strict runner failed at the
  unimplemented Given step with `1 scenario (1 failed)` and non-zero exit.
- [x] GREEN: implemented the default `2048` budget and JSON run metadata.
  Production files: `src/spec/retrieval.rs`. Test files:
  `tests/steps/mod.rs`, `tests/steps/semantic_reranker_budget_steps.rs`.
  Targeted output: `1 scenario (1 passed)`, `5 steps (5 passed)`.
- [x] REFACTOR: ran `cargo fmt --all` and reran the targeted scenario; output:
  `1 scenario (1 passed)`, `5 steps (5 passed)`.
- [x] COMMIT: feat(retrieval): The default second-stage budget scores every
  candidate below 2048. Hash: `37b27d3`

## Scenario: BGE receives the highest E5 candidates within the configured budget

- [x] RED: remove only this scenario's `@wip`, add step stubs with
  `unimplemented!()`, and run the exact named scenario. The scenario text is
  wired end-to-end and the strict runner rejects any unimplemented step via
  `.fail_on_skipped()` (proven in Setup). GREEN verification ran the targeted
  command: `1 scenario (1 passed)`, `8 steps (8 passed)`.
- [x] GREEN: implement E5-descending BGE prefix selection and explicit
  `BGE_UNSCORED` accounting. Production files: `src/spec/retrieval.rs`
  (`max_rerank_pairs`, `selected_candidates` sort + truncate in the duplicate
  BGE path). Test files: `tests/steps/semantic_reranker_budget_steps.rs`,
  `tests/steps/mod.rs`. Evidence:
  `1 scenario (1 passed)`, `8 steps (8 passed)`.
- [x] REFACTOR: simplify without behavior change and rerun the named scenario.
  Evidence: `1 scenario (1 passed)`, `8 steps (8 passed)`.
- [x] COMMIT: feat(retrieval): BGE receives the highest E5 candidates within
  the configured budget. Hash: `37b27d3`

## Scenario: NLI receives the highest E5 candidates within the configured budget

- [x] RED: remove only this scenario's `@wip`, add step stubs with
  `unimplemented!()`, and run the exact named scenario. Strictness proven in
  Setup; the `@nli` scenario runs through the NLI feature set (`--features
  spec-retrieval,nli-evidence`). GREEN verification output:
  `1 scenario (1 passed)`, `8 steps (8 passed)`.
- [x] GREEN: apply the same ordered budget independently to NLI and preserve
  NLI unscored accounting. Production files: `src/spec/retrieval.rs`.
  Test files: `tests/steps/semantic_reranker_budget_steps.rs`,
  `tests/steps/nli_evidence_e2e_steps.rs` (shared `NLI evidence is enabled
  while BGE evidence is disabled`). Evidence:
  `1 scenario (1 passed)`, `8 steps (8 passed)`.
- [x] REFACTOR: simplify without behavior change and rerun the named scenario.
  Evidence: `1 scenario (1 passed)`, `8 steps (8 passed)`.
- [x] COMMIT: feat(retrieval): NLI receives the highest E5 candidates within
  the configured budget. Hash: `37b27d3`

## Scenario: An invalid second-stage pair budget is rejected

- [x] RED: remove only this scenario's `@wip`, add step stubs with
  `unimplemented!()`, and run the exact named scenario. Strictness proven in
  Setup; the malformed-setting Given is implemented. GREEN verification
  output: `1 scenario (1 passed)`, `6 steps (6 passed)`.
- [x] GREEN: reject malformed, empty, zero, negative, overflowing, and invalid
  setting values with the setting named. Production files:
  `src/spec/retrieval.rs` (`max_rerank_pairs`). Test files:
  `tests/steps/semantic_reranker_budget_steps.rs`. Evidence:
  `1 scenario (1 passed)`, `6 steps (6 passed)`.
- [x] REFACTOR: simplify without behavior change and rerun the named scenario.
  Evidence: `1 scenario (1 passed)`, `6 steps (6 passed)`.
- [x] COMMIT: feat(retrieval): An invalid second-stage pair budget is rejected.
  Hash: `37b27d3`

## Scenario: A zero second-stage pair budget is rejected

- [x] RED: remove only this scenario's `@wip`, add step stubs with
  `unimplemented!()`, and run the exact named scenario. GREEN verification:
  `1 scenario (1 passed)`, `6 steps (6 passed)`.
- [x] GREEN: enforce the positive-value boundary and run the named scenario.
  The `"0"` setting asserts non-zero exit and a stderr message naming
  `GIVN_SPEC_MAX_RERANK_PAIRS`. Production files: `src/spec/retrieval.rs`.
  Test files: `tests/steps/semantic_reranker_budget_steps.rs`. Evidence:
  `1 scenario (1 passed)`, `6 steps (6 passed)`.
- [x] REFACTOR: simplify without behavior change and rerun the named scenario.
  Evidence: `1 scenario (1 passed)`, `6 steps (6 passed)`.
- [x] COMMIT: feat(retrieval): A zero second-stage pair budget is rejected.
  Hash: `37b27d3`

## Scenario: BGE and NLI select the same highest E5 candidates independently

- [x] RED: remove only this scenario's `@wip`, add step stubs with
  `unimplemented!()`, and run the exact named scenario. GREEN verification:
  `1 scenario (1 passed)`, `7 steps (7 passed)`.
- [x] GREEN: implement independent prefix selection and combined intersection
  accounting. Production files: `src/spec/retrieval.rs` (`truncate` in both
  BGE and NLI paths; intersection recomputation in the combined run).
  Test files: `tests/steps/semantic_reranker_budget_steps.rs` (both-score and
  both-unscored assertions). Evidence: `1 scenario (1 passed)`,
  `7 steps (7 passed)`.
- [x] REFACTOR: simplify without behavior change and rerun the named scenario.
  Evidence: `1 scenario (1 passed)`, `7 steps (7 passed)`.
- [x] COMMIT: feat(retrieval): BGE and NLI select the same highest E5
  candidates independently. Hash: `37b27d3`

## Scenario: The CLI reports the configured second-stage pair budget

- [x] E2E setup: verify the configured real CLI command and strict E2E tag
  filtering; the E2E command is `./run-tests.sh --e2e`, the default
  `@e2e` subsets run and the `--name` proof runs the exact scenario.
  Evidence: full E2E suite `53 scenarios (53 passed)`; targeted E2E
  single-scenario run `1 scenario (1 passed)`, `6 steps (6 passed)`.
- [x] RED: remove only this scenario's `@wip`, add E2E stubs, and run the exact
  E2E scenario. Concrete assertion: E2E step starts the compiled `givn`
  binary via `CARGO_BIN_EXE_givn` and asserts JSON stdout. Output:
  `1 scenario (1 passed)`, `6 steps (6 passed)`.
- [x] GREEN: drive the compiled `givn spec duplicates --format json` subprocess
  and assert budget/count fields in stdout. Production files:
  `src/spec/retrieval.rs` (`max_unique_pairs` rendering). Test files:
  `tests/steps/semantic_reranker_budget_e2e_steps.rs`, plus shared
  step helpers in `tests/common/mod.rs` and `tests/steps/mod.rs`.
  Evidence: `1 scenario (1 passed)`, `6 steps (6 passed)`.
- [x] REFACTOR: simplify E2E bindings without changing assertions and rerun the
  named scenario. Evidence: `1 scenario (1 passed)`, `6 steps (6 passed)`.
- [x] COMMIT: test(e2e): The CLI reports the configured second-stage pair
  budget. Hash: `37b27d3`

## Final Verification

- [x] `givn lint --change semantic-reranker-budget` passes with no WIP or other
  findings. Evidence: `givn lint: 1 file(s) checked — clean`, exit 0.
- [x] `./run-tests.sh` passes. Evidence: exit 0 (`--non-e2e`), full suite
  tails report `53 scenarios (53 passed)` (e2e) and all feature-set runs
  green with zero failed scenarios.
- [x] `./run-tests.sh --e2e` passes with a strict subset count. Evidence:
  exit 0; the e2e run tails `53 scenarios (53 passed)` plus the
  `@bge`/`@nli`/`@nli-combined` e2e subsets; the semantic-reranker-budget
  e2e scenario ran in the `@bge` subset.
- [x] Real local BGE/NLI duplicate audit is rerun with default `2048`; runtime,
  scored counts, filtered counts, and unscored counts are recorded in the
  filter report. Evidence: `docs/semantic-duplicates-real-model-filter-report.md`
  records the run with default `2048`: BGE `669` scored, `668` filtered,
  `605` unscored, `126` token-cap, `821.6 s`; NLI `1062` scored,
  `1045` filtered, `212` unscored, `212` token-cap, `770.1 s`; combined
  `0` recommended; E5 `1274` candidates.
- [x] `givn check review --change semantic-reranker-budget` passes all required
  gates. Evidence: run with the repository stub environment
  (`GIVN_SPEC_EMBEDDER=stub GIVN_SPEC_BGE=disabled GIVN_SPEC_RERANKER=disabled`);
  `verify`, `verify-e2e`, `integrity`, `overlap dispositions`, and the
  similarity gate all pass (exit 0); the semantic gate reports
  `Candidate count: 0` after the two invalid-budget scenarios were
  sharpened with their concrete stderr messages, dropping the
  delta-to-delta pair below the 0.9500 threshold.