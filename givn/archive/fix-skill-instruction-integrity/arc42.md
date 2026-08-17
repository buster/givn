# Arc42 Documentation Update: fix-skill-instruction-integrity

## Impact assessment

| # | Chapter | Affected? | Reason | Summary of change |
|---|---|---|---|---|
| 1 | 01 introduction-and-goals | Yes | Adds authoring-policy integrity and a documented instruction JSON contract as user-facing quality goals. | Added FR-30 and FR-31. |
| 2 | 02 architecture-constraints | Yes | The existing single-binary, local-first, cascade, and feature-gated retrieval constraints govern the fix. | No new dependency; clarified that embedded, override, and feature-free boundaries remain binding. |
| 3 | 03 context-and-scope | Yes | `givn instructions --json` and effective canonical guidance are external agent interfaces. | Documented `{id, generates, requires, instruction}` and override-aware canonical guidance. |
| 4 | 04 solution-strategy | Yes | The change repairs policy ownership and metadata flow across the cascade and generated guidance. | Strategy remains cascade plus generated snapshots; policy ownership and JSON flow are clarified. |
| 5 | 05 building-block-view | Yes | `cli::instructions`, `config::cascade`, and `skills::targets` responsibilities are clarified. | Documented effective-node JSON emission and generated-target boundaries. |
| 6 | 06 runtime-view | Yes | Instruction resolution and JSON emission are observable runtime flows. | Documented manifest/overlay/cascade resolution and the exact JSON object. |
| 7 | 07 deployment-view | No | No deployment target, service, native dependency, or distribution mechanism changes. | Local single-binary deployment remains unchanged. |
| 8 | 08 crosscutting-concepts | Yes | Canonical policy ownership, derived guidance, overrides, snapshots, and interface fidelity are cross-cutting concerns. | Expanded normative authoring policy and machine-readable contract sections. |
| 9 | 09 architecture-decisions | Yes | Completing the policy and JSON contract resolves a documented architectural inconsistency. | Added ADR-048 with alternatives, consequences, and confirmation. |
| 10 | 10 quality-requirements | Yes | The change adds acceptance scenarios for policy completeness, remediation evidence, retrieval modes, JSON, and overrides. | Added Q-71 through Q-74. |
| 11 | 11 risks-and-technical-debt | Yes | Policy drift, stale retrieval guidance, missing remediation evidence, and stale JSON consumers are explicit risks. | Added R-46 through R-49 with mitigations. |
| 12 | 12 glossary | Yes | The change introduces effective instruction, instruction JSON contract, and derived guidance as terms. | Added the three terms to the glossary. |

## Notes

The change does not alter semantic scoring, remediation verification, project
override precedence, or lifecycle order. It makes the already-documented
contracts executable and aligns source instructions, derived guidance, and the
CLI metadata response.

## Status

STATUS: DONE
