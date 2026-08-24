# Tasks: spec-progress-output

## Setup

- [ ] Register `spec_progress_output_steps.rs` and
  `spec_progress_output_e2e_steps.rs` in `tests/steps/mod.rs`, add the
  `hf_endpoint_override` field to `GivnWorld`, create the two step skeleton
  files (all steps `unimplemented!()`), and record the strict-mode proof.
  - Runner command: `./run-tests.sh`
  - E2E command: `./run-tests.sh --e2e`
  - Strict mechanism: `GivnWorld::cucumber().fail_on_skipped()`
    (`tests/features_runner.rs:20`)
  - Strict proof command:
    `GIVN_FEATURES=givn/changes/spec-progress-output/specs cargo test --no-default-features --features spec-retrieval --test features_runner -- --name "A warm model cache produces no download announcements"`
  - Strict proof output: _pending_ (must show the unimplemented step panic and
    non-zero exit; exit 0 means the strict config is broken)

## Scenario: A warm model cache produces no download announcements

- [ ] RED: remove only this scenario's `@wip`, add step stubs with
  `unimplemented!()`, and run the exact named scenario. Output: _pending_
- [ ] GREEN: implement the `notice` helper, the embedder-preparation
  announcement, the embedding count announcement in `refresh_index`, and the
  warm-cache silence (no download notices when artifacts are cached).
  Production files: _pending_. Output: _pending_
- [ ] REFACTOR: simplify without behavior change and rerun the named scenario.
  Output: _pending_
- [ ] COMMIT: commit RED/GREEN/REFACTOR with the scenario title. Hash: _pending_

## Scenario: A failed model download still announces the started download phase

- [ ] RED: remove only this scenario's `@wip`, add step stubs with
  `unimplemented!()`, and run the exact named scenario. Output: _pending_
- [ ] GREEN: emit the acquisition-start notice before the first fetch in
  `auto_e5_artifacts` (also on failure), add the `hf_endpoint_override` env
  plumbing, and the new `stderr identifies the failed E5 model artifact` step.
  Production files: _pending_. Output: _pending_
- [ ] REFACTOR: simplify without behavior change and rerun the named scenario.
  Output: _pending_
- [ ] COMMIT: commit RED/GREEN/REFACTOR with the scenario title. Hash: _pending_

## Scenario: Disabled evidence layers announce no downloads

- [ ] RED: remove only this scenario's `@wip`, add step stubs with
  `unimplemented!()`, and run the exact named scenario. Output: _pending_
- [ ] GREEN: verify disabled BGE/NLI produce no acquisition notices (reuse
  existing disabled-evidence steps; no new production code expected beyond
  the shared acquisition notices from the previous scenario).
  Production files: _pending_. Output: _pending_
- [ ] REFACTOR: simplify without behavior change and rerun the named scenario.
  Output: _pending_
- [ ] COMMIT: commit RED/GREEN/REFACTOR with the scenario title. Hash: _pending_

## Scenario: Embedding multiple scenarios announces the count and reports per-scenario progress

- [ ] RED: remove only this scenario's `@wip`, add step stubs with
  `unimplemented!()`, and run the exact named scenario. Output: _pending_
- [ ] GREEN: add the progress callback parameter to `embed_texts_with_policy`
  and emit `embedded <i>/<n> scenarios` per scenario in `refresh_index`; add
  the three-scenario warm-cache fixture. Production files: _pending_.
  Output: _pending_
- [ ] REFACTOR: simplify without behavior change and rerun the named scenario.
  Output: _pending_
- [ ] COMMIT: commit RED/GREEN/REFACTOR with the scenario title. Hash: _pending_

## Scenario: Cold-cache indexing announces each phase on stderr (@e2e)

- [ ] RED: remove only this scenario's `@wip`, add e2e step stubs with
  `unimplemented!()`, and run the exact named scenario via the e2e runner.
  Output: _pending_
- [ ] GREEN: implement `StderrDownloadProgress` and switch `fetch_hf_artifacts`
  to `download_with_progress`; emit acquisition start/end and per-file
  progress; extend the existing cold-cache e2e steps with the new stderr
  assertions. Production files: _pending_. Output: _pending_
- [ ] REFACTOR: simplify without behavior change and rerun the named scenario.
  Output: _pending_
- [ ] COMMIT: commit RED/GREEN/REFACTOR with the scenario title. Hash: _pending_

## Scenario: Search with a cold E5 cache announces model acquisition before results (@e2e)

- [ ] RED: remove only this scenario's `@wip`, add e2e step stubs with
  `unimplemented!()`, and run the exact named scenario via the e2e runner.
  Output: _pending_
- [ ] GREEN: add the cold-E5-cache-with-index fixture and the
  search-with-acquisition e2e when-step; confirm `search` emits the shared
  acquisition notices and ranked results on stdout. Production files:
  _pending_. Output: _pending_
- [ ] REFACTOR: simplify without behavior change and rerun the named scenario.
  Output: _pending_
- [ ] COMMIT: commit RED/GREEN/REFACTOR with the scenario title. Hash: _pending_

## Scenario: Duplicate audit keeps stdout pure JSON while progress runs on stderr (@e2e, @nli-combined)

- [ ] RED: remove only this scenario's `@wip`, add e2e step stubs with
  `unimplemented!()`, and run the exact named scenario via the e2e runner.
  Output: _pending_
- [ ] GREEN: emit BGE/NLI acquisition notices in `auto_bge_artifacts` and
  `auto_nli_artifacts` (one begin/end per phase, per-file progress per
  download); reuse the second-stage e2e steps with the new stderr assertions.
  Production files: _pending_. Output: _pending_
- [ ] REFACTOR: simplify without behavior change and rerun the named scenario.
  Output: _pending_
- [ ] COMMIT: commit RED/GREEN/REFACTOR with the scenario title. Hash: _pending_

## Scenario: Advisory review announces second-stage model acquisition during the run (@e2e, @nli-combined)

- [ ] RED: remove only this scenario's `@wip`, add e2e step stubs with
  `unimplemented!()`, and run the exact named scenario via the e2e runner.
  Output: _pending_
- [ ] GREEN: add the completed-index-with-cold-second-stage-cache fixture and
  the review-with-acquisition e2e when-step; confirm advisory review emits
  BGE/NLI acquisition notices and `BGE status:` evidence on stdout.
  Production files: _pending_. Output: _pending_
- [ ] REFACTOR: simplify without behavior change and rerun the named scenario.
  Output: _pending_
- [ ] COMMIT: commit RED/GREEN/REFACTOR with the scenario title. Hash: _pending_

## Final verification

- [ ] Full non-e2e suite: `GIVN_FEATURES=givn/changes/spec-progress-output/specs ./run-tests.sh`
  plus the default `./run-tests.sh` (permanent specs unaffected). Output: _pending_
- [ ] Full e2e suite: `GIVN_FEATURES=givn/changes/spec-progress-output/specs ./run-tests.sh --e2e`
  plus the default `./run-tests.sh --e2e`. Output: _pending_
- [ ] `cargo fmt --all && cargo clippy --workspace --all-targets -- -D warnings`.
  Output: _pending_
