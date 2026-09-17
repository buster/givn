# arc42 Impact Assessment: term-introduction-discipline

This change adds the term record schema, the search-before-coin procedure, and
the term introduction test to the generated ideation and domain-modeling
guidance, plus one workflow-document pointer. It adds no runtime code, gate,
or dependency.

| # | Chapter | Affected | Reason |
|---|---|---|---|
| 1 | Introduction and Goals | No | No project goal changes; term discipline is authoring guidance. |
| 2 | Architecture Constraints | No | No legal, organizational, or platform constraint changes. |
| 3 | Context and Scope | No | No interface or command behavior changes. |
| 4 | Solution Strategy | No | The existing asset cascade and renderer are reused. |
| 5 | Building-Block View | No | No component responsibility changes; guidance content only. |
| 6 | Runtime View | No | No command flow changes. |
| 7 | Deployment View | No | No deployment process or environment changes. |
| 8 | Crosscutting Concepts | Yes | New section 8.20 records the term record convention, the collision procedure, and the introduction test. |
| 9 | Architecture Decisions | No | Pure authoring guidance with no component boundary or durable runtime consequence; not a qualified ADR candidate. Canonical destination: chapter 8.20. |
| 10 | Quality Requirements | No | No quality attribute changes; the delta scenarios are functional guidance checks. |
| 11 | Risks and Technical Debt | Yes | R-68 records the unverified-collision risk and its recorded-command mitigation. |
| 12 | Glossary | Yes | `Term record` is a new term. |

## Status

STATUS: DONE
