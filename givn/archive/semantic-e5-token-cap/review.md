# Review: semantic-e5-token-cap

## Fabrication Audit

**Strict runner proof:** The setup task records the targeted Cucumber run with
the pending E5 step. It exited non-zero with `1 scenario (1 failed)` and the
`unimplemented!("E5 token-cap E2E setup step")` panic. The runner uses
`GivnWorld::cucumber().fail_on_skipped()`.

**Step definition scan:** The change-specific regular and E2E modules contain
no empty bodies, `unimplemented!()`, `todo!()`, bare returns, or no-op steps.
The reused duplicate and blocking CLI bindings perform real subprocess setup,
execution, and assertions. Empty step bodies found: 0.

**Commit cross-check:**

| Scenario/task | Commit | Production evidence |
|---|---|---|
| Setup and strict runner proof | `06d2238` | `src/spec/retrieval.rs` plus runner and E2E files |
| Indexing keeps fitting scenarios when another passage exceeds E5 capacity | `06d2238` | Partial E5 batch, tokenizer preflight, index metadata |
| Duplicate audit preserves deterministic findings and reports an E5 token-cap candidate | `700e1b6` | Duplicate cap collection and JSON/text rendering |
| Explore keeps an E5 token-cap finding visible while fitting results remain usable | `de4a25a` | Explore cap aggregation and query-vector handling |
| Advisory review exposes an E5 token-cap finding without blocking | `61a726f` | Advisory cap output and deterministic seam |
| Blocking review keeps an over-cap active scenario unresolved | `26063a3` | Blocking semantic report and unresolved gate |
| Exactly 512 E5 passage tokens retain normal embedding behaviour | `bf2c117` | Reused production boundary with real JSON assertions |
| Fitting-contract and second-stage hardening | `ee3ea4c` | Empty additive-field compatibility, cap propagation, index validation |

The exact-512 scenario is a deliberate production-behavior reuse case. Its
commit adds the boundary assertion through the real CLI; the production
partition it exercises is implemented and covered by `06d2238` and
`ee3ea4c`.

**Promised components:**

| Component | Exists | Evidence |
|---|---|---|
| E5 partial embedding and tokenizer preflight | Yes | `src/spec/retrieval.rs` |
| Regular capability steps | Yes | `tests/steps/semantic_e5_token_cap_steps.rs` |
| Separate E2E capability steps | Yes | `tests/steps/semantic_e5_token_cap_e2e_steps.rs` |
| Strict Cucumber runner | Yes | `tests/features_runner.rs` |
| Full/E2E command matrix | Yes | `run-tests.sh`, `givn/commands.yaml` |

**E2E real-interface audit:** Every E2E scenario drives the compiled `givn`
CLI through `common::run_givn_with_envs` and asserts exit status plus rendered
stdout, JSON, index bytes, or semantic report bytes. No browser or external
service exists in this capability, so no browser driver or digital twin is
required. No E2E scenario is repository-only or downgraded.

| Inventory action | E2E scenario | Interface assertion |
|---|---|---|
| Build or refresh the permanent E5 scenario index | Indexing keeps fitting scenarios when another passage exceeds E5 capacity | `givn spec index`, stdout and index JSON |
| Run the permanent duplicate audit | Duplicate audit preserves deterministic findings and reports an E5 token-cap candidate | `givn spec duplicates --format json`, parsed JSON |
| Explore a complete scenario | Explore keeps an E5 token-cap finding visible while fitting results remain usable | `givn spec explore --file ... --format json`, parsed JSON |
| Run advisory semantic change review | Advisory review exposes an E5 token-cap finding without blocking | `givn spec review --change ...`, stdout |
| Run blocking semantic change review | Blocking review keeps an over-cap active scenario unresolved | `givn check review --change ...`, exit status and report |

`verify.command` is `./run-tests.sh`; `verify.e2e_command` is
`./run-tests.sh --e2e`. The full run reported 388 scenarios and the E2E run
reported 55, proving strict subset isolation.

**Deterministic gate dispositions:** The overlap gate reported `6 added, 0
modified, 0 removed` and passed with no shape-match findings. There are no
removed-plus-added pairs. All six scenarios were reviewed as keep decisions;
the long scenario checks retain the complete canonical serialization because
token measurement must cover the whole input. No automatic score-avoidance
prose or scenario edit was introduced.

## Arc42 Implementation Conformance

The Arc42 addon is enabled. The twelve affected durable chapters and the
change-level `arc42.md` were checked against the implementation. Chapter 07
is correctly unaffected.

| Chapter/fact | Durable source | Design/task mapping | Implementation evidence | Match |
|---|---|---|---|---|
| Goals and constraints | `01`, `02` | FR-36, TC-26 | No truncation; 512 inclusive; cap findings | Yes |
| Context and scope | `03` | Five CLI surfaces | Index, duplicate, Explore, advisory, blocking paths | Yes |
| Retrieval strategy | `04` | Partition before inference | `EmbeddingBatch` maps fitting vectors by original position | Yes |
| Building blocks | `05` | Retrieval-only production module | `E5TokenCap`, `E5TokenBudget`, `SpecIndex` metadata | Yes |
| Runtime behavior | `06` | Non-aborting cap branch | Fitting work continues; capped work has no vector | Yes |
| Cross-cutting concepts | `08` | Deterministic additive output | Sorted cap records and legacy empty-field compatibility | Yes |
| Architecture decisions | `09` | No-vector/no-score and provenance | Hash, model, tokenizer, guidance fields | Yes |
| Quality requirements | `10` | Exact/partial/active/blocking scenarios | Six change scenarios plus full matrix | Yes |
| Risks | `11` | Stale metadata and fabricated scores | Hash-conditional reuse and cap validation | Yes |
| Glossary | `12` | Cap vocabulary | `E5_TOKEN_CAP` and `e5_token_caps` wire names | Yes |

ARC42 CONFORMANCE: CLEAN

## Coverage

Coverage was measured with `./measure-coverage.sh` using the instrumented
Cucumber runner and merged with `./merge-coverages.sh`, with
`GIVN_FEATURES=givn/changes/semantic-e5-token-cap/specs` and the stub embedder.

| Report | Lines covered | Lines valid | Line rate | Branches |
|---|---:|---:|---:|---|
| Non-E2E | 1313 | 7434 | 17.6621% | 0/0 |
| E2E | 2430 | 7434 | 32.6877% | 0/0 |
| Merged | 3743 | 14868 | 25.1749% | 0/0 |

Coverage gaps are classified only under the allowed buckets:

| Region | Classification | Justification/action |
|---|---|---|
| Real local E5 model construction and downloader refusal | 3. Legitimately hard to test | Requires local multi-gigabyte artifacts or network; production path is implemented and acceptance uses the approved stub seam |
| Malformed local tokenizer/model artifact combinations | 3. Legitimately hard to test | Existing artifact validation covers the local boundary; acceptance does not download large artifacts |

No new dead-code or missing-scenario classification remains.

## Semantic Review

`semantic-review.md` is `Status: PASS` with `Candidate count: 0`. The cap
scenarios intentionally treat `E5_TOKEN_CAP` as unresolved evidence rather
than a semantic classification. No `DUPLIKAT`, `VARIANTE`, or
`VALID-BOUNDARY` classification was fabricated for a capped endpoint.

## Verification

- `givn lint --change semantic-e5-token-cap` exited 0.
- `./run-tests.sh` exited 0: 388 scenarios passed.
- `./run-tests.sh --e2e` exited 0: 55 scenarios passed.
- `cargo fmt --check` exited 0.
- `cargo clippy --workspace --all-targets -- -D warnings` exited 0.
- `GIVN_SPEC_EMBEDDER=stub givn check review --change semantic-e5-token-cap` exited 0.

## Sign-off

- [x] Fabrication audit clean; no empty bodies or downgraded E2E scenarios.
- [x] Every checked scenario has a verified commit and production evidence.
- [x] Strict-mode proof is present and non-zero.
- [x] Full and E2E commands both pass with a strict-subset count proof.
- [x] Coverage includes the Gherkin runner and all gaps are classified.
- [x] No `@wip` tags remain in the delta spec.
- [x] The delta spec contains no implementation-layer detail.
- [x] Arc42 conformance is clean.
- [x] The deterministic CLI run requires no external service or model download.

REVIEW: PASS
