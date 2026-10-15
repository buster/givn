# arc42 Documentation Update: drop-legacy-residue

> This is the change-level arc42 **impact assessment template**. It is not an
> ADR template and must not be used to draft, store, or combine architecture
> decisions. An architecturally significant decision gets exactly one separate
> MADR file at `docs/arc42/adr/ADR-NNNN-short-slug.md`; chapter 09 is only its
> register/index. Use the ADR guidance and the Qualification gate before
> creating that file.

## Impact assessment

| # | Chapter | Affected? | Reason | Summary of change (if Yes) |
|---|---|---|---|---|
| 1 | 01 introduction-and-goals | Yes | FR-33 and the migrating-maintainer stakeholder row pinned 0.3.0-era release work, and the guide they named is deleted. | FR-33 now requires each release's changelog and its release-owned migration routes; the migrating-maintainer need is stated generically. |
| 2 | 02 architecture-constraints | Yes | Section 2.7 named the 0.2.0-to-0.3.0 route as the automatic migration scope. | The migration-ownership constraint now says "the automatic migration route" without a version pin. |
| 3 | 03 context-and-scope | Yes | The upgrade interface row and the in/out scope lists named 0.2.0/0.3.0. | The upgrade row describes a declared route plus the below-route managed refresh; in scope says the oldest declared source is 0.7.0; out of scope excludes migrations below that route. |
| 4 | 04 solution-strategy | Yes | The release-migration and upgradeability rows named a clean 0.2.0 project and the exact 0.2.0-to-0.3.0 route. | Strategy rows name a declared route (currently from 0.7.0) and the below-route refresh. |
| 5 | 05 building-block-view | Yes | The `cli::rewrite_history` module row described a removed component; the attribution-stamper description named legacy trailer stripping. | The module row is deleted; the stamper description keeps only the managed co-author line. |
| 6 | 06 runtime-view | Yes | The init sequence described enabling "other registered extras". | Step 6 names the current addon registry: Arc42, Absolute Mode, README, the coverage gate, and the four project-level skills/commands. |
| 7 | 07 deployment-view | Yes | Section 7.6 pinned the 0.2.0-to-0.3.0 route and Git-only commits. | 7.6 names a declared route, two revisions on the selected backend, and the managed refresh for versions below 0.7.0. |
| 8 | 08 crosscutting-concepts | Yes | The rules-policy paragraph named `givn rewrite-history` as the only remediation; 8.9 described the removed in-tool union merge; 8.12a described legacy attribution-trailer stripping. | Rule cleanliness is report-only (purging is external SCM work); the merge arithmetic belongs to the project-owned merge command (this repository's script unions); the stamper replaces only the managed co-author line. |
| 9 | 09 architecture-decisions | Yes | The chapter-9 register's release-migration evidence row linked the deleted 0.2.0-to-0.3.0 guide. | The evidence row links `docs/migrations/README.md`; the active register count and relationships are unchanged (verdict below). |
| 10 | 10 quality-requirements | Yes | Q-48, Q-55, and Q-77 pinned the 0.2.0/0.3.0 routes and the 0.5.0-to-0.6.0 bundle. | The scenarios now pin 0.7.0 as the oldest source, the 0.8.0-to-0.9.0 version-skip route, and the current bundle's contract. |
| 11 | 11 risks-and-technical-debt | Yes | R-29 named the 0.3.0 marker, and the change's configuration and no-purge consequences needed records. | R-29 is restated around the version marker; R-81 records the strict legacy-coverage refusal; R-82 records the report-only rules boundary; section 11.4 records the retirement and the 0.7.0 floor; debt D-3 is repaid by the scaffold correction and removed from the debt table, and R-7 names the refreshed template. |
| 12 | 12 glossary | Yes | The glossary defined `History rewrite` as a current capability and `Clean history` named it as the only remediation. | The `History rewrite` row is deleted; `Clean history` states that Givn never rewrites history. |

## Notes

The change removes code, catalog entries, guides, and stale documentation
claims. It moves no component, authority, dependency, or deployment boundary:
the SCM seam, the archive proof pipeline, the upgrade orchestration, and the
coverage gate stay, minus the legacy branches. The retired capability is
recorded by the tombstone and the review disclosure; its observable successor
behavior (report-only cleanliness) is owned by the `rules-policy` and
`publication-cleanliness` fragment specs. The spec-text corrections (the
archive and directory-resolution Given steps, plus the served guidance,
naming `givn/commands.yaml` for the runner commands) change no chapter:
chapters 2, 4, 6, 8, and the glossary already attribute those values to
`givn/commands.yaml`, so no chapter was edited for them. The reference and
configuration-scaffold corrections likewise need no content chapter edit —
the chapters already state the correct contract — but they repay debt D-3 and
refine R-7 in chapter 11, recorded there.

This impact marker does not replace the ADR Qualification Checklist or the
individual ADR file. Record the result once in the change assessment:

```text
ADR action: CANONICAL_ARTIFACT
ADR target: docs/arc42/08-crosscutting-concepts.md section 8.7 (rules policy)

qualification: NOT_QUALIFIED
alternatives: PASS
architectural_impact: FAIL
durable_consequence: FAIL
lower_level_artifact: FAIL
existing_adr_check: PASS
must_be_shared: NOT_SUPPORTING
routing: CANONICAL_ARTIFACT
canonical_artifact: docs/arc42/08-crosscutting-concepts.md
target_adr: none
replacement_adr: none
```

Alternatives were real: retire the rewrite against keeping it with clearer
guidance; re-point the pinned guide scenario against removing it (D6);
refuse a stale `extras` key against ignoring it (D1); close the legacy
coverage window against keeping the fields (D2). No component, authority,
dependency, or deployment boundary moves — a subcommand, a dead manifest
field, two legacy config fields, and five embedded bundles leave — so
`architectural_impact` fails. Reversal is one change's code and text sweep, so
`durable_consequence` fails. The complete rationale for report-only
cleanliness is cross-cutting documentation owned by 8.7 and the fragment
specs, and the route floor is release policy owned by the catalog and the
use-case specs, so `lower_level_artifact` fails. `existing_adr_check` searched
the register and archive: ADR-023 owns the versioned-upgrade boundary and
ADR-071 the SCM seam, both unchanged by this removal; no record covers in-tool
history rewriting or the catalog's supported floor, so no amendment or
supersession applies and the candidate routes to its canonical artifacts. The
same verdict covers the catalog-floor and configuration-key candidates.

Falsification tests applied after the mandatory dimensions: single-change
reversibility holds — the removal reverses within one change — which by itself
fails the gate; no-multi-change constraint does not hold, because the decision
constrains no two future changes; product-behavior exclusion does not hold,
because report-only cleanliness is use-case-ownable workflow behavior.
Two mandatory dimensions already FAIL.

---

## Status

STATUS: DONE
