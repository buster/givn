# arc42 Impact Assessment: finding-severity-routing-signal

This change classifies lint findings by severity, reports route confidence
with ownership-weighted scoring, and updates the proposal guidance for the
no-signal case. It changes CLI output, not the artifact graph or gates.

| # | Chapter | Affected | Reason |
|---|---|---|---|
| 1 | Introduction and Goals | No | No project goal changes; the change sharpens tool output. |
| 2 | Architecture Constraints | No | No legal, organizational, or platform constraint changes. |
| 3 | Context and Scope | No | No interface table row changes; the lint and route outputs gain fields. |
| 4 | Solution Strategy | No | The existing lint pipeline and route scoring are extended in place. |
| 5 | Building-Block View | Yes | `cli::lint` gains the severity classification; `spec::route` is recorded as a building block with confidence and ownership weighting. |
| 6 | Runtime View | Yes | Section 6.4 documents the severity classification; new section 6.17 documents route confidence and scoring. |
| 7 | Deployment View | No | No deployment process or environment changes. |
| 8 | Crosscutting Concepts | Yes | New section 8.24 records the severity mapping and the route confidence contract. |
| 9 | Architecture Decisions | No | Output classification and scoring weights; no component boundary or durable migration consequence. Not a qualified ADR candidate. Canonical destination: chapters 6.4 and 6.17. |
| 10 | Quality Requirements | No | No quality attribute changes; the delta scenarios are output checks. |
| 11 | Risks and Technical Debt | Yes | R-71 records the threshold-calibration risk and its mitigation. |
| 12 | Glossary | Yes | `Finding severity` and `Route confidence` are new terms. |

## ADR action

```text
ADR action: NONE
ADR target: docs/arc42/06-runtime-view.md §6.17
```

## Status

STATUS: DONE
