# Tasks: spec-progress-output

## Setup

- [x] Register `spec_progress_output_steps.rs` and
  `spec_progress_output_e2e_steps.rs` in `tests/steps/mod.rs`, add the
  `hf_endpoint_override` field to `GivnWorld`, create the two step skeleton
  files (all steps `unimplemented!()`), and record the strict-mode proof.
  - Runner command: `./run-tests.sh`
  - E2E command: `./run-tests.sh --e2e`
  - Strict mechanism: `GivnWorld::cucumber().fail_on_skipped()`
    (`tests/features_runner.rs:20`)
  - Strict proof command:
    `GIVN_FEATURES=givn/changes/spec-progress-output/specs cargo test --no-default-features --features spec-retrieval --test features_runner -- --name "A warm model cache produces no download announcements"`
  - Strict proof output:
    `1 feature`, `1 scenario (1 failed)`, `4 steps (3 passed, 1 failed)` with
    `Step panicked. Captured output: not implemented` and non-zero process
    exit. This proves `.fail_on_skipped()` rejects the unimplemented step.

## Scenario: A warm model cache produces no download announcements

- [x] RED: removed only this scenario's `@wip`; ran the named scenario with
  step stubs. Output: `1 scenario (1 failed)`, `4 steps (3 passed, 1 failed)`,
  `Step panicked. Captured output: not implemented`, non-zero exit.
- [x] GREEN: implemented the `notice` helper, the embedder-preparation
  announcement, the embedding count/complete announcements in `refresh_index`,
  and the warm-cache silence (notices only inside acquisition).
  Production files: `src/spec/retrieval.rs`. Test files:
  `tests/steps/mod.rs`, `tests/steps/spec_progress_output_steps.rs`.
  Targeted output: `1 scenario (1 passed)`, `6 steps (6 passed)`.
- [x] REFACTOR: removed the unused `mut` on the progress parameter
  (clippy `-D warnings` clean), reran the targeted scenario; output:
  `1 scenario (1 passed)`, `6 steps (6 passed)`.
- [x] COMMIT: commit RED/GREEN/REFACTOR with the scenario title.
  Hash: `62eb89b`

## Scenario: A failed model download still announces the started download phase

- [x] RED: removed only this scenario's `@wip`; ran the named scenario with
  the previous commit's step stubs. Output: `1 scenario (1 failed)`,
  `1 step (1 failed)`, `Step panicked. Captured output: not implemented`,
  non-zero exit.
- [x] GREEN: acquisition-start notice before the first fetch in
  `auto_e5_artifacts` fires even on failure; `hf_endpoint_override` env
  plumbing; new `stderr identifies the failed E5 model artifact` and
  `announces the start` steps.
  Production files: `src/spec/retrieval.rs`. Test files:
  `tests/steps/mod.rs`, `tests/steps/spec_progress_output_steps.rs`.
  Targeted output: `1 scenario (1 passed)`, `5 steps (5 passed)`.
- [x] REFACTOR: clippy `-D warnings` clean, `cargo fmt`; reran the named
  scenario: `1 scenario (1 passed)`, `5 steps (5 passed)`.
- [x] COMMIT: commit RED/GREEN/REFACTOR with the scenario title.
  Hash: `5bcce96`

## Scenario: Disabled evidence layers announce no downloads

- [x] RED: removed only this scenario's `@wip`; ran the named scenario with
  the previous commit's step stubs (6 of 7 steps reuse existing definitions).
  Output: `1 scenario (1 failed)`, `7 steps (6 passed, 1 failed)`,
  `Step panicked. Captured output: not implemented`, non-zero exit.
- [x] GREEN: implemented `stderr does not announce a BGE or NLI model
  download`; no new production code needed (acquisition notices only fire in
  the acquisition paths, which disabled layers never enter).
  Production files: none. Test files:
  `tests/steps/spec_progress_output_steps.rs`.
  Targeted output: `1 scenario (1 passed)`, `7 steps (7 passed)`.
- [x] REFACTOR: clippy `-D warnings` clean, `cargo fmt`; reran the named
  scenario: `1 scenario (1 passed)`, `7 steps (7 passed)`.
- [x] COMMIT: commit RED/GREEN/REFACTOR with the scenario title.
  Hash: `58f993c`

## Scenario: Embedding multiple scenarios announces the count and reports per-scenario progress

- [x] RED: removed only this scenario's `@wip`; ran the named scenario with
  the previous commit's step stubs. Output: `1 scenario (1 failed)`,
  `1 step (1 failed)`, `Step panicked. Captured output: not implemented`,
  non-zero exit.
- [x] GREEN: progress callback parameter in `embed_texts_with_policy`,
  per-scenario `embedded <i>/<n> scenarios` notices in `refresh_index`, and
  the three-scenario warm-cache fixture.
  Production files: `src/spec/retrieval.rs`. Test files:
  `tests/steps/spec_progress_output_steps.rs`.
  Targeted output: `1 scenario (1 passed)`, `5 steps (5 passed)`.
- [x] REFACTOR: clippy `-D warnings` clean, `cargo fmt`; reran the named
  scenario: `1 scenario (1 passed)`, `5 steps (5 passed)`.
- [x] COMMIT: commit RED/GREEN/REFACTOR with the scenario title.
  Hash: `566e8ba`

## Scenario: Cold-cache indexing announces each phase on stderr (@e2e)

- [x] RED: removed only this scenario's `@wip`; ran the named scenario via
  the e2e runner with the previous commit's step stubs. Output:
  `1 scenario (1 failed)`, `7 steps (6 passed, 1 failed)`,
  `Step panicked. Captured output: not implemented`, non-zero exit.
- [x] GREEN: `StderrDownloadProgress` + `download_with_progress` switch in
  `fetch_hf_artifacts`; acquisition start/end and per-file progress notices;
  new stderr assertions. Production files: `src/spec/retrieval.rs`. Test
  files: `tests/steps/spec_progress_output_steps.rs`.
  Targeted output: `1 scenario (1 passed)`, `11 steps (11 passed)`.
- [x] REFACTOR: clippy `-D warnings` clean, `cargo fmt`; reran the named
  scenario: `1 scenario (1 passed)`, `11 steps (11 passed)`.
- [x] COMMIT: commit RED/GREEN/REFACTOR with the scenario title.
  Hash: `c6082dc`

## Scenario: Search with a cold E5 cache announces model acquisition before results (@e2e)

- [x] RED: removed only this scenario's `@wip`; ran the named scenario via
  the e2e runner with the previous commit's step stubs. Output:
  `1 scenario (1 failed)`, `1 step (1 failed)`,
  `Step panicked. Captured output: not implemented`, non-zero exit.
- [x] GREEN: cold-E5-cache-with-index fixture, reachable-model-source step,
  search-with-acquisition when-step; search emits the shared acquisition
  notices and ranked results on stdout. Production files: none (shared
  notices from previous scenarios). Test files:
  `tests/steps/spec_progress_output_e2e_steps.rs`,
  `tests/steps/spec_progress_output_steps.rs`.
  Targeted output: `1 scenario (1 passed)`, `7 steps (7 passed)`.
- [x] REFACTOR: clippy `-D warnings` clean, `cargo fmt`; reran the named
  scenario: `1 scenario (1 passed)`, `7 steps (7 passed)`.
- [x] COMMIT: commit RED/GREEN/REFACTOR with the scenario title.
  Hash: `4173c75`

## Scenario: Duplicate audit keeps stdout pure JSON while progress runs on stderr (@e2e, @nli-combined)

- [x] RED: removed only this scenario's `@wip`; ran the named scenario via
  the e2e runner with the previous commit's step stubs. Output:
  `1 scenario (1 failed)`, `7 steps (6 passed, 1 failed)`,
  `Step panicked. Captured output: not implemented`, non-zero exit.
- [x] GREEN: BGE/NLI acquisition notices in `auto_bge_artifacts` and
  `auto_nli_artifacts` (one begin/end per phase, per-file progress per
  download); new stderr assertions; reused second-stage e2e steps.
  Production files: `src/spec/retrieval.rs`. Test files:
  `tests/steps/spec_progress_output_steps.rs`.
  Targeted output: `1 scenario (1 passed)`, `8 steps (8 passed)`.
- [x] REFACTOR: clippy `-D warnings` clean, `cargo fmt`; reran the named
  scenario: `1 scenario (1 passed)`, `8 steps (8 passed)`.
- [x] COMMIT: commit RED/GREEN/REFACTOR with the scenario title.
  Hash: `8c5be3b`

## Scenario: Advisory review announces second-stage model acquisition during the run (@e2e, @nli-combined)

- [x] RED: removed only this scenario's `@wip`; ran the named scenario via
  the e2e runner with the previous commit's step stubs. Output:
  `1 scenario (1 failed)`, `1 step (1 failed)`,
  `Step panicked. Captured output: not implemented`, non-zero exit.
- [x] GREEN: completed-index-with-cold-second-stage-cache fixture and the
  review-with-acquisition e2e when-step; advisory review emits BGE/NLI
  acquisition notices and `BGE status:` evidence on stdout.
  Production files: none (shared notices from previous scenarios). Test
  files: `tests/steps/spec_progress_output_e2e_steps.rs`,
  `tests/steps/spec_progress_output_steps.rs`.
  Targeted output: `1 scenario (1 passed)`, `8 steps (8 passed)`.
- [x] REFACTOR: clippy `-D warnings` clean, `cargo fmt`; reran the named
  scenario: `1 scenario (1 passed)`, `8 steps (8 passed)`.
- [x] COMMIT: commit RED/GREEN/REFACTOR with the scenario title.
  Hash: `bab8d25`

## Final verification

- [x] Full non-e2e suite: `GIVN_FEATURES=givn/changes/spec-progress-output/specs ./run-tests.sh`
  plus the default `./run-tests.sh` (permanent specs unaffected).
  Output: delta `6 scenarios (6 passed)`; permanent
  `349 + 34 + 9 + 8 + 6 scenarios, all passed`.
- [x] Full e2e suite: `GIVN_FEATURES=givn/changes/spec-progress-output/specs ./run-tests.sh --e2e`
  plus the default `./run-tests.sh --e2e`.
  Output: delta `2 + 2 scenarios, all passed`; permanent
  `53 + 7 + 4 + 2 scenarios, all passed`.
- [x] `cargo fmt --all && cargo clippy --workspace --all-targets -- -D warnings`.
  Output: `Finished dev profile` with no warnings; no unimplemented step
  stubs remain in the two capability step files (grep-verified).
