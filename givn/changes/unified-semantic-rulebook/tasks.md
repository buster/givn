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
      Commit 54e50e9.
- [x] S3 `A candidate at the exact E5 boundary is included` (modified):
      boundary fixture to 0.9000, threshold constants unified to
      `E5_ADMISSION_THRESHOLD = 0.90` (delete `SEMANTIC_SIMILARITY_THRESHOLD`
      and `SIMILARITY_THRESHOLD`).
      RED: exit 0 instead of blocking under old 0.95. GREEN: exit 0 with
      score 0.9000 admitted. Files: src/spec/retrieval.rs (constants),
      tests/steps/semantic_review_steps.rs (boundary fixture). Commit 07472c9.

## Rulebook core (S4–S7: selection + cache)

- [x] S4 `The neighbour count cuts per-scenario pairs before the pair cap`:
      hybrid selection (5 neighbours/scenario, cap 20, deterministic tie
      order), unselected pairs are not candidates.
      RED (setup proof): count 7 vs expected 5, exit 101. GREEN: exit 0.
      Files: src/spec/retrieval.rs (NEIGHBOURS_PER_SCENARIO, PAIR_CAP,
      apply_hybrid_selection), tests/steps/semantic_rulebook_steps.rs
      (disjoint-corpus fixtures with structural shape-match dispositions).
      Commit 8f2f6ad.
- [x] S5 `Consumer review caps the judged pairs at the pair cap` (@e2e):
      25 admitted stub pairs → exactly 20 judged, no `*_UNSCORED`.
      RED: overlap dispositions missing (fixture families) + report reads.
      GREEN: exit 0. Files: tests/steps/semantic_rulebook_steps.rs,
      tests/steps/semantic_rulebook_e2e_steps.rs, feature file.
      Commit 8f2f6ad.
- [x] S6 `The pair cap fills deterministically when scores tie`: two runs,
      identical candidate ids and order. GREEN-first over S4's deterministic
      selection (reused steps only). Commit 2d67b66.
- [x] S7 Pair-score cache: `givn/.spec-pair-cache.json`, key
      (stage, model_id, endpoint files/titles/hashes), stub model-id
      pinning, per-pair accounting (reused iff every stage hit), stdout
      line + report counters.
      Files: src/spec/retrieval.rs (PairScoreCache, pair_cache_key,
      PairCacheAccounting, apply_semantic_bge/nli integration,
      render_semantic_report). Commit fb41ff9.
- [x] S8 `Consumer rerun reuses recorded pair judgements` (@e2e): 0/1 → 1/0.
      GREEN: exit 0. Files: src/spec/retrieval.rs, feature file.
      Commit fb41ff9.
- [x] S9 `A changed endpoint re-judges its recorded pair`: delta-side edit.
      RED: accounting mismatch (3-pair fixture) → fixed to single pair;
      then GREEN. Files: tests/steps/semantic_rulebook_steps.rs
      (rulebook change steps), feature file. Commit fb41ff9.
- [x] S10 `A changed permanent endpoint re-judges its recorded pair`:
      permanent-side edit. GREEN: exit 0. Commit fb41ff9.
- [x] S11 `A change without candidates reports empty cache accounting`:
      0/0, no stage runs. GREEN: exit 0. Commit fb41ff9.
- [x] S12 `Emergency budget truncation keeps outside candidates unscored
      and blocking`: budget below selected count → `*_UNSCORED`, blocking.
      RED: shared binding expected count 1. GREEN: dedicated binding asserts
      Status BLOCKED, count 20, UNSCORED markers. Files:
      tests/steps/semantic_rulebook_steps.rs, feature file. Commit 2ab3918.
- [x] S13 `Consumer review judges the selected pairs with both evidence
      models` (@e2e): advisory run, both stages enabled, per-stage counts.
      RED: singular/plural step text mismatch. GREEN: exit 0.
      Files: tests/steps/semantic_rulebook_e2e_steps.rs, feature file.
      Commit 2ab3918.

## Gate absorption + bindings (S14–S16)

- [x] S14 bge delta: remove `E5 candidates outside the bounded BGE set
      remain explicitly unscored and blocking`; the `@gate*` scenarios keep
      their semantics under the absorbed gate. Files:
      specs/bge-reranking-evidence/bge-reranking-evidence.feature (removed
      entry), src/spec/retrieval.rs (single E5_ADMISSION_THRESHOLD, audit
      keeps SEMANTIC_AUDIT_THRESHOLD 0.95). Commit 6154f7d.
- [x] S15 Remediation step bindings: 0.9400 → 0.8900, 0.9500 → 0.9000
      (below/at boundary); advisory output bindings 0.95 → 0.90 (text line
      and JSON threshold). Files: tests/steps/semantic_remediation_steps.rs,
      tests/steps/semantic_review_output_steps.rs. Commit 6154f7d.
- [x] S16 `Consumer review keeps a duplicate pair born inside the delta`
      (@e2e): delta↔delta scope candidate on the unrelated-corpus fixture.
      RED: missing commands.yaml (verify hook) → fixed fixture; GREEN.
      Files: tests/steps/semantic_rulebook_steps.rs, feature file.
      Commit 6154f7d.

## Consumers (S17–S18)

- [x] S17 `check review` runs ONE rulebook run (collapse
      `semantic_review_without_index_notice` + `review_without_index_notice`);
      advisory evidence stays available via `givn spec review`.
      Files: src/cli/check.rs, feature file spec-retrieval delta.
- [x] S18 `Consumer archive applies the rulebook to the current permanent
      tree` (@e2e): archive gate = rulebook (Archive surface), stdout
      accounting, no worklist write, carried classifications.
      RED: archive failed without cache accounting. GREEN: exit 0 with
      `pair cache:` line and retained report. Files: src/spec/retrieval.rs
      (semantic_review_archive_gate), src/cli/archive.rs,
      tests/steps/semantic_rulebook_e2e_steps.rs.

## Standing-suite reconciliation (S19)

- [x] Fixtures and standing scenarios that relied on 0.95 raw-cosine
      admissions or the old advisory pool are reconciled: ownership fixture
      vocabulary, disposition-gates fixture per-scenario vocabularies,
      audit keeps SEMANTIC_AUDIT_THRESHOLD 0.95, ADR register successor
      binding. Full `./run-tests.sh` → exit 101 with exactly the four
      expected until-merge failures (each replaced by this change's delta
      and green there); `./run-tests.sh --e2e` → exit 0 (94 scenarios);
      delta suite → 17/17 green.


## Commit evidence

One atomic commit per scenario batch, hashes unique:

- [x] COMMIT: 55fbbc9 -- S1 (semantic report, single-candidate fixture)
- [x] COMMIT: 54e50e9 -- S2 (scopes, boundary binding parameterized)
- [x] COMMIT: 07472c9 -- S3 (E5 boundary 0.9000, constants unified)
- [x] COMMIT: 8f2f6ad -- S4+S5 (hybrid selection + pair cap e2e)
- [x] COMMIT: 2d67b66 -- S6 (deterministic tie order)
- [x] COMMIT: fb41ff9 -- S7+S8+S9+S10+S11 (pair-score cache + reuse/invalidation/zero scenarios)
- [x] COMMIT: 2ab3918 -- S12+S13 (budget truncation + advisory two-stage)
- [x] COMMIT: 6154f7d -- S14+S15+S16 (gate absorption, remediation bindings, delta-to-delta e2e)
- [x] COMMIT: 0c9f106 -- S17+S18 (one rulebook run in check review, archive gate rulebook)

## Review

- [x] `givn check review --change unified-semantic-rulebook` passes
      (real models, no stubs); review.md written; REVIEW: PASS.
      Evidence: pair cache: 0 reused, 20 computed; report Status PASS,
      0 unresolved candidates; overlap dispositions 2/2 (KEEP-AS-BOUNDARY).
