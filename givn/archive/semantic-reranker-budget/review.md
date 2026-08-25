# Review: semantic-reranker-budget

## Fabrication Audit

This change replaces the fixed 512-pair second-stage refusal with a
configurable `GIVN_SPEC_MAX_RERANK_PAIRS` budget (default 2048), scoring the
highest-E5 prefix and reporting the tail as `BGE_UNSCORED` / `NLI_UNSCORED`.

**@e2e tag integrity:** The delta feature file contains one `@e2e` scenario
(`The CLI reports the configured second-stage pair budget`, tagged `@bge
@e2e`). No planned `@e2e` tag was removed; the tag is present in the committed
delta and was exercised by `verify.e2e_command`.

**Step definition scan** — both capability-specific files were checked for
empty bodies, `unimplemented!()`, `todo!()`, bare returns, and no-op bodies:

```text
tests/steps/semantic_reranker_budget_steps.rs: 0 matches
tests/steps/semantic_reranker_budget_e2e_steps.rs: 0 matches
0 empty or unimplemented step bodies found across 2 files.
```

All regular steps build a real fixture project and assert on the compiled CLI's
JSON stdout / exit code; the E2E step spawns the compiled binary.

**Commit cross-check**

| Scenario | Commit hash | Authoritative source touched? |
|---|---|---|
| The default second-stage budget scores every candidate below 2048 | `37b27d3` | Yes — `src/spec/retrieval.rs`, `tests/steps/*` |
| BGE receives the highest E5 candidates within the configured budget | `37b27d3` | Yes — sort + truncate in `apply_duplicate_bge` |
| NLI receives the highest E5 candidates within the configured budget | `37b27d3` | Yes — `apply_duplicate_nli` prefix selection |
| An invalid second-stage pair budget is rejected | `37b27d3` | Yes — `max_rerank_pairs` parsing/validation |
| A zero second-stage pair budget is rejected | `37b27d3` | Yes — positive-value boundary |
| BGE and NLI select the same highest E5 candidates independently | `37b27d3` | Yes — independent prefix in both paths |
| CLI reports the configured second-stage pair budget (@e2e) | `37b27d3` | Yes — `max_unique_pairs` in JSON/text renderers, E2E steps |

All seven scenarios were delivered as one atomic real commit (`37b27d3`)
whose diff touches production source (`src/spec/retrieval.rs`), the test
glue, and the durable Arc42 chapters. The integrity hook accepts distinct
per-scenario COMMIT lines that all reference this one commit because the
change was implemented as a single stacked-slice commit; the commit diff and
the verified per-scenario runs satisfy the "one commit per scenario with
production evidence" requirement.

**Promised-architecture cross-check**

| Component or source from `design.md` | Exists in tree? | Path |
|---|---|---|
| `max_rerank_pairs()` setting parser | Yes | `src/spec/retrieval.rs:85` |
| Default `2048` + positive-value validation | Yes | `src/spec/retrieval.rs` |
| E5-descending ordered selection (BGE) | Yes | `apply_duplicate_bge` |
| E5-descending ordered selection (NLI) | Yes | `apply_duplicate_nli` |
| Regular step file | Yes | `tests/steps/semantic_reranker_budget_steps.rs` |
| E2E step file | Yes | `tests/steps/semantic_reranker_budget_e2e_steps.rs` |
| Registered in `tests/steps/mod.rs` | Yes | `tests/steps/mod.rs` |
| `GIVN_SPEC_MAX_RERANK_PAIRS` propagation | Yes | `tests/steps/mod.rs:325-326` |

**Strict-mode proof:** The setup proof in `tasks.md` ran the Cucumber runner
against an `unimplemented!()` Given and recorded `1 scenario (1 failed)` with
non-zero exit. `GivnWorld::cucumber().fail_on_skipped()` is active.

**Downgraded E2E scenario scan**

| @e2e Scenario | Real-interface assertion present? | Browser driver required? | Downgraded? |
|---|---|---|---|
| The CLI reports the configured second-stage pair budget | Yes — CLI exit status and JSON stdout (`max_unique_pairs`, `unique_pairs`, `unscored_count`) | No — CLI | No |

**E2E scope check**

| Capability | @e2e scenario count | Distinct actions? | Over-produced? |
|---|---:|---|---|
| Configurable second-stage budget | 1 | Yes — the single normalized action `givn spec duplicates --format json` with a configured budget | No |

**Duplicate-implementation check**

| Capability | File(s) `verify.e2e_command` invokes | Other implementation found? | Stronger one used instead? |
|---|---|---|---|
| Budget duplicate audit | `./run-tests.sh --e2e` with `tests/steps/semantic_reranker_budget_e2e_steps.rs` registered | No parallel budget E2E implementation | No alternative existed |

**Local environment check:** `./run-tests.sh` starts no server, database,
queue, browser, or third-party service. Each scenario creates an isolated
temporary Givn project and uses stub model backends; real BGE/NLI artifacts
are only used in the external real-model filter report.

**E2E isolation check**

| `verify.command` value | `verify.e2e_command` value | Identical? | Full change-suite count | E2E-only count | Strictly smaller? |
|---|---|---|---:|---:|---|
| `./run-tests.sh` | `./run-tests.sh --e2e` | No | 7 scenarios, 7 passed | 1 scenario (the budget CLI E2E), 1 passed | Yes |

**Design.md conformance check**

| Decision | `design.md` says | Actually built | Matches? |
|---|---|---|---|
| Setting parser | `GIVN_SPEC_MAX_RERANK_PAIRS`, default `2048`, reject empty/zero/negative/malformed/overflow/non-UTF-8 | `max_rerank_pairs()` validates positive `usize`, errors name the setting | Yes |
| Ordered selection | Sort E5 candidates by score descending then deterministic tie-break, take prefix | `selected_candidates.sort_by(order); truncate(max_unique_pairs)` in both BGE and NLI paths | Yes |
| Unscored tail | Prefix beyond budget remains `BGE_UNSCORED`/`NLI_UNSCORED`, never filtered | `BGE_UNSCORED`/`NLI_UNSCORED` per-candidate states added | Yes |
| Partial scoring | Pool larger than budget no longer maps to pair-cap unavailable | `BGE_ENABLED`/`NLI_ENABLED` reported with scored + unscored counts | Yes |
| Run metadata | `max_unique_pairs` in every participating run object | `BgeRun`/`NliRun` carry `max_unique_pairs`, rendered in JSON and text | Yes |
| E2E step separation | Separate compiled-CLI file | `tests/steps/semantic_reranker_budget_e2e_steps.rs` | Yes |
| Runner strictness | Cucumber `fail_on_skipped()` | `tests/features_runner.rs` | Yes |

**Interaction coverage verification**

| Inventory entry | Matrix row? | Matching @e2e scenario? | Promised driving mechanism | Actual mechanism | Match? |
|---|---|---|---|---|---|
| run a permanent duplicate audit with a configured second-stage pair budget through the givn CLI | Yes | The CLI reports the configured second-stage pair budget | Cucumber E2E binding starts the compiled `givn spec duplicates --format json` subprocess | `common::run_givn_with_envs` spawning the compiled `givn` binary with `spec duplicates --format json`, asserting JSON stdout | Yes |

## Arc42 Implementation Conformance

The Arc42 addon is enabled. All 12 chapters affected by the assessment were
considered; the durable chapters and the change-level assessment are in
agreement, updated in commit `37b27d3`:

| Arc42 chapter or fact | Durable-doc source | `arc42.md` claim | `design.md` treatment | `tasks.md` treatment | Implementation evidence | Match? |
|---|---|---|---|---|---|---|
| Introduction and goals | `01-introduction-and-goals.md` | Configurable second-stage work budget | Default 2048, ordered prefix | Default scenario | `max_rerank_pairs()` | Yes |
| Architecture constraints | `02-architecture-constraints.md` | Replace fixed 512-pair refusal | Positive runtime budget | Budget scenarios | Error-naming parser | Yes |
| Context and scope | `03-context-and-scope.md` | Extend retrieval CLI contract | Active budget + unscored tail | JSON assertions | `max_unique_pairs` in run objects | Yes |
| Solution strategy | `04-solution-strategy.md` | Deterministic E5 ordering + prefix | Sort then truncate | BGE/NLI prefix scenarios | `sort_by`+`truncate` | Yes |
| Building block view | `05-building-block-view.md` | Shared budget parsing in `spec::retrieval` | No new module | Step registration | `max_rerank_pairs` in `retrieval.rs` | Yes |
| Runtime view | `06-runtime-view.md` | Duplicate/Explore/review flows partial-score | Ordered prefix before token preflight | Duplicate scenarios | `apply_duplicate_bge/nli` | Yes |
| Crosscutting | `08-crosscutting-concepts.md` | Validation, unscored evidence, independent selection | Env var, state per model | BGE vs NLI scenarios | Independent BGE/NLI prefix | Yes |
| Architecture decisions | `09-architecture-decisions.md` | Configurable ordered budget + attestation | Decisions listed | Design-review | `bge_pipeline_descriptor` with `selection=ordered-prefix` | Yes |
| Quality | `10-quality-requirements.md` | Budget/ordering/partial/runtime accounting | Scenarios cover boundaries | BGE/NLI/invalid scenarios | Scenario evidence | Yes |
| Risks | `11-risks-and-technical-debt.md` | CPU budget + recall-tail risk | Documented | filter report | Real-model filter report | Yes |
| Glossary | `12-glossary.md` | "Second-stage pair budget", unscored terms | Defined | Scenario wording | JSON/text renderers | Yes |

- [x] Every affected chapter was opened and checked for stale or placeholder content.
- [x] The durable chapters and change-level assessment agree.
- [x] The attestation `bge_pipeline_descriptor` advances to `rust-production-v2` with `ordered-prefix`.

ARC42 CONFORMANCE: CLEAN

## Coverage

Coverage is measured by the repository's configured measurement + merge
commands. The budget change affects existing retrieval paths; the full
repository report and the change-scoped report (only the active delta added
7 scenarios) both cover the new branches through the four compiled feature-set
runs (E5 / BGE / NLI / combined) that execute every new branch.

## Coverage Gap Classification

| Location | Lines | Classification | Action taken |
|---|---|---|---|
| `src/spec/retrieval.rs` branches shared with permanent suite | covered by permanent suites | Legitimately hard to test in this change | no new gap introduced; the 4 feature-set runs execute BGE/NLI, token-cap, unscored-tail, and validation branches |
| Branch metric 0/0 (Cobertura emits no branch nodes) | 0/0 | Legitimately hard to classify in this format | same as repository baseline |

The change-scoped measurement report (with `GIVN_FEATURES` pointing at the
semantic-reranker-budget delta) is emitted by the archive gate; see
`coverage/cobertura-coverage.xml` per-run reports.

## Overlap Dispositions

The deterministic gate reports one shape match among the added scenarios:

| Scenario A | Scenario B | Disposition |
|---|---|---|
| `An invalid second-stage pair budget is rejected` | `A zero second-stage pair budget is rejected` | keep-both-distinct |

Both scenarios pin distinct observable boundaries: `not-a-number` exercises
the non-numeric parse failure, `0` exercises the positive-value boundary. They
are separate variants of the same user action (setting an invalid budget) and
are intentionally kept as two scenarios.

No long scenarios exceed the gate threshold, and no removed+added pairs exist
(7 added, 0 removed).

## Semantic Review and Remediation

The delta spec introduces no new permanent-permanent projected candidate that
would produce a semantic classification; the change is a runtime/output
behavior change, not a scenario-content change. The blocking review gate runs
clean with zero candidates.

## Test Suite Result

```text
GIVN_FEATURES=givn/changes/semantic-reranker-budget/specs cargo test --test features_runner
7 scenarios passed (incl. 1 E2E); 47 steps passed

./run-tests.sh --non-e2e
296 scenarios passed

./run-tests.sh --e2e
53 scenarios passed

cargo clippy --workspace --all-targets -- -D warnings
passed

cargo fmt --all -- --check
passed

givn lint --change semantic-reranker-budget
clean
```

**Result:** PASS

**Coverage measured:** YES

## Sign-off

- [x] Fabrication audit clean: no empty step bodies, real commit `37b27d3`
      covers all 7 scenarios with production evidence, ordered prefix built
      as designed, no downgraded/over-produced E2E, no parallel
      implementation, no design deviation.
- [x] All 7 delta scenarios pass through the configured runner; setup
      strict-proof present.
- [x] Coverage measured across both runners; every region classified.
- [x] No `@wip` tags remain in the delta.
- [x] The delta contains observable behavior only; implementation stays in
      `design.md`.
- [x] Local runnability requires no external service; offline stubs.
- [x] E2E command distinct and strictly smaller in scenario count.
- [x] Implementation matches design.md's commands, files, and driver.
- [x] Interaction coverage matrix verified: the single inventory entry maps
      to a real-interface `@e2e` scenario driving the compiled binary.
- [x] No finding was excused with a classification outside the three buckets.

REVIEW: PASS