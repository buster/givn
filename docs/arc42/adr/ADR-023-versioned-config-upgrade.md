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

# Version the project scaffold and preserve overrides on upgrade

## Qualification

- [x] **Hard to reverse** - changing upgrade precedence or version markers can damage project-owned configuration across releases.
- [x] **Shapes quality attributes** - preservation and reviewable upgrades protect maintainability and upgradeability.
- [x] **Structural** - `givn upgrade` owns the scaffold merge boundary while project values remain outside it.
- [x] **Must be shared** - users and tooling must know which lines may be rewritten.

## Context and Problem Statement

The embedded configuration documentation evolves with the binary, while project overrides must survive. Without a version marker and upgrade path, users must manually compare old scaffolds.

## Decision Drivers

- preserve user values
- refresh managed documentation
- make upgrades reviewable in git
- avoid hidden migration writes

## Considered Options

- Version the scaffold and merge upgrades while preserving uncommented values.
- Replace the project config wholesale on upgrade.
- Never update generated configuration.

## Decision Outcome

Chosen option: **Use `givn_config_version` and a preservation-oriented `givn upgrade`**. The current implementation re-renders the scaffold, retains uncommented blocks, appends preserved overrides where needed, and auto-enables newly known addons absent from the old config.

### Consequences

- Users review the resulting file with `git diff`.
- Ejected facets are not automatically upgraded; they remain project-owned.
- Line-based preservation is intentionally conservative and can leave legacy keys for manual review.

### Confirmation

Upgrade tests assert version replacement, valid YAML, idempotent rendering for the same version, and preservation of `verify` and addon overrides.

## Pros and Cons of the Options

### Preserve overrides while refreshing scaffold

- Good, because defaults and documentation improve without clobbering project intent.
- Bad, because merge behavior must handle old and new block layouts.

### Replace wholesale

- Good, because implementation is simple.
- Bad, because project values and local decisions can be lost.

### Never update

- Good, because the file is stable.
- Bad, because new supported settings remain undiscoverable and old guidance persists.

## More Information

See [chapter 8.10](../08-crosscutting-concepts.md#810-configuration-versioning-and-command-files) and [ADR-005](ADR-005-facet-cascade.md).
