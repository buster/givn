# Review: fix-outline-report-fidelity

Fresh-context review of the committed candidate (`HEAD` = db5b7e6d plus the
uncommitted documentation/format pass) against the proposal, delta spec,
design, design-review, tasks, arc42 and the owning use case.

## Fabrication Audit (do this first)

**Step definition scan** — every step definition file for this capability was
read and mechanically scanned for empty/no-op bodies (`{}`, bare `pass`,
bare `return`, no assertion reference):

```
Files scanned: tests/steps/outline_report_steps.rs (1455 lines),
tests/steps/outline_report_e2e_steps.rs (44 lines).
tests/steps/outline_report_support.rs holds fixture writers only (no step bindings).
0 empty step bodies found across 2 step definition files.
0 unimplemented!/todo! stubs in step bodies; the only `todo!()` match is inside
the generated Rust fixture text at outline_report_support.rs:581.
Every delta Then asserts a concrete value: an exact method id, an exact
resolution label, an exact unit/file metric number, an exact evidence rule, or
an indent comparison on the printed tree.
```

**Commit cross-check** — every checked-off `[x]` scenario in tasks.md has one
reachable commit whose body names the scenario; each diff touches production
source or the scenario's regression test:

| Scenario | Commit | Touches production code or regression test? |
|---|---|---|
| A method call on an unknown receiver is a name-only guess | b1fffc9e | production: 13 source files (12 modules; `python_adapter` is `extractor.py` + `mod.rs`) + tests |
| A name-only guess is not expanded as a same-unit call | 8144cca8 | regression test (steps + fixture); production landed in b1fffc9e |
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
| A same-unit method implemented in another file is project_other_file | 21b433b9 (actual `21b433b9390d0…2b2e`; tasks.md records an invalid hash — F7) | regression test |
| A developer reads each call's resolution in the human report | 240bdaad | regression test |
| A file's code lines exclude inline test modules | 887b990d | regression test |
| A unit's physical lines cover only its own spans | bda7ebdc | regression test |
| An impl-level test method is excluded from the unit's lines | ee1f69da | regression test |
| A name-only guess does not connect methods for cohesion | b5554de3 | production: `src/outline/metrics.rs` + test |
| A method signature keeps arrays, qualified paths, and its full type detail | db5b7e6d | regression test |

The actual revisions are reachable from `HEAD` and each message body is the
scenario title. The hash *recorded* in `tasks.md` for scenario 14 is not a real
revision (F7). Observation: 17 of the 20 commits carry only their regression test,
because the production implementation for those scenarios landed in b1fffc9e;
the `tasks.md` RED evidence therefore proves the missing step binding, not the
missing behaviour. The repo rule ("implementation or regression test") is met
and the tests fail if the behaviour regresses; not a fabrication finding.
Observation: the RED runs used absent bindings rather than the design's named
panic stub, so the RED did not prove the production behaviour was absent.
Final step-file quality is real (concrete assertions, real fixtures).

**Mechanical scenario-commit audit** — `givn check tasks --change
fix-outline-report-fidelity` fails before running the suite, so it produces no
change verification receipt. Two independent causes: (a) the recorded hash for
scenario 14 does not exist (F7), and (b) `tasks.md` writes each scenario as a
`## N. <title>` heading with `- [x] Evidence: Scenario: <title> | <runner
output>` lines, and the commit parser (`src/cli/check.rs:192-208`) treats every
`- [` line containing `Scenario:` as a scenario declaration. It therefore
reads the evidence suffix as part of the scenario title and reports for all 20
scenarios that the commit "does not name the scenario in its message"; the
canonical template declares scenarios as `- [ ] Scenario: <title>` checkbox
lines. F7 records this; until it is fixed, neither the tasks receipt nor the
outline gate can be produced.

**Promised-architecture cross-check** — every file and component design.md
promised exists:

| Component (from design.md) | Exists? | Path |
|---|---|---|
| `CallShape`, `ReceiverEvidence`, `Call.shape`, `AnalyzedFile.excluded_spans` | yes | `src/outline/model.rs` |
| Shape-consistent resolver | yes | `src/outline/resolve.rs` |
| Rust call-shape extraction | yes | `src/outline/syn_adapter/facts.rs` |
| Recursive type renderer | yes | `src/outline/syn_adapter/syntax.rs` |
| Rust excluded-test spans and module type items | yes | `src/outline/syn_adapter/file_builder.rs` |
| Python call-shape extraction | yes | `src/outline/python_adapter/extractor.py` |
| Python call-shape mapping and class items | yes | `src/outline/python_adapter/mod.rs` |
| Per-predicate evidence | yes | `src/outline/classify.rs` |
| Span-scoped metrics and evidenced LCOM4 | yes | `src/outline/metrics.rs` |
| Production line helpers | yes | `src/outline/scan.rs` (`production_code_lines`, `span_lines_excluding`) |
| JSON `target`, resolution tag, non-nesting `name_only` | yes | `src/outline/report.rs` |
| Method-bearing touched-unit selection | yes | `src/outline/gate.rs:569` |
| Regular bindings / fixtures / e2e file | yes | `tests/steps/outline_report_steps.rs`, `outline_report_support.rs`, `outline_report_e2e_steps.rs` |

`src/outline/syn_adapter/spans.rs` was also modified and is not named in the
design's file table; it is an existing helper for attribute-aware spans. The
table is not exhaustive, so this is a recorded note, not a missing component.

Contract check: the consumers (`src/cli/outline.rs`, `src/outline/gate*`,
`report`, `classify`, `metrics`) use the neutral `outline` API and the report
renderer; none imports `syn_adapter` or `python_adapter`. The only adapter
imports are inside the analyzers, the composition root
(`outline::analyzer::outline_analyzer`) and the test harness that implements
`SourceAnalyzer`.

**Strict-mode proof** — present with output in the setup task
(`tasks.md:72-86`): the runner calls `.fail_on_skipped()` on both paths
(`tests/features_runner.rs:55,151`), exits 1 on `stats.failed > 0 ||
stats.skipped > 0` (`:64`), and the undefined-step probe produced
`1 scenario (1 failed)`, `Step doesn't match any function`, exit status 1. The
all-`@wip` baseline was `20 scenarios (20 failed)`, exit 1.

**Downgraded e2e scenario scan** — the change adds no `@e2e` scenario. The
capability keeps its one permanent `@e2e` scenario:

| @e2e Scenario | Real-interface assertion present? | Driven via real interface? | Downgraded? |
|---|---|---|---|
| A developer runs a outline report and sees the classified units (permanent) | yes: stdout classification `god-type`, `HINT` label, no recommendation, no baseline file | real `givn` subprocess (`common::run_givn` through `user_runs_quoted`) | no |

The delta's 20 scenarios are regular. No `@e2e` tag was removed to bypass a
gate: the design-review records that a planned second e2e was withdrawn as a
duplicate of the same action (design-review Q1/F1, D8 withdrawn), and the
permanent e2e for that action still runs in the `--e2e` scope.

**E2e scope check**

| Capability | @e2e scenario count | Distinct actions? | Over-produced? |
|---|---|---|---|
| `outline-report` | 1 permanent (delta adds 0) | 1: run a outline report | no |

**Duplicate-implementation check** — `verify.e2e_command` is
`./run-tests.sh --e2e` (`givn/commands.yaml:4`), which runs
`features_runner` with `@e2e and not @givn.removed` over `givn/specs`
(`run-tests.sh:65-67`). For this capability it binds the permanent scenario's
Then steps in `tests/steps/outline_report_e2e_steps.rs`; the Given/When steps
are shared from `outline_report_steps.rs` and also drive the real binary.
`outline_review_gate_e2e_steps.rs` belongs to the different
`outline-review-gate` capability. No other outline e2e implementation exists
in the tree; `git status` untracked files are documentation only.

| Capability | File(s) the e2e command invokes | Other implementation found? (incl. untracked) | Stronger one used instead? |
|---|---|---|---|
| `outline-report` | `tests/steps/outline_report_e2e_steps.rs` + shared Given/When in `outline_report_steps.rs` | none | n/a |

**Local environment check** — the design's local run command is
`cargo run -- outline report` in this checkout; the product is a CLI with no
service, database, container, or digital twin. It starts cleanly and prints
`outline report — python+rust, 93 files, 334 units, 20 hints`.

**E2e isolation check** — values read literally from `givn/commands.yaml`:

| `verify.command` value | `verify.e2e_command` value | Identical? | Regular scope count | E2e scope count | E2e strictly smaller? |
|---|---|---|---|---|---|
| `./run-tests.sh --non-e2e` | `./run-tests.sh --e2e` | no | 686 (implementer-reported pre-archive run) | 111 (implementer-reported pre-archive run) | yes |

No archive verification receipt exists yet; the counts above are the
implementer-supplied pre-archive runs, not receipt evidence. The tag filters
in `run-tests.sh` are complementary, so the scopes are disjoint.

**Design.md conformance check**

| Decision (from design.md) | design.md says | Actually built | Matches? | Deviation resolved? |
|---|---|---|---|---|
| Verification commands | `./run-tests.sh --non-e2e` / `--e2e` | identical | yes | n/a |
| Runner/framework | existing cucumber-rs runner; `GIVN_FEATURES` for delta runs; anchored `--name` | unchanged `tests/features_runner.rs`, `run-tests.sh` | yes | n/a |
| Step files | regular bindings in `outline_report_steps.rs`, e2e file unchanged | same two files; e2e Given/When shared | yes | n/a |
| Files and modules table | 12 named production files + tests/docs | all exist; extra `syn_adapter/spans.rs` | yes (extra helper) | note only |
| Gate selection | `unit.metrics.methods > 0` | `!unit.method_names.is_empty()` (`gate.rs:569`); `method_names` is built from `unit.methods` (`classify.rs:163-168`) | equivalent | n/a |
| Gate regression test (type-only module / module with free functions) | promised in D4, files table, risks | absent | **no** | not resolved (F5) |
| Resolver table test for every shape row + qualifier-prefix and Python fallback rules | promised in files table and risks | one pre-existing test; no table test | **no** | not resolved (F5) |
| Multi-segment qualifier (`std::line::Line`) is `external` | normative rule | resolves `same_file` to project `Line::from` | **no** | not resolved (F1) |
| Python `Qualifier` with no matching owner → unique candidate is `name_only` | normative rule | resolves `external`, target null | **no** | not resolved (F2) |
| Python non-`Name`/`Attribute` callee is not recorded; `ast.unparse` fallback removed | normative rule | fallback still present; `make()()` records callee `make()` | **no** | not resolved (F3) |
| qself `<T as Trait>::method()` uses the trait path, not the qself type | normative shape table | qualifier is the qself type (`facts.rs:131-132`); `<Widget as Store>::make()` targets `Widget::make` | **no** | not resolved (F4) |
| Schema 3 / rules 1 | unchanged | `outline_json` schema 3, rules 1 | yes | n/a |
| Documentation plan | chapters 01, 03, 04, 05, 06 §6.19, 08, 10, 11, 12, arc42 README, usecase.md, outline.md, README | 03, 04, 05 (partial), 06, 08, 10, 11 (partial), 12, outline.md, README updated; 01, arc42 README, usecase.md, chapter-05 gate row, R-115/new risk absent | **partial** | not resolved (F6) |

**Interaction coverage verification**

| Inventory entry | Matrix row exists? | @e2e scenario exists? (title) | Promised driving mechanism | Actual driving mechanism | Match? |
|---|---|---|---|---|---|
| `outline-report` — run a outline report | yes (design §Interaction coverage matrix) | A developer runs a outline report and sees the classified units | real `givn` subprocess | `common::run_givn` via `user_runs_quoted` in `tests/steps/outline_report_steps.rs:15-18` | yes |

## Use-Case And Persona Conformance

| Contract | Evidence | Pass? |
|---|---|---|
| Owning use-case guarantees and rules preserved | `givn/specs/assess-code-structure/usecase.md` goal, main flow, rules, success/minimal guarantee unchanged; delta adds no capability or interaction | yes |
| Capability ownership and typed relationships preserved | `EXTEND outline-report` under `assess-code-structure`; `Includes`/`Extends` still `none` | yes |
| Confirmed Persona-relevant outcomes preserved | `developer--primary-user` (trustworthy labels/signatures), `agent--technical-role` (JSON target, production counts), `givn-maintainer--owner` (auditable resolver) | yes |
| Actors remain distinct from Persona biographies | Gherkin names "the developer" and "the report"; Personas stay review lenses | yes |
| Migration ledger and coverage evidence complete, if applicable | no migration; coverage is measured by archive (no receipt yet) | n/a |
| Use-case plan item: state that a module outline lists declared member types | design promised the edit; `usecase.md` is unchanged | **no (F6)** |

PERSONAS: developer--primary-user

The other confirmed Personas (`agent--technical-role`,
`givn-maintainer--owner`) are confirmed in the owning use case and are used as
review lenses; no Persona is invented or promoted by this change.

## Deferred mandatory checks

None.

## Visual Review

The design declares `Interface: terminal`, viewports not applicable, no
transcript committed for this delta (design §Visual Design Contract). Opposite
classification considered: a report rendered as a page would be a Web UI, but
this surface prints plain text from a CLI, emits no HTML/CSS/JavaScript, and
involves no browser or page context; the Web UI driver and rendered-state
rules therefore do not apply. The
capability's permanent e2e keeps its pre-existing evidence path; the delta adds
no `@e2e` scenario, so there is no new evidence file to commit.

| @e2e Scenario | Viewport | Evidence file | Present and non-empty? |
|---|---|---|---|
| A developer runs a outline report and sees the classified units (permanent, unchanged) | terminal | no committed transcript; pre-existing path unchanged by this delta | n/a for this delta |

The human surface was reviewed against live output of
`cargo run -- outline report` on this checkout and against the design's
rendered reference. Observed tags reproduce the reference form:
`-> [same_file, private, Engine] validate`,
`-> [project_other_file, public, module] normalize`,
`-> [name_only, public, UsecaseDiagnostics] is_empty`. No deeper-indented line
follows a `name_only` line; evidenced same-unit calls do nest.

| Confirmed Persona | Goal | Finding | Disposition |
|---|---|---|---|
| `developer--primary-user` | Read a call tree whose certainty labels are trustworthy | Tags lead with the resolution label, visibility, and owner; the tree reads in one pass | no finding (format baseline) |
| `agent--technical-role` | Use JSON `target` without re-deriving it | JSON call objects carry `resolution` and `target`; `name_only` keeps its candidate | no finding (format baseline) |
| `givn-maintainer--owner` | Audit resolver evidence through one neutral model | Tags distinguish same-file, cross-file, name-only, and ambiguous edges | semantic mislabels exist in four resolver/extractor cases; recorded as F1–F4 (open), not a text/format baseline failure |

**Baseline:** every call tag leads with a domain resolution label
(`same_file`, `project_other_file`, `name_only`, `ambiguous`), every line has
the callee, no internal identifier replaces a label, output is line-oriented
and deterministic (two consecutive runs were compared), the header names the
report, no duplicated visible content, English, no motion. Error state and
reduced-motion are not applicable to a text report.

VISUAL-REVIEW: PASS

## Retirements

N/A. This change retires no capability.

## Arc42 implementation conformance

The affected chapters were independently derived from the completed
implementation (resolver, adapters, report, gate, metrics, classify), the
design and the tasks, then compared with the durable chapters and `arc42.md`.

| Arc42 chapter or architectural fact | Durable-doc source | `arc42.md` claim | `design.md` treatment | `tasks.md` treatment | Implementation evidence | Match? / finding |
|---|---|---|---|---|---|---|
| Call resolution is shape-aware; `name_only` is a leaf | 03, 04, 05, 06 §6.19, 08 §8.26 | chapters 03–08 affected | D1, D2, files table, resolution rules | tasks 1–6, 14, 15 | `src/outline/resolve.rs`, `report.rs:227` | match |
| Module outlines list declared member types | 05, 06 §6.19, 08 §8.26, 12 | chapters 05, 06, 08, 12 affected | D4, type-item section | tasks 10–12 | `syn_adapter/file_builder.rs:211`, `python_adapter/mod.rs:222` | match |
| Production-only line accounting | 06 §6.19, 08 §8.26, 12 | chapters 06, 08, 12 affected | D6, line-accounting section | tasks 16–18 | `model.rs:175`, `scan.rs:206-230`, `metrics.rs` | match |
| JSON `target` and human resolution label | 03, 06 §6.19, 08 §8.26, 10 Q-119 | chapters 03, 06, 08, 10 affected | D5, JSON call object | tasks 13, 15 | `report.rs:379`, `report.rs:202-238` | match |
| Evidence lists fired predicates only | 08 §8.26, 12 | chapters 08, 12 affected | D3, classification evidence table | tasks 7–9, 19 | `classify.rs` evidence branches | match |
| Signatures keep type shape | 12, 05 | chapter 12 affected | D7, signature rendering | task 20 | `syn_adapter/syntax.rs` | match |
| Gate selects method-bearing units | 05 `outline::gate`, 03 interface row | chapters 03, 05 affected | D4, files table | no task (design gap) | `gate.rs:569` filters `method_names` | durable claim holds; design's promised chapter-05 gate-row edit absent (F6) |
| Chapter 01 FR-46 fidelity guarantee | 01 FR-46 | chapter 01 **not** affected | documentation plan says update FR-46 | no task | chapter 01 unchanged | contradiction: design says update, `arc42.md` says no (F6) |
| R-115 extension for additive `target`; new target-misuse risk | 11 | chapter 11 affected, claims only R-106/R-121 | documentation plan names R-115 + new risk | no task | only R-106 and R-121 changed | omission (F6) |
| Change status log entry | `docs/arc42/README.md` §Documentation Status | not mentioned | documentation plan says add entry | no task | every prior change has an entry; this one is absent | omission (F6) |
| Owning use case states module member types | `givn/specs/assess-code-structure/usecase.md` | not mentioned | documentation plan says update | no task | use case unchanged | omission (F6) |
| No ADR qualified | 09 | `NOT_QUALIFIED` | ADR verdict | n/a | no ADR file added/amended | match |

- [x] Every chapter marked affected by either assessment was opened and
      checked for stale, contradictory, or placeholder content.
- [ ] Every Arc42 fact or architecture decision relevant to this change is
      represented in `design.md` and mapped to one or more tasks in
      `tasks.md` — five documentation-plan items have no task (F6).
- [ ] The completed implementation follows those facts and decisions — four
      normative resolver/extractor rules do not (F1–F4).
- [ ] Architecture changes discovered in the completed implementation are
      recorded in the durable chapter and the change-level assessment — the
      Python callee fallback and the multi-segment qualifier rule are not.
- [ ] No mismatch remains.

ARC42 CONFORMANCE: FINDINGS

**Result:** FINDINGS — F1–F7 below. No remediation was applied in this review
(production code and documentation were not changed); the affected code,
tests, `design.md`/`arc42.md` and durable chapters must be corrected and the
review repeated.

---

## Coverage

No archive verification receipt exists for this change yet, so there is no
measured coverage result. This is an unproven candidate; the classification
below comes from static inspection and the delta scenarios, and the archive
receipt will carry the measured result.

```
unproven candidate — no givn/archive/<id>/verification.json exists yet.
```

## Coverage gap classification

| Location | Lines | Classification | Action taken |
|---|---|---|---|
| `src/outline/resolve.rs::function_qualifier_matches` multi-segment qualifier branch | 200-224 | 2. Missing test coverage (and a behaviour deviation) | none yet — F1 |
| `src/outline/resolve.rs` Python `Qualifier` fallback (`name_only` for a unique candidate) | 124-156 | 2. Missing test coverage (branch absent) | none yet — F2 |
| `src/outline/python_adapter/extractor.py` non-`Name`/`Attribute` callee fallback | 35-40 | 2. Missing test coverage (design says not recorded) | none yet — F3 |
| `src/outline/syn_adapter/facts.rs` qself qualifier derivation | 131-132 | 2. Missing test coverage (deviation from the shape table) | none yet — F4 |
| `src/outline/resolve.rs` resolution-table rows (qualifier prefix, trait-only fallback, Python convention, function variants) | 98-177 | 2. Missing test coverage: the promised table test is absent | none yet — F5 |
| `src/outline/gate.rs` type-only module selection | 569 | 2. Missing test coverage: the promised gate regression test is absent | none yet — F5 |
| `src/outline/classify.rs`, `metrics.rs`, `scan.rs`, `report.rs`, `file_builder.rs`, `python_adapter/mod.rs` | — | no uncovered region identified beyond the rows above | n/a |

No dead code was identified in the changed production files. No gap is
classified as "hard to test"; no fourth bucket is used.

## Scenario execution evidence

Review does not re-run the suite and no archive receipt exists yet:

```
unproven candidate — no givn/archive/fix-outline-report-fidelity/verification.json
receipt yet. Implementer-reported pre-archive runs (not receipt evidence):
./run-tests.sh --non-e2e -> 686 passed, 111 skipped; ./run-tests.sh --e2e ->
111 passed, 686 skipped; cargo test --lib -> 356 passed.
```

The change-scoped tasks receipt is also absent: `givn check tasks` fails its
scenario-commit audit (F7), so `givn check review` stops before the outline
gate. **Receipt status:** unproven — not archived yet

## Overlap dispositions

`givn lint --change fix-outline-report-fidelity` exits 0 with one advisory
shape match between two delta scenarios. Both inherit the identical Given
("unknown receiver"), but assert different resolution outcomes produced by
different rules: several compatible candidates (`ambiguous`, no target) versus
a contradictory explicit qualifier (`external`, no target). Merging them would
combine two observable behaviours, which the spec rules forbid.

| Scenario A | Scenario B | Disposition |
|---|---|---|
| An ambiguous method name has no resolved target | An associated-function call whose qualifier contradicts the target is external | boundary |

## Split-or-keep

`givn lint` reported no scenario above the long-scenario threshold; no split
decision is required.

| Scenario | Decision |
|---|---|

---

## Findings

**F1 — a multi-segment external qualifier is reported as a certain
`same_file` call (unfinished work).** `src/outline/resolve.rs:200-224` matches
the qualifier's last path segment only, so the design's normative rule "a
qualifier may match a project owner only when it is a single segment or starts
with `crate`, `self`, or `super`" is not implemented. Evidence: a scratch
fixture containing project `Line::from` and a call to
`std::line::Line::from(1)` produces
`{"callee":"from","resolution":"same_file","target":"...::Line::from"}`,
where the design says `external`. A realistic fixture reproduces it: a
project type `Path` with `new` plus a call to `std::path::Path::new("x")`
produces `{"callee":"new","resolution":"same_file","target":"...::Path::new"}`.
This is the same false-certainty defect the change exists to remove. No delta
scenario covers it.

**F2 — a Python named receiver with no matching owner loses its unique
candidate (unfinished work).** The design says a Python `Qualifier` is a
naming convention: when no owner matches, a unique project candidate becomes
`name_only`. `resolve.rs:140-156` only falls back for a trait candidate, so a
Python call `External.refresh()` with a unique project `Ledger::refresh`
resolves `external` with `target: null`. No delta scenario covers it.

**F3 — the Python `ast.unparse` callee fallback was not removed (unfinished
work).** Design says a callee that is not a `Name` or `Attribute` is not
recorded and the fallback is removed; `python_adapter/extractor.py:35-40`
still records `ast.unparse(func)` (or `"<call>"`). Evidence: a fixture with
`make()()` records the callee `make()` in the JSON output. The design and
`docs/outline.md` present callee names as domain names, so this still
fabricates a call name.

**F4 — qself calls use the qself type instead of the trait path (unfinished
work).** The design's shape table says `with qself the path segments before
the callee are the trait path` and `the qself type is not used as a qualifier`.
`syn_adapter/facts.rs:131-132` calls `type_owner(&qself.ty)`, so
`<Widget as Store>::make()` is qualified `Widget` and targets
`Widget::make`, not the trait's declaration. Observable when several
implementors exist; no scenario covers it.

**F5 — the promised resolver and gate regression tests are absent (bucket 2:
missing test coverage).** The design's files table and risks promise a resolver
table test for every resolution row (including the qualifier-prefix rule and
the Python convention fallback) and a gate regression test for a module with
only type items and a module with free functions. `src/outline/resolve.rs` has
one pre-existing test (same-unit/first-file sync) and `src/outline/gate.rs`
tests only the final verdict text. The missing tests are why F1–F4 reached
review.

**F6 — durable documentation and the design's documentation plan diverge
(unfinished work).** `docs/arc42/README.md` has no
`fix-outline-report-fidelity` entry although every prior change has one;
R-115 was not extended for the additive `target` field and the promised "reads
`target` without `resolution`" risk is absent; chapter 01 is asserted
unaffected by `arc42.md` while `design.md` requires an FR-46 edit; the
`outline::gate` building-block row and the owning use case's member-type
statement are also unmodified and untasked. `tasks.md`'s final verification
narrows the design's chapter list without updating `design.md` or re-running
design-review.

**F7 — the task record is not machine-checkable and records one invalid
revision (unfinished work).** `tasks.md` records
`21b433b939fbc8b97b7de2e55ffe4cc07ba6b520` for scenario 14; the real revision
is `21b433b9390d0b2031781877803990280ca42b2e`, and `git cat-file` rejects the
recorded hash. The file also declares scenarios as `## N. <title>` headings
with `- [x] Evidence: Scenario: <title> | <runner output>` lines instead of the
template's `- [ ] Scenario: <title>` checkbox form, so the commit parser reads
the runner-output suffix as the title. Result: `givn check tasks` fails, no
`verification/tasks-verification.json` receipt exists, and `givn check review`
stops at that missing receipt before the outline gate. The scenarios' real
commits and tests are sound; the task record and its recorded hash are not.

No production code, test, or documentation was changed by this review. The
findings above require code/test/doc changes and a repeated review; the
reviewer did not remediate them.

## Sign-off

- [ ] Fabrication audit: CLEAN in step bodies and commits, but design
      conformance is not clean: four normative resolver/extractor rules are
      unimplemented (F1–F4), two promised test groups are absent (F5), and
      the task record is not machine-checkable (F7).
- [ ] Scenario execution evidence recorded per scope in the archive
      verification receipt — no receipt exists yet (unproven candidate), and
      the change-scoped tasks receipt cannot be produced until F7 is fixed.
- [ ] Every coverage gap is classified and resolved — bucket-2 gaps remain
      open (F1–F5).
- [x] No `@wip` tags remain in the delta spec.
- [x] The delta spec contains no implementation-layer detail; it asserts CLI,
      JSON, and printed-report behaviour and exact observable values.
- [x] The local run command starts cleanly; the product is a CLI with no
      digital twins.
- [x] The exact file(s) `verify.e2e_command` invokes were identified and the
      tree, including untracked files, was searched for a parallel e2e
      implementation.
- [x] `verify.e2e_command` is not identical to `verify.command`; the tag
      filters are complementary and the e2e scope is strictly smaller (no
      receipt yet; pre-archive counts recorded above).
- [ ] Implementation matches design.md — four normative rules and the
      documentation plan do not (F1–F6).
- [x] Interaction coverage verified: the single inventory entry maps to one
      design matrix row, the permanent `@e2e` scenario, and real-subprocess
      step definitions.
- [x] Design.md's internal-primitives content (its `## Data model` section)
      lists `CallShape`, `ReceiverEvidence`, `Call.shape`/`target`,
      `AnalyzedFile.excluded_spans`, and type items; no step definition
      depends on an unlisted production type.
- [x] arc42 slug mapping resolves: every chapter named in `arc42.md` maps to
      a real `docs/arc42/<slug>.md` file.
- [x] No finding is excused with a classification outside the three buckets.

**REVIEW: FAIL**

## Outline gate

`givn check review --change fix-outline-report-fidelity` was run after this
file was written. It stopped at `change verification receipt missing at
givn/changes/fix-outline-report-fidelity/verification/tasks-verification.json`
before the outline stage, so the gate printed **0 outlines** and no per-unit
verdict can be issued. Reason: `givn check tasks` cannot create that receipt
because the task-record audit fails (F7: invalid scenario-14 hash and
scenario titles read from evidence lines).

OUTLINE: AUTHOR maki-architect
OUTLINE: REVIEWER maki-reviewer

The per-unit `OUTLINE: PASS <unit> | <responsibility; call order>` verdicts and
the final `OUTLINE: PASS` are not written because no outline was printed; they
belong to the re-review after F7 is remediated.

README-IMPACT: updated - Description
