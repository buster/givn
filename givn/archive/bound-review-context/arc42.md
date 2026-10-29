# arc42 Documentation Update: bound-review-context

## Impact assessment

| # | Chapter | Affected? | Reason | Summary of change (if Yes) |
|---|---|---|---|---|
| 1 | 01 introduction-and-goals | No | The change does not add or change a product goal or stakeholder class. | |
| 2 | 02 architecture-constraints | Yes | The review gate gains a recorded-kind constraint alongside the existing markers. | Added TC-44 for the review-context record and independence rule. |
| 3 | 03 context-and-scope | No | No external system or user-facing surface is added; the existing CLI check boundary is reused. | |
| 4 | 04 solution-strategy | No | The approach stays within the existing module and asset cascade. | |
| 5 | 05 building-block-view | Yes | A new internal component parses and validates the review record. | Added the `review_context` building block and its CLI consumers. |
| 6 | 06 runtime-view | Yes | `givn check review` and archive gain a validation step before the existing outline gate. | Added the review-context step to the check flow. |
| 7 | 07 deployment-view | No | No deployment, packaging, or runtime dependency changes. | |
| 8 | 08 crosscutting-concepts | Yes | The default and independent review policies are cross-cutting workflow concepts. | Added the review-context record section and the one-context policy. |
| 9 | 09 architecture-decisions | No | The decision is workflow policy; it does not qualify for a durable ADR boundary. | |
| 10 | 10 quality-requirements | Yes | Gate safety gains a scenario for a recorded independent review that is not independent. | Added Q-120. |
| 11 | 11 risks-and-technical-debt | Yes | A self-review record can still be mislabeled, and the gate cannot prove reviewer identity. | Added R-124. |
| 12 | 12 glossary | Yes | Self-review, independent review, and review context become project terms. | Added the three terms and corrected the quality-review wording. |

## Notes

The change is workflow policy plus an internal parser. The component boundary is
the change directory; no new external interface, persistent store, or deployment
surface is introduced. The outline gate now cross-checks `OUTLINE: AUTHOR` and
`OUTLINE: REVIEWER` against the recorded review context: a self-review accepts
the self context, an independent review requires the recorded reviewer and a
different author, and a legacy record without a kind keeps the reviewer-differs
rule.

ADR action: NONE
ADR target: `docs/arc42/08-crosscutting-concepts.md` and the review-context feature under `verify-change`

```text
qualification: NOT_QUALIFIED
alternatives: PASS
architectural_impact: FAIL
durable_consequence: PASS
lower_level_artifact: FAIL
existing_adr_check: PASS
must_be_shared: SUPPORTING
routing: CANONICAL_ARTIFACT
canonical_artifact: givn/specs/verify-change/givn-workflow-enforcement.feature and docs/arc42/08-crosscutting-concepts.md
target_adr: null
replacement_adr: null
```

The decision is observable workflow behavior with a change-local component. It
does not fix a new authority, deployment, or storage boundary, so the feature
and the cross-cutting concept chapter are its canonical homes.

---

## Status

STATUS: DONE
