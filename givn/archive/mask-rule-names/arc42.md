# arc42 Documentation Update: mask-rule-names

> Change-level arc42 impact assessment. Not an ADR template.

## Impact assessment

| # | Chapter | Affected? | Reason | Summary of change (if Yes) |
|---|---|---|---|---|
| 1 | 01 introduction-and-goals | No | No goal or stakeholder change. | |
| 2 | 02 architecture-constraints | No | No constraint change. | |
| 3 | 03 context-and-scope | No | No interface change. | |
| 4 | 04 solution-strategy | No | No strategy change. | |
| 5 | 05 building-block-view | No | The `skills::rules` row remains accurate; no new block. | |
| 6 | 06 runtime-view | No | No flow change. | |
| 7 | 07 deployment-view | No | No deployment change. | |
| 8 | 08 crosscutting-concepts | Yes | 8.7 describes the ignore block. | 8.7 now names the prefix-pattern form and the predicate-based discovery. |
| 9 | 09 architecture-decisions | No | No qualified ADR; the pattern set is a local implementation choice inside an existing policy. | |
| 10 | 10 quality-requirements | No | No new quality scenario. | |
| 11 | 11 risks-and-technical-debt | No | No new risk; the reserved namespace is documented. | |
| 12 | 12 glossary | No | No new term; "Ignore block" already covers it. | |

## Notes

The choice between per-file enumeration and reserved-namespace patterns is a
real alternative (hence `alternatives: PASS`); it does not qualify for an ADR
because it is local implementation detail inside the existing rules policy.

```text
ADR action: CANONICAL_ARTIFACT
ADR target: docs/arc42/08-crosscutting-concepts.md section 8.7

qualification: NOT_APPLICABLE
alternatives: PASS
architectural_impact: FAIL
durable_consequence: PASS
lower_level_artifact: FAIL
existing_adr_check: PASS
must_be_shared: NOT_SUPPORTING
routing: CANONICAL_ARTIFACT
canonical_artifact: docs/arc42/08-crosscutting-concepts.md section 8.7
target_adr: none
replacement_adr: none
```

---

## Status

STATUS: DONE
