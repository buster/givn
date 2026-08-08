# arc42 Documentation Update — fix-archive-merge-docstrings

## Impact Assessment

| # | Chapter | Affected? | Reason | Summary of change (if Yes) |
| --- | --- | --- | --- | --- |
| 01 | introduction-and-goals | No | Bug fix in the archive merge engine; no new goals, stakeholders, or quality attributes | — |
| 02 | architecture-constraints | No | No new constraints | — |
| 03 | context-and-scope | No | Archive's user-facing surface is unchanged (same command, same input/output shape; output is now correct) | — |
| 04 | solution-strategy | No | No strategic change | — |
| 05 | building-block-view | No | No new/moved modules; `gherkin::merge` responsibility text is unchanged (it strips tags and merges deltas) | — |
| 06 | runtime-view | No | Archive runtime flow sequence is unchanged; only the merged spec content is correct now | — |
| 07 | deployment-view | No | No deployment changes | — |
| 08 | crosscutting-concepts | No | Gherkin delta model semantics (ADR-011) unchanged; this fixes a rendering defect within the existing merge | — |
| 09 | architecture-decisions | No | No new architecture/tech decision; a correctness bug fix, not a tradeoff decision | — |
| 10 | quality-requirements | No | Correctness of the merged output is an existing quality goal; no new quality scenario | — |
| 11 | risks-and-technical-debt | Yes | The AST-reconstruction path cannot preserve docstring delimiter style / content-type annotation on replaced scenarios | Added deliberate-debt entry D-9 documenting the bounded fidelity loss |
| 12 | glossary | No | No new terms | — |

## Chapter Updates

### 11-risks-and-technical-debt.md
Added deliberate-debt entry D-9: for `@givn.added`/`@givn.modified` scenarios
the merge re-emits step docstrings with `"""` delimiters from the parsed AST,
so the original delimiter style and any content-type annotation are not
preserved on replaced scenarios (the `gherkin` crate retains only dedented
docstring content). The fix preserves docstring content and data tables; the
fidelity loss is bounded and deliberately out of scope.

## Status

STATUS: DONE
