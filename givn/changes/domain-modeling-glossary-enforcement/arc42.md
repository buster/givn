# Arc42 Documentation Update: domain-modeling-glossary-enforcement

## Impact assessment

| # | Chapter | Affected? | Reason | Summary of change |
|---|---|---|---|---|
| 1 | 01 introduction-and-goals | No | No goal change; the change refines how the domain-modeling skill maintains its vocabulary and where it is routed. | None. |
| 2 | 02 architecture-constraints | No | No build, deployment, or dependency constraint changes. | None. |
| 3 | 03 context-and-scope | No | No new external surface; skill guidance text and `givn instructions` output remain agent-facing guidance. | None. |
| 4 | 04 solution-strategy | No | No strategy change; the change alters guidance content and one routing predicate, not the architecture approach. | None. |
| 5 | 05 building-block-view | No | No module or component structure change; three skill assets edited, one router predicate widened. | None. |
| 6 | 06 runtime-view | No | The router emits the same informational line via existing output paths, now across more phases. | None. |
| 7 | 07 deployment-view | No | No deployment, distribution, or runtime environment change. | None. |
| 8 | 08 crosscutting-concepts | No | Authoring-guidance content is not a cross-cutting architecture concept. | None. |
| 9 | 09 architecture-decisions | No | No qualified architectural decision; the change implements the settled coupling decision (domain modeling requires arc42). | None. |
| 10 | 10 quality-requirements | No | No new quality scenario; acceptance coverage is confined to the change's own feature file. | None. |
| 11 | 11 risks-and-technical-debt | No | No new architectural risk; the rejected term-scanning enforcement is a scoped-out decision recorded in the proposal, not debt introduced here. | None. |
| 12 | 12 glossary | No | The glossary chapter becomes the write-target of the domain-modeling skill for future changes, but this change does not restructure the chapter or add givn architecture terms. | None. |

## Notes

Pure guidance and routing change — no architectural impact. The change
retargets the `givn-domain-modeling` skill from a per-change `domain.md` to
the project's durable arc42 glossary, widens its trigger and the `givn
instructions` routing to every workflow step (requiring the `arc42` addon),
and adds a human ubiquitous-language check to the design-review and review
skills. No artifact graph, manifest schema, lifecycle order, or contract
changes. No term-matching scanner is introduced; vocabulary is validated by
people.

## Status

STATUS: DONE
