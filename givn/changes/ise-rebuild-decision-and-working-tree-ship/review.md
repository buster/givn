# Review: ise-rebuild-decision-and-working-tree-ship

Review kind: independent
Author: Main (omp session)
Reviewer context: agent://DesignReviewIseVocab

## Implementation evidence check

- **Seven scenarios:** the delta has seven added scenarios — two in `ise-rebuild-decision.feature` and five in `instruction-integrity.feature`. All seven checked task records carry `COMMIT: 0e5c905 -- Batch: working-tree-ship-guidance`.
- **Batch commit:** `0e5c905` contains both delta feature files and both permanent step files. The commit tree contains every scenario title and its permanent binding; the diff adds the two new bindings and the phrase correction, while the other five bindings pre-exist in the same permanent files from the operator-decided direct work. The batch is not comment-only, task-only, or feature-only.
- **Direct work:** `bbd7de5`, `4b30dfb`, `92ad0a5`, and `8a7efce` are recorded as the direct vocabulary/guidance work and the e2e binding fix in `tasks.md` and `design-review.md`. The delta carries only the seven regression scenarios; the `prepare-an-ise-test` narrative and its two feature files remain the recorded D4 exception, not full corpus alignment.
- **Empty step bodies:** 0 empty/no-op step bodies across the two changed step files (`change_strategy_steps.rs`, `instruction_integrity_steps.rs`). The new bodies assert served CLI text.
- **E2E integrity:** the delta has no `@e2e` scenario, and the direct-work permanent `ise-rebuild-decision.feature` retains its `@e2e` scenario. No tag was removed to evade the E2E gate. The separate e2e suite passed 120 tests; the `givn check tasks` receipt records `e2e_scope: not_applicable` for the delta, consistent with its regular-only scenarios.
- **Receipt:** `verification/tasks-verification.json` records `head: 8a7efce`, 7/7 passed, 0 failed, 0 skipped; `run declaration`, step tables, overlap dispositions, and receipt reuse pass.
- **Design and interaction surfaces:** the named delta features, permanent step files, e2e step file, direct-work files, and site citation surface exist. The 44-row Interaction Coverage Matrix covers every inventory entry of the touched use cases with exact `@e2e` titles and drivers. No product command, flag, exit code, persisted format, report label, or `ise-manifest/v1` field changed.
- **Site:** `/home/buster/projects/iseprinciples/index.html` names no Givn command, flag, or record; the record heading is gone and the shared Ise wording is used.

## Coverage classification

No uncovered region remains. The seven delta scenarios are covered by `0e5c905` and the permanent bindings; the direct vocabulary/guidance work is outside the delta by recorded operator decision; the copy-path `prepare-an-ise-test` bodies are the documented D4 exception. No `Spec gap` outcome applies: this is a `No rebuild` change with no Ise Test or spec-copy trial. The change makes no product-code change, so no separate coverage finding is raised; the receipt records the regular scope 7/7, and the separate e2e suite passed 120.

No open implementation-evidence findings remain; the previously reported IE-1 (COMMIT-line format), IE-2 (Step Definitions column order), IE-3 (`8a7efce` direct-work accounting), IE-4 (setup task commit labels), and IE-5 (coverage disposition) are resolved.

PERSONAS: developer--primary-user, agent--technical-role, givn-maintainer--owner

REVIEW: PASS
