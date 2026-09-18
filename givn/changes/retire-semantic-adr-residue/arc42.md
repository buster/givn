# arc42 Documentation Update: retire-semantic-adr-residue

## Impact assessment

| # | Chapter | Affected? | Reason | Summary of change (if Yes) |
|---|---|---|---|---|
| 1 | 01 introduction-and-goals | No | No goal or stakeholder change. | |
| 2 | 02 architecture-constraints | No | No constraint change; the register contract already excludes superseded records. | |
| 3 | 03 context-and-scope | No | No interface or external-entity change. | |
| 4 | 04 solution-strategy | No | No strategy change. | |
| 5 | 05 building-block-view | No | No module change. | |
| 6 | 06 runtime-view | No | No runtime change. | |
| 7 | 07 deployment-view | No | No packaging change. | |
| 8 | 08 crosscutting-concepts | No | No concept change. | |
| 9 | 09 architecture-decisions | Yes | The register is chapter 9's content; three obsolete records leave the active register. | ADR-044, ADR-045, and ADR-054 move to `archive/` as superseded by ADR-069; ADR-069's supersession list and the register README are updated (active count 25). |
| 10 | 10 quality-requirements | No | No quality scenario change. | |
| 11 | 11 risks-and-technical-debt | No | No risk register entry names the three records. | |
| 12 | 12 glossary | No | No term change. | |

## Notes

The three records did not fail the qualification gate; they describe
mechanisms ADR-069 removed. Their end state matches the sixteen records
ADR-069 already superseded: historical context in `archive/`, never a
failing record retained or deleted.

---

## Status

STATUS: DONE
