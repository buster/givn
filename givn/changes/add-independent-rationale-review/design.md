# Design: add-independent-rationale-review

## Context

The gate already validates a rationale's unit, classification, finding, evidence, responsibility, alternative, owner, and banned reasons. This change adds an independent review record to the same artifact.

## Decisions

| ID | Decision | Realized in |
|---|---|---|
| D1 | **Decides D1:** the reviewer is a fresh-context subagent, not a human gate. A rationale carries `Author:`, `Reviewed-by:`, and `Review verdict:`. `Reviewed-by` must differ from `Author`; the verdict must be `accept`; a `reject` verdict fails. | `src/readability/gate/rationale.rs`; `assets/instructions/review.md` |
| DD1 | The gate does not verify a reviewer's identity cryptographically. The isolation is procedural: the orchestrator spawns the reviewer and the author does not fill the reviewer fields. The review instruction states this. | `assets/instructions/review.md` |
| DD2 | The existing rationale floor, FIXED/JUSTIFIED, and PASS/FAIL vocabulary are unchanged. The new fields are additive. | `rationale.rs` |
| DD3 | The change adds no new CLI command or flag. | `src/cli/check.rs` unchanged |

## Test strategy

- Four gate scenarios: independent reviewer passes; missing `Reviewed-by` fails; `Reviewed-by == Author` fails; `Review verdict: reject` fails.
- Smoke tests updated to write the new fields; the existing 19 gate smoke tests remain green.
- The full gate suite and report delta rerun.

## Use-case traceability

Owning use case: `verify-change`; capability `readability-review-gate`. The gate's success guarantee is unchanged: every touched finding is fixed or justified, and the review result is complete.
