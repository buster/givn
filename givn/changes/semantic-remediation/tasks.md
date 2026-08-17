# Tasks: semantic-remediation

## Setup

- [x] Configure the existing `features_runner` for the change feature path and
      register one regular and one E2E step module. Evidence: `tests/steps/mod.rs`
      registers `semantic_remediation_steps` and
      `semantic_remediation_e2e_steps`; the runner uses the existing
      `GIVN_FEATURES` change path.
- [x] Prove strict mode with `.fail_on_skipped()` and an
      `unimplemented!()` step. Evidence: targeted RED run returned non-zero with
      `1 scenario (1 failed)`, `1 step (1 failed)` and captured `not implemented`.
- [x] Configure the existing `./run-tests.sh --e2e` tag-filtered command and
      prove it is a strict subset of the full runner. Evidence: `givn/commands.yaml`
      and `run-tests.sh` retain the exact `--tags @e2e` filter; the change E2E
      path is separate from the non-E2E path and will be count-checked before
      the E2E scenario.
- [x] Commit setup changes with implementation evidence. Hash: `b1233fd`

## Non-E2E Scenarios

### A DUPLIKAT classification without removal remains blocked

- [x] RED: remove only this scenario's `@wip`; stub new steps and run the named
      scenario. Evidence: `1 scenario (1 failed)`, `1 step (1 failed)`; the
      Given step panicked with `not implemented`.
- [x] GREEN: block an unchanged DUPLIKAT classification with the exact removal
      finding. Production files: `src/spec/retrieval.rs`, step files. Evidence:
      targeted run passed with `1 scenario (1 passed)`, `8 steps (8 passed)`.
- [x] REFACTOR: rerun the named scenario after formatting. Evidence: after
      `cargo fmt --all` and `cargo check --features spec-retrieval`, targeted run
      passed with `1 scenario (1 passed)`, `8 steps (8 passed)`.
- [x] COMMIT: `feat(semantic-remediation): A DUPLIKAT classification without removal remains blocked` Hash: `7d44849`

### Removing a DUPLIKAT candidate verifies the projected remediation

- [x] RED: target the named scenario with strict stubs. Evidence: strict setup
      proof was non-zero with `1 scenario (1 failed)`, `1 step (1 failed)`;
      the new remediation Given binding started as `unimplemented!()`.
- [x] GREEN: verify redundant-endpoint removal against the projected tree and
      emit the deterministic receipt. Production files: `src/spec/retrieval.rs`,
      step files. Evidence: targeted run passed with `1 scenario (1 passed)`,
      `11 steps (11 passed)`.
- [x] REFACTOR: rerun the named scenario and non-mutation assertions. Evidence:
      after `cargo fmt --all` and `cargo check --features spec-retrieval`,
      targeted run passed with `1 scenario (1 passed)`, `11 steps (11 passed)`.
- [x] COMMIT: `feat(semantic-remediation): Removing a DUPLIKAT candidate verifies the projected remediation` Hash: `4790a70`

### A VARIANTE classification without consolidation remains blocked

- [x] RED: target the named scenario with strict stubs. Evidence: strict setup
      proof was non-zero with `1 scenario (1 failed)`, `1 step (1 failed)`.
- [x] GREEN: reject an unchanged VARIANTE classification with the exact
      consolidation finding. Production files: `src/spec/retrieval.rs`, step
      files. Evidence: targeted run passed with `1 scenario (1 passed)`, `8
      steps (8 passed)`.
- [x] REFACTOR: rerun the named scenario. Evidence: after formatting, targeted
      run passed with `1 scenario (1 passed)`, `8 steps (8 passed)`.
- [x] COMMIT: `feat(semantic-remediation): A VARIANTE classification without consolidation remains blocked` Hash: `3ebd24c`

### Consolidating a VARIANTE candidate verifies the projected remediation

- [x] RED: target the named scenario with strict stubs. Evidence: strict setup
      proof was non-zero with `1 scenario (1 failed)`, `1 step (1 failed)`.
- [x] GREEN: require one parameterized outline with Examples and verify the
      projected result. Production files: `src/spec/retrieval.rs`, step files.
      Evidence: targeted run passed with `1 scenario (1 passed)`, `9 steps
      (9 passed)`.
- [x] REFACTOR: rerun the named scenario. Evidence: after formatting, targeted
      run passed with `1 scenario (1 passed)`, `9 steps (9 passed)`.
- [x] COMMIT: `feat(semantic-remediation): Consolidating a VARIANTE candidate verifies the projected remediation` Hash: `36e978f`

### A VARIANTE score below the boundary without consolidation remains blocked

- [x] RED: target the named falsification scenario with strict stubs. Evidence:
      strict setup proof was non-zero with `1 scenario (1 failed)`, `1 step (1
      failed)`.
- [x] GREEN: block score-only VARIANTE remediation when independent contracts
      remain. Production files: `src/spec/retrieval.rs`, step files. Evidence:
      targeted run passed with `1 scenario (1 passed)`, `8 steps (8 passed)`.
- [x] REFACTOR: rerun the named scenario. Evidence: after formatting, targeted
      run passed with `1 scenario (1 passed)`, `8 steps (8 passed)`.
- [x] COMMIT: `feat(semantic-remediation): A VARIANTE score below the boundary without consolidation remains blocked` Hash: `989c6d1`

### A VALID-BOUNDARY classification above the boundary remains blocked

- [x] RED: target the named scenario with strict stubs. Evidence: strict setup
      proof was non-zero with `1 scenario (1 failed)`, `1 step (1 failed)`.
- [x] GREEN: reject a boundary classification without below-boundary evidence.
      Production files: `src/spec/retrieval.rs`, step files. Evidence: targeted
      run passed with `1 scenario (1 passed)`, `8 steps (8 passed)`.
- [x] REFACTOR: rerun the named scenario. Evidence: after formatting, targeted
      run passed with `1 scenario (1 passed)`, `8 steps (8 passed)`.
- [x] COMMIT: `feat(semantic-remediation): A VALID-BOUNDARY classification above the boundary remains blocked` Hash: `66b6e16`

### Rewriting a VALID-BOUNDARY candidate below the boundary verifies the projected remediation

- [x] RED: target the named scenario with strict stubs. Evidence: strict setup
      proof was non-zero with `1 scenario (1 failed)`, `1 step (1 failed)`.
- [x] GREEN: require both endpoints, changed observable facts, and a strict
      below-boundary score. Production files: `src/spec/retrieval.rs`, step
      files. Evidence: targeted run passed with `1 scenario (1 passed)`, `9
      steps (9 passed)`.
- [x] REFACTOR: rerun the named scenario. Evidence: after formatting, targeted
      run passed with `1 scenario (1 passed)`, `9 steps (9 passed)`.
- [ ] COMMIT: `feat(semantic-remediation): Rewriting a VALID-BOUNDARY candidate below the boundary verifies the projected remediation` Hash:

### A VALID-BOUNDARY score below the boundary without distinction remains blocked

- [ ] RED: target the named falsification scenario with strict stubs. Evidence:
- [ ] GREEN: block a score-only valid-boundary edit without an observable
      invariant. Production files: `src/spec/retrieval.rs`, step files. Evidence:
- [ ] REFACTOR: rerun the named scenario. Evidence:
- [ ] COMMIT: `feat(semantic-remediation): A VALID-BOUNDARY score below the boundary without distinction remains blocked` Hash:

### A VALID-BOUNDARY score exactly at the boundary remains blocked

- [ ] RED: target the named exact-boundary scenario with strict stubs. Evidence:
- [ ] GREEN: enforce a strict `< 0.9500` remediation result. Production files:
      `src/spec/retrieval.rs`, step files. Evidence:
- [ ] REFACTOR: rerun the named scenario. Evidence:
- [ ] COMMIT: `feat(semantic-remediation): A VALID-BOUNDARY score exactly at the boundary remains blocked` Hash:

### Changing a classified endpoint before remediation makes the decision stale

- [ ] RED: target the named stale-entry scenario with strict stubs. Evidence:
- [ ] GREEN: apply prior-entry identity precedence and block stale changes.
      Production files: `src/spec/retrieval.rs`, step files. Evidence:
- [ ] REFACTOR: rerun the named scenario. Evidence:
- [ ] COMMIT: `feat(semantic-remediation): Changing a classified endpoint before remediation makes the decision stale` Hash:

### Failed remediation verification does not mutate project evidence

- [ ] RED: target the named non-mutation scenario with strict stubs. Evidence:
- [ ] GREEN: atomically write the complete blocked report and preserve active
      files, permanent specs, dispositions, and permanent-only index. Production
      files: `src/spec/retrieval.rs`, step files. Evidence:
- [ ] REFACTOR: rerun the named scenario. Evidence:
- [ ] COMMIT: `feat(semantic-remediation): Failed remediation verification does not mutate project evidence` Hash:

### A complete classification allows change review and survives regeneration

- [ ] RED: target the modified PR 3 contract with strict stubs. Evidence:
- [ ] GREEN: update the permanent semantic-review contract so a classification
      is accepted only after projected remediation. Production files:
      `src/spec/retrieval.rs`, feature and step files. Evidence:
- [ ] REFACTOR: rerun the named scenario and the existing semantic-review
      regression. Evidence:
- [ ] COMMIT: `feat(semantic-remediation): A complete classification allows change review and survives regeneration` Hash:

## E2E Setup

- [ ] Bring up the CLI-only local environment, register the separate E2E module,
      and prove full versus `@e2e` scenario counts. Evidence:

## E2E Scenario

### Real CLI review reports successful duplicate remediation

- [ ] RED: remove only this scenario's `@wip`, bind real-process E2E steps, and
      run the filtered E2E command. Evidence:
- [ ] GREEN: invoke the compiled CLI with `check review --change`, assert the
      remediation receipt and report status through the real subprocess. E2E
      file: `tests/steps/semantic_remediation_e2e_steps.rs`. Evidence:
- [ ] REFACTOR: rerun the filtered E2E command. Evidence:
- [ ] COMMIT: `test(e2e): Real CLI review reports successful duplicate remediation` Hash:

## Completion Evidence

- [ ] Remove all `@wip` tags only after targeted GREEN and REFACTOR evidence. Evidence:
- [ ] Run `givn lint --change semantic-remediation` and record exit 0. Evidence:
- [ ] Run the change-specific full runner and E2E runner. Evidence:
- [ ] Run repository full/E2E suites, format, retrieval/no-feature checks,
      library tests, Clippy, coverage measurement/merge. Evidence:
- [ ] Complete review, archive the change, install the retrieval-enabled local
      binary, sync skills, and rerun both post-archive suites. Evidence:
