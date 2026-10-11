# arc42 Documentation Update: validate-generated-references

> This is the change-level arc42 **impact assessment template**. It is not an
> ADR template and must not be used to draft, store, or combine architecture
> decisions. An architecturally significant decision gets exactly one separate
> MADR file at `docs/arc42/adr/ADR-NNNN-short-slug.md`; chapter 09 is only its
> register/index. Use the ADR guidance and the Qualification gate before
> creating that file.

## Impact assessment

| # | Chapter | Affected? | Reason | Summary of change (if Yes) |
|---|---|---|---|---|
| 1 | 01 introduction-and-goals | No | No goal, stakeholder, or quality-attribute change; the change guards the content of guidance that already exists. | |
| 2 | 02 architecture-constraints | No | No new legal, technical, or organizational constraint; the change applies an existing consumer-view principle to generated guidance. | |
| 3 | 03 context-and-scope | No | No new external system or user-facing surface: the same `givn skills sync` and `givn instructions` outputs change content. | |
| 4 | 04 solution-strategy | No | No strategy or technology direction changes. | |
| 5 | 05 building-block-view | No | No production building block: the reference check is test support under `tests/common/`. | |
| 6 | 06 runtime-view | No | No runtime flow or sequence change; sync and instruction printing already exist. | |
| 7 | 07 deployment-view | No | No deployment or topology change. | |
| 8 | 08 crosscutting-concepts | Yes | Section 8.7 Skill and Rule Generation owns generated-guidance content; section 8.22 owns the persona schema home. | 8.7 gains the consumer-view resolution rule — every guidance reference must resolve in the receiving project or be owned by a named creation step; 8.22 repoints the persona schema sentence at the ideation guidance after the repository-only asset is deleted. |
| 9 | 09 architecture-decisions | No | No qualified ADR candidate; the reference-resolution rule is process/architecture documentation owned by 8.7 (verdict below). | |
| 10 | 10 quality-requirements | Yes | The change establishes a new observable quality property of generated guidance and measures it in the acceptance suite. | Q-97 records the property — every file reference resolves in the receiving project or is owned by a named creation step — and 10.3 names the reference check as its verification signal. |
| 11 | 11 risks-and-technical-debt | Yes | The check's graded surface and its resolution tables carry known limits. | D-20 records the rooted-only extraction, the out-of-surface fenced examples, and the obligation to add a table row with every new workflow-created location. | |
| 12 | 12 glossary | Yes | Two new ubiquitous-language rows: `Consumer project` and `Generated guidance`. | The glossary defines who the guidance is written for and what counts as generated guidance; 8.7 uses both terms. |

## Notes

The enforcement surface is the acceptance suite: the check extracts rooted
references from generated skills, commands, and printed instructions and fails
any reference that the receiving project neither has nor can create through a
named step. The durable rule lives in 8.7; the design's resolution tables (the
workflow-created and project-owned classes) are the realization, and the delta
scenarios prove both the pass and the fail-closed path.

This impact marker does not replace the ADR Qualification Checklist or the
individual ADR file. A change may affect chapter 09 without qualifying for an
ADR, and a qualifying ADR must still be recorded as its own file and linked
from the chapter-09 register. Record the result once in the change assessment:

```text
ADR action: CANONICAL_ARTIFACT
ADR target: docs/arc42/08-crosscutting-concepts.md section 8.7

qualification: NOT_QUALIFIED
alternatives: PASS
architectural_impact: FAIL
durable_consequence: FAIL
lower_level_artifact: FAIL
existing_adr_check: PASS
must_be_shared: NOT_SUPPORTING
routing: CANONICAL_ARTIFACT
canonical_artifact: docs/arc42/08-crosscutting-concepts.md
target_adr: none
replacement_adr: none
```

D1 offered materially different options (keep the use-case-model pointers as
marked repository sources vs drop them), and the design record also weighed a
test-support check against a `givn check` subcommand and a narrow resolution
table against a prefix allowlist, which satisfies the alternatives dimension.
No component, authority, dependency, or deployment boundary moves — the check
is test support — so architectural_impact fails. Reversing the wording is a
handful of text edits plus one test module, so durable_consequence fails. The
complete rationale for a guidance-content rule is process/architecture
documentation, which owns the decision in 8.7; the design records the
realization, so lower_level_artifact fails. `existing_adr_check` searched the
register and archive: ADR-013 owns the generated-guidance destination boundary
and the `AGENTS.md` ownership rule, not reference resolution inside the
guidance, so no amendment or supersession applies and the candidate routes to
its canonical artifact.

Falsification tests applied after the mandatory dimensions: single-change
reversibility holds — the rule and its check reverse within one change — which
by itself fails the gate; no-multi-change-constraint does not hold, because
the rule constrains every future guidance change; product-behavior exclusion
does not hold, because a guidance-content contract is not use-case-ownable
workflow behavior. The outcome is unchanged: two mandatory dimensions already
FAIL.

---

## Status

STATUS: DONE
