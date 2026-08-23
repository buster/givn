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
- [ ] COMMIT: commit RED/GREEN/REFACTOR with the scenario title. Hash: _pending_

## Scenario: A failed model download explains how to provide the missing artifacts

- [ ] RED: remove only this scenario's `@wip`, add `unimplemented!()` step
  bodies, and run the exact named scenario. Output: _pending_
- [ ] GREEN: report role, cache, failed artifact, and local override guidance;
  leave the index absent after E5 acquisition failure. Production files:
  _pending_. Test files: _pending_. Output: _pending_
- [ ] REFACTOR: preserve actionable error formatting and rerun the named
  scenario. Output: _pending_
- [ ] COMMIT: commit RED/GREEN/REFACTOR with the scenario title. Hash: _pending_

## Scenario: A failed model download leaves an existing permanent index unchanged

- [ ] RED: remove only this scenario's `@wip`, add `unimplemented!()` step
  bodies, and run the exact named scenario. Output: _pending_
- [ ] GREEN: preserve pre-existing index bytes when E5 acquisition fails.
  Production files: _pending_. Test files: _pending_. Output: _pending_
- [ ] REFACTOR: simplify failure transaction handling without behavior change
  and rerun the named scenario. Output: _pending_
- [ ] COMMIT: commit RED/GREEN/REFACTOR with the scenario title. Hash: _pending_

## Scenario: Malformed or incompatible indexes are not reported as absent

- [ ] RED: remove only this scenario's `@wip`, add `unimplemented!()` step
  bodies, and run the exact named scenario. Output: _pending_
- [ ] GREEN: distinguish absent index files from malformed, incompatible, and
  dimension-invalid indexes while retaining the existing validation error.
  Production files: _pending_. Test files: _pending_. Output: _pending_
- [ ] REFACTOR: centralize index-read error classification without behavior
  change and rerun the named scenario. Output: _pending_
- [ ] COMMIT: commit RED/GREEN/REFACTOR with the scenario title. Hash: _pending_

## Scenario: Duplicate audit identifies an absent persistent index without changing its fallback contract

- [ ] RED: remove only this scenario's `@wip`, add `unimplemented!()` step
  bodies, and run the exact named scenario. Output: _pending_
- [ ] GREEN: emit the shared missing-index repair diagnostic while retaining
  transient read-only duplicate auditing and no index write. Production files:
  _pending_. Test files: _pending_. Output: _pending_
- [ ] REFACTOR: ensure the diagnostic is emitted once and rerun the named
  scenario. Output: _pending_
- [ ] COMMIT: commit RED/GREEN/REFACTOR with the scenario title. Hash: _pending_

## Scenario: Failed second-stage downloads remain unavailable and actionable

- [ ] RED: remove only this scenario's `@wip`, add `unimplemented!()` step
  bodies, and run the exact named scenario. Output: _pending_
- [ ] GREEN: add structured BGE/NLI acquisition diagnostics, preserve valid
  duplicate JSON stdout, and retain `BGE_UNAVAILABLE`/`NLI_UNAVAILABLE`.
  Production files: _pending_. Test files: _pending_. Output: _pending_
- [ ] REFACTOR: simplify shared second-stage failure mapping and rerun the named
  scenario. Output: _pending_
- [ ] COMMIT: commit RED/GREEN/REFACTOR with the scenario title. Hash: _pending_

## Scenario: A corrupt second-stage cache is repaired before evidence scoring

- [ ] RED: remove only this scenario's `@wip`, add `unimplemented!()` step
  bodies, and run the exact named scenario. Output: _pending_
- [ ] GREEN: validate manifest, revision, size, digest, metadata, and snapshot
  consistency; reacquire only corrupt BGE/NLI snapshots before scoring.
  Production files: _pending_. Test files: _pending_. Output: _pending_
- [ ] REFACTOR: isolate role-specific cache repair without behavior change and
  rerun the named scenario. Output: _pending_
- [ ] COMMIT: commit RED/GREEN/REFACTOR with the scenario title. Hash: _pending_

## Scenario: Explicitly disabled evidence layers do not download models

- [ ] RED: remove only this scenario's `@wip`, add `unimplemented!()` step
  bodies, and run the exact named scenario. Output: _pending_
- [ ] GREEN: ensure disabled BGE/NLI selectors bypass acquisition and preserve
  E5-only JSON output. Production files: _pending_. Test files: _pending_.
  Output: _pending_
- [ ] REFACTOR: simplify selector checks without behavior change and rerun the
  named scenario. Output: _pending_
- [ ] COMMIT: commit RED/GREEN/REFACTOR with the scenario title. Hash: _pending_

## Scenario: Archive maintenance does not download models implicitly

- [ ] RED: remove only this scenario's `@wip`, add `unimplemented!()` step
  bodies, and run the exact named scenario. Output: _pending_
- [ ] GREEN: pass an explicit no-download acquisition policy through archive
  pre-merge and post-success refresh paths. Production files: _pending_. Test
  files: _pending_. Output: _pending_
- [ ] REFACTOR: keep archive download policy explicit and rerun the named
  scenario. Output: _pending_
- [ ] COMMIT: commit RED/GREEN/REFACTOR with the scenario title. Hash: _pending_

## Scenario: NLI uses the bounded E5 neighbour union for the permanent duplicate audit

- [ ] RED: remove only this scenario's `@wip`, add `unimplemented!()` step
  bodies, and run the exact named scenario. Output: _pending_
- [ ] GREEN: implement the amended independent full-E5 NLI audit-pool contract
  and preserve E5 identities/order. Production files: _pending_. Test files:
  _pending_. Output: _pending_
- [ ] REFACTOR: align wording and shared pool helpers without behavior change and
  rerun the named scenario. Output: _pending_
- [ ] COMMIT: commit RED/GREEN/REFACTOR with the scenario title. Hash: _pending

## Scenario: A feature-free build keeps retrieval explicitly unavailable

- [ ] RED: remove only this scenario's `@wip`, add `unimplemented!()` step
  bodies, and run the exact named scenario. Output: _pending_
- [ ] GREEN: preserve explicit retrieval-unavailable behavior in the
  feature-free build. Production files: _pending_. Test files: _pending_.
  Output: _pending_
- [ ] REFACTOR: keep feature gating explicit and rerun the named scenario.
  Output: _pending_
- [ ] COMMIT: commit RED/GREEN/REFACTOR with the scenario title. Hash: _pending

## E2E Scenario: First semantic indexing downloads E5 artifacts before applying the token guard

- [ ] E2E setup: start the local deterministic HF-compatible artifact source,
  isolate `XDG_CACHE_HOME`, prove strict E2E filtering, and record full/e2e
  counts. Evidence: _pending_.
- [ ] RED: remove only this scenario's `@wip`, add separate E2E step stubs with
  `unimplemented!()`, and run the exact E2E scenario. Output: _pending_
- [ ] GREEN: drive the compiled `givn spec index` subprocess, assert stdout and
  exit status, and verify the XDG E5 cache as secondary evidence. Production
  files: _pending_. Test files: _pending_. Output: _pending_
- [ ] REFACTOR: simplify the E2E binding without changing the CLI assertions and
  rerun the named scenario. Output: _pending_
- [ ] COMMIT: commit the E2E scenario with its title. Hash: _pending

## E2E Scenario: Search identifies the missing-index repair command

- [ ] RED: remove only this scenario's `@wip`, add separate E2E step stubs with
  `unimplemented!()`, and run the exact E2E scenario. Output: _pending_
- [ ] GREEN: drive the compiled `givn spec search` subprocess with no index and
  assert the shared missing-index stderr diagnostic. Production files:
  _pending_. Test files: _pending_. Output: _pending_
- [ ] REFACTOR: preserve the primary CLI assertion and rerun the named scenario.
  Output: _pending_
- [ ] COMMIT: commit the E2E scenario with its title. Hash: _pending

## E2E Scenario: Explore identifies the missing-index repair command

- [ ] RED: remove only this scenario's `@wip`, add separate E2E step stubs with
  `unimplemented!()`, and run the exact E2E scenario. Output: _pending_
- [ ] GREEN: drive the compiled `givn spec explore` subprocess with no index
  and assert the shared missing-index stderr diagnostic. Production files:
  _pending_. Test files: _pending_. Output: _pending_
- [ ] REFACTOR: preserve the primary CLI assertion and rerun the named scenario.
  Output: _pending_
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
