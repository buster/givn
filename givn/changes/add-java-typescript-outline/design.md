# Design: add-java-typescript-outline

## Overview

The change keeps the Rust `syn` and Python `ast` adapters and adds production Tree-sitter adapters for Java and TypeScript/TSX behind the existing `SourceAnalyzer` port.

Three boundaries define the work:

1. **Language boundary.** Java and TypeScript are parsed in-process with bundled grammars. `.tsx` selects the TSX grammar. No interpreter, Node, Tree-sitter CLI, or runtime download is required.
2. **Neutral-model boundary.** Java/TypeScript facts are converted into the existing `Analysis` model. A new `interface` type kind covers Java and TypeScript interfaces. The JSON schema advances to 4.
3. **Discovery and reporting boundary.** The scanner recognizes `.java`, `.ts`, and `.tsx`, applies language-specific test/build/dependency exclusions, and reports per-file parse failures. The report remains advisory and deterministic.

## Visual Design Contract

- Interface: terminal
- Viewports: not applicable
- Design system: not applicable
- Persona: developer--primary-user, agent--technical-role, givn-maintainer--owner

The visible surface is the outline human/JSON report. The scenarios drive the real CLI. No browser screen is rendered.

## Use-Case Traceability

Owning use case: `assess-code-structure`.

| Use-case contract element | Technical decision | Capability / E2E evidence |
|---|---|---|
| The report covers Java and TypeScript production sources | New adapters and scan extensions | `outline-report`, Java/TypeScript scenarios |
| The report remains advisory | No gate, verdict, or review change | Existing outline scenarios |
| Calls are evidence-labeled, not compiler-resolved | Tree-sitter supplies syntax; shared resolver keeps uncertainty | `outline-report`, mixed-language scenario |
| Unsupported languages remain disclosed | Go/JavaScript/JSX remain unsupported; message names all supported languages | `outline-report`, Go-only scenario |
| Malformed files do not abort the report | Per-file parse failure diagnostic | `outline-report`, malformed Java scenario |
| Schema changes are explicit | `interface` type kind; schema version 4 | `outline-report`, schema scenario |

## Technology Decisions

| Technology | Version | Resolution |
|---|---|---|
| Rust | existing workspace toolchain | `Cargo.toml` / `Cargo.lock` |
| tree-sitter | 0.27.1 | Production dependency, resolved by `cargo add` |
| tree-sitter-java | 0.23.5 | Production dependency, resolved by `cargo add` |
| tree-sitter-typescript | 0.23.2 | Production dependency, resolved by `cargo add` |
| tree-sitter-rust / tree-sitter-python | 0.24.2 / 0.25.0 | Dev-only evaluation dependencies, kept from 0.9.14 |

The Java/TypeScript grammars are compiled into the binary. The build requires a C toolchain; the report requires no interpreter or download.

## Decisions

- Decides D1: add Java and TypeScript — realized in `tree_sitter_adapter` and the scan extensions.
- Decides D2: retain Rust/Python — `outline_analyzer` composes the existing adapters plus the new ones.
- Decides D3: add `TypeKind::Interface`; schema version 4 — realized in `model.rs`, `report.rs`, and the schema scenario.
- Decides D4: malformed files are file-level failures — realized in the adapters and the malformed-Java scenario.

## Architecture Impact

### Contracts and Components

| Contract | Consumer use case | Consumers | Implementing component today | Split trigger |
|---|---|---|---|---|
| Outline report | assess-code-structure | `givn outline report`, review guidance | `src/outline` pipeline | A new report format or non-structural analysis appears |
| Java source analysis | assess-code-structure | Outline report | `tree_sitter_adapter::JavaAnalyzer` | Java-specific semantic resolution or another grammar variant is needed |
| TypeScript source analysis | assess-code-structure | Outline report | `tree_sitter_adapter::TypeScriptAnalyzer` | JavaScript/JSX or type-aware resolution is added |

### Module changes

- `Cargo.toml` / `Cargo.lock`: production Tree-sitter Java/TypeScript dependencies; Rust/Python evaluation grammars stay dev-only.
- `src/outline/model.rs`: `Language::Java`, `Language::TypeScript`, `TypeKind::Interface`, new constructs/limitations.
- `src/outline/scan.rs`: `.java`, `.ts`, `.tsx` discovery; language-specific exclusions; Java/TS comment classification.
- `src/outline/tree_sitter_adapter/mod.rs`: Java and TypeScript analyzers, member/call extraction, parse-failure handling.
- `src/outline/analyzer.rs`: compose the existing two adapters with the new two.
- `src/outline/classify.rs`: interface type kind for over-limit interface contracts.
- `src/outline/report.rs`: schema version 4.
- `src/cli/outline.rs`: request all four languages; supported-language text from capabilities.

No lifecycle, persistent state, or command surface changes.

## Data Model Changes

The JSON report schema advances from 3 to 4. It adds `java` and `typescript` language values and the `interface` type kind. Existing Rust/Python report shapes are otherwise unchanged.

## Step Definitions

| Capability | Step definition file | Permanent merge target |
|---|---|---|
| `outline-report` | `tests/steps/outline_report_steps.rs` | same file |

Test runner command:

```text
./run-tests.sh --non-e2e
```

Strict mode is unchanged.

## Interaction Coverage Matrix

| Inventory entry | Design matrix row | `@e2e` scenario | Driving mechanism |
|---|---|---|---|
| `outline-report` — run the advisory report | Existing permanent row | Existing outline E2E | Real `givn outline report` subprocess |

The new scenarios are regular variants of the existing report action.

## External Boundary Cases

| Boundary | Adversarial case | Handling |
|---|---|---|
| Tree-sitter parse | malformed Java/TypeScript, TSX dialect | file-level failure with diagnostic; other files continue; TSX grammar selected by extension |
| Source discovery | test/build/dependency roots, `.d.ts`, `.test.ts`, `.spec.ts` | excluded before parsing |
| Calls | same name in another language, unknown receiver | no cross-language resolution; unproven receiver remains name-only |
| Unsupported language | Go/JavaScript/JSX only | command exits 1 naming the scope and supported languages |

## Local Runnability

No server or external service. The report runs offline; grammars are bundled.

## Coverage Process Boundaries

| Process | Started by | Instrumented artifact | Profile output | Merge step | Non-zero production probe |
|---|---|---|---|---|---|
| Feature runner | `./run-tests.sh --non-e2e` | `features_runner` and `givn` | `target/llvm-cov-target` | `merge-coverages.sh` | Java/TypeScript adapter paths |
| Ready fixture | `run-tests.sh` before the suite | `ready_fixture` | same profile set, discarded | n/a | fixture startup only |

Measured population: production source under `src/`.

## Out-of-Scope Technical Decisions

- No Rust/Python replacement.
- No Go/JavaScript/JSX.
- No cross-language resolution.
- No partial outline.
- No runtime grammar download or plugin registry.
