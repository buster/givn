# Arc42 Documentation Update: template-engine-conditional-rendering

## Impact assessment

| # | Chapter | Affected? | Reason | Summary of change |
|---|---|---|---|---|
| 1 | 01 introduction-and-goals | No | No goal change; generated guidance becomes configuration-aware but the workflow contract is unchanged. | None. |
| 2 | 02 architecture-constraints | No | No build, deployment, or dependency constraint changes. | None. |
| 3 | 03 context-and-scope | No | No new external surface; skills and commands remain agent-facing guidance. | None. |
| 4 | 04 solution-strategy | No | Existing plain-string rendering gains conditional-block resolution; no architectural strategy change. | None. |
| 5 | 05 building-block-view | No | No module or component structure change; the renderer, sync, and addons modules are augmented, not restructured. | None. |
| 6 | 06 runtime-view | No | Generation reads the effective addon set; no new runtime path or daemon. | None. |
| 7 | 07 deployment-view | No | No deployment, distribution, or runtime environment change. | None. |
| 8 | 08 crosscutting-concepts | Yes | The glossary terms for addon-aware generation, conditional sections, generated skills, slash commands, and templates are added. | 12-glossary.md: added five rows. |
| 9 | 09 architecture-decisions | No | No qualified architectural decision introduced; the template-engine choice is a design decision, not an ADR. | None. |
| 10 | 10 quality-requirements | No | No new quality scenario beyond acceptance coverage in the change's own feature file. | None. |
| 11 | 11 risks-and-technical-debt | No | Template drift and stale-file risks are mitigated at design time without new architectural debt. | None. |
| 12 | 12 glossary | Yes | Added rows: Addon-aware generation, Conditional section, Generated skill, Slash command, Template. See row 8. | 12-glossary.md updated. |

## Notes

Generation becomes addon-aware: skill and command templates carry conditional
sections resolved against the effective addon set, and every `givn addons`
toggle regenerates the affected files. No new dependency; a minimal
conditional directive engine is added to the existing renderer.

## Status

STATUS: DONE
