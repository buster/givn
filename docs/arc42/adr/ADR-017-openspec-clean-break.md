---
status: accepted
date: 2026-06-19
decision-makers:
  - givn maintainers
consulted:
  - migration scope design record
informed:
  - contributors and users
---

# Keep a clean break from OpenSpec

## Qualification

- [x] **Hard to reverse** - accepting legacy directories and tags would make the compatibility surface permanent.
- [x] **Shapes quality attributes** - one layout reduces ambiguity and maintenance burden.
- [x] **Structural** - it defines which project directories and tag namespaces the parser and archive understand.
- [x] **Must be shared** - authors must not mix `openspec/` or `@openspec.*` data with givn artifacts.

## Context and Problem Statement

givn is a successor to an OpenSpec prototype, but carrying both layouts into the runtime would duplicate state and make precedence unclear. Migration support may be useful later without being part of the current core boundary.

## Decision Drivers

- unambiguous project layout
- small parser and merge surface
- no silent compatibility behavior
- explicit future migration option

## Considered Options

- Read both OpenSpec and givn layouts indefinitely.
- Read only givn's layout and defer a deliberate migration command.
- Convert projects automatically during every invocation.

## Decision Outcome

Chosen option: **Read only givn paths and `@givn.*` tags**. A future `givn migrate` command is a separate, deferred capability rather than implicit compatibility.

### Consequences

- OpenSpec data is not included in current state, lint, retrieval, or archive operations.
- Migration, if added, can be reviewed as an explicit transformation.
- Existing OpenSpec projects need an intentional transition.

### Confirmation

The source tree contains no runtime fallback to `openspec/`; current Gherkin parsing and merge tests use only the `@givn.*` namespace.

## Pros and Cons of the Options

### Clean break

- Good, because every command has one source and one namespace.
- Bad, because migration is not automatic.

### Permanent dual read

- Good, because old projects keep working.
- Bad, because ambiguous precedence and two schemas become permanent support obligations.

### Automatic conversion on invocation

- Good, because users may not need a migration command.
- Bad, because a read-like command would perform surprising writes.

## More Information

The deferred migration is tracked as technical debt in [chapter 11](../11-risks-and-technical-debt.md).
