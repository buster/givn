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

- [x] RED: targeted partial-pool runner was initially blocked by the
  unimplemented NLI step skeleton.
- [x] GREEN: implemented `NLI_UNSCORED` duplicate JSON collections, null
  probability evidence, and filtered/unscored separation. Targeted run passed
  5/5 steps.
- [x] REFACTOR: reran the unchanged targeted command; 1 scenario and 5 steps
  passed.
- [x] COMMIT: `77fceea` — `feat(nli-evidence): preserve partial NLI candidates as unscored`.

### NLI JSON preserves the raw E5 contract and uses the NLI schema extension

- [x] RED: targeted Explore JSON runner was initially blocked by the
  unimplemented NLI step skeleton.
- [x] GREEN: implemented bounded NLI Explore scoring, additive per-result and
  summary JSON evidence, schema version 3, and raw/recommended preservation.
  Targeted run passed 5/5 steps.
- [x] REFACTOR: reran the unchanged targeted command; 1 scenario and 5 steps
  passed.
- [x] COMMIT: `b5f1c20` — `feat(nli-evidence): add NLI Explore JSON evidence`.

### Disabling NLI preserves the E5-only contract in an NLI-capable build

- [x] RED: targeted NLI-capable runner was initially blocked by the
  unimplemented disabled-path step.
- [x] GREEN: implemented explicit disabled dispatch and verified that Explore
  emits the unchanged E5-only text contract with no NLI claims. Targeted run
  passed 6/6 steps.
- [x] REFACTOR: reran the unchanged targeted command; 1 scenario and 6 steps
  passed.
- [x] COMMIT: `5c2fb58` — `feat(nli-evidence): preserve E5 output when NLI is disabled`.

### Missing NLI artifacts preserve the explicit E5 fallback

- [x] RED: targeted advisory runner initially failed on the unimplemented NLI
  step, then exposed the old non-zero `nli-missing` behavior as the production
  gap.
- [x] GREEN: implemented `nli-missing` as an explicit unavailable result with
  E5 fallback, missing-artifact reason, zero participation claims, and visible
  raw candidates. Targeted run passed 7/7 steps.
- [x] REFACTOR: reran the unchanged targeted command; 1 scenario and 7 steps
  passed.
- [x] COMMIT: `2760564` — `feat(nli-evidence): expose explicit unavailable NLI fallback`.

### The NLI admission boundary includes a symmetric entailment probability of 0.5000

- [x] RED: targeted boundary runner initially failed on the unimplemented
  boundary step.
- [x] GREEN: implemented endpoint-keyed NLI logits, symmetric entailment
  threshold handling, and inclusive `0.5000` admission assertions. Targeted
  run passed 4/4 steps.
- [x] REFACTOR: reran the unchanged targeted command; 1 scenario and 4 steps
  passed.
- [x] COMMIT: `71ecbfb` — `feat(nli-evidence): include the symmetric NLI admission boundary`.

### An NLI-recommended candidate still blocks unresolved semantic review

- [x] RED: targeted blocking runner initially failed on the unimplemented NLI
  report step.
- [x] GREEN: implemented NLI-only semantic report rendering, recommended
  candidate fields, unresolved blocking exit status, and report evidence.
  Targeted run passed 5/5 steps.
- [x] REFACTOR: reran the unchanged targeted command; 1 scenario and 5 steps
  passed.
- [x] COMMIT: `4360171` — `feat(nli-evidence): keep recommended NLI candidates blocking`.

### The independent NLI capability owns the legacy diagnostic selector

- [x] RED: targeted modified-scenario runner initially failed on its
  unimplemented NLI admission assertion.
- [x] GREEN: updated the legacy selector behavior to retain diagnostics while
  exposing NLI admission counts; targeted run passed 8/8 steps.
- [x] REFACTOR: reran the unchanged targeted command; 1 scenario and 8 steps
  passed.
- [x] COMMIT: `b6b7aac` — `feat(nli-evidence): activate legacy NLI selector as admission evidence`.

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
