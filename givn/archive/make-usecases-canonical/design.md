# Design: make-usecases-canonical

## Overview

Replace the active narrative parser and discovery contract with strict
`usecase.md` and `fragment.md` documents. Migrate Givn's active corpus into
explicit use-case and fragment ownership while leaving archived history
untouched.

## Use-Case Traceability

- Owning use-case ID: `make-usecases-canonical`
- Ideation source: none
- Confirmed Personas: none

| Use-case contract element | Technical decision | Capability / E2E evidence |
|---|---|---|
| Explicit identity and goal level | Strict Markdown parser with `!` and `+` | `canonical-usecase-contract` |
| Capability ownership and interaction inventory | Use-case-aware tree, lint, routing, and coverage | `givn spec tree` scenario |
| Typed Includes / Extends | Relationship target enum and graph validation | Unit tests and lint findings |
| Atomic document replacement | Base SHA-256 validation before archive mutation | Archive unit tests |

## Technology Decisions

- Rust remains the implementation language and Gherkin remains the executable
  specification format.
- `usecase_md.rs` owns the strict Markdown schema. `fragment.md` is parsed by a
  separate schema and cannot be treated as a use case.
- SHA-256 records the base narrative revision for stale replacement detection.
- Existing Cobertura parsing is reused for source line/branch comparison in the
  later migration coverage gate.
- No compatibility reader or output alias is retained.

## Architecture Impact

Affected boundaries are `gherkin`, `overlap`, `spec`, `cli/archive`, `cli/lint`,
`cli/instructions`, and generated guidance assets. The permanent corpus becomes:

```text
givn/specs/<usecase-id>/usecase.md
givn/specs/<usecase-id>/<capability>.feature
givn/specs/fragments/fragment.md
givn/specs/fragments/<capability>.feature
```

## Step Definitions

| Capability | Step definition file |
|---|---|
| canonical-usecase-contract | `tests/steps/spec_discovery_deterministic_steps.rs` |

Test runner command:

```text
cargo test --test features_runner
```

Single-scenario command:

```text
cargo test --test features_runner -- --name "Use-case tree reports explicit use-case ownership"
```

## Strict Mode

The runner uses `GivnWorld::cucumber().fail_on_skipped()`. Undefined and
pending steps therefore fail with a non-zero exit. The not-implemented stub is
`unimplemented!()` in Rust step definitions.

## Local Runnability & Digital Twins

The capability is a CLI-only Givn behavior. No application server, database,
queue, or third-party dependency is required. The real interface is a Givn
subprocess in the feature runner; no digital twin is applicable.

## Interaction Coverage Matrix

| Inventory entry | @e2e scenario title | Real interface | Driving mechanism |
|---|---|---|---|
| inspect the use-case tree | Use-case tree reports explicit use-case ownership | CLI | Cucumber step launches `givn spec tree` as a subprocess and reads stdout |

## Archive Semantics

Validate all use-case replacements before writing. A replacement of an existing
document requires `<!-- givn:base-sha256:<hash> -->`; a mismatch blocks before
mutation. The specs backup includes narratives, fragments, feature files, and
retirement records.

## Verify Command

```text
cargo test --test features_runner
```

E2E smoke tests use the same configured feature runner with the `@e2e` scope.
