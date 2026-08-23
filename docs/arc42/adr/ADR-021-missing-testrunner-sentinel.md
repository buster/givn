---
status: accepted
date: 2026-06-19
decision-makers:
  - givn maintainers
consulted:
  - test-runner setup design record
informed:
  - contributors and users
---

# Fail explicitly when the effective test runner is the sentinel

## Qualification

- [ ] **Hard to reverse** - the sentinel command itself is replaceable.
- [x] **Shapes quality attributes** - a non-zero fallback prevents false GREEN results at gate points.
- [x] **Structural** - the sentinel is a built-in CLI command referenced by the effective `verify.command`.
- [x] **Must be shared** - every fresh or incompletely configured project needs the same failure semantics.

## Context and Problem Statement

givn cannot know how a project executes its step definitions. Silently doing nothing would make an unconfigured project appear verified.

## Decision Drivers

- fail closed at execution gates
- provide actionable setup guidance
- keep the runner language-agnostic
- keep static lint separate

## Considered Options

- Use `givn missing-testrunner` as the embedded fallback.
- Guess a runner from project files.
- Treat an absent runner as a successful no-op.

## Decision Outcome

Chosen option: **Use the non-zero `givn missing-testrunner` sentinel as the embedded fallback**. It prints setup examples and fails when it is the effective command. `givn init` currently creates `givn/commands.yaml` with project-owned example commands, so an initialized project normally resolves those commands instead of the embedded sentinel.

### Consequences

- A project must replace the generated command with a working Gherkin/Cucumber runner.
- A missing or non-working generated script fails through the shell command path, rather than automatically invoking the sentinel.
- `givn lint` remains independent and does not confirm GREEN.

### Confirmation

The missing-testrunner command exits non-zero and prints guidance; manifest and command-overlay tests assert the fallback and the project command precedence.

## Pros and Cons of the Options

### Explicit sentinel

- Good, because a missing decision is visible and fail-closed.
- Bad, because users must configure a project-specific command.

### Auto-detection

- Good, because setup may be shorter.
- Bad, because a wrong runner or feature path can produce misleading results.

### Successful no-op

- Good, because commands appear convenient.
- Bad, because it creates false confidence and hides missing tests.

## More Information

The declaration boundary is [ADR-015](ADR-015-manifest-test-runner.md); current command-file precedence and gates are in [chapter 8.12](../08-crosscutting-concepts.md#812-hooks-command-execution-and-mandatory-checks).
