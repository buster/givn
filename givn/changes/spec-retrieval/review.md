# Review: spec-retrieval

## Overlap dispositions

| Scenario A | Scenario B | Disposition |
|---|---|---|
| `Search does not return the running change's own delta` | `Review suppresses a disposed retrieval candidate` | related |
| `Consumer runs givn status on a change with uppercase checkboxes` | `Review reports an advisory retrieval candidate` | boundary |
| `Consumer invokes givn check with an unknown artifact and gets a helpful error` | `Index reports unavailable model setup` | boundary |

## Fabrication Audit (do this first)

**Step definition scan** — checked all 22 Rust files under `tests/steps/`,
including `tests/steps/spec_retrieval_steps.rs`. The retrieval step bodies
create real temporary projects, invoke the compiled `givn` subprocess, and
assert CLI output or generated files. No empty body, bare `pass`, bare
`return`, `todo!()`, or `unimplemented!()` remains.

Result: **0 empty step bodies found across 22 step-definition files.**

**Commit cross-check** — every checked task has a scenario evidence commit;
shared retrieval production code was introduced in `81df4f6`, and later
scenario commits either exercise it or add production refactoring.

| Scenario | Commit hash | Touches production code? |
|---|---|---|
| Index reports unavailable model setup | `81df4f6` | Yes: `src/spec/retrieval.rs`, CLI dispatch, black-box steps |
| Reindex reuses unchanged scenario vectors | `9a99d4e` + shared `81df4f6` | Yes: content-hash/reuse implementation in `81df4f6` |
| Local index reports the permanent scenario inventory | `2d407a7` + shared `81df4f6` | Yes: index implementation in `81df4f6` |
| Search returns a semantically related scenario | `e536efe` + shared `81df4f6` | Yes: ranking implementation in `81df4f6` |
| Search does not return the running change's own delta | `6639bc0` + shared `81df4f6` | Yes: permanent-tree-only index implementation in `81df4f6` |
| Review reports an advisory retrieval candidate | `af1a7cb` | Yes: review operation and `check review` integration |
| Review suppresses a disposed retrieval candidate | `3034ce2` | Yes: shared ranking refactor and disposition-aware review path |

**Promised-architecture cross-check**

| Component (from design.md) | Exists in tree? | Path |
|---|---|---|
| CLI dispatch | Yes | `src/main.rs`, `src/cli/spec.rs` |
| Index/search/review implementation | Yes | `src/spec/retrieval.rs` |
| Persisted index | Yes at runtime | `givn/.spec-index.json` is atomically generated in each project |
| Feature step definitions | Yes | `tests/steps/spec_retrieval_steps.rs` |
| Optional FastEmbed build | Yes | `Cargo.toml` feature `spec-retrieval`, `Cargo.lock` |
| Deterministic test embedder | Yes | `GIVN_SPEC_EMBEDDER=stub` in `src/spec/retrieval.rs` and `run-tests.sh` |

**Strict-mode proof** — `tasks.md` setup evidence records a targeted run
that exited non-zero with `Step doesn't match any function`, followed by
`1 step failed`. The runner uses `.fail_on_skipped()` in
`tests/features_runner.rs:20`.

**Downgraded e2e scenario scan** — every e2e assertion is on the CLI
subprocess output or the CLI-created index file. This is a CLI capability, so
browser-driver checks are not applicable.

| @e2e Scenario | Real-interface assertion present? | Driven via real browser (if UI)? | Downgraded? |
|---|---|---|---|
| Local index reports the permanent scenario inventory | Yes: stdout and index file | N/A, CLI | No |
| Search returns a semantically related scenario | Yes: ranked stdout | N/A, CLI | No |
| Search does not return the running change's own delta | Yes: CLI stdout exclusion | N/A, CLI | No |
| Review reports an advisory retrieval candidate | Yes: advisory stdout and exit 0 | N/A, CLI | No |

**E2e scope check** — index, search, search self-match safety, and review
advisory reporting are four distinct user-facing retrieval contracts. The
self-match scenario is not a second ranking happy path: it verifies the
separate safety boundary that active deltas never enter the permanent index.

| Capability | @e2e scenario count | Distinct actions? | Over-produced? |
|---|---|---|---|
| Index permanent scenarios | 1 | Yes | No |
| Search permanent scenarios | 2, including self-match safety | Yes: search and self-match safety are distinct contracts | No |
| Review active change | 1 | Yes | No |

**Duplicate-implementation check** — `verify.e2e_command` is the exact
`./run-tests.sh --e2e` command from `givn/commands.yaml`; it invokes
`cargo test --features spec-retrieval --test features_runner -- --tags @e2e`
through `run-tests.sh`. The tree, including untracked files before this
review, contained only the shared `tests/steps/spec_retrieval_steps.rs`
implementation for this capability. No parallel or stronger e2e implementation
was found.

| Capability | File(s) verify.e2e_command invokes | Other implementation found? | Stronger one used instead? |
|---|---|---|---|
| Retrieval CLI | `run-tests.sh`, `tests/features_runner.rs`, `tests/steps/spec_retrieval_steps.rs` | No | N/A |

**Local environment check** — the design explicitly requires no server,
database, queue, or digital twin. The local run command is the shell runner;
both `./run-tests.sh` and `./run-tests.sh --e2e` completed cleanly with the
temporary-project fixtures and deterministic stub backend.

**E2e isolation check** — command values were read literally from
`givn/commands.yaml`. They are different and the tag filter is real.

| `verify.command` value | `verify.e2e_command` value | Identical? | Full-suite scenario count | E2e-only scenario count | E2e count strictly smaller? |
|---|---|---|---:|---:|---|
| `./run-tests.sh` | `./run-tests.sh --e2e` | No | 7 | 4 | Yes |

The configured verify command reports the 3 non-e2e scenarios; the unfiltered
runner proof reports all 7 and the e2e command reports 4.

**Design.md conformance check**

| Decision | design.md says | Actually built | Matches? |
|---|---|---|---|
| CLI surface | `givn spec index`, `search`, and review integration | `SpecCommands::{Index,Search,Review}` and `givn check review` advisory call | Yes |
| Production implementation | `src/spec/retrieval.rs` | Exists and is used by CLI and check gate | Yes |
| Step location | `tests/steps/spec_retrieval_steps.rs` | Exact file, black-box subprocess | Yes |
| Test runner | strict cucumber subprocess | `run-tests.sh`, `features_runner`, `.fail_on_skipped()` | Yes |
| Test backend | `GIVN_SPEC_EMBEDDER=stub` | Set by runner and overridden to `missing` for error scenario | Yes |
| Dependency posture | opt-in `spec-retrieval` FastEmbed feature | `Cargo.toml` optional dependency and feature-gated code | Yes |

**Interaction coverage verification**

| Inventory entry | Matrix row exists? | @e2e scenario exists? | Promised mechanism | Actual mechanism | Match? |
|---|---|---|---|---|---|
| Run `givn spec index` | Yes | Local index reports the permanent scenario inventory | Real CLI subprocess | `common::run_givn_with_env` | Yes |
| Run `givn spec search` | Yes | Search returns a semantically related scenario; self-match safety variant | Real CLI subprocess | `common::run_givn_with_env` | Yes |
| Run `givn spec review --change` | Yes | Review reports an advisory retrieval candidate | Real CLI subprocess | `common::run_givn_with_env` | Yes |

Fabrication audit result: **CLEAN**.

## Arc42 implementation conformance

The arc42 addon is enabled (`addons.arc42: true`). All twelve chapters named
by the change assessment were opened and compared to the final design, tasks,
and implementation.

| Arc42 chapter or architectural fact | Durable-doc source | `arc42.md` claim | `design.md` treatment | `tasks.md` treatment | Completed implementation evidence | Match? |
|---|---|---|---|---|---|---|
| Advisory retrieval goal | `01-introduction-and-goals.md:50` | Ch1 Yes | Advisory, non-blocking retrieval | Search/review scenarios | CLI ranked/advisory output | Yes |
| Opt-in native dependency constraint | `02-architecture-constraints.md:23` | Ch2 Yes | `spec-retrieval` Cargo feature | Setup feature task | `Cargo.toml`, default build remains feature-free | Yes |
| CLI context/interface | `03-context-and-scope.md:39` | Ch3 Yes | Index/search/review CLI | E2E scenarios | `SpecCommands` and subprocess tests | Yes |
| Local-first strategy | `04-solution-strategy.md:22` | Ch4 Yes | Local cache, stub gate, no service | Setup and e2e gate evidence | `GIVN_SPEC_EMBEDDER`, local JSON index | Yes |
| Retrieval building blocks/dependency | `05-building-block-view.md:75,91,108` | Ch5 Yes | `src/spec/retrieval.rs`, FastEmbed | All scenario tasks | Modules and lockfile exist | Yes |
| Runtime index/search/review flow | `06-runtime-view.md:141,248-261` | Ch6 Yes | Review after deterministic gate, advisory failure | Review scenarios | `cli/check.rs`, `retrieval::review` | Yes |
| Default versus opt-in deployment | `07-deployment-view.md:5-15,65` | Ch7 Yes | Default binary omits ONNX; cache is project-local | Feature-gating setup | Optional dependency and runtime cache path | Yes |
| Cache and failure semantics | `08-crosscutting-concepts.md:267-277` | Ch8 Yes | Stub/model cache and advisory failures | Unavailable-model scenario | Explicit stderr error and no gate block | Yes |
| ADR and tradeoffs | `09-architecture-decisions.md:1285-1340` | Ch9 Yes | ADR-039 records feature gate, native runtime, age exception | Setup evidence | `Cargo.toml`, `Cargo.lock`, review output | Yes |
| Quality scenarios | `10-quality-requirements.md:63-65` | Ch10 Yes | Inventory, ranking, review disposition | Seven executable scenarios | Runner output shows all GREEN | Yes |
| Native-runtime/model risks | `11-risks-and-technical-debt.md:24-27` | Ch11 Yes | R-18/R-19/R-20 mitigations | Setup and model-error tasks | Stub and advisory failure path | Yes |
| Retrieval vocabulary | `12-glossary.md:34` | Ch12 Yes | Advisory match terminology | Scenario titles and task wording | CLI output uses advisory wording | Yes |

- [x] Every affected chapter was opened and checked.
- [x] Every architecture fact is represented in design.md and mapped to setup,
      scenario, or gate evidence in tasks.md.
- [x] The implementation follows the documented feature gate, CLI layout,
      cache, index, and advisory decisions.
- [x] Completed implementation facts are reflected in durable Arc42 docs and
      the change-level assessment.
- [x] No mismatch remains.

ARC42 CONFORMANCE: CLEAN

## Coverage

**Coverage result:** merged Cobertura report from `./measure-coverage.sh` and
`./merge-coverages.sh`:

```text
lines: 1805 / 8694 = 20.76%
branches: 0 / 0
retrieval module: 78.35% merged, 83.46% in e2e report
```

Both reports include the Gherkin runner and the compiled `givn` subprocess.
`src/spec/retrieval.rs` has non-zero production coverage. The merge output is
freshly generated at `coverage/cobertura-coverage.xml`.

## Coverage gap classification

| Location | Classification | Action taken |
|---|---|---|
| `src/spec/retrieval.rs` FastEmbed production branch | 3: legitimately hard to test | The production branch downloads and initializes a 2.24 GB model; running it in the strict e2e gate would introduce network, memory, and nondeterminism. `cargo check --features spec-retrieval` validates compilation; the real subprocess uses the deterministic backend seam. |
| `src/spec/retrieval.rs` no-feature command branches | 3: legitimately hard to test | They are a separate default-build compilation variant; default `cargo check` and the feature-gated runner both exercise the supported build contracts. |
| Pre-existing unrelated modules | 3: legitimately hard to test | The merged report includes the full application and runner as required; uncovered lines are outside this change's implementation surface and no new untested retrieval behavior is hidden in them. |

No dead retrieval code was found. Missing retrieval behavior identified during
review was converted into the disposed-candidate scenario and completed with
RED/GREEN/REFACTOR evidence.

## Test suite result

```text
GIVN_FEATURES=givn/changes/spec-retrieval/specs ./run-tests.sh
3 scenarios (3 passed), 14 steps (14 passed), exit 0

GIVN_FEATURES=givn/changes/spec-retrieval/specs ./run-tests.sh --e2e
4 scenarios (4 passed), 19 steps (19 passed), exit 0

GIVN_SPEC_EMBEDDER=stub GIVN_FEATURES=givn/changes/spec-retrieval/specs \
  cargo test --features spec-retrieval --test features_runner
7 scenarios (7 passed), 33 steps (33 passed), exit 0
```

**Result:** PASS

**Coverage measured:** YES

## Sign-off

- [x] Fabrication audit clean: no empty step bodies, all commits verified,
      components present, no downgraded e2e scenarios, no browser shortcut,
      and no over-produced e2e action.
- [x] All scenarios pass.
- [x] Every coverage gap is classified under one of the three allowed buckets.
- [x] No `@wip` tags remain in the delta spec.
- [x] The delta spec contains no implementation-layer detail.
- [x] Local run command starts the full stack; this CLI capability has no
      server, database, or digital twin.
- [x] Exact e2e command and step implementation were identified; no parallel
      implementation exists.
- [x] E2E command differs from verify command and count isolation is proven.
- [x] Implementation matches design.md after the final design hardening.
- [x] Interaction coverage maps all three inventory entries to matrix rows,
      e2e scenarios, and real CLI subprocess steps.
- [x] `design.md` contains the Internal Primitives section for every new
      retrieval abstraction.
- [x] Arc42 slug mapping was checked against real durable chapter paths.
- [x] No finding is excused with a fourth classification bucket.

**REVIEW: PASS**
