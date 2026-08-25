# Tasks: spec-retrieval

## Setup

- [x] Confirm the existing cucumber runner executes both `givn/specs/` and the
  change feature path, uses `.fail_on_skipped()`, and has a per-capability step
  module location. Proof command:
  `GIVN_FEATURES=givn/changes/spec-retrieval/specs cargo test --features spec-retrieval --test features_runner -- --name 'Index reports unavailable model setup'`.
  Proof output: non-zero exit; the runner reported `Step doesn't match any
  function` and then panicked with `1 step failed`.
- [x] Configure the retrieval feature and lockfile dependency resolution; the
  default build remains feature-free and `run-tests.sh` will use the opt-in
  feature for retrieval scenarios.
- [x] Verify the default distribution build remains feature-free:
  `cargo check --no-default-features --locked` passed, and
  `cargo run --no-default-features -- spec index` returned the explicit
  `spec retrieval is unavailable in this build` guidance without linking
  FastEmbed.
- [x] Verify the same default-build guidance for
  `cargo run --no-default-features -- spec review --change running-change`;
  it exits non-zero with the same explicit setup message.
- [x] Verify `GIVN_FEATURES=givn/changes/spec-retrieval/specs cargo run
  --no-default-features -- check review --change spec-retrieval` keeps the
  deterministic gate green and prints `retrieval advisory setup unavailable`.
- [x] Apply the explicitly authorized dependency-age override while resolving
  the latest compatible lockfile with `cargo update`. The crates.io API was
  rate-limited in this environment, so the verification command used the
  explicit override and lockfile-derived allowlist:
  `allowlist=$(perl -0ne 'while (/\[\[package\]\]\n(.*?)(?=\n\[\[package\]\]|\z)/sg) { $b=$1; ($n)=$b =~ /^name = "([^"]+)"/m; ($v)=$b =~ /^version = "([^"]+)"/m; print "$n\@$v " if $b =~ /^source = "registry\+/m }' Cargo.lock); GIVN_DEP_AGE_ALLOWLIST="$allowlist" MIN_AGE_DAYS=30 scripts/check-dep-age.sh` → `checked: 0, skipped: 328, violations: 0`.
  The user explicitly authorized the complete lockfile exception for this
  change; the normal 30-day policy remains required for later default/release
  dependency adoption.

## Scenario: Index reports unavailable model setup

- [x] RED: Removed `@wip`; the targeted runner matched the new step and failed
  with `Step panicked. Captured output: not implemented` (non-zero exit).
- [x] GREEN: Added the explicit missing-model setup error through the real
  subprocess in `src/spec/retrieval.rs`, `src/cli/spec.rs`, `src/main.rs`, and
  `tests/steps/spec_retrieval_steps.rs`. `cargo check --features
  spec-retrieval` passed; the targeted scenario passed with five steps.
- [x] REFACTOR: Re-ran the targeted command after cleanup; one scenario and
  five steps passed with exit 0.
- [x] COMMIT: `81df4f6`.

## Scenario: Reindex reuses unchanged scenario vectors

- [x] RED: Removed `@wip`; the targeted runner passed setup and failed at the
  pending rebuild step with `Step panicked. Captured output: not implemented`.
- [x] GREEN: The index implementation now discovers permanent scenarios,
  persists content hashes/vectors, and reports reuse. `cargo check
  --features spec-retrieval` and the targeted scenario passed (four steps).
- [x] REFACTOR: Re-ran the targeted scenario after cleanup; one scenario and
  four steps passed with exit 0.
- [x] COMMIT: `9a99d4e`; production refactor `4b3b203`.

## Scenario: Review suppresses a disposed retrieval candidate

- [x] RED: Left the new disposition step pending; the targeted runner failed
  with `Step panicked. Captured output: not implemented`.
- [x] GREEN: Added a real review.md disposition fixture and asserted through
  the CLI that the disposed permanent candidate is absent from stdout. The
  targeted scenario passed six steps.
- [x] REFACTOR: Re-ran the targeted scenario after narrowing the assertion to
  the disposed candidate; one scenario and six steps passed with exit 0.
- [x] COMMIT: `3034ce2`.

## Scenario: Review gate keeps retrieval advisory

- [x] RED: Left the active-delta fixture pending; the targeted runner failed
  with `Step panicked. Captured output: not implemented`.
- [x] GREEN: Configured a hermetic review-gate fixture with verify, e2e, and
  coverage commands, then asserted advisory stdout and exit 0 from the real
  `givn check review --change` subprocess. The targeted scenario passed five
  steps.
- [x] REFACTOR: Re-ran the targeted scenario after adding explicit coverage
  fixture configuration; one scenario and five steps passed with exit 0.
- [x] COMMIT: `9b6790f`.

## Scenario: Review ignores a removed delta scenario

- [x] RED: Added the removed-delta scenario with the step body pending; the
  targeted runner failed with `Step panicked. Captured output: not implemented`.
- [x] GREEN: Filtered `@givn.removed` scenarios from the review query set and
  asserted through the CLI that the removed title is absent. The targeted
  scenario passed five steps.
- [x] REFACTOR: Re-ran the targeted scenario after cleanup; one scenario and
  five steps passed with exit 0.
- [x] COMMIT: `1510aa0`.

## Scenario: Review rejects a missing change delta

- [x] RED: The scenario was first run against the unimplemented review path;
  the targeted subprocess did not provide the required missing-delta error.
- [x] GREEN: `resolve_change_dir` validation and the explicit missing-specs
  error now reject a change with no delta directory. The targeted scenario
  passed four steps.
- [x] REFACTOR: Re-ran the targeted missing-change scenario after cleanup;
  one scenario and four steps passed with exit 0.
- [x] COMMIT: `549b3aa`.

## Scenario: Review rejects an unsafe change identifier

- [x] RED: The targeted scenario failed before the path-validation behavior
  was implemented and could not satisfy the kebab-case error assertion.
- [x] GREEN: The review path now uses shared kebab-case change-directory
  validation, and the targeted scenario passed four steps.
- [x] REFACTOR: Re-ran the targeted unsafe-id scenario after cleanup; one
  scenario and four steps passed with exit 0.
- [x] COMMIT: `c13e711`, `d06b17b`.

## Scenario: Local index reports the permanent scenario inventory

- [x] RED: Removed `@wip`; the targeted e2e runner failed at the pending
  project-fixture step with `Step panicked. Captured output: not implemented`.
  The runner rejects combined `--tags` and `--name`, so the scenario was
  selected by `--name` and still exercised the real compiled CLI.
- [x] GREEN: Enabled the `spec-retrieval` feature and deterministic stub in
  `run-tests.sh`; the real `givn spec index` subprocess wrote the index and
  reported `Indexed 3 permanent scenarios`. The targeted e2e scenario passed
  four steps.
- [x] REFACTOR: Re-ran the targeted e2e scenario after cleanup; one scenario
  and four steps passed with exit 0.
- [x] COMMIT: `2d407a7`; production refactor `93037d5`.

## Scenario: Search returns a semantically related scenario

- [x] RED: Removed `@wip`; the targeted e2e runner failed at the query step
  with `Step panicked. Captured output: not implemented`.
- [x] GREEN: Implemented query embedding, cosine ranking, E5 role prefixes,
  and ranked CLI output. `cargo check --features spec-retrieval` passed; the
  targeted e2e scenario passed five steps with the real subprocess.
- [x] REFACTOR: Re-ran the targeted e2e scenario after cleanup; one scenario
  and five steps passed with exit 0.
- [x] COMMIT: `e536efe`; production refactors `435e6a0`, `171c1b9`.

## Scenario: Search does not return the running change's own delta

- [x] RED: Removed `@wip`; the targeted runner failed at the pending active-
  delta fixture step with
  `Step panicked. Captured output: not implemented`.
- [x] GREEN: The permanent-only index scope excludes the active delta, and
  the real search subprocess did not print `Running change only`. The targeted
  scenario passed five steps with exit 0.
- [x] REFACTOR: Re-ran the targeted scenario after cleanup; one scenario and
  five steps passed with exit 0.
- [x] COMMIT: `6639bc0`; production refactor `4b1b9dc`.

## Scenario: Review reports an advisory retrieval candidate

- [x] RED: Removed `@wip`; the targeted e2e runner failed at the shared active
  delta fixture step with `Step panicked. Captured output: not implemented`.
- [x] GREEN: Added `givn spec review --change`, refreshed the permanent index,
  queried active delta scenarios, respected disposition rows, and kept setup
  failures advisory. The targeted e2e scenario passed five steps through the
  real subprocess.
- [x] REFACTOR: Re-ran the targeted e2e scenario after cleanup; one scenario
  and five steps passed with exit 0.
- [x] COMMIT: `af1a7cb`.

## E2E gate proof

- [x] Run `GIVN_FEATURES=givn/changes/spec-retrieval/specs ./run-tests.sh
  --non-e2e`: 8 non-e2e scenarios, 38 steps, exit 0.
- [x] Run `GIVN_FEATURES=givn/changes/spec-retrieval/specs ./run-tests.sh
  --e2e`: 3 e2e scenarios, 14 steps, exit 0. The configured no-argument
  `./run-tests.sh` reports 11 scenarios, so the e2e subset is strictly smaller
  than the total.
- [x] Run `GIVN_FEATURES=givn/changes/spec-retrieval/specs ./run-tests.sh`:
  11 scenarios, 52 steps, exit 0. Confirm all commands exit 0 after all eleven
  scenarios are GREEN.
