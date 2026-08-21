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
- [ ] COMMIT: commit RED/GREEN/REFACTOR with the scenario title. Hash: _pending_

## Scenario: BGE receives the highest E5 candidates within the configured budget

- [ ] RED: remove only this scenario's `@wip`, add step stubs with
  `unimplemented!()`, and run the exact named scenario. Output: _pending_
- [ ] GREEN: implement E5-descending BGE prefix selection and explicit
  `BGE_UNSCORED` accounting. Production files: _pending_. Output: _pending_
- [ ] REFACTOR: simplify without behavior change and rerun the named scenario.
  Output: _pending_
- [ ] COMMIT: commit RED/GREEN/REFACTOR with the scenario title. Hash: _pending

## Scenario: NLI receives the highest E5 candidates within the configured budget

- [ ] RED: remove only this scenario's `@wip`, add step stubs with
  `unimplemented!()`, and run the exact named scenario. Output: _pending_
- [ ] GREEN: apply the same ordered budget independently to NLI and preserve
  NLI unscored accounting. Production files: _pending_. Output: _pending_
- [ ] REFACTOR: simplify without behavior change and rerun the named scenario.
  Output: _pending_
- [ ] COMMIT: commit RED/GREEN/REFACTOR with the scenario title. Hash: _pending

## Scenario: An invalid second-stage pair budget is rejected

- [ ] RED: remove only this scenario's `@wip`, add step stubs with
  `unimplemented!()`, and run the exact named scenario. Output: _pending_
- [ ] GREEN: reject malformed, empty, zero, negative, overflowing, and invalid
  setting values with the setting named. Production files: _pending_. Output:
  _pending_
- [ ] REFACTOR: simplify without behavior change and rerun the named scenario.
  Output: _pending_
- [ ] COMMIT: commit RED/GREEN/REFACTOR with the scenario title. Hash: _pending

## Scenario: A zero second-stage pair budget is rejected

- [ ] RED: remove only this scenario's `@wip`, add step stubs with
  `unimplemented!()`, and run the exact named scenario. Output: _pending_
- [ ] GREEN: enforce the positive-value boundary and run the named scenario.
  Production files: _pending_. Output: _pending_
- [ ] REFACTOR: simplify without behavior change and rerun the named scenario.
  Output: _pending_
- [ ] COMMIT: commit RED/GREEN/REFACTOR with the scenario title. Hash: _pending

## Scenario: BGE and NLI select the same highest E5 candidates independently

- [ ] RED: remove only this scenario's `@wip`, add step stubs with
  `unimplemented!()`, and run the exact named scenario. Output: _pending_
- [ ] GREEN: implement independent prefix selection and combined intersection
  accounting. Production files: _pending_. Output: _pending_
- [ ] REFACTOR: simplify without behavior change and rerun the named scenario.
  Output: _pending_
- [ ] COMMIT: commit RED/GREEN/REFACTOR with the scenario title. Hash: _pending

## Scenario: The CLI reports the configured second-stage pair budget

- [ ] E2E setup: verify the configured real CLI command and strict E2E tag
  filtering; full/e2e count proof: _pending_.
- [ ] RED: remove only this scenario's `@wip`, add E2E stubs, and run the exact
  E2E scenario. Output: _pending_
- [ ] GREEN: drive the compiled `givn spec duplicates --format json` subprocess
  and assert budget/count fields in stdout. Production files: _pending_.
  Output: _pending_
- [ ] REFACTOR: simplify E2E bindings without changing assertions and rerun the
  named scenario. Output: _pending_
- [ ] COMMIT: commit the E2E scenario with its title. Hash: _pending

## Final Verification

- [ ] `givn lint --change semantic-reranker-budget` passes with no WIP or other
  findings.
- [ ] `./run-tests.sh` passes.
- [ ] `./run-tests.sh --e2e` passes with a strict subset count.
- [ ] Real local BGE/NLI duplicate audit is rerun with default `2048`; runtime,
  scored counts, filtered counts, and unscored counts are recorded in the
  filter report.
- [ ] `givn check review --change semantic-reranker-budget` passes all required
  gates.
