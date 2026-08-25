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

# Use YAML for the artifact manifest

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

- [x] **Hard to reverse** - changing the manifest format would require migrating project overlays and embedded graph assets.
- [x] **Shapes quality attributes** - comments and nested data improve configuration maintainability and reviewability.
- [x] **Structural** - the manifest encodes artifact nodes, dependencies, archive requirements, and command settings.
- [x] **Must be shared** - projects and tooling must parse one manifest representation.

## Context and Problem Statement

The lifecycle graph needs nested artifact definitions, dependency lists, archive settings, and comments in the generated configuration. The format must be readable and supported by the Rust serialization stack.

## Decision Drivers

- readable nested configuration
- comments in the generated scaffold
- compatibility with Rust serialization

## Considered Options

- YAML.
- TOML.
- KDL or a custom format.

## Decision Outcome

Chosen option: **YAML**, parsed with `serde_yaml_ng`, because it supports the graph shape and commented project configuration without adding a custom parser.

### Consequences

- Embedded and project manifest documents use YAML.
- `serde_yaml_ng` is the maintained parser dependency used by the checkout.
- Overlay semantics must be documented separately from YAML syntax.

### Confirmation

Manifest and overlay tests parse the embedded YAML and exercise add, modify, remove, and graph-validation cases.

## Pros and Cons of the Options

### YAML

- Good, because nested graph data and comments are natural.
- Bad, because YAML has implicit-type and indentation hazards.

### TOML

- Good, because its scalar rules are comparatively strict.
- Bad, because the graph and patch representation would be less direct for this asset set.

### KDL or custom format

- Good, because syntax could be tailored to the graph.
- Bad, because it adds parser and tooling maintenance without a required capability.

## More Information

The effective manifest and overlay are described in [chapter 8.2](../08-crosscutting-concepts.md#82-manifest-and-overlaypatch).
