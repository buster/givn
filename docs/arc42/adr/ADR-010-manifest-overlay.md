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

Chosen option: **Overlay/patch the embedded manifest**. Existing artifact ids are deep-merged field by field; new ids are appended; `remove: true` removes an artifact; the `apply` block replaces its effective block.

**Current amendment (prepare-0-9-10).** The generic overlay is the policy and
artifact-graph authority in `givn/config.yaml`. Executable settings are not a
second generic overlay: `givn/commands.yaml` is parsed through a dedicated
schema limited to `verify`, `coverage`, and `run`, and its block defaults are
resolved there. A policy key in the commands file and an executable key in the
policy file are refused by name before any use. Both files remain
project-owned; the split gives each kind of setting one authority.

**Current amendment (add-ise-addon-and-specification).** A third authority is
added for the declared Specification: `givn/specification.yaml` holds the five
retained roles as path globs and one `source:` list of replaceable roots. It is
not an overlay and adds no manifest key. It is parsed by one strict read-only
loader — unknown or missing keys are refused by name — and only the one-time
`ise` scaffold writes it: when the addon becomes effective and the file is
absent, and on a managed upgrade when the addon is already effective and the
file is absent, where the written file joins the staged allowlist. An existing
declaration is never written or staged. The file is read only while `ise` is
effective, so a project without the addon never reads it. The alternatives were
a `specification:` block in `givn/config.yaml` (rejected: the lenient overlay
top level hides a misspelled block) and convention (rejected: convention cannot
express the `source:` partition).

### Consequences

- The effective graph is the embedded manifest plus `givn/config.yaml`; `givn/commands.yaml` supplies only the executable `verify`, `coverage`, and `run` values through its dedicated schema.
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
