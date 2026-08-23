---
status: accepted
date: 2026-06-19
decision-makers:
  - givn maintainers
consulted:
  - configuration design record
informed:
  - contributors and users
---

# Materialize defaults only through eject

## Qualification

- [x] **Hard to reverse** - changing what `init` writes would alter project ownership and upgrade behavior.
- [x] **Shapes quality attributes** - a small project footprint improves maintainability and upgrade safety.
- [x] **Structural** - `eject` defines the boundary between embedded fallback assets and project-owned files.
- [x] **Must be shared** - users and agents need one rule for discovering an editable default.

## Context and Problem Statement

Writing every default into a new project creates noise and makes ownership ambiguous. Users still need a supported way to inspect and customize a default.

## Decision Drivers

- minimal initialized projects
- explicit ownership transfer
- compatibility with the facet cascade

## Considered Options

- Materialize a selected default on demand with `givn eject`.
- Write all defaults during `givn init`.
- Write a fixed subset during init and expose the rest through a second mechanism.

## Decision Outcome

Chosen option: **Materialize selected defaults on demand with `givn eject`**, because the project contains only deliberate overrides and the embedded layer remains the fallback.

### Consequences

- `givn eject artifact:facet`, `givn eject skill:<name>`, and `givn eject manifest` create editable project files.
- An existing override is protected unless `--force` is used.
- Users must invoke eject before editing an embedded facet.

### Confirmation

Eject scenarios assert the output path, refusal to overwrite without `--force`, and subsequent cascade resolution from the ejected file.

## Pros and Cons of the Options

### Eject on demand

- Good, because ownership is explicit and the project stays small.
- Bad, because the discovery command must be known.

### Write all defaults at init

- Good, because every file is immediately visible.
- Bad, because it creates noise and freezes defaults into every project.

### Hybrid initialization

- Good, because it can optimize for a common subset.
- Bad, because ownership rules become inconsistent across facets.

## More Information

See [chapter 8.1](../08-crosscutting-concepts.md#81-the-cascade-engine-override-files).
