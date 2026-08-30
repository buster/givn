# Tasks: fix-remediation-delta2delta

## Commit Policy

One atomic commit per logical slice (code fix / regression tests / change
artifacts). Each scenario below is covered by the regression suite; the
verification command scoped to this change's delta is:

```sh
GIVN_FEATURES=givn/changes/fix-remediation-delta2delta/specs \
  cargo test --no-default-features --features spec-retrieval --test features_runner
```

## Setup

- [x] Create change `fix-remediation-delta2delta`; write `proposal.md` (user
      perspective: delta-to-delta candidates can be classified but never
      verified; blocks the semantic gate for first changes in a fresh repo).
- [x] Write the delta spec at
      `specs/semantic-review/semantic-review.feature` (10 scenarios:
      9 non-e2e + 1 e2e), tagged `@givn.delta @semantic-review` so archive
      merges into `givn/specs/semantic-review/semantic-review.feature`.
      Evidence: spec file present; scoped run reports `10 scenarios (10
      passed)`, `89 steps (89 passed)`.

## Non-e2e scenarios

### An unclassified delta-to-delta pair blocks review with a current-change scope

- [x] RED/GREEN/REFACTOR: report admits the delta-to-delta pair, lists
      `scope: delta-to-delta`, and gates with `BLOCKED`. Evidence: scenario
      passes in the scoped run; report shows `Status: BLOCKED` pre-fix.

### A DUPLICATE delta-to-delta classification without removal remains blocked

- [x] Classifying the pair DUPLICATE without removing the redundant endpoint
      keeps the gate blocked with stderr
      `DUPLICATE remediation requires removing the redundant scenario`.

### Removing the redundant delta-to-delta scenario verifies duplicate remediation

- [x] Removing the redundant delta endpoint after classification exits 0,
      prints `semantic remediation verified`, and reports `PASS`.

### A delta-to-delta DUPLICATE pair reworded below the boundary verifies without removal

- [x] Rewriting the pair below the boundary (recomputed score `< 0.9500`)
      verifies even with both endpoints retained.

### A VARIANT delta-to-delta classification without consolidation remains blocked

- [x] Classifying the pair VARIANT without consolidating stays blocked with
      stderr `VARIANT remediation requires consolidation`.

### Consolidating a delta-to-delta variant into a Scenario Outline verifies remediation

- [x] Consolidating the surviving endpoint into a parameterized
      `Scenario Outline` with `Examples` (and removing the redundant endpoint)
      exits 0 and reports `PASS`.

### A VALID-BOUNDARY delta-to-delta classification without rewording remains blocked

- [x] Classifying the pair VALID-BOUNDARY without rewording stays blocked with
      stderr `VALID-BOUNDARY remediation requires an explicit distinction
      below the similarity boundary`.

### Sharpening a delta-to-delta boundary below the similarity boundary verifies remediation

- [x] Rewriting the pair below the boundary with an explicit observable
      distinction exits 0, reports `PASS`, and the report shows
      `Candidate count: 0`.

### A delta-to-delta VALID-BOUNDARY without an explicit distinction remains blocked

- [x] Rewording below the boundary without an explicit distinction keeps the
      gate blocked.

## E2E scenario

### Real CLI review verifies a delta-to-delta duplicate remediation

- [x] End-to-end: a real `givn check review --change running-change` run
      verifies the DUPLICATE remediation with stub embedder and prints
      `semantic remediation verified`, `PASS`.
      Evidence: `./run-tests.sh --e2e` passes (5 scenarios / 46 steps among the
      summary output).

## Implementation

- [x] `src/spec/retrieval.rs`: relax the prior-entry guard to accept
      `right_scope` `permanent | current-change`; branch the DUPLICATE /
      VARIANT / VALID-BOUNDARY arms to the new delta-to-delta verifiers; add
      `recompute_delta_pair_score`, `delta_endpoint_unchanged`, and the three
      verifiers. Evidence: `cargo check --features spec-retrieval` passes.
- [x] `tests/steps/semantic_remediation_steps.rs` and
      `semantic_remediation_e2e_steps.rs`: add the delta-to-delta fixture
      (`remediation_delta_to_delta_project`), the remediation `when` steps, and
      the e2e fixture.
- [x] `cargo fmt` clean; `cargo clippy --workspace --all-targets --features
      spec-retrieval -- -D warnings` clean.
- [x] Full suites green: `./run-tests.sh` (non-e2e) and `./run-tests.sh
      --e2e`.

## Self-review gate

- [x] `givn check review --change fix-remediation-delta2delta`: verify,
      verify-e2e, integrity, and overlap pass. The semantic gate is green:
      each scenario stages its fixture under a distinct change id
      (`rc-*`) and carries a distinct long narrative marker, so the scenario
      surfaces are E5-separable; a real-embedder run reports zero semantic
      candidates (no delta-to-delta, no delta-to-permanent). The decoupled
      change ids also keep the fixture's evidence hashes stable across the
      classify/action runs (the retag no longer mutates `feature_tags`).

## Commits

- [x] COMMIT: `f97fe28` — test(semantic-review): stage delta-to-delta fixtures
      under distinct change ids (cid-aware fixture world, retagging, report-path
      helper).
- [x] COMMIT: `0a0be5d` — test(semantic-review): rework delta scenarios with
      decoupled change ids (semantic-review.feature).
- [x] COMMIT: `7121604` — docs(change): add fix-remediation-delta2delta change
      artifacts (overlap dispositions, design, design review).
- [x] COMMIT: `3fc1fa0` — test(semantic-review): cover delta-to-delta
      remediation in the cucumber suite.
- [x] COMMIT: `a7eee2d` — fix(semantic-review): verify delta-to-delta
      remediation candidates.
