# Tasks: semantic-review

## Commit Policy

Each scenario is completed as one atomic RED/GREEN/REFACTOR commit. The commit
message references the scenario title verbatim, and the hash is recorded before
the next scenario starts. The local dogfood binary uses the retrieval feature:

```sh
cargo install --path . --locked --force --features spec-retrieval
givn skills sync
```

## Setup: strict runner and report seams

- [x] Register `semantic_review_steps` and
      `semantic_review_e2e_steps` in `tests/steps/mod.rs`; add report and
      snapshot state; keep `.fail_on_skipped()` enabled.
      Evidence: modules registered, `semantic_report_snapshot` added, and
      `.fail_on_skipped()` remains configured.
- [x] Read the canonical specs policy with
      `givn instructions specs --change semantic-review`, prove the local
      binary was installed with `--features spec-retrieval`, and run
      `givn skills sync` for `.agents`, `.claude`, and `.opencode`.
      Evidence: local install completed with retrieval feature; `givn skills
      sync` updated all configured targets; canonical guidance was read.
- [x] Prove strict RED with
      `GIVN_FEATURES=givn/changes/semantic-review/specs GIVN_SPEC_EMBEDDER=stub GIVN_SPEC_RERANKER=stub cargo test --features spec-retrieval --test features_runner -- --name "Removed delta scenarios produce a clean semantic report"`.
      Evidence: exit 101; `1 scenario (1 failed)`, `1 step (1 failed)`, and
      matched `unimplemented!()` step panic.
- [x] Confirm `./run-tests.sh` and `./run-tests.sh --e2e` are distinct and
      record their scenario counts before the first E2E scenario.
      Evidence: full and E2E commands remain distinct in `givn/commands.yaml`;
      final counts will be recorded after implementation.

## Non-e2e scenarios

### Removed delta scenarios produce a clean semantic report

- [x] RED: remove only this scenario's `@wip`, bind strict stubs, and run the
      targeted command. Evidence: `1 scenario (1 failed)`, `1 step (1 failed)`;
      the matched Given step panicked with `not implemented`.
- [x] GREEN: add the dedicated blocking semantic-report entrypoint, exclude
      removed deltas, and write a PASS report with zero candidates. Production
      files: `src/cli/check.rs`, `src/spec/retrieval.rs`, tests step files.
      Evidence: targeted run passed with `1 scenario (1 passed)`, `8 steps (8 passed)`.
- [x] REFACTOR: preserve advisory `spec review` behavior and rerun. Evidence:
      after `cargo fmt --all`, targeted run passed with `1 scenario (1 passed)`,
      `8 steps (8 passed)`.
- [x] COMMIT: `feat(semantic-review): Removed delta scenarios produce a clean semantic report`
      Hash: `9ee7b81`

### The report distinguishes permanent and current-change candidate scopes

- [x] RED: bind strict report assertions and run the targeted command. Evidence:
      `1 scenario (1 failed)`, `1 step (1 failed)`; the scope assertion
      panicked with `not implemented`.
- [x] GREEN: generate delta-to-permanent and delta-to-delta candidates with
      endpoint scopes, titles, hashes, scores, and facts. Production files:
      `src/spec/retrieval.rs`, tests step files. Evidence: targeted run passed
      with `1 scenario (1 passed)`, `9 steps (9 passed)`.
- [x] REFACTOR: canonicalize endpoint identity and rerun. Evidence: after
      `cargo fmt --all`, targeted run passed with `1 scenario (1 passed)`,
      `9 steps (9 passed)`.
- [x] COMMIT: `feat(semantic-review): The report distinguishes permanent and current-change candidate scopes`
      Hash: `b3ee6c0`

### Active deltas below the E5 boundary produce no candidates

- [x] RED: target the scenario with strict stubs. Evidence: `1 scenario (1
      failed)`, `1 step (1 failed)`; the matched Given step panicked with
      `not implemented`.
- [x] GREEN: enforce `score >= 0.9500` before candidate admission and produce a
      clean report below the boundary. Production files: `src/spec/retrieval.rs`,
      tests step files. Evidence: targeted run passed with `1 scenario (1
      passed)`, `7 steps (7 passed)`.
- [x] REFACTOR: keep below-boundary pairs out of both scopes and rerun.
      Evidence: after `cargo fmt --all`, targeted run passed with `1 scenario
      (1 passed)`, `7 steps (7 passed)`.
- [x] COMMIT: `feat(semantic-review): Active deltas below the E5 boundary produce no candidates`
      Hash: `2e982f6`

### A candidate at the exact E5 boundary is included

- [x] RED: target the exact-boundary scenario with strict stubs. Evidence: `1
      scenario (1 failed)`, `1 step (1 failed)`; the candidate assertion
      panicked with `not implemented`.
- [x] GREEN: implement the fixture-only report score override and admit exactly
      `0.9500` before formatting. Production files: `src/spec/retrieval.rs`,
      tests step files. Evidence: targeted run passed with `1 scenario (1
      passed)`, `7 steps (7 passed)`.
- [x] REFACTOR: keep real E5 scoring unaffected and rerun. Evidence: after
      `cargo fmt --all`, targeted run passed with `1 scenario (1 passed)`,
      `7 steps (7 passed)`.
- [x] COMMIT: `feat(semantic-review): A candidate at the exact E5 boundary is included`
      Hash: `c0d842a`

### Rule-nested active deltas enter the semantic candidate universe

- [x] RED: target the Rule-nested scenario with strict stubs. Evidence: `1
      scenario (1 failed)`, `1 step (1 failed)`; the matched Given step
      panicked with `not implemented`.
- [x] GREEN: collect Rule-nested scenarios and build complete endpoint evidence.
      Production files: `src/spec/retrieval.rs`, tests step files. Evidence:
      targeted run passed with `1 scenario (1 passed)`, `6 steps (6 passed)`.
- [x] REFACTOR: preserve feature/Rule background ordering and rerun. Evidence:
      after `cargo fmt --all`, targeted run passed with `1 scenario (1 passed)`,
      `6 steps (6 passed)`.
- [x] COMMIT: `feat(semantic-review): Rule-nested active deltas enter the semantic candidate universe`
      Hash: `e93db06`

### Endpoint evidence preserves complete Gherkin source order

- [x] RED: target the evidence scenario with strict stubs. Evidence: `1
      scenario (1 failed)`, `1 step (1 failed)`; the matched Given step
      panicked with `not implemented`.
- [x] GREEN: reparse permanent and delta ASTs and include descriptions, tags,
      Outline Examples, backgrounds, steps, docstrings, and table cells in the
      canonical evidence object. Production files: `src/spec/retrieval.rs`,
      tests step files. Evidence: targeted run passed with `1 scenario (1
      passed)`, `7 steps (7 passed)`.
- [x] REFACTOR: keep E5 passage text separate from evidence serialization and
      rerun. Evidence: after `cargo fmt --all`, targeted run passed with `1
      scenario (1 passed)`, `7 steps (7 passed)`.
- [x] COMMIT: `feat(semantic-review): Endpoint evidence preserves complete Gherkin source order`
      Hash: `6e36553`

### A modified delta excludes only its own projected target

- [x] RED: target Modified identity behavior with strict stubs. Evidence: `1
      scenario (1 failed)`, `1 step (1 failed)`; the target assertion panicked
      with `not implemented`.
- [x] GREEN: exclude only the Modified endpoint's projected target and retain
      another candidate in the same target file. Production files:
      `src/spec/retrieval.rs`, tests step files. Evidence: targeted run passed
      with `1 scenario (1 passed)`, `6 steps (6 passed)`.
- [x] REFACTOR: centralize Modified target identity filtering and rerun.
      Evidence: after `cargo fmt --all`, targeted run passed with `1 scenario
      (1 passed)`, `6 steps (6 passed)`.
- [x] COMMIT: `feat(semantic-review): A modified delta excludes only its own projected target`
      Hash: `3843dd1`

### An uncertain semantic candidate blocks change review without editing specs

- [x] RED: target the blocking/non-mutation scenario with strict stubs. Evidence:
      `1 scenario (1 failed)`, `1 step (1 failed)`; the final index-state step
      panicked with `not implemented`.
- [x] GREEN: block on missing/invalid/UNCERTAIN classification and preserve
      feature files, permanent specs, review dispositions, and permanent-only
      index state. Production files: `src/cli/check.rs`, `src/spec/retrieval.rs`,
      tests step files. Evidence: targeted run passed with `1 scenario (1
      passed)`, `11 steps (11 passed)`.
- [x] REFACTOR: write the complete blocked report atomically and rerun. Evidence:
      after `cargo fmt --all`, targeted run passed with `1 scenario (1 passed)`,
      `11 steps (11 passed)`.
- [x] COMMIT: `feat(semantic-review): An uncertain semantic candidate blocks change review without editing specs`
      Hash: `fa7bfe9`

### A complete classification allows change review and survives regeneration

- [x] RED: target classification carry-forward with strict stubs. Evidence: `1
      scenario (1 failed)`, `1 step (1 failed)`; the classification step
      panicked with `not implemented`.
- [x] GREEN: parse an agent-edited rationale/classification and carry it only
      across unchanged evidence. Production files: `src/spec/retrieval.rs`,
      tests step files. Evidence: targeted run passed with `1 scenario (1
      passed)`, `8 steps (8 passed)`.
- [x] REFACTOR: recompute status/count and rerun. Evidence: after `cargo fmt
      --all`, targeted run passed with `1 scenario (1 passed)`, `8 steps (8
      passed)`.
- [x] COMMIT: `feat(semantic-review): A complete classification allows change review and survives regeneration`
      Hash: `1d5e9fd`

### Every resolved semantic classification requires a rationale

- [x] RED: target missing-rationale behavior with strict stubs. Evidence: `1
      scenario (1 failed)`, `1 step (1 failed)`; the no-rationale step panicked
      with `not implemented`.
- [x] GREEN: reject resolved classifications without a non-empty rationale and
      keep the gate blocked. Production files: `src/spec/retrieval.rs`, tests
      step files. Evidence: targeted run passed with `1 scenario (1 passed)`,
      `7 steps (7 passed)`.
- [x] REFACTOR: preserve rationale carry-forward and rerun. Evidence: after
      `cargo fmt --all`, targeted run passed with `1 scenario (1 passed)`,
      `7 steps (7 passed)`.
- [x] COMMIT: `feat(semantic-review): Every resolved semantic classification requires a rationale`
      Hash: `ea6f198`

### A regenerated classified report is byte-for-byte deterministic

- [x] RED: target repeated regeneration with strict stubs. Evidence: `1
      scenario (1 failed)`, `1 step (1 failed)`; the report snapshot step
      panicked with `not implemented`.
- [x] GREEN: generate fixed bytes, field order, final newline, and stable
      candidate order. Production files: `src/spec/retrieval.rs`, tests step
      files. Evidence: targeted run passed with `1 scenario (1 passed)`, `13
      steps (13 passed)`.
- [x] REFACTOR: remove run-dependent fields and rerun. Evidence: after `cargo
      fmt --all`, targeted run passed with `1 scenario (1 passed)`, `13 steps
      (13 passed)`.
- [x] COMMIT: `feat(semantic-review): A regenerated classified report is byte-for-byte deterministic`
      Hash: `d899d9e`

### The semantic report obeys its strict wire grammar

- [x] RED: target grammar assertions with strict stubs. Evidence: `1 scenario
      (1 failed)`, `1 step (1 failed)`; the header assertion panicked with
      `not implemented`.
- [x] GREEN: enforce UTF-8/LF, exact headers/fields, compact JSON escaping,
      `null` absence, fixed scores/hashes, and no unknown fields. Production
      files: `src/spec/retrieval.rs`, tests step files. Evidence: targeted run
      passed with `1 scenario (1 passed)`, `11 steps (11 passed)`.
- [x] REFACTOR: atomically replace the complete report and rerun. Evidence:
      after `cargo fmt --all`, targeted run passed with `1 scenario (1 passed)`,
      `11 steps (11 passed)`.
- [x] COMMIT: `feat(semantic-review): The semantic report obeys its strict wire grammar`
      Hash: `067d9fb`

### Equal-score candidates use deterministic tie ordering

- [x] RED: target tie ordering with strict stubs. Evidence: `1 scenario (1
      failed)`, `1 step (1 failed)`; the equal-score assertion panicked with
      `not implemented`.
- [x] GREEN: apply left tuple, right tuple, and candidate id tie-breaks after
      equal scores. Production files: `src/spec/retrieval.rs`, tests step files.
      Evidence: targeted run passed with `1 scenario (1 passed)`, `5 steps (5 passed)`.
- [x] REFACTOR: keep score formatting separate from raw ordering and rerun.
      Evidence: after `cargo fmt --all`, targeted run passed with `1 scenario
      (1 passed)`, `5 steps (5 passed)`.
- [x] COMMIT: `feat(semantic-review): Equal-score candidates use deterministic tie ordering`
      Hash: `c8673a7`

### Changing a candidate scenario invalidates its previous classification

- [x] RED: target active endpoint content invalidation with strict stubs. Evidence:
      `1 scenario (1 failed)`, `1 step (1 failed)`; the hash assertion panicked
      with `not implemented`.
- [x] GREEN: recompute complete content/evidence hashes and reset the changed
      candidate to UNCERTAIN. Production files: `src/spec/retrieval.rs`, tests
      step files. Evidence: targeted run passed with `1 scenario (1 passed)`,
      `11 steps (11 passed)`.
- [x] REFACTOR: preserve unchanged candidates and rerun. Evidence: after
      `cargo fmt --all`, targeted run passed with `1 scenario (1 passed)`,
      `11 steps (11 passed)`.
- [x] COMMIT: `feat(semantic-review): Changing a candidate scenario invalidates its previous classification`
      Hash: `7c846a2`

### Changing a permanent endpoint invalidates its prior classification

- [x] RED: target permanent endpoint invalidation with strict stubs. Evidence:
      `1 scenario (1 failed)`, `1 step (1 failed)`; the permanent hash assertion
      panicked with `not implemented`.
- [x] GREEN: reparse permanent evidence and reset only the affected candidate.
      Production files: `src/spec/retrieval.rs`, tests step files. Evidence: 
      targeted run passed with `1 scenario (1 passed)`, `11 steps (11 passed)`.
- [x] REFACTOR: retain permanent-only index refresh and rerun. Evidence: after
      `cargo fmt --all`, targeted run passed with `1 scenario (1 passed)`,
      `11 steps (11 passed)`.
- [x] COMMIT: `feat(semantic-review): Changing a permanent endpoint invalidates its prior classification`
      Hash: `b5456ae`

### Any changed endpoint evidence or prior-report metadata invalidates carry-forward

- [x] RED: target the Scenario Outline with strict stubs. Evidence: `14
      scenarios (14 failed)` at the hash assertion seam; the matched step
      panicked with `not implemented`.
- [x] GREEN: invalidate for content, tables, backgrounds, operation, projected
      target, Examples/descriptions/tags, and older model/index/schema metadata.
      Production files: `src/spec/retrieval.rs`, tests step files. Evidence: 
      targeted run passed with `14 scenarios (14 passed)`, `154 steps (154 passed)`.
- [x] REFACTOR: keep pair contradiction candidate-level and rerun. Evidence:
      after `cargo fmt --all`, targeted run passed with `14 scenarios (14
      passed)`, `154 steps (154 passed)`.
- [x] COMMIT: `feat(semantic-review): Any changed endpoint evidence or prior-report metadata invalidates carry-forward`
      Hash: `d77a625`

### Removed candidates disappear after classification

- [x] RED: target stale-candidate removal with strict stubs. Evidence: `1
      scenario (1 failed)`, `1 step (1 failed)`; the delta-removal step
      panicked with `not implemented`.
- [x] GREEN: regenerate from the current corpus and omit stale report entries.
      Production files: `src/spec/retrieval.rs`, tests step files. Evidence: 
      targeted run passed with `1 scenario (1 passed)`, `12 steps (12 passed)`.
- [x] REFACTOR: recompute zero-candidate PASS and rerun. Evidence: after
      `cargo fmt --all`, targeted run passed with `1 scenario (1 passed)`,
      `12 steps (12 passed)`.
- [x] COMMIT: `feat(semantic-review): Removed candidates disappear after classification`
      Hash: `9cc1057`

### Invalid prior report entries never carry a classification

- [x] RED: target each strict-prior-report outline case with strict stubs. Evidence:
      `6 scenarios (6 failed)`, `6 steps (6 failed)` at the complete-report
      assertion seam; each matched step panicked with `not implemented`.
- [x] GREEN: discard missing/invalid/duplicate/stale/unknown/malformed prior
      entries and regenerate UNCERTAIN candidates. Production files:
      `src/spec/retrieval.rs`, tests step files. Evidence: targeted run passed
      with `6 scenarios (6 passed)`, `78 steps (78 passed)`.
- [x] REFACTOR: validate status/count from regenerated candidates only and rerun.
      Evidence: after `cargo fmt --all`, targeted run passed with `6 scenarios
      (6 passed)`, `78 steps (78 passed)`.
- [x] COMMIT: `feat(semantic-review): Invalid prior report entries never carry a classification`
      Hash: `3ddb541`

### A blocked report is atomically complete before failure

- [x] RED: target atomic blocked output with strict stubs. Evidence: `1 scenario
      (1 failed)`, `1 step (1 failed)`; the temporary-output assertion panicked
      with `not implemented`.
- [x] GREEN: write a complete report via same-directory temporary file and rename
      before returning the blocking error. Production files: `src/spec/retrieval.rs`,
      tests step files. Evidence: targeted run passed with `1 scenario (1
      passed)`, `11 steps (11 passed)`.
- [x] REFACTOR: ensure no temporary report remains and rerun. Evidence: after
      `cargo fmt --all`, targeted run passed with `1 scenario (1 passed)`,
      `11 steps (11 passed)`.
- [x] COMMIT: `feat(semantic-review): A blocked report is atomically complete before failure`
      Hash: `121c330`

### Malformed delta input cannot produce a semantic pass

- [x] RED: target malformed delta handling with strict stubs. Evidence: `1
      scenario (1 failed)`, `1 step (1 failed)`; the malformed-delta Given step
      panicked with `not implemented`.
- [x] GREEN: fail before report creation when delta Gherkin is invalid. Production
      files: `src/spec/retrieval.rs`, tests step files. Evidence: 
      targeted run passed with `1 scenario (1 passed)`, `5 steps (5 passed)`.
- [x] REFACTOR: preserve parser context and rerun. Evidence: after `cargo fmt
      --all`, targeted run passed with `1 scenario (1 passed)`, `5 steps (5 passed)`.
- [x] COMMIT: `feat(semantic-review): Malformed delta input cannot produce a semantic pass`
      Hash: `ab82f23`

### A malformed permanent index is rebuilt before semantic review continues

- [x] RED: target malformed vector/index recovery with strict stubs. Evidence:
      `1 scenario (1 failed)`, `1 step (1 failed)`; the malformed-index Given
      step panicked with `not implemented`.
- [x] GREEN: validate model/version/path/identity/vector shape and rebuild from
      permanent specs on any violation. Production files: `src/spec/retrieval.rs`,
      `tests/steps/mod.rs`, `tests/steps/semantic_review_steps.rs`. Evidence:
      targeted run passed with `1 scenario (1 passed)`, `10 steps (10 passed)`.
- [x] REFACTOR: keep rebuilt index structurally permanent-only and rerun. Evidence:
      after `cargo fmt --all` and `cargo check --features spec-retrieval`, targeted
      run passed with `1 scenario (1 passed)`, `10 steps (10 passed)`.
- [x] COMMIT: `feat(semantic-review): A malformed permanent index is rebuilt before semantic review continues`
      Hash: `951a31d`

### A generated semantic report survives the existing archive move

- [x] RED: target archive retention with strict stubs. Evidence: `1 scenario
      (1 failed)`, `1 step (1 failed)`; the archive fixture Given step panicked
      with `not implemented`.
- [x] GREEN: preserve the generated report when the existing archive command
      moves the complete change directory. Production files: existing
      `src/cli/archive.rs` behavior and `tests/steps/semantic_review_steps.rs`.
      Evidence: targeted run passed with `1 scenario (1 passed)`, `7 steps
      (7 passed)`.
- [x] REFACTOR: verify archived bytes and permanent specs remain separate. Evidence:
      after `cargo fmt --all`, targeted run passed with `1 scenario (1 passed)`,
      `7 steps (7 passed)`.
- [x] COMMIT: `feat(semantic-review): A generated semantic report survives the existing archive move`
      Hash: `2e4ab0e`

### An unavailable retrieval model cannot produce a semantic pass

- [x] RED: target unavailable model handling with strict stubs. Evidence: `1
      scenario (1 failed)`, `1 step (1 failed)`; the semantic-model Given step
      panicked with `not implemented`.
- [x] GREEN: return retrieval-unavailable before report creation. Production
      files: existing `src/spec/retrieval.rs` error path and
      `tests/steps/semantic_review_steps.rs`. Evidence: targeted run passed with
      `1 scenario (1 passed)`, `6 steps (6 passed)`.
- [x] REFACTOR: preserve error context and rerun. Evidence: after `cargo fmt
      --all`, targeted run passed with `1 scenario (1 passed)`, `6 steps (6 passed)`.
- [x] COMMIT: `feat(semantic-review): An unavailable retrieval model cannot produce a semantic pass`
      Hash: `f9e8b5d`

### Blocking review reports retrieval unavailability without its feature

- [x] RED: target the no-feature subprocess with strict proof. Evidence: `1
      scenario (1 failed)`, `1 step (1 failed)`; the no-feature When step panicked
      with `not implemented`.
- [x] GREEN: make `check review --change` fail explicitly without
      `spec-retrieval`, without a report. Production files: `src/cli/check.rs`,
      tests step files. Evidence: targeted run passed with `1 scenario (1 passed)`,
      `5 steps (5 passed)`.
- [x] REFACTOR: keep archive's existing no-feature refresh behavior unchanged and
      rerun. Evidence: after `cargo fmt --all`, `cargo check --no-default-features`,
      and `cargo check --features spec-retrieval`, targeted run passed with `1
      scenario (1 passed)`, `5 steps (5 passed)`.
- [x] COMMIT: `feat(semantic-review): Blocking review reports retrieval unavailability without its feature`
      Hash: `1e1e2ee`

### Advisory spec review does not create the semantic report

- [x] RED: target advisory separation with strict stubs. Evidence: `1 scenario
      (1 failed)`, `1 step (1 failed)`; the advisory-command When step panicked
      with `not implemented`.
- [x] GREEN: leave `spec review` on the existing advisory entrypoint and assert
      its retrieval advisory output with no semantic report. Production files:
      existing `src/cli/spec.rs`/`src/spec/retrieval.rs` behavior and tests step
      files. Evidence: targeted run passed with `1 scenario (1 passed)`, `5
      steps (5 passed)`.
- [x] REFACTOR: rerun both advisory and blocking paths. Evidence: after `cargo
      fmt --all`, advisory targeted run passed with `1 scenario (1 passed)`, `5
      steps (5 passed)`; blocking path is covered by the no-feature and blocked
      semantic-review scenarios.
- [x] COMMIT: `feat(semantic-review): Advisory spec review does not create the semantic report`
      Hash: `1cb640f`

## E2E setup

- [x] Configure `verify.e2e_command` as `./run-tests.sh --e2e`, register the
      separate E2E module, and prove full versus E2E scenario counts. Evidence:
      `givn/commands.yaml` contains `./run-tests.sh --e2e`,
      `tests/steps/semantic_review_e2e_steps.rs` is registered separately, and
      `features_runner` uses `.fail_on_skipped()`. Full change run before E2E
      implementation: `44 scenarios (43 passed, 1 failed)`, `421 steps (420
      passed, 1 failed)`. E2E-filtered run: `1 scenario (1 failed)`, `1 step
      (1 failed)`. The E2E count is strictly less than the full count and the
      non-zero result is the intentional RED stub failure.

## E2E scenarios

### Change review generates a complete report for a semantic candidate

- [x] RED: remove only this scenario's `@wip`, bind real-process E2E stubs, and
      run the targeted E2E command. Evidence: `1 scenario (1 failed)`, `1 step
      (1 failed)`; the Given step panicked with `not implemented`.
- [x] GREEN: invoke the compiled CLI with `check review --change`, assert exit
      status and report evidence through the real subprocess. E2E file:
      `tests/steps/semantic_review_e2e_steps.rs`; shared fixture support is in
      `tests/steps/semantic_review_steps.rs`. Evidence: E2E run passed with `1
      scenario (1 passed)`, `12 steps (12 passed)`.
- [x] REFACTOR: keep report/CLI assertions on the real interface and rerun. Evidence:
      after `cargo fmt --all`, E2E run passed with `1 scenario (1 passed)`,
      `12 steps (12 passed)`.
- [x] COMMIT: `test(e2e): Change review generates a complete report for a semantic candidate`
      Hash: `4fe0259`

## Completion Evidence

- [x] Remove all `@wip` tags only after targeted GREEN and REFACTOR evidence.
      Evidence: `grep` over `givn/changes/semantic-review/specs` found no
      `@wip` tags; the only E2E tag was retained and the E2E scenario passed its
      targeted RED/GREEN/REFACTOR loop.
- [x] Run `givn lint --change semantic-review` and record exit 0. Evidence:
      `givn lint --change semantic-review` returned exit 0: `1 file(s) checked — clean`.
- [x] Run the change-specific full runner and E2E runner. Evidence:
      `GIVN_FEATURES=givn/changes/semantic-review/specs ./run-tests.sh` passed
      with `44 scenarios (44 passed)`, `432 steps (432 passed)`; the E2E-filtered
      command passed with `1 scenario (1 passed)`, `12 steps (12 passed)`.
- [x] Run `./run-tests.sh`, `./run-tests.sh --e2e`, format, retrieval/no-feature
      checks, library tests, Clippy, and coverage measurement/merge. Evidence:
      repository full runner passed with `239 scenarios`, `1360 steps`; repository
      E2E runner passed with `36 scenarios`, `219 steps`; `cargo fmt --all --
      --check`, `cargo check --no-default-features`, `cargo check --features
      spec-retrieval`, `cargo test --lib --no-default-features` (`142 passed`),
      `cargo test --lib --features spec-retrieval` (`144 passed`), and
      `cargo clippy --workspace --all-targets -- -D warnings` all passed.
      `./measure-coverage.sh && ./merge-coverages.sh` passed and produced the
      merged Cobertura report.
- [x] Confirm the semantic report never mutates feature files, permanent specs,
      dispositions, or the permanent-only index boundary. Evidence:
      change-specific non-mutation scenario passed with snapshots for active
      feature files, permanent specs, `review.md`, and the permanent index;
      malformed-index recovery also passed the permanent-only structural checks;
      archive retention passed with the report copied only to `givn/archive/`.
