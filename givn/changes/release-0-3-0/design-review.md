# Design Review: release-0-3-0

## Grilling Evidence

### Scope

**Question:** Does the change cover the requested release, complete stacked
delta, LLM migration guide, automatic migration change, clean Git boundary,
two-commit evidence, and the documented default Cargo feature set without
silently changing consumer project behavior?

**Answer:** Yes. The proposal, feature scenarios, and design cover package
versioning, changelog, the complete 0.2.0-to-0.3.0 guide, managed integration
refresh, migration-free Managed Upgrades, the embedded migration catalog,
allowlisted Commit A, aggregate migration plans with Commit A evidence, and the
refusal/no-duplication boundaries. The ordinary Cargo installation enables all
four released capabilities while `--no-default-features` remains explicit.
Domain specs, source code, ADRs, Arc42 facts, and ejected facets remain
LLM-reviewed migration work. The installed Givn binary is not self-upgraded.

### Technology Choices

**Question:** Is a Git subprocess plus a small embedded catalog and existing
rendering code simpler and safer than adding a Git library or a generic
migration framework?

**Answer:** Yes. The current project already treats Git diffs as the review
surface. Calling the installed Git CLI avoids a new dependency and preserves
the user's exact Git configuration. The catalog is declarative and validates
 assets and ordered edges; future migration content is added as assets rather
 than as new Rust conditionals. Cargo's default list is explicit and auditable;
 the four semantic capabilities remain independently selectable for constrained
 builds.

### Missing Scenarios

**Question:** What happens when no migration route applies, when a compatible
patch version is skipped, when several bundles are needed, when the target is
not a Git repository, when the worktree is dirty, when a migration plan already
exists, when the generated maintenance change is later archived without a
product spec, or when a maintainer installs the release without an explicit
feature flag?

**Answer:** The catalog returns an empty path for a migration-free transition,
and Givn performs one managed commit without creating a Change. A route covers
later compatible patch releases; multiple edges become one ordered aggregate
plan. The command refuses before writing for non-Git or dirty projects, does
not overwrite an existing plan, and the archive scenario verifies that skipped
specs use the permanent feature path for global gates without a change-specific
semantic comparison. The Cargo default list enables every released optional
feature, while `--no-default-features` remains a documented feature-free path.

### Testability

**Question:** Can every new scenario fail for a concrete reason rather than
merely proving that the command ran?

**Answer:** Yes. Documentation scenarios assert required headings, terms, and
prompt order. Unit scenarios assert catalog validation, patch compatibility,
and multi-bundle ordering. The E2E scenarios assert exit status, version
marker, exact commit paths, commit count, migration files, recorded hash, and
absent product spec. Dirty and non-Git scenarios assert byte-level preservation
and no commit. Idempotency asserts the unchanged plan and commit history. The
Cargo scenario asserts the complete default feature set and the documented
feature-free escape hatch.

### Primary Risk

**Question:** What is the most likely implementation failure?

**Answer:** A catalog edge could be incomplete or the allowlist could
accidentally stage user content from a generated-file directory. A failure
between Commit A and Commit B could also leave an unclear state. Mitigation is
catalog validation, path-based route resolution, full clean-worktree preflight,
explicit known generated paths, no directory-wide staging, an atomic plan
write, a pre-write snapshot, and an output that names the successful commits.
Enabling all features by default also increases native build and model-artifact
cost; the explicit `--no-default-features` path prevents that from becoming an
undocumented requirement for constrained environments. The command never
resets or reverts user history.

### ADR Qualification

The candidate decision is the automatic two-commit upgrade protocol and its
maintenance-change boundary.

```json
{
  "qualification": "QUALIFIED",
  "alternatives": "PASS",
  "architectural_impact": "PASS",
  "durable_consequence": "PASS",
  "lower_level_artifact": "PASS",
  "existing_adr_check": "PASS",
  "must_be_shared": "SUPPORTING",
  "routing": "AMEND_ADR",
  "canonical_artifact": null,
  "target_adr": "ADR-023",
  "replacement_adr": null,
  "evidence": {
    "alternatives": [
      "manual patch without an automatic commit",
      "one automatic commit without a recorded migration plan",
      "two commits separating machine-safe preparation from the LLM plan"
    ],
    "architectural_impact": [
      "defines compatibility boundaries from project config versions to installed tool versions",
      "defines which generated paths Givn may stage and commit"
    ],
    "durable_consequence": [
      "changes project history and future upgrade behavior",
      "a later reversal requires coordinated catalog, migration, and commit semantics"
    ],
    "lower_level_artifact": [
      "the existing ADR-023 owns versioned-config upgrade semantics but does not own catalog-selected aggregate plans or migration-free managed commits"
    ],
    "existing_adr_check": [
      "ADR-023 was found as the existing versioned-config upgrade decision; this catalog and managed-commit change refines it rather than creating a duplicate ADR"
    ]
  }
}
```

`Must be shared` is supporting evidence only. The qualification is based on
all five mandatory dimensions. The decision is an amendment to ADR-023; no new
ADR is created.

The default Cargo feature set is a second qualified refinement of the existing
independent-capability decision in ADR-055. The features remain independent in
the source and runtime selectors; only the ordinary Cargo installation default
changes.

```json
{
  "qualification": "QUALIFIED",
  "alternatives": "PASS",
  "architectural_impact": "PASS",
  "durable_consequence": "PASS",
  "lower_level_artifact": "PASS",
  "existing_adr_check": "PASS",
  "must_be_shared": "SUPPORTING",
  "routing": "AMEND_ADR",
  "canonical_artifact": null,
  "target_adr": "ADR-055",
  "replacement_adr": null,
  "evidence": {
    "alternatives": [
      "keep all optional retrieval capabilities out of the normal install",
      "enable only retrieval and leave second-stage capabilities implicit",
      "enable every declared capability while preserving --no-default-features"
    ],
    "architectural_impact": [
      "defines the default dependency and runtime capability boundary of the binary",
      "keeps Cargo feature independence and explicit runtime selectors intact"
    ],
    "durable_consequence": [
      "changes build cost and default runtime availability for every source installation",
      "a later reversal requires a documented release decision and compatibility review"
    ],
    "lower_level_artifact": [
      "ADR-055 owns independent semantic capability selection and is the existing canonical decision"
    ],
    "existing_adr_check": [
      "ADR-055 was found as the existing Cargo/runtime feature-boundary decision; no duplicate ADR is needed"
    ]
  }
}
```

This feature decision is an amendment to ADR-055, not a new architecture
record. The minimal feature-free build remains an explicit supported escape
hatch for constrained environments.

### Arc42 Impact

The change affects chapters 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, and 12:

| Chapter | Affected? | Reason |
|---|---|---|
| 01 introduction-and-goals | Yes | 0.3.0 adds upgradeability, migration, and the complete default Cargo capability set. |
| 02 architecture-constraints | Yes | Managed upgrades and catalog routes require a Git worktree boundary and explicit ownership rules; installation defaults change. |
| 03 context-and-scope | Yes | `givn upgrade` gains managed-only and aggregate migration interactions. |
| 04 solution-strategy | Yes | The release separates migration-free managed commits from ordered LLM-reviewed migration phases and defines the default/minimal feature paths. |
| 05 building-block-view | Yes | Catalog loading, path resolution, and upgrade orchestration gain migration responsibilities. |
| 06 runtime-view | Yes | Managed-only and aggregate upgrade sequences are new runtime flows. |
| 07 deployment-view | Yes | The binary carries migration assets, resolves version gaps through the Git CLI, and enables all declared features by default. |
| 08 crosscutting-concepts | Yes | Config, generated guidance, catalog ownership, aggregate ordering, and migration contracts change. |
| 09 architecture-decisions | Yes | ADR-023 is amended with the automatic migration protocol and ADR-055 with the default feature set. |
| 10 quality-requirements | Yes | New release, managed-upgrade, version-skipping, migration safety, and default-install scenarios are added. |
| 11 risks-and-technical-debt | Yes | Commit failure, clean-worktree, project-specific migration, and default-build cost risks change. |
| 12 glossary | Yes | Release migration, maintenance change, and managed upgrade commit become domain terms. |

The durable chapter updates and both ADR amendments are part of the
implementation tasks. They must be checked against the completed implementation
before `REVIEW: PASS`.

## Hardening Applied

- Added explicit non-Git and existing-plan scenarios to the feature.
- Required a clean worktree before any write or commit.
- Limited automatic staging to a calculated Givn-owned allowlist.
- Separated the machine-safe Commit A from the migration-plan Commit B because
  a commit cannot contain its own hash.
- Kept product specs, project source, ejected facets, Arc42 facts, and ADR
  decisions outside the automatic file operation.
- Required the generated maintenance change to skip `specs` and to contain no
  fabricated design-review, task, implementation, or review evidence.
- Added the archive contract for a no-product-delta maintenance change: global
  verification remains active, while delta merge and the change-specific
  semantic gate are skipped.
- Re-ran the interaction coverage review for the new archive action and added
  its executable E2E scenario and separate permanent-spec assertion.
- Added a declarative migration catalog, multi-bundle path resolver, aggregate
  `migration.yaml`, and migration-free managed-commit path.
- Added unit coverage for catalog validation, compatible patch skipping, and
  ordered multi-bundle resolution.
- Added a release scenario that asserts every declared feature is in the normal
  Cargo default set and that the feature-free build remains explicit.
- Removed obsolete feature-combination helpers exposed by the all-default build
  and made the full default-feature lint configuration warning-free.

## Review Result

The design is complete, the missing boundaries are represented by executable
scenarios or focused catalog tests, and the architectural decisions are routed
to existing ADR-023 and ADR-055 for amendment. Migration-free upgrades, skipped
compatible patches, aggregate bundle ordering, the no-product-delta archive
flow, and the default/minimal Cargo paths are covered without weakening
product-change gates.

DESIGN-REVIEW: IN PROGRESS
