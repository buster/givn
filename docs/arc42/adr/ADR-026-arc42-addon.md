---
status: accepted
date: 2026-06-19
decision-makers:
  - givn maintainers
consulted:
  - addon and durable-documentation design record
informed:
  - users enabling architecture documentation
---

# Provide durable Arc42 documentation as an addon

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

- [x] **Hard to reverse** - the addon changes project scaffolding, lifecycle dependencies, and durable documentation ownership.
- [x] **Shapes quality attributes** - it creates an explicit architecture-maintenance and conformance boundary.
- [x] **Structural** - the addon expands the effective graph with `arc42-docs` and fixes the durable output path.
- [x] **Must be shared** - projects and agents must know when Arc42 assessment and conformance checks apply.

## Context and Problem Statement

Projects need durable architecture documentation alongside per-change artifacts. The behavior should be supplied as a bundled addon rather than imposed on every project by the core lifecycle.

## Decision Drivers

- durable project-level chapters
- integration with design review and completion review
- no separate plugin runtime
- explicit enable/disable behavior

## Considered Options

- Bundle Arc42 behavior as an addon that expands the manifest.
- Make Arc42 a mandatory core artifact.
- Leave architecture documentation entirely outside givn.

## Decision Outcome

Chosen option: **Bundle `arc42-docs` as an addon**. The embedded default manifest has no Arc42 artifact, but current `givn init` enables all known addons unless `--no-addons` is passed. Enabling Arc42 creates missing chapter files at the fixed `docs/arc42/` path, adds a required `arc42-docs` artifact after `design`, makes it an archive requirement, and makes `design-review` depend on it.

### Consequences

- `docs/arc42/` is durable and is not archived with a change.
- The change-level `arc42.md` marker is the lifecycle artifact; its `STATUS: DONE` is tracked.
- There is no effective `arc42.output_dir` or chapter-list configuration, and chapter files are not selected through a chapter cascade.
- Disabling Arc42 does not delete existing documentation.

### Confirmation

Addon overlay and initialization tests assert graph expansion, idempotence, required dependencies, fixed output creation, and preservation on disable.

## Pros and Cons of the Options

### Bundled addon

- Good, because projects can use the capability without a plugin runtime and can opt out at init.
- Bad, because the enabled addon adds a required lifecycle and review boundary.

### Mandatory core artifact

- Good, because every project would have architecture docs.
- Bad, because it imposes a project process that is not universally wanted.

### External documentation only

- Good, because givn has no documentation behavior.
- Bad, because lifecycle and conformance checks cannot connect changes to durable architecture.

## More Information

The fixed runtime behavior is in [chapter 8.11](../08-crosscutting-concepts.md#811-addons-and-the-arc42-documentation-artifact); conformance review is described in [chapter 8.14](../08-crosscutting-concepts.md#814-arc42-implementation-conformance-review).
