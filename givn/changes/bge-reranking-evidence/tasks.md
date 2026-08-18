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
- [x] RED: removed only this scenario's `@wip`; the NLI-capable targeted run failed on the undefined capability step (non-zero).
  ```text
  GIVN_FEATURES=givn/changes/bge-reranking-evidence/specs GIVN_SPEC_EMBEDDER=stub GIVN_SPEC_BGE=disabled GIVN_SPEC_RERANKER=nli-stub cargo test --features spec-retrieval,nli-evidence --test features_runner -- --name "The independent NLI capability owns the legacy diagnostic selector"
  1 scenario (1 failed, 1 passed step, 1 undefined step); process exited non-zero.
  ```
- [x] GREEN: added the NLI-capable CLI fixture and assertions for diagnostic-only output, absent BGE admission, and unchanged E5 identities; the explicit NLI matrix passed 7/7 steps.
  ```text
  1 feature
  1 scenario (1 passed)
  7 steps (7 passed)
  ```
- [x] REFACTOR: reran the same explicit NLI matrix command unchanged; 1 scenario and 7 steps passed.
- [x] COMMIT: `57592dd` — `feat(bge-reranking-evidence): The independent NLI capability owns the legacy diagnostic selector`

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
- [x] COMMIT: `afde262` — `feat(bge-reranking-evidence): Unavailable BGE duplicate JSON preserves the E5 fallback accounting`

### Unavailable BGE blocking review keeps E5 candidates blocking
- [x] RED: removed only this scenario's `@wip`; targeted run failed on the undefined unresolved-blocking fixture.
- [x] GREEN: added unavailable semantic-report rendering, normal E5 fallback blocks, zero-load accounting, and non-zero unresolved assertions; targeted run passed 9/9 steps.
- [x] REFACTOR: reran the same targeted command; 1 scenario and 9 steps passed.
- [x] COMMIT: `f898cb2` — `feat(bge-reranking-evidence): Unavailable BGE blocking review keeps E5 candidates blocking`

### Pair-cap unavailable blocking review retains E5 candidates
- [x] RED: removed only this scenario's `@wip`; targeted run failed on the undefined 513-pair blocking fixture.
- [x] GREEN: added pair-cap semantic-report fallback and unresolved E5 block assertions; targeted run passed 7/7 steps.
- [x] REFACTOR: reran the same targeted command; 1 scenario and 7 steps passed.
- [x] COMMIT: `aefd07b` — `feat(bge-reranking-evidence): Pair-cap unavailable blocking review retains E5 candidates`

### Unavailable BGE change exploration retains the E5 raw fallback
- [x] RED: removed only this scenario's `@wip`; targeted run failed on the undefined permanent/active fallback fixture.
- [x] GREEN: added request-local current-change unavailable Explore fallback and source-scope assertions; targeted run passed 7/7 steps.
- [x] REFACTOR: reran the same targeted command; 1 scenario and 7 steps passed.
- [x] COMMIT: `7b0f781` — `feat(bge-reranking-evidence): Unavailable BGE change exploration retains the E5 raw fallback`

### Unavailable BGE advisory review remains advisory
- [x] RED: removed only this scenario's `@wip`; targeted run failed on the undefined active-advisory fixture.
- [x] GREEN: added missing-artifact advisory dispatch/output and non-classifying fallback assertions; targeted run passed 6/6 steps.
- [x] REFACTOR: reran the same targeted command; 1 scenario and 6 steps passed.
- [x] COMMIT: `65338f4` — `feat(bge-reranking-evidence): Unavailable BGE advisory review remains advisory`

### Failed benchmark approval keeps advisory review on E5-only evidence
- [x] RED: removed only this scenario's `@wip`; targeted run initially showed BGE participation because approval failure was not propagated through the advisory fixture.
- [x] GREEN: enforced failed-approval E5-only dispatch and added no-filtered-output assertions; targeted run passed 7/7 steps.
- [x] REFACTOR: reran the same targeted command; 1 scenario and 7 steps passed.
- [x] COMMIT: `abff45f` — `feat(bge-reranking-evidence): Failed benchmark approval keeps advisory review on E5-only evidence`

### Failed benchmark approval keeps permanent exploration on E5-only evidence
- [x] RED: removed only this scenario's `@wip`; targeted run failed on the missing E5-only approval path.
- [x] GREEN: preserved Explore's E5-only output when approval fails; targeted run passed 7/7 steps.
- [x] REFACTOR: reran the same targeted command; 1 scenario and 7 steps passed.
- [x] COMMIT: `92869d7` — `feat(bge-reranking-evidence): Failed benchmark approval keeps permanent exploration on E5-only evidence`

### Failed benchmark approval keeps change-aware exploration on E5-only evidence
- [x] RED: removed only this scenario's `@wip`; targeted run failed on the missing change-aware approval path.
- [x] GREEN: preserved change-aware E5-only output after failed approval and asserted both source scopes remain visible; targeted run passed 7/7 steps.
- [x] REFACTOR: reran the same targeted command; 1 scenario and 7 steps passed.
- [x] COMMIT: `350ca8c` — `feat(bge-reranking-evidence): Failed benchmark approval keeps change-aware exploration on E5-only evidence`

### Failed benchmark approval keeps blocking review on E5-only evidence
- [x] RED: removed only this scenario's `@wip`; targeted run failed before approval-failure dispatch was wired.
- [x] GREEN: kept semantic blocking/report schema v1 with no BGE fields when approval fails; targeted run passed 6/6 steps.
- [x] REFACTOR: reran the same targeted command; 1 scenario and 6 steps passed.
- [x] COMMIT: `980b5e1` — `feat(bge-reranking-evidence): Failed benchmark approval keeps blocking review on E5-only evidence`

### Failed benchmark approval keeps duplicate JSON on E5-only evidence
- [x] RED: removed only this scenario's `@wip`; targeted run failed on the undefined semantic duplicate fixture.
- [x] GREEN: kept duplicate JSON schema v1 and omitted all BGE collections after approval failure; targeted run passed 5/5 steps.
- [x] REFACTOR: reran the same targeted command; 1 scenario and 5 steps passed.
- [x] COMMIT: `2c7cb4c` — `feat(bge-reranking-evidence): Failed benchmark approval keeps duplicate JSON on E5-only evidence`

### E5-only blocking review preserves the semantic report contract
- [x] RED: removed only this scenario's `@wip`; targeted E5-only run failed on the undefined disabled-review setup.
- [x] GREEN: preserved semantic report schema v1 and unresolved E5 blocking behavior without BGE fields; targeted run passed 6/6 steps.
- [x] REFACTOR: reran the same targeted command; 1 scenario and 6 steps passed.
- [x] COMMIT: `d7d5380` — `feat(bge-reranking-evidence): E5-only blocking review preserves the semantic report contract`

### E5-only duplicate JSON preserves schema version 1
- [x] RED: removed only this scenario's `@wip`; targeted E5-only duplicate run initially had undefined change-specific assertions.
- [x] GREEN: asserted schema-v1 field absence against the real duplicate CLI; targeted run passed 5/5 steps.
- [x] REFACTOR: reran the same targeted command; 1 scenario and 5 steps passed.
- [x] COMMIT: `8705cb7` — `feat(bge-reranking-evidence): E5-only duplicate JSON preserves schema version 1`

### A malformed directional evidence seam is rejected before scoring
- [x] RED: removed only this scenario's `@wip`; targeted run failed on the undefined malformed-seam fixture.
- [x] GREEN: added strict directional override parsing and zero-load error accounting; targeted run passed 5/5 steps.
- [x] REFACTOR: reran the same targeted command; 1 scenario and 5 steps passed.
- [x] COMMIT: `2a2547b` — `feat(bge-reranking-evidence): A malformed directional evidence seam is rejected before scoring`

### A complete local BGE artifact loads without a downloader
- [x] RED: removed only this scenario's `@wip`; the targeted BGE run failed on the undefined complete-artifact fixture (non-zero).
  ```text
  GIVN_FEATURES=givn/changes/bge-reranking-evidence/specs GIVN_SPEC_EMBEDDER=stub GIVN_SPEC_BGE=stub cargo test --features spec-retrieval,cross-encoder-reranker --test features_runner -- --name "A complete local BGE artifact loads without a downloader"
  1 scenario (1 failed step); process exited non-zero.
  ```
- [x] GREEN: added a complete local artifact fixture, mandatory offline preflight, and a validated local-artifact test stub that never constructs the downloader-backed model; targeted run passed 7/7 steps.
  ```text
  1 feature
  1 scenario (1 passed)
  7 steps (7 passed)
  ```
- [x] REFACTOR: reran the same targeted command unchanged; 1 scenario and 7 steps passed.
- [x] COMMIT: `167a58a` — `feat(bge-reranking-evidence): A complete local BGE artifact loads without a downloader`

### Approval writer requires enforced Rust parity and complete artifacts
- [x] RED: removed only this scenario's `@wip`; the targeted approval run failed on the undefined benchmark-report fixture (non-zero).
  ```text
  GIVN_FEATURES=givn/changes/bge-reranking-evidence/specs GIVN_SPEC_EMBEDDER=stub GIVN_SPEC_BGE=stub cargo test --features spec-retrieval,cross-encoder-reranker --test features_runner -- --name "Approval writer requires enforced Rust parity and complete artifacts"
  1 scenario (1 failed step); process exited non-zero.
  ```
- [x] GREEN: added a passing bounded benchmark fixture, Rust parity subprocess, complete local artifact hashes, and strict approval-schema assertions; targeted run passed 7/7 steps.
  ```text
  1 feature
  1 scenario (1 passed)
  7 steps (7 passed)
  ```
- [x] REFACTOR: reran the same targeted command unchanged; 1 scenario and 7 steps passed.
- [x] COMMIT: `e81a5bb` — `feat(bge-reranking-evidence): Approval writer requires enforced Rust parity and complete artifacts`

### Approval writer rejects a missing Rust parity attestation
- [x] RED: removed only this scenario's `@wip`; the targeted approval run failed on the undefined passing-report fixture (non-zero).
  ```text
  GIVN_FEATURES=givn/changes/bge-reranking-evidence/specs GIVN_SPEC_EMBEDDER=stub GIVN_SPEC_BGE=stub cargo test --features spec-retrieval,cross-encoder-reranker --test features_runner -- --name "Approval writer rejects a missing Rust parity attestation"
  1 scenario (1 failed step); process exited non-zero.
  ```
- [x] GREEN: added the missing-parity report fixture, explicit approval-command exit assertion, and parity-required/no-artifact assertions; targeted run passed 5/5 steps.
  ```text
  1 feature
  1 scenario (1 passed)
  5 steps (5 passed)
  ```
- [x] REFACTOR: reran the same targeted command unchanged; 1 scenario and 5 steps passed.
- [x] COMMIT: `3a05607` — `feat(bge-reranking-evidence): Approval writer rejects a missing Rust parity attestation`

### Approval writer rejects an incomplete local BGE artifact set
- [x] RED: removed only this scenario's `@wip`; the targeted approval run failed on the undefined incomplete-artifact fixture (non-zero).
  ```text
  GIVN_FEATURES=givn/changes/bge-reranking-evidence/specs GIVN_SPEC_EMBEDDER=stub GIVN_SPEC_BGE=stub cargo test --features spec-retrieval,cross-encoder-reranker --test features_runner -- --name "Approval writer rejects an incomplete local BGE artifact set"
  1 scenario (1 failed step); process exited non-zero.
  ```
- [x] GREEN: added the missing-artifact fixture and assertions for the concrete `special_tokens_map.json` diagnostic and no output artifact; targeted run passed 7/7 steps.
  ```text
  1 feature
  1 scenario (1 passed)
  7 steps (7 passed)
  ```
- [x] REFACTOR: reran the same targeted command unchanged; 1 scenario and 7 steps passed.
- [x] COMMIT: `3178f51` — `feat(bge-reranking-evidence): Approval writer rejects an incomplete local BGE artifact set`

### Approval writer rejects a forged parity document
- [x] RED: removed only this scenario's `@wip`; the targeted approval run failed on the undefined forged-parity fixture (non-zero).
  ```text
  GIVN_FEATURES=givn/changes/bge-reranking-evidence/specs GIVN_SPEC_EMBEDDER=stub GIVN_SPEC_BGE=stub cargo test --features spec-retrieval,cross-encoder-reranker --test features_runner -- --name "Approval writer rejects a forged parity document"
  1 scenario (1 failed step); process exited non-zero.
  ```
- [x] GREEN: added a compiled-parity mutation fixture and exact mismatch/no-artifact assertions; targeted run passed 7/7 steps.
  ```text
  1 feature
  1 scenario (1 passed)
  7 steps (7 passed)
  ```
- [x] REFACTOR: reran the same targeted command unchanged; 1 scenario and 7 steps passed.
- [x] COMMIT: `d6098e8` — `feat(bge-reranking-evidence): Approval writer rejects a forged parity document`

### Approval writer requires enforcement for any approval output
- [x] RED: removed only this scenario's `@wip`; the targeted approval run failed on the undefined passing-report fixture (non-zero).
  ```text
  GIVN_FEATURES=givn/changes/bge-reranking-evidence/specs GIVN_SPEC_EMBEDDER=stub GIVN_SPEC_BGE=stub cargo test --features spec-retrieval,cross-encoder-reranker --test features_runner -- --name "Approval writer requires enforcement for any approval output"
  1 scenario (1 failed step); process exited non-zero.
  ```
- [x] GREEN: added the passing-report/parity fixture and the explicit `--approval-file`/`--enforce` contract assertion; targeted run passed 5/5 steps.
  ```text
  1 feature
  1 scenario (1 passed)
  5 steps (5 passed)
  ```
- [x] REFACTOR: reran the same targeted command unchanged; 1 scenario and 5 steps passed.
- [x] COMMIT: `<pending until commit>` — `feat(bge-reranking-evidence): Approval writer requires enforcement for any approval output`

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
