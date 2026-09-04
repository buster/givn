# arc42 assessment: retire-semantic-capabilities

## Status

STATUS: DONE

## 12-row assessment

| # | Chapter | Affected | Reason / summary |
| --- | --- | --- | --- |
| 1 | 01 introduction-and-goals | No | No new goals, stakeholders, or quality attributes. |
| 2 | 02 architecture-constraints | No | No new constraints. |
| 3 | 03 context-and-scope | Yes | The semantic-similarity engine surface is removed from the spec corpus; 15 capability directories deleted. Scope/interface rows naming retired capabilities updated in a follow-up docs pass. |
| 4 | 04 solution-strategy | No | No change to the overall strategic approach; the retirement uses the already-documented deterministic retirement primitive. |
| 5 | 05 building-block-view | Yes | `givn/specs/` loses 15 capability directories; the tombstone register `givn/specs/.retired.yaml` is added (inside the archive backup scope). Module table entries for the semantic capabilities become stale until the feature-flag drop. |
| 6 | 06 runtime-view | Yes | `givn archive` gains the retirement branch (delete directory + tombstone) in its merge loop; documented by the Phase 0 retirement mechanism. |
| 7 | 07 deployment-view | No | Single binary unchanged. |
| 8 | 08 crosscutting-concepts | No | No new crosscutting concepts; the retirement mechanism itself was introduced by a prior change. |
| 9 | 09 architecture-decisions | No | Retirement direction was decided in the retirement plan; this change applies it to the corpus. No new ADR. |
| 10 | 10 quality-requirements | No | No new quality scenarios. |
| 11 | 11 risks-and-technical-debt | Yes | Retires known debt: the semantic capabilities were dead weight once the gate was switched off; two of the retired entries were already empty corpses. |
| 12 | 12 glossary | Yes | New term: capability retirement (`@givn.retired`), tombstone register (`.retired.yaml`). |
