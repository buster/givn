# Design: add-javascript-outline

## Overview

The change adds JavaScript as a fifth production language behind the existing
neutral `SourceAnalyzer` port. The Rust, Python, Java, and TypeScript adapters
keep their current responsibilities; JavaScript gets a separate adapter because
its grammar and dialect rules are not the TypeScript adapter's contract.

Three boundaries define the work:

1. **Language boundary.** JavaScript is parsed in-process with a bundled
   Tree-sitter JavaScript grammar. `.js`, `.mjs`, and `.cjs` are in scope; JSX
   and Go remain unsupported. No interpreter, Node.js, Tree-sitter CLI, or
   runtime download is required.
2. **Neutral-model boundary.** JavaScript facts are converted into the existing
   `Analysis` model: a file is a module unit, a class is a class type unit, and
   functions, methods, fields, and calls use the existing neutral facts. The
   language value is `javascript`. The JSON schema advances to 5.
3. **Discovery and reporting boundary.** The scanner recognizes `.js`, `.mjs`,
   and `.cjs`, applies JavaScript test/build/dependency exclusions, and reports
   per-file parse failures. The report stays advisory and deterministic.

## Visual Design Contract

- Interface: terminal
- Viewports: not applicable
- Design system: not applicable
- Persona: developer--primary-user, agent--technical-role, givn-maintainer--owner

The visible surface is the outline human and JSON report. The scenarios drive
the real CLI. No browser screen is rendered.

### Persona disposition

| Persona | Relevance | Evidence |
|---|---|---|
| developer--primary-user | Runs the report and reads the outline during REFACTOR | CLI and JSON scenarios |
| agent--technical-role | Consumes the outline and JSON evidence during implementation and review | JSON scenarios and capability disclosure |
| givn-maintainer--owner | Maintains the language adapter and schema contract | Schema, exclusions, and mixed-language scenarios |

## Use-Case Traceability

Owning use case: `assess-code-structure`.

| Use-case contract element | Technical decision | Capability / E2E evidence |
|---|---|---|
| The report covers production sources in the supported set | JavaScript joins the supported language set | `outline-report`, JavaScript class/function and module scenarios |
| The report is advisory and never gates | No gate, verdict, or review change | Existing outline scenarios |
| Calls are evidence-labeled, not compiler-resolved | JavaScript uses the shared call shapes and resolver; no cross-language resolution | `outline-report`, mixed-language scenario |
| Unsupported languages are disclosed | JSX and Go remain outside the analyzer; the supported-language text names JavaScript | `outline-report`, JSX-only and Go-only scenarios |
| Test and tooling sources are excluded | JavaScript test/spec and dependency roots are filtered before parsing | `outline-report`, JavaScript exclusions scenario |
| Malformed files do not abort the report | Per-file parse failure diagnostic | `outline-report`, malformed JavaScript scenario |
| Schema changes are explicit | `javascript` language value; schema version 5 | `outline-report`, schema scenario |
| Class fields and methods are outline evidence | `field_definition` and `method_definition` map to neutral fields and methods | `outline-report`, JavaScript class field scenario and class scenario |

## Technology Decisions

| Technology | Version | Resolution |
|---|---|---|
| Rust | existing workspace toolchain | `Cargo.toml` / `Cargo.lock` |
| tree-sitter | 0.27.1 | Existing production dependency |
| tree-sitter-javascript | 0.25.0 | Checked with `cargo info tree-sitter-javascript` on 2026-11-02; the crate resolves `tree-sitter-language 0.1` against the lockfile's 0.1.9 |
| tree-sitter-java | 0.23.5 | Existing production dependency, unchanged |
| tree-sitter-typescript | 0.23.2 | Existing production dependency, unchanged |

The JavaScript grammar is compiled into the binary. The build already requires a
C toolchain for the existing grammars; the report itself needs no interpreter or
download.

## Decisions

- Decides D1: `.js`, `.mjs`, and `.cjs` are production JavaScript; JSX remains out of scope — realized in `scan.rs` discovery and the module-extension scenario.
- Decides D2: JavaScript is its own `Language::JavaScript` value — realized in `model.rs`, the adapter, the CLI scope, and the language assertions.
- Decides D3: the JSON schema advances from 4 to 5 — realized in `report.rs` and the schema scenario.
- Decides D4: a malformed JavaScript file is a per-file failure; other files continue — realized in the adapter's parse-failure path and the malformed-JavaScript scenario.

## Architecture Impact

### Contracts and Components

| Contract | Consumer use case | Consumers | Implementing component today | Split trigger |
|---|---|---|---|---|
| Outline report | assess-code-structure | `givn outline report`, review guidance | `src/outline` pipeline | A new report format or non-structural analysis appears |
| JavaScript source analysis | assess-code-structure | Outline report | `tree_sitter_adapter::JavaScriptAnalyzer` (new) | JavaScript dialect selection, JSX, or type-aware resolution is added |
| TypeScript source analysis | assess-code-structure | Outline report | `tree_sitter_adapter::TypeScriptAnalyzer` | TypeScript dialect selection or type-aware resolution is added |
| Java source analysis | assess-code-structure | Outline report | `tree_sitter_adapter::JavaAnalyzer` | Java-specific semantic resolution or another grammar variant is needed |

The JavaScript adapter is a separate component behind the same `SourceAnalyzer`
port. It shares syntax-walking helpers with the TypeScript adapter where the
grammar node kinds are identical, but it does not share the TypeScript
`Language` value or capability disclosure.

### Module changes

- `Cargo.toml` / `Cargo.lock`: add `tree-sitter-javascript = "0.25.0"`.
- `src/outline/model.rs`: add `Language::JavaScript` with label `javascript`.
- `src/outline/scan.rs`: recognize `.js`, `.mjs`, and `.cjs`; exclude JavaScript
  test/spec/build/dependency roots; remove `js` from the unsupported extensions;
  keep `jsx` and `go` unsupported. Rename the shared ECMAScript exclusion list
  so TypeScript and JavaScript use the same entries.
- `src/outline/tree_sitter_adapter/mod.rs`: add `JavaScriptAnalyzer`; select the
  JavaScript grammar; map `class_declaration`, `method_definition`,
  `field_definition`, `function_declaration`, generator function declarations,
  and arrow-function bindings; reuse the existing call-shape extraction; report
  `javascript` units and parse failures. Unnamed class and function expressions
  are recorded as unsupported constructs instead of invented units.
- `src/outline/analyzer.rs`: include the JavaScript adapter in capabilities and
  merge its files, units, and diagnostics.
- `src/outline/tree_sitter_eval/mod.rs`: satisfy the exhaustive `Language` match
  for the test-only evaluation path. JavaScript is not part of the evaluation
  corpus.
- `src/outline/report.rs`: advance `SCHEMA_VERSION` to 5.
- `src/cli/outline.rs`: request `Language::JavaScript` in the analysis scope.
- `tests/steps/outline_report_steps.rs`: fixtures and assertions for JavaScript
  classes, class fields, functions, modules, arrow-function bindings,
  exclusions, malformed files, schema 5, and all-supported-language disclosure.
- `docs/outline.md`, `docs/reference.md`, `README.md`, `docs/glossary.md`, and
  the affected `docs/arc42/` chapters: add JavaScript to the documented scope
  and advance the schema reference to 5.

No lifecycle, persistent state, or command-surface changes.

## Data Model Changes

The JSON report schema advances from 4 to 5. It adds the `javascript` language
value and lists it in `capabilities.languages` and the requested scope. No new
construct, type kind, resolution label, or metric is introduced. A JavaScript
file module uses the existing module unit; a class uses the existing class type
unit; fields and methods use the existing neutral facts. Unnamed class and
function expressions are recorded as unsupported constructs rather than
claimed as units.

The human report's supported-language text is derived from analyzer
capabilities, so it adds JavaScript without a separate rendering change.

## Step Definitions

| Capability | Step definition file | Permanent merge target |
|---|---|---|
| `outline-report` | `tests/steps/outline_report_steps.rs` | same file |

The JavaScript scenarios extend the existing outline step module. No delta step
copy is introduced; the permanent file is the definition home. The integrity
hook resolves the delta phrases against this file.

Test runner commands:

```text
./run-tests.sh --non-e2e
./run-tests.sh --e2e
```

Single-scenario command:

```text
GIVN_FEATURES=givn/changes/add-javascript-outline/specs \
  cargo nextest run --locked --test features_runner -- \
  'assess-code-structure/outline-report.feature::A JavaScript production class and function are reported'
```

The runner treats the unit as `<feature path>::<scenario title>` and runs it
with an exact name predicate.

## Strict Mode

Strict mode is unchanged. `tests/features_runner.rs` calls
`.fail_on_skipped()` on the Cucumber builder, so an undefined or pending step
fails the run rather than passing or skipping. The delta scenarios carry `@wip`
until their bindings exist.

## Interaction Coverage Matrix

| Inventory entry | `@e2e` scenario title | Real interface | Driving mechanism |
|---|---|---|---|
| `outline-report` — run a outline report | A developer runs a outline report and sees the classified units | CLI | Real `givn outline report` subprocess |

The JavaScript scenarios are regular variants of the existing report action.
No new user action is introduced, so the permanent `@e2e` scenario remains the
single interaction entry.

## External Boundary Cases

| Boundary | Adversarial case | Handling |
|---|---|---|
| Tree-sitter parse | Malformed JavaScript, unnamed class and function expressions | Per-file parse failure with a diagnostic; other files continue. Unnamed class and function expressions are recorded as unsupported constructs, not units. |
| Module extension | `.js`, `.mjs`, `.cjs`, `.jsx` | The first three select the JavaScript grammar; `.jsx` remains an unsupported production extension. |
| Source discovery | `*.test.js`, `*.spec.js`, `node_modules`, `dist`, `build`, `.next`, `coverage`, `out`, `__tests__` | Excluded before parsing. |
| Calls | Dynamic property access, `require`, unknown receiver, same name in another language | The shared resolver keeps unproven receivers name-only and never resolves across languages. |
| Unsupported language | Go-only or JSX-only scope | Command exits 1, names the scope and all supported languages, and does not claim no production sources exist. |
| Parse failure visibility | One malformed JavaScript file beside a valid TypeScript file | The failure is named for the JavaScript path; the TypeScript unit still reports. |

## Local Runnability

No server or external service. The report runs offline; the JavaScript grammar is
compiled into the binary. The local run command is the ordinary test command:

```text
./run-tests.sh --non-e2e
```

## Coverage Process Boundaries

| Process | Started by | Instrumented artifact | Profile output | Merge step | Non-zero production probe |
|---|---|---|---|---|---|
| Feature runner | `./run-tests.sh --non-e2e` | `features_runner` and `givn` | `target/llvm-cov-target` | `merge-coverages.sh` | JavaScript adapter discovery and class/function extraction |
| Ready fixture | `run-tests.sh` before the suite | `ready_fixture` | same profile set, discarded | n/a | fixture startup only |

Measured population: production source under `src/`.

## Out-of-Scope Technical Decisions

- No JavaScript or TypeScript import/export graph resolution.
- No type-aware JavaScript resolution.
- No JSX grammar or `.jsx` discovery.
- No CommonJS-specific export modeling.
- No cross-language call resolution.
- No partial outline for malformed files.
- No runtime grammar download, user-supplied parser, or plugin registry.
- No replacement of the Rust, Python, Java, or TypeScript adapters.
