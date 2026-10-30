# Proposal: add-java-typescript-outline

## Release note

Outline analysis now supports Java and TypeScript through bundled Tree-sitter grammars while keeping the existing Rust and Python analyzers.

## Use-Case Context

- Use-case ID: `assess-code-structure` — supporting owner: `obtain-effective-guidance` for the unchanged outline guidance.
- Ideation topic: none
- Seed path: none
- Confirmed Personas: `developer--primary-user`, `agent--technical-role`, `givn-maintainer--owner`.
- Source: `docs/improvements/0.9.14/outline-languages.md`; the operator selected Java and TypeScript as the first additional languages and required keeping Rust and Python on their current adapters.

## Review Context

- Review kind: self-review
- Reason: advisory outline report language coverage and a schema version bump; no stored data, security boundary, or lifecycle gate changes. The report remains deterministic and advisory.

## Problem / Opportunity

The outline report supports production Rust and Python. Java and TypeScript production code receives no structural outline, so reviewers must assess those components directly from source during REFACTOR and review.

The Tree-sitter evaluation established that replacing the Rust and Python adapters would lose required facts. The confirmed direction is therefore to keep those adapters and use Tree-sitter for additional languages.

Java and TypeScript are the selected first set. Java classes, interfaces, enums, records, methods, fields, and calls map to the existing neutral units. TypeScript classes, interfaces, enums, type aliases, functions, methods, properties, and calls map similarly; `.ts` and `.tsx` select the dialect grammar.

## Proposed Solution

Givn now supports Java and TypeScript in `givn outline report` without changing the Rust or Python implementations.

- Java is parsed in-process with a bundled Tree-sitter Java grammar. Classes, interfaces, enums, records, methods, constructors, fields, and calls are reported. Package-private visibility is reported as crate scope; records are reported as classes with an explicit limitation.
- TypeScript is parsed in-process with the bundled TypeScript or TSX grammar, selected by `.ts` or `.tsx`. Classes, interfaces, enums, type aliases, functions, named arrow functions, methods, properties, and calls are reported. Declaration files and test/spec sources are excluded.
- The neutral model gains an `interface` type kind. The JSON schema advances to version 4 for the added language values and type kind.
- Java and TypeScript need no language interpreter. The grammars are compiled into the binary; no Node, Tree-sitter CLI, or runtime grammar download is required.
- Calls remain structural. The shared resolver never resolves a call across languages, and an unproven receiver remains name-only.
- Unsupported Go, JavaScript, and JSX production sources remain source-assessment-only. A malformed Java or TypeScript file is a per-file parse failure; other files continue to report.

## Capability Routing

`givn spec route --change add-java-typescript-outline` reports `EXTEND givn-workflow-enforcement` (signal), which is the verification owner rather than the outline owner.

| Proposed capability | Route's recommendation | Decision | Rationale |
|---|---|---|---|
| `outline-report` | `EXTEND givn-workflow-enforcement` | `EXTEND outline-report` | The change extends the existing advisory report and its neutral analyzer port; it adds no lifecycle or review capability. |

## Out of Scope

- Replacing the Rust `syn` or Python `ast` adapters.
- Removing the Python `python3` requirement.
- Go, JavaScript, JSX, or any other language.
- Cross-language call resolution.
- Partial outlines for malformed files.
- Runtime grammar downloads, user-supplied parsers, or a plugin framework.
- Changing the advisory, non-gating behavior of the report.

## Open Questions

### D1 — Which additional languages?

- Status: answered 2026-10-30 (user)
- Disposition: Java and TypeScript. Go, JavaScript, and JSX remain out of scope.

### D2 — Replace Rust and Python?

- Status: answered by the 0.9.14 evaluation and the operator
- Disposition: keep the existing Rust and Python adapters. Tree-sitter is used for Java and TypeScript only.

### D3 — How are Java and TypeScript interfaces represented?

- Status: answered in this change
- Disposition: a new neutral `interface` type kind. It is not silently relabeled as a class or trait. The JSON schema advances to version 4.

### D4 — What happens on a malformed Java or TypeScript file?

- Status: answered by the improvement's conservative recommendation
- Disposition: file-level parse failure with a diagnostic; no partial outline is claimed, and other files continue.
