# Run 4: archive and review friction

This is a persistent copy of the finding written into the run-4 bundle. The
bundle itself remains the authoritative evidence:

`/tmp/opencode/real-runs4/20261019T160626Z-real-initial-long-r1`

## Counts

- `check review` failed 25 times (events `0:810` through `0:1229`).
- `archive` failed 10 times (events `0:1659` through `0:1692`).
- The trial eventually archived successfully and passed all seven independent
  initial requirements.
- Session cost: USD 0.6302; 460 model requests; 46.7 minutes.

## Archive failure sequence

| Time | Event | Failure |
|---|---|---|
| 16:50:32 | `0:1659` | Project had uncommitted `AGENTS.md`. |
| 16:50:37 | `0:1662` | Regular scope produced no runner result. |
| 16:51:28 | `0:1667` | Both change and archive directories existed. |
| 16:51:31 | `0:1668` | Uncommitted `run-tests.sh`. |
| 16:51:33 | `0:1671` | Runner result missing `scope`. |
| 16:51:40 | `0:1675` | Runner result missing `total`. |
| 16:51:44 | `0:1678` | Change missing after recovery reset. |
| 16:51:49 | `0:1681` | Runner result missing `passed`. |
| 16:51:55 | `0:1685` | Tested tree changed outside declared evidence. |
| 16:52:04 | `0:1692` | Same tested-tree error after reset and coverage rerun. |
| 16:52:39 | — | Archive passed after reset to `37a4c5f`. |

## Review failure sequence

| Time | Event | Failure |
|---|---|---|
| 16:29:49 | `0:810` | Missing `- [x] COMMIT:` revision identifiers. |
| 16:30:45 | `0:816` | Duplicate revision identifiers across tasks. |
| 16:35:11 | `0:827` | `VISUAL-REVIEW: UNVERIFIED` required human sign-off. |
| 16:35–16:40 | `0:829`–`0:1229` | Repeated integrity, revision and evidence failures. |

## Follow-up stage

The follow-up ran in a fresh session against the repository produced by the
initial stage and completed successfully:

- `reference-only-books` archive receipt created;
- public follow-up probe passed;
- independent grade: all six follow-up and cross-version requirements passed;
- full trial grade: **13/13**;
- follow-up duration: 1724 s (28.7 min);
- follow-up requests: 295;
- follow-up incremental cost: **USD 0.3978** (stage-scoped event segment);
- archive failures: 1 (uncommitted `task.md`);
- review failures: 6.

The follow-up review failures repeated the same structural friction:

- duplicate revision identifier across two tasks;
- missing `README-IMPACT:` decision line.

The revision-identifier rule is still brittle even after the initial stage
disclosed the one-pass implementation. The archive failure was caused by the
task file being uncommitted, not by a transactional partial archive.

## Candidate fixes

1. Make archive transactional: validate into a temporary location and move only
   after every gate passes. A failed archive must not leave partial state.
2. Document the complete runner-result schema or provide a helper writer; list
   all missing fields in one failure.
3. Add a cheap task-integrity preflight before implementation/review.
4. Make visual review applicability product-type-aware; a CLI has no visual
   interface.
5. Freeze the tested-tree definition across coverage and evidence generation, or
   declare generated evidence paths as expected mutations.

These are hypotheses from one trial, not proof. Test each current-versus-
candidate on the same benchmark, model, driver, timeout and grader.
