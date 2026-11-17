# Arc42 Documentation Update: split-stack-instructions

## Impact assessment

| # | Chapter | Affected? | Reason | Summary of change (if Yes) |
|---|---|---|---|---|
| 1 | 01 introduction-and-goals | No | The change delivers an already published guidance capability; it adds no goal, stakeholder, or quality goal. | |
| 2 | 02 architecture-constraints | Yes | The stack instruction id contract and the composition rule are new technical constraints on every future guidance change. | `TC-48` records the split rule and the id and load-condition contract; instruction length carries no numeric cap. |
| 3 | 03 context-and-scope | No | The serving interface is the same CLI request; six instruction ids are added, no external system or user-facing surface appears. | |
| 4 | 04 solution-strategy | Yes | The guidance strategy row now names on-demand stack instructions and the composition rule. | The Guidance row records that core instructions name stack ids instead of carrying stack bodies, with no length cap. |
| 5 | 05 building-block-view | Yes | `cli::instructions` gains the stack id set, and the acceptance suite gains an instruction-reference module. | The `cli::instructions` responsibility names the policy id set; a test-support row records the reference check and the composition assertion. |
| 6 | 06 runtime-view | No | No flow changes: one request still resolves one instruction through the cascade and prints it. | |
| 7 | 07 deployment-view | No | The product stays one binary with embedded assets; the asset set grows by six files. | |
| 8 | 08 crosscutting-concepts | Yes | Guidance composition changes: which instruction owns which rule is a cross-cutting rule for every served instruction. | The guidance section records the split rule, the reference check over served text, and the no-cap rule. |
| 9 | 09 architecture-decisions | Yes | The split is a durable guidance-architecture decision. | `ADR-077` is added to the active register as accepted, the register's count becomes 29, and the pinned register scenario is `@givn.modified` with the new count; chapter 09 stays the link and does not duplicate the body. |
| 10 | 10 quality-requirements | Yes | The composition rule is a maintainability quality boundary with an acceptance target. | `Q-126` records the rule and its served-text assertions; instruction length carries no cap. |
| 11 | 11 risks-and-technical-debt | Yes | The six small files and one-owner-per-rule trade-off are deliberate. | The withdrawn `R-148` and `D-29` are named as retired; no row cites a size ceiling. |
| 12 | 12 glossary | Yes | The split names a durable term, so the glossary gains one row. | `Stack instruction` records the instruction's genus and differentia, the contrast `Stack instruction, not Stack recipe`, and one domain sentence. |

## ADR qualification

```text
Architectural commitment: the guidance corpus carries a project-visible instruction id contract (`<id>-<stack>` resolved through the same cascade and overridable per id), and every rule has one owner: a core step instruction states the universal rules and the id mapping, and a stack instruction serves one runner's mechanics only on its own request.
Real alternatives: six stack instructions versus Rust only versus the outline's five languages (proposal D1); one instruction per stack named by both core instructions versus twelve paired files versus leaving runner mechanics in `design` (proposal D3); instruction length uncapped versus the withdrawn corpus ceiling (proposal D2, operator decision 2026-11-15).
Durable consequence: reversal re-merges six files into two core instructions; the id contract is a compatibility boundary because a project override is keyed by it.
Existing-decision search: active and archived ADR registers checked. ADR-005 owns the facet cascade, ADR-013 owns skills-only generated guidance, and ADR-048 owns the effective-policy and instruction JSON contract. None owns guidance composition.
Cheaper home: `docs/arc42/08-crosscutting-concepts.md` and `design.md` own the current split facts; neither can own the rejected compositions, so the ADR keeps those and chapter 08 carries the current facts and links this record.
ADR action: NEW_ADR
ADR target: ADR-077-stack-instructions
```

Rejected candidates: the six stack files' contents (observable guidance behavior
owned by `serve-extended-rules` and its Gherkin), the reference check's
extraction and resolution rules (a test-support contract recorded in
`design.md`), and the id registration in `POLICY_INSTRUCTION_IDS` (an
implementation detail whose reversal is one array edit).

## Notes

Every affected chapter named above is updated in this change, and
`docs/reference.md`'s policy-id list, `ADR-077`, the register's accepted count, the
pinned register scenario, and both ADR indexes are updated together.
`review.md` keeps every obligation; the earlier line-reduction consolidation is
withdrawn and its content restored, and the surviving checks are asserted by
`instruction-integrity`: `The review guidance keeps every surviving
obligation`, not a documentation side effect. Instruction length carries no
numeric cap; prose that adds no rule remains the length defect.

`docs/domain-model.md` needs no change: the model covers domain units and
explicitly excludes internal modules, command parsing, and file formats, so
instruction composition is architecture, not domain structure. `docs/glossary.md`
gains the `Stack instruction` row: the split makes the term part of the
vocabulary every guidance change uses, and the row records the contrast
`Stack instruction, not Stack recipe` that the ideation topic's `Q15` initially
avoided. The domain-model absence is the assessed result, not an omission.

## Status

STATUS: DONE
