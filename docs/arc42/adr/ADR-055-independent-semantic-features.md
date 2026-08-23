---
status: accepted
date: 2026-08-18
decision-makers:
  - givn maintainers
consulted:
  - Cargo feature matrix and retrieval implementation
informed:
  - contributors, agents, and CI users
---

# Keep retrieval, E5 gate, BGE, and NLI capabilities independent

## Qualification

- [x] **Hard to reverse** - feature combinations determine binary dependencies and review behavior.
- [x] **Shapes quality attributes** - optional native model paths must not silently change the default gate.
- [x] **Structural** - Cargo features and environment selectors define separate model capabilities.
- [x] **Must be shared** - build and runtime combinations must be interpreted consistently.

## Context and Problem Statement

Earlier experiments coupled reranking, the E5 gate, and an NLI selector. That coupling made it impossible to compare E5-only, E5+BGE, E5+NLI, and combined behavior without hidden model participation.

## Decision Drivers

- feature-free default build
- explicit E5 hard-gate selection
- independent BGE and NLI evidence
- testable dispatch matrix

## Considered Options

- Preserve historical feature implication and shared selector.
- Remove all experimental second-stage paths.
- Separate Cargo features and selectors while retaining the evidence paths.

## Decision Outcome

Chosen option: **Use independent `spec-retrieval`, `spec-similarity-gate`, `cross-encoder-reranker`, and `nli-evidence` capabilities**. BGE is selected through `GIVN_SPEC_BGE`; NLI is selected through `GIVN_SPEC_RERANKER=nli` or `nli-stub`; neither second-stage feature implies the E5 hard gate or the other model.

### Consequences

- The default build has no retrieval model dependency.
- Test scripts must select the feature and environment matrix deliberately.
- Combined output is produced only when both layers participate.

### Confirmation

Cargo feature checks and BGE/NLI isolation scenarios assert that enabling one model does not emit or enable the other model or the strict E5 gate.

## Pros and Cons of the Options

### Independent capabilities

- Good, because every comparison mode is explicit.
- Bad, because cfg and selector paths require a larger test matrix.

### Historical coupling

- Good, because migration is smaller.
- Bad, because model participation and gate behavior become ambiguous.

### Remove second-stage paths

- Good, because the binary is simpler.
- Bad, because measured and planned evidence is discarded.

## More Information

See [ADR-054](ADR-054-versioned-evidence-output.md), [ADR-058](ADR-058-nli-evidence.md), and [chapter 2](../02-architecture-constraints.md).
