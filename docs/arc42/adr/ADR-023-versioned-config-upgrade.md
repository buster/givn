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
- keep migration work visible and ordered

## Considered Options

- Version the scaffold and merge upgrades while preserving uncommented values.
- Replace the project config wholesale on upgrade.
- Never update generated configuration.

## Decision Outcome

Chosen option: **Use `givn_config_version` and a preservation-oriented managed
upgrade**. The implementation re-renders the scaffold, retains uncommented
blocks, synchronizes generated integrations, and commits only an explicit
Givn-owned path allowlist when files changed. Ignored generated paths are
force-staged only as individual allowlisted paths; no directory or complete
worktree is force-staged. Addon enablement remains explicit.

A later refinement added the embedded migration catalog. A declared route is
resolved from the active config version to the installed package version,
including compatible skipped patch releases. The managed preparation is the
first revision; the catalog bundles are then combined into one visible aggregate
maintenance change and committed as the second revision. The aggregate metadata
records every bundle in source-to-target order, and the plan contains no
synthetic product feature. Dirty projects and projects without a usable backend
are refused before any write; an active aggregate plan is detected through its
visible `migration.yaml` and is preserved.

**Current amendment (prepare-0-9-10).** The managed upgrade runs through the
Git/Fossil selection seam on both backends, captures a checkpoint revision, and
stages only the calculated allowlist. The supported migration baseline is 1.0.
The shipped catalog is empty, so no pre-1.0 route is selected; retired 0.x
configuration is refused with a manual-transition message and no alias. Future
routes keep the same bundle contract and resolver. On failure the command
reports the checkpoint, completed writes, revisions actually published,
best-effort restoration, remaining partial state, and pending work; it performs
no reset or history rewrite. `givn commit` separately requires explicit paths on
both backends.

### Consequences

- Users review the resulting files and managed/migration revisions with their backend diff (`git show` or `fossil diff`).
- Ejected facets are not automatically upgraded; they remain project-owned.
- Line-based preservation is intentionally conservative and can leave legacy keys for manual review.
- The config marker records the managed scaffold version; it does not mean that the aggregate project migration was reviewed or archived.
- A transition with no catalog route creates no migration change, while its managed files still receive one allowlist-only commit when changed. At the 1.0 baseline there is no pre-1.0 route.

### Confirmation

Upgrade tests assert version replacement, valid YAML, idempotent rendering for the same version, preservation of user overrides, refusal of retired/misplaced settings, catalog path ordering, skipped-patch resolution, managed-only refresh, aggregate plan creation for synthetic future routes, and the empty shipped 1.0 catalog.

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

See [chapter 8.10](../08-crosscutting-concepts.md#810-configuration-versioning-and-command-files), [chapter 6.13](../06-runtime-view.md#613-givn-upgrade-and-declared-migration-bundles), the [migration catalog](../../../assets/migrations/catalog.yaml), and [ADR-005](ADR-005-facet-cascade.md).
