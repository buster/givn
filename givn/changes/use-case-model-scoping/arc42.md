# arc42 Impact Assessment: use-case-model-scoping

This change publishes the canonical use-case model (definition, eight scoping
rules, authoring requirements, ten rejection criteria, and a worked example),
adds the capability placement test to the specs guidance, and adds authoring
guidance to the generated use-case document template. It adds no parser
warning, router rule, or gate.

| # | Chapter | Affected | Reason |
|---|---|---|---|
| 1 | Introduction and Goals | No | No project goal changes; the model is authoring and review guidance. |
| 2 | Architecture Constraints | No | No legal, organizational, or platform constraint changes. |
| 3 | Context and Scope | No | No command or interface behavior changes. |
| 4 | Solution Strategy | No | The existing cascade, renderer, and template paths are reused. |
| 5 | Building-Block View | No | `gherkin::usecase_md` keeps its responsibility; only its template text changes. |
| 6 | Runtime View | No | No command flow changes. |
| 7 | Deployment View | No | No deployment process or environment changes. |
| 8 | Crosscutting Concepts | Yes | New section 8.21 records the use-case model discipline and its guidance homes. |
| 9 | Architecture Decisions | No | Guidance and documentation only; no component boundary or durable runtime consequence. Not a qualified ADR candidate. Canonical destination: chapter 8.21. |
| 10 | Quality Requirements | No | No quality attribute changes; the delta scenarios are guidance-content checks. |
| 11 | Risks and Technical Debt | Yes | R-69 records the prose-only enforcement risk and its mechanical-review mitigation. |
| 12 | Glossary | Yes | `Use case` gains the canonical definition; `Primary actor` and `Summary use case` are new terms. |

## Status

STATUS: DONE
