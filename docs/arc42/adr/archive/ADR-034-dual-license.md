---
status: deprecated
date: 2026-08-06
decision-makers:
  - givn maintainers
consulted:
  - Rust distribution and contribution requirements
informed:
  - contributors and users
---

# Historical record: dual MIT/Apache-2.0 license

> This is retained for historical traceability. Licensing is legal/project governance, not an active architecture contract.

## Qualification

- [ ] **Hard to reverse** - licensing can be changed only with legal review, but it does not define runtime architecture.
- [ ] **Shapes quality attributes** - it does not materially set a product quality attribute.
- [ ] **Structural** - it does not define a component, dependency direction, or public protocol.
- [ ] **Must be shared** - contributors need the license, but it is legal project policy.

## Context and Problem Statement

The project needed an explicit open-source license for distribution and contribution.

## Decision Drivers

- Rust ecosystem convention
- permissive use
- patent and contribution terms

## Considered Options

- MIT OR Apache-2.0.
- MIT only.
- Apache-2.0 only.

## Decision Outcome

The historical choice was **MIT OR Apache-2.0**. It remains the license metadata in this checkout but is excluded from the active architecture register.

### Consequences

- `Cargo.toml` and project documentation declare the dual license.
- Future licensing changes require legal and contributor review.

### Confirmation

`Cargo.toml` contains `license = "MIT OR Apache-2.0"`.

## Pros and Cons of the Options

### MIT OR Apache-2.0

- Good, because it combines broad permissiveness with an Apache patent grant.
- Bad, because contributors must understand two license texts.

### MIT only

- Good, because the legal surface is shorter.
- Bad, because it omits the patent terms some Rust users expect.

### Apache-2.0 only

- Good, because patent and contribution terms are explicit.
- Bad, because some consumers prefer MIT compatibility.

## More Information

The active register is [../README.md](../README.md).
