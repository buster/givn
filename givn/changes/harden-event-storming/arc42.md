# arc42 Documentation Update: harden-event-storming

> This is the change-level arc42 **impact assessment template**. It is not an
> ADR template and must not be used to draft, store, or combine architecture
> decisions. An architecturally significant decision gets exactly one separate
> MADR file at `docs/arc42/adr/ADR-NNNN-short-slug.md`; chapter 09 is only its
> register/index. Use the ADR guidance and the Qualification gate before
> creating that file.

## Impact assessment

| # | Chapter | Affected? | Reason | Summary of change (if Yes) |
|---|---|---|---|---|
| 1 | 01 introduction-and-goals | No | No goal, stakeholder, or quality-attribute change; the canvas and legality rules are ideation authoring guidance. | |
| 2 | 02 architecture-constraints | No | No constraint changes; the change applies existing guidance constraints. | |
| 3 | 03 context-and-scope | No | Ideation facilitation is already in scope (chapter 3); no new external surface. | |
| 4 | 04 solution-strategy | No | No strategy or technology direction changes. | |
| 5 | 05 building-block-view | No | No building block, module, or ownership change. | |
| 6 | 06 runtime-view | No | No runtime flow or sequence change. | |
| 7 | 07 deployment-view | No | No deployment or topology change. | |
| 8 | 08 crosscutting-concepts | No | No 8.x section owns the event storming phase conduct; the rules live in the generated guidance the section does not restate. | |
| 9 | 09 architecture-decisions | No | No qualified ADR candidate; the rules are workflow policy routed to their canonical artifact. | |
| 10 | 10 quality-requirements | No | No new quality scenario or measurable quality target. | |
| 11 | 11 risks-and-technical-debt | No | No new risk or debt: R-46 already records ideation state as agent-written convention with review-time, not machine, enforcement, and this change strengthens the review lens instead of adding a check. | |
| 12 | 12 glossary | Yes | The phase definition changes: event storming now includes the rendered canvas and aggregate legality. | The "Event Storming" row defines the phase as the chain of past-tense domain events with actors, commands, aggregates, policies, and read models, drawn as a rendered mermaid canvas whose aggregates are declared consistency boundaries — never records, specifications, configurations, or services. |

## Notes

The generated ideation guidance (skill template and command guardrails) is the
enforcement surface for the canvas and legality rules; the workflow row and the
glossary row record the durable meaning. No auto-repair of existing topics and
no lint check is introduced (proposal D2).

This impact marker does not replace the ADR Qualification Checklist or the
individual ADR file. A change may affect chapter 09 without qualifying for an
ADR, and a qualifying ADR must still be recorded as its own file and linked
from the chapter-09 register. Record the result once in the change assessment:

```text
ADR action: CANONICAL_ARTIFACT
ADR target: assets/skills/givn-ideation/SKILL.md.tmpl

qualification: NOT_QUALIFIED
alternatives: PASS
architectural_impact: FAIL
durable_consequence: FAIL
lower_level_artifact: FAIL
existing_adr_check: PASS
must_be_shared: NOT_SUPPORTING
routing: CANONICAL_ARTIFACT
canonical_artifact: assets/skills/givn-ideation/SKILL.md.tmpl
target_adr: none
replacement_adr: none
```

D1 and D3 each offered two or three materially different options (coverage vs
prescribed shape vs presence only; distinguishable kinds vs untyped boxes),
which satisfies the alternatives dimension. No component, authority,
dependency, or deployment boundary moves, so architectural_impact fails;
reversing the wording is a text edit in one asset, so durable_consequence
fails. The complete rationale for authoring rules is process guidance, which
owns the decision — the skill template is its authoritative copy and the
command guardrails repeat it in one line. `existing_adr_check` searched the
register and archive: ADR-013 owns the guidance-generation boundary only, and
chapters 09/ADR-013 record guidance-level conduct as an extension of that
boundary, not a new one.

---

## Status

STATUS: DONE
