# Design Review: evaluate-tree-sitter-outline

Review kind: self-review
Author: Maki
Reviewer context: Maki

## Grilling log

| # | Branch | Question | Outcome |
|---|---|---|---|
| Q1 | Scope | Is the change evaluation-only, as the confirmed sequence requires? | Closed — no production adapter, default switch, new language, or CLI change. |
| Q2 | Scope | Does the evaluation decide replacement, or merely parse successfully? | Closed — it compares required fact categories and records missing required facts. |
| Q3 | Tech choices | Is Tree-sitter in the release binary? | Closed — dev dependencies only; the production outline command is unchanged. |
| Q4 | Missing scenarios | Does a scenario protect the default engine? | Closed — the JSON report still uses schema 3 and names Rust/Python. |
| Q5 | Testability | Can the evaluation fail meaningfully? | Closed — missing categories produce `RETAIN`; malformed input is recorded as an error. |
| Q6 | Testability | Is the decision record tied to the test? | Closed — the unit test reads the record and checks the decision and categories. |
| Q7 | Risk | What is the main implementation risk? | Closed — grammar/toolchain breakage and mistaking the minimal adapter for parity. Dev-only scope, pinned lockfile, and `RETAIN` mitigate both. |
| Q8 | Use-case and Persona context | Is the owning use case preserved? | Closed — `assess-code-structure`; confirmed Personas remain review lenses. |
| Q9 | Decision ledger | Do D1–D3 resolve? | Closed — retain, out of scope, file-level failure. |
| Q10 | Contracts and components | Does the change add a production contract? | Closed — the outline report contract is unchanged; the evaluation is test evidence. |
| Q11 | Visual design contract | What is the interface? | Closed — unchanged terminal JSON report. |
| Q12 | ADR qualification | Does the evaluation need an ADR? | Closed — no; the use case and evaluation record own it. |
| Q13 | Architecture documentation | Are the affected chapters current? | Closed — 8.26, R-130/R-131, and the glossary are updated. |
| Q14 | Ubiquitous language | Is the evaluation term recorded? | Closed — `docs/glossary.md`. |
| Q15 | Deferrals | Is replacement or a new language deferred without authority? | Closed — both are explicitly out of scope; the confirmed sequence places them after the decision. |

## Changes made during hardening

| Artifact | Change summary |
|---|---|
| `docs/outline-tree-sitter-evaluation.md` | Added the comparison matrix, observed gaps, distribution note, and retain decision. |
| `specs/assess-code-structure/outline-report.feature` | Added the default-engine regression scenario with a source fixture. |
| `src/outline/tree_sitter_eval/mod.rs` | Added the decision-record test and malformed-input test. |
| `docs/arc42/08-crosscutting-concepts.md` | Recorded the test-only evaluation and decision. |
| `docs/arc42/11-risks-and-technical-debt.md` | Added R-130 and R-131. |
| `docs/glossary.md` | Added Tree-sitter evaluation. |

## Sign-off

- [x] Every branch of the design tree has been walked.
- [x] Every proposal question ID (`D<n>`) resolves in the design's `## Decisions` ledger.
- [x] All questions are resolved.
- [x] `design.md` reflects the decisions reached during grilling.
- [x] `specs/**/*.feature` updated for the missing scenario.
- [x] `givn lint --change evaluate-tree-sitter-outline` has no parse errors.
- [x] Architecture documentation (arc42) check completed.

**DESIGN-REVIEW: PASS**
