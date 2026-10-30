# Tasks: add-java-typescript-outline

Owning use case: `assess-code-structure`. Personas: `developer--primary-user`,
`agent--technical-role`, `givn-maintainer--owner`.

Release note: Outline analysis now supports Java and TypeScript through bundled Tree-sitter grammars while keeping the existing Rust and Python analyzers.

## Setup

- [x] Confirm the existing runner and strict mode; no runner change.
- [x] Confirm the Java/TypeScript grammars are production dependencies while
      the Rust/Python evaluation grammars remain dev-only.
- [x] Confirm the adapter unit tests and delta scenarios run:
      `cargo test --locked --lib outline::tree_sitter_adapter` and the delta
      feature scope.

## Batch A — Java and TypeScript adapters

Design constraints:

- Add `tree-sitter 0.27.1`, `tree-sitter-java 0.23.5`, and
  `tree-sitter-typescript 0.23.2` as production dependencies.
- Add `Language::Java`, `Language::TypeScript`, `TypeKind::Interface`, and the
  Java/TypeScript constructs and limitations.
- Add `.java`, `.ts`, and `.tsx` discovery with language-specific exclusions;
  `.d.ts`, `.test.ts`, `.spec.ts`, `.test.tsx`, `.spec.tsx`, Java
  `*Test.java`/`*Tests.java`, Java test/build roots, and TypeScript
  test/build/dependency roots are excluded.
- Add `tree_sitter_adapter` with Java and TypeScript analyzers. Java selects
  the Java grammar; TypeScript selects TS/TSX by extension. Extract units,
  methods, fields, calls, spans, visibility, signatures, and unsupported
  constructs. A parse error is a per-file failure with no units.
- Compose Rust/Python adapters unchanged with the new adapters in
  `outline_analyzer`.
- Add the `interface` type kind to classification and the JSON schema version 4.
- Update the CLI request to all four languages and derive supported-language
  text from capabilities.
- No Go/JavaScript/JSX support and no cross-language call resolution.

### Scenario: A Java production class is reported

- [x] Scenario: A Java production class is reported
  - [x] COMMIT: 114afae1 -- Batch: java-typescript-adapters

### Scenario: A TypeScript production class and function are reported

- [x] Scenario: A TypeScript production class and function are reported
  - [x] COMMIT: 114afae1 -- Batch: java-typescript-adapters

### Scenario: A TypeScript interface is reported with its own type kind

- [x] Scenario: A TypeScript interface is reported with its own type kind
  - [x] COMMIT: 114afae1 -- Batch: java-typescript-adapters

### Scenario: A mixed-language report discloses every supported language

- [x] Scenario: A mixed-language report discloses every supported language
  - [x] COMMIT: 114afae1 -- Batch: java-typescript-adapters

### Scenario: A malformed Java file does not hide a valid TypeScript file

- [x] Scenario: A malformed Java file does not hide a valid TypeScript file
  - [x] COMMIT: 114afae1 -- Batch: java-typescript-adapters

### Scenario: Java and TypeScript test and dependency sources are excluded

- [x] Scenario: Java and TypeScript test and dependency sources are excluded
  - [x] COMMIT: 114afae1 -- Batch: java-typescript-adapters

### Scenario: Unsupported-only Go production sources are not reported as absent

- [x] Scenario: Unsupported-only Go production sources are not reported as absent
  - [x] COMMIT: 114afae1 -- Batch: java-typescript-adapters

### Scenario: The JSON report carries the schema and rules versions

- [x] Scenario: The JSON report carries the schema and rules versions
  - [x] COMMIT: 114afae1 -- Batch: java-typescript-adapters

## Batch B — documentation closeout

- [ ] Update `docs/outline.md`, `docs/reference.md`, `README.md`,
      `docs/INDEX.md`, the affected arc42 chapters, and `docs/glossary.md`.
- [ ] Record the 0.9.15 outcome in
      `docs/outline-tree-sitter-evaluation.md`.
- [ ] Move the pending agent-add-on note to `docs/improvements/0.9.16`.
- [ ] Confirm every delta step phrase has a permanent binding.
- [ ] COMMIT: <hash> -- Batch: java-typescript-documentation-closeout
