# Tasks: spec-explore-change

## Commit Policy

Each scenario is completed as one atomic commit containing its RED, GREEN, and
REFACTOR work. The commit message references the scenario title verbatim, and
the hash is recorded immediately before the next scenario starts. Planning and
Arc42 changes are already present before implementation; no scenario is marked
done without targeted runner evidence and a production or real-interface file
in its commit.

## Setup: strict runner and capability seams

- [x] Register `spec_explore_change_steps` and
      `spec_explore_change_e2e_steps` in `tests/steps/mod.rs`, add the
      permanent-index snapshot state, and keep `.fail_on_skipped()` as the
      strict runner configuration.
      Evidence: modules registered and snapshot state added; the existing
      `.fail_on_skipped()` remains active.
- [x] Prove strict RED with the first scenario using `unimplemented!()`:
      run
      `GIVN_FEATURES=givn/changes/spec-explore-change/specs GIVN_SPEC_EMBEDDER=stub GIVN_SPEC_RERANKER=stub cargo test --features spec-retrieval --test features_runner -- --name "Removed change scenarios are absent from exploration results"`.
      Evidence: exit 101; `1 scenario (1 failed)`, `1 step (1 failed)`, and
      the matched step panicked with `not implemented`.
- [x] Confirm the configured full runner and e2e runner remain distinct after
      the new modules are registered. Record both scenario counts and confirm
      the `@e2e` count is strictly smaller.
      Commands: `./run-tests.sh` and `./run-tests.sh --e2e`.
      Evidence: full runner `223 scenarios (223 passed), 1254 steps`; e2e
      runner `34 scenarios (34 passed), 203 steps`.

## Non-e2e scenarios

### Removed change scenarios are absent from exploration results

- [x] RED: remove only this scenario's `@wip`, bind the new regular steps with
      `unimplemented!()`, and run the targeted command. Evidence:
      `1 scenario (1 failed)`, `1 step (1 failed)`; the matched Given step
      panicked with `not implemented`.
- [x] GREEN: resolve the requested change, collect active delta scenarios, omit
      `@givn.removed`, and assert that the removed title is absent from ranked
      CLI results. Production files: `src/main.rs`, `src/cli/spec.rs`,
      `src/spec/retrieval.rs`, `tests/steps/mod.rs`,
      `tests/steps/spec_explore_change_steps.rs`. Evidence: targeted run
      passed with `1 scenario (1 passed)`, `5 steps (5 passed)`.
- [x] REFACTOR: keep removed scenarios out of transient embeddings and rerun
      the targeted command. Evidence: after `cargo fmt --all`, targeted run
      passed with `1 scenario (1 passed)`, `5 steps (5 passed)`.
- [x] COMMIT: `feat(spec-explore-change): Removed change scenarios are absent from exploration results`
      Hash: `0d36bd7`

### Rule-nested active change scenarios are searchable

- [x] RED: remove only this scenario's `@wip`, bind the Rule-nested fixture
      step with `unimplemented!()`, and run the targeted command. Evidence: `1
      scenario (1 failed)`, `1 step (1 failed)`; the matched Given step
      panicked with `not implemented`.
- [x] GREEN: collect active scenarios nested under `Rule:` and rank the
      request-local candidate through the real CLI. Production files:
      `src/spec/retrieval.rs`, `tests/steps/spec_explore_change_steps.rs`.
      Evidence: targeted run passed with `1 scenario (1 passed)`, `6 steps (6
      passed)`.
- [x] REFACTOR: preserve the shared delta collector and rerun the targeted
      command after `cargo fmt --all`. Evidence: targeted run passed with `1
      scenario (1 passed)`, `6 steps (6 passed)`.
- [x] COMMIT: `feat(spec-explore-change): Rule-nested active change scenarios are searchable`
      Hash: `df91758`

### A modified change scenario is not reported against itself from its delta source

- [x] RED: remove only this scenario's `@wip`, retain strict stubs, and run the
      targeted command. Evidence: `1 scenario (1 failed)`, `1 step (1
      failed)`; the matched Given step panicked with `not implemented`.
- [x] GREEN: normalize the delta source identity and filter the modified
      scenario against its own source while retaining another candidate.
      Production files: `src/spec/retrieval.rs`,
      `tests/steps/spec_explore_change_steps.rs`. Evidence: targeted run
      passed with `1 scenario (1 passed)`, `5 steps (5 passed)`.
- [x] REFACTOR: separate source identity filtering from ranking and rerun the
      targeted command. Evidence: after `cargo fmt --all`, targeted run
      passed with `1 scenario (1 passed)`, `5 steps (5 passed)`.
- [x] COMMIT: `feat(spec-explore-change): A modified change scenario is not reported against itself from its delta source`
      Hash: `eb3831b`

### Change exploration does not persist delta scenarios in the permanent index

- [x] RED: remove only this scenario's `@wip`, use strict stubs, and run the
      targeted command. Evidence: `1 scenario (1 failed)`, `1 step (1
      failed)`; the matched Given step panicked with `not implemented`.
- [x] GREEN: rank request-local delta vectors without passing them to index
      persistence and compare the snapped permanent index. Production files:
      `src/spec/retrieval.rs`, `tests/steps/mod.rs`,
      `tests/steps/spec_explore_change_steps.rs`. Evidence: targeted run
      passed with `1 scenario (1 passed)`, `7 steps (7 passed)`.
- [x] REFACTOR: preserve permanent refresh reuse and rerun the targeted
      command. Evidence: after `cargo fmt --all`, targeted run passed with `1
      scenario (1 passed)`, `7 steps (7 passed)`.
- [x] COMMIT: `feat(spec-explore-change): Change exploration does not persist delta scenarios in the permanent index`
      Hash: `df1b43f`

### An added change scenario is not reported against itself from its delta source

- [x] RED: remove only this scenario's `@wip`, bind the added-source identity
      steps with strict stubs, and run the targeted command. Evidence: `1
      scenario (1 failed)`, `1 step (1 failed)`; the matched Given step
      panicked with `not implemented`.
- [x] GREEN: exclude an added candidate whose normalized source file and title
      equal the query identity while retaining another result. Production
      files: `src/spec/retrieval.rs`,
      `tests/steps/spec_explore_change_steps.rs`. Evidence: targeted run
      passed with `1 scenario (1 passed)`, `5 steps (5 passed)`.
- [x] REFACTOR: centralize exact identity matching for added and modified
      records and rerun the targeted command. Evidence: after `cargo fmt
      --all`, targeted run passed with `1 scenario (1 passed)`, `5 steps (5
      passed)`.
- [x] COMMIT: `feat(spec-explore-change): An added change scenario is not reported against itself from its delta source`
      Hash: `af62538`

### A modified change scenario is not reported against itself through its projected permanent target

- [x] RED: remove only this scenario's `@wip`, bind strict stubs, and run the
      targeted command. Evidence: `1 scenario (1 failed)`, `1 step (1
      failed)`; the matched Given step panicked with `not implemented`.
- [x] GREEN: exclude a modified candidate when the query matches its projected
      permanent target identity, while preserving another eligible candidate.
      Production files: `src/spec/retrieval.rs`,
      `tests/steps/spec_explore_change_steps.rs`. Evidence: targeted run
      passed with `1 scenario (1 passed)`, `5 steps (5 passed)`.
- [x] REFACTOR: reuse the same canonical identity predicate for source and
      projected-target paths and rerun the targeted command. Evidence: after
      `cargo fmt --all`, targeted run passed with `1 scenario (1 passed)`, `5
      steps (5 passed)`.
- [x] COMMIT: `feat(spec-explore-change): A modified change scenario is not reported against itself through its projected permanent target`
      Hash: `5de7ff0`

### Change-aware exploration normalizes relative and absolute scenario identities

- [x] RED: remove only this scenario's `@wip`, use strict stubs, and run the
      targeted command. Evidence: `1 scenario (1 failed)`, `1 step (1
      failed)`; the matched When step panicked with `not implemented`.
- [x] GREEN: route relative and absolute query paths, delta source paths,
      permanent paths, and projected target paths through one project-relative
      forward-slash helper. Production files: `src/spec/retrieval.rs`,
      `tests/steps/spec_explore_change_steps.rs`. Evidence: targeted run
      passed with `1 scenario (1 passed)`, `5 steps (5 passed)`.
- [x] REFACTOR: remove duplicate path conversions and rerun both targeted path
      assertions. Evidence: after `cargo fmt --all`, targeted run passed with
      `1 scenario (1 passed)`, `5 steps (5 passed)`.
- [x] COMMIT: `feat(spec-explore-change): Change-aware exploration normalizes relative and absolute scenario identities`
      Hash: `2fd7acf`

### Change-aware standard input returns the same ranked candidates as file input

- [x] RED: remove only this scenario's `@wip`, use strict regular steps, and
      run the targeted command. Evidence: `1 scenario (1 failed)`, `1 step
      (1 failed)`; the matched stdin step panicked with `not implemented`.
- [x] GREEN: pass the optional change through the stdin path and compare the
      ranked result lines against file input while preserving source metadata.
      Production files: `src/main.rs`, `src/cli/spec.rs`,
      `src/spec/retrieval.rs`, `tests/steps/spec_explore_change_steps.rs`.
      Evidence: targeted run passed with `1 scenario (1 passed)`, `5 steps (5
      passed)`.
- [x] REFACTOR: keep source metadata differences out of ranked-line parity and
      rerun the targeted command. Evidence: after `cargo fmt --all`, targeted
      run passed with `1 scenario (1 passed)`, `5 steps (5 passed)`.
- [x] COMMIT: `feat(spec-explore-change): Change-aware standard input returns the same ranked candidates as file input`
      Hash: `58043f3`

### Permanent exploration text identifies permanent candidate sources

- [x] RED: remove only this scenario's `@wip`, bind strict regular steps, and
      run the targeted command. Evidence: `1 scenario (1 failed)`, `1 step
      (1 failed)`; the matched Then step panicked with `not implemented`.
- [x] GREEN: add source scope and source path to every human-readable result,
      including the no-change permanent-only path. Production files:
      `src/spec/retrieval.rs`, `tests/steps/spec_explore_change_steps.rs`.
      Evidence: targeted run passed with `1 scenario (1 passed)`, `7 steps (7
      passed)`.
- [x] REFACTOR: preserve PR-1 query/index/refresh text and rerun the targeted
      command. Evidence: after `cargo fmt --all`, targeted run passed with `1
      scenario (1 passed)`, `7 steps (7 passed)`.
- [x] COMMIT: `feat(spec-explore-change): Permanent exploration text identifies permanent candidate sources`
      Hash: `7386892`

### Permanent exploration JSON adds source metadata while retaining legacy result fields

- [x] RED: remove only this scenario's `@wip`, use strict regular steps, and
      run the targeted command. Evidence: `1 scenario (1 failed)`, `1 step
      (1 failed)`; the matched When step was initially undefined, then the
      first JSON assertion panicked with `not implemented` after binding.
- [x] GREEN: append `scope` and `change` to the existing JSON result fields,
      label permanent-only results, and assert the single-document output.
      Production files: `src/spec/retrieval.rs`,
      `tests/steps/spec_explore_change_steps.rs`. Evidence: targeted run
      passed with `1 scenario (1 passed)`, `7 steps (7 passed)`.
- [x] REFACTOR: keep legacy field values/order stable and rerun the targeted
      command. Evidence: after `cargo fmt --all`, targeted run passed with `1
      scenario (1 passed)`, `7 steps (7 passed)`.
- [x] COMMIT: `feat(spec-explore-change): Permanent exploration JSON adds source metadata while retaining legacy result fields`
      Hash: `ec9c082`

### Unsafe change identifiers are rejected without changing the permanent index

- [x] RED: remove only this scenario's `@wip`, bind strict validation and index
      snapshot steps, and run the targeted command. Evidence: `1 scenario (1
      failed)`, `1 step (1 failed)`; the unsafe-change When step panicked with
      `not implemented`.
- [x] GREEN: pass `--change` through validation before any path construction or
      refresh and report the kebab-case error. Production files:
      `src/spec/retrieval.rs`, `tests/steps/spec_explore_change_steps.rs`.
      Evidence: targeted run passed with `1 scenario (1 passed)`, `8 steps (8
      passed)`.
- [x] REFACTOR: reuse the shared change resolver and rerun the targeted
      command. Evidence: after `cargo fmt --all`, targeted run passed with `1
      scenario (1 passed)`, `8 steps (8 passed)`.
- [x] COMMIT: `feat(spec-explore-change): Unsafe change identifiers are rejected without changing the permanent index`
      Hash: `58a56c2`

### A missing change is rejected without changing the permanent index

- [x] RED: remove only this scenario's `@wip`, retain strict stubs, and run the
      targeted command. Evidence: `1 scenario (1 failed)`, `1 step (1
      failed)`; the missing-change assertion panicked with `not implemented`.
- [x] GREEN: require an existing change directory and emit a clear missing
      change error before refresh. Production files: `src/spec/retrieval.rs`,
      `tests/steps/spec_explore_change_steps.rs`. Evidence: targeted run
      passed with `1 scenario (1 passed)`, `9 steps (9 passed)`.
- [x] REFACTOR: keep missing-change validation separate from delta parsing and
      rerun the targeted command. Evidence: after `cargo fmt --all`, targeted
      run passed with `1 scenario (1 passed)`, `9 steps (9 passed)`.
- [x] COMMIT: `feat(spec-explore-change): A missing change is rejected without changing the permanent index`
      Hash: `bcff09f`

### A change without a specs directory is rejected without changing the permanent index

- [x] RED: remove only this scenario's `@wip`, bind strict stubs, and run the
      targeted command. Evidence: `1 scenario (1 failed)`, `1 step (1
      failed)`; the missing-specs assertion panicked with `not implemented`.
- [x] GREEN: require the change's specs directory before refresh and report the
      existing delta-path error with the change id. Production files:
      `src/spec/retrieval.rs`, `tests/steps/spec_explore_change_steps.rs`.
      Evidence: targeted run passed with `1 scenario (1 passed)`, `8 steps (8
      passed)`.
- [x] REFACTOR: reuse `delta_specs_path` and rerun the targeted command.
      Evidence: after `cargo fmt --all`, targeted run passed with `1 scenario
      (1 passed)`, `8 steps (8 passed)`.
- [x] COMMIT: `feat(spec-explore-change): A change without a specs directory is rejected without changing the permanent index`
      Hash: `fa4e4a5`

### A malformed delta feature is rejected without changing the permanent index

- [x] RED: remove only this scenario's `@wip`, bind strict stubs, and run the
      targeted command. Evidence: `1 scenario (1 failed)`, `1 step (1
      failed)`; the malformed-delta assertion panicked with `not implemented`.
- [x] GREEN: parse and validate all delta files before permanent refresh and
      report malformed delta Gherkin clearly. Production files:
      `src/spec/retrieval.rs`, `tests/steps/spec_explore_change_steps.rs`.
      Evidence: targeted run passed with `1 scenario (1 passed)`, `8 steps (8
      passed)`.
- [x] REFACTOR: preserve parser context and rerun the targeted command.
      Evidence: after `cargo fmt --all`, targeted run passed with `1 scenario
      (1 passed)`, `8 steps (8 passed)`.
- [x] COMMIT: `feat(spec-explore-change): A malformed delta feature is rejected without changing the permanent index`
      Hash: `7e007bd`

## E2E setup

- [x] Configure and prove `verify.e2e_command` as `./run-tests.sh --e2e` after
      all non-e2e scenarios are green. Run `./run-tests.sh` and
      `./run-tests.sh --e2e`; record a strictly smaller e2e scenario count.
      Evidence: full runner `223 scenarios (223 passed), 1254 steps`; e2e
      runner `34 scenarios (34 passed), 203 steps`; `givn/commands.yaml`
      remains configured with distinct commands.
- [x] Create the separate e2e step skeleton in
      `tests/steps/spec_explore_change_e2e_steps.rs` with strict
      `unimplemented!()` bodies before the first e2e RED run. No server,
      browser, external service, or digital twin is required; the real CLI
      subprocess is the interface. Evidence: module registered in
      `tests/steps/mod.rs` and initial strict e2e stubs are present.

## E2E scenarios

### Change-aware file exploration finds permanent and active change candidates

- [x] RED: remove only this scenario's `@wip`, bind the separate e2e steps with
      strict stubs, and run the targeted e2e scenario command. Evidence:
      `1 scenario (1 failed)`, `1 step (1 failed)`; the matched e2e Given step
      panicked with `not implemented`.
- [x] GREEN: drive the compiled `givn` process with `--change` and `--file` and
      assert terminal output contains permanent/current-change source labels,
      titles, and project-relative source paths. Production files:
      `src/main.rs`, `src/cli/spec.rs`, `src/spec/retrieval.rs`; e2e file:
      `tests/steps/spec_explore_change_e2e_steps.rs`; fixture file:
      `tests/steps/spec_explore_change_steps.rs`. Evidence: targeted e2e run
      passed with `1 scenario (1 passed)`, `10 steps (10 passed)`.
- [x] REFACTOR: keep assertions on real CLI stdout and rerun the targeted e2e
      command. Evidence: after `cargo fmt --all`, targeted e2e run passed with
      `1 scenario (1 passed)`, `10 steps (10 passed)`.
- [x] COMMIT: `test(e2e): Change-aware file exploration finds permanent and active change candidates`
      Hash: `2cc2d73`

### Change-aware JSON exploration labels candidate sources

- [x] RED: remove only this scenario's `@wip`, bind separate e2e steps with
      strict stubs, and run the targeted e2e scenario command. Evidence:
      `1 scenario (1 failed)`, `1 step (1 failed)`; the matched JSON When step
      panicked with `not implemented`.
- [x] GREEN: drive the real CLI with `--change`, `--file`, and `--format json`,
      parse one stdout document, and assert exact scope, owning-change, source
      file, permanent-only index count, and legacy result fields. Production
      files: `src/spec/retrieval.rs`; e2e file:
      `tests/steps/spec_explore_change_e2e_steps.rs`. Evidence: targeted e2e
      run passed with `1 scenario (1 passed)`, `6 steps (6 passed)`.
- [x] REFACTOR: keep JSON stdout free of refresh diagnostics and rerun the
      targeted e2e command. Evidence: after `cargo fmt --all`, targeted e2e
      run passed with `1 scenario (1 passed)`, `6 steps (6 passed)`.
- [x] COMMIT: `test(e2e): Change-aware JSON exploration labels candidate sources`
      Hash: `b70e495`

## Completion Evidence

- [x] Remove each `@wip` tag only after its targeted GREEN and REFACTOR evidence
      is recorded.
      Evidence: `givn/changes/spec-explore-change/specs/spec-explore-change/spec-explore-change.feature` contains no `@wip` tags; all 16 scenarios have targeted GREEN and REFACTOR evidence above.
- [x] Run `givn lint --change spec-explore-change` and record exit 0.
      Evidence: `givn lint: 1 file(s) checked — clean` after the Rule-nested scenario was completed.
- [x] Run the change-specific full runner and e2e runner:
      `GIVN_FEATURES=givn/changes/spec-explore-change/specs GIVN_SPEC_EMBEDDER=stub GIVN_SPEC_RERANKER=stub cargo test --features spec-retrieval --test features_runner`
      and the same command with `-- --tags @e2e`. Evidence:
      full `16 scenarios (16 passed)`, `106 steps (106 passed)`; e2e `2
      scenarios (2 passed)`, `16 steps (16 passed)`.
- [x] Run `./run-tests.sh`, `./run-tests.sh --e2e`, `cargo fmt --all -- --check`,
      `cargo check --features spec-retrieval`,
      `cargo check --no-default-features --locked`, retrieval and no-feature
      library tests, and clippy with `-D warnings`. Evidence: full runner
      `223 scenarios (223 passed)`, e2e runner `34 scenarios (34 passed)`;
      fmt check passed; retrieval/no-feature checks passed; library tests
      passed `144`/`142`; clippy passed with `-D warnings`; coverage merge
      measured non-e2e `1185/5309` lines, e2e `1123/5309`, merged `2308/10618`
      (`21.7367%`); branch counters were `0/0` in all three reports.
- [x] Confirm no production feature files changed during exploration and the
      permanent index remains permanent-only. Evidence: change-aware JSON
      asserts `index.entries == 2` for two permanent scenarios plus an active
      delta; the persistence scenario compares `.spec-index.json` byte-for-byte;
      `git diff` contains no modifications under `givn/specs/`.
