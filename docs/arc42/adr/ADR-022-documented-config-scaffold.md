---
status: accepted
date: 2026-06-19
decision-makers:
  - givn maintainers
consulted:
  - configuration and upgrade design record
informed:
  - contributors and users
---

# Generate a self-documenting configuration scaffold

## Qualification

- [ ] **Hard to reverse** - comments can be revised without changing project data.
- [x] **Shapes quality attributes** - discoverable defaults improve maintainability and reduce configuration drift.
- [x] **Structural** - the generated `givn/config.yaml` documents the project overlay surface.
- [x] **Must be shared** - users and agents need one visible explanation of supported configuration keys.

## Context and Problem Statement

Configuration that is technically possible but undocumented forces users to inspect source or guess. The project needs a visible scaffold with defaults, allowed forms, and ownership rules.

## Decision Drivers

- discoverability
- safe editing and upgrade review
- alignment with the manifest schema
- separation of structural config from project commands

## Considered Options

- Generate a fully commented `givn/config.yaml` scaffold.
- Generate a minimal empty configuration.
- Put all documentation in external reference pages only.

## Decision Outcome

Chosen option: **Generate a fully commented, versioned `givn/config.yaml`**. It documents the overlay, addon, archive, and skill-target surfaces. Test-runner and coverage commands are generated in the separate `givn/commands.yaml` file.

### Consequences

- `givn_config_version` records the binary version used for the scaffold.
- Comments are managed documentation; uncommented project values are user-owned.
- The template and schema require a synchronization test.

### Confirmation

The configuration-template test checks documented top-level manifest keys, and initialization tests assert both scaffold files are created.

## Pros and Cons of the Options

### Commented scaffold

- Good, because the configuration surface is visible at the point of editing.
- Bad, because the embedded template needs maintenance.

### Minimal empty file

- Good, because it is short.
- Bad, because users cannot discover defaults or supported fields locally.

### External documentation only

- Good, because the config file stays small.
- Bad, because external documentation can drift and is less available during editing.

## More Information

See [chapter 8.10](../08-crosscutting-concepts.md#810-configuration-versioning-and-command-files).
