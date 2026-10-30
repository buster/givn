# Proposal: evaluate-tree-sitter-outline

## Release note

Outline analysis now records a differential Tree-sitter evaluation for Rust and Python and retains the current adapters where parity is incomplete.

## Use-Case Context

- Use-case ID: `assess-code-structure` — supporting owner: `obtain-effective-guidance` for the unchanged outline guidance.
- Ideation topic: none
- Seed path: none
- Confirmed Personas: `developer--primary-user`, `agent--technical-role`, `givn-maintainer--owner` — their existing authority and goals apply.
- Source: `docs/improvements/0.9.14/outline-languages.md` records the confirmed evaluation-first sequence. The replacement and additional-language decisions remain after the comparison.

## Review Context

- Review kind: self-review
- Reason: test-only evaluation adapters, one regression scenario for the unchanged default outline engine, and a decision record. No production outline behavior, public command, or persistent format changes.

## Problem / Opportunity

The outline report supports Rust through `syn` and Python through a bundled `ast` extractor that requires `python3`. Adding each new language with a separate parser repeats integration work and may require the analyzed project's toolchain.

Tree-sitter could provide one parsing runtime and in-process analysis, but a grammar is not an outline implementation. Before replacing either adapter or adding a language, Givn needs evidence that Tree-sitter reproduces the facts the current outline needs: units and ownership, methods, fields, calls, spans, unsupported constructs, and deterministic reports.

The improvement note confirms an evaluation-first sequence: build parallel Rust and Python adapters, compare the required facts, record a replace-or-retain decision, and only then consider additional languages.

## Proposed Solution

Givn now records a differential Tree-sitter evaluation for Rust and Python.

- The evaluation parses the same representative production sources with Tree-sitter and the tree-sitter Rust/Python grammars.
- It extracts units, methods, fields, calls, spans, unsupported constructs, and parse errors into the same fact categories the current outline uses.
- It compares those facts with the current adapters and classifies each category as equivalent, extra syntax, or missing required facts.
- The evaluation is a test-only dev dependency. The production outline command, schema version, supported languages, and default adapters are unchanged.
- The decision record retains the current Rust and Python adapters for 0.9.14 because the comparison found missing required facts. No new language is added.
- A regression scenario confirms the default JSON report still uses schema version 3 and names Rust and Python.

## Capability Routing

`givn spec route --change evaluate-tree-sitter-outline` is not expected to signal a new capability; the work refines the existing outline-report behaviour and its evidence.

| Proposed capability | Route's recommendation | Decision | Rationale |
|---|---|---|---|
| `outline-report` | no signal | `EXTEND outline-report` | The change protects the existing outline report and adds a test-only evaluation; it does not add a public outline capability. |

## Out of Scope

- Replacing `syn` or the Python `ast` adapter.
- Removing the `python3` runtime requirement.
- Adding TypeScript, Go, Java, or any other language.
- Changing the outline schema, rules version, public CLI, or default engine.
- Partial-outline behavior for malformed files.
- Runtime grammar downloads, a plugin framework, or user-supplied parsers.

## Open Questions

### D1 — Replace or retain the current Rust and Python adapters?

- Status: answered 2026-10-30 by the evaluation recorded in this change
- Disposition: retain. The differential comparison records missing required facts in units, methods, calls, and spans; a replacement is not supported. The test-only adapter remains evidence for a later change.

### D2 — Which additional languages should be added?

- Status: out of scope for 0.9.14
- Disposition: out of scope. The confirmed sequence adds languages only after the Rust/Python comparison and replacement decision; this release records that decision and adds none.

### D3 — Does malformed Tree-sitter input enable partial analysis?

- Status: answered by the improvement's conservative recommendation
- Disposition: no. A file with error nodes is recorded as a parse error; no partial outline or apparently complete measurement is claimed. The current file-level boundary remains.
