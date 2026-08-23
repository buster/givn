---
status: accepted
date: 2026-06-19
decision-makers:
  - givn maintainers
consulted:
  - initial workflow design record
informed:
  - contributors and users
---

# Build givn as a standalone tool

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

- [x] **Hard to reverse** - replacing the tool boundary later would require migrating project layouts, commands, and generated guidance.
- [x] **Shapes quality attributes** - the boundary determines installability, upgrade safety, and the maintenance surface.
- [x] **Structural** - it defines a new CLI, project directory, artifact model, and ownership boundary instead of extending OpenSpec.
- [x] **Must be shared** - every contributor must know that givn owns its own workflow and does not read the OpenSpec layout.

## Context and Problem Statement

The prototype was a TypeScript/npm fork of OpenSpec with hardcoded workflow and configuration behavior. The project needs a standalone product boundary that can evolve without inheriting the prototype's install and customization constraints.

## Decision Drivers

- independent release and compatibility boundary
- no npm or root requirement
- project-owned customization without a source fork
- preservation of the Gherkin-first lifecycle

## Considered Options

- Continue extending the OpenSpec fork.
- Publish givn as a standalone successor.
- Build a library only and leave the workflow in consumer repositories.

## Decision Outcome

Chosen option: **Publish givn as a standalone successor**, because it preserves the useful lifecycle while allowing a new binary, configuration model, and ownership contract.

### Consequences

- givn has its own CLI, `givn/` project directory, embedded assets, and release lifecycle.
- OpenSpec compatibility is not part of the current product boundary.
- The lifecycle and Gherkin merge behavior must be maintained by this repository.

### Confirmation

The public CLI, embedded asset set, and repository-owned acceptance fixtures are built and tested in this repository; no runtime path reads an `openspec/` directory.

## Pros and Cons of the Options

### Continue the OpenSpec fork

- Good, because initial migration effort is lower.
- Bad, because prototype assumptions remain coupled to the product boundary.

### Publish a standalone successor

- Good, because architecture and distribution can be designed for givn's requirements.
- Bad, because compatibility and migration are not supplied automatically.

### Publish a library only

- Good, because consumers can choose their own workflow.
- Bad, because the product would lose the integrated lifecycle and executable-spec contract.

## More Information

The current system boundary is described in [chapter 3](../03-context-and-scope.md) and the OpenSpec migration non-goal is recorded in [ADR-017](ADR-017-openspec-clean-break.md).
