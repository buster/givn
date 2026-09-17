# arc42 Impact Assessment: persona-identity-reuse

This change adds persona identity criteria and a reuse-first protocol to the
ideation guidance, a persona template, an advisory lint finding for dangling
persona references, and a review-time ID resolution. It adds no artifact,
gate, or manifest entry.

| # | Chapter | Affected | Reason |
|---|---|---|---|
| 1 | Introduction and Goals | No | No project goal changes; the change hardens authoring guidance and validation. |
| 2 | Architecture Constraints | No | No legal, organizational, or platform constraint changes. |
| 3 | Context and Scope | No | No interface table row changes; the lint advisory and the review resolution are described in chapters 5 and 6. |
| 4 | Solution Strategy | No | The existing cascade, renderer, lint, and integrity-hook paths are reused. |
| 5 | Building-Block View | Yes | `cli::lint` gains the advisory persona scan; the `integrity_hook` row records the ID resolution. |
| 6 | Runtime View | Yes | Section 6.4 documents the persona scan and its advisory label; 6.6's review checks now include the resolved persona ID. |
| 7 | Deployment View | No | No deployment process or environment changes. |
| 8 | Crosscutting Concepts | Yes | New section 8.22 records the identity criteria, the reuse protocol, and the asymmetric validation. |
| 9 | Architecture Decisions | Yes | The review-time persona-ID resolution extends the ADR-028 mandatory-check boundary for the fourth time; ADR-028 and the chapter-09 register are amended in this change. |
| 10 | Quality Requirements | No | No quality attribute changes; the delta scenarios are functional checks. |
| 11 | Risks and Technical Debt | Yes | R-46's mitigation now names the advisory persona scan and the review-time resolution. |
| 12 | Glossary | Yes | `Persona segment` gains the identity criteria; `Persona promotion` becomes append-record semantics. |

## Status

STATUS: DONE
