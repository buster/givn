---
status: accepted
date: 2026-06-19
decision-makers:
  - givn maintainers
consulted:
  - distribution requirements
informed:
  - contributors and users
---

# Ship one native binary without runtime prerequisites

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

- [x] **Hard to reverse** - a runtime-dependent distribution model would change installation, release artifacts, and project support assumptions.
- [x] **Shapes quality attributes** - it directly sets the installability and operational portability floor.
- [x] **Structural** - the executable and embedded assets form the product deployment boundary.
- [x] **Must be shared** - release, documentation, and support must all treat `givn` as the one executable entry point.

## Context and Problem Statement

The predecessor's npm and root-install requirements obstruct adoption. givn needs a distribution unit that runs without npm, sudo, or a separate language runtime.

## Decision Drivers

- frictionless installation
- predictable project execution
- embedded default assets
- support for release, Cargo, and installer channels

## Considered Options

- A single native executable with embedded defaults.
- A package plus a required language runtime.
- A container or hosted service as the primary runtime.

## Decision Outcome

Chosen option: **A single native executable with embedded defaults**, because it removes runtime setup from normal use while keeping the project filesystem as the configuration boundary.

### Consequences

- The default binary does not require the retrieval feature or model cache.
- Optional retrieval builds add native ONNX dependencies and local caches.
- Multi-platform static-linking and release automation remain unfinished deployment work.

### Confirmation

The default Cargo feature set is empty, embedded assets are loaded from the binary, and the CLI can run commands such as `completions` without a project runtime.

## Pros and Cons of the Options

### Single native executable

- Good, because installation and invocation have one stable unit.
- Bad, because native optional dependencies complicate some feature-enabled builds.

### Package plus language runtime

- Good, because ecosystem tooling may be familiar.
- Bad, because every user must provision and maintain the runtime.

### Container or hosted service

- Good, because deployment can be centrally controlled.
- Bad, because it violates the local, repository-owned workflow and adds service dependencies.

## More Information

The target platforms and current release status are in [chapter 7](../07-deployment-view.md).
