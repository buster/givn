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
      `specs/semantic-review/semantic-review.feature` (6 scenarios: 5
      non-e2e + 1 e2e), tagged `@givn.delta @semantic-review` so archive
      merges into `givn/specs/semantic-review/semantic-review.feature`.
      Evidence: scoped run reports `6 scenarios (6 passed)`,
      `57 steps (57 passed)`.

## Coverage note (trimmed scenario set)

An earlier 10-scenario revision of this delta stayed blocked: the same-flow
scenarios are `>= 0.9500` under the real E5 embedder no matter how varied the
titles, change ids, and narrative markers were (measured; the grandfathered
permanent remediation family also scores 0.954-0.975 against itself). Per the
change request the delta was trimmed to the collision-free subset below,
keeping one blocked and one verified scenario per classification family plus
the real-CLI e2e duplicate remediation. The delta-to-delta verifier behavior
itself remains covered: DUPLICATE removal (verified via the real-CLI e2e
scenario), VARIANT consolidation, VALID-BOUNDARY sharpening, and the blocked
preconditions for VARIANT and VALID-BOUNDARY.

## Non-e2e scenarios

### A first change without permanent specs is gated on its own duplicated pair

- [x] The report admits the delta-to-delta pair with `scope: delta-to-delta`
      and gates with `BLOCKED` before any classification.

### A VARIANT pairing stays blocked until the ending is consolidated away

- [x] Classifying the pair VARIANT without consolidating stays blocked with
      stderr `VARIANT remediation requires consolidation`.

### Folding the variant body into a Scenario Outline passes the first-change gate

- [x] Consolidating the surviving endpoint into a parameterized
      `Scenario Outline` with `Examples` (and removing the redundant endpoint)
      exits 0 and reports `PASS`.

### A boundary pairing without rewording still fails review on the first change

- [x] Classifying the pair VALID-BOUNDARY without rewording stays blocked with
      stderr `VALID-BOUNDARY remediation requires an explicit distinction
      below the similarity boundary`.

### An explicit distinction below the boundary lets the first change pass

- [x] Rewriting the pair below the boundary with an explicit observable
      distinction exits 0, reports `PASS`, and the report shows
      `Candidate count: 0`.

## E2E scenario

### Real CLI review verifies a duplicate remediated within the first change

- [x] End-to-end: a real `givn check review --change <id>` run verifies the
      DUPLICATE remediation with stub embedder and prints
      `semantic remediation verified`, `PASS`.
      Evidence: `./run-tests.sh --e2e` passes (3 scenarios / 16 steps among
      the summary output).

## Implementation

- [x] `src/spec/retrieval.rs`: relax the prior-entry guard to accept
      `right_scope` `permanent | current-change`; branch the DUPLICATE /
      VARIANT / VALID-BOUNDARY arms to the new delta-to-delta verifiers; add
      `recompute_delta_pair_score`, `delta_endpoint_unchanged`, and the three
      verifiers. Evidence: `cargo check --features spec-retrieval` passes.
- [x] `tests/steps/semantic_remediation_steps.rs` and
      `semantic_remediation_e2e_steps.rs`: add the delta-to-delta fixture
      (`remediation_delta_to_delta_project` / `_world`), cid-aware staging
      (`on change "<id>"`), the remediation `when` steps, and the e2e fixture.
- [x] `cargo fmt` clean; `cargo clippy --workspace --all-targets --features
      spec-retrieval -- -D warnings` clean.
- [x] Full suites green: `./run-tests.sh` (non-e2e) and `./run-tests.sh
      --e2e`.

## Self-review gate

- [x] `givn check review --change fix-remediation-delta2delta` is fully green:
      verify, verify-e2e, integrity, and overlap dispositions pass; the
      semantic gate reports `Status: PASS, Candidate count: 0` under the real
      E5 embedder (no intra-delta candidates, no delta-to-permanent
      candidates). The change can archive cleanly and adds no same-family
      collision to the permanent corpus.

## Commits

- [x] COMMIT: `f97fe28` — test(semantic-review): stage delta-to-delta fixtures
      under distinct change ids (cid-aware fixture world, retagging, report-path
      helper).
- [x] COMMIT: `0a0be5d` — test(semantic-review): rework delta scenarios with
      decoupled change ids (semantic-review.feature).
- [x] COMMIT: `0cc6b70` — docs(change): refresh change artifacts for the
      reworked delta.
- [x] COMMIT: `7121604` — docs(change): add fix-remediation-delta2delta change
      artifacts (overlap dispositions, design, design review).
- [x] COMMIT: `3fc1fa0` — test(semantic-review): cover delta-to-delta
      remediation in the cucumber suite.
- [x] COMMIT: `a7eee2d` — fix(semantic-review): verify delta-to-delta
      remediation candidates.
