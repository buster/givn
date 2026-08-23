---
status: accepted
date: 2026-08-15
decision-makers:
  - givn maintainers
consulted:
  - repository-ownership acceptance fixtures
informed:
  - contributors and consumer repositories
---

# Keep Givn's CLI acceptance fixtures in Givn

## Qualification

- [x] **Hard to reverse** - moving the acceptance boundary changes release confidence and repository dependencies.
- [x] **Shapes quality attributes** - a real subprocess fixture protects integration correctness and failure recovery.
- [x] **Structural** - it defines ownership and the external CLI/filesystem test boundary.
- [x] **Must be shared** - contributors need one authoritative acceptance host for the product CLI.

## Context and Problem Statement

Testing Givn's archive and review behavior from a consumer repository makes the product depend on private consumer layout and CI. The product must test its own CLI at its public process boundary.

## Decision Drivers

- real compiled CLI
- isolated temporary project
- no provider or network dependency
- primary assertions on child-process output and status

## Considered Options

- Keep fixtures in a consumer repository.
- Replace the CLI with in-process test doubles.
- Own generic temporary-project fixtures in this repository.

## Decision Outcome

Chosen option: **Own generic temporary-project fixtures in Givn and invoke the compiled CLI**. Permanent-tree checks are secondary evidence; child exit status and public output are primary.

### Consequences

- Givn maintains fixture layout and subprocess helpers.
- Consumer repositories retain domain specs but do not host Givn's core acceptance contract.
- Child-process coverage must be assessed explicitly.

### Confirmation

Repository-ownership scenarios run review, archive, and pre-merge failure through the compiled CLI and assert the filesystem recovery boundary.

## Pros and Cons of the Options

### Givn-owned subprocess fixture

- Good, because it tests the product where its behavior is implemented.
- Bad, because the fixture itself requires maintenance.

### Consumer-hosted fixture

- Good, because a real consumer supplies context.
- Bad, because product acceptance depends on unrelated repository access and conventions.

### In-process fake

- Good, because tests can be fast.
- Bad, because the shipped command and process boundary remain untested.

## More Information

See [chapter 5.6](../05-building-block-view.md#56-repository-owned-acceptance-blocks) and [chapter 6.13](../06-runtime-view.md#613-repository-owned-acceptance-flow).
