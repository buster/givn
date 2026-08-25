---
status: accepted
date: 2026-06-19
decision-makers:
  - givn maintainers
consulted:
  - lifecycle manifest design record
informed:
  - contributors and users
---

# Represent the workflow as a configurable artifact graph

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

- [x] **Hard to reverse** - project lifecycle overlays, dependencies, and archive requirements depend on the graph schema.
- [x] **Shapes quality attributes** - configuration avoids source forks and allows projects to control process complexity.
- [x] **Structural** - artifact nodes, `requires`, `optional`, `post`, and `apply.requires` define the workflow topology.
- [x] **Must be shared** - commands and agents must follow the effective graph rather than a hidden fixed sequence.

## Context and Problem Statement

Projects may need an extra planning step, an optional artifact, or a different archive gate. A hardcoded lifecycle would force a source fork for each variation.

## Decision Drivers

- add, modify, remove, and reorder artifacts
- explicit dependency validation
- filesystem-derived state over the effective graph
- independent archive requirements

## Considered Options

- Hardcode the lifecycle and expose only shell hooks.
- Replace the entire manifest per project.
- Patch the embedded graph through a project overlay.

## Decision Outcome

Chosen option: **Patch the embedded graph through `givn/config.yaml`**. Artifact ids are deep-merged; new ids append; `remove: true` deletes an artifact; `requires` determines order; `optional` affects lifecycle completion; `apply.requires` determines archive prerequisites.

### Consequences

- Effective graph validation rejects cycles and dangling references.
- The graph can express lifecycle variations without changing the binary.
- The project must keep custom artifact templates and instructions available through the cascade.

### Confirmation

Overlay and state tests cover add, modify, remove, optional artifacts, dependency cycles, dangling references, and topological status order.

## Pros and Cons of the Options

### Overlayed artifact graph

- Good, because the default graph remains upgradeable.
- Bad, because partial merge semantics must be understood.

### Fully replaced manifest

- Good, because the project sees one complete graph.
- Bad, because embedded graph improvements do not flow into the project.

### Hardcoded lifecycle with hooks

- Good, because validation is smaller.
- Bad, because structural workflow changes require a source fork.

## More Information

See [chapter 8.2](../08-crosscutting-concepts.md#82-manifest-and-overlaypatch).
