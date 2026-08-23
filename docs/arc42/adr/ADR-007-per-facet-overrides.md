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

# Override templates, instructions, and other facets independently

## Qualification

- [ ] **Hard to reverse** - an individual facet can be restored or replaced without a data migration.
- [x] **Shapes quality attributes** - independent overrides reduce maintenance coupling and upgrade diff size.
- [x] **Structural** - it defines the addressable units of the configuration and asset boundary.
- [x] **Must be shared** - every author must know that changing one facet does not replace the artifact bundle.

## Context and Problem Statement

Templates, instructions, and dependency declarations have different owners and change rates. Treating an artifact as one indivisible override forces projects to copy content they do not intend to customize.

## Decision Drivers

- minimal project-owned content
- independent evolution of instructions and templates
- predictable cascade lookup

## Considered Options

- Override each facet independently.
- Override an entire artifact bundle.
- Use one monolithic project configuration for all facets.

## Decision Outcome

Chosen option: **Independent facet overrides**, with paths such as `givn/artifacts/<id>/template.*` and `givn/artifacts/<id>/instructions.md`.

### Consequences

- A project can keep the embedded instruction while ejecting only a template.
- Artifact identity and facet names must remain stable enough for lookup.
- The current resolver supports template and instruction facets; the manifest remains a separate structured overlay.

### Confirmation

The cascade tests and `givn eject` interactions verify a template-only override and an instruction-only override independently.

## Pros and Cons of the Options

### Independent facets

- Good, because the ownership boundary is precise.
- Bad, because users must choose the correct facet path.

### Whole artifact bundles

- Good, because lookup is simple.
- Bad, because unrelated defaults are copied and become stale together.

### Monolithic configuration

- Good, because there is one file.
- Bad, because content, graph, and instructions become unnecessarily coupled.

## More Information

See [ADR-005](ADR-005-facet-cascade.md) and [chapter 8.1](../08-crosscutting-concepts.md#81-the-cascade-engine-override-files).
