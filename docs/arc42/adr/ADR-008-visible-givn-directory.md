---
status: accepted
date: 2026-06-19
decision-makers:
  - givn maintainers
consulted:
  - project layout design record
informed:
  - contributors and users
---

# Keep project configuration in a visible `givn/` directory

## Qualification

- [x] **Hard to reverse** - changing the directory would require moving every project artifact and updating every command.
- [x] **Shapes quality attributes** - visibility improves discoverability and supportability.
- [x] **Structural** - `givn/` is the filesystem boundary for configuration, changes, specs, and archive state.
- [x] **Must be shared** - all commands and agents must resolve the same project root layout.

## Context and Problem Statement

The tool needs one recognizable location for configuration and lifecycle data. A hidden directory would reduce discoverability for a workflow whose state is intentionally filesystem-based.

## Decision Drivers

- discoverability
- one project-local configuration boundary
- stable paths for commands and agents

## Considered Options

- Visible `givn/`.
- Hidden `.givn/`.
- Multiple tool-specific locations.

## Decision Outcome

Chosen option: **Visible `givn/`**, containing `config.yaml`, `commands.yaml`, permanent specs, active changes, archive data, and caches.

### Consequences

- A project browser exposes workflow state directly.
- Scripts and documentation use stable `givn/` paths.
- Existing projects would require a migration if the location changed.

### Confirmation

`givn init` creates `givn/specs`, `givn/changes`, and `givn/archive`; all lifecycle and retrieval code resolves these paths from the project root.

## Pros and Cons of the Options

### Visible `givn/`

- Good, because ownership and state are easy to inspect.
- Bad, because the directory is part of the visible project tree.

### Hidden `.givn/`

- Good, because it reduces top-level visual noise.
- Bad, because users and agents can miss the lifecycle source of truth.

### Multiple locations

- Good, because each concern could choose a conventional path.
- Bad, because it recreates the configuration scattering this project is avoiding.

## More Information

The layout is shown in [chapter 8.4](../08-crosscutting-concepts.md#84-project-directory-layout).
