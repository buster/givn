# arc42 Documentation Update: rewrite-communication-policy

## Impact assessment

| # | Chapter | Affected? | Reason | Summary of change (if Yes) |
|---|---|---|---|---|
| 1 | 01 introduction-and-goals | No | Product goals and scope statement are unchanged. | |
| 2 | 02 architecture-constraints | Yes | The addon-scope constraint names what `communication_policy` controls. | Constraint now covers presentation and documentation, and states the block names formats rather than repository layout. |
| 3 | 03 context-and-scope | Yes | The chapter describes the managed fenced block written into `AGENTS.md`. | Block description covers documentation rules. |
| 4 | 04 solution-strategy | No | No strategy, technology, or structure choice changes. | |
| 5 | 05 building-block-view | No | The same asset and renderer implement the block; no component is added, split, or moved. | |
| 6 | 06 runtime-view | Yes | The initialization/sync sequence names which block the addon writes. | Step 7 wording covers presentation and documentation. |
| 7 | 07 deployment-view | No | No deployment topology, artifact, or runtime dependency changes. | |
| 8 | 08 crosscutting-concepts | Yes | The guidance-ownership section states which rules the addon owns and which stay in served instructions. | Addon scope extended to documentation; dialog ownership unchanged. |
| 9 | 09 architecture-decisions | No | No lasting component, ownership, dependency, or compatibility boundary is fixed; D1 is a content choice inside an existing owned asset. | |
| 10 | 10 quality-requirements | No | Size and content are bounded by the block itself; no new quality scenario or metric is introduced. | |
| 11 | 11 risks-and-technical-debt | No | The change removes the earlier draft's risk (repository-specific budgets) instead of adding new debt. | |
| 12 | 12 glossary | Yes | The "Communication policy" term defines the addon's scope. | Definition names presentation and documentation, and keeps the dialog anti-term. |

## Notes

```text
Architectural commitment: none — the block is content inside an existing asset and its existing renderer
Real alternatives: keep the draft's per-document budgets versus name formats only; both fit the same component
Durable consequence: low — reverting is one asset text edit plus the matching assertion
Existing-decision search: ADR-071..ADR-075 and the archive checked; no ADR owns generated guidance scope
ADR action: NONE
ADR target: none
```

## Status

STATUS: DONE
