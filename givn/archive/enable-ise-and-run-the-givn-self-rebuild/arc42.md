# Arc42 Documentation Update: enable-ise-and-run-the-givn-self-rebuild

## Impact assessment

| # | Chapter | Affected? | Reason | Summary of change (if Yes) |
|---|---|---|---|---|
| 1 | 01 introduction-and-goals | No | The change records a project add-on setting and one trial's evidence; it adds no goal, stakeholder, or requirement. | |
| 2 | 02 architecture-constraints | Yes | The add-on constraint now records that Givn itself is `ise`-effective and what its rebuild boundary removes. | Section 2.3 names `givn/specification.yaml`, the replaceable `src/**` and `tests/**` roots, and the frozen `e2e` package and declaration. |
| 3 | 03 context-and-scope | No | The declaration, the clean-room commands, and the E2E runner already cross the boundary; no new external interface appears. | |
| 4 | 04 solution-strategy | Yes | The strategy table is the home of the add-on strategy and the self-rebuild decision. | A `Self-rebuild` row records the `Dry-Run Ise`, the rebuild boundary, the `Spec gap` triage, and the no-promotion rule. |
| 5 | 05 building-block-view | No | No module, component, or responsibility changes. The trial exercises the existing `ise`, `e2e_boundary`, and `config::declaration` components. | |
| 6 | 06 runtime-view | No | The clean-room flow and the change verification flow are already documented and are executed unchanged. | |
| 7 | 07 deployment-view | No | No packaging, container, runtime, or topology change; the product stays one binary. | |
| 8 | 08 crosscutting-concepts | No | The add-on, declaration, receipt, and clean-room concepts already exist; the enablement is configuration, not a new concept. | |
| 9 | 09 architecture-decisions | No | No architectural commitment is chosen. The enablement is project configuration and the trial is evidence; ADR-010, ADR-076, and ADR-078 already own the relevant boundaries. | |
| 10 | 10 quality-requirements | No | No new quality requirement or scenario. The trial produces evidence for `S14`, not a durable quality gate. | |
| 11 | 11 risks-and-technical-debt | Yes | The trial exposes surfaces the declared Specification does not cover, and a rebuilt binary can differ for that reason. | `R-163` records the undeclared `assets/**`, version, repository-file, and test-fixture surfaces and the triage that contains them. |
| 12 | 12 glossary | No | No new domain term. `Specification`, `Spec copy`, `Spec gap`, `change strategy`, and `Ise Test` are already defined. Chapter 12 stays a link to `docs/glossary.md`. | |

## ADR qualification

```text
Architectural commitment: none. The change records a project's add-on setting and one trial's evidence; it fixes no component boundary, dependency direction, durable contract, or compatibility boundary.
Real alternatives: the trial scope (whole product versus one component) and the trial mode (Dry-Run Ise versus Full replacement) are real, but both are bounded by the ideation plan's S13 and S14 split and reverse with one configuration and one evidence run.
Durable consequence: none beyond evidence. Reversal means the next trial reads a different revision; no migration, interface, or cross-command coordination follows.
Existing-decision search: ADR-010 owns the configuration authority and the declaration, ADR-076 owns the clean-room manifest, and ADR-078 owns the frozen package outside the trial. The self-enablement is project configuration, not an architecture decision.
ADR action: none. Canonical home: docs/arc42/04-solution-strategy.md (the self-rebuild row) and docs/arc42/11-risks-and-technical-debt.md (R-163).
```

## Notes

The three affected chapters are updated in this change with the same vocabulary.
`docs/glossary.md` and `docs/domain-model.md` stay unchanged: the change adds no
term and no structural relationship. The trial's spec copy, manifest, and
rebuilt binary are ephemeral evidence outside the checkout; the architecture
documentation records only the durable enablement and its rebuild boundary.

STATUS: DONE
