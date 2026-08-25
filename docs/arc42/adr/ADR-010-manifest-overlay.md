---
status: accepted
date: 2026-06-19
decision-makers:
  - givn maintainers
consulted:
  - manifest design record
informed:
  - contributors and users
---

# Customize the manifest through an overlay

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

- [x] **Hard to reverse** - project overlays and upgrade behavior depend on merge precedence and artifact identity.
- [x] **Shapes quality attributes** - overlays provide customization without forking and preserve upgradeability.
- [x] **Structural** - the overlay patches the artifact dependency graph and archive gate.
- [x] **Must be shared** - project authors and commands must interpret the same effective manifest.

## Context and Problem Statement

Projects need to add, remove, or alter lifecycle artifacts while still receiving improvements to the embedded default graph. A fully replaced manifest would make every project responsible for future graph changes.

## Decision Drivers

- retain embedded defaults
- support add/modify/remove operations
- validate dependency and archive graphs
- keep project customization local

## Considered Options

- Eject and fully replace the manifest.
- Apply an overlay/patch to the embedded manifest.
- Hardcode a fixed lifecycle and expose only shell hooks.

## Decision Outcome

Chosen option: **Overlay/patch the embedded manifest**. Existing artifact ids are deep-merged field by field; new ids are appended; `remove: true` removes an artifact; scalar blocks such as `verify`, `coverage`, and `apply` replace their corresponding effective blocks.

### Consequences

- The effective graph is the embedded manifest plus `givn/config.yaml`, then `givn/commands.yaml` for command and coverage settings.
- Graph validation rejects dangling dependencies and cycles.
- A project can intentionally change lifecycle order and archive requirements, so the effective manifest must be inspected when debugging state.

### Confirmation

Overlay tests cover add, deep merge, remove, optional artifacts, cycle rejection, dangling references, and addon expansion.

## Pros and Cons of the Options

### Overlay/patch

- Good, because projects inherit new embedded defaults while retaining overrides.
- Bad, because precedence and partial merge rules must be explicit.

### Fully replaced manifest

- Good, because the effective graph is obvious in one file.
- Bad, because upgrades cannot improve the graph without manual migration.

### Fixed lifecycle with hooks

- Good, because validation is simpler.
- Bad, because projects must fork or cannot express legitimate workflow variants.

## More Information

See [chapter 8.2](../08-crosscutting-concepts.md#82-manifest-and-overlaypatch) and [ADR-024](ADR-024-configurable-artifact-graph.md).
