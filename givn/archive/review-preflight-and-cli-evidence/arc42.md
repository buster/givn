# Arc42 assessment: review-preflight-and-cli-evidence

| # | Chapter | Affected | Reason / change |
|---|---|---|---|
| 1 | 01 introduction-and-goals | No | The quality scenario lives in chapter 10. |
| 2 | 02 architecture-constraints | Yes | TC-40 records the unique, real, scenario-named revision rule. |
| 3 | 03 context-and-scope | No | No external system or rendered surface. |
| 4 | 04 solution-strategy | Yes | Scenario commits are the traceability evidence; no shared mode. |
| 5 | 05 building-block-view | Yes | `cli::check` and the SCM seam own the commit inspection. |
| 6 | 06 runtime-view | Yes | Check tasks/review run the scenario-commit gate. |
| 7 | 07 deployment-view | No | No deployment change. |
| 8 | 08 crosscutting-concepts | Yes | Revision evidence and CLI transcripts are cross-cutting workflow rules. |
| 9 | 09 architecture-decisions | No | Workflow policy owned by the use cases and `design.md`; ADR-070/072 unaffected. |
| 10 | 10 quality-requirements | Yes | Q-106 adds the scenario-commit and not-applicable scenario. |
| 11 | 11 risks-and-technical-debt | Yes | R-102 records the synthetic or shared-commit evidence this rule removes. |
| 12 | 12 glossary | Yes | Adds "Scenario revision" and "Interaction transcript". |

## Status

STATUS: DONE
