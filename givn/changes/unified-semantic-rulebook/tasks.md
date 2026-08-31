# Tasks: unified-semantic-rulebook

Scenario-by-scenario RED→GREEN→REFACTOR→COMMIT per the three delta feature
files. Rulebook mechanics first (non-@e2e), then the @e2e consumer flows.

## Setup: runner, strict mode, step skeletons, @rulebook group

- [ ] Add the `@rulebook` group to `run-tests.sh` (`BGE=stub`,
      `RERANKER=nli-stub`, full features, `run_review_output` config) in
      `--e2e`, `--non-e2e`, and the default case; exclude `@rulebook` from
      the `run_e5`/default tag filters.
- [ ] Register `tests/steps/semantic_rulebook_steps.rs` and
      `tests/steps/semantic_rulebook_e2e_steps.rs` in `tests/steps/mod.rs`
      with `unimplemented!()` skeletons for every new step text.
- [ ] Strictness proof: one delta scenario fails with an unimplemented step
      (record exit code here).
- [ ] e2e filter proof: `./run-tests.sh` and `./run-tests.sh --e2e` both
      exit 0 with the delta's `@wip` scenarios skipped.

## Gate constants and fixtures (S1–S3: semantic-review delta)

- [ ] S1 `Change review generates a complete report for a semantic candidate`
      (modified): new `single admitted candidate` fixture (disjoint corpus),
      parameterized four-decimals range binding, `BLOCKED` status.
- [ ] S2 `The report distinguishes permanent and current-change candidate
      scopes` (modified): score-boundary binding uses its parameter (0.9000).
- [ ] S3 `A candidate at the exact E5 boundary is included` (modified):
      boundary fixture to 0.9000, threshold constants unified to
      `E5_ADMISSION_THRESHOLD = 0.90` (delete `SEMANTIC_SIMILARITY_THRESHOLD`
      and `SIMILARITY_THRESHOLD`).

## Rulebook core (S4–S7: selection + cache)

- [ ] S4 `The neighbour count cuts per-scenario pairs before the pair cap`:
      hybrid selection (5 neighbours/scenario, cap 20, deterministic tie
      order), unselected pairs are not candidates.
- [ ] S5 `Consumer review caps the judged pairs at the pair cap` (@e2e):
      25 admitted stub pairs → exactly 20 judged, no `*_UNSCORED`.
- [ ] S6 `The pair cap fills deterministically when scores tie`: two runs,
      identical candidate ids and order.
- [ ] S7 Pair-score cache: `givn/.spec-pair-cache.json`, key
      (stage, model_id, endpoint files/titles/hashes), stub model-id
      pinning, per-pair accounting (reused iff every stage hit), stdout
      line + report/JSON counters.
- [ ] S8 `Consumer rerun reuses recorded pair judgements` (@e2e): 0/1 → 1/0.
- [ ] S9 `A changed endpoint re-judges its recorded pair`: delta-side edit.
- [ ] S10 `A changed permanent endpoint re-judges its recorded pair`:
      permanent-side edit.
- [ ] S11 `A change without candidates reports empty cache accounting`:
      0/0, no stage runs.
- [ ] S12 `Emergency budget truncation keeps outside candidates unscored
      and blocking`: budget below selected count → `*_UNSCORED`, blocking.
- [ ] S13 `Consumer review judges the selected pairs with both evidence
      models` (@e2e): advisory run, both stages enabled, per-stage counts.

## Gate absorption + bindings (S14–S16)

- [ ] S14 bge delta: remove `E5 candidates outside the bounded BGE set
      remain explicitly unscored and blocking`; modify the two `@gate*`
      scenarios to the absorbed rulebook admission.
- [ ] S15 Remediation step bindings: 0.9400 → 0.8900, 0.9500 → 0.9000
      (below/at boundary); advisory output bindings 0.95 → 0.90 (text line
      and JSON threshold).
- [ ] S16 `Consumer review keeps a duplicate pair born inside the delta`
      (@e2e): delta↔delta scope candidate on the disjoint corpus.

## Consumers (S17–S18)

- [ ] S17 `check review` runs ONE rulebook run (collapse
      `semantic_review_without_index_notice` + `review_without_index_notice`);
      `givn spec review` renders advisory from the same run.
- [ ] S18 `Consumer archive applies the rulebook to the current permanent
      tree` (@e2e): archive gate = rulebook (Archive surface), stdout
      accounting, no worklist write, carried classifications.

## Standing-suite reconciliation (S19)

- [ ] Fixtures and standing scenarios that relied on 0.95 raw-cosine
      admissions or the old advisory pool are reconciled; full
      `./run-tests.sh` and `./run-tests.sh --e2e` exit 0.

## Review

- [ ] `givn check review --change unified-semantic-rulebook` passes
      (real models, no stubs); review.md written; REVIEW: PASS.
