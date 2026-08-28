# Arc42 Documentation Update: fix-domain-modeling-skill-trigger

## Impact assessment

| # | Chapter | Affected? | Reason | Summary of change |
|---|---|---|---|---|
| 1 | 01 introduction-and-goals | No | No requirement change; the change repairs authoring-guidance wording and session routing only. | None. |
| 2 | 02 architecture-constraints | No | No constraint (build, deployment, dependency) changes. | None. |
| 3 | 03 context-and-scope | No | No new external surface; the skill tool text and `givn instructions` output are internal agent-facing guidance. | None. |
| 4 | 04 solution-strategy | No | No strategy change; the change alters guidance content, not architecture approach. | None. |
| 5 | 05 building-block-view | No | No module or component structure changes; one skill asset replaced, one CLI output enriched. | None. |
| 6 | 06 runtime-view | No | No new runtime flow; the router emits one extra informational line via existing output paths. | None. |
| 7 | 07 deployment-view | No | No deployment, distribution, or runtime environment change. | None. |
| 8 | 08 crosscutting-concepts | No | Authoring-guidance content is not a cross-cutting architecture concept. | None. |
| 9 | 09 architecture-decisions | No | No qualified architectural decision; the change is a refinement of existing guidance text. | None. |
| 10 | 10 quality-requirements | No | No new quality scenario; acceptance coverage is confined to the change's own feature file. | None. |
| 11 | 11 risks-and-technical-debt | No | No new architectural risk; the deferred enforcement layer is a scoped-out decision recorded in the proposal, not debt introduced here. | None. |
| 12 | 12 glossary | No | The anti-term list and ubiquitous-language table concern the example change's domain (shopping list), not givn's own architecture vocabulary. | None. |

## Notes

Pure tooling change — no architectural impact. The change rewrites the
`givn-domain-modeling` skill guidance so its trigger matches conversation
states instead of a phase label, and makes `givn instructions` route the agent
to that skill at the specs and design phases when the addon is enabled. No
artifact graph, manifest schema, lifecycle order, or contract changes.

## Status

STATUS: DONE
