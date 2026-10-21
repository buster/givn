# Review: fix-outline-report-fidelity

Fresh-context re-review of the committed candidate at `HEAD` = `656c1d58`
against the proposal, delta spec, design, design-review, tasks, arc42, the
owning use case, and the durable specs. The previous review ended
`REVIEW: FAIL` on the two coverage findings R1 and R2. Commit `656c1d58` adds
the two missing regression tests and corrects the design sentence about the
qself type. This review re-read the code, the tests, and the documentation,
re-ran both new tests and both gates, and re-checked the audit below.

## Fabrication Audit (do this first)

**Step definition scan** — every step definition file for this capability was
read and mechanically scanned for empty/no-op bodies (`{}`, bare `pass`, bare
`return`, `unimplemented!`, `todo!`, `#[ignore]`, or no assertion reference):

```
Files scanned:
  tests/steps/outline_report_steps.rs       1455 lines, 129 bindings
  tests/steps/outline_report_e2e_steps.rs     56 lines,   2 bindings
  tests/steps/outline_report_support.rs      583 lines, fixtures/constants only
0 empty step bodies found across the 2 step definition files.
0 unimplemented!/todo!/#[ignore] matches in step bodies. The only todo!()
match is inside the generated Rust fixture text at
outline_report_support.rs:581, not in a step body.
Every delta Then asserts a concrete observation: an exact method id, an exact
resolution label, an exact unit/file metric number, an exact evidence rule, or
an indent comparison on the printed tree.
```

**Commit cross-check** — every checked `[x] Scenario:` in tasks.md has one
reachable revision whose message body names the scenario; `givn check tasks`
validates all 20 and exits 0:

| Scenario | Commit | Touches production code or regression test? |
|---|---|---|
| A method call on an unknown receiver is a name-only guess | b1fffc9e | production: 13 source files + tests |
| A name-only guess is not expanded as a same-unit call | 8144cca8 | regression test (production landed in b1fffc9e) |
| A call on the unit's own receiver resolves within the unit | 1b37904d | regression test |
| A method call on a named receiver resolves to that owner | d3b72a13 | regression test |
| An ambiguous method name has no resolved target | ea3d0143 | regression test |
| An associated-function call whose qualifier contradicts the target is external | 43cee9c6 | regression test |
| A god type found through cohesion reports only the cohesion evidence | 9e3487b1 | regression test; classification branch landed in b1fffc9e |
| An interface found through its public methods reports only public-method evidence | 59b2188f | production: `src/outline/classify.rs` |
| A long method found through complexity reports only complexity evidence | 2a98d7fc | regression test |
| A Rust module outline lists the types it declares | f7e7e526 | regression test |
| A module outline interleaves member types and free functions in source order | e5e08c8f | regression test |
| A Python module outline lists its classes | b8ede9d9 | regression test |
| The JSON report names each call's resolved target | 631f483d | regression test; `target` landed in b1fffc9e |
| A same-unit method implemented in another file is project_other_file | 21b433b9 | regression test |
| A developer reads each call's resolution in the human report | 240bdaad | regression test |
| A file's code lines exclude inline test modules | 887b990d | regression test |
| A unit's physical lines cover only its own spans | bda7ebdc | regression test |
| An impl-level test method is excluded from the unit's lines | ee1f69da | regression test |
| A name-only guess does not connect methods for cohesion | b5554de3 | production: `src/outline/metrics.rs` + tests |
| A method signature keeps arrays, qualified paths, and its full type detail | db5b7e6d | regression test |

The recorded revisions are reachable and each message body is the scenario
title. The scenario-14 hash is now the real
`21b433b9390d0b2031781877803990280ca42b2e`; the previous invalid
`21b433b939fbc8b97b7de2e55ffe4cc07ba6b520` is gone. Observation: 17 of the 20
commits carry only their regression test, because the production
implementation for those scenarios landed in b1fffc9e; the RED evidence for
those scenarios proves the missing step binding, not the missing production
behaviour. The project rule in `AGENTS.md` allows the scenario's implementation
or its regression test, and the tests fail if the behaviour regresses, so this
is recorded as an observation, not a fabrication finding. Final step-file
quality is real: concrete assertions, real fixtures, and a real subprocess for
the CLI scenarios.

**Previous findings disposition**

| Previous finding | Status after 656c1d58 | Evidence |
|---|---|---|
| F1 — multi-segment external qualifier reported as same_file | remediated | `src/outline/resolve.rs:197-222`; `resolution_requires_shape_consistent_owners` at `:471` |
| F2 — Python named receiver lost its unique candidate | remediated | `src/outline/resolve.rs:140-153`; same test asserts a qualifier with no matching owner becomes `name_only` |
| F3 — Python `ast.unparse` callee fallback | remediated | `src/outline/python_adapter/extractor.py:19-51`; `a_computed_callee_is_not_invented` at `src/outline/python_adapter/mod.rs:420` |
| F4 — qself call used the qself type | remediated | `src/outline/syn_adapter/facts.rs:131-142`; `tests::qself_uses_the_trait_owner` at `src/outline/syn_adapter/facts.rs:321` asserts both `<Widget as Store>::make()` → `Store` and `<Widget>::run()` → `Widget` |
| F5 — promised resolver and gate regression tests absent | remediated | type-only gate test at `tests/outline_gate_smoke.rs:611`; free-function gate test `a_module_with_free_functions_still_needs_a_verdict` at `tests/outline_gate_smoke.rs:635`; qself test at `src/outline/syn_adapter/facts.rs:321` |
| F6 — durable docs and documentation plan diverged | remediated | chapters 01, 03, 04, 05, 06, 08, 10, 11, 12, arc42 README, use case updated; R-115 extended, R-122 added |
| F7 — task record not machine-checkable; invalid hash | remediated | `givn check tasks --change fix-outline-report-fidelity` exits 0 with 20 scenarios; hashes corrected |

**Promised-architecture cross-check** — every component, file, and test target
design.md promised exists, including the two regression tests that R1 and R2
previously reported as missing:

| Component or file (from design.md) | Exists? | Path |
|---|---|---|
| `CallShape`, `ReceiverEvidence`, `Call.shape`, `target`, `AnalyzedFile.excluded_spans` | yes | `src/outline/model.rs` |
| Shape-consistent resolver | yes | `src/outline/resolve.rs` |
| Rust call-shape extraction | yes | `src/outline/syn_adapter/facts.rs` |
| Recursive type renderer | yes | `src/outline/syn_adapter/syntax.rs` |
| Rust excluded-test spans and module type items | yes | `src/outline/syn_adapter/file_builder.rs` |
| Python call-shape extraction | yes | `src/outline/python_adapter/extractor.py` |
| Python call-shape mapping and class items | yes | `src/outline/python_adapter/mod.rs` |
| Per-predicate evidence | yes | `src/outline/classify.rs` |
| Span-scoped metrics and evidenced LCOM4 | yes | `src/outline/metrics.rs` |
| Production line helpers | yes | `src/outline/scan.rs` |
| JSON `target`, resolution tag, non-nesting `name_only` | yes | `src/outline/report.rs` |
| Method-bearing touched-unit selection | yes | `src/outline/gate.rs:569` |
| Gate regression test for a type-only module | yes | `tests/outline_gate_smoke.rs:611` |
| Gate regression test for a module with free functions | yes | `tests/outline_gate_smoke.rs:635` |
| Shape-row tests, including qself | yes | `src/outline/resolve.rs` resolver tests cover the shape-consistent rows; the qself shape row is covered at `src/outline/syn_adapter/facts.rs:321` |
| Regular bindings, fixtures, e2e file | yes | `tests/steps/outline_report_steps.rs`, `outline_report_support.rs`, `outline_report_e2e_steps.rs` |
| Durable docs and the owning use case | yes | `docs/outline.md`, `docs/arc42/*`, `givn/specs/assess-code-structure/usecase.md` |

`src/outline/syn_adapter/spans.rs` was also modified; it is an existing
attribute-aware span helper and an extra file, not a missing component.

Contract check: the consumers (`src/cli/outline.rs`, `src/outline/gate*`,
`report`, `classify`, `metrics`) use the neutral outline API and the report
renderer; none imports `syn_adapter` or `python_adapter`. The only adapter
imports are in the composition root (`src/outline/analyzer.rs:13,26,37`) and
the module declarations. No consumer imports an adapter implementation.

**Strict-mode proof** — present with output in the setup task
(`tasks.md:72-86`): `tests/features_runner.rs:55,151` call
`.fail_on_skipped()` on both execution paths, and the plain path exits 1 when
`stats.failed > 0 || stats.skipped > 0` (`:64`). The undefined-step probe
produced `1 scenario (1 failed)`, `Step doesn't match any function`, exit
status 1. The all-`@wip` baseline produced `20 scenarios (20 failed)`, exit 1.
Pending is not green.

**E2e tag integrity and downgraded e2e scan** — the change adds no `@e2e`
scenario. The capability keeps its one permanent `@e2e` scenario at
`givn/specs/assess-code-structure/outline-report.feature:3`:

| @e2e Scenario | Real-interface assertion present? | Driven via real interface? | Downgraded? |
|---|---|---|---|
| A developer runs a outline report and sees the classified units (permanent) | yes: stdout classification `god-type`, `HINT` label, no recommendation, no baseline file | real `givn` subprocess through `common::run_givn`/`user_runs_quoted` (`tests/steps/outline_report_steps.rs:15-18`) | no |

No `@e2e` tag was removed from the committed delta to bypass a gate: the first
committed version of the delta feature (`b1fffc9e`) and the current version
contain no `@e2e` tag. Design-review Q1 and withdrawn D8 record that a planned
second e2e was withdrawn as a duplicate of the same action; the permanent e2e
for that action still runs in the `--e2e` scope. This is a design-review
withdrawal, not a tag removal.

**E2e scope check**

| Capability | @e2e scenario count | Distinct actions? | Over-produced? |
|---|---|---|---|
| `outline-report` | 1 permanent (delta adds 0) | 1: run a outline report | no |

**Duplicate-implementation check** — `verify.e2e_command` is
`./run-tests.sh --e2e` (`givn/commands.yaml:4`), which runs `features_runner`
with `@e2e and not @givn.removed` (`run-tests.sh:71-72`). For this capability
the e2e bindings are in `tests/steps/outline_report_e2e_steps.rs`; the
Given/When steps are shared from `outline_report_steps.rs` and also drive the
real binary. `outline_review_gate_e2e_steps.rs` belongs to the different
`outline-review-gate` capability. `git status` shows only the generated
`verification/` receipt, no untracked e2e implementation. No parallel or
stronger outline e2e implementation exists.

| Capability | File(s) the e2e command invokes | Other implementation found? (incl. untracked) | Stronger one used instead? |
|---|---|---|---|
| `outline-report` | `tests/steps/outline_report_e2e_steps.rs` + shared Given/When in `outline_report_steps.rs` | none | n/a |

**Local environment check** — the design's local run command is
`cargo run -- outline report` in this checkout; the product is a CLI with no
service, database, container, or digital twin. It exits 0 and prints
`outline report — python+rust, 93 files, 334 units, 20 hints`, followed by the
outline-first report. Two consecutive runs produced identical output.

**E2e isolation check** — values read literally from `givn/commands.yaml`:

| `verify.command` value | `verify.e2e_command` value | Identical? | Regular scope count | E2e scope count | E2e strictly smaller? |
|---|---|---|---|---|---|
| `./run-tests.sh --non-e2e` | `./run-tests.sh --e2e` | no | 686 (implementer-reported pre-archive run) | 111 (implementer-reported pre-archive run) | yes |

No archive verification receipt exists yet; the counts above are the
implementer-supplied pre-archive runs recorded in `tasks.md`, not receipt
evidence. The tag filters in `run-tests.sh:71-75` are complementary
(`not @e2e` versus `@e2e`), so the scopes are disjoint. The archive receipt
will carry the measured per-scope result.

**Git tree status** — `git status --porcelain` was empty when this review
started at `656c1d58`: no untracked or modified file was present. After the
review runs it reports two entries:

```
 M givn/changes/fix-outline-report-fidelity/review.md
 M givn/changes/fix-outline-report-fidelity/verification/tasks-verification.json
```

The first is this review record. The second is the generated tasks receipt,
refreshed by the mandated `givn check tasks` run; it now names `656c1d58`
instead of the older `203f57ad`. Both are generated or review artefacts, not
production, test, spec, design, or task work. The caller must commit both
before the archive gate. The earlier `verification/` directory is tracked
since `656c1d58`, not untracked.

**Design.md conformance check**

| Decision (from design.md) | design.md says | Actually built | Matches? | Deviation resolved? |
|---|---|---|---|---|
| Verification commands | `./run-tests.sh --non-e2e` / `--e2e` | identical | yes | n/a |
| Runner/framework | existing cucumber-rs runner; `GIVN_FEATURES` for delta runs; anchored `--name` | unchanged `tests/features_runner.rs`, `run-tests.sh` | yes | n/a |
| Step files | regular bindings in `outline_report_steps.rs`; e2e file unchanged | same two files; e2e Given/When shared | yes | n/a |
| Files and modules table | named production files and docs | all exist; extra existing `syn_adapter/spans.rs` | yes (extra helper) | note only |
| Gate selection | `unit.metrics.methods > 0` | `!unit.method_names.is_empty()` (`gate.rs:569`); `method_names` is built from `unit.methods` (`classify.rs:163-168`) and `metrics.methods` is `unit.methods.len()` (`metrics.rs:147`) | yes, equivalent | n/a |
| Gate regression test | design.md:185 and :368 promise a test for a module with only type items **and** a module with free functions | both tests exist: `a_module_with_only_type_items_needs_no_verdict` at `tests/outline_gate_smoke.rs:611` and `a_module_with_free_functions_still_needs_a_verdict` at `:635`; the free-function test asserts `!passed` and `MISSING outline module:src/free.rs` | yes | R1 resolved |
| Resolver test | design.md:186 promises a table test for every shape row; the shape table includes qself | resolver tests cover the qualifier-prefix and Python-convention rows; `qself_uses_the_trait_owner` at `src/outline/syn_adapter/facts.rs:321` covers the qself shape row at the adapter, asserting `Store` for the as-trait form and `Widget` for the inherent form | yes, with the qself row covered at the adapter | R2 resolved |
| Multi-segment qualifier (`std::path::Path`) is external | normative rule | `function_qualifier_matches` rejects a multi-segment qualifier unless rooted at `crate`/`self`/`super`; test asserts `std::path::Path` is external and `crate::line::Line` can match | yes | n/a |
| Python `Qualifier` with no matching owner → unique candidate `name_only` | normative rule | `resolve.rs:140-153` returns `name_only_or_ambiguous` for a qualifier with no compatible owner | yes | n/a |
| Python computed callee is not recorded | normative rule | `extractor.py:19-51` returns `None` for a non-`Name`/`Attribute` callee; test asserts no `make()` or `<call>` name | yes | n/a |
| qself uses the trait owner segment before the callee | normative shape table; design.md:238 now scopes the fallback: the last segment before the callee for `<T as Trait>::method()`, the qself type `T` for the inherent `<T>::method()` form | `facts.rs:131-142` takes the last segment before the callee when `qself.position > 0` and the qself type at position 0; `tests::qself_uses_the_trait_owner` asserts both forms | yes | R2 resolved |
| Schema 3 / rules 1 | unchanged | `outline_json` schema 3, rules 1 | yes | n/a |
| Documentation plan | chapters 01, 03, 04, 05, 06 §6.19, 08, 10, 11, 12, arc42 README, usecase, outline.md, README | all updated | yes | n/a |

**Interaction coverage verification**

| Inventory entry | Matrix row exists? | @e2e scenario exists? (title) | Promised driving mechanism | Actual driving mechanism | Match? |
|---|---|---|---|---|---|
| `outline-report` — run a outline report | yes (design §Interaction coverage matrix) | A developer runs a outline report and sees the classified units (permanent) | real `givn` subprocess | `common::run_givn` via `user_runs_quoted` in `tests/steps/outline_report_steps.rs:15-18` | yes |

## Use-Case And Persona Conformance

| Contract | Evidence | Pass? |
|---|---|---|
| Owning use-case guarantees and rules preserved | `givn/specs/assess-code-structure/usecase.md` goal, success guarantee, minimal guarantee, actors, capabilities, and interactions are unchanged; the rules now state that a module outline lists declared member types, calls carry resolution labels, and an unknown receiver stays a visible `name_only` leaf | yes |
| Capability ownership and typed relationships preserved | `EXTEND outline-report` under `assess-code-structure`; `Includes`/`Extends` remain `none`; no capability is moved or renamed | yes |
| Confirmed Persona-relevant outcomes preserved | `developer--primary-user` (trustworthy labels and signatures), `agent--technical-role` (JSON `target`, production counts), `givn-maintainer--owner` (auditable shape-aware resolver) | yes |
| Actors remain distinct from Persona biographies | Gherkin names "the developer" and "the report"; Personas remain review lenses and are not embedded in scenarios | yes |
| Migration ledger and coverage evidence complete, if applicable | no migration; no archive receipt yet, so coverage is classified statically below | n/a |
| Use-case plan item: state that a module outline lists declared member types | `usecase.md:60-65` now records the member-type statement and the resolution-label/`name_only` rule | yes |

PERSONAS: developer--primary-user

The other confirmed Personas (`agent--technical-role`,
`givn-maintainer--owner`) are confirmed in the owning use case and are used as
review lenses; no Persona is invented or promoted by this change.

## Deferred mandatory checks

None. No `DEFERRAL:` or `OPERATOR-DECISION:` pair is present because no
mandatory check was deferred.

## Visual Review

The design declares `Interface: terminal`, viewports not applicable, and no
transcript committed for this delta (design §Visual Design Contract). Opposite
classification considered: a report rendered as a page would be a Web UI, but
this surface prints plain text from a CLI, emits no HTML/CSS/JavaScript, and
involves no browser or page context; the Web UI driver and rendered-state
rules therefore do not apply. The capability's permanent e2e keeps its
pre-existing evidence path; the delta adds no `@e2e` scenario, so there is no
new evidence file to commit.

| @e2e Scenario | Viewport | Evidence file | Present and non-empty? |
|---|---|---|---|
| None added by this delta; the permanent `A developer runs a outline report and sees the classified units` is unchanged | terminal | no new transcript; the live `cargo run -- outline report` output was inspected against the contract | n/a for this delta |

The human surface was reviewed against live output of
`cargo run -- outline report` on this checkout. The observed tags reproduce the
design's rendered reference form:
`-> [same_file, private, Engine] validate`,
`-> [project_other_file, public, module] normalize`, and
`-> [name_only, public, UsecaseDiagnostics] is_empty`. The live report
contained 2650 `name_only` tags and 979 `ambiguous` tags; no `name_only` line
was followed by a deeper-indented call line, while evidenced same-unit calls
did nest. Two consecutive runs were byte-identical.

| Confirmed Persona | Goal | Finding | Disposition |
|---|---|---|---|
| `developer--primary-user` | Read a call tree whose certainty labels are trustworthy | Tags lead with the resolution label, visibility, and owner; the tree reads in one pass; `name_only` stays a visible leaf | no finding |
| `agent--technical-role` | Use JSON `target` without re-deriving it | JSON call objects carry `resolution` and `target`; a `name_only` target is explicitly a candidate | no finding |
| `givn-maintainer--owner` | Audit resolver evidence through one neutral model | Tags distinguish same-file, cross-file, name-only, and ambiguous edges; the resolver and adapters are shape-neutral | no finding (the former R1/R2 regression-test gaps are closed and verified below) |

**Baseline:** every call tag leads with a domain resolution label
(`same_file`, `project_other_file`, `name_only`, `ambiguous`), every line has
the callee, no internal identifier replaces a label, output is line-oriented
and deterministic, the header names the report, no visible content is
duplicated, the output is English, and no motion exists. A designed error
state and reduced-motion are not applicable to a text report. The delta adds no
`@e2e` scenario, so the integrity check has no delta transcript to require;
the permanent e2e's pre-existing path is unchanged.

VISUAL-REVIEW: PASS

## Retirements

N/A. This change retires no capability.

## Arc42 implementation conformance

The affected chapters were independently derived from the completed
implementation (resolver, adapters, report, gate, metrics, classify), the
design, and the tasks, then compared with the durable chapters and `arc42.md`.
All chapters named by either assessment were opened and checked for stale,
contradictory, or placeholder content.

| Arc42 chapter or architectural fact | Durable-doc source | `arc42.md` claim | `design.md` treatment | `tasks.md` treatment | Completed implementation evidence | Match? / finding |
|---|---|---|---|---|---|---|
| FR-46 fidelity guarantee: resolution labels and JSON targets | `docs/arc42/01-introduction-and-goals.md:65` | chapter 01 affected; FR-46 states labels and targets | documentation plan names FR-46 | final verification docs task | FR-46 text updated; `src/outline/report.rs` renders labels and targets | match |
| `givn outline report` interface exposes resolution labels and targets | `docs/arc42/03-context-and-scope.md:62` | chapter 03 affected; interface row updated | documentation plan names the interface row | final verification docs task | chapter 03 text; live report and JSON | match |
| Shared resolution is shape-aware, not name-based | `docs/arc42/04-solution-strategy.md:25` | chapter 04 affected; code-structure report row updated | D1/D2, solution strategy | tasks 1-6, 14, 15 | `src/outline/resolve.rs`; live tags | match |
| `outline::resolve` and `outline::gate` building blocks | `docs/arc42/05-building-block-view.md:79-80` | chapter 05 affected; resolver and gate rows updated | D1, D2, D4, files table | tasks 1-6, 10-12, 14, 15 | resolver, gate selection (`gate.rs:569`), 34-outline gate output | match |
| Runtime §6.19: shape resolution, module member types, JSON `target`, production line accounting | `docs/arc42/06-runtime-view.md:784+` | chapter 06 affected; §6.19 updated | D1-D7, runtime flow | tasks 1-20 | chapter 06 text; live report; JSON | match |
| Crosscutting §8.26: call-shape resolution, `name_only` leaf, production accounting | `docs/arc42/08-crosscutting-concepts.md:1090+` | chapter 08 affected; outline concept updated | D1, D2, D4-D6 | tasks 1-20 | chapter 08 text; resolver/report/metrics | match |
| Q-119 call-tree fidelity includes `name_only` and resolution labels | `docs/arc42/10-quality-requirements.md:123` | chapter 10 affected; Q-119 updated | D2, D5 | tasks 13, 15, 19 | chapter 10 text; report output | match |
| R-106/R-121 shape-aware evidence; R-115 additive `target`; R-122 target-misuse risk | `docs/arc42/11-risks-and-technical-debt.md:90,102-103` | chapter 11 affected; R-106/R-121 and the guess-as-resolved risk summarized | D1, D2, D5 | final verification docs task | chapter 11 text has R-106, R-115, R-121, R-122 | match (the change-level summary does not repeat the R-115/R-122 ids, but the durable chapter carries both and the summary prose names the risk) |
| Glossary: Outline, Call tree, Outline report, Source unit, Production source, Call shape, Call resolution | `docs/arc42/12-glossary.md:212-214, 225-226, 232-233` | chapter 12 affected; entries added or updated | glossary plan | final verification docs task | glossary text | match |
| Documentation status log names this change and its durable facts | `docs/arc42/README.md` (status log, end) | `arc42.md` does not name the README | documentation plan names it | final verification docs task | README entry added | match |
| Owning use case states module member types and resolution semantics | `givn/specs/assess-code-structure/usecase.md:60-65` | not an arc42 chapter; design documentation plan names it | documentation plan | final verification docs task | use-case rules updated | match |
| No ADR qualified | `docs/arc42/09-architecture-decisions.md` | `NOT_QUALIFIED` | ADR verdict with `CANONICAL_ARTIFACT` → `design.md` | n/a | no ADR added or amended | match |

- [x] Every chapter marked affected by either the independent review or
      `arc42.md` was opened and checked for stale, contradictory, or placeholder
      content.
- [x] Every Arc42 fact or architecture decision relevant to this change is
      represented in `design.md` and mapped to one or more tasks in
      `tasks.md`.
- [x] The completed implementation follows those facts and decisions; the
      evidence names the built file, command, test, or observable result.
- [x] Architecture changes discovered in the completed implementation are
      recorded in the durable Arc42 chapter and the change-level assessment.
- [x] No mismatch remains; the durable chapters, `arc42.md`, `design.md`, and
      `tasks.md` agree.

ARC42 CONFORMANCE: CLEAN

## Ubiquitous language conformance

The durable glossary now holds **Call shape** and **Call resolution**
(`docs/arc42/12-glossary.md:213-214`) and the updated **Outline**, **Call
tree**, **Outline report**, **Source unit**, and **Production source** entries.
The delta spec, design, implementation, and report use the same terms:
`same_file`, `project_other_file`, `name_only`, `ambiguous`, `dynamic`, and
`external`; "call shape" is the adapter fact, "call resolution" is the label.
No spec, design, or code term introduced by this change is missing from the
glossary, and no inconsistent reuse was found. The use case's "name_only leaf"
and the report's "[name_only, ...]" tag use the same vocabulary.

## Coverage

No archive verification receipt exists for this change yet, so there is no
measured coverage result. This is an unproven candidate; the classification
below comes from static inspection, the delta scenarios, and the targeted
tests run during this review. The archive receipt will carry the measured
result.

```
unproven candidate — no givn/archive/fix-outline-report-fidelity/verification.json
exists yet. Static inspection and the committed tests are the only evidence.
```

## Coverage gap classification

| Location | Lines | Classification | Action taken |
|---|---|---|---|
| `src/outline/resolve.rs::function_qualifier_matches` multi-segment qualifier branch | 197-222 | no gap: covered by `resolution_requires_shape_consistent_owners` and `a_crate_rooted_qualifier_can_match_a_project_type` | verified by targeted `cargo test --lib outline::resolve` (3 passed) |
| `src/outline/resolve.rs` Python `Qualifier` fallback | 140-153 | no gap: covered by the same resolver test (a qualifier with no matching owner becomes `name_only`) and the Python mapping in `python_adapter/mod.rs` | verified by the same test |
| `src/outline/python_adapter/extractor.py` non-`Name`/`Attribute` callee | 19-51 | no gap: covered by `a_computed_callee_is_not_invented` (1 passed) | targeted test |
| `src/outline/syn_adapter/facts.rs` qself owner derivation | 131-142 | no gap: covered by `tests::qself_uses_the_trait_owner` at `src/outline/syn_adapter/facts.rs:321`, which asserts `Store` for `<Widget as Store>::make()` and `Widget` for `<Widget>::run()` | targeted `cargo test --locked --lib outline::syn_adapter::facts` (2 passed) |
| `src/outline/gate.rs` type-only module selection | 569 | no gap: covered by `a_module_with_only_type_items_needs_no_verdict` (1 passed) | targeted test |
| `src/outline/gate.rs` module with free functions | 569 | no gap: covered by `a_module_with_free_functions_still_needs_a_verdict` at `tests/outline_gate_smoke.rs:635`, which asserts `!passed` and `MISSING outline module:src/free.rs` for a touched free-function module | targeted `cargo test --locked --test outline_gate_smoke a_module_with_free_functions_still_needs_a_verdict` (1 passed) |
| `src/outline/resolve.rs` `Method { SelfUnit }` row | 111-123 | no gap: the delta scenario `A call on the unit's own receiver resolves within the unit` covers it through the real subprocess; a unit test would duplicate the black-box scenario | delta scenario passed |
| `src/outline/classify.rs`, `metrics.rs`, `scan.rs`, `report.rs`, `file_builder.rs`, `python_adapter/mod.rs`, `syntax.rs` | changed regions | no uncovered region identified beyond the rows above; the 20 delta scenarios and unit tests exercise the changed report, metrics, and rendering paths | verified by the 20-scenario tasks run |

No dead code was identified in the changed production files. No gap is
classified as "hard to test"; no fourth bucket is used. The two previous
bucket-2 gaps, R1 and R2, are closed by the committed regression tests named
above; no bucket-2 gap remains. The reviewer did not add tests or change
production code, design, spec, or tasks.

## Scenario execution evidence

Review does not re-run the full suite. The change-scoped tasks receipt was
refreshed during this review:

```
givn check tasks --change fix-outline-report-fidelity -> exit 0
1 feature
20 scenarios (20 passed), 94 steps (94 passed)
change verification receipt: 20 scenario(s), 0 failed, 0 skipped
at 656c1d587fd6b3b9efee4fdf02dbcf1e282b60bf
```

The run was made against the committed candidate at `656c1d58` with a clean
tree. The run itself refreshed the tracked receipt at
`givn/changes/fix-outline-report-fidelity/verification/tasks-verification.json`,
so the working tree now carries that generated modification alongside this
review record. The two new regression tests were run directly during this
review: `cargo test --locked --lib outline::syn_adapter::facts` → 2 passed, and
`cargo test --locked --test outline_gate_smoke
a_module_with_free_functions_still_needs_a_verdict` → 1 passed. No archive
receipt exists yet; the implementer-reported pre-archive full-suite runs
recorded in `tasks.md` predate `656c1d58`, so they do not include the new gate
smoke test. They are `./run-tests.sh --non-e2e` → 686 passed, 111 skipped, and
`./run-tests.sh --e2e` → 111 passed, 686 skipped. Those are not receipt
evidence; the archive will produce the authoritative per-scope receipt.
**Receipt status:** unproven candidate — not archived yet.

## Overlap dispositions

`givn lint --change fix-outline-report-fidelity` exits 0 with one advisory
shape match between two delta scenarios. Both share the same Given
("unknown receiver"), but assert different resolution outcomes produced by
different rules: several compatible candidates (`ambiguous`, no target) versus
an explicit contradicting qualifier (`external`, no target). Merging them would
combine two observable behaviours, which the spec rules forbid.

| Scenario A | Scenario B | Disposition |
|---|---|---|
| An ambiguous method name has no resolved target | An associated-function call whose qualifier contradicts the target is external | boundary |

## Split-or-keep

`givn lint` reported no scenario above the long-scenario threshold; no split
decision is required.

| Scenario | Decision |
|---|---|

## Findings

No open findings.

**R1 — resolved.** The promised gate regression test for a module with free
functions exists as `a_module_with_free_functions_still_needs_a_verdict`
(`tests/outline_gate_smoke.rs:635`), added in `656c1d58`. It commits a touched
production module containing only `pub fn load()`, writes a review record with
no verdict for that unit, and asserts the gate fails with
`MISSING outline module:src/free.rs`. That failure is only produced after the
gate selects the module unit and renders its outline, so a regression that
filtered free-function modules out would make the assertion fail. The test
passed in this review: `cargo test --locked --test outline_gate_smoke
a_module_with_free_functions_still_needs_a_verdict` → 1 passed (16 filtered
out). The companion type-only test at `:611` still passes, and the pair covers
both cases `design.md:185` and `design.md:368` promised.

**R2 — resolved.** The qself derivation is covered by
`qself_uses_the_trait_owner`
(`src/outline/syn_adapter/facts.rs:321`), added in `656c1d58`. It parses
`{ <Widget as Store>::make(); <Widget>::run(); }` and asserts
`Function { qualifier: Some("Store") }` and
`Function { qualifier: Some("Widget") }`. Against the pre-remediation
`Some(type_owner(&qself.ty))` code (`b1fffc9e`) the first assertion would read
`Widget` and fail, so the test is a real guard for the as-trait form; the
second assertion pins the inherent-form fallback. The test passed in this
review: `cargo test --locked --lib outline::syn_adapter::facts` → 2 passed
(358 filtered out). `design.md:238` now scopes the rule: the last segment
before the callee for `<T as Trait>::method()`, the qself type `T` for the
inherent `<T>::method()` form. The design row and the implementation agree.

Both findings were bucket 2 (missing test coverage). The committed tests close
them; no bucket-2 gap remains.

## Sign-off

- [x] Fabrication audit: step bodies, commits, promised components, strict
      mode, e2e tag integrity, e2e scope, duplicate implementations, local
      environment, and design commands/file layout/framework are clean. The
      R1/R2 test promises that the previous review reported are now delivered
      and verified by direct runs.
- [x] Every checked task has a verified commit touching production code or its
      scenario regression test; `givn check tasks` validated all 20 at the
      clean candidate before this review record was rewritten.
- [x] Every promised component and regression test exists, including the two
      tests recorded as R1 and R2 in the previous review.
- [x] Every consumer uses only its declared contract; no consumer imports an
      adapter implementation.
- [x] Strict-mode proof present and passing in `tasks.md:72-86`.
- [x] Scenario execution evidence is recorded per scope in the change
      verification receipt for the tasks scope at `656c1d58`; the archive
      per-scope receipt does not exist yet and will carry the measured result
      (unproven candidate).
- [x] Coverage classification uses the most recent available measured
      evidence plus static inspection; no archive receipt exists yet.
- [x] Every coverage gap classified and resolved — R1 and R2 are covered by
      `a_module_with_free_functions_still_needs_a_verdict` and
      `tests::qself_uses_the_trait_owner`; both tests pass.
- [x] Dead code deleted: none found. Missing tests added: the F1–F3 tests and
      the R1/R2 tests are present and passing. No hard-to-test gap is claimed.
- [x] No redundant unit tests were added; the new resolver and shape tests
      cover branch behaviour not asserted through the black-box scenarios.
- [x] No `@wip` tags remain in the delta spec; the delta contains no
      implementation-layer detail.
- [x] Canonical E2E policy was read from the resolved specs instruction; the
      one normalized inventory action has its permanent `@e2e` scenario, and
      the delta's variants are regular scenarios.
- [x] Every E2E scenario has a real-interface primary assertion and uses the
      reviewed driver; there is no browser UI and no HTTP/fetch substitute.
- [x] The local run command starts the full stack cleanly; the product is a
      CLI with no service, database, container, or digital twin.
- [x] The exact file(s) `verify.e2e_command` invokes were identified and read;
      the tree, including untracked files, was searched for a second/parallel
      e2e implementation.
- [x] `verify.e2e_command` is not identical to `verify.command`; the
      complementary tag filters prove isolation, and the pre-archive counts
      (686 regular / 111 e2e) are strictly different. The archive receipt will
      prove the scoped counts.
- [x] Implementation (commands, file layout, framework/driver) matches what
      design.md explicitly named; no design.md update or design-review re-run
      was needed for those decisions (the qself sentence update is recorded in
      the design conformance table above).
- [x] Interaction coverage verified: the one inventory entry maps to one design
      matrix row, one permanent `@e2e` scenario, and real-subprocess step
      definitions.
- [x] Design.md's internal-primitives content (its `## Data model` section)
      lists `CallShape`, `ReceiverEvidence`, `Call.shape`/`target`,
      `AnalyzedFile.excluded_spans`, and type items; no step definition depends
      on an unlisted production type.
- [x] arc42 slug mapping resolves: every chapter named in `arc42.md` maps to a
      real `docs/arc42/<slug>.md` file.
- [x] No finding is excused with a classification outside the three buckets.

## Outline gate

The gate printed 34 outlines (12 module outlines and 22 type outlines) and
3 metric hints. Every outline below is readable as a forward-only call tree;
same-unit calls nest under their caller, cross-component calls are leaves, and
the `[ambiguous]` and `[cycle]` markers are the analyzer's real edges rather
than invented callees.

OUTLINE: AUTHOR maki-architect
OUTLINE: REVIEWER maki-re-reviewer
OUTLINE: PASS module:src/outline/classify.rs | Turns an Analysis into per-unit classifications; assess computes metrics, compute_unit classifies each unit, and decide asks classify_type, classify_module, and over_method_limit before evidence records only the fired predicates.
OUTLINE: PASS module:src/outline/gate.rs | Renders the change-relative outline verdict; evaluate builds a request, review resolves base and head, materializes and analyzes both trees, selects method-bearing touched units, and renders each outline before outline_outcome folds marker failures into a GateOutcome, while enforce only renders that outcome.
OUTLINE: PASS module:src/outline/metrics.rs | Computes span-based line counts, complexity, nesting, and LCOM4; compute iterates units, compute_unit and method_metrics measure spans and flow, and cohesion unions only resolved same-unit method and field edges.
OUTLINE: PASS module:src/outline/python_adapter/extractor.py | Extracts a deterministic Python outline; parse_file walks methods and classes, method composes visibility, signature, calls, and fields, calls_in asks call_fact for each call and drops computed callees, and main reads stdin and writes the JSON result.
OUTLINE: PASS module:src/outline/python_adapter/mod.rs | Maps the Python extractor into the neutral Analysis; analyze_sources invokes python3, methods_from and fields_from translate methods and fields, and failed_analysis builds a named failure result when extraction fails.
OUTLINE: PASS module:src/outline/report.rs | Renders one classified Assessment as human text or schema-3 JSON; render_human builds the method index and unit list, render_outline and render_calls print the tree, and the files/units/outline JSON helpers serialize metrics, calls with target, and evidence.
OUTLINE: PASS module:src/outline/resolve.rs | Resolves every call against its shape; resolve looks up name candidates, filters by function qualifier or method receiver evidence through function_qualifier_matches, and returns same_file, project_other_file, name_only, ambiguous, or external with a candidate target, while resolve_calls syncs method and outline calls.
OUTLINE: PASS module:src/outline/scan.rs | Discovers production sources and counts lines; discover walks roots through language and exclusion filters, classify_line labels code, comment, and blank lines, and code_lines_excluding and span_lines_excluding subtract recorded test spans.
OUTLINE: PASS module:src/outline/syn_adapter/facts.rs | Extracts method facts from parsed Rust; method_from_fn and method_without_body build signatures and spans, FactsVisitor walks expressions for calls, fields, and flow, and receiver_evidence plus the qself and path derivation supply each call shape.
OUTLINE: PASS module:src/outline/syn_adapter/file_builder.rs | Builds one AnalyzedFile from a parsed Rust source; analyze_file sets the module path, FileBuilder.walk_items walks declarations, and the resulting units and excluded spans are appended to the analysis.
OUTLINE: PASS module:src/outline/syn_adapter/spans.rs | Derives attribute-aware source spans for items, idents, fields, impls, and macros so excluded test spans and unit spans share the same line accounting.
OUTLINE: PASS module:src/outline/syn_adapter/syntax.rs | Renders syn types and paths faithfully; type_label dispatches by Type, qualified_path_label, path_label, and segments_label render path structure, path_arguments_label and generic_argument_label recurse, and expr_label and bounds_label cover array lengths and trait bounds.
OUTLINE: PASS type:src/outline/classify.rs::Classification | The one classification label per unit; label returns its stable report text.
OUTLINE: PASS type:src/outline/classify.rs::Comparison | The evidence comparison operator; label returns its printed symbol.
OUTLINE: PASS type:src/outline/classify.rs::MetricValue | A measured metric that may be absent; number exposes the integer for evidence rendering.
OUTLINE: PASS type:src/outline/gate.rs::GateFailure | A named gate failure; code and message render its stable code and text, and sort_key orders failures by side, unit, and code.
OUTLINE: PASS type:src/outline/gate.rs::GateOutcome | The gate result; early builds a failure-only outcome, passed checks for no failures, render prints the header, hints, failures, and final line, and final_line summarizes the failure count.
OUTLINE: PASS type:src/outline/gate.rs::GateRequest | The immutable request for one gate run; new captures the project root and change id, and root_strings exposes both for diagnostics.
OUTLINE: PASS type:src/outline/gate.rs::Side | The base or head side of a gate analysis; label returns its printed name.
OUTLINE: PASS type:src/outline/metrics.rs::MethodFinding | The method-level finding label; label returns its stable text.
OUTLINE: PASS type:src/outline/metrics.rs::UnitMetrics | The measured per-unit metrics; method_findings lists the methods that exceeded a method threshold.
OUTLINE: PASS type:src/outline/model.rs::Construct | The analyzer construct label; label returns its stable text.
OUTLINE: PASS type:src/outline/model.rs::DiagnosticKind | The diagnostic kind label; label returns its stable text.
OUTLINE: PASS type:src/outline/model.rs::Language | The source language label; label returns its stable text.
OUTLINE: PASS type:src/outline/model.rs::Limitation | The analyzer limitation label; label returns its stable text.
OUTLINE: PASS type:src/outline/model.rs::RepoPath | A repository-relative path; new normalizes it, as_str exposes it, and fmt renders it for diagnostics.
OUTLINE: PASS type:src/outline/model.rs::Resolution | The call-resolution label; label returns its stable text.
OUTLINE: PASS type:src/outline/model.rs::SourceUnit | A neutral source unit; is_test classifies it from its origin.
OUTLINE: PASS type:src/outline/model.rs::TypeKind | The declared type kind label; label returns its stable text.
OUTLINE: PASS type:src/outline/model.rs::UnitId | The stable unit identifier; fmt renders it for diagnostics.
OUTLINE: PASS type:src/outline/python_adapter/mod.rs::PythonAnalyzer | The Python SourceAnalyzer adapter; capabilities advertises its support, and analyze discovers sources, runs the extractor, and maps the result or a named failure into an Analysis.
OUTLINE: PASS type:src/outline/report.rs::MethodIndex | A lookup from method id to classified method; from_assessment builds it, and get resolves a target id for rendering and nesting.
OUTLINE: PASS type:src/outline/syn_adapter/facts.rs::FactsVisitor | The syn visitor that collects a method's calls, field accesses, and flow; each visit method records its node and then recurses into the same visitor.
OUTLINE: PASS type:src/outline/syn_adapter/file_builder.rs::FileBuilder | The Rust item walker that builds module and type units; walk_items dispatches structs, enums, unions, traits, aliases, impls, and functions, push_type appends items, and push_impl merges impl methods into their type.
OUTLINE: PASS

README-IMPACT: updated - Description

The outline gate passes at `656c1d58` with 34 outlines (12 module and 22
type), 3 metric hints, and exit 0. Every per-unit `OUTLINE: PASS` verdict was
kept because the gate emitted the same 34 unit ids and the same description
sentence for each one as the record already held from the previous gate run;
no verdict needed an update. R1 and R2 are resolved by the committed tests
named in Findings. No production code, design, spec, tasks, or test file was
changed by this review; only this review record was rewritten and the generated
tasks receipt was refreshed.

**REVIEW: PASS**
