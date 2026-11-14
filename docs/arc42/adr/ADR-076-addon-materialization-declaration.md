---
status: accepted
date: 2026-11-14
decision-makers: project maintainer
consulted: none
informed: none

# Addon materialization is declared on the addon registry entry

## Qualification

ADR-QUALIFICATION: PASS

### Qualification verdict

```yaml
qualification: QUALIFIED
alternatives: PASS - registry-declared materialization versus a separate reporting table versus deriving paths by re-running scaffolds into a probe directory
architectural_impact: PASS - the field is the single source of truth shared by the enable path and the reporting path; it fixes the ownership boundary between `AddonSpec` and the scaffold routines
durable_consequence: PASS - reversing to a side table would leave two copies of every declared path, and the compile-time exhaustiveness guarantee would be lost, requiring a re-audit of all scaffold routines
lower_level_artifact: FAIL - no specification, chapter, or design document can make a new scaffold path fail to compile
existing_adr_check: PASS - searched `docs/arc42/09-architecture-decisions.md` and `docs/arc42/adr/` (including `archive/`); no record governs where an addon's filesystem effect is declared. ADR-023 covers managed config refresh, which is a different boundary
must_be_shared: PASS - a future addon added without a declaration must be impossible for any contributor, not merely caught in review
routing: NEW_ADR
canonical_artifact: none
target_adr: none
replacement_adr: none
```

### MADR checklist evidence

- [x] **Hard to reverse** - Reverting later would cost significant effort
      because: every consumer of `Materialization` would have to move to a
      second lookup, and the exhaustive match that makes an undeclared addon
      fail to compile would have to be rebuilt in the reporting module.
- [x] **Shapes quality attributes** - It materially affects maintainability
      because: one declaration serves the enable path and the report, so a
      scaffold change cannot leave the report describing stale paths.
- [x] **Structural** - It defines a boundary, contract, or dependency direction
      because: it makes `AddonSpec` the owner of an addon's filesystem effect,
      with `cli::addon_report` and the scaffold routines as consumers instead
      of independent producers of path literals.
- [x] **Must be shared** - Developers unfamiliar with it would systematically
      violate the architecture because: a contributor adding an addon would
      naturally write path literals inside its scaffold routine and forget the
      report.

## Context and Problem Statement

`givn addons list` must report whether each addon's materialized files exist.
Those paths today live inside the individual scaffold routines in
`src/cli/addons.rs`, and the generated skill name lives in a separate match
function. A report needs the same paths; taking a second copy creates a silent
divergence whenever a scaffold gains a file.

## Decision Drivers

- One source of truth for the paths an addon writes.
- A missing declaration must be detected mechanically, not in review.
- The enable path and the report must not be able to disagree.

## Considered Options

- Declare the materialization on the `AddonSpec` registry entry.
- Keep a check table beside the registry, used only by the report.
- Derive the paths by re-running each scaffold into a temporary probe directory.

## Decision Outcome

Chosen option: "Declare the materialization on the `AddonSpec` registry entry",
because it is the only option where an undeclared addon fails to build and both
consumers read the same value.

### Consequences

- Good, because the scaffold routines dispatch on the declaration, so a new
  path is reported the moment it is scaffolded.
- Bad, because four registry entries must declare an explicit `None` and their
  authors must confirm the addon truly writes nothing, which is a judgement the
  compiler cannot make.

### Confirmation

The capability's scenarios assert the reported state against real fixture
files, and the exhaustive match in `scaffold_enabled` fails the build when a
registry entry lacks a declaration.

## Pros and Cons of the Options

### Declare the materialization on the `AddonSpec` registry entry

- Good, because undeclared addons fail to compile.
- Good, because the enable path and the report read one value.
- Bad, because the registry type grows a field every addon must fill.

### Keep a check table beside the registry

- Good, because no existing code changes.
- Bad, because a scaffold that gains a path leaves the report silently wrong.

### Derive the paths by re-running each scaffold into a probe directory

- Good, because no declaration exists to drift.
- Bad, because scaffolding has side effects beyond file creation, and a probe
  run could write outside its directory.

## More Information

Introduced by the `addon-artifact-report` change. Recorded in
`docs/arc42/05-building-block-view.md` (`cli::addons`) and glossary terms
**Addon materialization** and **Materialization state**.
