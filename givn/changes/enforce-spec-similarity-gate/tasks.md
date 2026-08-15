# Tasks: enforce-spec-similarity-gate

## Setup

- [x] Confirm the existing Cucumber runner executes the change `.feature` files,
  uses `.fail_on_skipped()`, and has one step module per capability. Strict proof:
  `GIVN_FEATURES=givn/changes/enforce-spec-similarity-gate/specs cargo test --features spec-retrieval --test features_runner -- --name '^Temporary strict-mode proof$'`
  exited non-zero with `Step doesn't match any function`, followed by
  `1 scenario (1 failed)` and `1 step failed`. The temporary scenario was
  removed immediately after proof. The real targeted fixture then passed.
- [x] Confirm the no-feature build remains independent of retrieval:
  `cargo check --no-default-features --locked` passed. Both
  `cargo run --no-default-features -- spec index` and
  `cargo run --no-default-features -- spec review --change enforce-spec-similarity-gate`
  exited non-zero with `spec retrieval is unavailable in this build`.
- [ ] Add the strict production path and deterministic stub path without any
  project, CLI, or environment threshold override. Record the fixed constant and
  the failed attempted `retrieval.similarity_threshold: 1.00` override.
- [ ] Run the full permanent-tree baseline with the production passage-to-passage
  scoring algorithm. Record every score `>= 0.95`, remediate every candidate in
  this change, and record a final zero-candidate scan. No existing pair may be
  grandfathered.

## Scenario: Review blocks an unclassified permanent near-duplicate

- [ ] RED: Remove `@wip`; run the named scenario only. It must fail because the
  old retrieval path does not scan permanent pairs or return a similarity-gate
  error. Paste the command and non-zero output.
- [ ] GREEN: Implement all-pair permanent projected-tree scanning, candidate
  output, fixed threshold, and non-zero gate failure in `src/spec/retrieval.rs`.
  Add the real fixture in `tests/steps/enforce_spec_similarity_gate_steps.rs`.
  Run the named scenario and paste the passing output. Production files:
  `src/spec/retrieval.rs`, `tests/steps/enforce_spec_similarity_gate_steps.rs`.
- [ ] REFACTOR: Re-run the named scenario after deterministic ordering and output
  cleanup. Paste the passing output.
- [ ] COMMIT: Commit RED/GREEN/REFACTOR with message referencing
  `Review blocks an unclassified permanent near-duplicate`. Hash: __________.

## Scenario: Review blocks a same-feature near-duplicate

- [ ] RED: Run only the named scenario and capture the non-zero pending/old-path
  failure.
- [ ] GREEN: Ensure the permanent pair scan does not filter same-file pairs and
  reports the same hard failure. Run the named scenario and paste output.
- [ ] REFACTOR: Re-run the named scenario after sharing the pair canonicalisation
  helper. Paste output.
- [ ] COMMIT: Commit with message referencing `Review blocks a same-feature
  near-duplicate`. Hash: __________.

## Scenario: Review reports every permanent candidate beyond the top five

- [ ] RED: Run only the named scenario and capture failure showing the old top-five
  or advisory behavior cannot report `Candidate six`.
- [ ] GREEN: Remove the top-five limit from the strict gate while retaining it for
  interactive `givn spec search`; assert all candidate pairs are emitted once.
  Run the named scenario and paste output.
- [ ] REFACTOR: Re-run after sorting endpoint identities and merging duplicate
  scope reports. Paste output.
- [ ] COMMIT: Commit with message referencing `Review reports every permanent
  candidate beyond the top five`. Hash: __________.

## Scenario: Review does not compare a modified scenario with its own predecessor

- [ ] RED: Run only the named scenario and capture the self-collision failure.
- [ ] GREEN: Implement capability-plus-title logical target identity, exclude only
  the modified scenario's own predecessor, and apply modified projection. Run
  the named scenario and paste output.
- [ ] REFACTOR: Re-run after covering title-changing rewrites as removed-plus-added
  operations. Paste output.
- [ ] COMMIT: Commit with message referencing `Review does not compare a modified
  scenario with its own predecessor`. Hash: __________.

## Scenario: Review blocks an unclassified current scenario against the permanent specifications

- [ ] RED: Run only the named scenario and capture non-zero pending/old-path
  failure.
- [ ] GREEN: Embed current added/modified delta scenarios with the passage role,
  compare them against every pre-change permanent entry, and fail above the
  fixed threshold. Run the named scenario and paste output.
- [ ] REFACTOR: Re-run after excluding only the modified scenario's predecessor
  and preserving all other baseline comparisons. Paste output.
- [ ] COMMIT: Commit with message referencing `Review blocks an unclassified
  current scenario against the permanent specifications`. Hash: __________.

## Scenario: Review rejects a boundary scenario that remains too similar

- [ ] RED: Run only the named scenario and capture failure before remediation
  guidance exists.
- [ ] GREEN: Print `KEEP-AS-BOUNDARY` as rewrite guidance while keeping the score
  as a hard failure. Run the named scenario and paste output.
- [ ] REFACTOR: Re-run after making stdout candidate detail and stderr gate error
  contracts deterministic. Paste output.
- [ ] COMMIT: Commit with message referencing `Review rejects a boundary scenario
  that remains too similar`. Hash: __________.

## Scenario: Review passes after a valid boundary scenario is rewritten

- [ ] RED: Run only the named scenario and capture failure with the unrevised
  boundary wording.
- [ ] GREEN: Ensure the projected rewritten title/steps produce no pair at or
  above `0.95`. Run the named scenario and paste output.
- [ ] REFACTOR: Re-run after keeping the fixed threshold private and removing any
  accidental config lookup. Paste output.
- [ ] COMMIT: Commit with message referencing `Review passes after a valid
  boundary scenario is rewritten`. Hash: __________.

## Scenario: Review passes after a duplicate is removed from the projected permanent specifications

- [ ] RED: Run only the named scenario and capture failure because the old path
  audits only the current permanent tree or ignores the removal projection.
- [ ] GREEN: Apply `@givn.removed` by capability tag and title in memory before
  the projected pair scan. Run the named scenario and paste output.
- [ ] REFACTOR: Re-run after matching archive's sorted multi-file operation order.
  Paste output.
- [ ] COMMIT: Commit with message referencing `Review passes after a duplicate is
  removed from the projected permanent specifications`. Hash: __________.

## Scenario: Review cannot raise the fixed similarity threshold through project configuration

- [ ] RED: Run only the named scenario and capture failure proving the old or
  hypothetical configurable path can be weakened.
- [ ] GREEN: Keep `SIMILARITY_THRESHOLD = 0.95` private in the binary and ignore
  the attempted project value. Run the named scenario and paste output.
- [ ] REFACTOR: Re-run after checking no CLI/env/config override is read. Paste
  output and `cargo check --no-default-features --locked` evidence.
- [ ] COMMIT: Commit with message referencing `Review cannot raise the fixed
  similarity threshold through project configuration`. Hash: __________.

## Scenario: Accepted archive refreshes the permanent similarity index

- [ ] RED: Run only the named E2E scenario and capture failure because archive
  does not yet run the strict pre-merge gate or refresh the index.
- [ ] GREEN: Rerun strict similarity before permanent writes, move the accepted
  change, then perform a feature-gated best-effort index refresh and assert the
  merged title through the CLI-driven fixture. Paste E2E output. Production
  files: `src/cli/archive.rs`, `src/cli/check.rs`, `src/spec/retrieval.rs`.
- [ ] REFACTOR: Re-run the E2E scenario after verifying refresh occurs only after
  post-merge gates and archive move. Paste output.
- [ ] COMMIT: Commit with message referencing `Accepted archive refreshes the
  permanent similarity index`. Hash: __________.

## Scenario: Review reports an advisory retrieval candidate

- [ ] RED: Run only the modified retrieval scenario and capture the old advisory
  or undefined behavior failure.
- [ ] GREEN: Update the existing retrieval fixture to use a genuine high-score
  scenario pair and assert the new hard-failure/remediation output. Paste output.
  Production/test files: `src/spec/retrieval.rs`,
  `tests/steps/spec_retrieval_steps.rs`.
- [ ] REFACTOR: Re-run after removing stale advisory wording from the observable
  assertions where the existing title is retained for modification semantics.
  Paste output.
- [ ] COMMIT: Commit with message referencing `Review reports an advisory
  retrieval candidate`. Hash: __________.

## Scenario: Review suppresses a disposed retrieval candidate

- [ ] RED: Run only the modified scenario and capture the failure proving the old
  disposition suppression behavior contradicts the strict gate.
- [ ] GREEN: Keep the candidate visible and failing even when the review note
  says `KEEP-AS-BOUNDARY`. Paste output.
- [ ] REFACTOR: Re-run after confirming review notes remain informational only.
  Paste output.
- [ ] COMMIT: Commit with message referencing `Review suppresses a disposed
  retrieval candidate`. Hash: __________.

## Scenario: Review gate keeps retrieval advisory

- [ ] RED: Run only the modified `givn check review` scenario and capture the old
  advisory-only exit behavior.
- [ ] GREEN: Propagate the feature-enabled strict retrieval failure through
  `src/cli/check.rs`; keep the no-feature path unchanged. Paste output.
- [ ] REFACTOR: Re-run after confirming overlap/integrity failures remain ordered
  before retrieval output. Paste output.
- [ ] COMMIT: Commit with message referencing `Review gate keeps retrieval
  advisory`. Hash: __________.

## E2E Gate Proof

- [ ] Run `GIVN_FEATURES=givn/changes/enforce-spec-similarity-gate/specs ./run-tests.sh --non-e2e` and record the scenario/step count and exit 0.
- [ ] Run `GIVN_FEATURES=givn/changes/enforce-spec-similarity-gate/specs ./run-tests.sh --e2e` and record a strictly smaller scenario count and exit 0.
- [ ] Run `GIVN_FEATURES=givn/changes/enforce-spec-similarity-gate/specs ./run-tests.sh` and record the complete scenario/step count and exit 0.
- [ ] Run `cargo clippy --workspace --all-targets --features spec-retrieval -- -D warnings` and record the result.
