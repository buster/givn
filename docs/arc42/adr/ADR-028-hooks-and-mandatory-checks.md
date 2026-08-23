---
status: accepted
date: 2026-07-16
decision-makers:
  - givn maintainers
consulted:
  - hook implementation and gate tests
informed:
  - contributors and users configuring lifecycle checks
---

# Separate configurable post hooks from mandatory gate checks

## Qualification

- [x] **Hard to reverse** - gate ordering and failure semantics affect every project lifecycle.
- [x] **Shapes quality attributes** - fail-closed verification and integrity checks protect correctness.
- [x] **Structural** - the binary's mandatory checks and manifest `post` hooks are distinct execution paths.
- [x] **Must be shared** - users must not mistake a removable hook for a non-deactivatable review requirement.

## Context and Problem Statement

The project needs to run project commands at lifecycle gates while preventing a configuration overlay from disabling core safety checks. The current implementation evolved from hook-based verification to a split: mandatory checks are in `cli::check`, and `post` is additive.

## Decision Drivers

- mandatory review safety
- project-specific additional commands
- explicit `GIVN_FEATURES` propagation
- useful failure and `allow_failure` semantics

## Considered Options

- Put every check in user-configurable `post` lists.
- Hardcode every project command and disallow customization.
- Keep binary mandatory checks and allow additive manifest hooks.

## Decision Outcome

Chosen option: **Keep mandatory checks in the binary and use `post` for additional hooks**. `givn check tasks` always runs WIP protection and `verify.command`. `givn check review` always runs `verify`, `verify-e2e`, `integrity`, and, for a named change, overlap and retrieval checks. These mandatory checks are not disabled by removing or marking a `post` entry. Configured `post` hooks run separately; built-in names (`verify`, `verify-e2e`, `lint`, `integrity`) cannot be downgraded with `allow_failure`, while custom shell commands can.

### Consequences

- The embedded default manifest has empty `post` lists for `tasks` and `review`; older comments claiming default `post: [verify]` are not the effective runtime contract.
- `givn check` without an artifact runs only configured `post` hooks for applicable outputs; it does not invoke the artifact-specific mandatory path.
- `givn archive` runs configured hooks of `apply.requires`, then performs its own hard-coded similarity, merge, post-merge verify/e2e, and optional coverage stages.
- `verify.command` and coverage commands are read after applying `givn/config.yaml` and then `givn/commands.yaml` overlays.

### Confirmation

Hook and gate tests assert mandatory checks, non-degradable built-ins, custom `allow_failure`, feature-path propagation, archive hooks, and post-merge verification.

## Pros and Cons of the Options

### Mandatory checks plus additive hooks

- Good, because a project can extend checks without weakening the core gate.
- Bad, because users must understand two execution paths.

### All checks configurable

- Good, because the manifest is uniform.
- Bad, because an overlay could remove the checks that protect review integrity.

### All checks hardcoded

- Good, because behavior is simple to reason about.
- Bad, because projects cannot add their own validation or choose custom commands.

## More Information

The current execution split is documented in [chapter 8.12](../08-crosscutting-concepts.md#812-hooks-command-execution-and-mandatory-checks) and [chapter 6](../06-runtime-view.md).
