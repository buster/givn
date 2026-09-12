# arc42 Documentation Update: visual-ui-review

## Impact assessment

| # | Chapter | Affected? | Reason | Summary of change (if Yes) |
|---|---|---|---|---|
| 1 | 01 introduction-and-goals | No | No goal, stakeholder, or top-level quality goal changes; the change strengthens an existing review gate. | |
| 2 | 02 architecture-constraints | No | No new legal, technical, or organizational constraint. | |
| 3 | 03 context-and-scope | No | No new external system or interface; the visual evidence is produced by each project's existing e2e driver. | |
| 4 | 04 solution-strategy | No | The existing single-binary, embedded-guidance, machine-gate strategy is applied, not changed. | |
| 5 | 05 building-block-view | Yes | `integrity_hook` gains the visual verdict and evidence completeness check. | The `integrity_hook` responsibility row names the visual gate. |
| 6 | 06 runtime-view | Yes | The `givn check review` flow gains the visual gate inside the integrity hook. | Section 6.6 documents the contract-driven visual check and its exemptions. |
| 7 | 07 deployment-view | No | No deployment topology, port, or artifact change. | |
| 8 | 08 crosscutting-concepts | Yes | The visual review discipline is a new cross-cutting review concept. | New section 8.17 describes the contract, evidence, verdict, baseline, and machine boundary. |
| 9 | 09 architecture-decisions | Yes | The visual review gate extends ADR-028's mandatory-check boundary, exactly as the run-declaration gate did; a refinement amends ADR-028 rather than creating a new record. | Chapter 09 records the ADR-028 amendment; the policy rationale stays in the review guidance. |
| 10 | 10 quality-requirements | Yes | New gate-safety quality scenarios for the visual review. | Q-85 (missing verdict), Q-86 (missing evidence), Q-87 (exemption). |
| 11 | 11 risks-and-technical-debt | Yes | New risks: subjective findings, evidence bloat, non-vision harness, bypass by omission; new guidance-level debt. | R-56 to R-59 and D-18. |
| 12 | 12 glossary | Yes | New domain terms and invariants. | Visual interface, Visual Design Contract, Design system, Visual evidence, Visual baseline, Visual verdict, Style tile, and the visual-review invariants. |

## Notes

- The contract format, evidence paths, and slug rule are owned by
  `design.md`; the review obligations are owned by `assets/instructions/review.md`.
- The visual gate is deliberately mechanical only for the verdict marker and
  evidence presence. The persona vision judgment stays review guidance; the
  binary does not infer image quality (D-18).
- No diagram in the affected chapters uses ASCII art.

ADR action: AMEND_ADR
ADR target: ADR-028

---

## Status

STATUS: DONE
