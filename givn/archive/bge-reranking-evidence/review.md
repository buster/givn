# Review: bge-reranking-evidence

## Fabrication Audit

### Scope and tag integrity

- The delta contains one feature file and six normalized user interactions.
- All six inventory interactions have exactly one `@e2e` scenario.
- All six scenarios retain `@e2e` in the tracked feature file and in the implementation history. Only `@wip` was removed as each scenario became executable.
- `givn lint --change bge-reranking-evidence` exits 0 with no findings.
- No `@wip` tag remains in `givn/changes/bge-reranking-evidence/specs/`.

### Step-body audit

| File | Bodies inspected | Empty/trivial bodies | Result |
|---|---:|---:|---|
| `tests/steps/bge_reranking_evidence_steps.rs` | all change-owned regular bindings | 0 | clean |
| `tests/steps/bge_reranking_evidence_e2e_steps.rs` | all six real-CLI E2E bindings | 0 | clean |
| `tests/features_runner.rs` | runner setup | 0 | `.fail_on_skipped()` present |

The generic `placeholder_step` in `tests/steps/mod.rs` is pre-existing shared glue for `@givn.removed` scenarios and is not a BGE step or a newly checked scenario body. No BGE step uses `pass`, a bare return, `unimplemented!()`, or `todo!()`.

### Strict mode

The setup proof is recorded in `tasks.md` and was independently rerun. The targeted E2E setup command exited non-zero with one failed step because the registered setup stub panicked. The regular runner uses `GivnWorld::cucumber().fail_on_skipped()` in `tests/features_runner.rs`.

### Task and commit audit

- 48 checked scenario commit hashes are present and resolve to 48 unique Git commits.
- Every scenario has RED, GREEN, REFACTOR evidence and a recorded hash.
- Production-bearing implementation commits include `src/spec/retrieval.rs`, `src/main.rs`, `src/cli/spec.rs`, `Cargo.toml`, `assets/instructions/specs.md`, `tools/approach-comparison/`, `run-tests.sh`, and `measure-coverage.sh` changes.
- Pure black-box contract scenarios reuse those production paths and add executable CLI assertions and fixture seams; none is a spec-only or stub-only implementation. The NLI scenario's final hash is `868f1b8`, which contains the production `nli-evidence` capability boundary.
- No component promised by `design.md` is absent: the BGE selector, bounded pool, evidence renderers, parity command, approval writer, local artifact preflight, regular step module, E2E step module, and cucumber runner all exist.

### E2E fidelity

All E2E Then assertions include a primary assertion against the compiled `givn` CLI exit status, stdout, stderr, or generated semantic report emitted by that CLI. No browser UI exists. No HTTP shortcut, direct retrieval-function call, `fetch`, or `XMLHttpRequest` is used. The E2E step file invokes `common::run_givn_with_envs`, which starts `CARGO_BIN_EXE_givn` as a subprocess.

The design's local run section explicitly requires no server, container, digital twin, cloud service, or third-party runtime dependency. The local environment therefore has no dependency startup step; every E5/BGE path uses deterministic stubs and every E2E path reaches the compiled CLI directly.

### Runner binding and isolation

`givn/commands.yaml` binds:

```text
verify.command: ./run-tests.sh
verify.e2e_command: ./run-tests.sh --e2e
```

`run-tests.sh` invokes `tests/features_runner.rs`. Its E5, BGE, approval, gate, and E2E tag expressions are distinct. Approval scenarios tagged `@bge-approval` are excluded from the E5 binary and included in the BGE binary. The E2E command is not identical to the full command.

For the active change feature path:

| Runner | Command | Scenarios | Steps | Result |
|---|---|---:|---:|---|
| Full | `GIVN_FEATURES=givn/changes/bge-reranking-evidence/specs ./run-tests.sh` | 47 | 328 | pass |
| E2E subset | `GIVN_FEATURES=givn/changes/bge-reranking-evidence/specs ./run-tests.sh --e2e` | 6 | 48 | pass |

The E2E count is strictly lower than the full count. The E2E runner's first E5-filtered invocation reports zero scenarios for this BGE-only delta; its second BGE-filtered invocation reports all six E2E scenarios.

### Interaction coverage cross-reference

| Inventory entry | Design matrix row | Feature scenario | E2E driver and primary assertion |
|---|---|---|---|
| run permanent scenario exploration through the givn CLI | Permanent exploration exposes bounded second-stage evidence | present | `spec explore --file`; stdout, score, runtime, bounded-pool evidence |
| run change-aware scenario exploration through the givn CLI | Change-aware exploration narrows the union of permanent and active neighbours | present | `spec explore --change`; stdout scopes, reduced recommendations, identity exclusion |
| run advisory change review through the givn CLI | Advisory change review presents second-stage evidence without classifying candidates | present | `spec review --change`; stdout model, score, runtime, advisory/no-classification evidence |
| run blocking change review through the givn CLI | Blocking change review records second-stage evidence in the semantic report | present | `check review --change`; exit status, generated semantic report, unchanged specs |
| run the permanent duplicate audit through the givn CLI | Permanent duplicate audit uses a bounded endpoint-neighbour union | present | `spec duplicates`; stdout counts, model, score, runtime, deterministic exclusion and bound |
| request semantic authoring guidance through the givn CLI | Semantic authoring guidance prevents score-avoidance prose | present | `instructions specs --change`; CLI stdout guidance |

The implementation matches `design.md`: regular bindings are in `tests/steps/bge_reranking_evidence_steps.rs`, real-subprocess E2E bindings are in `tests/steps/bge_reranking_evidence_e2e_steps.rs`, and the framework is cucumber-rs with `.fail_on_skipped()`.

## Arc42 Implementation Conformance

The Arc42 addon is enabled in `givn/config.yaml`. All twelve chapters are affected according to both the independent assessment and `arc42.md`.

| Chapter | Durable source | `arc42.md` | `design.md` | Tasks/evidence | Implementation evidence | Match |
|---|---|---|---|---|---|---|
| 01 Introduction and goals | `docs/arc42/01-introduction-and-goals.md` | Yes | product goals, benchmark, authoring risk | BGE scenarios and benchmark tasks | E5 raw, BGE admission, runtime and guidance | Yes |
| 02 Architecture constraints | `docs/arc42/02-architecture-constraints.md` | Yes | feature independence, local artifacts, bounds | matrix, artifact, parity tasks | Cargo features and offline preflight | Yes |
| 03 Context and scope | `docs/arc42/03-context-and-scope.md` | Yes | five E5-backed surfaces and NLI boundary | surface and compatibility scenarios | CLI dispatch and output contracts | Yes |
| 04 Solution strategy | `docs/arc42/04-solution-strategy.md` | Yes | bounded E5 union, bidirectional BGE | pool, directional and fallback tasks | `src/spec/retrieval.rs` pipeline | Yes |
| 05 Building block view | `docs/arc42/05-building-block-view.md` | Yes | shared evidence types and renderers | JSON/report/approval tasks | BGE run/evidence/rendering types | Yes |
| 06 Runtime view | `docs/arc42/06-runtime-view.md` | Yes | explore/review/duplicate/unavailable flows | all surface and E2E tasks | real CLI outputs and reports | Yes |
| 07 Deployment view | `docs/arc42/07-deployment-view.md` | Yes | local cache and no-download behavior | artifact and feature-free tasks | preflight and local loader | Yes |
| 08 Cross-cutting concepts | `docs/arc42/08-crosscutting-concepts.md` | Yes | identity, states, counts, bounds, guidance | compatibility and boundary tasks | schemas, selectors and guidance | Yes |
| 09 Architecture decisions | `docs/arc42/09-architecture-decisions.md` | Yes | ADRs for admission, parity, bounds, NLI isolation | design and NLI tasks | approval and cfg boundary | Yes |
| 10 Quality requirements | `docs/arc42/10-quality-requirements.md` | Yes | precision/recall, runtime, accounting | benchmark and deterministic tasks | benchmark evaluator and reports | Yes |
| 11 Risks and technical debt | `docs/arc42/11-risks-and-technical-debt.md` | Yes | filtering, recall, overflow, artifacts, selector collision | failure-path tasks | explicit fallback/refusal behavior | Yes |
| 12 Glossary | `docs/arc42/12-glossary.md` | Yes | E5/BGE states, approval and artifact terms | all observable status tasks | matching runtime vocabulary | Yes |

`design-review.md` was rerun after the approval-matrix tag-filter correction. It records `DESIGN-REVIEW: PASS`, and the durable chapters contain decision-specific content rather than scaffold placeholders. `ARC42 CONFORMANCE: CLEAN`.

## Coverage

Coverage was measured with `cargo llvm-cov` around `tests/features_runner.rs`; the runner starts the compiled `givn` CLI subprocesses used by the scenarios. The measurement command was:

```text
GIVN_FEATURES=givn/changes/bge-reranking-evidence/specs \
GIVN_CARGO_FEATURES=spec-retrieval,cross-encoder-reranker,nli-evidence \
GIVN_COVERAGE_NON_E2E_TAGS='not @e2e and not @gate-combination and not @gate-only' \
GIVN_COVERAGE_E2E_TAGS='@e2e' ./measure-coverage.sh
./merge-coverages.sh
```

Fresh reports were written to `coverage/non-e2e-cobertura.xml`, `coverage/e2e-cobertura.xml`, and `coverage/cobertura-coverage.xml`.

| Report | Lines covered | Lines valid | Line rate | Branch result |
|---|---:|---:|---:|---|
| Non-E2E BGE/NLI | 3393 | 7508 | 45.19% | not emitted by stable llvm-cov |
| E2E BGE | 2905 | 7508 | 38.69% | not emitted by stable llvm-cov |
| Merged | 6298 | 15016 | 41.94% | not emitted by stable llvm-cov |
| Gate combination, separately instrumented | 2181 | 7507 | 29.05% | not emitted by stable llvm-cov |
| Gate-only, separately instrumented | 1835 | 6480 | 28.32% | not emitted by stable llvm-cov |

The changed Rust retrieval production class has non-zero coverage: `src/spec/retrieval.rs` reports 2253/3311 lines, 68.05%, in the merged report. The known production path used by the scenarios is therefore instrumented and exercised. Branch coverage is classified under bucket 3, legitimately hard to test in this environment: `cargo llvm-cov --branch` requires nightly `-Z coverage-options=branch`, while the available nightly toolchain predates this edition-2024 project; stable llvm-cov emits no branch counters. The separate gate reports prove both compile-time gate paths execute despite that toolchain limitation. The Python benchmark writer is exercised through the approval E2E subprocess; Rust LLVM coverage does not instrument Python and no Python-to-Cobertura merge command exists in the project configuration. This is also bucket 3, with the subprocess assertions serving as the executable coverage evidence.

No redundant unit test was added for behavior already covered through the feature runner. The two small NLI unit tests remain behind `nli-evidence`; the observable NLI contract is covered by the CLI feature scenario.

## Deterministic Gates

- `givn lint --change bge-reranking-evidence` is clean.
- Every deterministic shape match reported by `check review` is explicitly disposed below. All are `related`: the scenarios share generic CLI/retrieval wording but assert different commands, feature matrices, failure modes, or output contracts.
- No scenario exceeds the long-scenario threshold; no split-or-keep decision is required.
- No removed-plus-added pair exists in this delta.
- The final retrieval gate generated `semantic-review.md` with `Status: PASS` and `Candidate count: 0`. Generated fixture reports are asserted in their owning scenarios; no `UNCERTAIN` repository candidate is being waved through.

### Overlap dispositions

| Scenario A | Scenario B | Disposition | Rationale |
|---|---|---|---|
| `Failed benchmark approval keeps permanent exploration on E5-only evidence` | `Failed benchmark approval keeps change-aware exploration on E5-only evidence` | related | Same approval-failure setup, different exploration scope and source metadata. |
| `Failed benchmark approval keeps blocking review on E5-only evidence` | `E5-only blocking review preserves the semantic report contract` | related | Both validate E5 blocking output, but one covers failed approval and one explicit BGE disablement. |
| `Failed benchmark approval keeps duplicate JSON on E5-only evidence` | `E5-only duplicate JSON preserves schema version 1` | related | Both validate schema-v1 compatibility, but approval failure and disabled runtime are distinct paths. |
| `Detail mode — unknown change errors` | `A binary without retrieval reports the explicit unavailable mode` | related | Different CLI failures: unknown change validation versus feature-free retrieval compatibility. |
| `Uninitialized project errors` | `A binary without retrieval reports the explicit unavailable mode` | related | Initialization failure differs from a compiled capability-unavailable diagnostic. |
| `Integrity hook fails on Step Definitions section that is not a table` | `A binary without retrieval reports the explicit unavailable mode` | related | Integrity parsing and retrieval capability failure are distinct outputs. |
| `Archive blocked when delta spec has non-removed @wip scenario` | `A binary without retrieval reports the explicit unavailable mode` | related | Archive WIP enforcement differs from feature-free CLI behavior. |
| `Check tasks blocked when delta spec has @wip` | `A binary without retrieval reports the explicit unavailable mode` | related | Task completeness enforcement differs from retrieval availability. |
| `givn addons enable errors when givn/ is not initialised` | `A binary without retrieval reports the explicit unavailable mode` | related | Addon initialization failure differs from retrieval capability failure. |
| `givn addons disable errors when givn/ is not initialised` | `A binary without retrieval reports the explicit unavailable mode` | related | Addon initialization failure differs from retrieval capability failure. |
| `Id with spaces is rejected` | `A binary without retrieval reports the explicit unavailable mode` | related | Change-id validation differs from feature-free retrieval behavior. |
| `Id with non-ASCII letters is rejected` | `A binary without retrieval reports the explicit unavailable mode` | related | Change-id validation differs from feature-free retrieval behavior. |
| `Id with a dot is rejected` | `A binary without retrieval reports the explicit unavailable mode` | related | Change-id validation differs from feature-free retrieval behavior. |
| `Id ending with a hyphen is rejected` | `A binary without retrieval reports the explicit unavailable mode` | related | Change-id validation differs from feature-free retrieval behavior. |
| `Id beginning with a hyphen passed after the argument separator is rejected by validation` | `A binary without retrieval reports the explicit unavailable mode` | related | Change-id validation differs from feature-free retrieval behavior. |
| `Review blocks with a clear message when review.md is missing` | `A binary without retrieval reports the explicit unavailable mode` | related | Review artifact validation differs from feature-free retrieval behavior. |
| `Every resolved semantic classification requires a rationale` | `A binary without retrieval reports the explicit unavailable mode` | related | Semantic remediation validation differs from feature-free retrieval behavior. |
| `A VARIANTE score below the boundary without consolidation remains blocked` | `A binary without retrieval reports the explicit unavailable mode` | related | Variant remediation differs from feature-free retrieval behavior. |
| `A VALID-BOUNDARY score below the boundary without distinction remains blocked` | `A binary without retrieval reports the explicit unavailable mode` | related | Boundary remediation differs from feature-free retrieval behavior. |
| `A VALID-BOUNDARY score exactly at the boundary remains blocked` | `A binary without retrieval reports the explicit unavailable mode` | related | Threshold remediation differs from feature-free retrieval behavior. |
| `Changing a classified endpoint before remediation makes the decision stale` | `A binary without retrieval reports the explicit unavailable mode` | related | Evidence staleness differs from feature-free retrieval behavior. |
| `The audit reports retrieval unavailable without the retrieval feature` | `A binary without retrieval reports the explicit unavailable mode` | related | Duplicate-audit unavailable output and explore compatibility output are distinct command contracts. |
| `The audit reports an unavailable retrieval model` | `A binary without retrieval reports the explicit unavailable mode` | related | Model availability and compile-time capability availability are distinct. |
| `Invalid Gherkin input is rejected` | `A binary without retrieval reports the explicit unavailable mode` | related | Input parsing differs from feature-free retrieval behavior. |
| `Multiple scenarios in one input are rejected` | `A binary without retrieval reports the explicit unavailable mode` | related | Input cardinality differs from feature-free retrieval behavior. |
| `A valid feature without a scenario is rejected` | `A binary without retrieval reports the explicit unavailable mode` | related | Input content validation differs from feature-free retrieval behavior. |
| `A missing draft file is rejected` | `A binary without retrieval reports the explicit unavailable mode` | related | Missing input differs from feature-free retrieval behavior. |
| `File and standard input cannot be selected together` | `A binary without retrieval reports the explicit unavailable mode` | related | Input-source validation differs from feature-free retrieval behavior. |
| `Explore remains unavailable without the retrieval feature` | `A binary without retrieval reports the explicit unavailable mode` | related | Existing explore compatibility and the new explicit subprocess seam cover distinct invocation paths. |
| `Path traversal id is rejected by status` | `A binary without retrieval reports the explicit unavailable mode` | related | Change-id safety differs from feature-free retrieval behavior. |
| `Path traversal id is rejected by archive` | `A binary without retrieval reports the explicit unavailable mode` | related | Change-id safety differs from feature-free retrieval behavior. |
| `Path traversal id is rejected by check` | `A binary without retrieval reports the explicit unavailable mode` | related | Change-id safety differs from feature-free retrieval behavior. |
| `Path traversal id is rejected by lint` | `A binary without retrieval reports the explicit unavailable mode` | related | Change-id safety differs from feature-free retrieval behavior. |
| `Path traversal id is rejected by graph` | `A binary without retrieval reports the explicit unavailable mode` | related | Change-id safety differs from feature-free retrieval behavior. |
| `Path traversal id is rejected by instructions` | `A binary without retrieval reports the explicit unavailable mode` | related | Change-id safety differs from feature-free retrieval behavior. |

## Final Verification

- Full active-change runner: `GIVN_FEATURES=givn/changes/bge-reranking-evidence/specs ./run-tests.sh` passed 47 scenarios and 328 steps.
- E2E active-change runner: `GIVN_FEATURES=givn/changes/bge-reranking-evidence/specs ./run-tests.sh --e2e` passed 6 scenarios and 48 steps.
- `cargo check --features spec-retrieval,cross-encoder-reranker` passed.
- `cargo check --features spec-retrieval,nli-evidence` passed.
- Strict-mode proof is present and non-zero in `tasks.md`.
- No `@wip` tags remain.
- `givn lint --change bge-reranking-evidence` passed.

## Sign-off

- [x] Fabrication audit clean.
- [x] All checked tasks have resolved commit hashes and executable evidence.
- [x] Strict mode proven.
- [x] Full and E2E runners pass.
- [x] Coverage measured with the Gherkin runner and real CLI subprocesses; all gaps are classified under the three allowed buckets.
- [x] Arc42 implementation conformance clean.
- [x] Canonical E2E inventory, driver fidelity, and primary CLI assertions verified.
- [x] No design command, file-layout, framework, or runner-filter deviation remains undocumented.

REVIEW: PASS
