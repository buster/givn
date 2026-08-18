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
- [x] COMMIT: `c18ec32` — `feat(bge-reranking-evidence): BGE does not select NLI without the independent NLI capability`

### The independent NLI capability owns the legacy diagnostic selector
- [ ] RED / GREEN / REFACTOR / COMMIT evidence and hash for this exact scenario; run only in the explicit NLI-capable feature matrix.

### BGE JSON preserves raw ordering and names every admission count
- [x] RED: removed only this scenario's `@wip`; targeted run failed on the undefined BGE JSON invocation.
- [x] GREEN: added additive Explore JSON evidence, raw-order preservation, and count/model/runtime assertions; targeted run passed 7/7 steps.
- [x] REFACTOR: reran the same targeted command; 1 scenario and 7 steps passed.
- [x] COMMIT: `f7c1c05` — `feat(bge-reranking-evidence): BGE JSON preserves raw ordering and names every admission count`

### E5 candidates outside the bounded BGE set remain explicitly unscored and blocking
- [x] RED: removed only this scenario's `@wip`; targeted run failed on the undefined raw-513/top-K-512 fixture.
- [x] GREEN: added successful partial-pool seams and semantic-report `BGE_UNSCORED` blocking rendering with null directionals; targeted run passed 11/11 steps.
- [x] REFACTOR: reran the same targeted command; 1 scenario and 11 steps passed.
- [x] COMMIT: `6e4e6c4` — `feat(bge-reranking-evidence): E5 candidates outside the bounded BGE set remain explicitly unscored and blocking`

### Duplicate JSON exposes partial unscored candidates
- [x] RED: removed only this scenario's `@wip`; targeted run failed on the undefined partial duplicate JSON invocation.
- [x] GREEN: added schema-v2 unscored collections, endpoint/E5/null-BGE fields, deterministic ordering, and partial-pool JSON assertions; targeted run passed 9/9 steps.
- [x] REFACTOR: reran the same targeted command; 1 scenario and 9 steps passed.
- [x] COMMIT: `8efa530` — `feat(bge-reranking-evidence): Duplicate JSON exposes partial unscored candidates`

### The independent E5 gate remains separate from BGE evidence
- [x] RED: removed only this scenario's `@wip`; targeted gate-feature run failed before the undefined combination fixture.
- [x] GREEN: made the E5 gate execute independently before BGE advisory rendering and asserted both outputs; targeted run passed 6/6 steps.
- [x] REFACTOR: reran the same targeted command with `spec-similarity-gate`; 1 scenario and 6 steps passed.
- [x] COMMIT: `babaa8a` — `feat(bge-reranking-evidence): The independent E5 gate remains separate from BGE evidence`

### The E5 similarity gate remains available without BGE
- [x] RED: removed only this scenario's `@wip`; targeted gate-only run failed on the undefined blocking-review invocation.
- [x] GREEN: added the gate-only CLI fixture and assertions proving the E5 gate remains visible without BGE; targeted run passed 4/4 steps.
- [x] REFACTOR: reran the same targeted command; 1 scenario and 4 steps passed.
- [x] COMMIT: `e799f31` — `feat(bge-reranking-evidence): The E5 similarity gate remains available without BGE`

### A same-file candidate remains eligible after identity exclusions
- [x] RED: removed only this scenario's `@wip`; targeted run failed on the undefined same-file fixture.
- [x] GREEN: changed active exploration exclusion from file-level to exact `(file,title)` identity and added projected/source fixture assertions; targeted run passed 5/5 steps.
- [x] REFACTOR: reran the same targeted command; 1 scenario and 5 steps passed.
- [x] COMMIT: `05aceb0` — `feat(bge-reranking-evidence): A same-file candidate remains eligible after identity exclusions`

### The admission operating point includes a symmetric score of `3.0000`
- [x] RED: removed only this scenario's `@wip`; targeted run failed on the undefined exact-threshold fixture.
- [x] GREEN: added inclusive threshold directional overrides and advisory recommendation/classification assertions; targeted run passed 4/4 steps.
- [x] REFACTOR: reran the same targeted command; 1 scenario and 4 steps passed.
- [x] COMMIT: `1453c06` — `feat(bge-reranking-evidence): The admission operating point includes a symmetric score of 3.0000`

### Missing second-stage model artifacts are explicitly unavailable
- [x] RED: removed only this scenario's `@wip`; targeted run failed on the undefined missing-artifact setup.
- [x] GREEN: added missing-artifact BGE mode, zero-load unavailable accounting, and Explore fallback assertions; targeted run passed 9/9 steps.
- [x] REFACTOR: reran the same targeted command; 1 scenario and 9 steps passed.
- [x] COMMIT: `b101c7e` — `feat(bge-reranking-evidence): Missing second-stage model artifacts are explicitly unavailable`

### A corrupt local BGE cache is rejected before model construction
- [x] RED: removed only this scenario's `@wip`; targeted run failed on the undefined corrupt-cache fixture.
- [x] GREEN: added local-artifact preflight before FastEmbed construction and corrupt-cache zero-load assertions; targeted run passed 8/8 steps.
- [x] REFACTOR: reran the same targeted command; 1 scenario and 8 steps passed.
- [x] COMMIT: `96a201f` — `feat(bge-reranking-evidence): A corrupt local BGE cache is rejected before model construction`

### Unavailable BGE JSON preserves raw candidates and names unscored pairs
- [x] RED: removed only this scenario's `@wip`; targeted run failed on the missing draft/JSON path.
- [x] GREEN: added additive unavailable Explore JSON status/reason/count rendering and E5 raw/unscored assertions; targeted run passed 10/10 steps.
- [x] REFACTOR: reran the same targeted command; 1 scenario and 10 steps passed.
- [x] COMMIT: `e8e5833` — `feat(bge-reranking-evidence): Unavailable BGE JSON preserves raw candidates and names unscored pairs`

### Unavailable BGE duplicate JSON preserves the E5 fallback accounting
- [x] RED: removed only this scenario's `@wip`; targeted run failed on the undefined unavailable duplicate fixture.
- [x] GREEN: added duplicate schema-v2 unavailable status/reason/count handling and E5 fallback assertions; targeted run passed 8/8 steps.
- [x] REFACTOR: reran the same targeted command; 1 scenario and 8 steps passed.
- [x] COMMIT: `<pending until commit>` — `feat(bge-reranking-evidence): Unavailable BGE duplicate JSON preserves the E5 fallback accounting`

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
