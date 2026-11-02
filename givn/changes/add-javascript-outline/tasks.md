# Tasks: add-javascript-outline

Owning use case: `assess-code-structure`. Personas: `developer--primary-user`,
`agent--technical-role`, `givn-maintainer--owner`.

Release note: Outline analysis now supports production JavaScript, including ES module and CommonJS files, without adding a runtime dependency.

## Setup

- [x] Confirm the existing runner and strict mode; no runner change.
      `tests/features_runner.rs` calls `.fail_on_skipped()`, and
      `givn/commands.yaml` keeps `./run-tests.sh --non-e2e` and
      `./run-tests.sh --e2e`.
- [x] Prove an undefined step fails. A temporary feature with
      `Given an undefined strict-mode probe step` run through
      `GIVN_FEATURES=<tmp> cargo nextest run --locked --test features_runner`
      exited 100 with `Step doesn't match any function`.
- [x] Confirm the permanent baseline is green: `./run-tests.sh --non-e2e`
      reports 893 passed, 104 skipped.

## Batch A — JavaScript adapters

Design constraints:

- Add `tree-sitter-javascript 0.25.0` and `Language::JavaScript` with label
  `javascript`.
- Recognize `.js`, `.mjs`, and `.cjs`; exclude JavaScript test/spec/build/
  dependency roots and `*.test.js`/`*.spec.js` (and the `.mjs`/`.cjs`
  equivalents); remove `js` from unsupported extensions and keep `jsx` and
  `go`.
- Add `tree_sitter_adapter::JavaScriptAnalyzer` behind the existing
  `SourceAnalyzer` port. Select the JavaScript grammar. Map class declarations,
  `method_definition`, `field_definition`, function and generator function
  declarations, arrow-function bindings, and calls. Reuse the existing call
  shapes and resolver; never resolve across languages.
- Trim `.js`, `.mjs`, and `.cjs` from the module path. A parse error is a
  per-file failure with no units.
- Compose the JavaScript adapter with the existing adapters in
  `outline_analyzer`; add JavaScript to capabilities and the CLI request.
- Advance `SCHEMA_VERSION` to 5. The supported-language text is derived from
  capabilities.
- Step definitions live in `tests/steps/outline_report_steps.rs`; no delta
  step copy.
- Unnamed class and function expressions are recorded as unsupported
  constructs, not units.

### Scenario: The JSON report carries the schema and rules versions

- [x] Scenario: The JSON report carries the schema and rules versions
  - [x] COMMIT: eecf9262 -- Batch: javascript-adapters
  - Design constraints: `SCHEMA_VERSION` is 5; `rules_version` remains 1;
    schema 5 is the only report shape change.
  - Then step: update the schema assertion from 4 to 5.

### Scenario: A mixed-language report discloses every supported language

- [x] Scenario: A mixed-language report discloses every supported language
  - [x] COMMIT: eecf9262 -- Batch: javascript-adapters
  - Design constraints: capabilities list Rust, Python, Java, TypeScript, and
    JavaScript; the mixed fixture adds a JavaScript class `Widget`; the
    supported-language assertion includes `javascript`.
  - Then steps: all supported languages and the `javascript` unit.

### Scenario: A JavaScript production class and function are reported

- [x] Scenario: A JavaScript production class and function are reported
  - [x] COMMIT: eecf9262 -- Batch: javascript-adapters
  - Design constraints: class declarations map to `TypeKind::Class`; function
    declarations map to module outline methods; the unit language is
    `javascript`; exported declarations are walked through their wrapper.
  - Then steps: unit `Widget`, type kind `class`, function `makeWidget`.

### Scenario: A JavaScript class field is reported

- [x] Scenario: A JavaScript class field is reported
  - [x] COMMIT: eecf9262 -- Batch: javascript-adapters
  - Design constraints: JavaScript class fields use the
    `field_definition` node; a private `#name` field is private visibility;
    fields are neutral `Field` facts on the class unit.
  - Then step: 1 field on `Widget`.

### Scenario Outline: JavaScript production modules are reported for every supported extension

- [x] Scenario Outline: JavaScript production modules are reported for every supported extension
  - [x] COMMIT: eecf9262 -- Batch: javascript-adapters
  - Design constraints: `.js`, `.mjs`, and `.cjs` select the JavaScript
    grammar; the module path trims the extension; a top-level named function
    appears in the module outline.
  - Examples: `.js`, `.mjs`, `.cjs`; Then step asserts `drain` with language
    `javascript`.

### Scenario: A JavaScript arrow-function binding is reported

- [x] Scenario: A JavaScript arrow-function binding is reported
  - [x] COMMIT: eecf9262 -- Batch: javascript-adapters
  - Design constraints: a `lexical_declaration` whose declarator value is an
    `arrow_function` becomes a module outline function with the binding name.
  - Then step: function `clone` with language `javascript`.

### Scenario: A malformed JavaScript file does not hide a valid TypeScript file

- [x] Scenario: A malformed JavaScript file does not hide a valid TypeScript file
  - [x] COMMIT: eecf9262 -- Batch: javascript-adapters
  - Design constraints: `has_error()` produces a `ParseFailed` diagnostic for
    `src/Broken.js`; the valid TypeScript unit still reports.
  - Then steps: parse failure path and TypeScript unit.

### Scenario: JavaScript test and dependency sources are excluded

- [x] Scenario: JavaScript test and dependency sources are excluded
  - [x] COMMIT: eecf9262 -- Batch: javascript-adapters
  - Design constraints: `*.test.js`/`*.spec.js` and their `.mjs`/`.cjs`
    forms are excluded; `node_modules`, `dist`, `build`, `.next`, `coverage`,
    `out`, and `__tests__` are excluded. The fixture uses `*.test.js` and
    `node_modules`.
  - Then steps: stdout does not contain the ignored unit names.

### Scenario: JSX-only production sources remain unsupported

- [x] Scenario: JSX-only production sources remain unsupported
  - [x] COMMIT: eecf9262 -- Batch: javascript-adapters
  - Design constraints: `jsx` stays in `UNSUPPORTED_SOURCE_EXTENSIONS`;
    a JSX-only scope exits 1 and names the scope, the supported languages, and
    the unsupported paths.
  - Then steps: exit code, scope, supported languages, no false absence claim.

## Batch B — documentation closeout

- [ ] Update `docs/outline.md`, `docs/reference.md`, `README.md`, and
      `docs/INDEX.md` for JavaScript and schema 5.
- [ ] Confirm the affected `docs/arc42/` chapters and `docs/glossary.md` match
      the implemented adapter and schema.
- [ ] Confirm every delta step phrase has a permanent binding in
      `tests/steps/outline_report_steps.rs`.
- [ ] COMMIT: pending -- Batch: javascript-documentation-closeout

## Verification

- [ ] `./run-tests.sh --non-e2e` exits 0.
- [ ] `./run-tests.sh --e2e` exits 0.
- [ ] `./measure-coverage.sh` and `./merge-coverages.sh` complete.
- [ ] `givn check review --change add-javascript-outline` exits 0.
