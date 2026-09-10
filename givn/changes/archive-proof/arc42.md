# arc42 Documentation Update: archive-proof

## Impact assessment

| # | Chapter | Affected? | Reason | Summary of change (if Yes) |
|---|---|---|---|---|
| 1 | 01 introduction-and-goals | No | The change does not alter product goals, stakeholders, or quality goals. | |
| 2 | 02 architecture-constraints | Yes | Archive gains a Git prerequisite and a forward-only publication contract. | Added TC-31 and rewrote the archive gate bullet list. |
| 3 | 03 context-and-scope | Yes | Local Git becomes a first-class external entity and the runner gains `GIVN_RESULT_FILE`; the archive directory output includes the receipt. | External-entities, interface, and output rows updated. |
| 4 | 04 solution-strategy | Yes | The archive publication strategy and the review gate strategy change. | Added the archive-publication strategy row; updated the review mandatory checks. |
| 5 | 05 building-block-view | Yes | New `git` and `proof` modules; archive/check/status/state responsibilities change. | Added two building blocks and updated the responsibility table. |
| 6 | 06 runtime-view | Yes | The archive runtime is now a two-stage forward-only pipeline. | Rewrote 6.7, updated 6.6, 6.13, and 6.14. |
| 7 | 07 deployment-view | No | No deployment topology, runtime environment, or packaging change. | |
| 8 | 08 crosscutting-concepts | Yes | Archive provenance joins filesystem-derived state; coverage failure no longer rolls back; review mandatory checks shrink. | Updated 8.6, 8.9, and 8.12. |
| 9 | 09 architecture-decisions | Yes | The forward-only publication boundary is a new durable decision. | Added ADR-070 and its register entry; noted the `proposed` status. |
| 10 | 10 quality-requirements | Yes | New archive-proof quality scenarios. | Added Q-80 through Q-84 and an archive branch in the quality tree. |
| 11 | 11 risks-and-technical-debt | Yes | The old rollback risk is replaced; new review-evidence and induction risks are recorded. | Rewrote R-5 and added R-53, R-54, R-55. |
| 12 | 12 glossary | Yes | The archive-proof domain vocabulary and invariants are new. | Added archive candidate, proven archive, verification receipt, verification scope, runner result, forward repair, and archive-proof invariants. |

## Notes

The archive flow is the only writer of the permanent specification corpus, so
the green-corpus induction argument and its dependencies are stated explicitly
in R-53. Review-time coverage classification is intentionally weaker until a
receipt exists (R-54); this is the accepted cost of removing the duplicate
suite run from review.

ADR action: NEW_ADR
ADR target: docs/arc42/adr/ADR-070-forward-only-archive-proof.md

---

## Status

STATUS: DONE
