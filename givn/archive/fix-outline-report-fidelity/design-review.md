# Design review — fix-outline-report-fidelity (fresh-context grilling)

Phase 2: hardened. Every question was dispositioned and the plan artifacts
were amended; no question remains open, no decision is carried. Hardening
date: 2026-10-21.

Disposition vocabulary: **applied** (the artifact now records the
resolution), **no issue** (the branch was checked and needed no change),
**withdrawn** (the reviewed element was removed).

## Question dispositions

| Q | Topic | Disposition |
|---|---|---|
| Q1 | Duplicate e2e action | **Applied.** `A developer reads each call's resolution in the human report` is now a regular `@givn.added @wip` scenario with no `@e2e` tag, no transcript Then, no e2e binding, no second `## Interactions` row, and no new transcript. D8 is withdrawn and the interaction matrix shows only the permanent e2e. |
| Q2 | Cross-file, same-unit label | **Applied.** The `SelfUnit` row is file-conditional; `project_other_file` is the label when the target file differs. Nesting and LCOM4 are predicates on unit membership plus `same_file`/`project_other_file`, not on the label. A scenario covers the cross-file same-unit call, its nesting, and its LCOM4 edge. |
| Q3 | Gate eligibility after module type items | **Applied.** `src/outline/gate.rs` now filters on `unit.metrics.methods > 0`, and D4, the files table, the architecture-impact paragraph, the type-item section, and the risks record the correction and its regression test. |
| Q4 | Shape derivation coverage | **Applied.** The design has a normative per-language shape table covering Rust `self`/`&self`, `Self::method`, `Type::method`, `crate`/`super` paths, turbofish, qself, `Expr::Field` and every other receiver expression, and macro-argument calls; Python `self`/`cls`, uppercase class receivers, lowercase module/object receivers, longer chains, and the removed `ast.unparse` fallback. The resolver table matches it, accepts module owners and associated functions, and applies the single-segment or `crate`/`self`/`super` qualifier rule. |
| Q5 | Named receiver against a trait method | **Applied.** A named receiver whose only candidate is a trait method becomes `name_only`, never `external`; a contradictory explicit qualifier still becomes `external`. The rule is in D1, the resolution table, and the prose. |
| Q6 | `name_only` and LCOM4 | **Resolved by operator (option A) and applied.** Name-only edges never contribute to LCOM4 or nesting. The rule set and thresholds are unchanged; the proposal's out-of-scope wording now says a corrected input may change an individual unit's classification. A scenario asserts LCOM4 4 and `god-type` for a unit that today is `cohesive-large-type`. |
| Q7 | Excluded spans in unit metrics | **Applied.** `excluded_spans` is subtracted from file code lines, unit code lines, and unit physical lines. An impl-level `#[cfg(test)]` method is excluded from both unit counts. Rust module physical lines are its own spans and omit unrelated `use`/`const`/`static`; file physical lines remain the whole file. Two scenarios pin the counts. |
| Q8 | Adapter-produced target ids | **Applied.** Every id Then asserts the exact adapter id: `tier::TierReasoning::is_empty`, `engine::Engine::validate`, `engine::Engine::check`, `line::Line::from`, `engine_parts::Engine::validate`, and unit ids such as `type:src/engine_core.rs::Engine`. |
| Q9 | RED reason and concrete Thens | **Applied.** The name-only fixture puts the unique candidate in the caller's unit with a nested `probe`, so non-expansion is RED today. Exact counts replace every vague Then: 8/18 file lines, 8/8 unit lines, 8/9 guarded unit lines, LCOM4 4/1, and full human tags. |
| Q10 | Missing scenarios | **Applied.** Added named-receiver positive, ambiguous with no target, same-unit cross-file (label, nesting, LCOM4), module interleaving order, name-only cohesion, and an impl-level test-method count. The signature scenario gained `[u8; 4]` and `crate::config::Config` as Then steps rather than a duplicate scenario. The permanent e2e is not duplicated. |
| Q11 | arc42 and glossary coverage | **Applied.** The design's documentation plan was re-derived from the served 12-row table and now names 01, 03, 04, 05, 06 §6.19, 08, 10 (Q-119; Q-108/Q-109 need no wording), 11 (R-106/R-115/R-121 plus the new target-misuse risk; R-105/R-108/R-112 need no wording), 12, `docs/arc42/README.md`, `arc42.md`, `README.md`, `docs/outline.md`, and the use-case member-type wording. |
| Q12 | ADR and version verdicts | **Applied.** The ledger records `NOT_QUALIFIED` with `CANONICAL_ARTIFACT` → `design.md` and no ADR-074 amendment, and the schema-3/rules-1 rationale now names the corrected LCOM4 input as a correction, not a tuning. |
| Q13 | RED/stub pattern | **Applied.** The test strategy adds the per-scenario binding-and-fixture rule, the named panic stub for RED, the anchored single-scenario command, and the rule that only the current scenario's production change follows. Fixture roots use `common::make_init_root` where the addon and subprocess are needed; `make_bare_root` is limited to in-process assessment. |
| Q14 | Visual contract and transcript path | **Applied.** The contract has a rendered reference with the three tag lines. Because the delta has no `@e2e`, it states that no transcript is committed and the permanent e2e keeps the only evidence path. |

## Finding dispositions

| # | Required change | Disposition |
|---|---|---|
| F1 | Demote the resolution-report scenario | **Applied.** Tagged `@givn.added @wip`; the transcript Then and the e2e-specific step are gone; the label assertions are regular bindings. |
| F2 | Delete D8 and the second matrix row | **Applied.** D8 is withdrawn in the ledger; the matrix has one row; the delta adds no interaction. |
| F3 | Make the `SelfUnit` row file-conditional and restate the nesting predicate | **Applied.** Resolution table, nesting bullets, and LCOM4 rule. |
| F4 | Correct the gate row and add a gate test | **Applied.** Files table, D4, type-item section, risks, and the test note. |
| F5 | Add the shape table and align the rows | **Applied.** D1, the shape table, and the resolver table; `ast.unparse` is removed. |
| F6 | Record the classification drift and add the LCOM4 scenario | **Applied.** D2, the version verdict, the proposal wording, and the cohesion scenario. |
| F7 | Subtract excluded spans from unit counts | **Applied.** D6, the line-accounting section, the files table, and two scenarios with exact constants. |
| F8 | Exact target ids | **Applied.** All id Thens use adapter ids. |
| F9 | Concrete non-expansion and line assertions | **Applied.** The fixture table pins every count and the name-only check. |
| F10 | Missing scenario coverage | **Applied.** The feature adds the six operator-listed behaviours; the signature detail is folded into its existing scenario. |
| F11 | Extend the documentation list | **Applied.** The documentation plan names the verified chapters and states the unaffected rows. |
| F12 | Create `arc42.md` | **Applied as a plan item.** The design's documentation task now requires the full 12-row assessment with `STATUS: DONE`; the marker is an implementation artifact and is not written during hardening. |
| F13 | Glossary updates | **Applied as a plan item.** The design names every glossary entry and the new terms; the glossary edit belongs to the implementation documentation task. |
| F14 | ADR and version rationale | **Applied.** Two ledger entries. |
| F15 | Stub pattern and `make_init_root` | **Applied.** RED/GREEN subsection and e2e infrastructure. |
| F16 | Rendered reference and transcript decision | **Applied.** Visual contract. |
| F17 | Risks paragraph | **Applied.** Shape mis-derivation, Python convention risk, and the gate regression guard. |
| F18 | `excluded_spans` in hand-built literals | **Applied as a design note.** The files table and model change name the field; implementation must update the fake and hand-built literals. |
| F19 | Use-case member types | **Applied as a plan item.** The design's use-case edit states it; the interaction table is unchanged. |
| F20 | R-106/R-115/R-121 | **Applied as a plan item.** The documentation plan names the rewrites and the new target-misuse risk. |
| F21 | `docs/outline.md` and `README.md` | **Applied as a plan item.** Both are named in the plan. |
| F22 | Type-item JSON shape and gate skip | **Applied.** The type-item section states `kind`, empty signature/calls, and the `MethodIndex` skip. |

No finding is left without a disposition, and no decision is carried into
implementation as an open question.

## Branch checklist

| Branch | Result |
|---|---|
| Scope | Resolved. The six proposal defects are covered, the operator accepted the LCOM4 input correction as in scope, and the duplicate e2e action was withdrawn. |
| Tech choices | Resolved. No new dependency; `syn` and the existing `ast.unparse` usage stay. The shape model is now normative per language. |
| Missing scenarios | Resolved. Named receiver, ambiguous, cross-file same-unit, module order, name-only cohesion, impl-level test counts, and signature array/qualified path are covered. |
| Testability | Resolved. Strict mode, the anchored single-scenario command, the named stub pattern, and exact observable values are recorded. |
| E2E fidelity | Resolved. The delta has no `@e2e`; the capability keeps one real-interface action and one inventory row. |
| Interaction coverage | Resolved. The matrix has the existing permanent row only; the delta adds no interaction. |
| Risk | Resolved. The shape-mis-derivation risk, its mitigation, and the gate guard are recorded. |
| Use-case and Persona context | Resolved. No Persona is invented; the use-case plan adds declared member types without changing the interaction table. |
| Decision ledger | Resolved. D1–D7 and D9 stand; D8 is withdrawn; the proposal's "D1–D9" reference is corrected. |
| Contracts and components | Resolved. `CallShape` and `excluded_spans` are the only contract additions; the gate correction is recorded as a component behaviour change. |
| Decision timing | Resolved. All decisions are foundational; no contract is an open question. |
| Visual design contract | Resolved. Rendered reference present; no transcript because no delta e2e. |
| Deferrals | Resolved. No mandatory check is deferred; the out-of-scope list now distinguishes rule changes from corrected evidence. |
| Persona disposition | Resolved. The three confirmed Personas are unchanged and recorded. |
| Interface classification | Resolved. CLI/terminal is correct; the e2e drives a real subprocess. |
| ADR qualification | Resolved. `NOT_QUALIFIED`, `CANONICAL_ARTIFACT` → `design.md`; no amendment. |
| Architecture documentation (arc42) | Resolved. The 12-row derivation is recorded in the design's documentation plan, with unaffected rows named. |
| Ubiquitous language | Resolved. The glossary plan adds **Call shape** and **Call resolution** and updates the six affected entries. |

## Count of user questions

Zero. The one `needs user` question (Q6) was answered by the operator with
option A and is recorded as a design decision.

## Verification

`givn lint --change fix-outline-report-fidelity` exits 2 with 20 `@wip`
findings and no other finding or advisory: every new or modified delta
scenario keeps its `@wip` tag until its steps exist.

DESIGN-REVIEW: PASS
