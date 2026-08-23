---
status: accepted
date: 2026-06-19
decision-makers:
  - givn maintainers
consulted:
  - CLI contract design record
informed:
  - contributors and users
---

# Name the static Gherkin check `givn lint`

## Qualification

- [x] **Hard to reverse** - the command name and exit-code contract are used in scripts and generated guidance.
- [x] **Shapes quality attributes** - clear separation prevents a static check from being mistaken for executable verification.
- [x] **Structural** - it defines a public CLI interface distinct from `verify.command`.
- [x] **Must be shared** - agents, CI, and hooks must use the same command semantics.

## Context and Problem Statement

The prototype used `givn verify` for a static check while `verify.command` named the test runner. The shared word implied that the static command executed tests.

## Decision Drivers

- unambiguous CLI vocabulary
- stable parse/WIP/duplicate exit codes
- no hidden test execution
- compatibility with hook configuration

## Considered Options

- Keep `givn verify` for static checking.
- Rename the static command to `givn lint` and reserve `verify.command` for the runner.
- Remove the built-in static command.

## Decision Outcome

Chosen option: **Use `givn lint` for static Gherkin checking**. It parses features, reports WIP/PENDING, performs the default tree-wide overlap scan, and returns 0/1/3/2 with precedence parse error, duplicate title, WIP, clean. It never runs tests.

### Consequences

- `givn verify` is not a current command.
- GREEN comes from the configured test runner, not lint.
- The original lint contract is retained as the superseded [ADR-019](archive/ADR-019-lint-contract.md).

### Confirmation

Lint scenarios assert parse, duplicate-title, WIP, scope, JSON, and precedence behavior; command help exposes `lint` rather than `verify`.

## Pros and Cons of the Options

### `givn lint`

- Good, because the name states that the operation is static.
- Bad, because existing scripts using `givn verify` need migration.

### Keep `givn verify`

- Good, because old scripts remain unchanged.
- Bad, because it remains ambiguous beside `verify.command`.

### Remove static checking

- Good, because the CLI is smaller.
- Bad, because parse and WIP feedback would move into every external runner.

## More Information

See [chapter 8.8](../08-crosscutting-concepts.md#88-error-handling-and-exit-codes) and [ADR-019](archive/ADR-019-lint-contract.md).
