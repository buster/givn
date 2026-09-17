# arc42 Impact Assessment: guarantee-question-traceability

This change adds the `(open: Q<N>)` qualifier rule, the fail-closed question
disposition, and the decision ledger to the ideation, proposal, design, and
design-review guidance and templates. It changes no runtime behavior.

| # | Chapter | Affected | Reason |
|---|---|---|---|
| 1 | Introduction and Goals | No | No project goal changes; the change hardens authoring contracts. |
| 2 | Architecture Constraints | No | No legal, organizational, or platform constraint changes. |
| 3 | Context and Scope | No | No command or interface behavior changes. |
| 4 | Solution Strategy | No | The existing cascade, renderer, and template paths are reused. |
| 5 | Building-Block View | No | No component responsibility changes; guidance and templates only. |
| 6 | Runtime View | No | No command flow changes. |
| 7 | Deployment View | No | No deployment process or environment changes. |
| 8 | Crosscutting Concepts | Yes | New section 8.23 records the qualifier, the disposition rule, and the ledger contract. |
| 9 | Architecture Decisions | No | Authoring contracts with no component boundary or durable runtime consequence; not a qualified ADR candidate. Canonical destination: chapter 8.23. |
| 10 | Quality Requirements | No | No quality attribute changes; the delta scenarios are guidance-content checks. |
| 11 | Risks and Technical Debt | Yes | R-70 records the prose-only enforcement risk and its mitigation. |
| 12 | Glossary | Yes | `Open-question qualifier` and `Decision ledger` are new terms. |

## ADR action

```text
ADR action: NONE
ADR target: docs/arc42/08-crosscutting-concepts.md §8.23
```

## Status

STATUS: DONE
