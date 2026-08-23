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

# Resolve project facets through a CSS-style cascade

## Qualification

ADR-QUALIFICATION: PASS

### Qualification verdict

qualification: QUALIFIED
alternatives: PASS
architectural_impact: PASS
durable_consequence: PASS
lower_level_artifact: PASS
existing_adr_check: PASS
must_be_shared: SUPPORTING
routing: NEW_ADR
canonical_artifact: null
target_adr: null
replacement_adr: null

- [x] **Hard to reverse** - changing precedence would alter every project override and upgrade result.
- [x] **Shapes quality attributes** - project-owned overrides preserve upgrade safety and maintainability.
- [x] **Structural** - it defines the dependency direction from embedded defaults to project-owned facets.
- [x] **Must be shared** - contributors and agents must know which file is authoritative for a template or instruction.

## Context and Problem Statement

givn ships default templates, instructions, and skill assets, while projects need selective customization. A project must be able to replace one facet without copying and owning the entire embedded asset set.

## Decision Drivers

- project files must win over binary defaults
- upgrades must not overwrite project-owned files
- overrides must be independently addressable
- zero-override projects must work

## Considered Options

- Embedded defaults with project overrides resolved per facet.
- One fully copied project bundle.
- A monolithic configuration file containing all asset content.

## Decision Outcome

Chosen option: **Embedded defaults with project-overrides-first facet resolution**. The resolver checks `givn/artifacts/<id>/` or the corresponding project asset path before the embedded fallback.

### Consequences

- `givn eject` is the discovery mechanism for making a facet project-owned.
- The binary can improve its fallback assets without rewriting ejected files.
- An override can become stale and must be reviewed by the project owner.

### Confirmation

The cascade tests and CLI interactions assert that an ejected template or instruction wins while unrelated facets continue to resolve from embedded assets.

## Pros and Cons of the Options

### Per-facet cascade

- Good, because customization is minimal and upgrade-safe.
- Bad, because users must understand which facet they own.

### Fully copied bundle

- Good, because every asset is visible in the project.
- Bad, because upgrades create noisy diffs and stale copies.

### Monolithic content configuration

- Good, because there is one file to inspect.
- Bad, because independent template and instruction changes become coupled.

## More Information

The resolver is described in [chapter 8.1](../08-crosscutting-concepts.md#81-the-cascade-engine-override-files).
