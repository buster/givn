# Design: evaluate-tree-sitter-outline

## Overview

The change adds a test-only Tree-sitter evaluation under `src/outline/tree_sitter_eval/`. It parses representative Rust and Python sources, extracts the fact categories the current outline needs, compares them with `outline_analyzer()`, and records the decision.

The production outline command and adapters are untouched. Tree-sitter and its grammars are dev dependencies, so the release binary does not link them.

## Visual Design Contract

- Interface: terminal
- Viewports: not applicable
- Design system: not applicable
- Persona: developer--primary-user, agent--technical-role, givn-maintainer--owner

The visible surface is the unchanged outline JSON report. The regression scenario drives the real CLI. No browser screen is rendered.

## Use-Case Traceability

Owning use case: `assess-code-structure`.

| Use-case contract element | Technical decision | Capability / E2E evidence |
|---|---|---|
| The outline report remains advisory and deterministic | No production path changes; the existing E2E remains | `outline-report`, existing E2E |
| Supported languages remain Rust and Python | The evaluation is not wired into the CLI | `outline-report`, `The Tree-sitter evaluation leaves the current outline engine in place` |
| The evaluation compares the facts the outline needs | Test-only extractor and comparison categories | Unit tests in `tree_sitter_eval` |
| No replacement without parity evidence | Missing-required categories produce `RETAIN` | Unit test and `docs/outline-tree-sitter-evaluation.md` |
| Malformed input is not treated as clean | Tree-sitter error nodes recorded | Unit test `malformed_tree_sitter_input_is_reported_as_an_error` |

## Technology Decisions

| Technology | Version | Resolution |
|---|---|---|
| Rust | existing workspace toolchain | `Cargo.toml` / `Cargo.lock` |
| tree-sitter | 0.27.1 | Dev dependency, resolved by `cargo add` |
| tree-sitter-rust | 0.24.2 | Dev dependency, resolved by `cargo add` |
| tree-sitter-python | 0.25.0 | Dev dependency, resolved by `cargo add` |

The grammars' C sources are built for tests; the release binary does not include them.

## Decisions

- Decides D1: retain the current adapters for 0.9.14 — realized in `docs/outline-tree-sitter-evaluation.md` and the evaluation tests.
- Decides D2: no additional language — realized by the unchanged production scope and the proposal's out-of-scope section.
- Decides D3: file-level parse failure remains; no partial outline — realized in the malformed-input test and the decision record.

## Architecture Impact

### Contracts and Components

| Contract | Consumer use case | Consumers | Implementing component today | Split trigger |
|---|---|---|---|---|
| Outline report | assess-code-structure | `givn outline report`, review guidance | Existing `src/outline` pipeline | A production engine change or new public language appears |
| Outline parser evaluation | assess-code-structure (test evidence) | Rust unit tests, reviewers | Test-only `src/outline/tree_sitter_eval/` | The evaluation becomes a production adapter or needs a public command |

### Module changes

- `Cargo.toml` / `Cargo.lock`: Tree-sitter dev dependencies.
- `src/outline/mod.rs`: `#[cfg(test)] mod tree_sitter_eval;`.
- `src/outline/tree_sitter_eval/mod.rs`: fact extraction, comparison, decision assertion.
- `docs/outline-tree-sitter-evaluation.md`: comparison matrix, observed gaps, retain decision.
- `givn/changes/evaluate-tree-sitter-outline/specs/assess-code-structure/outline-report.feature`: default-engine regression.

No production module, public command, or schema changes.

## Data Model Changes

None. The evaluation uses a test-only `Facts` type; the neutral production model is unchanged.

## Step Definitions

| Capability | Step definition file | Permanent merge target |
|---|---|---|
| `outline-report` | `tests/steps/outline_report_steps.rs` | same file |

Test runner command:

```text
./run-tests.sh --non-e2e
```

Evaluation unit test:

```text
cargo test --locked --lib tree_sitter_eval
```

Strict mode is unchanged.

## Interaction Coverage Matrix

| Inventory entry | Design matrix row | `@e2e` scenario | Driving mechanism |
|---|---|---|---|
| `outline-report` — run the advisory report | Existing permanent row | Existing outline E2E | Real `givn outline report` subprocess |

The new scenario is a regular variant of the existing action and does not add an E2E.

## External Boundary Cases

| Boundary | Adversarial case | Handling |
|---|---|---|
| Tree-sitter parse | malformed source with `ERROR`/missing nodes | recorded as an error; no partial facts |
| Comparison | missing current fact | classified `missing_required`, decision `RETAIN` |
| Dependency | grammar C build unavailable | test build fails clearly; production binary unaffected |

## Local Runnability

No server or external service. The unit test uses temp fixtures.

## Coverage Process Boundaries

| Process | Started by | Instrumented artifact | Profile output | Merge step | Non-zero production probe |
|---|---|---|---|---|---|
| Feature runner | `./run-tests.sh --non-e2e` | `features_runner` and `givn` | `target/llvm-cov-target` | `merge-coverages.sh` | unchanged outline CLI path |
| Evaluation unit test | `cargo test --lib` | `givn` lib test binary | same profile set | n/a | test-only comparison path |

Measured population: production source under `src/`. Test-only evaluation code is outside the production coverage claim.

## Out-of-Scope Technical Decisions

- No production Tree-sitter adapter, CLI flag, or engine switch.
- No new language or grammar beyond the Rust/Python evaluation pair.
- No partial outline or error-recovery contract.
