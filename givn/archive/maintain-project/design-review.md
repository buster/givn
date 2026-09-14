# Design Review: maintain-project

## Grilling

A fresh-context grilling subagent verified the Fossil mechanics against the
installed binary and the code. Per the operator's standing directive, its
findings were resolved with recorded assumptions
(`givn/ideation/multi-scm-support/assumptions.md`) instead of interactive
prompts.

### Findings and hardening applied

| # | Severity | Finding | Resolution |
|---|---|---|---|
| B1 | Blocker | `fossil add` on an ignored path exits 0 after an interactive "Add it?" prompt (null stdin answers N), so a retry-on-refusal design can never detect the refusal. | `stage_paths` passes `--force` exactly when the caller passes `force`; `force` is only true for the calculated allowlist (TC-22). `fossil add` prints `SKIP` (exit 0) for an unchanged managed file — not a refusal. A7 updated. |
| B2 | Blocker | The shared `the user changes remain byte-for-byte unchanged` step shells out to `git status` and would panic on a Fossil fixture. | The binding branches on the fixture backend; Fossil asserts the user file via `fossil changes --differ`. |
| B3 | Blocker | design.md named no arc42 alignment although the change makes permanent chapters false (02 TC-21, 03, 05, 06 6.13, 08 8.18, 11 D-20, 12 glossary). | The chapter list is in Production Impact; all chapters are updated in this change. |
| S4 | Should-fix | Three more "Git could not ..." error contexts remain in migration.rs. | All four are neutralized to backend-neutral wording. |
| N5 | Note | The Fossil migration-bearing case was a regular scenario while its Git counterpart is the inventoried `@e2e`. | The migration e2e became a Git/Fossil outline (archive-change precedent); the standalone regular scenario was dropped. |
| N6 | Note | ADR route for the upgrade-on-both-backends decision. | Amendment of ADR-071 (chapter 09 prose + register relationship row), no new record (A12). |
| N7 | Note | Verified sound: unchanged-managed `fossil add` is a `SKIP` no-op; `--force` adds ignored paths; check-in counting via the `event` table; ignore-glob matches nested skill paths. | none |
| N8 | Note | The embedded migration guide keeps Git wording inside Fossil plans. | Deferred explicitly: the proposal fixes the catalog content as out of scope (A13). |

## Branch dispositions

- **Scope** — the delta matches the proposal: Fossil upgrade path, allowlist
  force staging, refusal removal, fragment retirement; run-command assertions
  stay deferred (A10).
- **Tech choices** — the seam operations mirror the Git semantics; no simpler
  alternative preserves allowlist-only force staging.
- **Missing scenarios** — none after N5; the dirty-Fossil protection covers
  the mirrored strictness on the upgrade path.
- **Testability** — every scenario can fail in RED; the five new bindings
  start as `unimplemented!()`.
- **Risk** — the most likely failure is the interactive `fossil add` prompt
  (B1); the `--force` rule and the fixture's pre-committed ignore-glob remove
  it.
- **Use-case and Persona context** — `maintain-project` (existing, modified,
  `release-0-3-0`); the two confirmed Personas are review lenses.
- **Visual design contract** — not applicable (CLI text only).
- **Interface classification** — CLI; the e2e drives the compiled binary.
- **Persona disposition** — developer--primary-user (both backends upgrade),
  givn-maintainer--owner (one seam, truthful docs). No deferrals.
- **Interaction coverage** — the inventory gains exactly one row whose `@e2e`
  title matches the new scenario; the migration row's outline keeps one row.
- **ADR qualification** — amendment of ADR-071; no new record (A12).
- **Arc42** — the affected chapters are updated in this change (B3).
- **Ubiquitous language** — revision, checkout, clean state, allowlist, plan
  revision are glossary terms used consistently.

## Hardening verification

`givn lint --change maintain-project` exits 0 (clean). The delta carries the
migration outline (Git/Fossil), the Fossil migration-free `@e2e`, the Fossil
dirty-protection scenario, and the fragment tombstone.

DESIGN-REVIEW: PASS
