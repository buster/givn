---
status: deprecated
date: 2026-06-19
decision-makers:
  - givn maintainers
consulted:
  - initial repository setup
informed:
  - contributors
---

# Historical record: repository location and version-control setup

> This is retained for historical traceability. It is repository process, not an active architecture contract.

## Qualification

- [ ] **Hard to reverse** - repository location and local git setup can change without changing product architecture.
- [ ] **Shapes quality attributes** - the setup does not define runtime quality.
- [ ] **Structural** - it describes the development repository, not the product's component boundaries.
- [ ] **Must be shared** - contributors need to locate the checkout, but this is operational information.

## Context and Problem Statement

The initial project was created in a local checkout before a public remote existed.

## Decision Drivers

- local development
- git history during early design
- defer remote publication decisions

## Considered Options

- Work in the local `~/projects/givn` checkout with local git only.
- Start with a hosted remote repository.
- Use no version control during prototyping.

## Decision Outcome

The historical setup used **a local git repository at the development checkout**.

### Consequences

- Early design and implementation evidence was recorded locally.
- Distribution and remote CI remained separate work.

### Confirmation

The checkout has git history; current release and CI plans are documented separately.

## Pros and Cons of the Options

### Local git first

- Good, because design work can proceed without external infrastructure.
- Bad, because remote collaboration is unavailable until separately configured.

### Hosted remote first

- Good, because collaboration is immediate.
- Bad, because it was not required to establish the local architecture.

### No version control

- Good, because setup is minimal.
- Bad, because architecture and behavior history would be lost.

## More Information

See [chapter 7](../../07-deployment-view.md) for the current distribution status.
