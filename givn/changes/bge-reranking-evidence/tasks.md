# Tasks: bge-reranking-evidence

RED -> GREEN -> REFACTOR -> COMMIT per scenario. Non-`@e2e` scenarios run
before the six primary CLI smoke scenarios. Every checked item requires pasted
runner evidence and an atomic commit hash.

## Strict-mode proof

- [x] **Setup**: registered the BGE regular and E2E step modules and proved
  strict mode with the required non-zero output.
  ```text
  GIVN_FEATURES=givn/changes/bge-reranking-evidence/specs GIVN_SPEC_EMBEDDER=stub GIVN_SPEC_BGE=stub cargo test --features spec-retrieval,cross-encoder-reranker --test features_runner -- --name "E5-only exploration remains the default when second-stage evidence is disabled"
  1 feature, 1 scenario, 1 failed step; process exited non-zero because the
  registered step panicked with `not implemented: BGE setup step is implemented scenario by scenario`.
  ```
  The existing runner uses `.fail_on_skipped()` and the project verify hooks
  remain `./run-tests.sh` and `./run-tests.sh --e2e`.

## Non-E2E Scenarios

The following sections use the exact scenario title as the task identity. The
targeted runner command is the design command with `--name <scenario title>`.

### E5-only exploration remains the default when second-stage evidence is disabled
- [x] RED: removed only this scenario's `@wip`; the targeted run failed in the registered `unimplemented!()` setup step (non-zero).
- [x] GREEN: implemented the temporary CLI fixture, disabled-BGE environment, E5 result assertion, and E5-only/no-BGE output assertions in `tests/steps/bge_reranking_evidence_steps.rs`; targeted run passed 8/8 steps.
- [x] REFACTOR: reran the targeted command unchanged; 1 scenario and 8 steps passed.
- [x] COMMIT: `8ad8db7` — `feat(bge-reranking-evidence): E5-only exploration remains the default when second-stage evidence is disabled`

### E5-only JSON omits opt-in BGE evidence fields
- [x] RED: after removing only this scenario's `@wip`, the targeted run failed on the undefined JSON exploration step (non-zero).
- [x] GREEN: added the JSON CLI invocation and assertions for `query`, `index`, `results`, and absent `evidence`/BGE fields; targeted run passed 6/6 steps.
- [x] REFACTOR: reran the same targeted command; 1 scenario and 6 steps passed.
- [x] COMMIT: `661ad63` — `feat(bge-reranking-evidence): E5-only JSON omits opt-in BGE evidence fields`

### Enabling second-stage evidence preserves the E5 raw candidate identities
- [x] RED: removed only this scenario's `@wip`; targeted run failed on the undefined disabled-exploration step (non-zero).
- [x] GREEN: added disabled/enabled CLI parity invocations, raw rank identity comparison, reduced recommendation assertion, and participation isolation; targeted run passed 7/7 steps.
- [x] REFACTOR: reran the same targeted command; 1 scenario and 7 steps passed.
- [x] COMMIT: `3e21ade` — `feat(bge-reranking-evidence): Enabling second-stage evidence preserves the E5 raw candidate identities`

### Both pair directions form the documented symmetric raw score
- [x] RED: removed only this scenario's `@wip`; targeted run failed because duplicate JSON had no BGE directional evidence.
- [x] GREEN: added the deterministic semantic-pair fixture, directional override validation, duplicate BGE scoring, and symmetric JSON assertions; targeted run passed 5/5 steps.
- [x] REFACTOR: reran the same targeted command; 1 scenario and 5 steps passed.
- [x] COMMIT: `0bda9d8` — `feat(bge-reranking-evidence): Both pair directions form the documented symmetric raw score`

### Deterministic exclusions happen before second-stage scoring
- [x] RED: removed only this scenario's `@wip`; targeted run first failed on the undefined enabled-evidence setup step.
- [x] GREEN: added exact/structural/semantic fixtures, deterministic exclusion assertions, and duplicate BGE accounting; targeted run passed 6/6 steps.
- [x] REFACTOR: reran the same targeted command; 1 scenario and 6 steps passed.
- [x] COMMIT: `a5e0877` — `feat(bge-reranking-evidence): Deterministic exclusions happen before second-stage scoring`

### A low-relevance E5 false positive is filtered without receiving a Givn classification
- [x] RED: removed only this scenario's `@wip`; targeted run failed on the undefined two-candidate fixture step.
- [x] GREEN: added the active-change advisory fixture and BGE advisory renderer with filtered/recommended non-classifying evidence; targeted run passed 7/7 steps.
- [x] REFACTOR: reran the same targeted command; 1 scenario and 7 steps passed.
- [x] COMMIT: `55d16be` — `feat(bge-reranking-evidence): A low-relevance E5 false positive is filtered without receiving a Givn classification`

### Filtered evidence does not block the blocking change review
- [x] RED: removed only this scenario's `@wip`; targeted run initially failed at the unresolved review-disposition fixture.
- [x] GREEN: added the blocking review fixture, BGE semantic-report admission fields, non-blocking filtered blocks, and PASS accounting; targeted run passed 6/6 steps.
- [x] REFACTOR: reran the same targeted command; 1 scenario and 6 steps passed.
- [x] COMMIT: `2ba3c6f` — `feat(bge-reranking-evidence): Filtered evidence does not block the blocking change review`

### The exact bounded pair limit refuses inference before model loading
- [x] RED: removed only this scenario's `@wip`; targeted run failed on the undefined exact-pool fixture.
- [x] GREEN: added the deterministic overflow pool seam, full pre-load refusal, unavailable text accounting, and E5 fallback assertions; targeted run passed 12/12 steps.
- [x] REFACTOR: reran the same targeted command; 1 scenario and 12 steps passed.
- [x] COMMIT: `523fc75` — `feat(bge-reranking-evidence): The exact bounded pair limit refuses inference before model loading`

### The exact bounded pair limit accepts 512 unique pairs
- [x] RED: removed only this scenario's `@wip`; targeted run failed on the undefined exact-512 fixture.
- [x] GREEN: added the exact-512 deterministic pool, synthetic bounded scoring, and 512/1024/no-overflow assertions; targeted run passed 6/6 steps.
- [x] REFACTOR: reran the same targeted command; 1 scenario and 6 steps passed.
- [x] COMMIT: `4ea55c1` — `feat(bge-reranking-evidence): The exact bounded pair limit accepts 512 unique pairs`

### BGE does not select NLI without the independent NLI capability
- [x] RED: removed only this scenario's `@wip`; targeted run failed on the undefined NLI-isolation setup step.
- [x] GREEN: added explicit BGE/NLI selector fixtures and assertions proving BGE output without NLI evidence; targeted run passed 8/8 steps.
- [x] REFACTOR: reran the same targeted command; 1 scenario and 8 steps passed.
- [x] COMMIT: `<pending until commit>` — `feat(bge-reranking-evidence): BGE does not select NLI without the independent NLI capability`

### The independent NLI capability owns the legacy diagnostic selector
- [ ] RED / GREEN / REFACTOR / COMMIT evidence and hash for this exact scenario; run only in the explicit NLI-capable feature matrix.

### BGE JSON preserves raw ordering and names every admission count
- [ ] RED / GREEN / REFACTOR / COMMIT evidence and hash for this exact scenario.

### E5 candidates outside the bounded BGE set remain explicitly unscored and blocking
- [ ] RED / GREEN / REFACTOR / COMMIT evidence and hash for this exact scenario.

### Duplicate JSON exposes partial unscored candidates
- [ ] RED / GREEN / REFACTOR / COMMIT evidence and hash for this exact scenario.

### The independent E5 gate remains separate from BGE evidence
- [ ] RED / GREEN / REFACTOR / COMMIT evidence and hash for this exact scenario.

### The E5 similarity gate remains available without BGE
- [ ] RED / GREEN / REFACTOR / COMMIT evidence and hash for this exact scenario.

### A same-file candidate remains eligible after identity exclusions
- [ ] RED / GREEN / REFACTOR / COMMIT evidence and hash for this exact scenario.

### The admission operating point includes a symmetric score of `3.0000`
- [ ] RED / GREEN / REFACTOR / COMMIT evidence and hash for this exact scenario.

### Missing second-stage model artifacts are explicitly unavailable
- [ ] RED / GREEN / REFACTOR / COMMIT evidence and hash for this exact scenario.

### A corrupt local BGE cache is rejected before model construction
- [ ] RED / GREEN / REFACTOR / COMMIT evidence and hash for this exact scenario.

### Unavailable BGE JSON preserves raw candidates and names unscored pairs
- [ ] RED / GREEN / REFACTOR / COMMIT evidence and hash for this exact scenario.

### Unavailable BGE duplicate JSON preserves the E5 fallback accounting
- [ ] RED / GREEN / REFACTOR / COMMIT evidence and hash for this exact scenario.

### Unavailable BGE blocking review keeps E5 candidates blocking
- [ ] RED / GREEN / REFACTOR / COMMIT evidence and hash for this exact scenario.

### Pair-cap unavailable blocking review retains E5 candidates
- [ ] RED / GREEN / REFACTOR / COMMIT evidence and hash for this exact scenario.

### Unavailable BGE change exploration retains the E5 raw fallback
- [ ] RED / GREEN / REFACTOR / COMMIT evidence and hash for this exact scenario.

### Unavailable BGE advisory review remains advisory
- [ ] RED / GREEN / REFACTOR / COMMIT evidence and hash for this exact scenario.

### Failed benchmark approval keeps advisory review on E5-only evidence
- [ ] RED / GREEN / REFACTOR / COMMIT evidence and hash for this exact scenario.

### Failed benchmark approval keeps permanent exploration on E5-only evidence
- [ ] RED / GREEN / REFACTOR / COMMIT evidence and hash for this exact scenario.

### Failed benchmark approval keeps change-aware exploration on E5-only evidence
- [ ] RED / GREEN / REFACTOR / COMMIT evidence and hash for this exact scenario.

### Failed benchmark approval keeps blocking review on E5-only evidence
- [ ] RED / GREEN / REFACTOR / COMMIT evidence and hash for this exact scenario.

### Failed benchmark approval keeps duplicate JSON on E5-only evidence
- [ ] RED / GREEN / REFACTOR / COMMIT evidence and hash for this exact scenario.

### E5-only blocking review preserves the semantic report contract
- [ ] RED / GREEN / REFACTOR / COMMIT evidence and hash for this exact scenario.

### E5-only duplicate JSON preserves schema version 1
- [ ] RED / GREEN / REFACTOR / COMMIT evidence and hash for this exact scenario.

### A malformed directional evidence seam is rejected before scoring
- [ ] RED / GREEN / REFACTOR / COMMIT evidence and hash for this exact scenario.

### A complete local BGE artifact loads without a downloader
- [ ] RED / GREEN / REFACTOR / COMMIT evidence and hash for this exact scenario.

### Approval writer requires enforced Rust parity and complete artifacts
- [ ] RED / GREEN / REFACTOR / COMMIT evidence and hash for this exact scenario.

### Approval writer rejects a missing Rust parity attestation
- [ ] RED / GREEN / REFACTOR / COMMIT evidence and hash for this exact scenario.

### Approval writer rejects an incomplete local BGE artifact set
- [ ] RED / GREEN / REFACTOR / COMMIT evidence and hash for this exact scenario.

### Approval writer rejects a forged parity document
- [ ] RED / GREEN / REFACTOR / COMMIT evidence and hash for this exact scenario.

### Approval writer requires enforcement for any approval output
- [ ] RED / GREEN / REFACTOR / COMMIT evidence and hash for this exact scenario.

### A binary without retrieval reports the explicit unavailable mode
- [ ] RED / GREEN / REFACTOR / COMMIT evidence and hash for this exact scenario.

### Repeated bounded scoring is deterministic and reports runtime separately
- [ ] RED / GREEN / REFACTOR / COMMIT evidence and hash for this exact scenario.

## E2E Smoke Tests

- [ ] Read `givn instructions specs --change bge-reranking-evidence`; configure
  `verify.e2e_command` as the strict `@e2e` subset and prove its scenario count
  is lower than the full change runner count.
  ```text
  <!-- commands and counts -->
  ```

### Permanent exploration exposes bounded second-stage evidence
- [ ] RED / GREEN / REFACTOR / COMMIT evidence and hash for this exact `@e2e` scenario.

### Change-aware exploration narrows the union of permanent and active neighbours
- [ ] RED / GREEN / REFACTOR / COMMIT evidence and hash for this exact `@e2e` scenario.

### Advisory change review presents second-stage evidence without classifying candidates
- [ ] RED / GREEN / REFACTOR / COMMIT evidence and hash for this exact `@e2e` scenario.

### Blocking change review records second-stage evidence in the semantic report
- [ ] RED / GREEN / REFACTOR / COMMIT evidence and hash for this exact `@e2e` scenario.

### Permanent duplicate audit uses a bounded endpoint-neighbour union
- [ ] RED / GREEN / REFACTOR / COMMIT evidence and hash for this exact `@e2e` scenario.

### Semantic authoring guidance prevents score-avoidance prose
- [ ] RED / GREEN / REFACTOR / COMMIT evidence and hash for this exact `@e2e` scenario.

---

**Done when:** every scenario box has RED/GREEN/REFACTOR evidence and its own
atomic commit hash; the non-E2E and strict `@e2e` runners both pass; and
`givn review`/archive gates report GREEN.
