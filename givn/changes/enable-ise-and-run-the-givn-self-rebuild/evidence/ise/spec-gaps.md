# Spec gaps: enable-ise-and-run-the-givn-self-rebuild

Trial mode: `Dry-Run Ise`, whole-product rebuild. The trial revision is named
in `trial-revision.txt`. A setup failure is not a `Spec gap`; each difference
between the frozen `@e2e` result and the rebuilt binary receives exactly one
outcome under `## Spec gaps`.

## Setup failures

| Date | Setup failure | Evidence | Disposition |
|---|---|---|---|
| 2026-11-17 | `S12` evidence absent. `givn/ideation/ise-rebuildability/plan.md` section 0 records `S10` to `S12`, the Passuff arm, as outstanding, and the phase table's `S12` row has no recorded evidence. The Passuff product is committed on `ise/trial` per the operator, so the plan's "uncommitted work" premise is stale, but no `S10` to `S12` evidence is archived. | `plan.md` section 0 (`**Outstanding.**`: "`S10` to `S12`, the Passuff arm. Blocked: the product exists only as uncommitted work in another workspace") and the `S12` phase row ("Full-product `Dry-Run Ise` from `givn/specs` plus `docs`"). | Operator override recorded in `proposal.md` `## Operator Override: S12 Prerequisite` (S13 execution instruction, 2026-11-17). The trial proceeds with the row outstanding. `S12` is not satisfied and this trial is not `S12` evidence. |

## Spec gaps

Not yet triaged; the frozen End-to-end result is written after the rebuild.
