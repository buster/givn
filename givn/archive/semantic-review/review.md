# Review: semantic-review

## Fabrication Audit

**Step definition scan** — clean.

Scanned `tests/steps/semantic_review_steps.rs` and
`tests/steps/semantic_review_e2e_steps.rs`. No `unimplemented!()`, `todo!()`,
empty bodies, bare returns, or silent-pass bodies remain. The regular steps
perform real fixture setup or report assertions. The E2E steps invoke the
compiled CLI subprocess and assert its exit status and generated report.

**Commit cross-check** — all 114 task boxes have evidence and every scenario
has a recorded commit. The production implementation is present in
`9ee7b81`, `b3ee6c0`, `c0d842a`, `6e36553`, `3843dd1`, `1d5e9fd`, `d77a625`,
`3ddb541`, `951a31d`, and `1e1e2ee`; the remaining scenario commits add the
black-box executable coverage and task evidence for those production paths.
No checked task is backed only by a feature-file edit or an empty step stub.

| Scenario group | Evidence commits | Production/acceptance evidence |
|---|---|---|
| Report entrypoint, scopes, boundary, Rule nesting | `9ee7b81`, `b3ee6c0`, `2e982f6`, `c0d842a`, `e93db06` | `src/cli/check.rs`, `src/spec/retrieval.rs`, executable feature runner |
| Endpoint evidence, target filtering, blocking, carry-forward | `6e36553`, `3843dd1`, `fa7bfe9`, `1d5e9fd`, `ea6f198` | `src/spec/retrieval.rs`, non-mutation and rationale assertions |
| Determinism, grammar, invalidation, prior state | `d899d9e`, `067d9fb`, `c8673a7`, `7c846a2`, `b5456ae`, `d77a625`, `9cc1057`, `3ddb541` | deterministic report and strict prior-state feature scenarios |
| Atomic output, malformed inputs, index recovery | `121c330`, `ab82f23`, `951a31d` | atomic report writer, delta parser context, `validate_index` |
| Archive, unavailable model/feature, advisory separation | `2e4ab0e`, `f9e8b5d`, `1e1e2ee`, `1cb640f` | archive retention, explicit error paths, unchanged advisory entrypoint |
| CLI E2E interaction | `4fe0259` | separate `tests/steps/semantic_review_e2e_steps.rs`, real compiled subprocess |

**Promised-architecture cross-check**

| Component | Exists in tree? | Path |
|---|---|---|
| Blocking semantic-review entrypoint | Yes | `src/cli/check.rs`, `src/spec/retrieval.rs` |
| Semantic candidate/evidence/report primitives | Yes | `src/spec/retrieval.rs` |
| Regular Cucumber bindings | Yes | `tests/steps/semantic_review_steps.rs` |
| Separate CLI E2E bindings | Yes | `tests/steps/semantic_review_e2e_steps.rs` |
| Strict feature runner | Yes | `tests/features_runner.rs` |
| Project-owned full/E2E commands | Yes | `run-tests.sh`, `givn/commands.yaml` |

**Strict-mode proof** — present in `tasks.md`: the first E2E RED run exited
non-zero with `1 scenario (1 failed)`, `1 step (1 failed)`. The change-specific
full runner uses `features_runner`, which calls `.fail_on_skipped()`.

**E2E fidelity**

| @e2e scenario | Real-interface assertion | Driver | Downgraded? |
|---|---|---|---|
| Change review generates a complete report for a semantic candidate | CLI exit status, report existence, report status/evidence, hashes, score, and facts | `common::run_givn_with_envs` starts `CARGO_BIN_EXE_givn` as a real subprocess | No |

**E2E scope**

| Capability | @e2e count | Distinct action | Over-produced? |
|---|---:|---|---|
| Semantic review report | 1 | Invoke `givn check review --change <id>` and inspect the report | No |

**Duplicate-implementation check** — `givn/commands.yaml` configures
`verify.e2e_command: "./run-tests.sh --e2e"`; `run-tests.sh` invokes the same
Cucumber binary with `--tags @e2e`. The tree, including current uncommitted
files, was searched for semantic-review E2E bindings. Only
`tests/steps/semantic_review_e2e_steps.rs` implements this capability; other
`*_e2e_steps.rs` files cover unrelated capabilities.

**Local environment check** — the capability is CLI-only. Design.md declares
no server, database, queue, browser, or digital twin. The configured local
commands start no external service and the full repository runner passed.

**E2E isolation check**

| Full command | E2E command | Identical? | Full count | E2E count | Strictly smaller? |
|---|---|---:|---:|---:|---:|
| `./run-tests.sh` | `./run-tests.sh --e2e` | No | 239 | 36 | Yes |

For the change-specific feature set the corresponding counts are 44 and 1.
The commands are distinct and the E2E command applies the real `@e2e` tag
filter.

**Design conformance**

| Decision | Design says | Actually built | Match? |
|---|---|---|---|
| Blocking entrypoint | Only `givn check review --change` invokes semantic report generation | `src/cli/check.rs` calls `semantic_review` before existing advisory retrieval | Yes |
| Advisory separation | `givn spec review --change` remains advisory and creates no report | Existing `src/cli/spec.rs`/`src/spec/retrieval.rs` path unchanged | Yes |
| Regular step location | `tests/steps/semantic_review_steps.rs` | File exists and is registered | Yes |
| E2E step location | `tests/steps/semantic_review_e2e_steps.rs` | Separate file exists and is registered | Yes |
| E2E command | `./run-tests.sh --e2e` with a real tag filter | Script runs `cargo test ... -- --tags @e2e` | Yes |
| E2E driver | Compiled CLI subprocess | `common::run_givn_with_envs` starts `CARGO_BIN_EXE_givn` | Yes |
| Internal primitives inventory | Design lists the introduced structs/functions | `design.md## Internal Primitives` matches `src/spec/retrieval.rs` | Yes |
| Archive outline preservation | Existing archive interface; AST renderer preserves `Scenario Outline` `Examples` | `src/gherkin/merge.rs` renders examples and has a regression test | Yes |

The Step Definitions markdown table and Internal Primitives section were added
after the integrity hook exposed their required format. The design-review
artifact records the correction and retains `DESIGN-REVIEW: PASS`; no reviewed
architecture decision changed.

**Interaction coverage verification**

| Inventory entry | Matrix row | @e2e scenario | Promised mechanism | Actual mechanism | Match? |
|---|---|---|---|---|---|
| Invoke `givn check review --change <id>` and inspect the generated semantic report and exit status | `design.md` Interaction Coverage Matrix row for the same CLI action | Change review generates a complete report for a semantic candidate | `std::process::Command` compiled CLI subprocess | `common::run_givn_with_envs` plus report/exit assertions | Yes |

## Arc42 Implementation Conformance

Arc42 is enabled in the project manifest. All twelve durable chapter files
exist and were checked for current content. No ASCII-art diagram violation was
found in the reviewed semantic additions; Mermaid diagrams remain Mermaid.

| Chapter/fact | Durable source | `arc42.md` claim | Design/tasks | Implementation evidence | Match? |
|---|---|---|---|---|---|
| 01 goals / FR-28 | `docs/arc42/01-introduction-and-goals.md` | Blocking deterministic semantic worklist with archive evidence | `design.md` report contract; tasks cover all report scenarios | `src/spec/retrieval.rs`, report and E2E suites | Yes |
| 02 constraints / TC-17 | `docs/arc42/02-architecture-constraints.md` | Retrieval-enabled blocking build; explicit no-feature failure | Feature policy in design; no-feature task and test | `src/cli/check.rs`, no-feature subprocess test | Yes |
| 03 context and scope | `docs/arc42/03-context-and-scope.md` | Dedicated check entrypoint, delta-only scopes, permanent-only index | Candidate-universe and compatibility sections | `semantic_candidates`, scope scenarios, index assertions | Yes |
| 04 strategy | `docs/arc42/04-solution-strategy.md` | E5 boundary, strict grammar, evidence hashes, atomic replacement | Report lifecycle and evidence design | `render_semantic_report`, `write_semantic_report`, grammar tests | Yes |
| 05 building blocks | `docs/arc42/05-building-block-view.md` | Separate check/retrieval/report and CLI E2E blocks | Internal Primitives and Step Definitions tables | `src/cli/check.rs`, `src/spec/retrieval.rs`, separate step files | Yes |
| 06 runtime | `docs/arc42/06-runtime-view.md` | Structural gate, semantic report, advisory retrieval, archive flow | Gate sequence and archive scenario | Full check, E2E, and archive-retention results | Yes |
| 07 deployment | `docs/arc42/07-deployment-view.md` | No new service/deployment; retrieval remains opt-in local | Design Local Runnability | No service added; feature/no-feature builds pass | Yes |
| 08 crosscutting | `docs/arc42/08-crosscutting-concepts.md` | Vocabulary, evidence, grammar, index boundary, no mutation | Specs/design scenarios and review dispositions | Report hashes, strict parser, permanent-only recovery tests | Yes |
| 09 decisions / ADR-046 | `docs/arc42/09-architecture-decisions.md` | Dedicated blocking report, fixed candidate universe, agent-owned classification | Design decisions and all task groups | Gate/report implementation and classified current report | Yes |
| 10 quality / Q-52..Q-66 | `docs/arc42/10-quality-requirements.md` | Boundary, grammar, invalidation, recovery, archive, E2E, compatibility | 25 regular + 1 E2E scenarios | 44 change scenarios and 239 repository scenarios pass | Yes |
| 11 risks / R-34..R-43 | `docs/arc42/11-risks-and-technical-debt.md` | Stale state, malformed index, no-feature, vocabulary, archive risks mitigated | Evidence/hash/index/error/archive tasks | Targeted scenarios and no-mutation assertions | Yes |
| 12 glossary | `docs/arc42/12-glossary.md` | Candidate, evidence, grammar, rationale, index, gate vocabulary | Feature and design use the same terms | Report and review use `VARIANTE` with explicit rationale; no `UNRELATED` | Yes |

ARC42 CONFORMANCE: CLEAN

## Coverage

Coverage was freshly measured with `./measure-coverage.sh` and merged with
`./merge-coverages.sh`. Both source reports were regenerated before the merge.
The Gherkin runner and the real CLI subprocess paths were included.

```text
Merged Cobertura report: coverage/cobertura-coverage.xml
Lines: 7083/11502 = 61.58%
Branches: 0/0 (the instrumented tool emitted no branch counters)
Non-E2E source report: coverage/non-e2e-cobertura.xml
E2E source report: coverage/e2e-cobertura.xml
```

The merged report has non-zero production line coverage. The real CLI E2E path
is exercised through `CARGO_BIN_EXE_givn`; the report is not runner-only.

### Coverage Gap Classification

| Region | Classification | Action/evidence |
|---|---|---|
| Semantic report generation, parsing, index recovery, atomic writes, and gate paths | Missing test coverage resolved | Change-specific feature scenarios cover boundary, grammar, invalid prior state, malformed index/delta, no-feature, model error, archive, and mutation contracts. |
| CLI dispatch and real subprocess report path | Missing test coverage resolved | Dedicated E2E scenario and repository E2E runner pass. |
| Optional real-model download/inference and feature combinations not enabled by the deterministic fixture runner | Legitimately hard to test | The local contract requires no network/model download; deterministic stub coverage plus explicit unavailable-model and no-feature failure paths exercise the supported local behavior. |
| Dead code | None found | Clippy with `-D warnings` passes; no new unused production primitive remains. |

## Test Suite Result

Configured commands and results:

```text
./run-tests.sh                         239 scenarios, 1360 steps: PASS
./run-tests.sh --e2e                    36 scenarios, 219 steps: PASS
change-specific ./run-tests.sh          44 scenarios, 432 steps: PASS
change-specific ./run-tests.sh --e2e     1 scenario, 12 steps: PASS
givn lint --change semantic-review       1 file checked: clean
cargo check --no-default-features        PASS
cargo check --features spec-retrieval    PASS
cargo test --lib --no-default-features   142 passed: PASS
cargo test --lib --features spec-retrieval 144 passed: PASS
cargo clippy --workspace --all-targets -- -D warnings: PASS
cargo fmt --all -- --check              PASS
cargo test --lib gherkin::merge::tests::added_scenario_keeps_outline_examples: PASS
```

`givn check review --change semantic-review` passed after all overlap
dispositions were recorded and the one generated candidate was classified as
`VARIANTE` with rationale. The current `semantic-review.md` is `PASS` and has
one candidate.

**Result:** PASS

**Coverage measured:** YES

## Sign-off

- [x] Fabrication audit: CLEAN; no empty step bodies, all task evidence and commits verified, all promised components exist, no downgraded or over-produced E2E scenario.
- [x] All scenarios pass in the configured full and E2E runners.
- [x] Every coverage region is covered, dead, or explicitly hard to test under one of the three allowed buckets.
- [x] No `@wip` tags remain in the delta spec.
- [x] The delta spec contains no implementation-layer detail.
- [x] Local run command starts the full CLI-only environment cleanly; no external stack or digital twin is required.
- [x] The exact E2E command and step-definition file were identified; no parallel semantic E2E implementation exists.
- [x] `verify.e2e_command` is distinct from `verify.command`; scenario counts prove isolation.
- [x] Implementation commands, file layout, framework, and driver match the reviewed design; documentation-format corrections were re-reviewed.
- [x] Interaction coverage maps the single inventory entry to the design matrix, one E2E scenario, and the compiled CLI subprocess.
- [x] `design.md` contains the Internal Primitives inventory for the introduced report structures and functions.
- [x] Arc42 slug mapping was verified against real `docs/arc42/<slug>.md` files.
- [x] No finding is excused with a classification outside Dead code, Missing test coverage, or Legitimately hard to test.

REVIEW: PASS
