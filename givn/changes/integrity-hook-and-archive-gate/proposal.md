# Proposal: integrity-hook-and-archive-gate

## Problem / Opportunity

An end-to-end review of a completed givn-driven implementation (consumables shopping list, Rust/Axum/HTMX/SQLite) revealed systematic gaps in givn's workflow enforcement. Specifically:

1. The review (review.md) is a self-attested document with no mechanical verification — every fabrication-audit row was filled in but at least half were false. Givn provides no built-in check that tasks.md records actual commit hashes, that design.md's promised components exist on disk, or that arc42 chapter updates are real.
2. Archive (givn archive) merges delta specs into permanent specs but does not verify the merged result passes — a user can archive a change whose permanent-specs tree is broken.
3. During the review, the team noticed that design.md and review.md lack explicit affordances to prevent e2e step bodies from relying on internal primitives (broadcast channels, event emitters) instead of the real user-facing interface.

This change adds three mechanisms to givn that close these gaps: a mechanical integrity hook for the review gate, a post-archive verify gate, and explicit documentation fields in the design/review templates for internal primitives.

## Proposed Solution

When a user has completed implementation and is writing review.md (or running givn check review), givn runs a built-in `integrity` hook that examines tasks.md, design.md, and optionally arc42.md, and reports any concrete fabrication evidence — missing commit hashes, missing step-definition files, unedited arc42 chapter files — as hard failures.

When a user runs givn archive on a completed change, after merging the delta specs into the permanent specs, givn re-runs the project's verify command against the merged permanent specs. If the merged suite fails, archive is blocked and the user is shown where the breakage occurred. The pre-merge permanent specs are preserved in a backup directory so the user can recover manually.

When a user writes design.md, the template includes a mandatory `## Internal Primitives` section that inventories any data-mover primitives (broadcast channels, event emitters, queues) that might be mistaken for the user-facing interface in e2e tests. When the user writes review.md, the review sign-off includes a specific item checking that no e2e step body asserts only on a declared primitive.

## Out of Scope

- Automated scanning of step-definition source code for primitive references. The internal-primitives inventory improves reviewer guidance but is not mechanically enforced.
- Conventional Commits enforcement or git-log cross-referencing. The integrity hook checks only that a non-placeholder hash string is recorded in tasks.md, not that the hash resolves or follows a format.
- `.gitignore` hygiene or any lint checks outside the review/archive gates.
- Any behavioural change to the e2e step-writing workflow itself — only the templates and review criteria change.

## Open Questions

None. The design has been stress-tested via the grill process. All decisions are captured in the conversation archive for `integrity-hook-and-archive-gate`.
