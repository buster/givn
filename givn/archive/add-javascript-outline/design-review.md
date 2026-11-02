# Design Review: add-javascript-outline

Review kind: self-review
Author: Maki
Reviewer context: Maki

## Grilling

| # | Branch | Question | Resolution |
|---|---|---|---|
| Q1 | Scope | Does the delta contain exactly the proposal's JavaScript scope? | Yes. `.js`, `.mjs`, `.cjs`, class/function/field/arrow coverage, exclusions, malformed files, schema 5, mixed-language disclosure, and the JSX boundary. JSX, Go, import graphs, and type-aware resolution stay out. The class-field scenario was added during hardening because the proposal claims fields. |
| Q2 | Tech choices | Is the official JavaScript grammar the right choice over reusing the TypeScript grammar? | Yes. The TypeScript grammar conflates JavaScript and TypeScript dialect rules and does not make JavaScript-specific syntax honest. `tree-sitter-javascript 0.25.0` was resolved with `cargo info` on 2026-11-02 and its `tree-sitter-language 0.1` dependency matches the lockfile's 0.1.9. No simpler alternative satisfies the three extensions and the separate language label. |
| Q3 | Missing scenarios | Are observable behaviours missing from the spec? | The proposal claims class fields; no field scenario existed. Hardened with "A JavaScript class field is reported." Malformed files, exclusions, all extensions, arrow bindings, schema 5, mixed languages, and the JSX boundary already have scenarios. Unnamed class/function expressions are an internal limitation, not a proposal behaviour; the design names them and they are covered by adapter unit tests. |
| Q4 | Testability | Can every scenario fail RED, and are the assertions concrete? | Yes. Every new step is unbound or unimplemented until the implementation batch; `@wip` is present. The Then steps assert unit names, language values, type kind, field count, parse-failure path, absent output, schema version, and exit code. The runner uses `.fail_on_skipped()`. The single-scenario command was proven against a permanent scenario: `GIVN_FEATURES=givn/specs cargo nextest run --locked --test features_runner -- 'assess-code-structure/outline-report.feature::A Java production class is reported'` ran 1 test and passed. |
| Q5 | E2E fidelity | Does the plan redefine the real interface or add duplicate E2E coverage? | No. The real interface is the CLI subprocess. The capability's existing permanent `@e2e` scenario remains the one interaction entry; the JavaScript scenarios are regular variants of the same report action. No browser interface is involved. |
| Q6 | Interaction coverage | Does every use-case inventory entry map to a design row and an existing `@e2e` scenario? | Yes. `assess-code-structure` has one inventory row: run the report. The design matrix has one row, pointing at the permanent `@e2e` scenario "A developer runs a outline report and sees the classified units". The scenario uses the real `givn outline report` subprocess. |
| Q7 | Risk | What is the most likely implementation failure? | Tree-sitter JavaScript node kinds differ from TypeScript (`field_definition`, export wrappers, generator functions), so units could be silently missing while scenarios still pass for the wrong reason. Mitigation: adapter unit tests for class, field, method, function, arrow binding, calls, and malformed source; the Gherkin scenarios assert concrete unit and field names; parse errors are named. |
| Q8 | Use-case and Persona | Is the use case identity, contract, and Persona coverage intact? | Yes. Owning use case `assess-code-structure`; confirmed Personas `developer--primary-user`, `agent--technical-role`, `givn-maintainer--owner`. The out-of-scope entry no longer claims JavaScript is unsupported. Actors remain Developer and Authoring agent; no Persona biography is used as an actor. |
| Q9 | Use-case rejection criteria | Does the owning use case still pass the scope rejection criteria? | Yes. One goal verb, a primary Developer actor, a trigger naming the developer, no mutation under an observing goal, extensions named by main-flow step, consistent preconditions, no mechanism vocabulary in the goal or guarantees, and a goal-shaped `outline-report` capability. |
| Q10 | Decision ledger | Is every `D<n>` resolved? | Yes. D1 extensions, D2 separate language value, D3 schema 5, and D4 per-file parse failure are all answered in the proposal and realized in the design ledger. No decision is carried. |
| Q11 | Contracts and components | Does every contract have exactly one implementing component? | Yes. The outline report is the existing pipeline contract; JavaScript source analysis is a new contract implemented by `tree_sitter_adapter::JavaScriptAnalyzer` behind the existing `SourceAnalyzer` port. Consumers use the port, not the implementation. The split trigger is JSX, JavaScript dialect selection, or type-aware resolution. |
| Q12 | Decision timing | Is any foundational decision left to usage, or any speculative abstraction built? | No. All four decisions are foundational and answered. No carried decision exists. The shared syntax helpers are internal and do not create a speculative abstraction. |
| Q13 | Visual design contract | Is the interface classified correctly, and is Persona disposition recorded? | The interface is terminal text and JSON, not a browser-rendered screen. A browser driver is not applicable. The design now records a Persona disposition table for all three confirmed Personas. |
| Q14 | Deferrals | Is any mandatory check deferred? | No. |
| Q15 | ADR qualification | Does the language-coverage decision require a new ADR? | No. It extends an existing adapter behind the existing neutral port. It fixes no new component, authority, storage, or deployment boundary, and the use case plus design own the rationale. The recorded canonical artifacts are `givn/specs/assess-code-structure` and `docs/outline.md`. |
| Q16 | Arc42 | Does the independent 12-row assessment match `arc42.md`? | Yes, after checking each row against the proposal and design. See the table below. |
| Q17 | Arc42 integrity | Do all 12 chapter files exist, contain real content, and avoid ASCII diagrams? | Yes. All 12 files exist; the affected chapters were edited surgically. A scan found no Unicode box-drawing or pseudo-diagram blocks. |
| Q18 | Ubiquitous language | Are the glossary and structural model current? | `docs/glossary.md` now defines the outline report, production source, and Tree-sitter language adapter with JavaScript and schema 5. `docs/domain-model.md` explicitly excludes internal Rust components and file formats, so this adapter change has no structural-model impact. |
| Q19 | Test evidence | Does the plan prove strict mode and a single-scenario command? | Yes. `.fail_on_skipped()` is the strict-mode mechanism; the single-scenario command above was executed and passed. |

## Independent Arc42 Assessment

| # | Chapter | Affected? | Reason |
|---|---|---|---|
| 1 | 01 introduction-and-goals | Yes | FR-46 lists supported outline languages; JavaScript was added. |
| 2 | 02 architecture-constraints | Yes | TC-42 lists the parser set and offline constraint; the JavaScript grammar was added. |
| 3 | 03 context-and-scope | Yes | The report interface exposes schema 5 and the new language value. |
| 4 | 04 solution-strategy | Yes | The code-structure report strategy now covers five languages. |
| 5 | 05 building-block-view | Yes | `tree_sitter_adapter` gains the JavaScript responsibility. |
| 6 | 06 runtime-view | Yes | Discovery, parsing, and schema output change. |
| 7 | 07 deployment-view | No | The grammar is compiled into the existing binary; no new deployment unit. |
| 8 | 08 crosscutting-concepts | Yes | The outline analysis boundary and evaluation-scope sentence change. |
| 9 | 09 architecture-decisions | No | No new ADR; the decision is owned by the use case and design. |
| 10 | 10 quality-requirements | Yes | Q-112's unsupported-language example changes from JavaScript to JSX. |
| 11 | 11 risks-and-technical-debt | Yes | R-114, R-115, and the coverage backlog row change. |
| 12 | 12 glossary | Yes | Outline report, Production source, and Tree-sitter language adapter change. |

No row marked "No" in this table is marked "Yes" in `arc42.md`.

## Hardening Applied

- Added the `A JavaScript class field is reported` scenario to the delta spec.
- Added the class-field scenario to the design's use-case traceability and step-definition coverage.
- Added a Persona disposition table to the design.
- Confirmed and recorded the exact single-scenario command with a passing run.
- Re-ran `givn lint --change add-javascript-outline`; findings are the intentional `@wip` tags and advisory shape/subset notes, with no syntax or tag errors.

## Sign-off

DESIGN-REVIEW: PASS
