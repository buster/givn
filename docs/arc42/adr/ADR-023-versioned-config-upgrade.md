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

ADR-QUALIFICATION: PASS

### Qualification verdict

qualification: QUALIFIED
alternatives: PASS
architectural_impact: PASS
durable_consequence: PASS
lower_level_artifact: PASS
existing_adr_check: PASS
must_be_shared: SUPPORTING
routing: NEW_ADR
canonical_artifact: null
target_adr: null
replacement_adr: null

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

The 0.3.0 refinement adds an exact 0.2.0-to-0.3.0 compatibility route. When
the project is a clean Git worktree, `givn upgrade` refreshes the managed
scaffold and generated integrations, commits only an explicit Givn-owned
allowlist as Commit A, then creates a visible maintenance migration change and
commits that plan separately as Commit B. The plan records Commit A's hash and
contains no synthetic product feature. Non-Git or dirty projects are refused
before any write; existing active or archived plans are preserved.

### Consequences

- Users review the resulting file and the two migration commits with `git diff`.
- Ejected facets are not automatically upgraded; they remain project-owned.
- Line-based preservation is intentionally conservative and can leave legacy keys for manual review.
- The config marker reaching 0.3.0 means scaffold preparation completed, not that the project-specific migration was reviewed or archived.

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

See [chapter 8.10](../08-crosscutting-concepts.md#810-configuration-versioning-and-command-files), [chapter 6.14](../06-runtime-view.md#614-givn-upgrade-from-020-to-030), and [ADR-005](ADR-005-facet-cascade.md).
