# arc42 assessment: spec-overlap-gates

## Status

STATUS: DONE

## 12-row assessment

| # | Chapter | Affected | Reason / summary |
| --- | --- | --- | --- |
| 1 | 01 introduction-and-goals | No | No new goals, stakeholders, or quality attributes. |
| 2 | 02 architecture-constraints | No | No new constraints (legal, tech, org). |
| 3 | 03 context-and-scope | Yes | New user-facing surface: `givn lint --json` (machine-readable findings), new exit code 3 for `givn lint`, net-delta receipt in `givn check review` output. Interface table and exit-code row updated. |
| 4 | 04 solution-strategy | No | No change to the overall strategic approach. |
| 5 | 05 building-block-view | Yes | New module `src/overlap/` (`analysis`, `fingerprint`, `disposition`); `cli::lint` and `cli::check` responsibilities extended. Module table updated. |
| 6 | 06 runtime-view | Yes | New lint tree-scan flow (default scope excludes `givn/archive`, exit precedence 1 > 3 > 2) and the review overlap-disposition gate with net-delta receipt. Sections 6.4 and 6.5b updated. |
| 7 | 07 deployment-view | No | No deployment changes (single binary unchanged). |
| 8 | 08 crosscutting-concepts | Yes | Exit-code convention extended (0/1/3/2 with precedence 1 > 3 > 2, warnings never change the code) and the JSON findings contract. Section 8.8 updated. |
| 9 | 09 architecture-decisions | Yes | New ADR-037: deterministic scenario-overlap fingerprint instead of embedding models (with measured recall evidence, 5/8 dup pairs). |
| 10 | 10 quality-requirements | No | No new quality scenarios. |
| 11 | 11 risks-and-technical-debt | Yes | New R-10: overlap-disposition volume on the own tree (self-blocking review gate risk), consequence of ADR-037. |
| 12 | 12 glossary | Yes | New terms: fingerprint, shape match, disposition table, net-delta receipt. |

## Note

The overlap gates are deterministic and model-free by decision (ADR-037);
embedding retrieval stays a conditional authoring aid (plan change
`spec-retrieval`) and is explicitly not part of this change.
