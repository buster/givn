---
status: superseded by ADR-027
date: 2026-06-19
decision-makers:
  - givn maintainers
consulted:
  - initial CLI contract
informed:
  - contributors and CI users
---

# Historical record: initial static-check contract

> This record is superseded by [ADR-027](../ADR-027-lint-command-name.md). The exit-code intent is retained there with the final command name.

## Qualification

- [x] **Hard to reverse** - scripts and CI depend on the static-check command and exit values.
- [x] **Shapes quality attributes** - distinct parse/WIP outcomes protect verification clarity.
- [x] **Structural** - it defined a public CLI interface.
- [x] **Must be shared** - callers need stable exit semantics.

## Context and Problem Statement

The prototype needed a static Gherkin check that did not execute tests.

## Decision Drivers

- parse and WIP distinction
- CI-friendly exit values
- separation from the test runner

## Considered Options

- Keep the prototype command name `givn verify`.
- Rename the static command to `givn lint`.
- Delegate all static checking to the external runner.

## Decision Outcome

The historical contract defined clean, parse-error, and WIP outcomes. The command-name portion was superseded by **ADR-027**, which calls the command `givn lint` and adds duplicate-title precedence.

### Consequences

- Existing `givn verify` references required migration.
- The current contract is maintained in ADR-027.

### Confirmation

Current lint tests assert the ADR-027 command and exit-code behavior.

## Pros and Cons of the Options

### Rename to lint

- Good, because it separates static checking from `verify.command`.
- Bad, because old invocations need updating.

### Keep verify

- Good, because old invocations remain familiar.
- Bad, because the name remains ambiguous.

### External-only checking

- Good, because the CLI is smaller.
- Bad, because static feedback depends on each project's runner.

## More Information

See [ADR-027](../ADR-027-lint-command-name.md) and [chapter 8.8](../../08-crosscutting-concepts.md#88-error-handling-and-exit-codes).
