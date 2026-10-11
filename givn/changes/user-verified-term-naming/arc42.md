# arc42 Documentation Update: user-verified-term-naming

> This is the change-level arc42 **impact assessment template**. It is not an
> ADR template and must not be used to draft, store, or combine architecture
> decisions. An architecturally significant decision gets exactly one separate
> MADR file at `docs/arc42/adr/ADR-NNNN-short-slug.md`; chapter 09 is only its
> register/index. Use the ADR guidance and the Qualification gate before
> creating that file.

## Impact assessment

| # | Chapter | Affected? | Reason | Summary of change (if Yes) |
|---|---|---|---|---|
| 1 | 01 introduction-and-goals | No | No goal, stakeholder, or quality-attribute change; the verification gate is ideation authoring guidance. | |
| 2 | 02 architecture-constraints | No | No constraint changes; the change applies existing guidance constraints. | |
| 3 | 03 context-and-scope | No | No new external surface: the change alters the content of files the existing `givn skills sync` already writes. | |
| 4 | 04 solution-strategy | No | No strategy or technology direction changes. | |
| 5 | 05 building-block-view | No | No building block, module, or ownership change. | |
| 6 | 06 runtime-view | No | No runtime flow or sequence change; skill rendering already exists. | |
| 7 | 07 deployment-view | No | No deployment or topology change. | |
| 8 | 08 crosscutting-concepts | Yes | Section 8.20 Term Introduction Discipline owns the term-record convention; this change amends the `confirmed` definition. | Section 8.20's first paragraph now defines `confirmed` as requiring the user's verified name in addition to the term introduction test and the recorded search-before-coin check. |
| 9 | 09 architecture-decisions | No | No qualified ADR candidate; the term-verification gate is workflow conduct routed to its canonical artifact. | |
| 10 | 10 quality-requirements | No | No new quality scenario or measurable quality target. | |
| 11 | 11 risks-and-technical-debt | No | No new risk or debt class: R-68 already records the term-verification convention boundary, and this change hardens the same review-time lens by adding the user's verified name instead of a machine check. | |
| 12 | 12 glossary | Yes | The `Term record` row now defines `confirmed` as also carrying the user's verified name, asked immediately when the name fails and otherwise at the phase's term check. | The "Term record" row adds the user's verified name to the `confirmed` status and names the phase term check as the asking point; `docs/workflow.md` records the same rule in its term-record paragraph. |

## Notes

The generated ideation guidance (skill templates and command guardrails) is the
enforcement surface for the naming dialog and the term check; the workflow
paragraph and the glossary row record the durable meaning. The verification is
convention with user and review oversight, not a machine check (proposal Out of
Scope); every `confirmed` term record already carries the reviewed evidence.

This impact marker does not replace the ADR Qualification Checklist or the
individual ADR file. A change may affect chapter 09 without qualifying for an
ADR, and a qualifying ADR must still be recorded as its own file and linked
from the chapter-09 register. Record the result once in the change assessment:

```text
ADR action: CANONICAL_ARTIFACT
ADR target: assets/skills/givn-domain-modeling/SKILL.md.tmpl

qualification: NOT_QUALIFIED
alternatives: PASS
architectural_impact: FAIL
durable_consequence: FAIL
lower_level_artifact: FAIL
existing_adr_check: PASS
must_be_shared: NOT_SUPPORTING
routing: CANONICAL_ARTIFACT
canonical_artifact: assets/skills/givn-domain-modeling/SKILL.md.tmpl
target_adr: none
replacement_adr: none
```

D1 and D2 each offered materially different options (immediate per term vs one
list at phase closing vs the hybrid; all-in-ideation vs all-in-domain-modeling
vs split), which satisfies the alternatives dimension. No component, authority,
dependency, or deployment boundary moves, so architectural_impact fails;
reversing the wording is a text edit in three assets, so durable_consequence
fails. The complete rationale for authoring rules is process guidance, which
owns the decision — the domain-modeling skill template is its authoritative
copy, the ideation skill and command guardrails repeat the binding, and
`design.md` records the realization. `existing_adr_check` searched the register
and archive: ADR-013 owns the generated-guidance destination boundary, and no
existing record decides the naming-dialog content rule, so the candidate
routes to its canonical artifact without amendment or supersession.

---

## Status

STATUS: DONE
