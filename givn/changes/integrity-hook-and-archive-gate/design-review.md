# Design Review: integrity-hook-and-archive-gate

## Grilling log

The full plan (proposal + spec + design) was stress-tested via a six-question
grill in plan mode before the design was drafted. Each question was raised,
answered by the user, and the decision hardened. The design above reflects all
resolved outcomes.

### Q1 — Post-archive verify gate transactionality (violates all-or-nothing)

**Issue:** archive.rs merges specs by iterating delta files and calling
`merge_file`, which writes to disk immediately. The plan's "save Option<String>
per permanent path" hand-waved how to enumerate target paths before the merge
loop.

**Resolution:** Snapshot entire `givn/specs/` tree to `givn/.archive-backup-<id>/`
before merge. On verify failure: do NOT auto-restore. Leave post-merge state
in `givn/specs/`, backup in backup dir. Print error naming both paths + verify
output. On success: clean up backup, proceed.

**Design impact:** mechanism 2 scope changed from auto-restore to backup-only,
with recovery instructions in the error message.

### Q2 — Internal-primitives: hook vs review sign-off

**Issue:** The plan initially proposed an automated `givn lint` scan of step
definition files for declared primitive names. On inspection, the motivating
example (givntest7's `app.tx.subscribe()`) uses field aliases, not the type
name `broadcast::Sender` — a text scan for the declared type finds nothing.
Scanning for the alias (`tx`) is too generic.

**Resolution:** Drop the automated scan. The `## Internal Primitives` section
stays as documentation for the reviewer. The review.md sign-off item stays as
human judgment. No mechanical enforcement.

**Design impact:** mechanism 3 reduced from automated hook to template-only.
Design.md `## Internal Primitives` section remains mandatory. Review sign-off
item added but not hook-enforced.

### Q3 — hygiene/.gitignore decoupled from enforcement

**Issue:** The plan proposed writing .gitignore via `givn init` and a
`givn lint --hygiene` pass checking `git ls-files` against configured globs.

**Resolution:** User: "forget about the gitignore. It's not in the scope of
givn." Entire mechanism dropped.

**Design impact:** F6 (repo hygiene) is out of scope for this change.

### Q4 — Internal-primitives scan can't catch the motivating case

**Issue:** Confirmed that the motivating e2e step body uses `app.tx.subscribe()`,
not `broadcast::Sender`. The declared type name is absent from step definition
files. Mechanism 3's automated scan is unworkable.

**Resolution:** Confirmed Q2's decision. Picked option A: drop automated scan;
design and review templates are the enforcement mechanism.

### Q5 — arc42 conformance check semantics

**Issue (a):** Does byte-compare hard-fail if the file is identical to the
template but the user genuinely didn't need to change it?

**Resolution:** Yes, hard-fail. Purpose is a one-time gate ensuring the FIRST
change marking a chapter "Yes" actually edits it. Upgrade drift produces false
negatives (untouched chapter whose template changed in a later givn release
passes), accepted trade-off.

**Issue (b):** Chapter slug in the marker table column 2 may not resolve to a
real `docs/arc42/` file (agent-typed typo).

**Resolution:** Hook does byte-compare for slugs it CAN resolve; skips
unresolvable ones without failing. Review instructions tell the reviewer to
verify every "Yes" row's slug maps to a real file.

### Q6 — Promised-files-exist: no parseable table in design.md

**Issue:** givntest7's design.md used prose with backticks instead of the
template's markdown table. A table-only parser would find nothing and pass.

**Resolution:** Hard-fail when `## Step Definitions` section exists but
contains no parseable markdown table. Escape hatch: explicit "N/A" or "none"
recognized and skipped.

## Sign-off

All six questions resolved before design was drafted. The design faithfully
implements every resolved decision. No new risks introduced by the design text.

- Scope: spec matches proposal. Three mechanisms, no more.
- Tech choices: appropriate for a Rust CLI tool. No over-engineering.
- Missing scenarios: none identified. All observable behaviours covered.
- Testability: every scenario in the spec can fail in RED (placeholder hash,
  missing file, conflict scenario) and pass in GREEN with the real implemention.
  Scenarios assert concrete observable values (exit code, stdout, stderr,
  filesystem state).
- Risk: post-archive verify gate failure is the highest-impact path — mitigated
  by full backup (no auto-restore), clear error messages, and recovery
  instructions. Implementation risk is low — the design reuses existing patterns
  (hooks.rs match arms, tempfile for backup, shell command runner for verify).
- arc42: N/A for this change. The arc42 addon is not enabled in the givn
  project itself. Mechanism 1(c) only runs when arc42 IS enabled, which is
  correct.

DESIGN-REVIEW: PASS
