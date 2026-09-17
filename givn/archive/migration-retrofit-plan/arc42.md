# arc42 Documentation Update: migration-retrofit-plan

## Impact assessment

| # | Chapter | Affected? | Reason | Summary of change (if Yes) |
|---|---|---|---|---|
| 1 | 01 introduction-and-goals | No | No product goal, stakeholder, or quality goal changes. | |
| 2 | 02 architecture-constraints | Yes | The migration-ownership constraint sentence now says project-owned artifacts change through LLM-guided project changes, not only the maintenance change. | Constraint sentence reworded. |
| 3 | 03 context-and-scope | No | No interface or external entity changes. | |
| 4 | 04 solution-strategy | No | The release migration strategy (catalog bundle + LLM maintenance change) is unchanged. | |
| 5 | 05 building-block-view | No | No module or responsibility change; bundle assets are data. | |
| 6 | 06 runtime-view | Yes | 6.13 now says the maintenance plan archives once the inventory is complete and every deviation names a follow-up change. | One sentence in 6.13. |
| 7 | 07 deployment-view | Yes | The bundle-delivery sentence now says phases are inventoried in one aggregate change and the repairs run as follow-ups. | One sentence in chapter 7. |
| 8 | 08 crosscutting-concepts | No | The migration catalog is documented in 8.10; amending bundle prose is data. | |
| 9 | 09 architecture-decisions | No | No decision boundary; the change amends consumer guidance. | |
| 10 | 10 quality-requirements | No | No new quality scenario. | |
| 11 | 11 risks-and-technical-debt | No | No new risk; the amendment reduces the unreviewed-retrofit risk the shipped bundle carried. | |
| 12 | 12 glossary | No | No new term. | |

## Notes

The amended bundle's inventory-and-plan model matches how the eight 0.8.0
changes were themselves executed (each capability and rule landed as its own
reviewed change), so no durable chapter text needs to change.

---

## Status

STATUS: DONE
