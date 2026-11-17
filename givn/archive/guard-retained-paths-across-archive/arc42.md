# arc42 Documentation Update: guard-retained-paths-across-archive

## Impact assessment

| # | Chapter | Affected? | Reason | Summary of change (if Yes) |
|---|---|---|---|---|
| 1 | 01 introduction-and-goals | No | The change adds no goal, stakeholder, or functional requirement; it enforces the existing archive success guarantee and the declared-Specification contract. | |
| 2 | 02 architecture-constraints | No | The rule is behavioral and owned by the `archive-change` use case; no technology, legal, organizational, or deployment constraint changes. | |
| 3 | 03 context-and-scope | No | No new external system or interface: the same `givn archive` CLI command gains one pre-mutation refusal. | |
| 4 | 04 solution-strategy | No | No strategy change; the guard reuses the existing declaration loader, `Glob` matcher, and SCM revision reader. | |
| 5 | 05 building-block-view | Yes | The `cli::archive` responsibility gains one refusal, and the declared-path parent matcher moves to `e2e_boundary::paths` so the archive and the clean-room partition share it. | Extended the `cli::archive` row and the `e2e_boundary` row description. |
| 6 | 06 runtime-view | Yes | The fresh-archive stage list and sequence diagram gain the guard between the receipt check and the structural gates. | Added the guard to stage 3 and the fresh-archive sequence line. |
| 7 | 07 deployment-view | No | No deployment, packaging, or runtime-environment change. | |
| 8 | 08 crosscutting-concepts | Yes | The declaration's read-only configuration rule gains its archive reader condition and the refusal behavior. | Stated that archive reads the declaration only under `ise` and refuses a retained path the move would isolate. |
| 9 | 09 architecture-decisions | No | No ADR: the guard is a refusal rule inside one component, owned completely by `archive-change` and its scenarios. | |
| 10 | 10 quality-requirements | No | No new quality scenario or attribute; the existing archive success guarantee is protected more strictly. | |
| 11 | 11 risks-and-technical-debt | Yes | The comparison basis (content paths versus the trial's tracked revision content) and the accepted symlink hole must be visible. | Added R-149 and R-150. |
| 12 | 12 glossary | Yes | Two terms the change's rule and design use are added to the standalone glossary that chapter 12 links. | Added "Retained path" and "Content path" to `docs/glossary.md`; chapter 12 remains the pointer. |

## ADR qualification

```text
Architectural commitment: none — a refusal inside the existing `cli::archive`
  component; no component, authority, deployment, storage, or compatibility
  boundary moves.
Real alternatives: refuse before the move vs rewrite the declaration vs
  discover the state after the move (proposal D1–D3 and the guard's two
  survival tests).
Durable consequence: reversal is deleting one pre-mutation check; no migration,
  no stored data, and no public contract changes.
Existing-decision search: ADR-010 owns the declaration schema and its read-only
  writers; ADR-070 owns archive proof and the receipt. Neither records archive
  preconditions; the complete home is the `archive-change` use case plus its
  Gherkin and the served `ise` obligation.
ADR action: NONE
ADR target: givn/specs/archive-change/usecase.md
```

## Notes

The guard's acceptance test is stated against the project's content paths
(tracked plus untracked non-ignored), while the trial re-checks declared
retained entries against tracked revision content. The archive's clean-state
witness removes the difference outside evidence, and the finalization revision
publishes the evidence the guard sees; ignored files are absent from both. The
accepted symlink hole is R-150.

---

## Status

STATUS: DONE
