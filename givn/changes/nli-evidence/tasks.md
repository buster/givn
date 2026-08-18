# Tasks: nli-evidence

RED -> GREEN -> REFACTOR -> COMMIT per scenario. Non-E2E scenarios run before
the five CLI E2E smoke scenarios. Each completed checkbox contains runner
evidence and its atomic commit hash.

## Setup

- [x] **Strict runner and step skeletons**: register the regular and E2E NLI
  step modules, extend `run-tests.sh` with E5+BGE/NLI feature-matrix commands,
  and prove `.fail_on_skipped()` rejects an `unimplemented!()` step with a
  non-zero result.
  ```text
  GIVN_FEATURES=givn/changes/nli-evidence/specs GIVN_SPEC_EMBEDDER=stub GIVN_SPEC_BGE=disabled GIVN_SPEC_RERANKER=nli-stub cargo test --features spec-retrieval,nli-evidence --test features_runner -- --name "NLI reduces permanent exploration candidates without changing E5 raw results"
  1 feature, 1 scenario, 1 failed step; process exited 101 because the first
  step was undefined. The runner also panicked after reporting the failed step.
  ```
  Commit: `6c5fbf0` — `chore(nli-evidence): scaffold bounded NLI change`.

## Non-E2E Scenarios

### NLI reports a symmetric probability summary from both directions

- [x] RED: removed only this scenario's `@wip`; targeted NLI runner exited 101
  at the unimplemented Given step.
- [x] GREEN: added NLI admission output and concrete symmetric-probability
  assertions in `src/spec/retrieval.rs` and `tests/steps/nli_evidence_steps.rs`.
  Targeted run passed 6/6 steps.
- [x] REFACTOR: reran the unchanged targeted command; 1 scenario and 6 steps
  passed.
- [x] COMMIT: `a0cbb6a` — `feat(nli-evidence): NLI reports a symmetric probability summary from both directions`.

### BGE and NLI jointly reduce the recommended candidate set

- [x] RED: removed only this scenario's `@wip`; targeted combined runner exited
  101 at the unimplemented Given step.
- [x] GREEN: implemented independent BGE/NLI advisory dispatch, admission
  intersection, raw candidate preservation, and non-classifying output in
  `src/spec/retrieval.rs` and `tests/steps/nli_evidence_steps.rs`. Targeted run
  passed 5/5 steps.
- [x] REFACTOR: reran the unchanged combined targeted command; 1 scenario and
  5 steps passed.
- [x] COMMIT: `e42afc6` — `feat(nli-evidence): BGE and NLI jointly reduce the recommended candidate set`.

### An unavailable NLI layer keeps a combined candidate blocking

- [x] RED: targeted combined blocking runner exited 101 at the unimplemented
  Given step.
- [x] GREEN: implemented NLI selection/preflight, semantic-review NLI run
  accounting, unavailable fallback, null probability fields, and blocking
  semantics in `src/spec/retrieval.rs` and tests. Targeted run passed 5/5
  steps.
- [x] REFACTOR: reran the unchanged targeted command; 1 scenario and 5 steps
  passed.
- [x] COMMIT: `6d4dd95` — `feat(nli-evidence): keep combined candidates blocking when NLI is unavailable`.

### NLI refuses an overflowing bounded pair pool before model loading

- [x] RED: targeted NLI duplicate runner was initially blocked by the
  unimplemented NLI step skeleton.
- [x] GREEN: implemented the NLI duplicate-audit run, synthetic pool seam,
  `NLI_UNAVAILABLE` pair-cap output, zero-load accounting, and E5 unscored
  fallback in `src/spec/retrieval.rs` and tests. Targeted run passed 6/6 steps.
- [x] REFACTOR: reran the unchanged targeted command; 1 scenario and 6 steps
  passed.
- [x] COMMIT: `3f7bf6e` — `feat(nli-evidence): refuse overflowing NLI candidate pools`.

### NLI keeps candidates outside a partial bounded pool blocking

- [ ] RED: targeted runner fails on the unimplemented step. Evidence: pending.
- [ ] GREEN: implement partial-pool `NLI_UNSCORED` JSON/report evidence.
  Production files: `src/spec/retrieval.rs` and tests. Evidence: pending.
- [ ] REFACTOR: targeted runner passes unchanged. Evidence: pending.
- [ ] COMMIT: pending.

### NLI JSON preserves the raw E5 contract and uses the NLI schema extension

- [ ] RED: targeted runner fails on the unimplemented step. Evidence: pending.
- [ ] GREEN: implement additive Explore JSON NLI fields and schema version 3.
  Production files: `src/spec/retrieval.rs` and tests. Evidence: pending.
- [ ] REFACTOR: targeted runner passes unchanged. Evidence: pending.
- [ ] COMMIT: pending.

### Disabling NLI preserves the E5-only contract in an NLI-capable build

- [ ] RED: targeted runner fails on the unimplemented step. Evidence: pending.
- [ ] GREEN: implement explicit disabled dispatch and E5-only output
  preservation. Production files: `src/spec/retrieval.rs` and tests. Evidence:
  pending.
- [ ] REFACTOR: targeted runner passes unchanged. Evidence: pending.
- [ ] COMMIT: pending.

### Missing NLI artifacts preserve the explicit E5 fallback

- [ ] RED: targeted runner fails on the unimplemented step. Evidence: pending.
- [ ] GREEN: implement NLI artifact preflight and `NLI_UNAVAILABLE` output.
  Production files: `src/spec/retrieval.rs` and tests. Evidence: pending.
- [ ] REFACTOR: targeted runner passes unchanged. Evidence: pending.
- [ ] COMMIT: pending.

### The NLI admission boundary includes a symmetric entailment probability of 0.5000

- [ ] RED: targeted runner fails on the unimplemented step. Evidence: pending.
- [ ] GREEN: implement inclusive NLI admission comparison. Production files:
  `src/spec/retrieval.rs` and tests. Evidence: pending.
- [ ] REFACTOR: targeted runner passes unchanged. Evidence: pending.
- [ ] COMMIT: pending.

### An NLI-recommended candidate still blocks unresolved semantic review

- [ ] RED: targeted runner fails on the unimplemented step. Evidence: pending.
- [ ] GREEN: implement NLI-recommended semantic report candidate blocks and
  unresolved exit status. Production files: `src/spec/retrieval.rs` and tests.
  Evidence: pending.
- [ ] REFACTOR: targeted runner passes unchanged. Evidence: pending.
- [ ] COMMIT: pending.

### The independent NLI capability owns the legacy diagnostic selector

- [ ] RED: targeted runner fails on the unimplemented modified scenario.
  Evidence: pending.
- [ ] GREEN: update the legacy scenario binding/output to the NLI admission
  path while retaining diagnostic evidence. Production files:
  `src/spec/retrieval.rs` and tests. Evidence: pending.
- [ ] REFACTOR: targeted runner passes unchanged. Evidence: pending.
- [ ] COMMIT: pending.

## E2E Scenarios

### NLI reduces permanent exploration candidates without changing E5 raw results

- [ ] RED: remove only this scenario's `@wip`; `./run-tests.sh --e2e` targeted
  run fails on the unimplemented E2E step. Evidence: pending.
- [ ] GREEN: implement the real compiled CLI subprocess assertions in
  `tests/steps/nli_evidence_e2e_steps.rs`. Evidence: pending.
- [ ] REFACTOR: targeted E2E command passes unchanged. Evidence: pending.
- [ ] COMMIT: pending.

### NLI reduces change-aware exploration candidates without changing E5 raw results

- [ ] RED: targeted E2E run fails on the unimplemented step. Evidence: pending.
- [ ] GREEN: implement the real `--change` CLI interaction and source/identity
  assertions. Evidence: pending.
- [ ] REFACTOR: targeted E2E command passes unchanged. Evidence: pending.
- [ ] COMMIT: pending.

### NLI reduces advisory review candidates without assigning a Givn class

- [ ] RED: targeted E2E run fails on the unimplemented step. Evidence: pending.
- [ ] GREEN: implement the real advisory review CLI assertions. Evidence:
  pending.
- [ ] REFACTOR: targeted E2E command passes unchanged. Evidence: pending.
- [ ] COMMIT: pending.

### NLI filtered evidence does not block the blocking review

- [ ] RED: targeted E2E run fails on the unimplemented step. Evidence: pending.
- [ ] GREEN: implement the real blocking review CLI/report assertions.
  Evidence: pending.
- [ ] REFACTOR: targeted E2E command passes unchanged. Evidence: pending.
- [ ] COMMIT: pending.

### NLI uses the bounded E5 neighbour union for the permanent duplicate audit

- [ ] RED: targeted E2E run fails on the unimplemented step. Evidence: pending.
- [ ] GREEN: implement the real duplicate-audit CLI assertions. Evidence:
  pending.
- [ ] REFACTOR: targeted E2E command passes unchanged. Evidence: pending.
- [ ] COMMIT: pending.

## Full Verification

- [ ] `givn lint --change nli-evidence` exits 0.
- [ ] `./run-tests.sh` exits 0 across E5-only, BGE-only, NLI-only, and combined
  matrices.
- [ ] `./run-tests.sh --e2e` exits 0 and runs fewer scenarios than the full
  command because it filters to `@e2e`.
- [ ] `cargo fmt --check`, `cargo check`, and
  `cargo clippy --workspace --all-targets -- -D warnings` pass.
