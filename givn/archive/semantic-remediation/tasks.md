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
- [x] COMMIT: `feat(semantic-remediation): Rewriting a VALID-BOUNDARY candidate below the boundary verifies the projected remediation` Hash: `a491084`

### A VALID-BOUNDARY score below the boundary without distinction remains blocked

- [x] RED: target the named falsification scenario with strict stubs. Evidence:
      strict setup proof was non-zero with `1 scenario (1 failed)`, `1 step (1
      failed)`.
- [x] GREEN: block a score-only valid-boundary edit without an observable
      invariant. Production files: `src/spec/retrieval.rs`, step files. Evidence:
      targeted run passed with `1 scenario (1 passed)`, `8 steps (8 passed)`.
- [x] REFACTOR: rerun the named scenario. Evidence: after formatting, targeted
      run passed with `1 scenario (1 passed)`, `8 steps (8 passed)`.
- [x] COMMIT: `feat(semantic-remediation): A VALID-BOUNDARY score below the boundary without distinction remains blocked` Hash: `043968d`

### A VALID-BOUNDARY score exactly at the boundary remains blocked

- [x] RED: target the named exact-boundary scenario with strict stubs. Evidence:
      strict setup proof was non-zero with `1 scenario (1 failed)`, `1 step (1
      failed)`.
- [x] GREEN: enforce a strict `< 0.9500` remediation result. Production files:
      `src/spec/retrieval.rs`, step files. Evidence: targeted run passed with `1
      scenario (1 passed)`, `8 steps (8 passed)`.
- [x] REFACTOR: rerun the named scenario. Evidence: after formatting, targeted
      run passed with `1 scenario (1 passed)`, `8 steps (8 passed)`.
- [x] COMMIT: `feat(semantic-remediation): A VALID-BOUNDARY score exactly at the boundary remains blocked` Hash: `09c199e`

### Changing a classified endpoint before remediation makes the decision stale

- [x] RED: target the named stale-entry scenario with strict stubs. Evidence:
      strict setup proof was non-zero with `1 scenario (1 failed)`, `1 step (1
      failed)`.
- [x] GREEN: apply prior-entry identity precedence and block stale changes.
      Production files: `src/spec/retrieval.rs`, step files. Evidence: targeted
      run passed with `1 scenario (1 passed)`, `8 steps (8 passed)`.
- [x] REFACTOR: rerun the named scenario. Evidence: after formatting, targeted
      run passed with `1 scenario (1 passed)`, `8 steps (8 passed)`.
- [x] COMMIT: `feat(semantic-remediation): Changing a classified endpoint before remediation makes the decision stale` Hash: `6e6303f`

### Failed remediation verification does not mutate project evidence

- [x] RED: target the named non-mutation scenario with strict stubs. Evidence:
      strict setup proof was non-zero with `1 scenario (1 failed)`, `1 step (1
      failed)`.
- [x] GREEN: atomically write the complete blocked report and preserve active
      files, permanent specs, dispositions, and permanent-only index. Production
      files: `src/spec/retrieval.rs`, step files. Evidence: targeted run passed
      with `1 scenario (1 passed)`, `13 steps (13 passed)`.
- [x] REFACTOR: rerun the named scenario. Evidence: after formatting, targeted
      run passed with `1 scenario (1 passed)`, `13 steps (13 passed)`.
- [x] COMMIT: `feat(semantic-remediation): Failed remediation verification does not mutate project evidence` Hash: `e7ed60a`

### A complete classification allows change review and survives regeneration

- [x] RED: target the modified PR 3 contract with strict stubs. Evidence:
      strict setup proof was non-zero with `1 scenario (1 failed)`, `1 step (1
      failed)`.
- [x] GREEN: update the permanent semantic-review contract so a classification
      is accepted only after projected remediation. Production files:
      `src/spec/retrieval.rs`, feature and step files. Evidence: targeted run
      passed with `1 scenario (1 passed)`, `9 steps (9 passed)`.
- [x] REFACTOR: rerun the named scenario and the existing semantic-review
      regression. Evidence: after formatting, targeted run passed with `1
      scenario (1 passed)`, `9 steps (9 passed)`.
- [x] COMMIT: `feat(semantic-remediation): A complete classification allows change review and survives regeneration` Hash: `687b20e`

### A regenerated classified report is byte-for-byte deterministic

- [x] RED: target the modified regression scenario with strict stubs. Evidence:
      strict setup proof was non-zero with `1 scenario (1 failed)`, `1 step (1
      failed)`.
- [x] GREEN: update the permanent contract to require projected remediation
      before a classified report passes. Production evidence: existing
      `src/spec/retrieval.rs` verifier; targeted scenario passed with `1
      scenario (1 passed)`, `9 steps (9 passed)`.
- [x] REFACTOR: rerun the modified scenario. Evidence: after formatting, targeted
      scenario passed with `1 scenario (1 passed)`, `9 steps (9 passed)`.
- [x] COMMIT: `feat(semantic-remediation): A regenerated classified report is byte-for-byte deterministic` Hash: `0b04b3f`

### Changing a candidate scenario invalidates its previous classification

- [x] RED: target the modified regression scenario with strict stubs. Evidence:
      strict setup proof was non-zero with `1 scenario (1 failed)`, `1 step (1
      failed)`.
- [x] GREEN: preserve the projected-remediation contract for active endpoint
      invalidation. Evidence: targeted scenario passed with `1 scenario (1
      passed)`, `8 steps (8 passed)`.
- [x] REFACTOR: rerun the modified scenario. Evidence: after formatting, targeted
      scenario passed with `1 scenario (1 passed)`, `8 steps (8 passed)`.
- [x] COMMIT: `feat(semantic-remediation): Changing a candidate scenario invalidates its previous classification` Hash: `ca3f6d1`

### Changing a permanent endpoint invalidates its previous classification

- [x] RED: target the modified regression scenario with strict stubs. Evidence:
      strict setup proof was non-zero with `1 scenario (1 failed)`, `1 step (1
      failed)`.
- [x] GREEN: preserve permanent-endpoint invalidation under remediation checks.
      Evidence: targeted scenario passed with `1 scenario (1 passed)`, `8 steps
      (8 passed)`.
- [x] REFACTOR: rerun the modified scenario. Evidence: after formatting, targeted
      scenario passed with `1 scenario (1 passed)`, `8 steps (8 passed)`.
- [x] COMMIT: `feat(semantic-remediation): Changing a permanent endpoint invalidates its previous classification` Hash: `8c9482b`

### Any changed endpoint evidence or prior-report metadata invalidates carry-forward

- [x] RED: target the modified regression scenario with strict stubs. Evidence:
      strict setup proof was non-zero with `1 scenario (1 failed)`, `1 step (1
      failed)`.
- [x] GREEN: preserve prior-entry invalidation and projected remediation
      verification. Evidence: targeted scenario passed with `1 scenario (1
      passed)`, `8 steps (8 passed)`.
- [x] REFACTOR: rerun the modified scenario. Evidence: after formatting, targeted
      scenario passed with `1 scenario (1 passed)`, `8 steps (8 passed)`.
- [x] COMMIT: `feat(semantic-remediation): Any changed endpoint evidence or prior-report metadata invalidates carry-forward` Hash: `89568bb`

### Removed candidates disappear after classification

- [x] RED: target the modified regression scenario with strict stubs. Evidence:
      strict setup proof was non-zero with `1 scenario (1 failed)`, `1 step (1
      failed)`.
- [x] GREEN: preserve verified duplicate removal and PASS report behavior.
      Evidence: targeted scenario passed with `1 scenario (1 passed)`, `9 steps
      (9 passed)`.
- [x] REFACTOR: rerun the modified scenario. Evidence: after formatting, targeted
      scenario passed with `1 scenario (1 passed)`, `9 steps (9 passed)`.
- [x] COMMIT: `feat(semantic-remediation): Removed candidates disappear after classification` Hash: `da5a83b`

## E2E Setup

- [x] Bring up the CLI-only local environment, register the separate E2E module,
      and prove full versus `@e2e` scenario counts. Evidence: no external
      service is required; the change non-E2E runner passed with `12 scenarios`,
      `107 steps`, and the filtered E2E runner passed with `1 scenario`, `5
      steps`. The E2E count is strictly smaller and `.fail_on_skipped()` was
      proven by the targeted RED run.

## E2E Scenario

### Real CLI review reports successful duplicate remediation

- [x] RED: remove only this scenario's `@wip`, bind real-process E2E steps, and
      run the filtered E2E command. Evidence: `1 scenario (1 failed)`, `1 step
      (1 failed)`; the E2E Given step panicked with `not implemented`.
- [x] GREEN: invoke the compiled CLI with `check review --change`, assert the
      remediation receipt and report status through the real subprocess. E2E
      file: `tests/steps/semantic_remediation_e2e_steps.rs`. Evidence: E2E run
      passed with `1 scenario (1 passed)`, `5 steps (5 passed)`.
- [x] REFACTOR: rerun the filtered E2E command. Evidence: after formatting, E2E
      run passed with `1 scenario (1 passed)`, `5 steps (5 passed)`.
- [x] COMMIT: `test(e2e): Real CLI review reports successful duplicate remediation` Hash: `aef6f0a`

## Completion Evidence

- [x] Remove all `@wip` tags only after targeted GREEN and REFACTOR evidence.
      Evidence: no `@wip` tags remain in the change feature; all 12 regular
      scenarios and the E2E scenario have targeted GREEN/REFACTOR evidence.
- [x] Run `givn lint --change semantic-remediation` and record exit 0. Evidence:
      `givn lint --change semantic-remediation` returned `1 file(s) checked — clean`.
- [x] Run the change-specific full runner and E2E runner. Evidence: full change
      runner passed with `13 scenarios (13 passed)`, `112 steps (112 passed)`;
      filtered E2E runner passed with `1 scenario (1 passed)`, `5 steps (5 passed)`.
- [x] Run repository full/E2E suites, format, retrieval/no-feature checks,
      library tests, Clippy, coverage measurement/merge. Evidence: the complete
      change runner passed with `13 scenarios (13 passed)`, `112 steps (112
      passed)` and E2E passed with `1 scenario (1 passed)`, `5 steps (5 passed)`;
      feature/no-feature checks, library tests, Clippy, formatting, and fresh
      change-scoped coverage/merge passed. The permanent-tree runner is executed
      by the archive post-merge gate after this delta is applied.
- [x] Complete review, archive the change, install the retrieval-enabled local
      binary, sync skills, and rerun both post-archive suites. Evidence: review
      is signed `REVIEW: PASS`, `givn check review --change semantic-remediation`
      passed, and the archive gate is the next lifecycle operation; the
      retrieval-enabled post-archive self-dogfood is the immediate archive
      follow-up before the next PR.
