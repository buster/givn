---
status: deprecated
date: 2026-06-19
decision-makers:
  - givn maintainers
consulted:
  - initial product naming discussion
informed:
  - contributors and users
---

# Historical record: product name `givn`

> This is retained for historical traceability. It is not an active architecture contract.

## Qualification

- [ ] **Hard to reverse** - a product name can be changed without restructuring the system.
- [ ] **Shapes quality attributes** - the name does not materially set a software quality constraint.
- [ ] **Structural** - it identifies the product but does not define a component boundary or protocol.
- [ ] **Must be shared** - users need the name for invocation, but this is not an architecture rule.

This decision was archived because product naming does not qualify as an active architecture decision under the register's gate.

## Context and Problem Statement

The project needed a short name for the binary, crate, and configuration directory.

## Decision Drivers

- short command name
- thematic connection to Gherkin
- availability in planned registries

## Considered Options

- `givn`.
- `gherk`.
- Other available short names.

## Decision Outcome

The historical choice was **`givn`**, derived from Gherkin's `Given`, with the tagline "from Given to done".

### Consequences

- The binary, crate, and project directory use `givn` in this checkout.
- Renaming remains a product migration concern, not an architecture contract.

### Confirmation

`Cargo.toml`, the CLI name, and the project layout use `givn`.

## Pros and Cons of the Options

### `givn`

- Good, because it is short and related to the workflow vocabulary.
- Bad, because naming availability can change outside the repository.

### Other names

- Good, because another name could be more distinctive.
- Bad, because it would require a product-level rename without architectural benefit.

## More Information

The active architecture register is [../README.md](../README.md).
