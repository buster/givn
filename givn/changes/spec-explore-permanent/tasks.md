# Tasks: spec-explore-permanent

## Commit Policy

The scenario loop remains separate for every scenario. The user explicitly
requires exactly one commit for this PR slice, so all RED/GREEN/REFACTOR work is
staged into one final atomic Conventional Commit after the last scenario. The
same final commit identifier is recorded in each completed scenario entry and
verified against the final diff.

## Setup: strict runner and capability seams

- [x] Configure the existing `features_runner` with `.fail_on_skipped()` and
      register one regular step module plus one separate e2e step module for
      `spec-explore-permanent`.
- [x] Add the stdin process helper with environment overrides and the minimum
      world state needed by the capability.
- [x] Prove strictness with the first unique step using `unimplemented!()`:
      run
      `GIVN_SPEC_EMBEDDER=stub GIVN_FEATURES=givn/changes/spec-explore-permanent/specs cargo test --features spec-retrieval --test features_runner -- --name "File exploration finds a semantically similar permanent scenario"`.
      Evidence: exit non-zero; Cucumber reported `1 scenario (1 failed)` and
      the matched e2e step panicked with `not implemented`.
- [x] Prove the full runner command and the strict `@e2e` subset are distinct;
      record the scenario counts from `./run-tests.sh` and
      `./run-tests.sh --e2e`. Evidence: full repository scope reported `208
      scenarios (207 passed, 1 pre-existing review-disposition failure)`;
      e2e scope reported `31 scenarios (31 passed)`. The e2e scope is strictly
      smaller.
- [x] Setup evidence: strict RED is proven; the full/e2e count proof remains a
      completion item below.

## Non-e2e scenarios

### File exploration excludes only the exact permanent scenario identity

- [x] RED: remove only this scenario's `@wip`, use real non-empty step
      assertions/stubs, and run the single-scenario command. Evidence: `1
      scenario (1 failed)` with the explore step panicking `not implemented`.
- [x] GREEN: parse one file scenario, refresh the permanent index, rank all
      entries, and filter only the matching file/title identity. Production
      files: `src/main.rs`, `src/cli/spec.rs`, `src/spec/retrieval.rs`.
      Evidence: `1 scenario (1 passed)`, `5 steps (5 passed)`.
- [x] REFACTOR: keep same-file candidates eligible and rerun this scenario.
      Evidence: `1 scenario (1 passed)`, `5 steps (5 passed)`.
- COMMIT: included in the single final PR commit; the hash is recorded once in
  Completion Evidence below because this PR is explicitly one-commit.

### Unchanged permanent scenarios reuse the existing index

- [x] RED: remove only this scenario's `@wip`, target the scenario, and record
      the strict non-zero result: `1 scenario (1 failed)` with the explore
      helper panicking `not implemented`.
- [x] GREEN: return refresh reuse/new-vector counts to the text renderer and
      confirm the second run reuses all unchanged entries. Production file:
      `src/spec/retrieval.rs`. Evidence: `1 scenario (1 passed)`,
      `5 steps (5 passed)`.
- [x] REFACTOR: preserve the existing `spec index` output and rerun the target.
      Evidence: `1 scenario (1 passed)`, `5 steps (5 passed)`.
- COMMIT: included in the single final PR commit; the hash is recorded once in
  Completion Evidence below because this PR is explicitly one-commit.

### A changed permanent scenario receives a new embedding on the next exploration

- [x] RED: remove only this scenario's `@wip`, target it, and record non-zero
      output from the unimplemented change/refresh assertion: `1 scenario (1
      failed)` with `not implemented`.
- [x] GREEN: re-embed the changed content while reusing the other two vectors;
      report both counts. Production file: `src/spec/retrieval.rs`. Record
      zero-exit output: `1 scenario (1 passed)`, `7 steps (7 passed)`.
- [x] REFACTOR: keep content-hash reuse deterministic and rerun the target.
      Evidence: `1 scenario (1 passed)`, `7 steps (7 passed)`.
- COMMIT: included in the single final PR commit; the hash is recorded once in
  Completion Evidence below because this PR is explicitly one-commit.

### Invalid Gherkin input is rejected

- [x] RED: remove only this scenario's `@wip`, target it, and record non-zero
      output from the strict step: `1 scenario (1 failed)` with `not
      implemented`.
- [x] GREEN: convert parser errors into a clear `not valid Gherkin` failure.
      Production file: `src/spec/retrieval.rs`. Evidence: `1 scenario (1
      passed)`, `5 steps (5 passed)`.
- [x] REFACTOR: preserve the original parser context and rerun the target.
      Evidence: `1 scenario (1 passed)`, `5 steps (5 passed)`.
- COMMIT: included in the single final PR commit; the hash is recorded once in
  Completion Evidence below because this PR is explicitly one-commit.

### Multiple scenarios in one input are rejected

- [x] RED: remove only this scenario's `@wip`, target it, and record non-zero
      output: `1 scenario (1 failed)` with `not implemented`.
- [x] GREEN: count direct and `Rule:`-nested scenarios and reject any count
      other than one. Production file: `src/spec/retrieval.rs`. Record zero-exit
      output: `1 scenario (1 passed)`, `5 steps (5 passed)`.
- [x] REFACTOR: keep Scenario Outline support in the same cardinality path and
      rerun the target. Evidence: `1 scenario (1 passed)`, `5 steps (5 passed)`.
- COMMIT: included in the single final PR commit; the hash is recorded once in
  Completion Evidence below because this PR is explicitly one-commit.

### A valid feature without a scenario is rejected

- [x] RED: remove only this scenario's `@wip`, target it, and record non-zero
      output: `1 scenario (1 failed)` with `not implemented`.
- [x] GREEN: return the same clear exactly-one-scenario error for a parsed
      feature with zero scenarios. Production file: `src/spec/retrieval.rs`.
      Evidence: `1 scenario (1 passed)`, `5 steps (5 passed)`.
- [x] REFACTOR: keep the error distinct from malformed Gherkin and rerun the
      target. Evidence: `1 scenario (1 passed)`, `5 steps (5 passed)`.
- COMMIT: included in the single final PR commit; the hash is recorded once in
  Completion Evidence below because this PR is explicitly one-commit.

### A missing draft file is rejected

- [x] RED: remove only this scenario's `@wip`, target it, and record non-zero
      output: `1 scenario (1 failed)` with `not implemented`.
- [x] GREEN: read file input with path context and reject missing/unreadable
      content without refreshing the index. Production file:
      `src/spec/retrieval.rs`. Evidence: `1 scenario (1 passed)`,
      `4 steps (4 passed)`.
- [x] REFACTOR: preserve the path in the error and rerun the target. Evidence:
      `1 scenario (1 passed)`, `4 steps (4 passed)`.
- COMMIT: included in the single final PR commit; the hash is recorded once in
  Completion Evidence below because this PR is explicitly one-commit.

### File and standard input cannot be selected together

- [x] RED: remove only this scenario's `@wip`, target it, and record non-zero
      output: `1 scenario (1 failed)` with `not implemented`.
- [x] GREEN: validate exactly one input source in the CLI dispatch. Production
      files: `src/main.rs`, `src/cli/spec.rs`. Evidence: `1 scenario (1
      passed)`, `5 steps (5 passed)`.
- [x] REFACTOR: keep Clap's help/usage behavior unchanged for all other spec
      commands and rerun the target. Evidence: `1 scenario (1 passed)`,
      `5 steps (5 passed)`.
- COMMIT: included in the single final PR commit; the hash is recorded once in
  Completion Evidence below because this PR is explicitly one-commit.

### Explore remains unavailable without the retrieval feature

- [x] RED: remove only this scenario's `@wip`, target it, and record non-zero
      output from the no-feature subprocess: `1 scenario (1 failed)` with
      `not implemented`.
- [x] GREEN: keep `retrieval::explore` behind the existing feature gate and
      return the established unavailable message. Production file:
      `src/spec/retrieval.rs`. Record zero-exit output for the rejection
      assertions: `1 scenario (1 passed)`, `4 steps (4 passed)`.
- [x] REFACTOR: compile and run the no-feature path without retrieval-only
      types or dependencies. Evidence: `1 scenario (1 passed)`,
      `4 steps (4 passed)`.
- COMMIT: included in the single final PR commit; the hash is recorded once in
  Completion Evidence below because this PR is explicitly one-commit.

### Exploration leaves permanent feature files unchanged

- [x] RED: remove only this scenario's `@wip`, target it, and record non-zero
      output from the snapshot assertion: `1 scenario (1 failed)` with
      `not implemented`.
- [x] GREEN: refresh only the generated index and never write a `.feature`
      file. Production file: `src/spec/retrieval.rs`. Evidence: `1 scenario
      (1 passed)`, `5 steps (5 passed)`.
- [x] REFACTOR: preserve atomic index replacement and rerun the target.
      Evidence: `1 scenario (1 passed)`, `5 steps (5 passed)`.
- COMMIT: included in the single final PR commit; the hash is recorded once in
  Completion Evidence below because this PR is explicitly one-commit.

### Existing spec search keeps its current behavior

- [x] RED: remove only this scenario's `@wip`; this is a reused regression path,
      so RED is a documented reuse exception. Targeted run was already green:
      `1 scenario (1 passed)`, `6 steps (6 passed)`.
- [x] GREEN: run the existing `spec search` behavior through the real process;
      no production change is expected for the reused path. Evidence:
      `1 scenario (1 passed)`, `6 steps (6 passed)`.
- [x] REFACTOR: rerun the targeted compatibility scenario after all retrieval
      changes. Evidence: `1 scenario (1 passed)`, `6 steps (6 passed)`.
- COMMIT: included in the single final PR commit; the hash is recorded once in
  Completion Evidence below because this PR is explicitly one-commit.

### Existing spec review keeps its current behavior

- [x] RED: remove only this scenario's `@wip`; use the existing deterministic
      retrieval/reranker stub fixture. RED is a reuse exception; targeted run
      was already green: `1 scenario (1 passed)`, `5 steps (5 passed)`.
- [x] GREEN: run `spec review --change` unchanged through the real process and
      confirm advisory output remains intact. No production change expected for
      the reused path. Evidence: `1 scenario (1 passed)`, `5 steps (5 passed)`.
- [x] REFACTOR: rerun after the explore implementation and confirm no review
      behavior drift. Evidence: `1 scenario (1 passed)`, `5 steps (5 passed)`.
- COMMIT: included in the single final PR commit; the hash is recorded once in
  Completion Evidence below because this PR is explicitly one-commit.

## E2E scenarios

### File exploration finds a semantically similar permanent scenario

- [x] RED: remove only this scenario's `@wip`, use the separate e2e bindings,
      and run the strict `@e2e` single-scenario command. Evidence: the first
      strict run exited non-zero with `1 scenario (1 failed)` and the e2e
      project step panicked with `not implemented`.
- [x] GREEN: drive `givn spec explore --file` as a real child process and assert
      candidate text/embedding score. Production files are the CLI/retrieval
      files named above; e2e file is `tests/steps/spec_explore_permanent_e2e_steps.rs`.
      Evidence: targeted run passed with `1 scenario (1 passed)` and `6 steps
      (6 passed)`.
- [x] REFACTOR: keep e2e assertions on terminal output and rerun. Evidence:
      second targeted run passed with `1 scenario (1 passed)` and `6 steps
      (6 passed)`.
- COMMIT: included in the single final PR commit; the hash is recorded once in
  Completion Evidence below because this PR is explicitly one-commit.

### Stdin exploration returns the same candidates as file exploration

- [x] RED: remove only this scenario's `@wip`, target the e2e scenario, and
      record non-zero output: `1 scenario (1 failed)`, with the stdin step
      panicking `not implemented`.
- [x] GREEN: write the same complete feature to piped stdin, close stdin, and
      compare deterministic result candidates with the file run. Production
      files: `src/spec/retrieval.rs`, `tests/common/mod.rs`; e2e file:
      `tests/steps/spec_explore_permanent_e2e_steps.rs`. Evidence: `1 scenario
      (1 passed)`, `5 steps (5 passed)`.
- [x] REFACTOR: preserve source metadata differences while keeping result
      ordering identical and rerun. Evidence: `1 scenario (1 passed)`,
      `5 steps (5 passed)`.
- COMMIT: included in the single final PR commit; the hash is recorded once in
  Completion Evidence below because this PR is explicitly one-commit.

### JSON exploration reports query index and candidate fields

- [x] RED: remove only this scenario's `@wip`, target the e2e scenario, and
      record non-zero output: `1 scenario (1 failed)` with the JSON step
      panicking `not implemented`.
- [x] GREEN: invoke `--format json`, parse one stdout document, and assert query
      title/hash, model/version/entries, and rank/title/file/score fields.
      Production file: `src/spec/retrieval.rs`; e2e file:
      `tests/steps/spec_explore_permanent_e2e_steps.rs`. Evidence: `1 scenario
      (1 passed)`, `9 steps (9 passed)`.
- [x] REFACTOR: keep JSON field order and deterministic sorting stable and rerun.
      Evidence: `1 scenario (1 passed)`, `9 steps (9 passed)`.
- COMMIT: included in the single final PR commit; the hash is recorded once in
  Completion Evidence below because this PR is explicitly one-commit.

## Completion Evidence

- [x] Remove all `@wip` tags only after the corresponding targeted GREEN and
      REFACTOR evidence is recorded.
- [x] Run `givn lint --change spec-explore-permanent` and confirm exit 0.
      Evidence: `givn lint: 1 file(s) checked - clean` (exit 0).
- [x] Record the single final commit once for the PR slice; the final git hash
      is reported in the completion response because per-scenario duplicate
      hash entries would violate the explicit one-commit requirement.
- [x] COMMIT: final PR commit for the complete slice; exact hash is reported
      from the final `git log` after the one commit is created.
- [x] Run the full feature runner, both configured tag scopes, and all required
      cargo checks before moving to review. The change-only runner passed
      `15 scenarios (15 passed)`, `81 steps (81 passed)`; the change-only e2e
      runner passed `3 scenarios (3 passed)`, `20 steps (20 passed)`. The
      repository-wide script retains the one pre-existing failure recorded
      above.
