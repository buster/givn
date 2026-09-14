# arc42 Documentation Update: scm-backend

## Impact assessment

| # | Chapter | Affected? | Reason | Summary of change (if Yes) |
|---|---|---|---|---|
| 1 | 01 introduction-and-goals | Yes | A stakeholder expectation named a "clean Git boundary". | The migrating-maintainer row now says "clean version-control boundary". |
| 2 | 02 architecture-constraints | Yes | The managed-upgrade constraint named Git specifically, and the new selection rule is a durable technical constraint. | TC-21 rewritten for the selected backend; TC-33 added for selection and refusals; 2.7 wording neutralized. |
| 3 | 03 context-and-scope | Yes | A new external probe (Git executable, Fossil marker) and a changed upgrade interface contract. | Git CLI entity extended with backend selection; Fossil checkout marker entity added; `givn upgrade` interface and in-scope list updated. |
| 4 | 04 solution-strategy | Yes | Backend selection is a new cross-cutting strategy concern. | "SCM backend" row added to the strategy table; upgradeability wording moved from "clean Git worktree" to "clean checkout of the selected backend". |
| 5 | 05 building-block-view | Yes | A new module owns selection; the Git wrapper and migration responsibilities change. | `scm` building block added to the level-1 diagram and the responsibility table; `git` and `cli::migration` rows updated. |
| 6 | 06 runtime-view | Yes | A new selection flow precedes every repository-state command. | 6.13 preflight wording updated; new 6.16 documents the detection flow and refusals. |
| 7 | 07 deployment-view | Yes | The managed-route paragraph still said "requires Git" and "Non-Git projects". | Paragraph corrected to the selected backend and a clean checkout. |
| 8 | 08 crosscutting-concepts | Yes | Backend selection and refusal behavior is a cross-cutting concept shared by commands; 8.10 still said "non-Git or dirty worktree". | New 8.18 "SCM Backend Selection"; 8.10 wording neutralized. |
| 9 | 09 architecture-decisions | Yes | The seam-versus-config-key choice is architecturally significant. | ADR-071 accepted and registered; chapter 09 gains its rationale paragraph. |
| 10 | 10 quality-requirements | Yes | Q-50 asserted the old "required Git boundary" wording. | Q-50 rewritten to the no-usable-backend refusal. |
| 11 | 11 risks-and-technical-debt | Yes | Two ADR "Bad, because..." consequences and the deliberate Fossil increment are durable limitations. | R-63 (ambiguous root has no override), R-64 (missing executable vs. no version control), D-20 (Fossil backend not implemented). |
| 12 | 12 glossary | Yes | The change's specs use the SCM vocabulary merged with the ideation handoff. | SCM backend, revision, checkout, clean state, candidate/tested revision, revision binding, upgrade/plan revision are in the glossary. |

## Notes

The archive publication path still uses its own Git boundary; the selection
seam is shared first by the managed upgrade. Moving the archive onto the seam
(and the Fossil backend) is the next change in the topic's seed order, so
TC-31/TC-32 stay as they are for now.

```text
ADR action: NEW_ADR
ADR target: ADR-071
```

## Status

STATUS: DONE
