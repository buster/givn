# arc42 Documentation Update: change-release-note

> Change-level arc42 impact assessment. Not an ADR template.

## Impact assessment

| # | Chapter | Affected? | Reason | Summary of change (if Yes) |
|---|---|---|---|---|
| 1 | 01 introduction-and-goals | No | No goal, stakeholder, or quality-attribute change. | |
| 2 | 02 architecture-constraints | No | The shared-subject rule is workflow guidance owned by the use case and the generated guidance, not a legal, technical, or organizational architecture constraint. | |
| 3 | 03 context-and-scope | No | No new external system, interface, or user-facing surface; the existing CLI commands keep their contracts. | |
| 4 | 04 solution-strategy | No | No strategy change; the existing embedded-asset and generation mechanism carries the rule. | |
| 5 | 05 building-block-view | No | No module added or changed; `cli::new`, `cli::instructions`, `cli::project_hint`, and `skills::targets` keep their code paths. | |
| 6 | 06 runtime-view | No | No new flow; `givn new`, `givn instructions`, and `givn skills sync` runtimes are unchanged. | |
| 7 | 07 deployment-view | No | No deployment topology or artifact change. | |
| 8 | 08 crosscutting-concepts | Yes | 8.7 describes skill and rule generation. | 8.7 now records that the generated implementation guidance states the release-note revision-message rule, guidance only. |
| 9 | 09 architecture-decisions | No | No qualified ADR; the guidance-only choice is a workflow policy, not an architectural boundary. | |
| 10 | 10 quality-requirements | No | No new quality scenario; the drift-detection posture stays human review. | |
| 11 | 11 risks-and-technical-debt | No | No new ADR consequence and no new risk register entry; the guidance-only trade-off is recorded in `design.md` (D3). | |
| 12 | 12 glossary | Yes | New domain term. | **Release note** added: the one authored sentence a change's proposal carries, shared as every revision's message subject with the scenario title in the body. |

## Notes

The choice between guidance-only, compose, and warn (D3) has real alternatives
(`alternatives: PASS`). It does not qualify for an ADR: it moves no component,
authority, or deployment boundary, is reversible within this change, and its
complete rationale is owned by `design.md` plus the guidance assets. The
existing-ADR check (`existing_adr_check: PASS`) searched the active register
and the archived records in `docs/arc42/adr/`; no ADR owns commit-subject
composition or generated-guidance policy, so nothing is amended or superseded.

```text
ADR action: CANONICAL_ARTIFACT
ADR target: design.md

qualification: NOT_QUALIFIED
alternatives: PASS
architectural_impact: FAIL
durable_consequence: FAIL
lower_level_artifact: FAIL
existing_adr_check: PASS
must_be_shared: NOT_SUPPORTING
routing: CANONICAL_ARTIFACT
canonical_artifact: design.md
target_adr: none
replacement_adr: none
```

---

## Status

STATUS: DONE
