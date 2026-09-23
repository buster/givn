# Design Review: local-only-rules

## Grilling

A fresh-context grilling subagent reviewed the proposal, design, arc42
marker, the delta specs, and the implemented tree, and reproduced findings
against live fixtures. All required branches were covered. Findings and
resolutions:

1. **Blocking — `givn commit` staged `AGENTS.md` under `versioned`.** The
   no-path stage set now excludes `AGENTS.md` under both policies (the
   strict rules only under `local-only`). A delta scenario covers the
   versioned path.
2. **Blocking — allowlist-derived discovery missed tracked rules that left
   the configuration** (disabled addon, removed target). Discovery now scans
   tracked paths with a rule-path predicate; the transition and the check use
   it. A delta scenario covers an orphan rule from a disabled addon.
3. **Blocking — Fossil local ignore parsing.** The setting is read with
   `fossil settings --value ignore-glob`; the destination is documented as
   repository-local (a clone inherits it). The tracked `.fossil-settings`
   block remains the primary Fossil destination; a Fossil local-only
   scenario is deferred with this note.
4. **Should-fix — proposal item 4 was unrealizable in sequence.** Amended:
   `givn init` leaves the removals staged for the developer; `givn upgrade`
   performs the transition on a clean checkout.
5. **Should-fix — constraint 2.7, Q-57, and R-33 still named unconditional
   force-staging.** Amended in this change; `arc42.md` rows 2, 10, and 11
   now say Yes.
6. **Should-fix — two vacuous scenarios** (they passed before the change).
   Replaced with the orphan-rule transition and the versioned stage-set
   exclusion.
7. **Should-fix — the managed-revision assertion could not distinguish add
   from delete.** The step now uses `--name-status` and requires `D` for rule
   paths under Git.
8. **Should-fix — missing versioned assertion.** Added: under `versioned`
   sync removes the tracked ignore block.
9. **Should-fix — formatting.** `cargo fmt` applied; clippy clean.
10. **Should-fix — installation report still named `.claude/settings.json`.**
    Updated to `.claude/settings.local.json`.
11. **Should-fix — a failed managed upgrade left staged index removals.**
    The error path now restores the index (`git reset --mixed HEAD`) after
    the file snapshot is restored.
12. **Should-fix — ignore files were deleted when the block was their only
    content**, so a local-only→versioned flip could leave an unstaged
    deletion. The block is now cleared in place; the file is never deleted.
13. **Should-fix — design drift.** Delta-document count, chapter list, and
    file list corrected.
14. **Note — `git log --all` includes remote-tracking refs.** The report
    boundary text now says "local reachable refs (including
    remote-tracking); remote repositories and forks are outside the
    guarantee".
15. **Note — glossary "Strict rule" hid the user-owned `settings.json`.**
    The row now states the shared file is user-owned and untouched and that
    the hook entry lives in `.claude/settings.local.json`.
16. **Note — fragment-capability lint findings clear at archive.** The merge
    adds the new fragment features; confirmed by the archive verification.
17. **Note — ADR qualification.** Both the reviewer and `arc42.md` route the
    policy to arc42 8.7 plus the generated guidance; no ADR qualifies.

## Persona disposition

- `developer--primary-user` — the repository stays clean by default and the
  opt-in keeps the old behavior.
- `agent--technical-role` — deterministic reports; `givn check rules` exits
  non-zero on a violation.
- `external-contributor--open-source-consumer` — a published clone carries no
  rule and needs no Givn.
- `givn-maintainer--owner` — constraint, quality, risk, and scenario records
  stay truthful.

No Persona was invented or auto-promoted; the capability has CLI text output,
so confirmed Personas are named.

## Deferrals

None. The Fossil local-only delta scenario is recorded as a residual in this
review (finding 3) rather than a deferred mandatory check: the archive still
verifies every Fossil scenario in the permanent corpus.

## Hardening applied

- `src/cli/commit.rs`, `src/skills/rules.rs`, `src/cli/migration.rs`,
  `src/git.rs`, `src/scm.rs`, `src/cli/init.rs` updated per findings 1-3,
  9-12.
- Delta specs corrected (findings 1, 2, 6, 8) and the modified
  `enable-optional-addons` scenario added.
- `release_0_3_0_e2e_steps.rs` strengthened (finding 7).
- `docs/arc42/02`, `05`, `08`, `10`, `11`, `12` updated.
- `givn lint --change local-only-rules` clean; `cargo clippy -- -D warnings`
  clean; delta suite 15/15.

DESIGN-REVIEW: PASS
