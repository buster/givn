# Review: semantic-token-cap-review

## Fabrication Audit (do this first)

**Step definition scan** — the two change-specific step files and the reused
blocking-review glue were read after the final GREEN runs. The change-specific
files contain no `unimplemented!()`, `todo!()`, empty body, bare `return`, or
no-op step. The compiled runner uses `.fail_on_skipped()`.

Result: `0 empty step bodies found across the change-specific regular and E2E
step-definition files`; the reused `bge_reranking_evidence_steps.rs` action has
real subprocess behavior and was extended only to pass the token-budget seam.

**Commit cross-check** — setup is infrastructure rather than production code;
the scenario commits that implement behavior touch `src/spec/retrieval.rs` or
the real CLI test boundary.

| Scenario/task | Commit hash | Touches production code? |
|---|---|---|
| Strict runner and step skeleton setup | `a783bc3` | Infrastructure commit; runner/config/test files, not product source |
| Permanent duplicate audit preserves BGE evidence when NLI reaches the cap | `b233079` | Yes, `src/spec/retrieval.rs` |
| Exactly 512 tokens keeps normal second-stage behavior | `87baf9f` | Yes, `src/spec/retrieval.rs` |
| Permanent duplicate audit keeps partial BGE token-cap evidence per candidate | `e3a48c5` | Reuses production behavior from `b233079`; real compiled-CLI E2E glue |
| Blocking change review keeps an all-over-cap candidate unresolved | `44a2c94` | Yes, `src/spec/retrieval.rs` |

**Promised-architecture cross-check**

| Component from design.md | Exists in tree? | Path |
|---|---|---|
| Complete semantic serializer and retrieval guard | Yes | `src/spec/retrieval.rs` |
| Regular step definitions | Yes | `tests/steps/semantic_token_cap_review_steps.rs` |
| E2E step definitions | Yes | `tests/steps/semantic_token_cap_review_e2e_steps.rs` |
| Strict Cucumber runner | Yes | `tests/features_runner.rs` |
| Deterministic runner matrix | Yes | `run-tests.sh` |
| Canonical retrieval guidance | Yes | `assets/instructions/specs.md` |

**Strict-mode proof** — `tasks.md` setup evidence records a targeted undefined
step failure with `cargo test` exit 101. The same runner then rejected the
registered `unimplemented!()` body with a non-zero exit. The runner source
contains `GivnWorld::cucumber().fail_on_skipped()`.

**Downgraded E2E scenario scan**

| @e2e scenario | Real-interface assertion present? | Driven via real browser (if UI)? | Downgraded? |
|---|---|---|---|
| Permanent duplicate audit keeps partial BGE token-cap evidence per candidate | Yes; compiled `givn spec duplicates --format json` exit/stdout JSON | N/A, CLI capability | No |
| Blocking change review keeps an all-over-cap candidate unresolved | Yes; compiled `givn check review --change running-change` exit/report | N/A, CLI capability | No |

**E2E scope check** — the feature inventory contains two distinct CLI actions,
and exactly two matching `@e2e` scenarios. The partial-BGE and all-over-cap
cases are model/input variants of those two actions, not extra interface
actions.

**Duplicate-implementation check** — `verify.e2e_command` is literally
`./run-tests.sh --e2e`. Its compiled Cucumber invocations use the `@e2e` tag
filters in `run-tests.sh`. The tree was searched for other
`semantic_token_cap_review` E2E implementations; only
`tests/steps/semantic_token_cap_review_e2e_steps.rs` contains the new capability
steps. The reused blocking action is in
`tests/steps/bge_reranking_evidence_steps.rs` and is the same real subprocess
boundary, not a weaker duplicate.

**Local environment check** — design.md specifies no server, database, digital
twin, network service, or model download. The E2E fixtures create isolated
temporary projects and invoke the compiled CLI with deterministic stub
embedder/token-budget variables. Startup is clean and no multi-gigabyte
artifact is required.

**E2E isolation check** — `givn/commands.yaml` reads as follows:

| `verify.command` | `verify.e2e_command` | Identical? | Full active-change count | E2e-only active-change count | Strictly smaller? |
|---|---|---|---:|---:|---|
| `./run-tests.sh` | `./run-tests.sh --e2e` | No | 4 scenarios | 2 scenarios | Yes |

The proof run reported `4 scenarios (2 passed, 2 failed)` while E2E stubs were
RED, and `2 scenarios (2 failed)` for the E2E-only filter. After GREEN, the
active regular command and active E2E command both exited 0. The permanent
repository matrix also passed with `328` scenarios.

**Design.md conformance check**

| Decision | design.md says | Actually built | Matches? |
|---|---|---|---|
| Runner | `tests/features_runner.rs`, strict Cucumber | Same runner with `.fail_on_skipped()` | Yes |
| Regular steps | Separate capability file | `semantic_token_cap_review_steps.rs` | Yes |
| E2E steps | Separate capability file | `semantic_token_cap_review_e2e_steps.rs` | Yes |
| Verify commands | `./run-tests.sh`, `./run-tests.sh --e2e` | Same values; active paths are passed through `GIVN_FEATURES` | Yes |
| E2E boundary | Compiled CLI subprocess | `common::run_givn_with_envs` invokes `CARGO_BIN_EXE_givn` | Yes |
| Test seam | Stub-only deterministic token budgets | `GIVN_SPEC_*_TOKEN_BUDGET`, gated by `GIVN_SPEC_EMBEDDER=stub` | Yes |

**Interaction coverage verification**

| Inventory entry | Matrix row exists? | @e2e scenario | Promised mechanism | Actual mechanism | Match? |
|---|---|---|---|---|---|
| run the permanent duplicate audit with complete semantic-pair token-cap evidence | Yes | Permanent duplicate audit keeps partial BGE token-cap evidence per candidate | Compiled `givn spec duplicates --format json` subprocess | Same; stdout/JSON primary | Yes |
| run a blocking change review with a complete semantic-pair token-cap finding | Yes | Blocking change review keeps an all-over-cap candidate unresolved | Compiled `givn check review --change running-change` subprocess | Same; exit/report primary | Yes |

## Arc42 implementation conformance

The Arc42 addon is enabled. All twelve durable chapters and the change-level
assessment were opened. The completed implementation matches the reviewed
serializer/hash, tokenizer-preflight, independent-layer, cap-state, test-seam,
and guidance decisions.

| Arc42 chapter or fact | Durable-doc source | arc42.md claim | design.md | tasks.md | Completed implementation evidence | Match? |
|---|---|---|---|---|---|---|
| 01 goals | `docs/arc42/01-introduction-and-goals.md` | Complete input, hash, cap, blocking goals | Scope/invariants | All scenario tasks | Serializer, cap JSON/report, blocking E2E | Yes |
| 02 constraints | `docs/arc42/02-architecture-constraints.md` | Actual tokenizer, no truncation/padding, test seam | Token guard | Exact/over-cap evidence | `load_unbounded_tokenizer`, fixture gate | Yes |
| 03 context | `docs/arc42/03-context-and-scope.md` | Four retrieval surfaces and output contract | Surface/output sections | E2E CLI actions | Duplicate, Explore, advisory, blocking paths | Yes |
| 04 strategy | `docs/arc42/04-solution-strategy.md` | Shared serialization and independent layers | Candidate pool flow | NLI independence scenario | BGE/NLI receive immutable candidate set | Yes |
| 05 building blocks | `docs/arc42/05-building-block-view.md` | Serializer, hash, guard, cap evidence | Retrieval responsibility table | Production files listed | All implemented in `src/spec/retrieval.rs` | Yes |
| 06 runtime | `docs/arc42/06-runtime-view.md` | Pre-model partition and all-over refusal | Preflight sequence | Blocking E2E evidence | Zero-score all-over runs do not construct models | Yes |
| 07 deployment | No affected claim | No deployment change | No external artifact required | No deployment task | Stub CLI fixtures only | Yes |
| 08 crosscutting | `docs/arc42/08-crosscutting-concepts.md` | Guidance/state distinction and shared guard | Guidance contract | Final lint and review | Canonical guidance and output states | Yes |
| 09 decisions | `docs/arc42/09-architecture-decisions.md` | ADR-059 through ADR-061 | Serializer/token/pool decisions | Scenario evidence | Production follows all three ADRs | Yes |
| 10 quality | `docs/arc42/10-quality-requirements.md` | 512/513, no-load, independence, bounded tests | Deterministic seam | Targeted and full runs | Exact, partial, all-over scenarios pass | Yes |
| 11 risks | `docs/arc42/11-risks-and-technical-debt.md` | R-66 through R-71 mitigations | Explicit guard/guidance | Review evidence | Hash, tokenizer, cap, pool, seam safeguards | Yes |
| 12 glossary | `docs/arc42/12-glossary.md` | Complete serialization and cap vocabulary | Named states and counts | JSON/report assertions | Fields/statuses use the documented terms | Yes |

- [x] Every affected chapter was opened and checked.
- [x] Every relevant Arc42 fact is represented in design.md and mapped to tasks.
- [x] The implementation follows the documented facts and decisions.
- [x] No new undocumented architecture change was found.
- [x] No mismatch remains.

ARC42 CONFORMANCE: CLEAN

## Coverage

**Coverage result:** measured with `GIVN_FEATURES` set to the active change and
`cargo llvm-cov`, then merged by `./merge-coverages.sh`:

| Report | Lines covered | Lines valid | Line rate | Branches |
|---|---:|---:|---:|---|
| Non-E2E | 1817 | 8997 | 20.1956% | 0/0 |
| E2E | 2634 | 8997 | 29.2764% | 0/0 |
| Merged | 4451 | 17994 | 24.7360% | 0/0 |

The reports are fresh, include the Gherkin runner binary, and are present at
`coverage/non-e2e-cobertura.xml`, `coverage/e2e-cobertura.xml`, and
`coverage/cobertura-coverage.xml`.

## Coverage gap classification

| Location | Lines/region | Classification | Action taken |
|---|---|---|---|
| `src/spec/retrieval.rs` actual local ONNX/model construction and downloader refusal branches | Artifact-dependent paths | 3. Legitimately hard to test | Production uses local artifacts; regular/E2E tests must not download or require multi-GB models. Deterministic stub and local-artifact validation paths are covered. |
| `src/spec/retrieval.rs` malformed/invalid local tokenizer and model combinations | Artifact-dependent error branches | 3. Legitimately hard to test | Preflight rejects before construction; focused tests use deterministic fixtures and existing corrupt-artifact scenarios. |
| Unrelated legacy feature combinations outside this change's retrieval matrix | Existing code paths | 1. Dead code | No new untested abstraction was added for these paths; existing private legacy paths remain outside this change's acceptance surface. |

## Test suite result

`./run-tests.sh` exited 0. The permanent matrix reported `32 features`, `328
scenarios (328 passed)`, and `2019 steps (2019 passed)`; BGE, gate, disabled,
NLI, and combined matrices also exited 0. The active-change matrix passed both
regular scenarios.

`./run-tests.sh --e2e` exited 0. The active-change E2E matrix passed both CLI
scenarios; the permanent E2E matrices also exited 0.

**Result:** PASS

**Coverage measured:** YES

## Semantic Review and Remediation

The new token-cap candidates are intentionally unresolved evidence, not Givn
classifications. The dedicated scenarios assert exact cap state, counts,
visibility, blocking behavior, and shorten/split guidance. No automatic
classification or remediation was applied.

| Candidate | Classification | Rationale | Projected remediation | Score after remediation | Verified? |
|---|---|---|---|---|---|
| Partial NLI-cap candidate | UNCERTAIN/token-cap evidence | Pair is over the NLI limit; no classification is allowed | Shorten or split the scenarios, then rerun audit | Not applicable until rerun | Evidence verified |
| All-over-cap blocking candidate | UNCERTAIN/token-cap evidence | Neither layer scored a pair; candidate remains blocking | Shorten or split both scenarios, then rerun review | Not applicable until rerun | Evidence verified |

```text
semantic remediation verified: 0 (no resolved classification was claimed)
```

## Sign-off

- [x] Fabrication audit clean; no empty bodies or downgraded E2E scenarios.
- [x] All scenarios pass the configured regular and E2E runners.
- [x] Coverage is measured and all listed gaps are classified.
- [x] No `@wip` tags remain in the delta spec.
- [x] The delta spec contains no implementation-layer detail.
- [x] The local deterministic CLI run starts cleanly without external services.
- [x] The configured E2E command and all parallel implementations were checked.
- [x] E2E isolation is proven by active-scope counts 4 versus 2.
- [x] Implementation matches design.md's reviewed file layout and commands.
- [x] Interaction coverage maps both inventory entries to scenarios and CLI steps.
- [x] Design.md's retrieval architecture-impact table covers the serializer,
      budget, run, and candidate evidence structures; no unreviewed external
      primitive or service was introduced.
- [x] Arc42 slug mapping and implementation conformance are clean.
- [x] No finding uses an invalid fourth coverage classification.

**REVIEW: PASS**
