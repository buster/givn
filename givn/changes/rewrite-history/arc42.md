# arc42 Documentation Update: rewrite-history

> Change-level arc42 impact assessment. Not an ADR template.

## Impact assessment

| # | Chapter | Affected? | Reason | Summary of change (if Yes) |
|---|---|---|---|---|
| 1 | 01 introduction-and-goals | No | No goal or stakeholder change. | |
| 2 | 02 architecture-constraints | No | No constraint change; the command never pushes and refuses on Fossil. | |
| 3 | 03 context-and-scope | No | No external system; the CLI surface gains a command already described in the glossary. | |
| 4 | 04 solution-strategy | No | No strategy change. | |
| 5 | 05 building-block-view | Yes | A new command module. | Added the `cli::rewrite_history` row. |
| 6 | 06 runtime-view | No | No flow beyond the command sequence in the use-case document. | |
| 7 | 07 deployment-view | No | No deployment change. | |
| 8 | 08 crosscutting-concepts | Yes | Section 8.7 owns generated-rule reporting. | 8.7 now names `givn rewrite-history` as the only remediation. |
| 9 | 09 architecture-decisions | No | No qualified ADR; the explicit-confirmation and no-push boundaries are process guidance. | |
| 10 | 10 quality-requirements | No | No new quality scenario. | |
| 11 | 11 risks-and-technical-debt | No | No new risk; the destructive operation is guarded by preflight and confirmation. | |
| 12 | 12 glossary | No | "History rewrite" was merged during the topic handoff. | |

## Notes

```text
ADR action: CANONICAL_ARTIFACT
ADR target: docs/arc42/08-crosscutting-concepts.md section 8.7

qualification: NOT_APPLICABLE
alternatives: PASS
architectural_impact: PASS
durable_consequence: PASS
lower_level_artifact: FAIL
existing_adr_check: PASS
must_be_shared: NOT_SUPPORTING
routing: CANONICAL_ARTIFACT
canonical_artifact: docs/arc42/08-crosscutting-concepts.md section 8.7
target_adr: none
replacement_adr: none
```

The command's complete rationale is owned by the generated guidance, the
use-case document, and arc42 8.7; no separate ADR is created.

---

## Status

STATUS: DONE
