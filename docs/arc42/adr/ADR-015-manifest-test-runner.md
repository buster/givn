---
status: accepted
date: 2026-06-19
decision-makers:
  - givn maintainers
consulted:
  - lifecycle and test-runner design record
informed:
  - contributors and users
---

# Declare the test runner explicitly

## Qualification

- [x] **Hard to reverse** - runner configuration is part of every project gate and generated command contract.
- [x] **Shapes quality attributes** - explicit runner selection protects correctness and language neutrality.
- [x] **Structural** - `verify.command`, `verify.e2e_command`, and `GIVN_FEATURES` define the runner interface.
- [x] **Must be shared** - agents, CI, hooks, and archive must use the same command and feature scope.

## Context and Problem Statement

givn cannot reliably infer how a project's step definitions are executed. It must distinguish static linting from running executable specifications and provide one explicit runner contract.

## Decision Drivers

- language and runner neutrality
- explicit feature path
- reproducible CI and local gates
- separation of lint from test execution

## Considered Options

- Declare the runner in project command configuration.
- Auto-detect a runner with an override.
- Let givn only lint and leave every gate entirely external.

## Decision Outcome

Chosen option: **Declare `verify.command` in the project command configuration**. The effective value is resolved from the manifest overlay, with the initialized project's `givn/commands.yaml` providing the checked-in project command; `GIVN_FEATURES` identifies the feature scope.

### Consequences

- The current checkout's `givn/commands.yaml` uses `./run-tests.sh` and `./run-tests.sh --e2e`.
- The embedded manifest fallback is `givn missing-testrunner`, but `givn init` creates `commands.yaml` with project-owned command entries.
- `givn lint` remains a static check and is not the test runner.

### Confirmation

The command overlay tests, hook tests, and Cucumber runner prove that `verify.command` is passed through the effective manifest and that the feature path is exported to the runner.

## Pros and Cons of the Options

### Explicit project command

- Good, because the execution contract is visible and language-agnostic.
- Bad, because each project must provide a working command.

### Auto-detection

- Good, because initial setup can be shorter.
- Bad, because detection can select the wrong runner or feature scope.

### External-only execution

- Good, because givn has no hook responsibility.
- Bad, because the lifecycle cannot enforce the configured runner at its gates.

## More Information

The sentinel refinement is [ADR-021](ADR-021-missing-testrunner-sentinel.md); command and hook behavior is in [chapter 8.12](../08-crosscutting-concepts.md#812-hooks-command-execution-and-mandatory-checks).
