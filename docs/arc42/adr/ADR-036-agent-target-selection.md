---
status: accepted
date: 2026-08-10
decision-makers:
  - givn maintainers
consulted:
  - `givn init` and `givn skills sync` behavior
informed:
  - users with existing agent integrations
---

# Select agent targets at initialization and persist the choice

## Qualification

- [x] **Hard to reverse** - changing target defaults affects generated files and project repository contents.
- [x] **Shapes quality attributes** - selective generation reduces repository noise while retaining multi-agent support.
- [x] **Structural** - `skills.targets` controls the generated skill and command destinations.
- [x] **Must be shared** - init, sync, and agents must use the same persisted selection.

## Context and Problem Statement

Generating all supported agent directories creates files users did not request. A project may nevertheless need all supported targets, so the choice must be explicit and durable.

## Decision Drivers

- clean single-agent default
- persistent multi-agent selection
- non-interactive fallback
- no writes outside owned `AGENTS.md` blocks

## Considered Options

- Write all targets unconditionally.
- Ask during init and persist `skills.targets`, defaulting to opencode.
- Write only the neutral `.agents` target.

## Decision Outcome

Chosen option: **Prompt for targets on a new config and persist the selection**, with `opencode` as the default and EOF/empty/no-known selection falling back to it. `givn skills sync` writes only selected skills and opencode commands. This refines the target portion of [ADR-013](ADR-013-agent-integration.md); the user-owned `AGENTS.md` rule remains.

### Consequences

- The supported target set is `opencode`, `agents`, and `claude`.
- Existing config preserves its selection on re-init; unknown target names are dropped by the loader.
- The `givn-project` and, when enabled, `absolute-mode` fenced blocks remain separate from generated skill targets.

### Confirmation

Interactive, non-interactive, and synchronization tests assert persisted selection, exact output directories, and target exclusion.

## Pros and Cons of the Options

### Persisted init selection

- Good, because the common single-agent case is clean and the multi-agent case is supported.
- Bad, because init has an input step for a new config.

### Write all targets

- Good, because no choice is required.
- Bad, because every project receives unused directories.

### Neutral target only

- Good, because it avoids vendor-specific mirrors.
- Bad, because it ignores the target an opencode user actually invokes.

## More Information

See [chapter 8.7](../08-crosscutting-concepts.md#87-skill-and-rule-generation) and [ADR-013](ADR-013-agent-integration.md).
