# Arc42 Impact Assessment: coverage-merge-union

This change corrects the merge arithmetic for coverage reports: the merged
report counts every measured line once and counts a line as covered when
either source report hit it, instead of summing both reports' counters. It
touches the built-in legacy merge (src/cobertura.rs) and the repository's own
merge script; measurement instrumentation, gate decisions, and badge
rendering are unchanged in mechanism.

| # | Chapter | Affected? | Reason | Summary of change (if Yes) |
|---:|---|:---:|---|---|
| 1 | 01 introduction-and-goals | No | No goal, stakeholder, or quality-attribute change. | |
| 2 | 02 architecture-constraints | No | No constraint change; no new dependencies. | |
| 3 | 03 context-and-scope | No | No external system or interface change; the merged report's consumers (README badge) keep the same contract, now with correct numbers. | |
| 4 | 04 solution-strategy | No | The coverage-gate strategy (measure → merge → validate → publish) is unchanged; only the merge arithmetic is corrected. | |
| 5 | 05 building-block-view | No | No new component or module; `crate::cobertura` gains one data field and corrected merge logic within its existing responsibility. | |
| 6 | 06 runtime-view | No | No new runtime flow; the archive gate's sequence is unchanged. | |
| 7 | 07 deployment-view | No | Binary shape, builds, releases, hosts unchanged. | |
| 8 | 08 crosscutting-concepts | Yes | The coverage-gate section documents the merge contract; the union rule and the documented branch-summation limitation change what that section must state. | The coverage-gate section states that a merged report counts every measured line once and counts a line as covered when either source report hit it, with the aggregate-only fallback and the branch-summation limitation. |
| 9 | 09 architecture-decisions | No | ADR disposition `NONE`. The union-vs-sum correction is a defect fix with one materially correct outcome; it fixes no component boundary, dependency direction, or durable contract beyond the corrected numbers, and its complete rationale (including the documented branch limitation) is owned by the change's `design.md`. | |
| 10 | 10 quality-requirements | No | No new quality scenario; report correctness is the defect being fixed, not a new quality attribute. | |
| 11 | 11 risks-and-technical-debt | No | The documented branch-summation limitation is recorded in `design.md` and the function contract; it is not new debt introduced by this change (the sum existed before; the union is strictly narrower). | |
| 12 | 12 glossary | No | No new domain terms; "merged report", "source report", and coverage vocabulary already exist in the project's usage. | |

## Notes

- The delta extends `extract-coverage-gate` (archival group) with two
  regular scenarios driving the legacy merge path through `givn archive`.
- The repository's own merge-coverages.sh gets the same root-total
  correction; verification is the regenerated repository report (no
  scenario possible for a project-owned script).
- docs/arc42/08-crosscutting-concepts.md (coverage-gate section) is updated
  with the union rule and the branch-summation limitation during
  implementation.

STATUS: DONE
