# arc42 Documentation Update: ise-rebuild-decision-and-working-tree-ship

## Impact assessment

| # | Chapter | Affected? | Reason | Summary of change (if Yes) |
|---|---|---|---|---|
| 1 | 01 introduction-and-goals | No | No top-level goal, quality goal, or stakeholder changes; the guidance sits below the goals. | |
| 2 | 02 architecture-constraints | Yes | Constraint rows name the decision record, Specification category, manifest entry, spec-copy manifest, and project working tree. | TC-45, TC-46, TC-47, TC-49, TC-51, and TC-52 use the settled vocabulary; the whole-system source scope is named. |
| 3 | 03 context-and-scope | Yes | The Specification input row, the `ise` command rows, and the verifier row use the renamed objects and the project working tree. | The `givn/specification.yaml` row names categories and entries; the command rows name the spec-copy manifest and project working tree. |
| 4 | 04 solution-strategy | Yes | The copy strategy and self-rebuild rows name the Specification Manifest and the discard practice. | The copy row names the spec-copy manifest; the self-rebuild row names a discard rebuild and the files the rebuild needs. |
| 5 | 05 building-block-view | Yes | `ise`, `cli::archive`, `cli::addons`, `config::declaration`, and `e2e_boundary` rows use category and manifest-entry wording. | The five building-block descriptions use the settled nouns; no block boundary moves. |
| 6 | 06 runtime-view | Yes | The lint, task, archive, addon, upgrade, and ise partition/compare descriptions use the renamed objects. | The ise partition and comparison steps and their reports use category, manifest entry, spec-copy manifest, and project working tree. |
| 7 | 07 deployment-view | No | No deployment topology, process, or environment change. | |
| 8 | 08 crosscutting-concepts | Yes | The configuration authority, the spec-copy manifest section, the four copy rule sentences, and the comparison contract change. | The copy rule and comparison descriptions use the settled vocabulary; the contract shape is unchanged. |
| 9 | 09 architecture-decisions | Yes | ADR-076 is amended to scope the clean-room trial record to the copy-based Ise Test. | ADR-076 gains an amendment; the register and ADR index record it. |
| 10 | 10 quality-requirements | No | No new quality scenario or measurable requirement. | |
| 11 | 11 risks-and-technical-debt | Yes | Risk rows name the spec-copy manifest, manifest entry, and Whole-system rebuild; the working-tree limitation is a new risk. | R-142, R-143, and R-150 use the settled vocabulary; R-165 records that a working-tree green verification proves tested behaviour, not Specification sufficiency. |
| 12 | 12 glossary | Yes | The glossary is the vocabulary authority and gains the settled terms. | Adds Ise rebuild decision, No rebuild, Single-Component Rebuild, Whole-system rebuild, Rebuilt code values, Component, Component Interface, System, Project working tree, Selected revision, Specification Manifest, Specification category, manifest entry, matched file, Specification file in the spec copy, Givn configuration, and spec-copy manifest. |

## Notes

The vocabulary change stays inside the existing ise architecture. It adds no building block, external interface, or ADR.

```text
Architectural commitment: none
Real alternatives: none for the reworded guidance; the Ise Test scoping rejected reserving the term for the copy-based path (ADR-076 amendment)
Durable consequence: the settled nouns bind future served guidance and the site
Existing-decision search: ADR-010 (configuration authority), ADR-076 (clean-room trial record), ADR-078, ADR-079; ADR-076 is amended to scope its record to the copy-based Ise Test
ADR action: AMEND_ADR
ADR target: ADR-076-clean-room-trial-record.md
```

---

## Status

STATUS: DONE
