---
status: accepted
date: 2026-06-19
decision-makers:
  - givn maintainers
consulted:
  - initial workflow design record
informed:
  - contributors and users
---

# Implement the CLI in Rust

## Qualification

- [x] **Hard to reverse** - changing the implementation language would replace the binary, dependency, and test integration surface.
- [x] **Shapes quality attributes** - Rust supports a small native binary and type-checked merge, state, and parsing logic.
- [x] **Structural** - the language determines the crate boundary and the implementation of every production module.
- [x] **Must be shared** - contributors need one language and toolchain expectation for production changes.

## Context and Problem Statement

givn needs a native CLI that can package defaults, parse Gherkin, merge deltas, and run on projects without a language runtime. The implementation language must support the binary and distribution constraints without reintroducing npm.

## Decision Drivers

- native single-binary distribution
- type safety for filesystem and merge state
- mature CLI and serialization libraries
- contributor familiarity and release tooling

## Considered Options

- Rust.
- Go.
- Zig or a bundled-runtime language.

## Decision Outcome

Chosen option: **Rust**, because it combines native packaging, strong types for the merge/state core, and the required CLI and serialization ecosystem.

### Consequences

- Production code is one Rust crate with feature-gated retrieval dependencies.
- Cargo and a Rust toolchain are required to build from source.
- Cross-target linking and optional ONNX dependencies remain deployment risks.

### Confirmation

`cargo check` and the repository's Cucumber acceptance binary compile from the checkout; feature combinations are exercised by the configured test scripts.

## Pros and Cons of the Options

### Rust

- Good, because it produces a native binary and has the required ecosystem.
- Bad, because compile times and native optional dependencies increase build complexity.

### Go

- Good, because it has straightforward distribution and onboarding.
- Bad, because the project would replace the chosen type and ecosystem constraints without a demonstrated benefit.

### Zig or a bundled-runtime language

- Good, because some deployment paths can be simple.
- Bad, because the required parser, CLI, and contributor ecosystem would be less established for this project.

## More Information

The crate and feature boundary are described in [chapter 5](../05-building-block-view.md) and [chapter 7](../07-deployment-view.md).
