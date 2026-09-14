# arc42 Documentation Update: inspect-change-state

## Impact assessment

| # | Chapter | Affected? | Reason | Summary of change (if Yes) |
|---|---|---|---|---|
| 1 | 01 introduction-and-goals | No | No goal change. | — |
| 2 | 02 architecture-constraints | No | No constraint change; the seam already governs selection. | — |
| 3 | 03 context-and-scope | No | No external-interface change. | — |
| 4 | 04 solution-strategy | No | No strategy change. | — |
| 5 | 05 building-block-view | No | `cli::status`'s responsibility row already covers rendered state; the backend line is part of it. | — |
| 6 | 06 runtime-view | Yes | The status output gains the backend line. | 6.5 records the SCM line. |
| 7 | 07 deployment-view | No | No deployment change. | — |
| 8 | 08 crosscutting-concepts | No | Selection is already documented (8.18). | — |
| 9 | 09 architecture-decisions | No | No decision. | — |
| 10 | 10 quality-requirements | No | No quality scenario change. | — |
| 11 | 11 risks-and-technical-debt | No | A19 records the none-fallback as an assumption, not debt. | — |
| 12 | 12 glossary | No | SCM backend is already defined. | — |

```text
ADR action: none
ADR target: null
```

## Status

STATUS: DONE
