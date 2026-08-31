# Tasks: unified-semantic-rulebook

Scenario-by-scenario RED→GREEN→REFACTOR→COMMIT per the three delta feature
files. Rulebook mechanics first (non-@e2e), then the @e2e consumer flows.

## Setup: runner, strict mode, step skeletons, @rulebook group

- [x] Add the `@rulebook` group to `run-tests.sh` (`BGE=stub`,
      `RERANKER=nli-stub`, full features, `run_review_output` config) in
      `--e2e`, `--non-e2e`, and the default case; exclude `@rulebook` from
      the `run_e5`/default tag filters.
      Files: `run-tests.sh`.
- [x] Step definitions for every new step text live in
      `tests/steps/semantic_rulebook_steps.rs` and
      `tests/steps/semantic_rulebook_e2e_steps.rs` (registered in
      `tests/steps/mod.rs`); each binding carries a real assertion, and the
      ADR register successor binding was added to
      `tests/steps/adr_qualification_gate_steps.rs` (ADR-065 is a reserved
      non-ADR id — the new ADR is ADR-066).
- [x] Strictness proof (behavioural RED — every step binding is real, the
      runner fails on a genuine assertion mismatch, exit 101):

  ```
  $ GIVN_FEATURES=givn/changes/unified-semantic-rulebook/specs \
    GIVN_SPEC_EMBEDDER=stub GIVN_SPEC_BGE=stub GIVN_SPEC_RERANKER=nli-stub \
    cargo test --no-default-features --features spec-retrieval,cross-encoder-reranker,nli-evidence \
    --test features_runner -- \
    --name "The neighbour count cuts per-scenario pairs before the pair cap"
  1 scenario (1 failed)
  4 steps (3 passed, 1 failed)
  error: test failed ... EXIT=101
  ```

- [x] e2e filter proof: `./run-tests.sh` → exit 101 with exactly one failing
      permanent scenario (the ADR register count text that this change's
      delta replaces until merge — verified: the delta successor passes,
      exit 0); `./run-tests.sh --e2e` → exit 0 (8 permanent @e2e scenarios,
      delta not in scope).

## Gate constants and fixtures (S1–S3: semantic-review delta)

- [x] S1 `Change review generates a complete report for a semantic candidate`
      (modified): new `single admitted candidate` fixture (disjoint corpus),
      parameterized four-decimals range binding, `BLOCKED` status.
      RED: undefined range step -> exit 101. GREEN: exit 0. Files:
      tests/steps/semantic_rulebook_steps.rs (fixtures + coverage commands),
      tests/steps/semantic_review_e2e_steps.rs (parameterized range binding),
      specs/semantic-review/semantic-review.feature. Commit 55fbbc9.
- [x] S2 `The report distinguishes permanent and current-change candidate
      scopes` (modified): score-boundary binding uses its parameter (0.9000);
      two-similar rulebook fixture gains deterministic BGE/NLI recommended
      overrides for all three pairs.
      RED: scope line missing under stub-filtered pairs -> exit 101.
      GREEN: exit 0. Files: tests/steps/semantic_review_steps.rs (boundary
      binding), tests/steps/semantic_rulebook_steps.rs (overrides).
      Commit for S2:
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
