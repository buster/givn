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

> This record was superseded by ADR-027, which was later retired as non-qualifying; the exit-code intent and the final command name are retained in `givn/specs/verify-change/usecase.md` and the lint capability specification.

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

The historical contract defined clean, parse-error, and WIP outcomes. The command-name portion was superseded by **ADR-027** (retired as non-qualifying), which called the command `givn lint` and added duplicate-title precedence; the current contract lives in the verify-change specification.

### Consequences

- Existing `givn verify` references required migration.
- The current contract is maintained in `givn/specs/verify-change/`.

### Confirmation

Current lint tests assert the `givn lint` command and exit-code behavior specified in `givn/specs/verify-change/lint-command.feature`.

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

See `givn/specs/verify-change/usecase.md` and [chapter 8.8](../../08-crosscutting-concepts.md#88-error-handling-and-exit-codes).
