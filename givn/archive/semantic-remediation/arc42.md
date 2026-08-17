# Arc42 Documentation Update: semantic-remediation

## Impact assessment

| # | Chapter | Affected? | Reason | Summary of change |
|---|---|---|---|---|
| 1 | 01 introduction-and-goals | Yes | Adds a quality goal that classifications must be verified against projected remediation. | Added FR-29 for duplicate removal, variant consolidation, and explicit valid boundaries. |
| 2 | 02 architecture-constraints | Yes | Keeps agent-owned edits while adding a deterministic projected-tree verification boundary. | Existing retrieval-enabled/no-feature constraint remains; no new dependency. |
| 3 | 03 context-and-scope | Yes | Blocking review now returns remediation findings and a verification receipt. | Updated the user-facing `check review` contract and non-goal wording. |
| 4 | 04 solution-strategy | Yes | Adds a prior-entry verification path independent of report candidate admission. | Documented recomputation below the boundary and class-specific remediation contracts. |
| 5 | 05 building-block-view | Yes | Extends `cli::check`/`spec::retrieval` responsibilities with projected remediation primitives. | Added the remediation verifier and prior/projected endpoint concepts. |
| 6 | 06 runtime-view | Yes | Adds projected-tree construction, identity matching, remediation verification, and receipt/failure flow. | Documented exact-match, changed-evidence, disappearance, and stale branches. |
| 7 | 07 deployment-view | No | No service, deployment target, runtime dependency, or distribution artifact changes. | CLI remains local and feature-gated as in PR 3. |
| 8 | 08 crosscutting-concepts | Yes | Adds projected remediation, parameterized variant, and observable boundary invariants. | Documented non-mutation and class-specific evidence contracts. |
| 9 | 09 architecture-decisions | Yes | The change introduces a projected-tree verification trade-off. | Added ADR-047 with options, decision, consequences, and confirmation. |
| 10 | 10 quality-requirements | Yes | Adds quality scenarios for remediation, stale entries, below-boundary recomputation, and false score-only passes. | Added Q-67 through Q-70. |
| 11 | 11 risks-and-technical-debt | Yes | Candidate disappearance and score-only false remediation are new risks. | Added R-44 and R-45 with deterministic mitigations. |
| 12 | 12 glossary | Yes | Adds domain terms for projected remediation and class-specific evidence. | Added projected remediation, prior entry, parameterized variant, invariant, and receipt. |

## Notes

The change preserves PR 3's E5 boundary, report grammar, permanent-only index,
advisory review path, and agent-owned feature-file edits. It adds verification
only after the prior report classification is read and the projected tree is
constructed in memory. No classification or remediation is selected or applied
by Givn.

## Status

STATUS: DONE
