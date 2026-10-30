# Design Review: add-java-typescript-outline

Review kind: self-review
Author: Maki
Reviewer context: Maki

## Grilling log

| # | Branch | Question | Outcome |
|---|---|---|---|
| Q1 | Scope | Does the change keep Rust and Python on their existing adapters? | Closed — `outline_analyzer` composes them unchanged; only Java/TypeScript use Tree-sitter. |
| Q2 | Scope | Are Go, JavaScript, and JSX excluded? | Closed — the scanner leaves them unsupported and the Go-only scenario proves the refusal. |
| Q3 | Tech choices | Is a new parser stack needed per language? | Closed — one Tree-sitter runtime plus two grammar crates and explicit mappings. |
| Q4 | Missing scenarios | Are Java classes, TypeScript classes/functions, and interfaces covered? | Closed — separate scenarios assert language and type kind. |
| Q5 | Missing scenarios | Are exclusions and malformed files covered? | Closed — test/dependency exclusions and a malformed Java file beside valid TypeScript. |
| Q6 | Testability | Can each scenario fail meaningfully? | Closed — missing units/type kinds, schema mismatch, and failure diagnostics fail before implementation. |
| Q7 | Testability | Is strict mode unchanged? | Closed — `.fail_on_skipped()` remains. |
| Q8 | E2E fidelity | Does the change add a new E2E action? | Closed — the existing outline E2E remains; new scenarios are regular variants of the report action. |
| Q9 | Interaction Coverage | Is the inventory entry mapped? | Closed — `outline-report` maps to the existing E2E. |
| Q10 | Risk | What is the main implementation risk? | Closed — grammar node-kind drift and silent misclassification. Pinned grammars, per-language tests, and explicit unsupported notes mitigate. |
| Q11 | Risk | Could calls resolve across languages? | Closed — the resolver filters candidates by language; a shared method name cannot cross. |
| Q12 | Use-case and Persona context | Is the owning use case preserved? | Closed — `assess-code-structure`; Personas remain review lenses. |
| Q13 | Decision ledger | Do D1–D4 resolve? | Closed — Java/TypeScript, retain Rust/Python, interface type kind/schema 4, file-level failures. |
| Q14 | Contracts and components | Does one component implement each contract? | Closed — Java and TypeScript analyzers each own one adapter contract; the report owns the neutral pipeline. |
| Q15 | Decision timing | Is anything foundational carried? | Closed — none. |
| Q16 | Visual design contract | What is the interface? | Closed — terminal human/JSON report. |
| Q17 | ADR qualification | Does this need an ADR? | Closed — no; the use case and outline docs own it. |
| Q18 | Architecture documentation | Are affected chapters current? | Closed — chapters 1–6, 8, 10–12 updated. |
| Q19 | Ubiquitous language | Are interface and Tree-sitter adapter terms recorded? | Closed — glossary updated. |
| Q20 | Deferrals | Is anything deferred without authority? | Closed — Go/JS/JSX and Rust/Python replacement are out of scope. |

## Changes made during hardening

| Artifact | Change summary |
|---|---|
| `docs/outline.md` | Added Java/TypeScript support, exclusions, schema 4, and runtime limits. |
| `docs/arc42/01`, `02`, `03`, `04`, `05`, `06`, `08`, `10`, `11` | Updated language coverage, parser boundary, runtime, quality, and risk rows. |
| `docs/glossary.md` | Updated Outline report and Production source; added Tree-sitter language adapter. |
| `src/outline/tree_sitter_adapter/mod.rs` | Added body-member extraction, visibility, interface type kind, and adapter tests. |
| `specs/assess-code-structure/outline-report.feature` | Added Java/TypeScript, mixed-language, malformed, exclusion, and Go-only scenarios. |

## Sign-off

- [x] Every branch of the design tree has been walked.
- [x] Every proposal question ID (`D<n>`) resolves in the design's `## Decisions` ledger.
- [x] All questions are resolved.
- [x] `design.md` reflects the decisions reached during grilling.
- [x] `specs/**/*.feature` updated for the missing scenarios.
- [x] `givn lint --change add-java-typescript-outline` has no parse errors.
- [x] Architecture documentation (arc42) check completed.

**DESIGN-REVIEW: PASS**
