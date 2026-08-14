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
- [x] Apply the explicitly authorized dependency-age override while resolving
  the latest compatible lockfile with `cargo update`. The crates.io API was
  rate-limited in this environment, so the verification command used the
  explicit override and lockfile-derived allowlist:
  `GIVN_DEP_AGE_ALLOWLIST=<all locked registry entries> MIN_AGE_DAYS=30
  scripts/check-dep-age.sh` → `checked: 0, skipped: 328, violations: 0`.
  The normal 30-day policy remains required for default/release dependency
  adoption; this optional retrieval tree is the documented exception.

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
- [x] COMMIT: `9a99d4e`.

## Scenario: Review suppresses a disposed retrieval candidate

- [x] RED: Left the new disposition step pending; the targeted runner failed
  with `Step panicked. Captured output: not implemented`.
- [x] GREEN: Added a real review.md disposition fixture and asserted through
  the CLI that the disposed permanent candidate is absent from stdout. The
  targeted scenario passed five steps.
- [x] REFACTOR: Re-ran the targeted scenario after narrowing the assertion to
  the disposed candidate; one scenario and five steps passed with exit 0.
- [x] COMMIT: `3034ce2`.

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
- [x] COMMIT: `2d407a7`.

## Scenario: Search returns a semantically related scenario

- [x] RED: Removed `@wip`; the targeted e2e runner failed at the query step
  with `Step panicked. Captured output: not implemented`.
- [x] GREEN: Implemented query embedding, cosine ranking, E5 role prefixes,
  and ranked CLI output. `cargo check --features spec-retrieval` passed; the
  targeted e2e scenario passed five steps with the real subprocess.
- [x] REFACTOR: Re-ran the targeted e2e scenario after cleanup; one scenario
  and five steps passed with exit 0.
- [x] COMMIT: `e536efe`.

## Scenario: Search does not return the running change's own delta

- [x] RED: Added the immutable `@e2e` tag and removed `@wip`; the targeted
  e2e runner failed at the pending active-delta fixture step with
  `Step panicked. Captured output: not implemented`.
- [x] GREEN: The permanent-only index scope excludes the active delta, and
  the real search subprocess did not print `Running change only`. The targeted
  e2e scenario passed five steps with exit 0.
- [x] REFACTOR: Re-ran the targeted e2e scenario after cleanup; one scenario
  and five steps passed with exit 0.
- [x] COMMIT: `6639bc0`.

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

- [x] Run `GIVN_FEATURES=givn/changes/spec-retrieval/specs ./run-tests.sh`:
  3 non-e2e scenarios, 14 steps, exit 0.
- [x] Run `GIVN_FEATURES=givn/changes/spec-retrieval/specs ./run-tests.sh
  --e2e`: 4 e2e scenarios, 19 steps, exit 0. The unfiltered full run reports
  7 scenarios, so the e2e subset is strictly smaller than the total.
- [x] Confirm both commands exit 0 after all seven scenarios are GREEN.
