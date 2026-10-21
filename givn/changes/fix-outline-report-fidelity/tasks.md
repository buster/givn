# Tasks: fix-outline-report-fidelity

RED→GREEN→REFACTOR→COMMIT per delta scenario, in `.feature` order. All 20
delta scenarios are non-`@e2e`: the permanent `A developer runs a outline
report and sees the classified units` e2e remains the capability's single
real-interface action, and this change adds no interaction.

Legend: unchecked boxes are pending; check a box only when its evidence and
commit hash are filled in. Check each box immediately after its step — never
batch-check.

## Interaction coverage pre-flight

Owning use case: `assess-code-structure`
(`givn/specs/assess-code-structure/usecase.md`). Inventory cross-check
against design.md's Interaction Coverage Matrix:

| Inventory entry | Matrix row | Existing `@e2e` scenario | Gap |
|---|---|---|---|
| `outline-report` — run a outline report | Present | `A developer runs a outline report and sees the classified units` (`givn/specs/assess-code-structure/outline-report.feature`, `@e2e`) | none |

Result: **1/1 inventory entry mapped; no coverage gap.** The delta changes
what the same report prints; it introduces no new consumer action, no new
capability, and no new `@e2e` scenario.

## Domain constraints (from the use case and design)

- Goal: see the structure of an existing codebase and the units that are
  findings.
- Rule: the report leads with each unit's per-component call tree; same-unit
  calls nest and calls to another component are leaves.
- Rule: the report is deterministic, offline, and advisory; it emits no
  action or recommendation and stores no baseline.
- Rule: every analyzed unit has exactly one classification.
- Rule: conditional code is counted as written; `#[cfg(test)]` items are not
  production sources.
- Confirmed Personas: `developer--primary-user`, `agent--technical-role`,
  `givn-maintainer--owner`.
- Contracts used: `SourceAnalyzer` (ADR-074) supplies neutral facts; the
  outline call-tree contract renders text and JSON from one classified model.
  Consumers do not import an adapter implementation.

Common design constraints for every scenario:

- `CallShape` is `Function { qualifier: Option<String> }` or
  `Method { receiver: ReceiverEvidence }`; `ReceiverEvidence` is `SelfUnit`,
  `Qualifier(String)`, or `Unknown`. `Call` gains `shape` and keeps
  `resolution`/`target`.
- `same_file` / `project_other_file` require an owner consistent with the
  call shape. Unknown receivers cap at `name_only`; several candidates are
  `ambiguous`; a contradictory explicit qualifier is `external`.
- `name_only` keeps its unique candidate target but never nests and never
  contributes an LCOM4 edge.
- Nesting and LCOM4 use same-unit membership; a same-unit target in another
  file is labelled `project_other_file` and still nests.
- Evidence lists only predicates that fired. No recommendation or action is
  added.
- JSON schema stays `3`, rules stay `1`; call objects add `target`; human
  call tags are `[resolution, visibility, owner]`; ambiguous stays
  `[ambiguous] callee`.
- Production file code lines exclude recorded `#[cfg(test)]` spans; unit
  physical and code lines use the unit's spans and subtract those exclusions.
- Module outlines list member type items; type items carry no signature and
  no calls and never enter the method index.
- Signatures render paths and generic arguments recursively without a new
  dependency.
- Gate selection uses units with at least one method, not a non-empty
  outline.

## Setup — runner, strict mode, and baseline (no scenario commit)

- [x] **Verify the runner and strict mode.**
      Confirm `givn/commands.yaml` sets `verify.command` to
      `./run-tests.sh --non-e2e` and `verify.e2e_command` to
      `./run-tests.sh --e2e`; confirm `tests/features_runner.rs` calls
      `.fail_on_skipped()` on both execution paths and exits `1` when
      `stats.failed > 0 || stats.skipped > 0`.
      Proof: create a scratch feature whose only step is undefined, run
      `GIVN_FEATURES=<scratch-dir> cargo test --locked --test features_runner -- --name "Undefined step fails"`,
      and paste the non-zero output below. Remove the scratch feature.
      - [x] Evidence: `GIVN_FEATURES=/tmp/givn-strict-probe cargo test --locked --test features_runner -- --name "Undefined step fails"`
            → `1 scenario (1 failed)`, `Step doesn't match any function`,
            `exit status: 1`. Probe removed.
      - [x] Baseline: `GIVN_FEATURES=givn/changes/fix-outline-report-fidelity/specs cargo test --locked --test features_runner`
            → `20 scenarios (20 failed)`, `24 steps (4 passed, 20 failed)`,
            `exit status: 1`; pending is not green.

## Commit convention

One atomic commit per scenario. Subject is the release note
`fix(outline): make call graphs, module outlines, and metrics faithful`;
body is the scenario title.

```
givn commit -m "$(printf '%s\n\n%s\n' \
  'fix(outline): make call graphs, module outlines, and metrics faithful' \
  '<scenario title>')"
```

Before every scenario commit run `cargo fmt --all` and
`cargo clippy --locked --all-targets -- -D warnings`. Never check a box
before the commit exists. Record exactly one reachable `COMMIT:` revision per
scenario.

---

## 1. A method call on an unknown receiver is a name-only guess

Design constraints: `CallShape`/`ReceiverEvidence`; an `ExprMethodCall`
whose receiver is not `self` and not a statically named owner is
`ReceiverEvidence::Unknown`; the resolver returns `name_only` with the
unique candidate target. JSON calls gain `target`.

- RED: remove `@wip` from this scenario in
  `givn/changes/fix-outline-report-fidelity/specs/assess-code-structure/outline-report.feature`;
  add the fixture `write_unknown_receiver_call` and step bindings with
  `unimplemented!("TODO")`; run
  `GIVN_FEATURES=givn/changes/fix-outline-report-fidelity/specs cargo test --locked --test features_runner -- --name "A method call on an unknown receiver is a name-only guess"`;
  paste the non-zero output.
  - [x] Evidence: baseline: ✘  Given a production source unit whose method calls a receiver of unknown type | Defined: givn/changes/fix-outline-report-fidelity/specs/assess-code-structure/outline-report.feature:7:5 | Step doesn't match any function
- GREEN: implement the model fields, Rust call-shape extraction, the resolver
  unknown-receiver branch, and JSON `target`. List production files:
  `src/outline/model.rs`, `src/outline/resolve.rs`,
  `src/outline/syn_adapter/facts.rs`, `src/outline/python_adapter/mod.rs`,
  `src/outline/report.rs` (constructor/test updates as needed). Run the same
  targeted command; paste the zero output.
  - [x] Evidence:   Scenario: A method call on an unknown receiver is a name-only guess | 1 scenario (1 passed) | 4 steps (4 passed)
- REFACTOR: clean up, re-run the targeted command, paste the zero output.
  - [x] Evidence:   Scenario: A method call on an unknown receiver is a name-only guess | 1 scenario (1 passed) | 4 steps (4 passed)
- [x] COMMIT: b1fffc9e26fd686eaa1164837179b592871893e3

## 2. A name-only guess is not expanded as a same-unit call

Design constraints: render `name_only` as a leaf; never recurse into its
target; LCOM4 unions only `same_file`/`project_other_file` same-unit edges.
The fixture must put the candidate in the caller's unit so the old renderer
would wrongly expand it.

- RED: remove `@wip`; add the fixture and bindings (stubs); run the targeted
  command; paste non-zero.
  - [x] Evidence: baseline: ✘  Given a production source unit whose method calls a receiver of unknown type | Defined: givn/changes/fix-outline-report-fidelity/specs/assess-code-structure/outline-report.feature:14:5 | Step doesn't match any function
- GREEN: update `render_calls`, `metrics::cohesion`; run targeted; paste zero.
  - [x] Evidence:   Scenario: A name-only guess is not expanded as a same-unit call | 1 scenario (1 passed) | 5 steps (5 passed)
- REFACTOR: re-run targeted; paste zero.
  - [x] Evidence:   Scenario: A name-only guess is not expanded as a same-unit call | 1 scenario (1 passed) | 5 steps (5 passed)
- [x] COMMIT: 8144cca87d0a24175c92c68785b3d51c8085e6ef

## 3. A call on the unit's own receiver resolves within the unit

Design constraints: `self.method()` / `&self` is `SelfUnit`; the resolver
selects the current unit's method; `same_file` keeps its target and nests.

- RED: remove `@wip`; add `write_self_receiver_call` and bindings; targeted
  run; paste non-zero.
  - [x] Evidence: baseline: ✘  Given a production source unit whose method calls another method on itself | Defined: givn/changes/fix-outline-report-fidelity/specs/assess-code-structure/outline-report.feature:22:5 | Step doesn't match any function
- GREEN: add `SelfUnit` extraction and resolution; targeted run; paste zero.
  - [x] Evidence:   Scenario: A call on the unit's own receiver resolves within the unit | 1 scenario (1 passed) | 6 steps (6 passed)
- REFACTOR: targeted run; paste zero.
  - [x] Evidence:   Scenario: A call on the unit's own receiver resolves within the unit | 1 scenario (1 passed) | 6 steps (6 passed)
- [x] COMMIT: 1b37904d21dac651b575bd18159164c2e061d385

## 4. A method call on a named receiver resolves to that owner

Design constraints: `Type::method()` is a function call with qualifier
`Type`; a method call on a named receiver is `Qualifier("Type")`; the
resolver accepts candidates whose owner name equals the qualifier.

- RED: remove `@wip`; add `write_named_receiver_call` and bindings; targeted
  run; paste non-zero.
  - [x] Evidence: baseline: ✘  Given a production source unit whose method calls an associated function on a project type | Defined: givn/changes/fix-outline-report-fidelity/specs/assess-code-structure/outline-report.feature:31:5 | Step doesn't match any function
- GREEN: add qualifier extraction and owner filtering; targeted run; paste
  zero.
  - [x] Evidence:   Scenario: A method call on a named receiver resolves to that owner | 1 scenario (1 passed) | 5 steps (5 passed)
- REFACTOR: targeted run; paste zero.
  - [x] Evidence:   Scenario: A method call on a named receiver resolves to that owner | 1 scenario (1 passed) | 5 steps (5 passed)
- [x] COMMIT: d3b72a1353ea16aadb76cd184aaeb941a96911c9

## 5. An ambiguous method name has no resolved target

Design constraints: several compatible candidates produce `ambiguous` with
no target; JSON emits `null`.

- RED: remove `@wip`; add `write_ambiguous_call` and bindings; targeted run;
  paste non-zero.
  - [x] Evidence: baseline: ✘  Given a production source unit whose method calls an unknown receiver whose name two project types declare | Defined: givn/changes/fix-outline-report-fidelity/specs/assess-code-structure/outline-report.feature:39:5 | Step doesn't match any function
- GREEN: implement the ambiguous branch; targeted run; paste zero.
  - [x] Evidence:   Scenario: An ambiguous method name has no resolved target | 1 scenario (1 passed) | 4 steps (4 passed)
- REFACTOR: targeted run; paste zero.
  - [x] Evidence:   Scenario: An ambiguous method name has no resolved target | 1 scenario (1 passed) | 4 steps (4 passed)
- [x] COMMIT: ea3d014306923460a1423f0746cebd6e65c2f140

## 6. An associated-function call whose qualifier contradicts the target is external

Design constraints: explicit qualifier `Line` cannot match candidate owner
`Error`; return `external` with no target; never fall back to a name guess.

- RED: remove `@wip`; add `write_contradictory_qualifier` and bindings;
  targeted run; paste non-zero.
  - [x] Evidence: baseline: ✘  Given a production source unit that calls "Line::from" while the project declares "Error::from" | Defined: givn/changes/fix-outline-report-fidelity/specs/assess-code-structure/outline-report.feature:46:5 | Step doesn't match any function
- GREEN: implement qualifier mismatch -> external; targeted run; paste zero.
  - [x] Evidence:   Scenario: An associated-function call whose qualifier contradicts the target is external |    ✔  Given a production source unit that calls "Line::from" while the project declares "Error::from" | 1 scenario (1 passed) | 4 steps (4 passed)
- REFACTOR: targeted run; paste zero.
  - [x] Evidence:   Scenario: An associated-function call whose qualifier contradicts the target is external |    ✔  Given a production source unit that calls "Line::from" while the project declares "Error::from" | 1 scenario (1 passed) | 4 steps (4 passed)
- [x] COMMIT: 43cee9c62e34c3169f98c34d7ac4cb779f2f21eb

## 7. A god type found through cohesion reports only the cohesion evidence

Design constraints: evidence is built per fired predicate. LCOM4 > 2 emits
`type-lcom4 > 2`; TCC < 500 emits `type-tcc < 500`; both only when both
fired. A fixture with LCOM4 <= 2 and TCC < 500 must emit no LCOM4 line.

- RED: remove `@wip`; add `write_tcc_god_type` and bindings; targeted run;
  paste non-zero.
  - [x] Evidence: baseline: ✘  Given a production source unit over the readable size whose methods are connected only by calls | Defined: givn/changes/fix-outline-report-fidelity/specs/assess-code-structure/outline-report.feature:52:5 | Step doesn't match any function
- GREEN: rewrite the god-type evidence branch; targeted run; paste zero.
  - [x] Evidence:   Scenario: A god type found through cohesion reports only the cohesion evidence | 1 scenario (1 passed) | 5 steps (5 passed)
- REFACTOR: targeted run; paste zero.
  - [x] Evidence:   Scenario: A god type found through cohesion reports only the cohesion evidence | 1 scenario (1 passed) | 5 steps (5 passed)
- [x] COMMIT: 9e3487b175791adcd43832e31ae457ad7171f3cc

## 8. An interface found through its public methods reports only public-method evidence

Design constraints: public-methods branch emits `type-public-methods > 50`;
the trait predicate is emitted only when the trait-and-size predicate fired;
no `type-kind-trait` for a public-method-only interface.

- RED: remove `@wip`; extend the public-methods fixture and bindings;
  targeted run; paste non-zero.
  - [x] Evidence: baseline: ✘  And the report records the evidence "type-public-methods > 50" | Defined: givn/changes/fix-outline-report-fidelity/specs/assess-code-structure/outline-report.feature:63:5 | Step doesn't match any function
- GREEN: rewrite the interface evidence branch; targeted run; paste zero.
  - [x] Evidence:   Scenario: An interface found through its public methods reports only public-method evidence | 1 scenario (1 passed) | 5 steps (5 passed)
- REFACTOR: targeted run; paste zero.
  - [x] Evidence:   Scenario: An interface found through its public methods reports only public-method evidence | 1 scenario (1 passed) | 5 steps (5 passed)
- [x] COMMIT: 59b2188f78e430bde5625a39bc920609d94166b7

## 9. A long method found through complexity reports only complexity evidence

Design constraints: `method-code-lines > 100` and `method-complexity > 50`
are independent; emit only the predicate that fired.

- RED: remove `@wip`; add `write_complexity_method` and bindings; targeted
  run; paste non-zero.
  - [x] Evidence: baseline: ✘  Given a production source unit with a method over the complexity limit but within the length limit | Defined: givn/changes/fix-outline-report-fidelity/specs/assess-code-structure/outline-report.feature:68:5 | Step doesn't match any function
- GREEN: rewrite the method-length evidence branch and emit separate
  complexity evidence; targeted run; paste zero.
  - [x] Evidence:   Scenario: A long method found through complexity reports only complexity evidence | 1 scenario (1 passed) | 5 steps (5 passed)
- REFACTOR: targeted run; paste zero.
  - [x] Evidence:   Scenario: A long method found through complexity reports only complexity evidence | 1 scenario (1 passed) | 5 steps (5 passed)
- [x] COMMIT: 2a98d7fc7698b2e3fad028b017d85d42241f0709

## 10. A Rust module outline lists the types it declares

Design constraints: both adapters add `OutlineKind::Type` items to module
outlines; Rust derives them from declared type units; type items carry an
empty signature and no calls and never enter `MethodIndex`.

- RED: remove `@wip`; add `write_module_with_types` and bindings; targeted
  run; paste non-zero.
  - [x] Evidence: baseline: ✘  Given a production Rust module with three declared types and one free function | Defined: givn/changes/fix-outline-report-fidelity/specs/assess-code-structure/outline-report.feature:76:5 | Step doesn't match any function
- GREEN: add Rust module type items; targeted run; paste zero.
  - [x] Evidence:   Scenario: A Rust module outline lists the types it declares | 1 scenario (1 passed) | 3 steps (3 passed)
- REFACTOR: targeted run; paste zero.
  - [x] Evidence:   Scenario: A Rust module outline lists the types it declares | 1 scenario (1 passed) | 3 steps (3 passed)
- [x] COMMIT: f7e7e526be5db6dd3ad554778f394f812990752a

## 11. A module outline interleaves member types and free functions in source order

Design constraints: module outline items sort by `(order, name)`; free
functions and type items interleave by declaration line.

- RED: remove `@wip`; add the interleave fixture and bindings; targeted run;
  paste non-zero.
  - [x] Evidence: baseline: ✘  Given a production Rust module with a free function, a type, a free function, and a type in that order | Defined: givn/changes/fix-outline-report-fidelity/specs/assess-code-structure/outline-report.feature:82:5 | Step doesn't match any function
- GREEN: sort module outlines and set type-item order; targeted run; paste
  zero.
  - [x] Evidence:   Scenario: A module outline interleaves member types and free functions in source order | 1 scenario (1 passed) | 3 steps (3 passed)
- REFACTOR: targeted run; paste zero.
  - [x] Evidence:   Scenario: A module outline interleaves member types and free functions in source order | 1 scenario (1 passed) | 3 steps (3 passed)
- [x] COMMIT: e5e08c8fe7e17ad60366687e9966c887e766d8e1

## 12. A Python module outline lists its classes

Design constraints: the Python adapter adds type items for class units to
the module outline, with the class unit id, visibility, and declaration
line; no calls/signature.

- RED: remove `@wip`; add `write_python_classes_module` and bindings;
  targeted run; paste non-zero.
  - [x] Evidence: baseline: ✘  Given a Python module with two classes and one function | Defined: givn/changes/fix-outline-report-fidelity/specs/assess-code-structure/outline-report.feature:88:5 | Step doesn't match any function
- GREEN: add Python module type items; targeted run; paste zero.
  - [x] Evidence:   Scenario: A Python module outline lists its classes | 1 scenario (1 passed) | 3 steps (3 passed)
- REFACTOR: targeted run; paste zero.
  - [x] Evidence:   Scenario: A Python module outline lists its classes | 1 scenario (1 passed) | 3 steps (3 passed)
- [x] COMMIT: b8ede9d98bf499b19601f5ced21b71ed085cafaa

## 13. The JSON report names each call's resolved target

Design constraints: JSON call objects add `target` (method id or `null`);
module free functions keep their module method id.

- RED: remove `@wip`; extend the cross-component fixture and bindings;
  targeted run; paste non-zero.
  - [x] Evidence: baseline: ✘  Given a production source unit whose method calls a module function | Defined: givn/changes/fix-outline-report-fidelity/specs/assess-code-structure/outline-report.feature:94:5 | Matched: tests/steps/outline_report_steps.rs:984:1
- GREEN: include `target` in `outline_json`; targeted run; paste zero.
  - [x] Evidence:   Scenario: The JSON report names each call's resolved target | 1 scenario (1 passed) | 3 steps (3 passed)
- REFACTOR: targeted run; paste zero.
  - [x] Evidence:   Scenario: The JSON report names each call's resolved target | 1 scenario (1 passed) | 3 steps (3 passed)
- [x] COMMIT: 631f483d011cce928f6b1bea61d0bf2ff9135cfb

## 14. A same-unit method implemented in another file is project_other_file

Design constraints: `same_file` compares the caller's method span file with
the target's span file; same-unit nesting uses unit membership, not the
label; LCOM4 uses same-unit membership. A cross-file impl method is labelled
`project_other_file` and still nests.

- RED: remove `@wip`; add the cross-file impl fixture and bindings; targeted
  run; paste non-zero.
  - [x] Evidence: baseline: ✘  Given a production source unit declared in one file and implemented in another | Defined: givn/changes/fix-outline-report-fidelity/specs/assess-code-structure/outline-report.feature:100:5 | Step doesn't match any function
- GREEN: compare files for the label and decouple nesting/LCOM4 from the
  label; targeted run; paste zero.
  - [x] Evidence:   Scenario: A same-unit method implemented in another file is project_other_file | 1 scenario (1 passed) | 5 steps (5 passed)
- REFACTOR: targeted run; paste zero.
  - [x] Evidence:   Scenario: A same-unit method implemented in another file is project_other_file | 1 scenario (1 passed) | 5 steps (5 passed)
- [ ] COMMIT:

## 15. A developer reads each call's resolution in the human report

Design constraints: human tags lead with the resolution label:
`[same_file, public, Engine]`, `[project_other_file, public, module]`,
`[name_only, public, Engine]`; ambiguous stays `[ambiguous] callee`;
name-only never nests.

- RED: remove `@wip`; add `write_resolution_report` and bindings; targeted
  run; paste non-zero.
  - [x] Evidence: baseline: ✘  And a production source unit whose method calls its own method, a cross-file module function, and a receiver of unknown type | Defined: givn/changes/fix-outline-report-fidelity/specs/assess-code-structure/outline-report.feature:109:5 | Step doesn't match any function
- GREEN: render the resolution label; targeted run; paste zero.
  - [x] Evidence:   Scenario: A developer reads each call's resolution in the human report | 1 scenario (1 passed) | 7 steps (7 passed)
- REFACTOR: targeted run; paste zero.
  - [x] Evidence:   Scenario: A developer reads each call's resolution in the human report | 1 scenario (1 passed) | 7 steps (7 passed)
- [x] COMMIT: 240bdaadd0ade475b6ce74962c3d5f2b5f40d17e

## 16. A file's code lines exclude inline test modules

Design constraints: `AnalyzedFile.excluded_spans` records every
`#[cfg(test)]` item/impl-method span; `files[].code_lines` counts code lines
outside those spans; file `physical_lines` stays the whole file.

- RED: remove `@wip`; add `write_inline_test_module` with exact counts and
  bindings; targeted run; paste non-zero.
  - [x] Evidence: baseline: ✘  Given a production source file with an inline test module | Defined: givn/changes/fix-outline-report-fidelity/specs/assess-code-structure/outline-report.feature:118:5 | Step doesn't match any function
- GREEN: add `excluded_spans`, record Rust test spans, add
  `scan::production_code_lines`, use it in `file_json`; targeted run; paste
  zero.
  - [x] Evidence:   Scenario: A file's code lines exclude inline test modules | 1 scenario (1 passed) | 4 steps (4 passed)
- REFACTOR: targeted run; paste zero.
  - [x] Evidence:   Scenario: A file's code lines exclude inline test modules | 1 scenario (1 passed) | 4 steps (4 passed)
- [x] COMMIT: 887b990d5c4cb1ea2209ae34ee2cd0c92601230a

## 17. A unit's physical lines cover only its own spans

Design constraints: unit physical lines are the union of the unit's spans
per file, regardless of line kind; unit code lines are unchanged except for
excluded spans.

- RED: remove `@wip`; add `write_span_scoped_unit` and bindings; targeted
  run; paste non-zero.
  - [x] Evidence: baseline: ✘  Given a production source file with a small type and a large unrelated function | Defined: givn/changes/fix-outline-report-fidelity/specs/assess-code-structure/outline-report.feature:125:5 | Step doesn't match any function
- GREEN: compute physical lines from span unions in `metrics.rs`; targeted
  run; paste zero.
  - [x] Evidence:   Scenario: A unit's physical lines cover only its own spans | 1 scenario (1 passed) | 5 steps (5 passed)
- REFACTOR: targeted run; paste zero.
  - [x] Evidence:   Scenario: A unit's physical lines cover only its own spans | 1 scenario (1 passed) | 5 steps (5 passed)
- [x] COMMIT: bda7ebdc0a7cd254da6e350c20115a1a70e5329e

## 18. An impl-level test method is excluded from the unit's lines

Design constraints: subtract `AnalyzedFile.excluded_spans` from both unit
code lines and physical lines; a `#[cfg(test)]` method inside a non-test
impl is excluded; the production method count stays correct.

- RED: remove `@wip`; add `write_impl_test_method` with exact counts and
  bindings; targeted run; paste non-zero.
  - [x] Evidence: baseline: ✘  Given a production source unit with an impl-level test method | Defined: givn/changes/fix-outline-report-fidelity/specs/assess-code-structure/outline-report.feature:133:5 | Step doesn't match any function
- GREEN: apply exclusions in unit metrics; targeted run; paste zero.
  - [x] Evidence:   Scenario: An impl-level test method is excluded from the unit's lines | 1 scenario (1 passed) | 5 steps (5 passed)
- REFACTOR: targeted run; paste zero.
  - [x] Evidence:   Scenario: An impl-level test method is excluded from the unit's lines | 1 scenario (1 passed) | 5 steps (5 passed)
- [x] COMMIT: ee1f69dabc247c37c8e86c383ca64ae067b434df

## 19. A name-only guess does not connect methods for cohesion

Design constraints: LCOM4 unions only resolved same-file/project-other-file
same-unit edges; name-only edges never connect methods. The classification
rule set and thresholds are unchanged; a corrected input may change an
individual unit's classification.

- RED: remove `@wip`; add `write_name_only_cohesion` and bindings; targeted
  run; paste non-zero.
  - [x] Evidence: baseline: ✘  Given a production source unit over the readable size whose only call edges are name-only guesses | Defined: givn/changes/fix-outline-report-fidelity/specs/assess-code-structure/outline-report.feature:141:5 | Step doesn't match any function
- GREEN: filter LCOM4 edges by resolution; targeted run; paste zero.
  - [x] Evidence:   Scenario: A name-only guess does not connect methods for cohesion | 1 scenario (1 passed) | 5 steps (5 passed)
- REFACTOR: targeted run; paste zero.
  - [x] Evidence:   Scenario: A name-only guess does not connect methods for cohesion | 1 scenario (1 passed) | 5 steps (5 passed)
- [ ] COMMIT:

## 20. A method signature keeps arrays, qualified paths, and its full type detail

Design constraints: recursive `syn::Type` rendering; paths with every
segment and generic arguments; references, tuples, slices, arrays, pointers,
trait objects, impl trait, never, infer; no new dependency.

- RED: remove `@wip`; add `write_signature_shapes` and bindings; targeted
  run; paste non-zero.
  - [ ] Evidence:
- GREEN: replace `type_label` with the recursive renderer; targeted run;
  paste zero.
  - [ ] Evidence:
- REFACTOR: targeted run; paste zero.
  - [ ] Evidence:
- [ ] COMMIT:

---

## Final verification

- [ ] `./run-tests.sh --non-e2e` exits 0.
- [ ] `./run-tests.sh --e2e` exits 0.
- [ ] `cargo fmt --all` clean.
- [ ] `cargo clippy --locked --all-targets -- -D warnings` clean.
- [ ] `givn lint --change fix-outline-report-fidelity` exits 0 after `@wip`
      removal.
- [ ] `docs/outline.md`, the arc42 chapters named in design.md, and
      `docs/arc42/12-glossary.md` are updated.
- [ ] `givn/specs/assess-code-structure/usecase.md` interaction inventory
      still has exactly one entry and one `@e2e` scenario.
