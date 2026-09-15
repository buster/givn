# Design Review: scm-backend

## Grilling

A fresh-context grilling subagent (read-only, full planning artifacts plus the
affected code and permanent specs) walked every required branch. It found no
genuinely operator-facing question: every finding had an unambiguous authoring
fix derived from recorded decisions (Q1, Q4, Q6, Q14, TC-32) and the seed
order.

### Findings and hardening applied

| # | Severity | Finding | Resolution |
|---|---|---|---|
| B1 | Blocker | Two refusal scenarios were not drivable as written: the generic `the consumer runs` binding ignores `cwd_override`/`world_envs`, and a same-text binding would be ambiguous. | The nested scenario now uses `the user runs` (honors `cwd_override`); the missing-`git` scenario uses capability-specific text bound to a PATH-reduced invocation. Recorded in `design.md` "Step-text strategy". |
| B2 | Blocker | Both flow diagrams routed Git-true straight to the Git backend, contradicting the "both markers → ambiguity" rule. | Both diagrams (design and chapter 6.16) now probe both markers before deciding. |
| S3 | Should-fix | The `release-0-3-0` delta renames a Then step while the permanent scenario keeps the old text; `fail_on_skipped` would fail the suite mid-change. | The design now requires binding both texts to the same backend-neutral assertion and updating the shared `command_fails_before_file_changes` acceptance. |
| S4 | Should-fix | The proposal claimed the managed upgrade behaves "exactly as today" while the design adds a new nested-root refusal. | Proposal reworded: the upgrade keeps the dirty refusal and gains the archive's nested-root refusal. |
| S5 | Should-fix | arc42 row 10 was marked No, but Q-50 asserted the old "required Git boundary". | Q-50 rewritten; arc42.md row 10 marked Yes. |
| S6 | Should-fix | arc42 8.10 still said "non-Git or dirty worktree". | Wording neutralized; row 8 summary updated. |
| S7 | Should-fix | Stale Git-only text in chapters 01, 07 and the README status. | Chapter 01 stakeholder row, chapter 07 managed-route paragraph, and the README status paragraph updated; arc42.md rows 1 and 7 marked Yes. |
| S8 | Should-fix | The fragment description and proposal overclaimed: the archive shares nothing in this increment. | Fragment description scoped to "commands that select a backend"; proposal states the archive keeps its current Git boundary until the Fossil-backend change. |
| S9 | Should-fix | The `Worktree::commit` rebuild could drop the `check-ignore` filter for archive evidence. | Design states `commit` keeps filtering ignored evidence before non-force staging. |
| S10 | Should-fix | Glossary Fossil clauses read as current fact although the Fossil backend is unimplemented. | A target-contract note added under the glossary table; D-20 tracks the backend. |
| S11 | Should-fix | `_FOSSIL_` was untested and the ambiguity message hardcoded `.fslckout`. | The message names the marker actually found; `_FOSSIL_` is recorded as an explicit exclusion sharing the same `exists()` path. |
| S12 | Should-fix | The ambiguity and missing-`git` scenarios lacked the no-mutation assertion. | "no file in the project changed" added to both. |
| S13 | Should-fix | The design claimed a `rust-toolchain.toml` pin that does not exist. | Replaced with "the runner's stable toolchain; this repository pins no version". |
| N14 | Note | The matrix's "not `@e2e`" row conflicted with the inventory contract. | Relabeled as an evidence map for a fragment with no inventory. |
| N15 | Note | ADR-071's verdict evidence did not address `lower_level_artifact`; option labels differed. | Evidence paragraph extended; Decision Outcome label aligned with Considered Options. |
| N16 | Note | No step-definition table. | Table added to the design's Test Design. |
| N17-N19 | Note | E2E RED is step-level only; no-config-key rule has no direct scenario; the no-backend refusal appears in two scenarios. | Accepted as-is; all three are honest and the overlap gate passes. |

## Branch dispositions

- **Scope** — the spec matches the proposal after the S4/S8 wording fixes; the
  `release-0-3-0` modification is covered by the design.
- **Tech choices** — the `scm::select` + `Backend` seam is minimal and
  consistent with the recorded decision to detect from markers rather than
  configure; the archive stays on its own Git boundary by seed order.
- **Missing scenarios** — none after S11/S12; `_FOSSIL_` is an explicit
  exclusion, and `stage_paths` force semantics are already covered by the
  legacy-migration fixture.
- **Testability** — every scenario can fail in RED; the two drivability
  blockers (B1) are resolved with concrete step text and a PATH fixture.
- **Risk** — the most likely failure is detection order (both probes must run
  before deciding); B2 hardened both diagrams, and the ambiguity scenario is
  the regression guard.
- **Use-case and Persona context** — the fragment `scm-backend-selection`
  (routing `NEW in fragments`) plus `EXTEND release-0-3-0`; all three
  confirmed Personas exist and are review lenses, never Gherkin actors.
- **Visual design contract** — not applicable: the interface is CLI text, no
  rendered screen; the design records no browser driver for that reason.
- **Interface classification** — tested the opposite: nothing in this change
  renders HTML/CSS/JS, so no Web UI classification applies.
- **Persona disposition** — developer--primary-user (named refusals, proven
  by the refusal scenarios), agent--technical-role (deterministic command
  surface), givn-maintainer--owner (one seam, truthful docs). No deferrals.
- **ADR qualification** — ADR-071 passes all five mandatory dimensions; no
  existing record owns the boundary; registered as `NEW_ADR`.
- **Architecture documentation** — the independent 12-row walk initially
  disagreed on rows 1, 7, and 10 (stale Git-only text); all three chapters
  were corrected and `arc42.md` now matches. All 12 chapter files exist with
  real content and no ASCII-art diagrams.
- **Ubiquitous language** — `marker`, `named refusal`, and
  `version-controlled project` were added to the glossary; `clean state` is
  qualified as the archive witness semantics.

## Hardening checks

- `givn lint --change scm-backend` — 6 findings, all expected `@wip` markers;
  exit 0.
- `givn check review --change scm-backend` — integrity, run declaration, and
  overlap dispositions pass; net delta 6 added, 1 modified.
- No `@e2e` tag was removed or deferred; `verify.e2e_command` is configured.

## Post-implementation design addendum

The implementation made `Worktree::commit_staged` commit-only and reads the
resulting revision through `Backend::revision()`. `design.md` was updated to
match, and a fresh-context re-review confirmed the delta is valid: no
scenario, step definition, arc42 chapter, or ADR depends on the old signature;
`Backend::revision()` is used in production; `cargo check --all-targets` is
warning-free. The re-review's residual finding — archive-specific error
wording leaking into the managed upgrade through `Worktree::head` and
`Worktree::dirty_entries` — was fixed by neutralizing those two messages.
`DESIGN-REVIEW: PASS` stands.

DESIGN-REVIEW: PASS
