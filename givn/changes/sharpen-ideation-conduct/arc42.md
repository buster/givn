# arc42 Documentation Update: sharpen-ideation-conduct

> This is the change-level arc42 **impact assessment template**. It is not an
> ADR template and must not be used to draft, store, or combine architecture
> decisions. An architecturally significant decision gets exactly one separate
> MADR file at `docs/arc42/adr/ADR-NNNN-short-slug.md`; chapter 09 is only its
> register/index. Use the ADR guidance and the Qualification gate before
> creating that file.

## Impact assessment

| # | Chapter | Affected? | Reason | Summary of change (if Yes) |
|---|---|---|---|---|
| 1 | 01 introduction-and-goals | No | No goal, stakeholder, or quality-attribute change; ideation conduct is process guidance. | |
| 2 | 02 architecture-constraints | No | No constraint changes; the change applies existing guidance constraints. | |
| 3 | 03 context-and-scope | No | No external system, interface, or user-facing surface change. | |
| 4 | 04 solution-strategy | No | No strategy or technology direction changes. | |
| 5 | 05 building-block-view | No | No building block, module, or ownership change. | |
| 6 | 06 runtime-view | No | No runtime flow or sequence change. | |
| 7 | 07 deployment-view | No | No deployment or topology change. | |
| 8 | 08 crosscutting-concepts | Yes | Section 8.19 owns the change seed lifecycle and must state the deliverable cut and the generalized seed identity. | 8.19 now cuts seeds by deliverable guarantee, allows one seed to cover several use cases, forbids intermediate states where a confirmed rule is untrue, and names the seed file by change identity. |
| 9 | 09 architecture-decisions | No | No qualified ADR candidate; the conduct rules are workflow policy routed to their canonical artifact. | |
| 10 | 10 quality-requirements | No | No new quality scenario or measurable quality target. | |
| 11 | 11 risks-and-technical-debt | No | No new risk or debt; no ADR consequence to record. | |
| 12 | 12 glossary | Yes | The glue term "Change seed" changes meaning from per-use-case to per-deliverable. | The row now defines the seed as per-change, cut by deliverable guarantee, possibly covering several use cases, and never creating a state where a confirmed rule is untrue. |

## Notes

The generated ideation guidance (skill template and command guardrails) is the
enforcement surface for the conduct rules; arc42 8.19 and the glossary row
describe the durable meaning. No diagram is introduced.

This impact marker does not replace the ADR Qualification Checklist or the
individual ADR file. A change may affect chapter 09 without qualifying for an
ADR, and a qualifying ADR must still be recorded as its own file and linked
from the chapter-09 register. Record the result once in the change assessment:

```text
ADR action: CANONICAL_ARTIFACT
ADR target: assets/skills/givn-ideation/SKILL.md.tmpl

qualification: NOT_APPLICABLE
alternatives: FAIL
architectural_impact: FAIL
durable_consequence: PASS
lower_level_artifact: FAIL
existing_adr_check: PASS
must_be_shared: NOT_SUPPORTING
routing: CANONICAL_ARTIFACT
canonical_artifact: assets/skills/givn-ideation/SKILL.md.tmpl
target_adr: none
replacement_adr: none
```

The two decisions (D1 seed cut, D2 frame confirmation) are workflow policy;
the qualification procedure routes them to their owning process artifact
instead of an ADR. `lower_level_artifact` fails with the complete rationale
home named: the generated ideation guidance source, whose authoritative copy
is the skill template (the command guardrails repeat the same rules in one
line each). `alternatives` / `architectural_impact` fail because no component,
authority, or deployment boundary moves.

---

## Status

STATUS: DONE
