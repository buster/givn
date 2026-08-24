# Tasks: semantic-model-download

## Setup

- [x] Configure the existing strict Cucumber runner for the active change,
  register `semantic_model_download_steps.rs` and
  `semantic_model_download_e2e_steps.rs`, add isolated XDG/download-fixture
  state, route `@nli @nli-combined` only through the combined feature phase, and
  record the strict-mode proof.
  - Runner command: `./run-tests.sh`
  - E2E command: `./run-tests.sh --e2e`
  - Strict mechanism: `GivnWorld::cucumber().fail_on_skipped()`
  - Strict proof command: `GIVN_FEATURES=givn/changes/semantic-model-download/specs cargo test --features spec-retrieval --test features_runner -- --name "A successful E5 setup is reused on the next indexing run"`
  - Strict proof output: exit non-zero; `1 feature`, `1 scenario (1 failed)`,
    `1 step (1 failed)`, with `Step panicked ... Captured output: not
    implemented`.
  - Full/e2e scenario-count proof: `./run-tests.sh` exit 0 with phase counts
    `336`, `34`, `9`, `7`, and `4` scenarios passed; `./run-tests.sh --e2e`
    exit 0 with phase counts `48`, `7`, `5`, and `1` scenarios passed. E2E is
    a strict subset of the full invocation (61 phase scenarios versus 390).

## Scenario: A successful E5 setup is reused on the next indexing run

- [x] RED: removed only this scenario's `@wip`; exact named run exited non-zero
  with `1 scenario (1 failed)`, `1 step (1 failed)`, and the defined Given step
  panicking with `not implemented`.
- [x] GREEN: resolved the XDG E5 cache and reused a complete cache marker without
  a second acquisition. Production files: `src/spec/retrieval.rs`. Test files:
  `tests/steps/mod.rs`, `tests/steps/semantic_model_download_steps.rs`, and
  `tests/steps/semantic_model_download_e2e_steps.rs`. Compile step:
  `cargo check --features spec-retrieval` passed. Targeted output: `1 scenario
  (1 passed)`, `5 steps (5 passed)`.
- [x] REFACTOR: ran `cargo fmt --all -- --check` and reran the named scenario;
  output: `1 scenario (1 passed)`, `5 steps (5 passed)`.
- [x] COMMIT: commit RED/GREEN/REFACTOR with the scenario title. Hash:
  `dd3079e`.

## Scenario: A failed model download explains how to provide the missing artifacts

- [x] RED: removed only this scenario's `@wip`; exact named run exited non-zero
  with `1 scenario (1 failed)`, `1 step (1 failed)`, and the defined Given step
  panicking with `not implemented`.
- [x] GREEN: reported the E5 cache, failed artifact, and local override guidance;
  left the index absent after E5 setup failure. Production files:
  `src/spec/retrieval.rs`. Test files: `tests/steps/semantic_model_download_steps.rs`.
  Compile step: `cargo check --features spec-retrieval` passed. Targeted output:
  `1 scenario (1 passed)`, `6 steps (6 passed)`.
- [x] REFACTOR: formatted the changed Rust files with `cargo fmt --all`, checked
  formatting, and reran the named scenario; output: `1 scenario (1 passed)`,
  `6 steps (6 passed)`.
- [x] COMMIT: commit RED/GREEN/REFACTOR with the scenario title. Hash:
  `c41b655`.

## Scenario: A failed model download leaves an existing permanent index unchanged

- [x] RED: removed only this scenario's `@wip`; exact named run exited non-zero
  with `1 scenario (1 failed)`, `1 step (1 failed)`, and the defined Given step
  panicking with `not implemented`.
- [x] GREEN: preserved pre-existing index bytes when E5 setup failed.
  Production files: `src/spec/retrieval.rs`. Test files:
  `tests/steps/semantic_model_download_steps.rs`. Compile step:
  `cargo check --features spec-retrieval` passed. Targeted output: `1 scenario
  (1 passed)`, `5 steps (5 passed)`.
- [x] REFACTOR: ran `cargo fmt --all -- --check` and reran the named scenario;
  output: `1 scenario (1 passed)`, `5 steps (5 passed)`.
- [x] COMMIT: commit RED/GREEN/REFACTOR with the scenario title. Hash:
  `f98bbd2`.

## Scenario: Malformed or incompatible indexes are not reported as absent

- [x] RED: removed only this scenario's `@wip`; exact named run exited non-zero
  with `1 scenario (1 failed)`, `1 step (1 failed)`, and the defined Given step
  panicking with `not implemented`.
- [x] GREEN: distinguished absent index files from malformed, incompatible, and
  dimension-invalid indexes while retaining validation errors. Production files:
  `src/spec/retrieval.rs`. Test files: `tests/steps/semantic_model_download_steps.rs`.
  Compile step: `cargo check --features spec-retrieval` passed. Targeted output:
  `1 scenario (1 passed)`, `5 steps (5 passed)`.
- [x] REFACTOR: formatted and checked the changed Rust files and reran the named
  scenario; output: `1 scenario (1 passed)`, `5 steps (5 passed)`.
- [x] COMMIT: commit RED/GREEN/REFACTOR with the scenario title. Hash:
  `94fde14`.

## Scenario: Duplicate audit identifies an absent persistent index without changing its fallback contract

- [x] RED: removed only this scenario's `@wip`; exact named run exited non-zero
  with `1 scenario (1 failed)`, `1 step (1 failed)`, and the defined Given step
  panicking with `not implemented`.
- [x] GREEN: emitted the shared missing-index repair diagnostic while retaining
  transient read-only duplicate auditing and no index write. Production files:
  `src/spec/retrieval.rs`. Test files: `tests/steps/mod.rs` and
  `tests/steps/semantic_model_download_steps.rs`. Compile step:
  `cargo check --features spec-retrieval` passed. Targeted output: `1 scenario
  (1 passed)`, `6 steps (6 passed)`.
- [x] REFACTOR: formatted and checked the changed Rust files, verified the
  diagnostic is emitted once, and reran the named scenario; output: `1 scenario
  (1 passed)`, `6 steps (6 passed)`.
- [x] COMMIT: commit RED/GREEN/REFACTOR with the scenario title. Hash:
  `b686cd6`.

## Scenario: Failed second-stage downloads remain unavailable and actionable

- [x] RED: removed only this scenario's `@wip`; exact named run exited non-zero
  with `1 scenario (1 failed)`, `1 step (1 failed)`, and the defined Given step
  panicking with `not implemented`.
- [x] GREEN: added structured BGE/NLI unavailable diagnostics, preserved valid
  duplicate JSON stdout, and retained `BGE_UNAVAILABLE`/`NLI_UNAVAILABLE`.
  Production files: `src/spec/retrieval.rs`. Test files:
  `tests/steps/semantic_model_download_steps.rs`. Compile step:
  `cargo check --features spec-retrieval,cross-encoder-reranker,nli-evidence`
  passed. Targeted output: `1 scenario (1 passed)`, `7 steps (7 passed)`.
- [x] REFACTOR: formatted and checked the changed Rust files and reran the named
  scenario; output: `1 scenario (1 passed)`, `7 steps (7 passed)`.
- [x] COMMIT: commit RED/GREEN/REFACTOR with the scenario title. Hash:
  `f8541ff`.

## Scenario: A corrupt second-stage cache is repaired before evidence scoring

- [x] RED: removed only this scenario's `@wip`; exact named run exited non-zero
  with `1 scenario (1 failed)`, `1 step (1 failed)`, and the defined Given step
  panicking with `not implemented`.
- [x] GREEN: validated BGE/NLI cache snapshots and reacquired corrupt role
  artifacts before scoring. Production files: `Cargo.toml`, `Cargo.lock`, and
  `src/spec/retrieval.rs`. Test files: `tests/common/mod.rs`,
  `tests/steps/mod.rs`, and `tests/steps/semantic_model_download_steps.rs`.
  Compile step: `cargo check --features spec-retrieval,cross-encoder-reranker,nli-evidence`
  passed. Targeted output: `1 scenario (1 passed)`, `7 steps (7 passed)`.
- [x] REFACTOR: formatted and checked the changed Rust files, asserted fixture
  request evidence for both repositories, and reran the named scenario; output:
  `1 scenario (1 passed)`, `7 steps (7 passed)`.
- [x] COMMIT: commit RED/GREEN/REFACTOR with the scenario title. Hash:
  `7c87c6d`.

## Scenario: Explicitly disabled evidence layers do not download models

- [x] RED: removed only this scenario's `@wip`; exact named run exited non-zero
  with `1 scenario (1 failed)`, `1 step (1 failed)`, and the defined Given step
  panicking with `not implemented`.
- [x] GREEN: ensured disabled BGE/NLI selectors bypass acquisition and preserve
  E5-only JSON output. Production files: none; existing selector behavior was
  reused. Test files: `tests/steps/semantic_model_download_steps.rs`. Compile
  step: `cargo check --features spec-retrieval` passed. Targeted output:
  `1 scenario (1 passed)`, `7 steps (7 passed)`.
- [x] REFACTOR: formatted and checked the changed Rust files and reran the named
  scenario; output: `1 scenario (1 passed)`, `7 steps (7 passed)`.
- [x] COMMIT: commit RED/GREEN/REFACTOR with the scenario title. Hash:
  `02b61b0`.

## Scenario: Archive maintenance does not download models implicitly

- [x] RED: removed only this scenario's `@wip`; exact named run exited non-zero
  with `1 scenario (1 failed)`, `1 step (1 failed)`, and the defined Given step
  panicking with `not implemented`.
- [x] GREEN: passed an explicit no-download acquisition policy through archive
  pre-merge and post-success refresh paths. Production files:
  `src/spec/retrieval.rs`, `src/cli/archive.rs`. Test files:
  `tests/steps/semantic_model_download_steps.rs`. Compile step:
  `cargo check --features spec-retrieval` passed. Targeted output: `1 scenario
  (1 passed)`, `5 steps (5 passed)`.
- [x] REFACTOR: formatted and checked the changed Rust files and reran the named
  scenario; output: `1 scenario (1 passed)`, `5 steps (5 passed)`.
- [x] COMMIT: commit RED/GREEN/REFACTOR with the scenario title. Hash:
  `0a4c4f9`.

## Scenario: NLI uses the bounded E5 neighbour union for the permanent duplicate audit

- [x] RED: removed only this scenario's `@wip`; exact named run exited non-zero
  with `1 scenario (1 failed)`, `1 step (1 failed)`, and the defined Given step
  panicking with `not implemented`.
- [x] GREEN: implemented and verified the amended independent full-E5 NLI
  audit-pool contract while preserving E5 identities/order. Production files:
  none; the existing implementation already matched the amended contract. Test
  files: `tests/steps/semantic_model_download_steps.rs`. Compile step:
  `cargo check --features spec-retrieval,nli-evidence` passed. Targeted output:
  `1 scenario (1 passed)`, `4 steps (4 passed)`.
- [x] REFACTOR: formatted and checked the changed Rust files and reran the named
  scenario; output: `1 scenario (1 passed)`, `4 steps (4 passed)`.
- [x] COMMIT: commit RED/GREEN/REFACTOR with the scenario title. Hash:
  `532d569`.

## Scenario: A feature-free build keeps retrieval explicitly unavailable

- [x] RED: removed only this scenario's `@wip`; exact no-default-feature run
  exited non-zero with `1 scenario (1 failed)`, `1 step (1 failed)`, and the
  undefined Given step rejected by strict mode.
- [x] GREEN: preserved explicit retrieval-unavailable behavior in the
  feature-free build. Production files: none; existing feature gate was reused.
  Test files: `tests/steps/semantic_model_download_steps.rs`. Compile step:
  `cargo check --no-default-features` passed. Targeted output: `1 scenario
  (1 passed)`, `4 steps (4 passed)`.
- [x] REFACTOR: ran `cargo fmt --all -- --check` and reran the named scenario;
  output: `1 scenario (1 passed)`, `4 steps (4 passed)`.
- [x] COMMIT: commit RED/GREEN/REFACTOR with the scenario title. Hash:
  `78bd375`.

## E2E Scenario: First semantic indexing downloads E5 artifacts before applying the token guard

- [x] E2E setup: local runnability is self-contained; the per-scenario local
  HF-compatible artifact fixture starts on loopback, and each E2E world receives
  an isolated `XDG_CACHE_HOME`. Strict E2E proof command:
  `GIVN_FEATURES=givn/changes/semantic-model-download/specs cargo test --features spec-retrieval --test features_runner -- --name "First semantic indexing downloads E5 artifacts before applying the token guard"`.
  It exited non-zero with `1 scenario (1 failed)`, `1 step (1 failed)`, and the
  E2E Given step panicking with `not implemented`. Baseline count proof:
  `./run-tests.sh` passed phase counts `336/34/9/7/4`; `./run-tests.sh --e2e`
  passed `48/7/5/1` (61 E2E phase scenarios versus 390 full phase scenarios).
- [x] RED: removed only this scenario's `@wip`; exact E2E run exited non-zero
  with `1 scenario (1 failed)`, `1 step (1 failed)`, and the defined Given step
  panicking with `not implemented`.
- [x] GREEN: drove the compiled `givn spec index` subprocess through the local
  HF-compatible source, asserted stdout/exit status, and verified the XDG E5
  cache. Production files: `src/spec/retrieval.rs`. Test files:
  `tests/common/mod.rs`, `tests/steps/semantic_model_download_e2e_steps.rs`,
  and `tests/steps/mod.rs`. Compile step: `cargo check --features
  spec-retrieval` passed. Targeted output: `1 scenario (1 passed)`, `7 steps
  (7 passed)`.
- [x] REFACTOR: ran `cargo fmt --all -- --check` and reran the named E2E
  scenario; output: `1 scenario (1 passed)`, `7 steps (7 passed)`.
- [x] COMMIT: commit the E2E scenario with its title. Hash: `e16fe88`.

## E2E Scenario: Search identifies the missing-index repair command

- [x] RED: removed only this scenario's `@wip`; exact E2E run exited non-zero
  with `1 scenario (1 failed)`, `1 step (1 failed)`, and the defined When step
  panicking with `not implemented`.
- [x] GREEN: drove the compiled `givn spec search` subprocess with no index and
  asserted the shared missing-index stderr diagnostic. Production files:
  `src/spec/retrieval.rs`. Test files:
  `tests/steps/semantic_model_download_e2e_steps.rs`. Compile step:
  `cargo check --features spec-retrieval` passed. Targeted output: `1 scenario
  (1 passed)`, `3 steps (3 passed)`.
- [x] REFACTOR: formatted and checked the changed Rust files and reran the named
  E2E scenario; output: `1 scenario (1 passed)`, `3 steps (3 passed)`.
- [x] COMMIT: commit the E2E scenario with its title. Hash: `8e875f7`.

## E2E Scenario: Explore identifies the missing-index repair command

- [x] RED: removed only this scenario's `@wip`; exact E2E run exited non-zero
  with `1 scenario (1 failed)`, `1 step (1 failed)`, and the defined And step
  panicking with `not implemented`.
- [x] GREEN: drove the compiled `givn spec explore` subprocess with no index and
  asserted the shared missing-index stderr diagnostic. Production files:
  `src/spec/retrieval.rs`. Test files:
  `tests/steps/semantic_model_download_e2e_steps.rs`. Compile step:
  `cargo check --features spec-retrieval` passed. Targeted output: `1 scenario
  (1 passed)`, `4 steps (4 passed)`.
- [x] REFACTOR: formatted and checked the changed Rust files and reran the named
  E2E scenario; output: `1 scenario (1 passed)`, `4 steps (4 passed)`.
- [ ] COMMIT: commit the E2E scenario with its title. Hash: _pending

## E2E Scenario: Advisory review identifies the missing-index repair command

- [ ] RED: remove only this scenario's `@wip`, add separate E2E step stubs with
  `unimplemented!()`, and run the exact E2E scenario. Output: _pending_
- [ ] GREEN: drive the compiled `givn spec review --change` subprocess with no
  index and assert the shared missing-index stderr diagnostic. Production files:
  _pending_. Test files: _pending_. Output: _pending_
- [ ] REFACTOR: preserve the primary CLI assertion and rerun the named scenario.
  Output: _pending_
- [ ] COMMIT: commit the E2E scenario with its title. Hash: _pending

## E2E Scenario: Blocking review identifies the missing-index repair command

- [ ] RED: remove only this scenario's `@wip`, add separate E2E step stubs with
  `unimplemented!()`, and run the exact E2E scenario. Output: _pending_
- [ ] GREEN: drive the compiled `givn check review --change` subprocess with no
  index and assert the shared missing-index stderr diagnostic. Production files:
  _pending_. Test files: _pending_. Output: _pending
- [ ] REFACTOR: preserve the primary CLI assertion and rerun the named scenario.
  Output: _pending_
- [ ] COMMIT: commit the E2E scenario with its title. Hash: _pending

## E2E Scenario: Duplicate audit identifies the second-stage model caches

- [ ] RED: remove only this scenario's `@wip`, add separate E2E step stubs with
  `unimplemented!()`, and run the exact combined E2E scenario. Output: _pending_
- [ ] GREEN: drive the compiled `givn spec duplicates --format json` subprocess
  through the combined BGE/NLI feature build, assert JSON stdout and exit
  status, and verify both XDG caches as secondary evidence. Production files:
  _pending_. Test files: _pending_. Output: _pending_
- [ ] REFACTOR: simplify the combined E2E binding without changing assertions and
  rerun the named scenario. Output: _pending_
- [ ] COMMIT: commit the E2E scenario with its title. Hash: _pending

## Final Verification

- [ ] `givn lint --change semantic-model-download` passes with no WIP or other
  findings.
- [ ] `./run-tests.sh` passes and includes the active change feature path at the
  mandatory task/review gates.
- [ ] `./run-tests.sh --e2e` passes with a strict subset count and the combined
  BGE/NLI scenario runs only in the combined feature phase.
- [ ] Default-feature unit tests pass, minimal `--no-default-features` tests
  pass, and XDG/override precedence tests pass.
- [ ] Real local E5/BGE/NLI probes confirm cold-cache setup, cache reuse, and
  actionable offline failure without writing model files into a project.
- [ ] `givn check review --change semantic-model-download` passes all required
  gates.
