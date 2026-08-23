---
status: accepted
date: 2026-08-16
decision-makers:
  - givn maintainers
consulted:
  - retrieval identity scenarios
informed:
  - authors and tooling supplying feature paths
---

# Use one project-relative scenario identity

## Qualification

- [x] **Hard to reverse** - persisted identities and self-filter behavior depend on the normalization rule.
- [x] **Shapes quality attributes** - deterministic identity prevents false self-matches and false exclusions.
- [x] **Structural** - the identity is the key connecting permanent entries, delta sources, projected targets, and queries.
- [x] **Must be shared** - every retrieval surface must use the same file/title representation.

## Context and Problem Statement

The same feature can be supplied as a relative or absolute path, with platform-specific separators. Comparing caller-specific strings can make a scenario match itself or hide a valid candidate.

## Decision Drivers

- project-move-friendly persisted data
- relative/absolute path parity
- forward-slash output
- one rule for source and projected-target identities

## Considered Options

- Compare paths as supplied.
- Persist absolute paths.
- Normalize every identity relative to the project root with forward slashes.

## Decision Outcome

Chosen option: **Normalize every permanent, delta-source, projected-target, and file-query path relative to the project root**, rendering forward slashes; combine it with the scenario title. Paths outside the project root retain a normalized supplied representation.

### Consequences

- Equivalent relative and absolute inputs filter identically.
- A future normalization change invalidates identity comparisons and needs regression evidence.
- Identity normalization is separate from semantic model text.

### Confirmation

Retrieval scenarios query added and modified self-identities through both path forms and assert that another eligible same-file or cross-file candidate remains visible.

## Pros and Cons of the Options

### Project-relative forward-slash identity

- Good, because persisted and rendered identities are stable across checkout locations and hosts.
- Bad, because every new identity-producing path must use the shared helper.

### Supplied path strings

- Good, because no normalization code is needed.
- Bad, because equivalent paths compare differently.

### Absolute paths

- Good, because same-host comparisons are direct.
- Bad, because moving the project changes every persisted identity.

## More Information

See [chapter 8.5c](../08-crosscutting-concepts.md#85c-current-change-scenario-exploration).
