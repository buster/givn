# arc42 Documentation Update: make-ise-guidance-self-contained

## Impact assessment

| # | Chapter | Affected? | Reason | Summary of change (if Yes) |
|---|---|---|---|---|
| 1 | 01 introduction-and-goals | No | No goal, quality goal, or stakeholder changes. | |
| 2 | 02 architecture-constraints | No | No constraint changes; the served instruction text only. | |
| 3 | 03 context-and-scope | No | No interface, actor, or external-system change. | |
| 4 | 04 solution-strategy | No | The strategy is unchanged; the instruction defines its terms. | |
| 5 | 05 building-block-view | No | No component, module, or dependency change. | |
| 6 | 06 runtime-view | No | No runtime flow, command, or gate change. | |
| 7 | 07 deployment-view | No | No deployment topology or environment change. | |
| 8 | 08 crosscutting-concepts | No | The crosscutting vocabulary is unchanged; the instruction now states the terms it uses. | |
| 9 | 09 architecture-decisions | No | No new or amended ADR; the change has no architectural commitment. | |
| 10 | 10 quality-requirements | No | No new quality scenario or measurable requirement. | |
| 11 | 11 risks-and-technical-debt | No | The duplicate definitions are bounded in the design, not a new architecture risk. | |
| 12 | 12 glossary | No | The glossary is unchanged; the inline definitions are instruction-local. | |

## Notes

The change corrects served instruction vocabulary and adds two regression scenarios. It does not change a building block, external interface, or architecture decision.

```text
Architectural commitment: none
Real alternatives: none for the wording correction
Durable consequence: the served ise instruction no longer points at a repository-only file
Existing-decision search: ADR-010 (configuration authority), ADR-076 (clean-room trial record); neither changes
ADR action: NONE
ADR target: none
```

---

## Status

STATUS: DONE
