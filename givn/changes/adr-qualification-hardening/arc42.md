# arc42 Impact Assessment: adr-qualification-hardening

This change adds three falsification tests to the ADR qualification gate,
records the verdict before drafting, blanks the ADR template, requires the
cheaper-home challenge in design-review, and rewords the row-9 pressure
language. It changes no runtime behavior, gate, or verdict schema.

| # | Chapter | Affected | Reason |
|---|---|---|---|
| 1 | Introduction and Goals | No | No project goal changes; the change hardens existing authoring guidance. |
| 2 | Architecture Constraints | No | No legal, organizational, or platform constraint changes. |
| 3 | Context and Scope | No | No command or interface behavior changes. |
| 4 | Solution Strategy | No | The existing Arc42 addon and design-review surfaces are reused. |
| 5 | Building-Block View | No | No component responsibility changes. |
| 6 | Runtime View | No | No command flow changes. |
| 7 | Deployment View | No | No deployment process or environment changes. |
| 8 | Crosscutting Concepts | Yes | Section 8.15 records the falsification tests, the verdict placement, the blank template, and the cheaper-home challenge. |
| 9 | Architecture Decisions | No | This change hardens the ADR process itself and mints no new decision record; its own choices are guidance text with no new boundary. Not a qualified candidate. Canonical destination: chapter 8.15. |
| 10 | Quality Requirements | No | No quality attribute changes; the delta scenarios are guidance-content checks. |
| 11 | Risks and Technical Debt | Yes | R-23's mitigation now names the falsification tests and the cheaper-home challenge. |
| 12 | Glossary | Yes | `Falsification tests` and `Cheaper-home challenge` are new terms. |

## Status

STATUS: DONE
