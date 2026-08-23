# Arc42 Assessment: release-0-3-0

## Chapter Impact

| Chapter | Affected? | Evidence / change |
|---|---|---|
| 01 introduction-and-goals | Yes | Adds release, migration, default-feature, and migrating-maintainer requirements. |
| 02 architecture-constraints | Yes | Adds the clean-Git and Givn-owned-path boundary for managed upgrades and declared migrations, plus the default/minimal Cargo paths. |
| 03 context-and-scope | Yes | Adds the upgraded `givn upgrade` interaction and embedded migration catalog scope. |
| 04 solution-strategy | Yes | Separates migration-free managed upgrades from catalog-selected aggregate migrations and defines default/minimal feature behavior. |
| 05 building-block-view | Yes | Adds catalog loading, path resolution, and aggregate migration orchestration. |
| 06 runtime-view | Yes | Documents managed commit behavior, ordered bundle aggregation, Commit B, and no-product-delta archive sequence. |
| 07 deployment-view | Yes | Documents binary-version gaps, catalog bundles, Git requirement, and all-feature default installation. |
| 08 crosscutting-concepts | Yes | Defines catalog ownership, idempotency, managed-only upgrades, aggregate phases, and two-commit semantics. |
| 09 architecture-decisions | Yes | Records the ADR-023 upgrade-protocol amendment and ADR-055 default-feature amendment. |
| 10 quality-requirements | Yes | Adds release, safety, migration-guidance, maintenance-archive, managed-upgrade, version-skipping, and default-install scenarios Q-48 through Q-56. |
| 11 risks-and-technical-debt | Yes | Adds risks for generated-path ownership, split commits, marker interpretation, catalog integrity, aggregate ordering, and default-build cost. |
| 12 glossary | Yes | Adds managed upgrade, migration catalog, bundle, aggregate plan, and commit terminology. |

## ADR Disposition

The managed-upgrade and aggregate migration protocol is a qualified
architectural refinement of ADR-023. The default Cargo feature set is a
qualified refinement of ADR-055. Both are routed as `AMEND_ADR`; no new ADR is
created.

## Status

STATUS: DONE
